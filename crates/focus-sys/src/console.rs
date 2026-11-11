//! Mode console de Windows : bureau arrêté (ou barre des tâches masquée),
//! notifications en sourdine, PC éveillé. Tout est réversible. L'Explorateur est
//! arrêté puis relancé, jamais remplacé : aucun réglage de Windows ne désigne Focus
//! comme bureau (décision 23, accord de l'utilisateur du 2026-11-11).

use std::sync::mpsc;
use std::time::{SystemTime, UNIX_EPOCH};
use windows::Foundation::DateTime;
use windows::Win32::Foundation::{CloseHandle, BOOL, HWND, LPARAM};
use windows::Win32::System::Threading::{OpenProcess, WaitForSingleObject, INFINITE, PROCESS_SYNCHRONIZE};
use windows::Win32::System::Power::{
    SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED,
};
use windows::Win32::UI::WindowsAndMessaging::{
    EnumWindows, GetClassNameW, IsWindowVisible, ShowWindow, SW_HIDE, SW_SHOWNA,
};
use windows::UI::Shell::FocusSessionManager;

/// Classes des barres des tâches : écran principal et écrans secondaires.
const TASKBAR_CLASSES: [&str; 2] = ["Shell_TrayWnd", "Shell_SecondaryTrayWnd"];

unsafe extern "system" fn collect_taskbar(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // SAFETY: `lparam` pointe sur le vecteur de `taskbars`, vivant pendant l'appel.
    let list = unsafe { &mut *(lparam.0 as *mut Vec<HWND>) };
    let mut class = [0_u16; 64];
    // SAFETY: tampon local de taille fixe.
    let len = unsafe { GetClassNameW(hwnd, &mut class) };
    let name = String::from_utf16_lossy(&class[..len.max(0) as usize]);
    if TASKBAR_CLASSES.contains(&name.as_str()) {
        list.push(hwnd);
    }
    BOOL(1)
}

fn taskbars() -> Vec<HWND> {
    let mut list: Vec<HWND> = Vec::new();
    // SAFETY: le rappel ne garde pas le pointeur au-delà de l'énumération.
    unsafe {
        let _ = EnumWindows(
            Some(collect_taskbar),
            LPARAM(&mut list as *mut Vec<HWND> as isize),
        );
    }
    list
}

/// Affiche ou masque toutes les barres des tâches. Renvoie le nombre traité.
pub fn set_taskbars_visible(visible: bool) -> usize {
    let command = if visible { SW_SHOWNA } else { SW_HIDE };
    let bars = taskbars();
    for hwnd in &bars {
        // SAFETY: fenêtre de l'Explorateur, seulement montrée ou cachée.
        unsafe {
            let _ = ShowWindow(*hwnd, command);
        }
    }
    bars.len()
}

/// Une barre des tâches est-elle visible ? Sert à la remasquer si Windows la remontre.
pub fn any_taskbar_visible() -> bool {
    // SAFETY: lecture d'état d'une fenêtre.
    taskbars()
        .into_iter()
        .any(|hwnd| unsafe { IsWindowVisible(hwnd) }.as_bool())
}

fn filetime_now() -> i64 {
    // Intervalles de 100 ns depuis 1601, le format de `DateTime`.
    let unix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_micros() as i64);
    unix * 10 + 116_444_736_000_000_000
}

/// Lance une session de concentration (Ne pas déranger) qui finit seule après
/// `minutes`, même si le launcher plante. Faux si Windows ne la propose pas, ou
/// si l'utilisateur en a déjà une : on ne la lui prend jamais.
pub fn start_focus_session(minutes: i64) -> bool {
    let start = || -> windows::core::Result<bool> {
        if !FocusSessionManager::IsSupported()? {
            return Ok(false);
        }
        let manager = FocusSessionManager::GetDefault()?;
        if manager.IsFocusActive()? {
            return Ok(false);
        }
        let end = DateTime {
            UniversalTime: filetime_now() + minutes * 60 * 10_000_000,
        };
        manager.TryStartFocusSession2(end)?;
        Ok(true)
    };
    start().unwrap_or_else(|err| {
        tracing::warn!("focus session not started: {err}");
        false
    })
}

