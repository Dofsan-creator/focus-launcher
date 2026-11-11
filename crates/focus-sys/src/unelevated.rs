//! Lancement sans droits administrateur depuis un Focus élevé.
//!
//! Focus tourne en administrateur pour piloter les jeux qui l'exigent. Ce qu'il
//! lance par ailleurs (boutiques, navigateur, jeux ordinaires, applications
//! relancées) ne doit pas hériter de ces droits : on demande à l'Explorateur, qui
//! tourne avec les droits normaux de l'utilisateur, de l'ouvrir à notre place.
//! Méthode documentée par Microsoft (bureau du Shell, `IShellDispatch2::ShellExecute`).

use crate::wide::wide;
use windows::core::{Interface, BSTR, PCWSTR, PWSTR, VARIANT};
use windows::Win32::Foundation::{CloseHandle, HANDLE};
use windows::Win32::Security::{
    DuplicateTokenEx, GetTokenInformation, SecurityImpersonation, TokenElevation, TokenPrimary,
    TOKEN_ADJUST_DEFAULT, TOKEN_ADJUST_SESSIONID, TOKEN_ASSIGN_PRIMARY, TOKEN_DUPLICATE, TOKEN_ELEVATION,
    TOKEN_QUERY,
};
use windows::Win32::System::Threading::{
    CreateProcessWithTokenW, OpenProcess, OpenProcessToken, CREATE_PROCESS_LOGON_FLAGS,
    PROCESS_CREATION_FLAGS, PROCESS_INFORMATION, PROCESS_QUERY_LIMITED_INFORMATION, STARTUPINFOW,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoUninitialize, IDispatch, IServiceProvider, CLSCTX_LOCAL_SERVER,
    COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::{
    IShellBrowser, IShellDispatch2, IShellFolderViewDual, IShellView, IShellWindows, ShellWindows,
    SID_STopLevelBrowser, SVGIO_BACKGROUND, SWC_DESKTOP, SWFO_NEEDDISPATCH,
};

fn through_explorer(file: &str, args: &str, dir: &str) -> windows::core::Result<()> {
    // SAFETY: appels COM sur le fil courant, initialisé par l'appelant.
    unsafe {
        let windows: IShellWindows = CoCreateInstance(&ShellWindows, None, CLSCTX_LOCAL_SERVER)?;
        let desktop = VARIANT::from(0_i32);
        let mut hwnd = 0_i32;
        let dispatch: IDispatch =
            windows.FindWindowSW(&desktop, &VARIANT::default(), SWC_DESKTOP, &mut hwnd, SWFO_NEEDDISPATCH)?;
        let provider: IServiceProvider = dispatch.cast()?;
        let browser: IShellBrowser = provider.QueryService(&SID_STopLevelBrowser)?;
        let view: IShellView = browser.QueryActiveShellView()?;
        let background: IDispatch = view.GetItemObject(SVGIO_BACKGROUND)?;
        let folder: IShellFolderViewDual = background.cast()?;
        let shell: IShellDispatch2 = folder.Application()?.cast()?;
        shell.ShellExecute(
            &BSTR::from(file),
            &VARIANT::from(BSTR::from(args)),
            &VARIANT::from(BSTR::from(dir)),
            &VARIANT::from(BSTR::from("open")),
            &VARIANT::from(1_i32),
        )
    }
}

/// Programmes de la session qui tournent toujours avec les droits normaux de
/// l'utilisateur, même quand le bureau est arrêté.
const DONORS: [&str; 4] = ["sihost", "taskhostw", "ctfmon", "runtimebroker"];

/// Copie du jeton d'un programme ordinaire de la session. Sans bureau, c'est lui
/// qui donne les droits normaux à ce que Focus lance. Un jeton élevé est refusé.
fn user_token() -> Option<HANDLE> {
    let session = crate::scan::session_of(std::process::id())?;
    let access = TOKEN_QUERY | TOKEN_DUPLICATE | TOKEN_ASSIGN_PRIMARY | TOKEN_ADJUST_DEFAULT | TOKEN_ADJUST_SESSIONID;
    for (pid, name) in crate::scan::process_ids() {
        if !DONORS.contains(&name.as_str()) || crate::scan::session_of(pid) != Some(session) {
            continue;
        }
        // SAFETY: handles locaux, tous fermés ; tampons de la taille annoncée.
        unsafe {
            let Ok(process) = OpenProcess(PROCESS_QUERY_LIMITED_INFORMATION, false, pid) else {
                continue;
            };
            let mut token = HANDLE::default();
            let opened = OpenProcessToken(process, TOKEN_DUPLICATE | TOKEN_QUERY, &mut token);
            let _ = CloseHandle(process);
            if opened.is_err() {
                continue;
            }
            let mut elevation = TOKEN_ELEVATION::default();
            let mut size = 0_u32;
            let ordinary = GetTokenInformation(
                token,
                TokenElevation,
                Some(std::ptr::from_mut(&mut elevation).cast()),
                std::mem::size_of::<TOKEN_ELEVATION>() as u32,
                &mut size,
            )
            .is_ok()
                && elevation.TokenIsElevated == 0;
            let mut primary = HANDLE::default();
            let copied = ordinary
                && DuplicateTokenEx(token, access, None, SecurityImpersonation, TokenPrimary, &mut primary).is_ok();
            let _ = CloseHandle(token);
            if copied {
                return Some(primary);
            }
        }
    }
    None
}

/// Lance une ligne de commande avec les droits normaux de l'utilisateur, sans passer
/// par le bureau. Renvoie le pid, ou `None` si aucun jeton ordinaire n'est disponible.
pub fn spawn_as_user(command_line: &str, dir: Option<&str>) -> Option<u32> {
    let token = user_token()?;
    let mut command = wide(command_line);
    let dir = dir.filter(|dir| !dir.is_empty()).map(wide);
    let startup = STARTUPINFOW {
        cb: std::mem::size_of::<STARTUPINFOW>() as u32,
        ..Default::default()
    };
    let mut info = PROCESS_INFORMATION::default();
    // SAFETY: tampons locaux vivants pendant l'appel ; handles fermés ensuite.
    unsafe {
        let result = CreateProcessWithTokenW(
            token,
            CREATE_PROCESS_LOGON_FLAGS(0),
            PCWSTR::null(),
            PWSTR(command.as_mut_ptr()),
            PROCESS_CREATION_FLAGS(0),
            None,
            dir.as_ref().map_or(PCWSTR::null(), |dir| PCWSTR(dir.as_ptr())),
            &startup,
            &mut info,
        );
        let _ = CloseHandle(token);
        match result {
            Ok(()) => {
                let _ = CloseHandle(info.hThread);
                let _ = CloseHandle(info.hProcess);
                Some(info.dwProcessId)
            }
            Err(err) => {
                tracing::warn!("launch with ordinary rights failed for {command_line}: {err}");
                None
            }
        }
    }
}

/// Ligne de commande qui ouvre `file` : directe pour un programme, par
/// l'association de Windows pour une adresse, un raccourci ou un document.
fn command_for(file: &str, args: &str) -> String {
    let lower = file.to_lowercase();
    if [".exe", ".bat", ".cmd", ".com"].iter().any(|ext| lower.ends_with(ext)) {
        return format!("\"{file}\" {args}").trim_end().to_owned();
    }
    let root = std::env::var("SystemRoot").unwrap_or_else(|_| "C:\\Windows".to_owned());
    format!("\"{}\\System32\\rundll32.exe\" url.dll,FileProtocolHandler {file}", root.trim_end_matches('\\'))
}

/// Le bureau de Windows répond-il ? Sans lui, on ne lui demande rien : l'appel
/// pourrait le relancer.
fn desktop_alive() -> bool {
    // SAFETY: simple lecture.
    !unsafe { windows::Win32::UI::WindowsAndMessaging::GetShellWindow() }.is_invalid()
}

/// Ouvre `file` (programme, adresse ou raccourci) avec les droits normaux de
/// l'utilisateur : par l'Explorateur s'il tourne, sinon avec le jeton d'un programme
/// ordinaire de la session (bureau arrêté par le mode console). Faux si aucune des
/// deux voies n'a marché.
pub fn open(file: &str, args: &str, dir: &str) -> bool {
    if !desktop_alive() {
        return spawn_as_user(&command_for(file, args), Some(dir)).is_some();
    }
    let (file, args, dir) = (file.to_owned(), args.to_owned(), dir.to_owned());
    // Fil dédié : COM y est initialisé en appartement, sans toucher aux autres.
    std::thread::spawn(move || {
        // SAFETY: initialisation et libération COM appariées sur ce fil.
        let initialized = unsafe { CoInitializeEx(None, COINIT_APARTMENTTHREADED) }.is_ok();
        let result = through_explorer(&file, &args, &dir);
        if initialized {
            // SAFETY: appariée avec `CoInitializeEx` ci-dessus.
            unsafe { CoUninitialize() };
        }
        if let Err(err) = &result {
            tracing::warn!("unelevated launch failed for {file}: {err}");
        }
        result.is_ok()
    })
    .join()
    .unwrap_or(false)
}

/// L'exécutable exige-t-il les droits administrateur (manifeste) ? Lecture seule,
/// par morceaux : un jeu peut peser plusieurs centaines de Mo.
pub fn requires_admin(path: &std::path::Path) -> bool {
    use std::io::Read;
    const NEEDLES: [&[u8]; 2] = [b"requireAdministrator", b"highestAvailable"];
    let Ok(mut file) = std::fs::File::open(path) else { return false };
    let mut buffer = vec![0_u8; 1 << 20];
    let mut carry: Vec<u8> = Vec::new();
    loop {
        let Ok(read) = file.read(&mut buffer) else { return false };
        if read == 0 {
            return false;
        }
        let mut window = std::mem::take(&mut carry);
        window.extend_from_slice(&buffer[..read]);
        if NEEDLES.iter().any(|needle| window.windows(needle.len()).any(|chunk| chunk == *needle)) {
            return true;
        }
        // Garde la fin, au cas où le mot chevauche deux morceaux.
        carry = window[window.len().saturating_sub(32)..].to_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::command_for;

    #[test]
    fn program_should_be_started_directly_with_its_arguments() {
        assert_eq!(command_for("D:\\Games\\My Game\\game.exe", "-windowed"), "\"D:\\Games\\My Game\\game.exe\" -windowed");
        assert_eq!(command_for("D:\\Games\\game.EXE", ""), "\"D:\\Games\\game.EXE\"");
    }

    #[test]
    fn address_or_shortcut_should_go_through_the_windows_association() {
        for target in ["steam://rungameid/10", "C:\\Users\\me\\Desktop\\Game.lnk", "https://example.org"] {
            let line = command_for(target, "ignored");
            assert!(line.ends_with(&format!("url.dll,FileProtocolHandler {target}")), "{line}");
            assert!(line.to_lowercase().contains("\\system32\\rundll32.exe\""), "{line}");
        }
    }
}
