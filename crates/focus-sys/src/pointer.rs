//! Envoi au système de la souris à la manette (mouvements, clics, molette) et du
//! clavier à l'écran de Windows. La décision vient de `focus_input::pointer`.

use windows::Win32::UI::Input::KeyboardAndMouse::{
    SendInput, INPUT, INPUT_0, INPUT_MOUSE, MOUSEEVENTF_LEFTDOWN, MOUSEEVENTF_LEFTUP, MOUSEEVENTF_MOVE,
    MOUSEEVENTF_RIGHTDOWN, MOUSEEVENTF_RIGHTUP, MOUSEEVENTF_WHEEL, MOUSEINPUT, MOUSE_EVENT_FLAGS,
};

/// Un cran de molette, tel que Windows le compte.
const WHEEL_STEP: i32 = 120;

fn mouse(flags: MOUSE_EVENT_FLAGS, dx: i32, dy: i32, data: i32) -> INPUT {
    INPUT {
        r#type: INPUT_MOUSE,
        Anonymous: INPUT_0 {
            mi: MOUSEINPUT {
                dx,
                dy,
                mouseData: data as u32,
                dwFlags: flags,
                time: 0,
                dwExtraInfo: 0,
            },
        },
    }
}

/// Envoie un pas de souris : déplacement relatif, boutons (`Some(true)` enfoncé,
/// `Some(false)` relâché), molette en crans.
pub fn send(dx: i32, dy: i32, left: Option<bool>, right: Option<bool>, wheel: i32) {
    let mut inputs = Vec::with_capacity(4);
    if dx != 0 || dy != 0 {
        inputs.push(mouse(MOUSEEVENTF_MOVE, dx, dy, 0));
    }
    match left {
        Some(true) => inputs.push(mouse(MOUSEEVENTF_LEFTDOWN, 0, 0, 0)),
        Some(false) => inputs.push(mouse(MOUSEEVENTF_LEFTUP, 0, 0, 0)),
        None => {}
    }
    match right {
        Some(true) => inputs.push(mouse(MOUSEEVENTF_RIGHTDOWN, 0, 0, 0)),
        Some(false) => inputs.push(mouse(MOUSEEVENTF_RIGHTUP, 0, 0, 0)),
        None => {}
    }
    if wheel != 0 {
        inputs.push(mouse(MOUSEEVENTF_WHEEL, 0, 0, wheel * WHEEL_STEP));
    }
    if inputs.is_empty() {
        return;
    }
    // SAFETY: tableau local de structures complètes ; la taille passée est exacte.
    unsafe {
        SendInput(&inputs, std::mem::size_of::<INPUT>() as i32);
    }
}

/// Ouvre le clavier à l'écran de Windows, ou le ferme s'il est déjà ouvert.
pub fn toggle_keyboard() {
    let open: Vec<u32> = crate::scan::process_ids()
        .into_iter()
        .filter(|(_, name)| name == "osk")
        .map(|(pid, _)| pid)
        .collect();
    if open.is_empty() {
        let root = std::env::var("SystemRoot").unwrap_or_else(|_| r"C:\Windows".to_owned());
        // Le clavier de Windows exige d'être ouvert par le shell (accès spécial à
        // l'interface) : CreateProcess le refuserait.
        if let Err(err) = crate::launch::open_system_tool(&format!(r"{root}\System32\osk.exe")) {
            tracing::warn!("on-screen keyboard not opened: {err}");
        }
    } else {
        for pid in open {
            crate::ops::request_close(pid);
        }
    }
}