/// Termine la session de concentration lancée par le launcher.
pub fn end_focus_session() {
    let end = || -> windows::core::Result<()> {
        if FocusSessionManager::IsSupported()? {
            FocusSessionManager::GetDefault()?.DeactivateFocus()?;
        }
        Ok(())
    };
    if let Err(err) = end() {
        tracing::warn!("focus session not ended: {err}");
    }
}

/// Garde le PC et l'écran éveillés tant que la valeur vit. L'état d'exécution
/// est propre à un fil : un fil dédié le tient.
pub struct KeepAwake {
    stop: Option<mpsc::Sender<()>>,
}

impl KeepAwake {
    /// Commence à garder le PC éveillé.
    pub fn start() -> Self {
        let (stop, wait) = mpsc::channel::<()>();
        std::thread::spawn(move || {
            // SAFETY: change seulement l'état d'exécution de ce fil.
            unsafe {
                SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED);
            }
            let _ = wait.recv();
            // SAFETY: rend l'état normal avant la fin du fil.
            unsafe {
                SetThreadExecutionState(ES_CONTINUOUS);
            }
        });
        Self { stop: Some(stop) }
    }
}

impl Drop for KeepAwake {
    fn drop(&mut self) {
        // Fermer le canal réveille le fil, qui rend l'état normal.
        self.stop.take();
    }
}

/// Attend la fin d'un processus du launcher (la sentinelle surveille son parent).
/// Jamais utilisé sur un jeu (SPEC D7).
pub fn wait_for_exit(pid: u32) {
    // SAFETY: poignée de synchronisation seule, fermée après l'attente.
    unsafe {
        if let Ok(handle) = OpenProcess(PROCESS_SYNCHRONIZE, false, pid) {
            let _ = WaitForSingleObject(handle, INFINITE);
            let _ = CloseHandle(handle);
        }
    }
}


// --- Touche Windows : elle ramène au launcher, comme le bouton central ---

use std::sync::Mutex;
use windows::Win32::Foundation::{LRESULT, WPARAM};
use windows::Win32::System::LibraryLoader::GetModuleHandleW;
use windows::Win32::System::Threading::GetCurrentThreadId;
use windows::Win32::UI::Input::KeyboardAndMouse::{VK_LWIN, VK_RWIN};
use windows::Win32::UI::WindowsAndMessaging::{
    CallNextHookEx, GetMessageW, PostThreadMessageW, SetWindowsHookExW, UnhookWindowsHookEx,
    KBDLLHOOKSTRUCT, MSG, WH_KEYBOARD_LL, WM_KEYDOWN, WM_QUIT, WM_SYSKEYDOWN,
};

static WIN_KEY: Mutex<Option<mpsc::Sender<()>>> = Mutex::new(None);

unsafe extern "system" fn keyboard_hook(code: i32, wparam: WPARAM, lparam: LPARAM) -> LRESULT {
    if code >= 0 {
        // SAFETY: pour WH_KEYBOARD_LL, lparam pointe sur un KBDLLHOOKSTRUCT valide.
        let info = unsafe { &*(lparam.0 as *const KBDLLHOOKSTRUCT) };
        if info.vkCode == u32::from(VK_LWIN.0) || info.vkCode == u32::from(VK_RWIN.0) {
            let message = wparam.0 as u32;
            if message == WM_KEYDOWN || message == WM_SYSKEYDOWN {
                if let Ok(slot) = WIN_KEY.lock() {
                    if let Some(sender) = slot.as_ref() {
                        let _ = sender.send(());
                    }
                }
            }
            return LRESULT(1);
        }
    }
    // SAFETY: transmission normale au crochet suivant.
    unsafe { CallNextHookEx(None, code, wparam, lparam) }
}

/// Tant que la valeur vit, la touche Windows ouvre le launcher au lieu du menu
/// Démarrer. Le crochet ne regarde que cette touche ; Windows le retire seul si
/// le processus disparaît. Ctrl+Alt+Suppr reste toujours disponible.
pub struct WinKeyGuard {
    thread: u32,
}

