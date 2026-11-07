//! Traduction des états de manette en actions d'interface.

use crate::pad::{Button, PadState};
use serde::{Deserialize, Serialize};

/// Action d'interface : les vues ne connaissent jamais un bouton.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Action {
    /// Haut.
    Up,
    /// Bas.
    Down,
    /// Gauche.
    Left,
    /// Droite.
    Right,
    /// Valider.
    Confirm,
    /// Retour.
    Back,
    /// Action secondaire.
    Secondary,
    /// Options de l'élément.
    Options,
    /// Menu général.
    Menu,
    /// Filtre précédent.
    PagePrev,
    /// Filtre suivant.
    PageNext,
    /// Panneau rapide (combo court).
    QuickPanel,
    /// Réglages rapides (combo long).
    QuickSettings,
}

/// Délai avant la première répétition, en millisecondes.
const REPEAT_DELAY_MS: u64 = 350;
/// Intervalle de répétition.
const REPEAT_INTERVAL_MS: u64 = 90;
/// Seuil au-delà duquel le stick compte comme une direction.
const STICK_THRESHOLD: f32 = 0.6;
/// Maintien du combo pour le panneau rapide.
const COMBO_HOLD_MS: u64 = 1000;
/// Maintien du combo pour les réglages rapides.
const COMBO_LONG_HOLD_MS: u64 = 2000;

const DIRECTIONS: [Action; 4] = [Action::Up, Action::Down, Action::Left, Action::Right];

const BUTTONS: [(Button, Action); 7] = [
    (Button::South, Action::Confirm),
    (Button::East, Action::Back),
    (Button::West, Action::Secondary),
    (Button::North, Action::Options),
    (Button::Start, Action::Menu),
    (Button::LeftShoulder, Action::PagePrev),
    (Button::RightShoulder, Action::PageNext),
];

/// Convertit le flux d'états en actions : anti-rebond, répétition, combos.
#[derive(Debug, Default)]
pub struct ActionMapper {
    previous: PadState,
    held_direction: Option<(Action, u64)>,
    combo_since: Option<u64>,
    combo_fired: u8,
}

impl ActionMapper {
    /// Traite un état à l'instant `now_ms`. Appelé environ 60 fois par seconde.
    pub fn update(&mut self, state: &PadState, now_ms: u64) -> Vec<Action> {
        let mut actions = Vec::new();
        let combo = state.is_pressed(Button::Back) && state.is_pressed(Button::Start);
        self.update_combo(combo, now_ms, &mut actions);
        // Le combo masque Start : sans cela, le menu s'ouvrirait en même temps.
        let start_blocked = combo || self.combo_since.is_some();
        for (button, action) in BUTTONS {
            let pressed_now = state.is_pressed(button) && !self.previous.is_pressed(button);
            if pressed_now && !(button == Button::Start && start_blocked) {
                actions.push(action);
            }
        }
        self.update_direction(direction_of(state), now_ms, &mut actions);
        self.previous = state.clone();
        actions
    }

    fn update_combo(&mut self, combo: bool, now_ms: u64, actions: &mut Vec<Action>) {
        if !combo {
            self.combo_since = None;
            self.combo_fired = 0;
            return;
        }
        let since = *self.combo_since.get_or_insert(now_ms);
        let held = now_ms.saturating_sub(since);
        if held >= COMBO_HOLD_MS && self.combo_fired == 0 {
            actions.push(Action::QuickPanel);
            self.combo_fired = 1;
        }
        if held >= COMBO_LONG_HOLD_MS && self.combo_fired == 1 {
            actions.push(Action::QuickSettings);
            self.combo_fired = 2;
        }
    }

    fn update_direction(
        &mut self,
        direction: Option<Action>,
        now_ms: u64,
        actions: &mut Vec<Action>,
    ) {
        match (direction, self.held_direction) {
            (None, _) => self.held_direction = None,
            (Some(dir), Some((held, next))) if dir == held => {
                if now_ms >= next {
                    actions.push(dir);
                    self.held_direction = Some((dir, now_ms + REPEAT_INTERVAL_MS));
                }
            }
            (Some(dir), _) => {
                actions.push(dir);
                self.held_direction = Some((dir, now_ms + REPEAT_DELAY_MS));
            }
        }
    }
}

