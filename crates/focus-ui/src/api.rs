//! Pont vers le cœur Rust. Hors de Tauri (tests dans un navigateur), des données
//! de démonstration répondent : aucune machine n'est touchée.

use focus_core::art::{art_key, ArtInfo, ArtKind};
use js_sys::{Function, Promise, Reflect};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// Ligne de bibliothèque, avec ses autres sources et ses images.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Merged {
    pub entry: focus_core::model::Entry,
    pub alternates: Vec<focus_core::model::Entry>,
    #[serde(default)]
    pub art: ArtInfo,
}

impl Merged {
    /// Adresse d'une image, ou `None` si elle n'existe pas.
    pub fn art_url(&self, kind: ArtKind) -> Option<String> {
        self.art.has(kind).then(|| {
            format!(
                "http://focusart.localhost/{}/{}?v={}",
                art_key(&self.entry.id),
                kind.slug(),
                self.art.version
            )
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    pub plan: focus_core::plan::Plan,
    pub simulation: bool,
    pub guardian: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEnd {
    pub entry_id: String,
    pub title: String,
    pub seconds: u64,
    pub restored: bool,
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningEntry {
    pub entry_id: String,
    pub title: String,
    pub kind: focus_core::model::EntryKind,
    pub real: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirItem {
    pub name: String,
    pub path: String,
    pub is_dir: bool,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    pub id: String,
    pub name: String,
    pub category: String,
    pub publisher: String,
    pub winget: bool,
    #[serde(default)]
    pub installed: bool,
    #[serde(default)]
    pub art: ArtInfo,
}

impl CatalogItem {
    /// Icône de la boutique installée, si Focus a pu la lire.
    pub fn icon_url(&self) -> Option<String> {
        self.art.icon.then(|| {
            format!("http://focusart.localhost/{}/icon?v={}", art_key(&format!("store:{}", self.id)), self.art.version)
        })
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatus {
    pub family: String,
    pub mode: String,
    #[serde(default)]
    pub battery: Option<u8>,
}

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Hardware {
    pub cpu: String,
    pub memory: u64,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemReport {
    #[serde(default)]
    pub hardware: Hardware,
    pub processes: u64,
    pub services: u64,
    pub top: Vec<(String, u64)>,
}

/// Jeu que Steam télécharge ou met à jour.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteamDownload {
    pub appid: String,
    pub name: String,
    pub done: u64,
    pub total: u64,
}

/// Installation lancée depuis Focus.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallJob {
    pub id: String,
    pub name: String,
    pub state: String,
}

/// Tout ce qui se télécharge ou s'installe.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Downloads {
    pub steam: Vec<SteamDownload>,
    pub installs: Vec<InstallJob>,
}

/// Mise à jour de Focus disponible.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    pub version: String,
    #[serde(default)]
    pub notes: String,
}

/// Bilan du mode console, miroir de `focus-app` : ce qui a changé sur le PC.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ConsoleReport {
    pub level: String,
    pub shell_stopped: bool,
    pub power_before: Option<String>,
    pub power_after: Option<String>,
    pub memory_total: u64,
    pub memory_before: u64,
    pub memory_after: u64,
    pub slowed: u64,
    pub closed: u64,
    pub paused: u64,
    pub trimmed: u64,
    pub failed: u64,
    pub apps: Vec<String>,
    pub millis: u64,
}

/// Mode d'alimentation « performances élevées » de Windows.
const POWER_HIGH: &str = "8c5e7fdae8bf4a969a85a6e23a8c635c";
/// Mode « performances optimales ».
const POWER_ULTIMATE: &str = "e9a42b02d5df448daa0003f14749eb61";

impl ConsoleReport {
    /// Les optimisations sont-elles appliquées ?
    pub fn optimized(&self) -> bool {
        self.level != "off"
    }

    /// Mémoire rendue aux jeux, en gigaoctets (0 si elle n'a pas augmenté).
    pub fn freed_gb(&self) -> f64 {
        self.memory_after.saturating_sub(self.memory_before) as f64 / f64::from(1 << 30)
    }

    /// Le PC tourne-t-il à pleine puissance ?
    pub fn full_power(&self) -> bool {
        self.power_after
            .as_deref()
            .is_some_and(|plan| plan == POWER_HIGH || plan == POWER_ULTIMATE)
    }

    /// Clé de traduction d'un mode d'alimentation.
    pub fn power_key(plan: &str) -> &'static str {
        match plan {
            "381b4222f69441f09685ff5bb260df2e" => "power-plan-balanced",
            "a184130835414fabbc81f71556f20b4a" => "power-plan-saver",
            POWER_HIGH | POWER_ULTIMATE => "power-plan-high",
            _ => "power-plan-other",
        }
    }
}

/// Gigaoctets avec une décimale, pour l'affichage.
pub fn gb(bytes: u64) -> String {
    format!("{:.1}", bytes as f64 / f64::from(1 << 30))
}

/// Connexion à Internet.
#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct NetStatus {
    pub kind: String,
    pub internet: bool,
    pub ssid: Option<String>,
}

/// Radio Wi-Fi ou Bluetooth.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RadioInfo {
    pub kind: String,
    pub on: bool,
}

/// Réseau Wi-Fi visible.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WifiNetwork {
    pub ssid: String,
    pub bars: u8,
    pub secure: bool,
    pub connected: bool,
}