impl WinKeyGuard {
    /// Pose le crochet. `on_press` est appelé à chaque appui.
    pub fn start(on_press: impl Fn() + Send + 'static) -> Option<Self> {
        let (sender, receiver) = mpsc::channel::<()>();
        *WIN_KEY.lock().ok()? = Some(sender);
        std::thread::spawn(move || {
            while receiver.recv().is_ok() {
                on_press();
            }
        });
        let (tell, wait) = mpsc::channel::<u32>();
        std::thread::spawn(move || {
            // SAFETY: crochet et boucle de messages sur ce fil, retirés en sortant.
            unsafe {
                let instance = GetModuleHandleW(None)
                    .ok()
                    .map(|module| windows::Win32::Foundation::HINSTANCE(module.0));
                let hook = SetWindowsHookExW(WH_KEYBOARD_LL, Some(keyboard_hook), instance.unwrap_or_default(), 0);
                match &hook {
                    Ok(_) => tracing::info!("windows key hook installed"),
                    Err(err) => tracing::warn!("windows key hook not installed: {err}"),
                }
                let _ = tell.send(GetCurrentThreadId());
                let mut message = MSG::default();
                while GetMessageW(&mut message, None, 0, 0).as_bool() {}
                if let Ok(hook) = hook {
                    let _ = UnhookWindowsHookEx(hook);
                }
            }
        });
        let thread = wait.recv().ok()?;
        Some(Self { thread })
    }
}

impl Drop for WinKeyGuard {
    fn drop(&mut self) {
        // SAFETY: demande de fin à la boucle de messages du fil du crochet.
        unsafe {
            let _ = PostThreadMessageW(self.thread, WM_QUIT, WPARAM(0), LPARAM(0));
        }
        if let Ok(mut slot) = WIN_KEY.lock() {
            *slot = None;
        }
    }
}


// --- Fenêtres des autres programmes et plein écran ---

use windows::Win32::Foundation::RECT;
use windows::Win32::Graphics::Gdi::{GetMonitorInfoW, MonitorFromWindow, MONITORINFO, MONITOR_DEFAULTTONEAREST};
use windows::Win32::UI::WindowsAndMessaging::{
    GetWindowLongPtrW, GetWindowTextLengthW, GetWindowThreadProcessId, IsIconic, SetWindowLongPtrW,
    SetWindowPos, GWL_EXSTYLE, GWL_STYLE, SWP_FRAMECHANGED, SWP_NOOWNERZORDER, SWP_NOZORDER,
    SW_SHOWMINNOACTIVE, SW_SHOWNOACTIVATE, WS_CAPTION, WS_EX_TOOLWINDOW, WS_MAXIMIZEBOX,
    WS_MINIMIZEBOX, WS_SYSMENU, WS_THICKFRAME,
};

unsafe extern "system" fn collect_all(hwnd: HWND, lparam: LPARAM) -> BOOL {
    // SAFETY: `lparam` pointe sur le vecteur de l’appelant, vivant pendant l’énumération.
    let list = unsafe { &mut *(lparam.0 as *mut Vec<HWND>) };
    list.push(hwnd);
    BOOL(1)
}

fn top_windows() -> Vec<HWND> {
    let mut list: Vec<HWND> = Vec::new();
    // SAFETY: le rappel ne garde pas le pointeur.
    unsafe {
        let _ = EnumWindows(Some(collect_all), LPARAM(&mut list as *mut Vec<HWND> as isize));
    }
    list
}

fn window_pid(hwnd: HWND) -> u32 {
    let mut pid = 0_u32;
    // SAFETY: lit le propriétaire de la fenêtre, sans poignée sur le processus.
    unsafe {
        GetWindowThreadProcessId(hwnd, Some(&mut pid));
    }
    pid
}

fn is_app_window(hwnd: HWND) -> bool {
    // SAFETY: lectures d’état de fenêtre.
    unsafe {
        IsWindowVisible(hwnd).as_bool()
            && GetWindowTextLengthW(hwnd) > 0
            && (GetWindowLongPtrW(hwnd, GWL_EXSTYLE) as u32 & WS_EX_TOOLWINDOW.0) == 0
    }
}

/// Processus qui ont au moins une fenêtre d'application visible (même réduite).
pub(crate) fn app_window_pids() -> Vec<u32> {
    let mut pids: Vec<u32> = top_windows()
        .into_iter()
        .filter(|hwnd| is_app_window(*hwnd))
        .map(window_pid)
        .collect();
    pids.sort_unstable();
    pids.dedup();
    pids
}

// --- Bureau de Windows (Explorateur) : arrêté tant que Focus est ouvert ---

