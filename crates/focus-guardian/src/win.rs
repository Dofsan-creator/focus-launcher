//! Partie Windows : gestionnaire de services et canal nommé.

use focus_core::journal::JournalStore;
use focus_guardian::{Guardian, Request, Response, ServiceBackend, PIPE_NAME, SERVICE_NAME};
use std::ffi::{OsStr, OsString};
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use std::os::windows::ffi::OsStrExt;
use std::os::windows::io::FromRawHandle;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};
use windows::core::PCWSTR;
use windows::Win32::Foundation::{LocalFree, ERROR_PIPE_CONNECTED, HANDLE, HLOCAL};
use windows::Win32::Security::Authorization::{
    ConvertStringSecurityDescriptorToSecurityDescriptorW, SDDL_REVISION_1,
};
use windows::Win32::Security::{PSECURITY_DESCRIPTOR, SECURITY_ATTRIBUTES};
use windows::Win32::Storage::FileSystem::{FILE_FLAGS_AND_ATTRIBUTES, PIPE_ACCESS_DUPLEX};
use windows::Win32::System::Pipes::{
    ConnectNamedPipe, CreateNamedPipeW, PIPE_READMODE_BYTE, PIPE_TYPE_BYTE, PIPE_WAIT,
};
use windows_service::service::{
    ServiceAccess, ServiceControl, ServiceControlAccept, ServiceErrorControl, ServiceExitCode,
    ServiceInfo, ServiceStartType, ServiceState, ServiceStatus, ServiceType,
};
use windows_service::service_control_handler::{self, ServiceControlHandlerResult};
use windows_service::service_manager::{ServiceManager, ServiceManagerAccess};
use windows_service::{define_windows_service, service_dispatcher};

/// Système et administrateurs : tout. Utilisateurs interactifs : lecture et écriture.
const PIPE_SDDL: &str = "D:(A;;GA;;;SY)(A;;GA;;;BA)(A;;GRGW;;;IU)";
const STOP_TIMEOUT: Duration = Duration::from_secs(20);

type BoxError = Box<dyn std::error::Error>;

fn wide(text: impl AsRef<OsStr>) -> Vec<u16> {
    text.as_ref()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect()
}

/// Journal dans ProgramData, hérité en écriture réservée aux administrateurs.
fn journal_path() -> PathBuf {
    let root = std::env::var_os("ProgramData")
        .map_or_else(|| PathBuf::from("C:\\ProgramData"), PathBuf::from);
    root.join("FocusLauncher").join("guardian.lock.json")
}

struct Scm;

impl Scm {
    fn open(
        name: &str,
        access: ServiceAccess,
    ) -> Result<windows_service::service::Service, String> {
        ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)
            .and_then(|manager| manager.open_service(name, access))
            .map_err(|e| e.to_string())
    }
}

impl ServiceBackend for Scm {
    fn is_running(&mut self, name: &str) -> Result<bool, String> {
        let service = Self::open(name, ServiceAccess::QUERY_STATUS)?;
        let status = service.query_status().map_err(|e| e.to_string())?;
        Ok(status.current_state == ServiceState::Running)
    }

    fn stop(&mut self, name: &str) -> Result<(), String> {
        let service = Self::open(name, ServiceAccess::STOP | ServiceAccess::QUERY_STATUS)?;
        service.stop().map_err(|e| e.to_string())?;
        let start = Instant::now();
        while start.elapsed() < STOP_TIMEOUT {
            if service
                .query_status()
                .map_err(|e| e.to_string())?
                .current_state
                == ServiceState::Stopped
            {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(250));
        }
        Err(format!("timeout stopping {name}"))
    }

    fn start(&mut self, name: &str) -> Result<(), String> {
        let service = Self::open(name, ServiceAccess::START | ServiceAccess::QUERY_STATUS)?;
        if service
            .query_status()
            .map_err(|e| e.to_string())?
            .current_state
            == ServiceState::Running
        {
            return Ok(());
        }
        service.start::<OsString>(&[]).map_err(|e| e.to_string())
    }
}

fn create_pipe() -> Result<HANDLE, BoxError> {
    let sddl = wide(PIPE_SDDL);
    let name = wide(PIPE_NAME);
    let mut descriptor = PSECURITY_DESCRIPTOR::default();
    // SAFETY: les tampons vivent pendant les appels ; le descripteur est libéré après usage.
    unsafe {
        ConvertStringSecurityDescriptorToSecurityDescriptorW(
            PCWSTR(sddl.as_ptr()),
            SDDL_REVISION_1,
            &mut descriptor,
            None,
        )?;
        let attributes = SECURITY_ATTRIBUTES {
            nLength: std::mem::size_of::<SECURITY_ATTRIBUTES>() as u32,
            lpSecurityDescriptor: descriptor.0,
            bInheritHandle: false.into(),
        };
        let pipe = CreateNamedPipeW(
            PCWSTR(name.as_ptr()),
            FILE_FLAGS_AND_ATTRIBUTES(PIPE_ACCESS_DUPLEX.0),
            PIPE_TYPE_BYTE | PIPE_READMODE_BYTE | PIPE_WAIT,
            1,
            4096,
            4096,
            0,
            Some(&attributes),
        );
        let _ = LocalFree(HLOCAL(descriptor.0));
        if pipe.is_invalid() {
            return Err(windows::core::Error::from_win32().into());
        }
        Ok(pipe)
    }
}