/// Appareil Bluetooth.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BtDevice {
    pub id: String,
    pub name: String,
    pub paired: bool,
    pub connected: bool,
}

/// Réglages, miroir de `focus-app`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    pub language: Option<String>,
    pub first_run_done: bool,
    pub real_optimization: bool,
    pub optimization: String,
    pub high_performance: bool,
    pub keep_awake: bool,
    pub keep_social: bool,
    pub block_big_picture: bool,
    pub console_mode: bool,
    pub stop_shell: bool,
    pub hide_taskbar: bool,
    pub silence_notifications: bool,
    pub glyphs: String,
    pub controller_mode: Option<String>,
    pub sounds: bool,
    pub music: bool,
    pub vibration: bool,
    pub reduce_motion: bool,
    pub auto_theme: bool,
    pub autostart_delay: u32,
    pub start_minimized: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: None,
            first_run_done: false,
            real_optimization: false,
            optimization: "off".to_owned(),
            high_performance: false,
            keep_awake: true,
            keep_social: true,
            block_big_picture: true,
            console_mode: true,
            stop_shell: true,
            hide_taskbar: true,
            silence_notifications: true,
            glyphs: "auto".to_owned(),
            controller_mode: None,
            sounds: true,
            music: true,
            vibration: true,
            reduce_motion: false,
            auto_theme: false,
            autostart_delay: 3,
            start_minimized: false,
        }
    }
}

fn tauri() -> Option<JsValue> {
    let window = web_sys::window()?;
    let value = Reflect::get(&window, &"__TAURI__".into()).ok()?;
    (!value.is_undefined()).then_some(value)
}

/// Appelle une commande.
pub async fn invoke<T: DeserializeOwned>(command: &str, args: Value) -> Result<T, String> {
    let Some(tauri) = tauri() else {
        return serde_json::from_value(demo(command)).map_err(|e| e.to_string());
    };
    let core = Reflect::get(&tauri, &"core".into()).map_err(|_| "no core".to_owned())?;
    let invoke = Reflect::get(&core, &"invoke".into()).map_err(|_| "no invoke".to_owned())?;
    let invoke: Function = invoke.unchecked_into();
    let payload = args
        .serialize(&serde_wasm_bindgen::Serializer::json_compatible())
        .map_err(|e| e.to_string())?;
    let promise = invoke
        .call2(&core, &command.into(), &payload)
        .map_err(|e| format!("{e:?}"))?;
    let value = JsFuture::from(Promise::from(promise))
        .await
        .map_err(|e| e.as_string().unwrap_or_else(|| format!("{e:?}")))?;
    if value.is_null() || value.is_undefined() {
        return serde_json::from_value(Value::Null).map_err(|e| e.to_string());
    }
    serde_wasm_bindgen::from_value(value).map_err(|e| e.to_string())
}

/// Écoute un événement du cœur.
pub fn listen(event: &str, callback: impl Fn(JsValue) + 'static) {
    let Some(tauri) = tauri() else { return };
    let closure = Closure::<dyn Fn(JsValue)>::new(move |value: JsValue| {
        callback(Reflect::get(&value, &"payload".into()).unwrap_or(JsValue::NULL));
    });
    let Ok(module) = Reflect::get(&tauri, &"event".into()) else {
        return;
    };
    let Ok(listen) = Reflect::get(&module, &"listen".into()) else {
        return;
    };
    let listen: Function = listen.unchecked_into();
    let _ = listen.call2(&module, &event.into(), closure.as_ref().unchecked_ref());
    // L'écouteur vit aussi longtemps que l'application.
    closure.forget();
}