/// Message que la barre des tâches comprend comme « quitter l'Explorateur » : il
/// enregistre son état et sort proprement, sans que Windows le relance.
const EXIT_EXPLORER: u32 = 0x5B4;

/// Explorateurs de la session de l'utilisateur.
fn explorers() -> Vec<u32> {
    let own = crate::scan::session_of(std::process::id());
    crate::scan::process_ids()
        .into_iter()
        .filter(|(pid, name)| name == "explorer" && crate::scan::session_of(*pid) == own)
        .map(|(pid, _)| pid)
        .collect()
}

/// Le bureau de Windows tourne-t-il ?
pub fn shell_running() -> bool {
    use windows::Win32::UI::WindowsAndMessaging::GetShellWindow;
    // SAFETY: simple lecture.
    !unsafe { GetShellWindow() }.is_invalid() || !explorers().is_empty()
}

/// Arrête le bureau de Windows : plus de barre des tâches, de menu Démarrer ni
/// d'icônes. Rien n'est écrit nulle part : Windows le relance à la prochaine
/// ouverture de session, et `start_shell` le relance tout de suite. Vrai s'il ne
/// reste aucun Explorateur.
pub fn stop_shell() -> bool {
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{FindWindowW, PostMessageW};
    let class = crate::wide::wide("Shell_TrayWnd");
    // SAFETY: tampon local vivant pendant l'appel ; message sans pointeur.
    unsafe {
        if let Ok(tray) = FindWindowW(windows::core::PCWSTR(class.as_ptr()), windows::core::PCWSTR::null()) {
            let _ = PostMessageW(tray, EXIT_EXPLORER, WPARAM(0), LPARAM(0));
        }
    }
    for _ in 0..40 {
        if explorers().is_empty() {
            return true;
        }
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
    // Il n'a pas répondu : arrêt direct, comme le Gestionnaire des tâches.
    for pid in explorers() {
        if let Err(err) = crate::ops::terminate(pid) {
            tracing::warn!("desktop process {pid} not stopped: {err}");
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(500));
    explorers().is_empty()
}

/// Relance le bureau de Windows avec les droits normaux de l'utilisateur. Sans effet
/// s'il tourne déjà.
pub fn start_shell() -> bool {
    if shell_running() {
        return true;
    }
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_owned());
    let exe = format!("{}\\explorer.exe", root.trim_end_matches('\\'));
    if crate::unelevated::spawn_as_user(&format!("\"{exe}\""), None).is_some() {
        return true;
    }
    // Mieux vaut un bureau avec trop de droits que pas de bureau du tout.
    tracing::warn!("desktop restarted with the rights of Focus: no ordinary process to borrow rights from");
    std::process::Command::new(exe).spawn().is_ok()
}

/// Mémoire vive : totale et disponible, en octets.
pub fn memory() -> (u64, u64) {
    use windows::Win32::System::SystemInformation::{GlobalMemoryStatusEx, MEMORYSTATUSEX};
    let mut status = MEMORYSTATUSEX {
        dwLength: std::mem::size_of::<MEMORYSTATUSEX>() as u32,
        ..Default::default()
    };
    // SAFETY: structure locale dont la taille est renseignée.
    match unsafe { GlobalMemoryStatusEx(&mut status) } {
        Ok(()) => (status.ullTotalPhys, status.ullAvailPhys),
        Err(_) => (0, 0),
    }
}

/// Réduit les fenêtres des autres programmes ; renvoie leur liste pour les rendre ensuite.
pub fn minimize_others() -> Vec<isize> {
    let own = std::process::id();
    let mut done = Vec::new();
    for hwnd in top_windows() {
        // SAFETY: lecture d’état.
        let iconic = unsafe { IsIconic(hwnd).as_bool() };
        if iconic || !is_app_window(hwnd) || window_pid(hwnd) == own {
            continue;
        }
        // SAFETY: réduction sans activation.
        unsafe {
            let _ = ShowWindow(hwnd, SW_SHOWMINNOACTIVE);
        }
        done.push(hwnd.0 as isize);
    }
    done
}

/// Fenêtre affichée à l'écran : poignée, processus, part de l'écran couverte (0 à 1).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ShownWindow {
    /// Poignée de fenêtre (pas de processus : aucun handle sur le jeu, SPEC D7).
    pub hwnd: isize,
    /// Processus propriétaire.
    pub pid: u32,
    /// Part de son écran qu'elle couvre.
    pub coverage: f32,
}

