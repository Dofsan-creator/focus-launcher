//! Opérations réelles de session, sauf l'arrêt des services (`focus-guardian`).

use crate::scan;
use crate::wide::wide;
use focus_core::model::ProcessInfo;
use focus_core::session::{OpError, SystemOps};
use std::mem::size_of;
use std::process::Command;
use std::sync::mpsc::{self, Sender};
use std::time::{Duration, Instant};
use windows::core::{GUID, PCWSTR};
use windows::Win32::Foundation::{CloseHandle, BOOL, HANDLE, HWND, LPARAM, WPARAM};
use windows::Win32::System::Power::{
    PowerGetActiveScheme, PowerSetActiveScheme, SetThreadExecutionState, ES_CONTINUOUS,
    ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
};
use windows::Win32::System::Threading::{
    GetPriorityClass, OpenProcess, ProcessPowerThrottling, SetPriorityClass, SetProcessInformation,
    TerminateProcess, ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS,
    PROCESS_CREATION_FLAGS, PROCESS_POWER_THROTTLING_CURRENT_VERSION,
    PROCESS_POWER_THROTTLING_EXECUTION_SPEED, PROCESS_POWER_THROTTLING_STATE,
    PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SET_INFORMATION, PROCESS_SUSPEND_RESUME,
    PROCESS_TERMINATE,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetWindowThreadProcessId, IsWindowVisible, PostMessageW, WM_CLOSE,
};

/// Schéma Windows « Performances élevées » (GUID_MIN_POWER_SAVINGS).
const HIGH_PERFORMANCE: GUID = GUID::from_u128(0x8c5e7fda_e8bf_4a96_9a85_a6e23a8c635c);
/// Délai de la fermeture propre.
const CLOSE_TIMEOUT: Duration = Duration::from_secs(10);

#[link(name = "ntdll")]
extern "system" {
    fn NtSuspendProcess(process: HANDLE) -> i32;
    fn NtResumeProcess(process: HANDLE) -> i32;
}

fn err(context: &str, error: impl std::fmt::Display) -> OpError {
    OpError(format!("{context}: {error}"))
}

/// Handle fermé automatiquement.
struct Process(HANDLE);

impl Process {
    fn open(
        pid: u32,
        access: windows::Win32::System::Threading::PROCESS_ACCESS_RIGHTS,
    ) -> Result<Self, OpError> {
        // SAFETY: appel sans pointeur, le résultat est vérifié.
        unsafe { OpenProcess(access, false, pid) }
            .map(Self)
            .map_err(|e| err("open process", e))
    }
}

impl Drop for Process {
    fn drop(&mut self) {
        // SAFETY: le handle vient d'`OpenProcess` et n'est fermé qu'ici.
        let _ = unsafe { CloseHandle(self.0) };
    }
}

/// Force le mode efficacité d'un processus.
fn set_efficiency(handle: HANDLE, enabled: bool) -> Result<(), OpError> {
    let state = PROCESS_POWER_THROTTLING_STATE {
        Version: PROCESS_POWER_THROTTLING_CURRENT_VERSION,
        ControlMask: PROCESS_POWER_THROTTLING_EXECUTION_SPEED,
        StateMask: if enabled {
            PROCESS_POWER_THROTTLING_EXECUTION_SPEED
        } else {
            0
        },
    };
    // SAFETY: la structure et sa taille sont exactes pour cette classe d'information.
    unsafe {
        SetProcessInformation(
            handle,
            ProcessPowerThrottling,
            std::ptr::from_ref(&state).cast(),
            size_of::<PROCESS_POWER_THROTTLING_STATE>() as u32,
        )
    }
    .map_err(|e| err("efficiency mode", e))
}

/// Opérations locales. Les services passent par `services`, la connexion au gardien.
pub struct WinOps {
    /// Client du service élevé, absent si aucune coupure de service n'est prévue.
    pub services: Option<Box<dyn ServiceControl>>,
    awake: Option<Sender<()>>,
}

/// Contrôle des services, assuré par `focus-guardian`.
pub trait ServiceControl: Send {
    /// Arrête un service.
    fn stop(&mut self, name: &str) -> Result<(), OpError>;
    /// Démarre un service.
    fn start(&mut self, name: &str) -> Result<(), OpError>;
}