fn demo_entry(id: &str, title: &str, kind: &str, store: &str, hours: u64, last: Option<i64>) -> Value {
    json!({ "entry": {
        "id": id, "title": title, "kind": kind, "store": store,
        "target": { "type": "protocol", "uri": "steam://rungameid/1" },
        "installDir": "D:\\Games\\Demo", "cover": null, "favorite": false,
        "stats": { "totalSeconds": hours * 3600, "lastPlayed": last, "launches": 3 }
    }, "alternates": [], "art": { "accent": null } })
}

fn demo(command: &str) -> Value {
    match command {
        "boot_info" => json!({ "autostartDisabled": false }),
        "get_settings" => serde_json::to_value(Settings {
            sounds: false,
            music: false,
            reduce_motion: true,
            ..Settings::default()
        })
        .unwrap_or_default(),
        "list_entries" => json!([
            demo_entry("demo:1", "Northern Lights", "game", "steam", 12, Some(1_790_000_000)),
            demo_entry("demo:2", "Harbor Tales", "game", "epic", 3, Some(1_780_000_000)),
            demo_entry("demo:3", "Crimson Orbit", "game", "gog", 41, None),
            demo_entry("demo:4", "Salt Road", "game", "manual", 1, None),
            demo_entry("demo:5", "Paper Foxes", "game", "steam", 0, None),
            demo_entry("demo:6", "Sculpt Studio", "software", "manual", 8, None),
            demo_entry("demo:7", "Voice Room", "social", "manual", 0, None),
        ]),
        "refresh_library" | "list_drives" | "list_dir" | "suggest_executables"
        | "audio_sessions" | "bt_nearby" => json!([]),
        "preview_session" => json!({
            "plan": { "steps": [{ "step": "keepAwake" }], "askUser": [], "simulationOnly": true },
            "simulation": true, "guardian": false
        }),
        "running_entry" => Value::Null,
        "steam_guide_status" => json!(true),
        "catalog" => {
            json!([{ "id": "discord", "name": "Discord", "category": "social", "publisher": "Discord", "winget": true }])
        }
        "controller_status" => json!({ "family": "other", "mode": "native", "battery": null }),
        "system_report" => json!({
            "hardware": { "cpu": "Demo CPU", "memory": 34359738368_u64 },
            "processes": 214, "services": 287, "top": [["demo", 1073741824_u64]]
        }),
        "downloads" => json!({
            "steam": [
                { "appid": "20", "name": "Northern Lights", "done": 6_442_450_944_u64, "total": 16_106_127_360_u64 },
                { "appid": "30", "name": "Paper Foxes", "done": 0, "total": 0 }
            ],
            "installs": [{ "id": "discord", "name": "Discord", "state": "running" }]
        }),
        "app_version" => json!("0.1.0"),
        "update_check" => json!({ "version": "0.2.0", "notes": "" }),
        "console_report" | "close_open_apps" => json!({
            "level": "max", "shellStopped": true,
            "powerBefore": "381b4222f69441f09685ff5bb260df2e", "powerAfter": POWER_HIGH,
            "memoryTotal": 34359738368_u64, "memoryBefore": 19756849562_u64, "memoryAfter": 22011707392_u64,
            "slowed": 31, "closed": 4, "paused": 7, "trimmed": 58, "failed": 0,
            "apps": if command == "console_report" { json!(["chrome", "notepad"]) } else { json!([]) },
            "millis": 2800
        }),
        "net_status" => json!({ "kind": "wifi", "internet": true, "ssid": "Home" }),
        "radios" => json!([{ "kind": "wifi", "on": true }, { "kind": "bluetooth", "on": true }]),
        "wifi_networks" => json!([
            { "ssid": "Home", "bars": 4, "secure": true, "connected": true },
            { "ssid": "Cafe", "bars": 2, "secure": false, "connected": false },
            { "ssid": "Neighbour 5G", "bars": 1, "secure": true, "connected": false }
        ]),
        "bt_paired" => json!([
            { "id": "bt1", "name": "Wireless Controller", "paired": true, "connected": true },
            { "id": "bt2", "name": "Headset", "paired": true, "connected": false }
        ]),
        "wifi_connect" | "bt_pair" | "bt_unpair" => json!("success"),
        "guardian_status" | "set_autostart" | "apply_console" | "set_radio" => json!(false),
        _ => Value::Null,
    }
}