fn shown(hwnd: HWND) -> Option<ShownWindow> {
    use windows::Win32::Foundation::RECT;
    use windows::Win32::UI::WindowsAndMessaging::GetWindowRect;
    let mut rect = RECT::default();
    // SAFETY: lectures d’état d’une fenêtre de premier niveau.
    unsafe {
        if !IsWindowVisible(hwnd).as_bool() || IsIconic(hwnd).as_bool() || GetWindowRect(hwnd, &mut rect).is_err() {
            return None;
        }
        let (width, height) = (rect.right - rect.left, rect.bottom - rect.top);
        if width < 200 || height < 150 {
            return None;
        }
        let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
        let mut info = MONITORINFO {
            cbSize: std::mem::size_of::<MONITORINFO>() as u32,
            ..Default::default()
        };
        if !GetMonitorInfoW(monitor, &mut info).as_bool() {
            return None;
        }
        let screen = info.rcMonitor;
        let overlap_w = (rect.right.min(screen.right) - rect.left.max(screen.left)).max(0) as f32;
        let overlap_h = (rect.bottom.min(screen.bottom) - rect.top.max(screen.top)).max(0) as f32;
        let area = ((screen.right - screen.left) * (screen.bottom - screen.top)).max(1) as f32;
        Some(ShownWindow {
            hwnd: hwnd.0 as isize,
            pid: window_pid(hwnd),
            coverage: overlap_w * overlap_h / area,
        })
    }
}

/// Fenêtres affichées de ces processus.
pub fn shown_windows(pids: &[u32]) -> Vec<ShownWindow> {
    top_windows()
        .into_iter()
        .filter(|hwnd| pids.contains(&window_pid(*hwnd)))
        .filter_map(shown)
        .collect()
}

/// Fenêtre au premier plan, si elle est affichée.
pub fn foreground_window() -> Option<ShownWindow> {
    use windows::Win32::UI::WindowsAndMessaging::GetForegroundWindow;
    // SAFETY: simple lecture.
    let hwnd = unsafe { GetForegroundWindow() };
    (!hwnd.is_invalid()).then(|| shown(hwnd)).flatten()
}

/// Réduit une fenêtre sans l'activer.
pub fn minimize_window(raw: isize) {
    let hwnd = HWND(raw as *mut core::ffi::c_void);
    // SAFETY: une fenêtre disparue est ignorée par Windows.
    unsafe {
        let _ = ShowWindow(hwnd, SW_SHOWMINNOACTIVE);
    }
}

/// Laisse n'importe quel programme passer au premier plan : la fenêtre d'un
/// lanceur ou d'un anticheat ouverte par la boutique s'affiche devant Focus.
pub fn allow_any_foreground() {
    use windows::Win32::UI::WindowsAndMessaging::AllowSetForegroundWindow;
    // SAFETY: appel sans pointeur ; ASFW_ANY vaut u32::MAX.
    unsafe {
        let _ = AllowSetForegroundWindow(u32::MAX);
    }
}

/// Met une fenêtre au premier plan (lanceur caché derrière Focus, par exemple).
/// Seule notre file d'entrée est rattachée, jamais celle du programme visé.
pub fn bring_to_front(raw: isize) -> bool {
    use windows::Win32::System::Threading::{AttachThreadInput, GetCurrentThreadId};
    use windows::Win32::UI::Input::KeyboardAndMouse::SetFocus;
    use windows::Win32::UI::WindowsAndMessaging::{BringWindowToTop, GetForegroundWindow, SetForegroundWindow};
    let hwnd = HWND(raw as *mut core::ffi::c_void);
    // SAFETY: fenêtre de premier niveau ; le rattachement est défait avant de sortir.
    unsafe {
        let foreground = GetForegroundWindow();
        let theirs = GetWindowThreadProcessId(foreground, None);
        let mine = GetCurrentThreadId();
        let attached = theirs != 0 && theirs != mine && AttachThreadInput(mine, theirs, true).as_bool();
        let _ = BringWindowToTop(hwnd);
        let done = SetForegroundWindow(hwnd).as_bool();
        if window_pid(hwnd) == std::process::id() {
            let _ = SetFocus(hwnd);
        }
        if attached {
            let _ = AttachThreadInput(mine, theirs, false);
        }
        done
    }
}

