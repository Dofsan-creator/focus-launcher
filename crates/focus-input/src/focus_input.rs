//! Focus Input : traduit l'état d'une DualShock 4 en rapport de manette Xbox.

use crate::pad::{Button, PadState};

/// Rapport de manette Xbox, au format XInput.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct XReport {
    /// Masque de boutons XInput.
    pub buttons: u16,
    /// Gâchette gauche.
    pub left_trigger: u8,
    /// Gâchette droite.
    pub right_trigger: u8,
    /// Stick gauche.
    pub thumb_lx: i16,
    /// Stick gauche.
    pub thumb_ly: i16,
    /// Stick droit.
    pub thumb_rx: i16,
    /// Stick droit.
    pub thumb_ry: i16,
}

const BUTTON_BITS: [(Button, u16); 12] = [
    (Button::DpadUp, 0x0001),
    (Button::DpadDown, 0x0002),
    (Button::DpadLeft, 0x0004),
    (Button::DpadRight, 0x0008),
    (Button::Start, 0x0010),
    (Button::Back, 0x0020),
    (Button::LeftShoulder, 0x0100),
    (Button::RightShoulder, 0x0200),
    (Button::South, 0x1000),
    (Button::East, 0x2000),
    (Button::West, 0x4000),
    (Button::North, 0x8000),
];

fn thumb(value: f32) -> i16 {
    // La conversion sature : 1.0 donne i16::MAX, -1.0 donne -32767.
    (value.clamp(-1.0, 1.0) * f32::from(i16::MAX)) as i16
}

/// Construit le rapport virtuel.
pub fn to_xreport(state: &PadState) -> XReport {
    let buttons = BUTTON_BITS
        .iter()
        .filter(|(button, _)| state.is_pressed(*button))
        .fold(0, |mask, (_, bit)| mask | bit);
    let trigger = |button| if state.is_pressed(button) { u8::MAX } else { 0 };
    XReport {
        buttons,
        left_trigger: trigger(Button::LeftTrigger),
        right_trigger: trigger(Button::RightTrigger),
        thumb_lx: thumb(state.left_x),
        thumb_ly: thumb(state.left_y),
        thumb_rx: thumb(state.right_x),
        thumb_ry: thumb(state.right_y),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_should_become_the_a_bit() {
        let state = PadState {
            pressed: vec![Button::South],
            ..PadState::default()
        };
        assert_eq!(to_xreport(&state).buttons, 0x1000);
    }

    #[test]
    fn digital_trigger_should_be_full_scale() {
        let state = PadState {
            pressed: vec![Button::RightTrigger],
            ..PadState::default()
        };
        assert_eq!(to_xreport(&state).right_trigger, 255);
    }

    #[test]
    fn stick_extremes_should_not_overflow() {
        let state = PadState {
            left_x: 5.0,
            left_y: -5.0,
            ..PadState::default()
        };
        let report = to_xreport(&state);
        assert_eq!((report.thumb_lx, report.thumb_ly), (i16::MAX, -i16::MAX));
    }
}