/// Sert un client jusqu'à sa déconnexion, puis restaure tout.
fn serve_client(pipe: HANDLE, guardian: &mut Guardian<Scm>) -> Result<(), BoxError> {
    // SAFETY: `pipe` est un handle valide, dont `File` prend possession.
    let file = unsafe { File::from_raw_handle(pipe.0) };
    let mut writer = file.try_clone()?;
    for line in BufReader::new(file).lines() {
        let Ok(line) = line else { break };
        let response = match serde_json::from_str::<Request>(&line) {
            Ok(request) => guardian.handle(&request),
            Err(_) => Response::Error {
                message: "malformed request".to_owned(),
            },
        };
        writeln!(writer, "{}", serde_json::to_string(&response)?)?;
    }
    if let Err(err) = guardian.restore_all() {
        tracing::error!("restore after client loss failed: {err}");
    }
    Ok(())
}

fn serve(stop: &Arc<AtomicBool>) -> Result<(), BoxError> {
    if let Some(parent) = journal_path().parent() {
        std::fs::create_dir_all(parent)?;
    }
    let mut guardian = Guardian::new(Scm, JournalStore::new(journal_path()));
    if let Err(err) = guardian.restore_all() {
        tracing::error!("restore at startup failed: {err}");
    }
    while !stop.load(Ordering::SeqCst) {
        let pipe = create_pipe()?;
        // SAFETY: handle valide ; appel synchrone sans OVERLAPPED.
        let connected = unsafe { ConnectNamedPipe(pipe, None) };
        let accepted = connected.is_ok()
            || windows::core::Error::from_win32().code() == ERROR_PIPE_CONNECTED.to_hresult();
        if stop.load(Ordering::SeqCst) {
            break;
        }
        if accepted {
            serve_client(pipe, &mut guardian)?;
        }
    }
    guardian.restore_all().map_err(Into::into)
}

define_windows_service!(ffi_service_main, service_main);

fn service_main(_arguments: Vec<OsString>) {
    if let Err(err) = run_service() {
        tracing::error!("service failed: {err}");
    }
}

fn run_service() -> Result<(), BoxError> {
    let stop = Arc::new(AtomicBool::new(false));
    let handler_stop = Arc::clone(&stop);
    let handle = service_control_handler::register(SERVICE_NAME, move |control| match control {
        ServiceControl::Stop | ServiceControl::Shutdown => {
            handler_stop.store(true, Ordering::SeqCst);
            // Débloque ConnectNamedPipe.
            let _ = File::options().read(true).write(true).open(PIPE_NAME);
            ServiceControlHandlerResult::NoError
        }
        ServiceControl::Interrogate => ServiceControlHandlerResult::NoError,
        _ => ServiceControlHandlerResult::NotImplemented,
    })?;
    let status = |state, accepted| ServiceStatus {
        service_type: ServiceType::OWN_PROCESS,
        current_state: state,
        controls_accepted: accepted,
        exit_code: ServiceExitCode::Win32(0),
        checkpoint: 0,
        wait_hint: Duration::from_secs(5),
        process_id: None,
    };
    handle.set_service_status(status(
        ServiceState::Running,
        ServiceControlAccept::STOP | ServiceControlAccept::SHUTDOWN,
    ))?;
    let result = serve(&stop);
    handle.set_service_status(status(ServiceState::Stopped, ServiceControlAccept::empty()))?;
    result
}

fn install() -> Result<(), BoxError> {
    let manager = ServiceManager::local_computer(
        None::<&str>,
        ServiceManagerAccess::CONNECT | ServiceManagerAccess::CREATE_SERVICE,
    )?;
    let service = manager.create_service(
        &ServiceInfo {
            name: OsString::from(SERVICE_NAME),
            display_name: OsString::from("Focus Launcher Guardian"),
            service_type: ServiceType::OWN_PROCESS,
            start_type: ServiceStartType::AutoStart,
            error_control: ServiceErrorControl::Normal,
            executable_path: std::env::current_exe()?,
            launch_arguments: vec![OsString::from("service")],
            dependencies: Vec::new(),
            account_name: None,
            account_password: None,
        },
        ServiceAccess::START,
    )?;
    service.start::<OsString>(&[])?;
    Ok(())
}

fn uninstall() -> Result<(), BoxError> {
    let manager = ServiceManager::local_computer(None::<&str>, ServiceManagerAccess::CONNECT)?;
    let service = manager.open_service(
        SERVICE_NAME,
        ServiceAccess::STOP | ServiceAccess::DELETE | ServiceAccess::QUERY_STATUS,
    )?;
    if service.query_status()?.current_state != ServiceState::Stopped {
        service.stop()?;
    }
    service.delete()?;
    Ok(())
}

/// Point d'entrée du programme.
pub fn run() -> Result<(), BoxError> {
    match std::env::args().nth(1).as_deref() {
        Some("install") => install(),
        Some("uninstall") => uninstall(),
        Some("service") => {
            service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
            Ok(())
        }
        _ => Err("usage: focus-guardian install|uninstall|service".into()),
    }
}
