//! Décodage des rapports HID de la DualShock 4, à partir du format public.

use crate::pad::{Button, PadState};

/// Identifiant fabricant Sony.
pub const SONY_VENDOR_ID: u16 = 0x054C;
/// DualShock 4 première génération, deuxième génération et adaptateur sans fil.
pub const DS4_PRODUCT_IDS: [u16; 3] = [0x05C4, 0x09CC, 0x0BA0];
/// DualSense et DualSense Edge.
pub const DUALSENSE_PRODUCT_IDS: [u16; 2] = [0x0CE6, 0x0DF2];

/// Rapport USB et Bluetooth court.
const REPORT_SHORT: u8 = 0x01;
/// Rapport Bluetooth complet.
const REPORT_BLUETOOTH: u8 = 0x11;
/// En Bluetooth, deux octets précèdent les données.
const BLUETOOTH_OFFSET: usize = 2;
/// Taille minimale : sticks, boutons, gâchettes analogiques.
const MIN_PAYLOAD: usize = 9;

/// Le couple fabricant et produit est-il une DualShock 4 ?
pub fn is_ds4(vendor: u16, product: u16) -> bool {
    vendor == SONY_VENDOR_ID && DS4_PRODUCT_IDS.contains(&product)
}

/// Est-ce une manette Sony prise en charge ?
pub fn is_playstation(vendor: u16, product: u16) -> bool {
    is_ds4(vendor, product)
        || (vendor == SONY_VENDOR_ID && DUALSENSE_PRODUCT_IDS.contains(&product))
}

/// Décode un rapport d'entrée. Renvoie `None` pour tout rapport inconnu ou tronqué.
pub fn parse_report(report: &[u8]) -> Option<PadState> {
    let data = match *report.first()? {
        REPORT_SHORT => report.get(1..)?,
        REPORT_BLUETOOTH => report.get(1 + BLUETOOTH_OFFSET..)?,
        _ => return None,
    };
    if data.len() < MIN_PAYLOAD {
        return None;
    }
    let mut pressed = Vec::new();
    let mut set = |on: bool, button| {
        if on {
            pressed.push(button);
        }
    };
    let main = data[4];
    match main & 0x0F {
        0 => set(true, Button::DpadUp),
        1 => {
            set(true, Button::DpadUp);
            set(true, Button::DpadRight);
        }
        2 => set(true, Button::DpadRight),
        3 => {
            set(true, Button::DpadDown);
            set(true, Button::DpadRight);
        }
        4 => set(true, Button::DpadDown),
        5 => {
            set(true, Button::DpadDown);
            set(true, Button::DpadLeft);
        }
        6 => set(true, Button::DpadLeft),
        7 => {
            set(true, Button::DpadUp);
            set(true, Button::DpadLeft);
        }
        _ => {}
    }
    set(main & 0x10 != 0, Button::West); // carré
    set(main & 0x20 != 0, Button::South); // croix
    set(main & 0x40 != 0, Button::East); // rond
    set(main & 0x80 != 0, Button::North); // triangle
    let extra = data[5];
    set(extra & 0x01 != 0, Button::LeftShoulder);
    set(extra & 0x02 != 0, Button::RightShoulder);
    set(extra & 0x04 != 0, Button::LeftTrigger);
    set(extra & 0x08 != 0, Button::RightTrigger);
    set(extra & 0x10 != 0, Button::Back); // Share
    set(extra & 0x20 != 0, Button::Start); // Options
    Some(PadState {
        pressed,
        left_x: axis(data[0]),
        left_y: -axis(data[1]),
        right_x: axis(data[2]),
        right_y: -axis(data[3]),
    })
}

/// Convertit un axe 0..=255 (centre 128) en -1..=1, y arrière plan vertical inversé par l'appelant.
fn axis(raw: u8) -> f32 {
    (f32::from(raw) - 128.0) / 127.0
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Rapport USB au repos : sticks centrés, croix relâchée (valeur 8).
    fn idle() -> [u8; 10] {
        [0x01, 128, 128, 128, 128, 0x08, 0, 0, 0, 0]
    }

    fn parsed(report: &[u8]) -> PadState {
        parse_report(report).unwrap_or_default()
    }

    #[test]
    fn idle_report_should_have_nothing_pressed() {
        assert!(parsed(&idle()).pressed.is_empty());
    }

    #[test]
    fn cross_should_map_to_south() {
        let mut report = idle();
        report[5] |= 0x20;
        assert_eq!(parsed(&report).pressed, [Button::South]);
    }

    #[test]
    fn circle_should_map_to_east() {
        let mut report = idle();
        report[5] |= 0x40;
        assert_eq!(parsed(&report).pressed, [Button::East]);
    }

    #[test]
    fn share_and_options_should_form_the_combo_buttons() {
        let mut report = idle();
        report[6] = 0x30;
        assert_eq!(parsed(&report).pressed, [Button::Back, Button::Start]);
    }

    #[test]
    fn diagonal_should_press_both_directions() {
        let mut report = idle();
        report[5] = 0x01;
        assert_eq!(parsed(&report).pressed, [Button::DpadUp, Button::DpadRight]);
    }

    #[test]
    fn stick_up_should_be_positive() {
        let mut report = idle();
        report[2] = 0;
        assert!(parsed(&report).left_y > 0.9);
    }

    #[test]
    fn bluetooth_report_should_use_the_shifted_payload() {
        let mut report = [0_u8; 12];
        report[0] = 0x11;
        report[3..8].copy_from_slice(&[128, 128, 128, 128, 0x28]);
        assert_eq!(parsed(&report).pressed, [Button::South]);
    }

    #[test]
    fn unknown_report_should_be_refused() {
        assert_eq!(parse_report(&[0x05, 0, 0]), None);
    }

    #[test]
    fn truncated_report_should_be_refused() {
        assert_eq!(parse_report(&[0x01, 128, 128]), None);
    }

    #[test]
    fn empty_report_should_be_refused() {
        assert_eq!(parse_report(&[]), None);
    }

    #[test]
    fn hardware_ids_should_recognise_a_ds4() {
        assert!(is_ds4(0x054C, 0x09CC) && !is_ds4(0x045E, 0x09CC));
    }
}
