//! Raccourcis, démarrage de session et informations de fichier. Aucun registre.

use crate::wide::{from_wide, wide};
use std::path::{Path, PathBuf};
use windows::core::{Interface, GUID, PCWSTR, PWSTR};
use windows::Win32::Foundation::{BOOL, HWND, LPARAM};
use windows::Win32::Storage::FileSystem::{
    GetFileVersionInfoSizeW, GetFileVersionInfoW, VerQueryValueW,
};
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CoTaskMemFree, IPersistFile, CLSCTX_INPROC_SERVER,
    COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::{
    FOLDERID_Programs, FOLDERID_Startup, IShellLinkW, SHGetKnownFolderPath, ShellLink,
    KF_FLAG_DEFAULT,
};
use windows::Win32::UI::WindowsAndMessaging::{EnumWindows, GetWindowTextW, IsWindowVisible};

/// Nom du raccourci de démarrage.
const STARTUP_LINK: &str = "Focus Launcher.lnk";

/// Dossier connu de Windows, jamais par un chemin écrit en dur.
fn known_folder(id: &GUID) -> Result<PathBuf, windows::core::Error> {
    // SAFETY: Windows alloue la chaîne, copiée puis libérée ici.
    unsafe {
        let path: PWSTR = SHGetKnownFolderPath(id, KF_FLAG_DEFAULT, None)?;
        let text = path.to_string().unwrap_or_default();
        CoTaskMemFree(Some(path.0.cast()));
        Ok(PathBuf::from(text))
    }
}

/// Crée un raccourci `.lnk`.
pub fn create_shortcut(link: &Path, target: &Path, args: &str) -> Result<(), windows::core::Error> {
    let (target, args, link) = (wide(target), wide(args), wide(link));
    // SAFETY: les tampons vivent jusqu'à la fin ; COM est initialisé sur ce fil.
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let shell: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_INPROC_SERVER)?;
        shell.SetPath(PCWSTR(target.as_ptr()))?;
        shell.SetArguments(PCWSTR(args.as_ptr()))?;
        let file: IPersistFile = shell.cast()?;
        file.Save(PCWSTR(link.as_ptr()), true)
    }
}

/// Tâche planifiée qui ouvre Focus à l'ouverture de session.
const STARTUP_TASK: &str = "Focus Launcher";

fn schtasks(args: &[&str]) -> std::io::Result<bool> {
    use std::os::windows::process::CommandExt;
    const CREATE_NO_WINDOW: u32 = 0x0800_0000;
    let status = std::process::Command::new("schtasks.exe")
        .args(args)
        .creation_flags(CREATE_NO_WINDOW)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()?;
    Ok(status.success())
}

/// Focus s'ouvre-t-il à l'ouverture de session ?
pub fn autostart_enabled() -> bool {
    schtasks(&["/Query", "/TN", STARTUP_TASK]).unwrap_or(false)
}

/// Active ou retire l'ouverture à l'ouverture de session. Focus exige les droits
/// administrateur : Windows ne lance pas un tel programme depuis le dossier
/// Démarrage, d'où une tâche planifiée « privilèges les plus élevés », limitée à
/// l'utilisateur courant et à sa session interactive, sans invite.
pub fn set_autostart(enabled: bool, exe: &Path) -> Result<(), windows::core::Error> {
    // L'ancien raccourci du dossier Démarrage disparaît dans les deux cas.
    if let Ok(dir) = known_folder(&FOLDERID_Startup) {
        let _ = std::fs::remove_file(dir.join(STARTUP_LINK));
    }
    if enabled {
        let command = format!("\"{}\" --autostart", exe.display());
        let user = match (std::env::var("USERDOMAIN"), std::env::var("USERNAME")) {
            (Ok(domain), Ok(name)) => format!("{domain}\\{name}"),
            (_, Ok(name)) => name,
            _ => String::new(),
        };
        let mut args = vec!["/Create", "/F", "/TN", STARTUP_TASK, "/TR", command.as_str(), "/SC", "ONLOGON", "/RL", "HIGHEST"];
        if !user.is_empty() {
            args.extend(["/RU", user.as_str(), "/IT"]);
        }
        if !schtasks(&args)? {
            return Err(std::io::Error::other("schtasks /Create failed").into());
        }
    } else if autostart_enabled() && !schtasks(&["/Delete", "/F", "/TN", STARTUP_TASK])? {
        return Err(std::io::Error::other("schtasks /Delete failed").into());
    }
    Ok(())
}

