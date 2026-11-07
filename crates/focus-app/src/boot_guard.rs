//! Garde-fou : deux plantages de suite au démarrage coupent le lancement automatique.
//!
//! La récupération de session a déjà tourné avant ce code : désactiver le
//! lancement automatique n'empêche donc jamais la restauration (SPEC D6).

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::time::Duration;

const MAX_FAILED_BOOTS: u32 = 2;
const STABLE_AFTER: Duration = Duration::from_secs(30);

/// Information de démarrage pour l'interface.
#[derive(Debug, Clone, Copy, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootStatus {
    /// Le lancement automatique vient d'être désactivé par le garde-fou.
    pub autostart_disabled: bool,
}

fn counter_file(data: &Path) -> PathBuf {
    data.join("boot-attempts")
}

/// Décision pure, testée à part.
fn next_attempts(previous: u32) -> (u32, bool) {
    if previous >= MAX_FAILED_BOOTS {
        (0, true)
    } else {
        (previous + 1, false)
    }
}

/// Note un démarrage, et remet le compteur à zéro si l'application tient.
pub fn on_start(data: &Path) -> BootStatus {
    let file = counter_file(data);
    let previous = std::fs::read_to_string(&file)
        .ok()
        .and_then(|text| text.trim().parse().ok())
        .unwrap_or(0);
    let (attempts, disable) = next_attempts(previous);
    if let Err(err) = std::fs::write(&file, attempts.to_string()) {
        tracing::warn!("boot counter not written: {err}");
    }
    let mut status = BootStatus::default();
    if disable {
        #[cfg(windows)]
        if let Ok(exe) = std::env::current_exe() {
            if focus_sys::shell::autostart_enabled()
                && focus_sys::shell::set_autostart(false, &exe).is_ok()
            {
                status.autostart_disabled = true;
            }
        }
    }
    std::thread::spawn(move || {
        std::thread::sleep(STABLE_AFTER);
        let _ = std::fs::remove_file(file);
    });
    status
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_crash_should_not_disable_autostart() {
        assert_eq!(next_attempts(1), (2, false));
    }

    #[test]
    fn second_crash_in_a_row_should_disable_autostart() {
        assert_eq!(next_attempts(2), (0, true));
    }
}
