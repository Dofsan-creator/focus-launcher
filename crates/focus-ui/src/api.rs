//! Pont vers le cœur Rust. Hors de Tauri (tests dans un navigateur), des données
//! de démonstration répondent : aucune machine n'est touchée.

use js_sys::{Function, Promise, Reflect};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;

/// Ligne de bibliothèque, avec ses autres sources.
#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Merged {
    pub entry: focus_core::model::Entry,
    pub alternates: Vec<focus_core::model::Entry>,
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
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatus {
    pub family: String,
    pub mode: String,
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

/// Réglages, miroir de `focus-app`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Settings {
    pub language: Option<String>,
    pub first_run_done: bool,
    pub real_optimization: bool,
    pub high_performance: bool,
    pub keep_awake: bool,
    pub keep_social: bool,
    pub block_big_picture: bool,
    pub glyphs: String,
    pub controller_mode: Option<String>,
    pub sounds: bool,
    pub reduce_motion: bool,
    pub auto_theme: bool,
}

fn tauri() -> Option<JsValue> {
    let window = web_sys::window()?;
    let value = Reflect::get(&window, &"__TAURI__".into()).ok()?;
    (!value.is_undefined()).then_some(value)
}

/// Vrai hors de Tauri.
pub fn is_demo() -> bool {
    tauri().is_none()
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
    let Ok(module) = Reflect::get(&tauri, &"event".into()) else { return };
    let Ok(listen) = Reflect::get(&module, &"listen".into()) else { return };
    let listen: Function = listen.unchecked_into();
    let _ = listen.call2(&module, &event.into(), closure.as_ref().unchecked_ref());
    // L'écouteur vit aussi longtemps que l'application.
    closure.forget();
}

fn demo_entry(id: &str, title: &str, kind: &str, store: &str, hours: u64) -> Value {
    json!({ "entry": {
        "id": id, "title": title, "kind": kind, "store": store,
        "target": { "type": "protocol", "uri": "steam://rungameid/1" },
        "installDir": "D:\\Games\\Demo", "cover": null, "favorite": false,
        "stats": { "totalSeconds": hours * 3600, "lastPlayed": null, "launches": 3 }
    }, "alternates": [] })
}

fn demo(command: &str) -> Value {
    match command {
        "boot_info" => json!({ "autostartDisabled": false }),
        "get_settings" => json!({
            "language": null, "firstRunDone": false, "realOptimization": false, "highPerformance": false,
            "keepAwake": true, "keepSocial": true, "blockBigPicture": true, "glyphs": "auto",
            "controllerMode": null, "sounds": false, "reduceMotion": true, "autoTheme": false
        }),
        "list_entries" => json!([
            demo_entry("demo:1", "Northern Lights", "game", "steam", 12),
            demo_entry("demo:2", "Harbor Tales", "game", "epic", 3),
            demo_entry("demo:3", "Crimson Orbit", "game", "gog", 41),
            demo_entry("demo:4", "Salt Road", "game", "manual", 1),
            demo_entry("demo:5", "Paper Foxes", "game", "steam", 0),
            demo_entry("demo:6", "Sculpt Studio", "software", "manual", 8),
            demo_entry("demo:7", "Voice Room", "social", "manual", 0),
        ]),
        "refresh_library" | "list_drives" | "list_dir" | "suggest_executables" | "audio_sessions" => json!([]),
        "preview_session" => json!({
            "plan": { "steps": [{ "step": "keepAwake" }], "askUser": [], "simulationOnly": true },
            "simulation": true, "guardian": false
        }),
        "running_entry" => Value::Null,
        "catalog" => json!([{ "id": "discord", "name": "Discord", "category": "social", "publisher": "Discord", "winget": true }]),
        "controller_status" => json!({ "family": "other", "mode": "native" }),
        "system_report" => json!({
            "hardware": { "cpu": "Demo CPU", "memory": 34359738368_u64 },
            "processes": 214, "services": 287, "top": [["demo", 1073741824_u64]]
        }),
        "guardian_status" | "set_autostart" => json!(false),
        _ => Value::Null,
    }
}
