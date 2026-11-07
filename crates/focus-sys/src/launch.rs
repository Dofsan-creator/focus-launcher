//! Lancement des entrées, et signes d'anticheat.

use crate::wide::wide;
use focus_core::model::LaunchTarget;
use std::path::Path;
use std::process::{Child, Command};
use thiserror::Error;
use windows::core::PCWSTR;
use windows::Win32::UI::Shell::ShellExecuteW;
use windows::Win32::UI::WindowsAndMessaging::SW_SHOWNORMAL;

/// Erreur de lancement.
#[derive(Debug, Error)]
pub enum LaunchError {
    /// Programme introuvable ou refusé.
    #[error("cannot start: {0}")]
    Spawn(#[from] std::io::Error),
    /// Adresse non autorisée.
    #[error("refused address: {0}")]
    Refused(String),
    /// Échec de `ShellExecuteW`.
    #[error("shell refused the request ({0})")]
    Shell(isize),
}

/// Schémas que le launcher accepte d'ouvrir.
const ALLOWED_SCHEMES: &[&str] = &[
    "steam://",
    "com.epicgames.launcher://",
    "uplay://",
    "battlenet://",
    "link2ea://",
];

/// Ouvre une adresse par l'association de Windows.
fn shell_open(target: &str) -> Result<(), LaunchError> {
    let (open, target) = (wide("open"), wide(target));
    // SAFETY: les tampons vivent pendant l'appel synchrone.
    let result = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(open.as_ptr()),
            PCWSTR(target.as_ptr()),
            None,
            None,
            SW_SHOWNORMAL,
        )
    };
    if result.0 as isize > 32 {
        Ok(())
    } else {
        Err(LaunchError::Shell(result.0 as isize))
    }
}

/// Démarre une entrée. Renvoie le processus enfant s'il est direct.
pub fn start(target: &LaunchTarget) -> Result<Option<Child>, LaunchError> {
    match target {
        LaunchTarget::Executable {
            path,
            args,
            working_dir,
        } => {
            let mut command = Command::new(path);
            command.args(args);
            if let Some(dir) = working_dir {
                command.current_dir(dir);
            }
            Ok(Some(command.spawn()?))
        }
        LaunchTarget::Protocol { uri } => {
            if !ALLOWED_SCHEMES.iter().any(|scheme| uri.starts_with(scheme)) {
                return Err(LaunchError::Refused(uri.clone()));
            }
            shell_open(uri)?;
            Ok(None)
        }
        LaunchTarget::Packaged { aumid } => {
            Command::new("explorer.exe")
                .arg(format!("shell:AppsFolder\\{aumid}"))
                .spawn()?;
            Ok(None)
        }
        LaunchTarget::Website { url } => {
            if !url.starts_with("https://") {
                return Err(LaunchError::Refused(url.clone()));
            }
            shell_open(url)?;
            Ok(None)
        }
    }
}

/// Lance un programme avec élévation : Windows affiche son autorisation.
pub fn run_elevated(exe: &Path, args: &str) -> Result<(), LaunchError> {
    let (verb, file, params) = (wide("runas"), wide(exe), wide(args));
    // SAFETY: les tampons vivent pendant l'appel synchrone.
    let result = unsafe {
        ShellExecuteW(
            None,
            PCWSTR(verb.as_ptr()),
            PCWSTR(file.as_ptr()),
            PCWSTR(params.as_ptr()),
            None,
            SW_SHOWNORMAL,
        )
    };
    if result.0 as isize > 32 {
        Ok(())
    } else {
        Err(LaunchError::Shell(result.0 as isize))
    }
}

/// Fichiers qui révèlent un anticheat dans le dossier d'un jeu.
const ANTICHEAT_MARKERS: &[&str] = &[
    "EasyAntiCheat",
    "BattlEye",
    "EasyAntiCheat_EOS.sys",
    "BEService.exe",
    "vanguard",
    "PnkBstrA.exe",
    "nProtect",
    "xigncode",
];

/// Un anticheat est-il présent ? En cas de doute, le launcher n'ouvre rien sur le jeu (SPEC D7).
pub fn has_anticheat(install_dir: &Path) -> bool {
    let Ok(entries) = std::fs::read_dir(install_dir) else {
        return true;
    };
    entries.flatten().any(|entry| {
        let name = entry.file_name().to_string_lossy().to_lowercase();
        ANTICHEAT_MARKERS
            .iter()
            .any(|marker| name.contains(&marker.to_lowercase()))
    })
}
