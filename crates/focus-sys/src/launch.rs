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

/// Ouvre une adresse ou un raccourci avec les droits normaux de l'utilisateur, par
/// l'Explorateur ; à défaut, directement.
fn open_as_user(target: &str) -> Result<(), LaunchError> {
    if crate::unelevated::open(target, "", "") {
        return Ok(());
    }
    shell_open(target)
}

/// Arguments en une ligne de commande Windows (guillemets si espaces).
fn command_line(args: &[String]) -> String {
    args.iter()
        .map(|arg| if arg.contains(' ') && !arg.starts_with('"') { format!("\"{arg}\"") } else { arg.clone() })
        .collect::<Vec<_>>()
        .join(" ")
}

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
            let lower = path.to_lowercase();
            // Raccourcis : c’est le shell de Windows qui sait les ouvrir.
            if lower.ends_with(".lnk") || lower.ends_with(".url") {
                open_as_user(path)?;
                return Ok(None);
            }
            let mut command = Command::new(path);
            command.args(args);
            // Beaucoup de jeux cherchent leurs fichiers à côté de l’exécutable.
            let dir = working_dir
                .clone()
                .or_else(|| Path::new(path).parent().map(|p| p.display().to_string()));
            // Focus tourne en administrateur : seuls les programmes qui l'exigent
            // (beaucoup de jeux gacha) en héritent, sans nouvelle invite. Les autres
            // partent avec les droits normaux, par l'Explorateur.
            if !crate::unelevated::requires_admin(Path::new(path))
                && crate::unelevated::open(path, &command_line(args), dir.as_deref().unwrap_or(""))
            {
                return Ok(None);
            }
            if let Some(dir) = dir {
                command.current_dir(dir);
            }
            match command.spawn() {
                Ok(child) => Ok(Some(child)),
                // 740 : le programme exige les droits administrateur ; Windows demande l’accord.
                Err(err) if err.raw_os_error() == Some(740) => {
                    run_elevated(Path::new(path), &args.join(" "))?;
                    Ok(None)
                }
                Err(err) => Err(err.into()),
            }
        }
        LaunchTarget::Protocol { uri } => {
            if !ALLOWED_SCHEMES.iter().any(|scheme| uri.starts_with(scheme)) {
                return Err(LaunchError::Refused(uri.clone()));
            }
            open_as_user(uri)?;
            Ok(None)
        }
        LaunchTarget::Packaged { aumid } => {
            // Par l'association de Windows, pas par `explorer.exe` : lancé sans bureau,
            // celui-ci deviendrait le bureau et sortirait du mode console.
            open_as_user(&format!("shell:AppsFolder\\{aumid}"))?;
            Ok(None)
        }
        LaunchTarget::Website { url } => {
            if !url.starts_with("https://") {
                return Err(LaunchError::Refused(url.clone()));
            }
            open_as_user(url)?;
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
    "AntiCheatExpert",
    "ACE-Service",
    "mhyprot",
    "EAAntiCheat",
];

/// Un anticheat est-il présent ? En cas de doute, le launcher n'ouvre rien sur le jeu (SPEC D7).
pub fn has_anticheat(install_dir: &Path) -> bool {
    // Les jeux rangent leur anticheat loin de la racine
    // (`Jeu/Binaries/Win64/EasyAntiCheat`, `Client/Binaries/Win64/AntiCheatExpert`).
    fn search(dir: &Path, depth: usize) -> bool {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return false;
        };
        entries.flatten().any(|entry| {
            let name = entry.file_name().to_string_lossy().to_lowercase();
            ANTICHEAT_MARKERS.iter().any(|marker| name.contains(&marker.to_lowercase()))
                || (depth > 0 && entry.path().is_dir() && search(&entry.path(), depth - 1))
        })
    }
    if std::fs::read_dir(install_dir).is_err() {
        return true;
    }
    search(install_dir, 6)
}

#[cfg(test)]
mod tests {
    use super::has_anticheat;

    #[test]
    fn anticheat_deep_in_the_game_folder_should_be_found() -> std::io::Result<()> {
        let game = tempfile::tempdir()?;
        let deep = game.path().join("Wuthering Waves Game/Client/Binaries/Win64/AntiCheatExpert");
        std::fs::create_dir_all(&deep)?;
        std::fs::write(deep.join("ACE-Service64.exe"), b"")?;
        assert!(has_anticheat(game.path()));
        Ok(())
    }

    #[test]
    fn game_without_anticheat_should_be_seen_as_such() -> std::io::Result<()> {
        let game = tempfile::tempdir()?;
        std::fs::create_dir_all(game.path().join("Binaries/Win64"))?;
        std::fs::write(game.path().join("Binaries/Win64/Game.exe"), b"")?;
        assert!(!has_anticheat(game.path()));
        Ok(())
    }

    #[test]
    fn unreadable_folder_should_count_as_protected() {
        assert!(has_anticheat(std::path::Path::new("Z:/nowhere/at/all")));
    }
}
