//! Lecture des manettes et envoi des actions à l'interface.

use crate::runtime::App;
use std::sync::Arc;
use tauri::AppHandle;

#[cfg(windows)]
pub fn spawn(app: AppHandle, state: Arc<App>) {
    use crate::commands::show_launcher;
    use focus_input::actions::{Action, ActionMapper};
    use focus_input::pad::{Family, PadState};
    use focus_input::source::{read_xinput, HidPad};
    use std::sync::atomic::Ordering;
    use std::time::{Duration, Instant};
    use tauri::Emitter;

    const MENU_POLL: Duration = Duration::from_millis(16);
    /// En jeu, le launcher ne cherche que le combo : il reste discret.
    const GAME_POLL: Duration = Duration::from_millis(50);
    const RESCAN: Duration = Duration::from_secs(3);

    std::thread::spawn(move || {
        let mut mapper = ActionMapper::default();
        let mut hid: Option<HidPad> = None;
        let mut last_scan = Instant::now() - RESCAN;
        let mut last_family = None;
        let started = Instant::now();
        loop {
            if hid.is_none() && last_scan.elapsed() >= RESCAN {
                hid = HidPad::open_first();
                last_scan = Instant::now();
            }
            let mut sony = None;
            if let Some(pad) = &hid {
                match pad.read() {
                    Ok(Some(state)) => sony = Some(state),
                    Ok(None) => {}
                    Err(_) => hid = None,
                }
            }
            let xbox = (0..4).find_map(read_xinput);
            // Dernière manette utilisée : l'activité décide de la manette active.
            let (active, family) = match (&sony, &xbox) {
                (Some(s), _) if s.has_activity() || xbox.is_none() => (s.clone(), Some(Family::PlayStation)),
                (_, Some(x)) => (x.clone(), Some(Family::Xbox)),
                _ => (PadState::default(), last_family.filter(|_| hid.is_some())),
            };
            if family != last_family {
                last_family = family;
                let _ = app.emit("pad-family", family);
            }
            let in_session = state.in_session.load(Ordering::SeqCst);
            let now = started.elapsed().as_millis() as u64;
            for action in mapper.update(&active, now) {
                if in_session {
                    if matches!(action, Action::QuickPanel | Action::QuickSettings) {
                        show_launcher(&app);
                        let _ = app.emit("pad-action", action);
                    }
                } else {
                    let _ = app.emit("pad-action", action);
                }
            }
            std::thread::sleep(if in_session { GAME_POLL } else { MENU_POLL });
        }
    });
}

#[cfg(not(windows))]
pub fn spawn(_app: AppHandle, _state: Arc<App>) {}