/// Ramène la fenêtre de Focus au premier plan, avec le clavier. Windows refuse
/// `SetForegroundWindow` à un programme en arrière-plan quand un jeu a la main :
/// on se rattache un instant à la file d’entrée de la fenêtre active.
pub fn bring_own_to_front() -> bool {
    let own = std::process::id();
    top_windows()
        .into_iter()
        .find(|hwnd| window_pid(*hwnd) == own && is_app_window(*hwnd))
        .is_some_and(|hwnd| bring_to_front(hwnd.0 as isize))
}

/// Rend les fenêtres réduites par `minimize_others`.
pub fn restore_windows(list: &[isize]) {
    for raw in list {
        let hwnd = HWND(*raw as *mut core::ffi::c_void);
        // SAFETY: une fenêtre disparue est ignorée par Windows.
        unsafe {
            if IsIconic(hwnd).as_bool() {
                let _ = ShowWindow(hwnd, SW_SHOWNOACTIVATE);
            }
        }
    }
}

/// Met les fenêtres principales des processus donnés en plein écran sans bordure.
/// Seule la fenêtre change, jamais le processus. Vrai si une fenêtre a été traitée.
pub fn fill_screen(pids: &[u32]) -> bool {
    let frame = WS_CAPTION.0 | WS_THICKFRAME.0 | WS_MINIMIZEBOX.0 | WS_MAXIMIZEBOX.0 | WS_SYSMENU.0;
    let mut done = false;
    for hwnd in top_windows() {
        if !pids.contains(&window_pid(hwnd)) || !is_app_window(hwnd) {
            continue;
        }
        // SAFETY: style et position d’une fenêtre visible.
        unsafe {
            let style = GetWindowLongPtrW(hwnd, GWL_STYLE) as u32;
            if style & WS_CAPTION.0 == 0 {
                continue;
            }
            SetWindowLongPtrW(hwnd, GWL_STYLE, (style & !frame) as isize);
            let monitor = MonitorFromWindow(hwnd, MONITOR_DEFAULTTONEAREST);
            let mut info = MONITORINFO {
                cbSize: std::mem::size_of::<MONITORINFO>() as u32,
                ..Default::default()
            };
            if GetMonitorInfoW(monitor, &mut info).as_bool() {
                let RECT { left, top, right, bottom } = info.rcMonitor;
                let flags = SWP_FRAMECHANGED | SWP_NOZORDER | SWP_NOOWNERZORDER;
                let _ = SetWindowPos(hwnd, None, left, top, right - left, bottom - top, flags);
                done = true;
            }
        }
    }
    done
}


// --- Réglages du compte (autorisés par l’utilisateur le 2026-11-08), effets visuels, mémoire ---

use crate::wide::wide;
use windows::core::PCWSTR;
use windows::Win32::System::Registry::{
    RegDeleteKeyValueW, RegGetValueW, RegSetKeyValueW, HKEY_CURRENT_USER, REG_DWORD, RRF_RT_REG_DWORD,
};

/// Valeur DWORD sous HKCU, ou `None` si elle n’existe pas.
pub fn user_dword(path: &str, name: &str) -> Option<u32> {
    let (path, name) = (wide(path), wide(name));
    let mut value = 0_u32;
    let mut size = 4_u32;
    // SAFETY: tampons locaux de la taille annoncée.
    let status = unsafe {
        RegGetValueW(HKEY_CURRENT_USER, PCWSTR(path.as_ptr()), PCWSTR(name.as_ptr()), RRF_RT_REG_DWORD,
            None, Some((&mut value as *mut u32).cast()), Some(&mut size))
    };
    status.is_ok().then_some(value)
}

/// Écrit (`Some`) ou supprime (`None`) une valeur DWORD sous HKCU.
pub fn set_user_dword(path: &str, name: &str, value: Option<u32>) -> bool {
    let (path, name) = (wide(path), wide(name));
    // SAFETY: tampons locaux vivants pendant l’appel.
    let status = unsafe {
        match value {
            Some(value) => RegSetKeyValueW(HKEY_CURRENT_USER, PCWSTR(path.as_ptr()), PCWSTR(name.as_ptr()),
                REG_DWORD.0, Some((&value as *const u32).cast()), 4),
            None => RegDeleteKeyValueW(HKEY_CURRENT_USER, PCWSTR(path.as_ptr()), PCWSTR(name.as_ptr())),
        }
    };
    status.is_ok()
}