fn direction_of(state: &PadState) -> Option<Action> {
    let pressed = [
        state.is_pressed(Button::DpadUp) || state.left_y > STICK_THRESHOLD,
        state.is_pressed(Button::DpadDown) || state.left_y < -STICK_THRESHOLD,
        state.is_pressed(Button::DpadLeft) || state.left_x < -STICK_THRESHOLD,
        state.is_pressed(Button::DpadRight) || state.left_x > STICK_THRESHOLD,
    ];
    DIRECTIONS
        .into_iter()
        .zip(pressed)
        .find(|(_, on)| *on)
        .map(|(dir, _)| dir)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pad(buttons: &[Button]) -> PadState {
        PadState {
            pressed: buttons.to_vec(),
            ..PadState::default()
        }
    }

    #[test]
    fn press_should_fire_once_while_held() {
        let mut mapper = ActionMapper::default();
        let first = mapper.update(&pad(&[Button::South]), 0);
        let second = mapper.update(&pad(&[Button::South]), 16);
        assert_eq!((first, second), (vec![Action::Confirm], vec![]));
    }

    #[test]
    fn release_then_press_should_fire_again() {
        let mut mapper = ActionMapper::default();
        mapper.update(&pad(&[Button::East]), 0);
        mapper.update(&pad(&[]), 16);
        assert_eq!(mapper.update(&pad(&[Button::East]), 32), [Action::Back]);
    }

    #[test]
    fn held_direction_should_repeat_after_a_delay() {
        let mut mapper = ActionMapper::default();
        let hold = pad(&[Button::DpadRight]);
        assert_eq!(mapper.update(&hold, 0), [Action::Right]);
        assert!(mapper.update(&hold, 300).is_empty());
        assert_eq!(mapper.update(&hold, 350), [Action::Right]);
        assert!(mapper.update(&hold, 400).is_empty());
        assert_eq!(mapper.update(&hold, 440), [Action::Right]);
    }

    #[test]
    fn stick_should_behave_like_the_dpad() {
        let mut mapper = ActionMapper::default();
        let state = PadState {
            left_y: -0.9,
            ..PadState::default()
        };
        assert_eq!(mapper.update(&state, 0), [Action::Down]);
    }

    #[test]
    fn small_stick_drift_should_be_ignored() {
        let mut mapper = ActionMapper::default();
        let state = PadState {
            left_x: 0.2,
            ..PadState::default()
        };
        assert!(mapper.update(&state, 0).is_empty());
    }

    #[test]
    fn combo_should_open_the_panel_after_one_second_only() {
        let mut mapper = ActionMapper::default();
        let combo = pad(&[Button::Back, Button::Start]);
        assert!(mapper.update(&combo, 0).is_empty());
        assert!(mapper.update(&combo, 900).is_empty());
        assert_eq!(mapper.update(&combo, 1000), [Action::QuickPanel]);
        assert!(mapper.update(&combo, 1100).is_empty());
    }

    #[test]
    fn long_combo_should_open_quick_settings() {
        let mut mapper = ActionMapper::default();
        let combo = pad(&[Button::Back, Button::Start]);
        mapper.update(&combo, 0);
        mapper.update(&combo, 1000);
        assert_eq!(mapper.update(&combo, 2000), [Action::QuickSettings]);
    }

    #[test]
    fn combo_should_not_also_open_the_menu() {
        let mut mapper = ActionMapper::default();
        mapper.update(&pad(&[Button::Back]), 0);
        assert!(mapper
            .update(&pad(&[Button::Back, Button::Start]), 16)
            .is_empty());
    }

    #[test]
    fn combo_released_early_should_do_nothing() {
        let mut mapper = ActionMapper::default();
        let combo = pad(&[Button::Back, Button::Start]);
        mapper.update(&combo, 0);
        mapper.update(&pad(&[]), 500);
        mapper.update(&combo, 600);
        assert!(mapper.update(&combo, 1100).is_empty());
    }
}
