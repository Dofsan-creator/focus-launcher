//! Lecture réelle des manettes sous Windows : XInput et HID.

use crate::ds4;
use crate::focus_input::XReport;
use crate::pad::{Button, Family, PadState};
use hidapi::{HidApi, HidDevice};
use windows::Win32::UI::Input::XboxController::{
    XInputGetState, XINPUT_GAMEPAD_A, XINPUT_GAMEPAD_B, XINPUT_GAMEPAD_BACK,
    XINPUT_GAMEPAD_DPAD_DOWN, XINPUT_GAMEPAD_DPAD_LEFT, XINPUT_GAMEPAD_DPAD_RIGHT,
    XINPUT_GAMEPAD_DPAD_UP, XINPUT_GAMEPAD_LEFT_SHOULDER, XINPUT_GAMEPAD_RIGHT_SHOULDER,
    XINPUT_GAMEPAD_START, XINPUT_GAMEPAD_X, XINPUT_GAMEPAD_Y, XINPUT_STATE,
};

const ERROR_SUCCESS: u32 = 0;
const TRIGGER_THRESHOLD: u8 = 128;

/// Lit la manette XInput `index` (0 à 3). `None` si elle est absente.
pub fn read_xinput(index: u32) -> Option<PadState> {
    let mut state = XINPUT_STATE::default();
    // SAFETY: `state` est une structure valide, possédée, de la taille attendue.
    let code = unsafe { XInputGetState(index, &mut state) };
    if code != ERROR_SUCCESS {
        return None;
    }
    let pad = state.Gamepad;
    let map = [
        (XINPUT_GAMEPAD_A, Button::South),
        (XINPUT_GAMEPAD_B, Button::East),
        (XINPUT_GAMEPAD_X, Button::West),
        (XINPUT_GAMEPAD_Y, Button::North),
        (XINPUT_GAMEPAD_LEFT_SHOULDER, Button::LeftShoulder),
        (XINPUT_GAMEPAD_RIGHT_SHOULDER, Button::RightShoulder),
        (XINPUT_GAMEPAD_BACK, Button::Back),
        (XINPUT_GAMEPAD_START, Button::Start),
        (XINPUT_GAMEPAD_DPAD_UP, Button::DpadUp),
        (XINPUT_GAMEPAD_DPAD_DOWN, Button::DpadDown),
        (XINPUT_GAMEPAD_DPAD_LEFT, Button::DpadLeft),
        (XINPUT_GAMEPAD_DPAD_RIGHT, Button::DpadRight),
    ];
    let mut pressed: Vec<Button> = map
        .iter()
        .filter(|(flag, _)| pad.wButtons.0 & flag.0 != 0)
        .map(|(_, button)| *button)
        .collect();
    if pad.bLeftTrigger > TRIGGER_THRESHOLD {
        pressed.push(Button::LeftTrigger);
    }
    if pad.bRightTrigger > TRIGGER_THRESHOLD {
        pressed.push(Button::RightTrigger);
    }
    let axis = |value: i16| f32::from(value) / f32::from(i16::MAX);
    Some(PadState {
        pressed,
        left_x: axis(pad.sThumbLX),
        left_y: axis(pad.sThumbLY),
        right_x: axis(pad.sThumbRX),
        right_y: axis(pad.sThumbRY),
    })
}

/// Lecteur direct d'une DualShock 4.
pub struct HidPad {
    device: HidDevice,
    /// Nom du produit.
    pub name: String,
}

impl HidPad {
    /// Ouvre la première manette Sony trouvée, en non bloquant.
    pub fn open_first() -> Option<Self> {
        let api = HidApi::new().ok()?;
        let info = api
            .device_list()
            .find(|d| ds4::is_playstation(d.vendor_id(), d.product_id()))?;
        let device = info.open_device(&api).ok()?;
        device.set_blocking_mode(false).ok()?;
        Some(Self {
            device,
            name: info.product_string().unwrap_or_default().to_owned(),
        })
    }

    /// Lit le dernier rapport disponible. `Err` si la manette a disparu.
    pub fn read(&self) -> Result<Option<PadState>, hidapi::HidError> {
        let mut buffer = [0_u8; 128];
        let size = self.device.read(&mut buffer)?;
        Ok(if size == 0 {
            None
        } else {
            ds4::parse_report(&buffer[..size])
        })
    }
}

/// Une manette PlayStation est-elle présente ? Peut être faux si Steam la masque.
pub fn detect_family() -> Family {
    if let Ok(api) = HidApi::new() {
        if api
            .device_list()
            .any(|d| ds4::is_playstation(d.vendor_id(), d.product_id()))
        {
            return Family::PlayStation;
        }
    }
    if (0..4).any(|index| read_xinput(index).is_some()) {
        Family::Xbox
    } else {
        Family::Other
    }
}

/// Manette virtuelle Xbox fournie par le pilote ViGEmBus.
pub struct VirtualPad {
    target: vigem_client::Xbox360Wired<vigem_client::Client>,
}

impl VirtualPad {
    /// Crée la manette. `Err` si le pilote est absent.
    pub fn create() -> Result<Self, vigem_client::Error> {
        let client = vigem_client::Client::connect()?;
        let mut target =
            vigem_client::Xbox360Wired::new(client, vigem_client::TargetId::XBOX360_WIRED);
        target.plugin()?;
        target.wait_ready()?;
        Ok(Self { target })
    }

    /// Envoie un rapport.
    pub fn send(&mut self, report: &XReport) -> Result<(), vigem_client::Error> {
        self.target.update(&vigem_client::XGamepad {
            buttons: vigem_client::XButtons {
                raw: report.buttons,
            },
            left_trigger: report.left_trigger,
            right_trigger: report.right_trigger,
            thumb_lx: report.thumb_lx,
            thumb_ly: report.thumb_ly,
            thumb_rx: report.thumb_rx,
            thumb_ry: report.thumb_ry,
        })
    }
}

/// Le pilote ViGEmBus est-il présent ?
pub fn vigem_present() -> bool {
    vigem_client::Client::connect().is_ok()
}