impl WinOps {
    /// Crée les opérations, avec ou sans gardien.
    pub fn new(services: Option<Box<dyn ServiceControl>>) -> Self {
        Self {
            services,
            awake: None,
        }
    }
}

/// Termine l'anti-veille : l'état est lié à ce fil, qui vit tant que l'envoyeur existe.
fn awake_thread() -> Sender<()> {
    let (sender, receiver) = mpsc::channel::<()>();
    std::thread::spawn(move || {
        // SAFETY: appel sans pointeur. L'état cesse avec le fil, donc à la fin du processus.
        unsafe {
            SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED)
        };
        let _ = receiver.recv();
        // SAFETY: idem.
        unsafe { SetThreadExecutionState(ES_CONTINUOUS) };
    });
    sender
}

struct CloseSearch {
    pid: u32,
    found: bool,
}

unsafe extern "system" fn post_close(window: HWND, param: LPARAM) -> BOOL {
    // SAFETY: `param` pointe sur le `CloseSearch` de `close`, vivant pendant EnumWindows.
    let search = unsafe { &mut *(param.0 as *mut CloseSearch) };
    let mut pid = 0;
    // SAFETY: `pid` est local ; `window` vient de Windows.
    unsafe {
        GetWindowThreadProcessId(window, Some(&mut pid));
        if pid == search.pid && IsWindowVisible(window).as_bool() {
            search.found = true;
            let _ = PostMessageW(window, WM_CLOSE, WPARAM(0), LPARAM(0));
        }
    }
    true.into()
}

/// Demande la fermeture des fenêtres d'un processus. Renvoie faux s'il n'en a pas.
pub fn request_close(pid: u32) -> bool {
    let mut search = CloseSearch { pid, found: false };
    // SAFETY: `search` survit à l'appel synchrone.
    let _ = unsafe {
        EnumWindows(
            Some(post_close),
            LPARAM(std::ptr::from_mut(&mut search) as isize),
        )
    };
    search.found
}

/// Attend la fin d'un processus observé.
pub fn wait_gone(process: &ProcessInfo, timeout: Duration) -> bool {
    let start = Instant::now();
    while start.elapsed() < timeout {
        if scan::process_start_time(process.pid) != Some(process.start_time) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(200));
    }
    false
}

/// Arrêt forcé d'un seul processus.
pub fn terminate(pid: u32) -> Result<(), OpError> {
    let process = Process::open(pid, PROCESS_TERMINATE)?;
    // SAFETY: handle valide.
    unsafe { TerminateProcess(process.0, 1) }.map_err(|e| err("terminate", e))
}

/// Relève la priorité d'un processus lancé directement. Jamais temps réel (SPEC D8).
pub fn boost(pid: u32) -> Result<(), OpError> {
    let process = Process::open(pid, PROCESS_SET_INFORMATION)?;
    // SAFETY: handle valide.
    unsafe { SetPriorityClass(process.0, ABOVE_NORMAL_PRIORITY_CLASS) }
        .map_err(|e| err("priority", e))
}

impl SystemOps for WinOps {
    fn is_same_process(&self, process: &ProcessInfo) -> bool {
        scan::process_start_time(process.pid) == Some(process.start_time)
    }

    fn keep_awake(&mut self) -> Result<(), OpError> {
        self.awake = Some(awake_thread());
        Ok(())
    }

    fn allow_sleep(&mut self) -> Result<(), OpError> {
        self.awake = None;
        Ok(())
    }

    fn active_power_scheme(&self) -> Result<String, OpError> {
        let mut scheme: *mut GUID = std::ptr::null_mut();
        // SAFETY: Windows alloue `scheme`, lu immédiatement ; le journal en garde une copie texte.
        unsafe {
            PowerGetActiveScheme(None, &mut scheme)
                .ok()
                .map_err(|e| err("read power scheme", e))?;
            let value = (*scheme).to_u128();
            let _ = windows::Win32::Foundation::LocalFree(windows::Win32::Foundation::HLOCAL(
                scheme.cast(),
            ));
            Ok(format!("{value:032x}"))
        }
    }

    fn set_high_performance(&mut self) -> Result<(), OpError> {
        // SAFETY: le GUID est une constante vivante pendant l'appel.
        unsafe { PowerSetActiveScheme(None, Some(&HIGH_PERFORMANCE)) }
            .ok()
            .map_err(|e| err("set power scheme", e))
    }

