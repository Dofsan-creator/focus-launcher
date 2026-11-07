//! Tâches de fond : détection de nouveaux jeux et blocage de Big Picture.

use crate::commands::{scan_into, show_launcher};
use crate::runtime::App;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tauri::AppHandle;

const LIBRARY_POLL: Duration = Duration::from_secs(15);
const BIG_PICTURE_POLL: Duration = Duration::from_secs(2);

fn latest_change(paths: &[PathBuf]) -> Option<SystemTime> {
    paths
        .iter()
        .flat_map(|path| std::fs::read_dir(path).into_iter().flatten().flatten())
        .filter_map(|entry| entry.metadata().ok()?.modified().ok())
        .max()
}

/// Démarre les tâches de fond.
pub fn spawn(app: AppHandle, state: Arc<App>) {
    let scan_app = app.clone();
    let scan_state = state.clone();
    std::thread::spawn(move || {
        // L'accueil s'ouvre tout de suite : la première analyse se fait ici.
        if let Err(err) = scan_into(&scan_state, &scan_app) {
            tracing::warn!("library scan failed: {err}");
        }
        let paths = focus_library::discover::watch_paths();
        let mut known = latest_change(&paths);
        loop {
            std::thread::sleep(LIBRARY_POLL);
            let current = latest_change(&paths);
            if current != known {
                known = current;
                if let Err(err) = scan_into(&scan_state, &scan_app) {
                    tracing::warn!("library scan failed: {err}");
                }
            }
        }
    });

    #[cfg(windows)]
    std::thread::spawn(move || loop {
        std::thread::sleep(BIG_PICTURE_POLL);
        if state.settings().block_big_picture && focus_sys::shell::big_picture_open() {
            // Lien non documenté par Valve : un échec ne doit jamais gêner le launcher.
            let target = focus_core::model::LaunchTarget::Protocol {
                uri: "steam://close/bigpicture".to_owned(),
            };
            if let Err(err) = focus_sys::launch::start(&target) {
                tracing::warn!("big picture not closed: {err}");
            }
            show_launcher(&app);
        }
    });
    #[cfg(not(windows))]
    let _ = (app, state);
}
