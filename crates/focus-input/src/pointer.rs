//! Souris à la manette : le stick gauche déplace le curseur, les boutons cliquent.
//! Sert là où une console n'aurait pas besoin de souris mais où un PC l'exige (le
//! lanceur d'un jeu, une fenêtre de connexion). Logique pure, testée sans matériel ;
//! l'envoi au système est dans `focus-sys`.

use crate::pad::{Button, PadState};

/// Zone morte du stick : en dessous, la manette au repos ne fait rien bouger.
const DEADZONE: f32 = 0.16;
/// Vitesse maximale du curseur, en pixels par seconde, stick à fond.
const MAX_SPEED: f32 = 1500.0;
/// Défilement maximal, en crans par seconde, stick droit à fond.
const MAX_SCROLL: f32 = 12.0;

/// Ce qu'il faut envoyer au système pour un pas de temps.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PointerOutput {
    /// Déplacement en pixels.
    pub dx: i32,
    /// Déplacement en pixels, positif vers le bas.
    pub dy: i32,
    /// Bouton gauche enfoncé (`Some(true)`) ou relâché (`Some(false)`).
    pub left: Option<bool>,
    /// Bouton droit enfoncé ou relâché.
    pub right: Option<bool>,
    /// Molette, en crans (positif vers le haut).
    pub wheel: i32,
    /// Ouvrir ou fermer le clavier à l'écran.
    pub keyboard: bool,
}

/// Transforme les états successifs de la manette en mouvements et clics.
#[derive(Debug, Default)]
pub struct PointerMapper {
    previous: PadState,
    /// Restes de pixels et de crans : un stick à peine incliné finit par bouger.
    carry_x: f32,
    carry_y: f32,
    carry_wheel: f32,
}

/// Inclinaison utile du stick, de 0 à 1, au carré : précise au centre, rapide au bord.
fn curve(value: f32) -> f32 {
    let magnitude = value.abs();
    if magnitude <= DEADZONE {
        return 0.0;
    }
    let scaled = ((magnitude - DEADZONE) / (1.0 - DEADZONE)).min(1.0);
    scaled * scaled * value.signum()
}

impl PointerMapper {
    /// Traite un état ; `dt` est le temps écoulé depuis le précédent, en secondes.
    pub fn update(&mut self, state: &PadState, dt: f32) -> PointerOutput {
        let dt = dt.clamp(0.0, 0.1);
        self.carry_x += curve(state.left_x) * MAX_SPEED * dt;
        // Le stick monte en positif, l'écran descend en positif.
        self.carry_y -= curve(state.left_y) * MAX_SPEED * dt;
        self.carry_wheel += curve(state.right_y) * MAX_SCROLL * dt;
        let take = |carry: &mut f32| {
            let whole = carry.trunc();
            *carry -= whole;
            whole as i32
        };
        let edge = |button: Button| -> Option<bool> {
            let (now, before) = (state.is_pressed(button), self.previous.is_pressed(button));
            (now != before).then_some(now)
        };
        let output = PointerOutput {
            dx: take(&mut self.carry_x),
            dy: take(&mut self.carry_y),
            left: edge(Button::South),
            right: edge(Button::East),
            wheel: take(&mut self.carry_wheel),
            keyboard: edge(Button::North) == Some(true),
        };
        self.previous = state.clone();
        output
    }

    /// Relâche ce qui est encore enfoncé, à la sortie du mode : jamais de clic bloqué.
    pub fn release(&mut self) -> PointerOutput {
        let output = PointerOutput {
            left: self.previous.is_pressed(Button::South).then_some(false),
            right: self.previous.is_pressed(Button::East).then_some(false),
            ..PointerOutput::default()
        };
        *self = Self::default();
        output
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stick(x: f32, y: f32) -> PadState {
        PadState { left_x: x, left_y: y, ..PadState::default() }
    }

    fn pressed(buttons: &[Button]) -> PadState {
        PadState { pressed: buttons.to_vec(), ..PadState::default() }
    }

    #[test]
    fn resting_stick_should_not_move_the_cursor() {
        let mut mapper = PointerMapper::default();
        for _ in 0..100 {
            let out = mapper.update(&stick(0.1, -0.12), 0.016);
            assert_eq!((out.dx, out.dy), (0, 0));
        }
    }

    #[test]
    fn full_stick_should_move_at_full_speed_and_up_should_go_up() {
        let mut mapper = PointerMapper::default();
        let out = mapper.update(&stick(1.0, 1.0), 0.1);
        assert_eq!(out.dx, MAX_SPEED as i32 / 10);
        assert_eq!(out.dy, -(MAX_SPEED as i32 / 10));
    }

    #[test]
    fn slight_tilt_should_still_move_over_time() {
        let mut mapper = PointerMapper::default();
        let moved: i32 = (0..120).map(|_| mapper.update(&stick(0.3, 0.0), 0.016).dx).sum();
        assert!(moved > 0, "a slight tilt must move a little ({moved})");
        assert!(moved < 100, "a slight tilt must stay precise ({moved})");
    }

    #[test]
    fn bottom_button_should_press_then_release_the_left_button() {
        let mut mapper = PointerMapper::default();
        assert_eq!(mapper.update(&pressed(&[Button::South]), 0.016).left, Some(true));
        assert_eq!(mapper.update(&pressed(&[Button::South]), 0.016).left, None);
        assert_eq!(mapper.update(&pressed(&[]), 0.016).left, Some(false));
    }

    #[test]
    fn right_button_should_right_click_and_top_button_should_toggle_the_keyboard_once() {
        let mut mapper = PointerMapper::default();
        let out = mapper.update(&pressed(&[Button::East, Button::North]), 0.016);
        assert_eq!(out.right, Some(true));
        assert!(out.keyboard);
        assert!(!mapper.update(&pressed(&[Button::East, Button::North]), 0.016).keyboard);
    }

    #[test]
    fn leaving_should_release_a_held_click() {
        let mut mapper = PointerMapper::default();
        mapper.update(&pressed(&[Button::South]), 0.016);
        assert_eq!(mapper.release().left, Some(false));
        assert_eq!(mapper.release().left, None);
    }

    #[test]
    fn right_stick_should_scroll() {
        let mut mapper = PointerMapper::default();
        let state = PadState { right_y: 1.0, ..PadState::default() };
        let wheel: i32 = (0..10).map(|_| mapper.update(&state, 0.1).wheel).sum();
        assert_eq!(wheel, MAX_SCROLL as i32);
    }
}