    fn set_power_scheme(&mut self, guid: &str) -> Result<(), OpError> {
        let value = u128::from_str_radix(guid, 16).map_err(|e| err("bad scheme", e))?;
        let scheme = GUID::from_u128(value);
        // SAFETY: `scheme` est local.
        unsafe { PowerSetActiveScheme(None, Some(&scheme)) }
            .ok()
            .map_err(|e| err("restore power scheme", e))
    }

    fn process_priority(&self, process: &ProcessInfo) -> Result<u32, OpError> {
        let handle = Process::open(process.pid, PROCESS_QUERY_LIMITED_INFORMATION)?;
        // SAFETY: handle valide.
        match unsafe { GetPriorityClass(handle.0) } {
            0 => Err(OpError("read priority failed".to_owned())),
            class => Ok(class),
        }
    }

    fn throttle(&mut self, process: &ProcessInfo) -> Result<(), OpError> {
        let handle = Process::open(process.pid, PROCESS_SET_INFORMATION)?;
        set_efficiency(handle.0, true)?;
        // SAFETY: handle valide.
        unsafe { SetPriorityClass(handle.0, BELOW_NORMAL_PRIORITY_CLASS) }
            .map_err(|e| err("priority", e))
    }

    fn unthrottle(&mut self, process: &ProcessInfo, class: u32) -> Result<(), OpError> {
        let handle = Process::open(process.pid, PROCESS_SET_INFORMATION)?;
        set_efficiency(handle.0, false)?;
        // SAFETY: handle valide.
        unsafe { SetPriorityClass(handle.0, PROCESS_CREATION_FLAGS(class)) }
            .map_err(|e| err("priority", e))
    }

    fn suspend(&mut self, process: &ProcessInfo) -> Result<(), OpError> {
        let handle = Process::open(process.pid, PROCESS_SUSPEND_RESUME)?;
        // SAFETY: handle valide, ouvert avec le droit requis.
        match unsafe { NtSuspendProcess(handle.0) } {
            0 => Ok(()),
            status => Err(OpError(format!("suspend failed: {status:#x}"))),
        }
    }

    fn resume(&mut self, process: &ProcessInfo) -> Result<(), OpError> {
        let handle = Process::open(process.pid, PROCESS_SUSPEND_RESUME)?;
        // SAFETY: handle valide, ouvert avec le droit requis.
        match unsafe { NtResumeProcess(handle.0) } {
            0 => Ok(()),
            status => Err(OpError(format!("resume failed: {status:#x}"))),
        }
    }

    fn relaunch_info(&self, process: &ProcessInfo) -> Result<(String, Vec<String>), OpError> {
        process
            .path
            .clone()
            .map(|path| (path, Vec::new()))
            .ok_or_else(|| OpError("unknown process path".to_owned()))
    }

    fn close(&mut self, process: &ProcessInfo, allow_force: bool) -> Result<(), OpError> {
        request_close(process.pid);
        if wait_gone(process, CLOSE_TIMEOUT) {
            return Ok(());
        }
        if allow_force {
            return terminate(process.pid);
        }
        Err(OpError(format!("{} did not close", process.name)))
    }

    fn relaunch(&mut self, path: &str, args: &[String]) -> Result<(), OpError> {
        Command::new(path)
            .args(args)
            .spawn()
            .map(drop)
            .map_err(|e| err("relaunch", e))
    }

    fn stop_service(&mut self, name: &str) -> Result<(), OpError> {
        match &mut self.services {
            Some(control) => control.stop(name),
            None => Err(OpError("guardian service is not installed".to_owned())),
        }
    }

    fn start_service(&mut self, name: &str) -> Result<(), OpError> {
        match &mut self.services {
            Some(control) => control.start(name),
            None => Err(OpError("guardian service is not installed".to_owned())),
        }
    }
}

/// Encode une valeur pour un futur appel Windows (utilitaire de ce module).
pub fn pcwstr_owned(text: &str) -> (Vec<u16>, PCWSTR) {
    let buffer = wide(text);
    let pointer = PCWSTR(buffer.as_ptr());
    (buffer, pointer)
}
