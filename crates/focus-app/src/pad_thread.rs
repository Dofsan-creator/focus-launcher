//! Lecture des manettes, envoi des actions à l'interface, et moteur Focus Input.

use crate::runtime::App;
use std::sync::Arc;
use tauri::AppHandle;

#[cfg(windows)]
pub fn spawn(app: AppHandle, state: Arc<App>) {
    use crate::commands::show_launcher;
    use focus_core::model::Store;
    use focus_input::actions::{Action, ActionMapper};
    use focus_input::focus_input::to_xreport;
    use focus_input::modes::{resolve_mode, Environment, Mode};
    use focus_input::pad::{Family, PadState};
    use focus_input::source::{read_xinput, vigem_present, HidPad, VirtualPad};
    use std::sync::atomic::Ordering;
    use std::time::{Duration, Instant};
    use tauri::Emitter;

    const MENU_POLL: Duration = Duration::from_millis(16);
    /// En jeu, le launcher ne cherche que le combo : il reste discret.
    const GAME_POLL: Duration = Duration::from_millis(50);
    /// Focus Input relaie la manette : il suit le rythme des rapports HID.
    const ENGINE_POLL: Duration = Duration::from_millis(4);
    const RESCAN: Duration = Duration::from_secs(3);

    std::thread::spawn(move || {
        let mut mapper = ActionMapper::default();
        let mut hid: Option<HidPad> = None;
        let mut virtual_pad: Option<VirtualPad> = None;
        let mut engine_failed = false;
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
            let in_session = state.in_session.load(Ordering::SeqCst);

            // Focus Input : pendant un jeu hors Steam, la DualShock 4 devient une
            // manette Xbox virtuelle. Sans HidHide, le jeu peut voir les deux
            // manettes : c'est le repli prévu par la v2 (cas 7), signalé dans l'interface.
            let engine_wanted = in_session
                && hid.is_some()
                && !engine_failed
                && state
                    .running()
                    .is_some_and(|running| running.store != Store::Steam)
                && resolve_mode(
                    Family::PlayStation,
                    Environment {
                        steam_running: false,
                        vigem_present: true,
                        hidhide_present: false,
                    },
                    state.settings().controller_mode,
                ) == Mode::FocusInput;
            if engine_wanted && virtual_pad.is_none() {
                match vigem_present().then(VirtualPad::create) {
                    Some(Ok(pad)) => virtual_pad = Some(pad),
                    Some(Err(err)) => {
                        tracing::warn!("focus input unavailable: {err}");
                        engine_failed = true;
                    }
                    None => engine_failed = true,
                }
            }
            if !engine_wanted {
                // Retrait de la manette virtuelle dès la fin de la session.
                virtual_pad = None;
                if !in_session {
                    engine_failed = false;
                }
            }
            if let (Some(pad), Some(report)) = (&mut virtual_pad, &sony) {
                if let Err(err) = pad.send(&to_xreport(report)) {
                    tracing::warn!("virtual pad update failed: {err}");
                    virtual_pad = None;
                    engine_failed = true;
                }
            }

            // La manette virtuelle apparaît aussi en XInput : on l'ignore alors.
            let xbox = if virtual_pad.is_some() {
                None
            } else {
                (0..4).find_map(read_xinput)
            };
            // Dernière manette utilisée : l'activité décide de la manette active.
            let (active, family) = match (&sony, &xbox) {
                (Some(s), _) if s.has_activity() || xbox.is_none() => {
                    (s.clone(), Some(Family::PlayStation))
                }
                (_, Some(x)) => (x.clone(), Some(Family::Xbox)),
                _ => (PadState::default(), last_family.filter(|_| hid.is_some())),
            };
            if family != last_family {
                last_family = family;
                let _ = app.emit("pad-family", family);
            }
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
            std::thread::sleep(if virtual_pad.is_some() {
                ENGINE_POLL
            } else if in_session {
                GAME_POLL
            } else {
                MENU_POLL
            });
        }
    });
}

#[cfg(not(windows))]
pub fn spawn(_app: AppHandle, _state: Arc<App>) {}
