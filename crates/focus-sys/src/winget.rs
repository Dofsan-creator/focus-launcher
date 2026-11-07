//! Installation par winget, identifiants venant uniquement du catalogue compilé.

use std::os::windows::process::CommandExt;
use std::process::{Command, Stdio};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Installe une application, en bloquant jusqu'à la fin.
pub fn install(winget_id: &str) -> Result<(), String> {
    let status = Command::new("winget")
        .args(["install", "--id", winget_id, "-e", "--silent"])
        .args(["--accept-source-agreements", "--accept-package-agreements"])
        .stdin(Stdio::null())
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .map_err(|err| err.to_string())?;
    if status.success() {
        Ok(())
    } else {
        Err(format!("winget exit code {:?}", status.code()))
    }
}

/// winget est-il disponible sur cette édition de Windows ?
pub fn available() -> bool {
    Command::new("winget")
        .arg("--version")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .creation_flags(CREATE_NO_WINDOW)
        .status()
        .is_ok_and(|status| status.success())
}
