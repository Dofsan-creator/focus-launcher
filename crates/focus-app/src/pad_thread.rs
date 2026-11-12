//! Lecture des manettes, envoi des actions à l'interface, et moteur Focus Input.

use crate::runtime::App;
use std::sync::Arc;
use tauri::AppHandle;

#[cfg(windows)]
pub fn spawn(app: AppHandle, state: Arc<App>) {
    use focus_core::model::Store;
    use focus_input::actions::{Action, ActionMapper};
    use focus_input::focus_input::to_xreport;
    use focus_input::modes::{resolve_mode, Environment, Mode};
    use focus_input::pad::{Family, PadState};
    use focus_input::pointer::PointerMapper;
    use focus_input::source::{read_xinput, rumble, vigem_present, HidPad, VirtualPad};
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
        let mut pointer = PointerMapper::default();
        let mut was_pointing = false;
        let mut last_tick = Instant::now();
        let mut hid: Option<HidPad> = None;
        let mut virtual_pad: Option<VirtualPad> = None;
        let mut engine_failed = false;
        let mut last_scan = Instant::now() - RESCAN;
        let mut last_family = None;
        let started = Instant::now();
        let hidhide_present = focus_input::source::hidhide_present();
        // Fin de la vibration en cours : l'impulsion reste brève, sans bloquer la boucle.
        let mut rumble_until: Option<(u32, Instant)> = None;
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
            // manette Xbox virtuelle. Sans HidHide, le jeu verrait les deux
            // manettes : le mode n'est alors pris que sur choix explicite (`resolve_mode`).
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
                        hidhide_present,
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
            let xbox_pad = if virtual_pad.is_some() {
                None
            } else {
                (0..4).find_map(|index| read_xinput(index).map(|pad| (index, pad)))
            };
            let xbox = xbox_pad.as_ref().map(|(_, pad)| pad.clone());
            // Dernière manette utilisée : l'activité décide de la manette active.
            let (active, family) = match (&sony, &xbox) {
                // Une manette PlayStation branchée garde ses symboles, même si Steam
                // en montre aussi une copie Xbox.
                (Some(s), _) => {
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
            let actions = mapper.update(&active, now);
            // Souris à la manette : le stick déplace le curseur, le bouton du bas clique,
            // celui de droite fait le clic droit, celui du haut ouvre le clavier.
            let pointing = state.pointer.load(Ordering::SeqCst);
            if pointing {
                let out = pointer.update(&active, last_tick.elapsed().as_secs_f32());
                focus_sys::pointer::send(out.dx, out.dy, out.left, out.right, out.wheel);
                if out.keyboard {
                    focus_sys::pointer::toggle_keyboard();
                }
            } else if was_pointing {
                // Éteinte ailleurs (le jeu est apparu) : aucun clic ne reste enfoncé.
                let out = pointer.release();
                focus_sys::pointer::send(0, 0, out.left, out.right, 0);
            }
            was_pointing = pointing;
            last_tick = Instant::now();
            for action in actions {
                // En mode souris, seul le bouton central compte : il rend la manette à
                // Focus et ramène son menu.
                if pointing {
                    if matches!(action, Action::Home) {
                        state.pointer.store(false, Ordering::SeqCst);
                        let out = pointer.release();
                        focus_sys::pointer::send(0, 0, out.left, out.right, 0);
                        was_pointing = false;
                        crate::commands::back_to_launcher(&app);
                        let _ = app.emit("pad-action", action);
                    }
                    continue;
                }
                // En jeu, Focus ne réagit qu’au bouton central, sauf quand son menu est
                // affiché : alors Retour, les flèches et Valider lui reviennent aussi.
                if in_session && !crate::commands::launcher_visible(&app) {
                    if matches!(action, Action::QuickPanel | Action::QuickSettings | Action::Home) {
                        crate::commands::back_to_launcher(&app);
                        let _ = app.emit("pad-action", action);
                    }
                } else {
                    if matches!(action, Action::Home) {
                        crate::commands::back_to_launcher(&app);
                    }
                    let _ = app.emit("pad-action", action);
                    // Retour sensoriel léger, plus marqué pour valider (spec v1, 6.5).
                    if let Some((index, _)) = xbox_pad.as_ref().filter(|_| state.settings().vibration) {
                        let strength = if matches!(action, Action::Confirm) { 14_000 } else { 6_000 };
                        rumble(*index, strength, strength);
                        rumble_until = Some((*index, Instant::now() + Duration::from_millis(40)));
                    }
                }
            }
            if let Some((index, until)) = rumble_until {
                if Instant::now() >= until {
                    rumble(index, 0, 0);
                    rumble_until = None;
                }
            }
            std::thread::sleep(if virtual_pad.is_some() {
                ENGINE_POLL
            } else if was_pointing {
                MENU_POLL
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
