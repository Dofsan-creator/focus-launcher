//! Garde-fou : deux démarrages ratés de suite coupent le lancement automatique.
//!
//! Un démarrage est raté si le processus meurt avant `STABLE_AFTER`, ou si
//! l'interface ne s'annonce pas avant `UI_TIMEOUT` (écran noir). Dans ce second
//! cas, la version publiée se ferme seule : une fenêtre plein écran vide ne doit
//! jamais bloquer quelqu'un qui n'a qu'une manette en main.
//!
//! La récupération de session a déjà tourné avant ce code : désactiver le
//! lancement automatique n'empêche donc jamais la restauration (SPEC D6).

use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

const MAX_FAILED_BOOTS: u32 = 2;
const STABLE_AFTER: Duration = Duration::from_secs(30);
/// Large : WebView2 peut être lent à l'ouverture de session sur un disque dur.
const UI_TIMEOUT: Duration = Duration::from_secs(60);
const POLL: Duration = Duration::from_millis(250);

/// Information de démarrage pour l'interface.
#[derive(Debug, Clone, Copy, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BootStatus {
    /// Le lancement automatique vient d'être désactivé par le garde-fou.
    pub autostart_disabled: bool,
}

/// Levé par le premier appel de `boot_info` : l'interface WebAssembly tourne.
#[derive(Debug, Clone, Default)]
pub struct UiStarted(Arc<AtomicBool>);

impl UiStarted {
    /// Note que l'interface a démarré. Vrai au premier appel seulement.
    pub fn mark(&self) -> bool {
        !self.0.swap(true, Ordering::SeqCst)
    }

    fn is_set(&self) -> bool {
        self.0.load(Ordering::SeqCst)
    }
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

/// État d'un démarrage, selon le temps écoulé et l'interface.
#[derive(Debug, PartialEq, Eq)]
enum Outcome {
    Waiting,
    Stable,
    UiMissing,
}

/// Décision pure, testée à part.
fn outcome(elapsed: Duration, ui_started: bool) -> Outcome {
    if ui_started && elapsed >= STABLE_AFTER {
        Outcome::Stable
    } else if !ui_started && elapsed >= UI_TIMEOUT {
        Outcome::UiMissing
    } else {
        Outcome::Waiting
    }
}

/// Note un démarrage. `watch` remet ensuite le compteur à zéro si tout tient.
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
    status
}

/// Surveille le démarrage : compteur effacé si l'interface tient, `close`
/// appelée si elle ne s'annonce jamais. Le compteur reste alors : ce démarrage
/// compte comme raté.
pub fn watch(data: &Path, ui: UiStarted, close: impl FnOnce() + Send + 'static) {
    let file = counter_file(data);
    let started = Instant::now();
    std::thread::spawn(move || loop {
        match outcome(started.elapsed(), ui.is_set()) {
            Outcome::Waiting => std::thread::sleep(POLL),
            Outcome::Stable => {
                let _ = std::fs::remove_file(&file);
                return;
            }
            Outcome::UiMissing => {
                tracing::error!(
                    "interface did not start within {} s",
                    UI_TIMEOUT.as_secs()
                );
                close();
                return;
            }
        }
    });
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

    #[test]
    fn boot_should_be_stable_only_once_the_interface_has_started() {
        assert_eq!(outcome(STABLE_AFTER, false), Outcome::Waiting);
        assert_eq!(outcome(STABLE_AFTER, true), Outcome::Stable);
        assert_eq!(outcome(Duration::from_secs(1), true), Outcome::Waiting);
    }

    #[test]
    fn interface_that_never_starts_should_count_as_a_failed_boot() {
        assert_eq!(outcome(UI_TIMEOUT, false), Outcome::UiMissing);
        assert_eq!(outcome(UI_TIMEOUT, true), Outcome::Stable);
    }

    #[test]
    fn interface_should_be_marked_started_once() {
        let ui = UiStarted::default();
        assert!(ui.mark());
        assert!(!ui.mark());
        assert!(ui.is_set());
    }
}
