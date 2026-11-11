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
use windows::Win32::UI::Input::XboxController::{
    XInputGetBatteryInformation, XInputSetState, BATTERY_DEVTYPE_GAMEPAD, BATTERY_TYPE_DISCONNECTED,
    BATTERY_TYPE_WIRED, XINPUT_BATTERY_INFORMATION, XINPUT_VIBRATION,
};

const ERROR_SUCCESS: u32 = 0;
/// Bit du bouton central dans l’état étendu (non documenté, utilisé par SDL et Chromium).
const XINPUT_GUIDE: u16 = 0x0400;

type GetStateEx = unsafe extern "system" fn(u32, *mut XinputStateEx) -> u32;

#[repr(C)]
#[derive(Default)]
struct XinputStateEx {
    state: XINPUT_STATE,
    _reserved: u32,
}

/// Fonction étendue d’XInput (ordinal 100), qui expose le bouton central.
fn get_state_ex() -> Option<GetStateEx> {
    static CELL: std::sync::OnceLock<Option<usize>> = std::sync::OnceLock::new();
    let address = *CELL.get_or_init(|| {
        use windows::core::{w, PCSTR};
        use windows::Win32::System::LibraryLoader::{GetProcAddress, LoadLibraryW};
        // SAFETY: bibliothèque système ; l’ordinal 100 est l’adresse attendue.
        unsafe {
            let module = LoadLibraryW(w!("xinput1_4.dll")).ok()?;
            GetProcAddress(module, PCSTR(100 as *const u8)).map(|f| f as usize)
        }
    });
    // SAFETY: adresse issue de GetProcAddress, signature de l’état étendu.
    address.map(|address| unsafe { std::mem::transmute::<usize, GetStateEx>(address) })
}

/// Le bouton central de la manette XInput est-il enfoncé ?
fn guide_pressed(index: u32) -> bool {
    let Some(get) = get_state_ex() else {
        return false;
    };
    let mut state = XinputStateEx::default();
    // SAFETY: structure locale de la taille attendue par la fonction étendue.
    let code = unsafe { get(index, &mut state) };
    code == ERROR_SUCCESS && state.state.Gamepad.wButtons.0 & XINPUT_GUIDE != 0
}
const TRIGGER_THRESHOLD: u8 = 128;

/// Fait vibrer la manette XInput `index` (force de 0 à 65535 par moteur).
/// Sans manette, l'appel échoue en silence.
pub fn rumble(index: u32, left: u16, right: u16) {
    let vibration = XINPUT_VIBRATION {
        wLeftMotorSpeed: left,
        wRightMotorSpeed: right,
    };
    // SAFETY: structure locale valide, lue seulement pendant l'appel.
    unsafe {
        let _ = XInputSetState(index, &vibration);
    }
}

/// Batterie de la manette XInput, de 0 (vide) à 3 (pleine). `None` si elle est
/// filaire, absente ou inconnue.
pub fn battery(index: u32) -> Option<u8> {
    let mut info = XINPUT_BATTERY_INFORMATION::default();
    // SAFETY: structure locale valide, de la taille attendue.
    let code = unsafe { XInputGetBatteryInformation(index, BATTERY_DEVTYPE_GAMEPAD, &mut info) };
    if code != ERROR_SUCCESS
        || info.BatteryType == BATTERY_TYPE_WIRED
        || info.BatteryType == BATTERY_TYPE_DISCONNECTED
    {
        return None;
    }
    Some(info.BatteryLevel.0)
}

/// Lit la manette XInput `index` (0 à 3). `None` si elle est absente.
pub fn read_xinput(index: u32) -> Option<PadState> {
    let mut state = XINPUT_STATE::default();
    // SAFETY: `state` est une structure valide, possédée, de la taille attendue.
    let code = unsafe { XInputGetState(index, &mut state) };
    if code != ERROR_SUCCESS {
        return None;
    }
    let pad = state.Gamepad;
    let guide = guide_pressed(index);
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
    if guide {
        pressed.push(Button::Guide);
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

/// HidHide (masquage de la vraie manette) est installé.
pub fn hidhide_present() -> bool {
    std::path::Path::new(r"C:\Program Files\Nefarius Software Solutions\HidHide").is_dir()
}

/// Le pilote ViGEmBus est-il présent ?
pub fn vigem_present() -> bool {
    vigem_client::Client::connect().is_ok()
}