/// Prévient Windows qu’un réglage d’apparence a changé (transparence).
pub fn broadcast_settings() {
    use windows::Win32::Foundation::{LPARAM, WPARAM};
    use windows::Win32::UI::WindowsAndMessaging::{SendMessageTimeoutW, HWND_BROADCAST, SMTO_ABORTIFHUNG, WM_SETTINGCHANGE};
    let area = wide("ImmersiveColorSet");
    // SAFETY: diffusion standard ; le tampon vit pendant l’appel.
    unsafe {
        let _ = SendMessageTimeoutW(HWND_BROADCAST, WM_SETTINGCHANGE, WPARAM(0), LPARAM(area.as_ptr() as isize),
            SMTO_ABORTIFHUNG, 500, None);
    }
}

/// Animations de Windows (fenêtres, menus) : renvoie l’état d’avant. Le changement
/// n’est pas enregistré dans le profil : il disparaît aussi à la fermeture de session.
pub fn set_animations(enabled: bool) -> bool {
    use windows::Win32::UI::WindowsAndMessaging::{
        SystemParametersInfoW, SPI_GETCLIENTAREAANIMATION, SPI_SETCLIENTAREAANIMATION, SPI_SETMENUANIMATION,
        SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS,
    };
    let mut before = windows::Win32::Foundation::BOOL(1);
    let flag = windows::Win32::Foundation::BOOL::from(enabled);
    // SAFETY: valeurs locales ; aucune écriture dans le profil (pas de SPIF_UPDATEINIFILE).
    unsafe {
        let _ = SystemParametersInfoW(SPI_GETCLIENTAREAANIMATION, 0, Some((&mut before as *mut windows::Win32::Foundation::BOOL).cast()),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(0));
        let _ = SystemParametersInfoW(SPI_SETCLIENTAREAANIMATION, 0, Some(flag.0 as usize as *mut core::ffi::c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2));
        let _ = SystemParametersInfoW(SPI_SETMENUANIMATION, 0, Some(flag.0 as usize as *mut core::ffi::c_void),
            SYSTEM_PARAMETERS_INFO_UPDATE_FLAGS(2));
    }
    before.as_bool()
}


/// Priorité de Focus : au-dessus de la normale dans les menus, en dessous pendant
/// un jeu pour ne rien lui prendre. Jamais « temps réel » (gèlerait Windows).
pub fn set_own_priority(in_game: bool) {
    use windows::Win32::System::Threading::{
        GetCurrentProcess, SetPriorityClass, ABOVE_NORMAL_PRIORITY_CLASS, BELOW_NORMAL_PRIORITY_CLASS,
    };
    let class = if in_game { BELOW_NORMAL_PRIORITY_CLASS } else { ABOVE_NORMAL_PRIORITY_CLASS };
    // SAFETY: pseudo-poignée de notre propre processus.
    unsafe {
        let _ = SetPriorityClass(GetCurrentProcess(), class);
    }
}

/// Libère la mémoire vive de Focus quand un jeu démarre.
pub fn trim_self() {
    use windows::Win32::System::ProcessStatus::K32EmptyWorkingSet;
    use windows::Win32::System::Threading::GetCurrentProcess;
    // SAFETY: pseudo-poignée de notre propre processus.
    unsafe {
        let _ = K32EmptyWorkingSet(GetCurrentProcess());
    }
}

/// Libère la mémoire vive des programmes d’arrière-plan (`EmptyWorkingSet`) : les pages
/// reviennent à la demande, rien n’est fermé. `skip` dit quels noms ne jamais toucher
/// (liste blanche, boutiques, jeux). Renvoie le nombre de processus allégés.
pub fn trim_background(skip: impl Fn(&str) -> bool) -> usize {
    use windows::Win32::Foundation::CloseHandle;
    use windows::Win32::System::ProcessStatus::K32EmptyWorkingSet;
    use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_LIMITED_INFORMATION, PROCESS_SET_QUOTA};
    let own = std::process::id();
    let mut count = 0;
    for process in crate::scan::processes() {
        if process.pid == own || process.pid <= 4 || skip(&process.name) {
            continue;
        }
        // SAFETY: poignée limitée, fermée aussitôt.
        unsafe {
            if let Ok(handle) = OpenProcess(PROCESS_SET_QUOTA | PROCESS_QUERY_LIMITED_INFORMATION, false, process.pid) {
                if K32EmptyWorkingSet(handle).as_bool() {
                    count += 1;
                }
                let _ = CloseHandle(handle);
            }
        }
    }
    count
}