/// Crée un raccourci d'entrée dans « Focus Launcher » du Menu Démarrer.
pub fn add_start_menu_entry(
    title: &str,
    entry_id: &str,
    exe: &Path,
) -> Result<(), windows::core::Error> {
    let dir = known_folder(&FOLDERID_Programs)?.join("Focus Launcher");
    std::fs::create_dir_all(&dir)?;
    let safe: String = title
        .chars()
        .filter(|c| !r#"<>:"/\|?*"#.contains(*c))
        .collect();
    create_shortcut(
        &dir.join(format!("{safe}.lnk")),
        exe,
        &format!("--launch \"{entry_id}\""),
    )
}

/// Titres de fenêtre de Big Picture, repérés par leur début, Steam traduisant la fin.
const BIG_PICTURE_TITLES: &[&str] = &["Steam Big Picture", "Big Picture"];

unsafe extern "system" fn find_big_picture(window: HWND, param: LPARAM) -> BOOL {
    let mut buffer = [0_u16; 256];
    // SAFETY: `buffer` est local et `param` pointe sur le booléen de l'appelant.
    unsafe {
        let size = GetWindowTextW(window, &mut buffer);
        let title = String::from_utf16_lossy(&buffer[..size.max(0) as usize]);
        if IsWindowVisible(window).as_bool()
            && BIG_PICTURE_TITLES.iter().any(|t| title.starts_with(t))
        {
            *(param.0 as *mut bool) = true;
            return false.into();
        }
    }
    true.into()
}

/// La fenêtre Big Picture est-elle ouverte ? Une détection ratée n'est pas une erreur.
pub fn big_picture_open() -> bool {
    let mut found = false;
    // SAFETY: `found` survit à l'appel synchrone.
    let _ = unsafe {
        EnumWindows(
            Some(find_big_picture),
            LPARAM(std::ptr::from_mut(&mut found) as isize),
        )
    };
    found
}

/// Nom de produit d'un exécutable. Le fichier n'est lu, jamais exécuté.
pub fn product_name(path: &Path) -> Option<String> {
    let file = wide(path);
    // SAFETY: le tampon a la taille annoncée par Windows ; `value` pointe dedans.
    unsafe {
        let size = GetFileVersionInfoSizeW(PCWSTR(file.as_ptr()), None);
        if size == 0 {
            return None;
        }
        let mut data = vec![0_u8; size as usize];
        GetFileVersionInfoW(PCWSTR(file.as_ptr()), 0, size, data.as_mut_ptr().cast()).ok()?;
        let query = wide("\\VarFileInfo\\Translation");
        let (mut pointer, mut length) = (std::ptr::null_mut(), 0_u32);
        if !VerQueryValueW(
            data.as_ptr().cast(),
            PCWSTR(query.as_ptr()),
            &mut pointer,
            &mut length,
        )
        .as_bool()
            || (length as usize) < 4
        {
            return None;
        }
        let codes = std::slice::from_raw_parts(pointer.cast::<u16>(), 2);
        let query = wide(format!(
            "\\StringFileInfo\\{:04x}{:04x}\\ProductName",
            codes[0], codes[1]
        ));
        if !VerQueryValueW(
            data.as_ptr().cast(),
            PCWSTR(query.as_ptr()),
            &mut pointer,
            &mut length,
        )
        .as_bool()
        {
            return None;
        }
        let text = std::slice::from_raw_parts(pointer.cast::<u16>(), length as usize);
        let name = from_wide(text).trim().to_owned();
        (!name.is_empty()).then_some(name)
    }
}
