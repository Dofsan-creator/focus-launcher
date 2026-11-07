//! Rapport système en lecture seule, et applications du Menu Démarrer.

use serde::{Deserialize, Serialize};
use std::os::windows::process::CommandExt;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Matériel de base.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hardware {
    /// Processeur.
    pub cpu: String,
    /// Mémoire totale en octets.
    pub memory: u64,
    /// Cartes graphiques.
    pub gpus: Vec<String>,
    /// Version de Windows.
    pub os: String,
}

fn powershell(script: &str) -> Option<String> {
    let output = Command::new("powershell.exe")
        .args(["-NoProfile", "-NonInteractive", "-Command"])
        .arg(format!(
            "[Console]::OutputEncoding=[Text.Encoding]::UTF8;{script}"
        ))
        .creation_flags(CREATE_NO_WINDOW)
        .output()
        .ok()?;
    output
        .status
        .success()
        .then(|| String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Lit le matériel par CIM, en lecture seule.
pub fn hardware() -> Hardware {
    let script = "$c=Get-CimInstance Win32_Processor|Select-Object -First 1;\
        $o=Get-CimInstance Win32_OperatingSystem;\
        $g=@(Get-CimInstance Win32_VideoController|ForEach-Object Name);\
        @{cpu=$c.Name;memory=[uint64]$o.TotalVisibleMemorySize*1024;gpus=$g;os=$o.Caption}|ConvertTo-Json -Compress";
    powershell(script)
        .and_then(|json| serde_json::from_str(json.trim()).ok())
        .unwrap_or_default()
}

/// Applications du Menu Démarrer, y compris celles du Microsoft Store.
pub fn start_apps<T: for<'de> Deserialize<'de>>() -> Vec<T> {
    powershell("@(Get-StartApps)|ConvertTo-Json -Compress")
        .and_then(|json| serde_json::from_str(json.trim()).ok())
        .unwrap_or_default()
}
