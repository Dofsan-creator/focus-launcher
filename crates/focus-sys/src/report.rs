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

/// Points de Windows qui pèsent sur le jeu et que Focus ne change jamais : il les montre
/// seulement (décision 32). `None` : valeur absente, la machine ne le gère pas.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Checks {
    /// Intégrité de la mémoire (sécurité basée sur la virtualisation).
    pub memory_integrity: Option<bool>,
    /// Planification du processeur graphique par le matériel.
    pub gpu_scheduling: Option<bool>,
    /// Démarrage rapide de Windows.
    pub fast_startup: Option<bool>,
}

/// Lit une valeur DWORD sous HKLM, en lecture seule.
fn machine_dword(path: &str, name: &str) -> Option<u32> {
    use windows::core::PCWSTR;
    use windows::Win32::System::Registry::{RegGetValueW, HKEY_LOCAL_MACHINE, RRF_RT_REG_DWORD};
    let wide = |text: &str| text.encode_utf16().chain(std::iter::once(0)).collect::<Vec<u16>>();
    let (path, name) = (wide(path), wide(name));
    let mut value = 0_u32;
    let mut size = 4_u32;
    // SAFETY: tampons locaux de la taille annoncée, vivants pendant l'appel.
    let status = unsafe {
        RegGetValueW(HKEY_LOCAL_MACHINE, PCWSTR(path.as_ptr()), PCWSTR(name.as_ptr()), RRF_RT_REG_DWORD,
            None, Some((&mut value as *mut u32).cast()), Some(&mut size))
    };
    status.is_ok().then_some(value)
}

/// Lit les points du bilan, sans rien écrire.
pub fn checks() -> Checks {
    Checks {
        memory_integrity: machine_dword(
            r"SYSTEM\CurrentControlSet\Control\DeviceGuard\Scenarios\HypervisorEnforcedCodeIntegrity",
            "Enabled",
        )
        .map(|value| value == 1),
        gpu_scheduling: machine_dword(r"SYSTEM\CurrentControlSet\Control\GraphicsDrivers", "HwSchMode")
            .map(|value| value == 2),
        fast_startup: machine_dword(r"SYSTEM\CurrentControlSet\Control\Session Manager\Power", "HiberbootEnabled")
            .map(|value| value == 1),
    }
}