/// Capture de l’écran principal, en RGBA. Un jeu en plein écran exclusif peut
/// donner une image noire : Windows ne la partage pas.
pub fn capture_screen() -> Option<crate::icon::Rgba> {
    use windows::Win32::Graphics::Gdi::{
        BitBlt, CreateCompatibleBitmap, CreateCompatibleDC, DeleteDC, DeleteObject, GetDC, GetDIBits,
        ReleaseDC, SelectObject, BITMAPINFO, BITMAPINFOHEADER, BI_RGB, DIB_RGB_COLORS, SRCCOPY,
    };
    use windows::Win32::UI::WindowsAndMessaging::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};
    // SAFETY: objets GDI locaux, tous libérés avant de rendre la main.
    unsafe {
        let (w, h) = (GetSystemMetrics(SM_CXSCREEN), GetSystemMetrics(SM_CYSCREEN));
        if w <= 0 || h <= 0 {
            return None;
        }
        let screen = GetDC(None);
        let memory = CreateCompatibleDC(screen);
        let bitmap = CreateCompatibleBitmap(screen, w, h);
        let old = SelectObject(memory, bitmap);
        let copied = BitBlt(memory, 0, 0, w, h, screen, 0, 0, SRCCOPY).is_ok();
        let mut info = BITMAPINFO {
            bmiHeader: BITMAPINFOHEADER {
                biSize: std::mem::size_of::<BITMAPINFOHEADER>() as u32,
                biWidth: w,
                biHeight: -h,
                biPlanes: 1,
                biBitCount: 32,
                biCompression: BI_RGB.0,
                ..Default::default()
            },
            ..Default::default()
        };
        let mut pixels = vec![0_u8; (w as usize) * (h as usize) * 4];
        SelectObject(memory, old);
        let lines = GetDIBits(memory, bitmap, 0, h as u32, Some(pixels.as_mut_ptr().cast()), &mut info, DIB_RGB_COLORS);
        let _ = DeleteObject(bitmap);
        let _ = DeleteDC(memory);
        ReleaseDC(None, screen);
        if !copied || lines == 0 {
            return None;
        }
        for pixel in pixels.as_chunks_mut::<4>().0 {
            pixel.swap(0, 2);
            pixel[3] = 255;
        }
        Some(crate::icon::Rgba { width: w as u32, height: h as u32, pixels })
    }
}

/// Met le PC en veille, comme le mode repos d’une console.
pub fn sleep_now() -> bool {
    use windows::Win32::System::Power::SetSuspendState;
    // SAFETY: demande de veille simple, sans hibernation forcée.
    unsafe { SetSuspendState(false, false, false).as_bool() }
}

/// Réduit les fenêtres des processus nommés (boutiques) sans les fermer.
pub fn minimize_processes(names: &[&str]) -> usize {
    // Noms seulement : aucun processus n'est ouvert (SPEC D7).
    let pids: Vec<u32> = crate::scan::process_ids()
        .into_iter()
        .filter(|(_, process)| names.iter().any(|name| process.eq_ignore_ascii_case(name)))
        .map(|(pid, _)| pid)
        .collect();
    let mut count = 0;
    for hwnd in top_windows() {
        // SAFETY: lecture d’état puis réduction sans activation.
        let iconic = unsafe { IsIconic(hwnd).as_bool() };
        if !iconic && is_app_window(hwnd) && pids.contains(&window_pid(hwnd)) {
            unsafe {
                let _ = ShowWindow(hwnd, SW_SHOWMINNOACTIVE);
            }
            count += 1;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filetime_should_be_after_2020() {
        // 1er janvier 2020 en intervalles de 100 ns depuis 1601.
        assert!(filetime_now() > 132_223_104_000_000_000);
    }

    #[test]
    fn keep_awake_should_start_and_stop_without_blocking() {
        let guard = KeepAwake::start();
        drop(guard);
    }
}
