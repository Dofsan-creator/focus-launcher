//! Commandes appelées par l'interface.

use crate::boot_guard::BootStatus;
use crate::runtime::{App, AppError, Running};
use crate::settings::Settings;
use focus_core::model::{Entry, EntryKind, LaunchTarget, Stats, Store};
use focus_core::plan::Plan;
use focus_library::catalog::{self, CatalogApp};
use focus_library::merge::{merge_duplicates, MergedEntry};
use focus_library::{discover, exe_score};
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tauri::{AppHandle, Emitter, Manager, State};

type Shared<'a> = State<'a, Arc<App>>;
type CmdResult<T> = Result<T, AppError>;

/// Fait passer le launcher au premier plan.
pub fn show_launcher(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.show();
        let _ = window.set_focus();
    }
}

#[tauri::command]
pub fn boot_info(boot: State<'_, BootStatus>) -> BootStatus {
    *boot
}

#[tauri::command]
pub fn get_settings(state: Shared<'_>) -> Settings {
    state.settings()
}

#[tauri::command]
pub fn set_settings(state: Shared<'_>, settings: Settings) -> CmdResult<()> {
    state.save_settings(settings)
}

#[tauri::command]
pub fn list_entries(state: Shared<'_>) -> CmdResult<Vec<MergedEntry>> {
    Ok(merge_duplicates(state.repo().all()?))
}

/// Relit toutes les boutiques, dans un fil pour ne pas bloquer l'interface.
#[tauri::command]
pub async fn refresh_library(app: AppHandle, state: Shared<'_>) -> CmdResult<Vec<String>> {
    let shared = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || scan_into(&shared, &app))
        .await
        .map_err(|err| AppError::Other(err.to_string()))?
}

/// Synchronise la bibliothèque et signale les boutiques illisibles.
pub fn scan_into(app_state: &App, app: &AppHandle) -> CmdResult<Vec<String>> {
    #[cfg(windows)]
    let start_apps = focus_sys::report::start_apps();
    #[cfg(not(windows))]
    let start_apps = Vec::new();
    let mut failed = Vec::new();
    for scan in discover::scan_all(&start_apps) {
        if let Some(error) = scan.error {
            tracing::warn!("{} scan failed: {error}", scan.prefix);
            failed.push(scan.prefix.to_owned());
            continue;
        }
        app_state.repo().sync_store(scan.prefix, scan.entries)?;
        let _ = app.emit("library-changed", scan.prefix);
    }
    Ok(failed)
}

/// Ce qui serait fait, sans rien faire.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Preview {
    plan: Plan,
    simulation: bool,
    guardian: bool,
}

#[tauri::command]
pub fn preview_session(state: Shared<'_>, id: String) -> CmdResult<Preview> {
    let entry = state.repo().get(&id)?.ok_or(AppError::UnknownEntry(id))?;
    let plan = state.plan_for(&entry)?;
    Ok(Preview {
        simulation: plan.simulation_only,
        plan,
        guardian: guardian_running(),
    })
}

#[tauri::command]
pub fn play(app: AppHandle, state: Shared<'_>, id: String) -> CmdResult<()> {
    let handle = app.clone();
    state.play(&id, move |end| {
        show_launcher(&handle);
        let _ = handle.emit("session-ended", end);
    })?;
    if let Some(window) = app.get_webview_window("main") {
        // Une fenêtre cachée ne dessine plus : c'est la pause du rendu.
        let _ = window.hide();
    }
    Ok(())
}

#[tauri::command]
pub fn running_entry(state: Shared<'_>) -> Option<Running> {
    state.running()
}

#[tauri::command]
pub fn return_to_running(app: AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.hide();
    }
}

/// Fermeture douce : la seule demandée par défaut.
#[tauri::command]
pub fn close_running(state: Shared<'_>) -> bool {
    #[cfg(windows)]
    if let Some(running) = state.running() {
        let mut asked = running.pid.is_some_and(focus_sys::ops::request_close);
        for process in focus_sys::scan::processes() {
            if running.watched.contains(&process.name) {
                asked |= focus_sys::ops::request_close(process.pid);
            }
        }
        return asked;
    }
    false
}

/// Fermeture forcée de l'arbre du jeu seulement, jamais du client de boutique.
#[tauri::command]
pub fn force_close_running(state: Shared<'_>) -> CmdResult<()> {
    #[cfg(windows)]
    if let Some(running) = state.running() {
        for process in focus_sys::scan::processes() {
            if running.watched.contains(&process.name)
                && focus_core::whitelist::store_of_process(&process.name).is_none()
                && !focus_core::whitelist::is_protected_process(&process.name)
            {
                focus_sys::ops::terminate(process.pid).map_err(|e| AppError::Other(e.0))?;
            }
        }
    }
    Ok(())
}

fn valid_target(path: &str) -> bool {
    let lower = path.to_lowercase();
    [".exe", ".lnk", ".bat", ".cmd"]
        .iter()
        .any(|ext| lower.ends_with(ext))
        && !exe_score::is_protected_dir(Path::new(path))
}

/// Ajoute un fichier choisi par l'utilisateur. Il n'est jamais exécuté pour l'inspecter.
#[tauri::command]
pub fn add_file(
    state: Shared<'_>,
    path: String,
    title: String,
    kind: EntryKind,
) -> CmdResult<Entry> {
    if !valid_target(&path) {
        return Err(AppError::Other(format!("unsupported file: {path}")));
    }
    let file = PathBuf::from(&path);
    let dir = file.parent().map(|p| p.display().to_string());
    let entry = Entry {
        id: format!("manual:{}", path.to_lowercase()),
        title,
        kind,
        store: Store::Manual,
        target: LaunchTarget::Executable {
            path,
            args: Vec::new(),
            working_dir: dir.clone(),
        },
        install_dir: dir,
        cover: None,
        favorite: false,
        stats: Stats::default(),
    };
    state.repo().save(&entry)?;
    Ok(entry)
}

#[tauri::command]
pub fn remove_entry(state: Shared<'_>, id: String) -> CmdResult<()> {
    Ok(state.repo().delete(&id)?)
}

#[tauri::command]
pub fn set_favorite(state: Shared<'_>, id: String, favorite: bool) -> CmdResult<()> {
    let repo = state.repo();
    let mut entry = repo.get(&id)?.ok_or(AppError::UnknownEntry(id))?;
    entry.favorite = favorite;
    Ok(repo.save_keeping_source(&entry)?)
}

/// Une ligne de l'explorateur intégré.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirItem {
    name: String,
    path: String,
    is_dir: bool,
}

#[tauri::command]
pub fn list_drives() -> Vec<DirItem> {
    ('A'..='Z')
        .map(|letter| format!("{letter}:\\"))
        .filter(|root| Path::new(root).is_dir())
        .map(|root| DirItem {
            name: root.clone(),
            path: root,
            is_dir: true,
        })
        .collect()
}

#[tauri::command]
pub fn list_dir(path: String) -> CmdResult<Vec<DirItem>> {
    if exe_score::is_protected_dir(Path::new(&path)) {
        return Ok(Vec::new());
    }
    let mut items: Vec<DirItem> = std::fs::read_dir(&path)?
        .flatten()
        .filter_map(|entry| {
            let is_dir = entry.file_type().ok()?.is_dir();
            let full = entry.path().display().to_string();
            (is_dir || valid_target(&full)).then(|| DirItem {
                name: entry.file_name().to_string_lossy().into_owned(),
                path: full,
                is_dir,
            })
        })
        .collect();
    items.sort_by_key(|item| (!item.is_dir, item.name.to_lowercase()));
    Ok(items)
}

/// Les trois exécutables les plus probables d'un dossier.
#[tauri::command]
pub async fn suggest_executables(path: String) -> Vec<String> {
    tauri::async_runtime::spawn_blocking(move || {
        exe_score::best_candidates(Path::new(&path), exe_score::DEFAULT_DEPTH, 3, &|| false)
            .into_iter()
            .map(|candidate| candidate.path.display().to_string())
            .collect()
    })
    .await
    .unwrap_or_default()
}

/// Application du catalogue avec son état.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CatalogItem {
    #[serde(flatten)]
    app: CatalogApp,
    winget: bool,
}

#[tauri::command]
pub fn catalog() -> CmdResult<Vec<CatalogItem>> {
    #[cfg(windows)]
    let winget = focus_sys::winget::available();
    #[cfg(not(windows))]
    let winget = false;
    let apps = catalog::load().map_err(|e| AppError::Other(e.to_string()))?;
    Ok(apps
        .into_iter()
        .map(|app| CatalogItem { app, winget })
        .collect())
}

/// Installe par winget. L'identifiant vient du catalogue, jamais de l'interface.
#[tauri::command]
pub fn install_app(app: AppHandle, id: String) -> CmdResult<()> {
    let apps = catalog::load().map_err(|e| AppError::Other(e.to_string()))?;
    let item = apps
        .into_iter()
        .find(|a| a.id == id)
        .ok_or(AppError::UnknownEntry(id))?;
    std::thread::spawn(move || {
        #[cfg(windows)]
        let result = focus_sys::winget::install(&item.winget_id);
        #[cfg(not(windows))]
        let result: Result<(), String> = Err("Windows only".to_owned());
        let payload =
            serde_json::json!({ "id": item.id, "ok": result.is_ok(), "error": result.err() });
        tracing::info!("install finished: {payload}");
        let _ = app.emit("install-finished", payload);
    });
    Ok(())
}

/// Ouvre la page officielle, prise dans le catalogue.
#[tauri::command]
pub fn open_official_page(id: String) -> CmdResult<()> {
    let apps = catalog::load().map_err(|e| AppError::Other(e.to_string()))?;
    let item = apps
        .into_iter()
        .find(|a| a.id == id)
        .ok_or(AppError::UnknownEntry(id))?;
    #[cfg(windows)]
    focus_sys::launch::start(&LaunchTarget::Website {
        url: item.official_page,
    })
    .map_err(|e| AppError::Other(e.to_string()))?;
    #[cfg(not(windows))]
    drop(item);
    Ok(())
}

/// Rapport d'analyse, en lecture seule.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemReport {
    #[cfg(windows)]
    hardware: focus_sys::report::Hardware,
    processes: usize,
    services: usize,
    top: Vec<(String, u64)>,
}

#[tauri::command]
pub async fn system_report() -> Option<SystemReport> {
    #[cfg(windows)]
    {
        tauri::async_runtime::spawn_blocking(|| {
            let snapshot = focus_sys::scan::snapshot();
            let mut top: Vec<(String, u64)> = snapshot
                .processes
                .iter()
                .map(|p| (p.name.clone(), p.memory))
                .collect();
            top.sort_by_key(|(_, memory)| std::cmp::Reverse(*memory));
            top.truncate(10);
            SystemReport {
                hardware: focus_sys::report::hardware(),
                processes: snapshot.processes.len(),
                services: snapshot.services.len(),
                top,
            }
        })
        .await
        .ok()
    }
    #[cfg(not(windows))]
    None
}

/// Coche ou décoche le lancement à l'ouverture de session et renvoie l'état réel.
#[tauri::command]
pub fn set_autostart(enabled: Option<bool>) -> CmdResult<bool> {
    #[cfg(windows)]
    {
        if let Some(enabled) = enabled {
            let exe = std::env::current_exe()?;
            focus_sys::shell::set_autostart(enabled, &exe)
                .map_err(|e| AppError::Other(e.to_string()))?;
        }
        Ok(focus_sys::shell::autostart_enabled())
    }
    #[cfg(not(windows))]
    {
        let _ = enabled;
        Ok(false)
    }
}

fn guardian_running() -> bool {
    #[cfg(windows)]
    {
        Path::new(focus_guardian::PIPE_NAME).exists()
    }
    #[cfg(not(windows))]
    false
}

#[tauri::command]
pub fn guardian_status() -> bool {
    guardian_running()
}

/// Lance l'installation du gardien : Windows demandera l'autorisation une seule fois.
#[tauri::command]
pub fn install_guardian() -> CmdResult<()> {
    #[cfg(windows)]
    {
        let exe = std::env::current_exe()?.with_file_name("focus-guardian.exe");
        focus_sys::launch::run_elevated(&exe, "install")
            .map_err(|e| AppError::Other(e.to_string()))?;
    }
    Ok(())
}

/// État des manettes.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ControllerStatus {
    family: focus_input::pad::Family,
    env: focus_input::modes::Environment,
    mode: focus_input::modes::Mode,
}

#[tauri::command]
pub fn controller_status(state: Shared<'_>) -> Option<ControllerStatus> {
    #[cfg(windows)]
    {
        let family = focus_input::source::detect_family();
        let env = focus_input::modes::Environment {
            steam_running: focus_sys::scan::is_running("steam"),
            vigem_present: focus_input::source::vigem_present(),
            hidhide_present: Path::new(r"C:\Program Files\Nefarius Software Solutions\HidHide")
                .is_dir(),
        };
        let mode = focus_input::modes::resolve_mode(family, env, state.settings().controller_mode);
        Some(ControllerStatus { family, env, mode })
    }
    #[cfg(not(windows))]
    {
        let _ = state;
        None
    }
}

/// Quitter, redémarrer, éteindre. L'interface a déjà demandé confirmation.
#[tauri::command]
pub fn power_action(app: AppHandle, action: String) -> CmdResult<()> {
    match action.as_str() {
        "quit" => app.exit(0),
        #[cfg(windows)]
        "restart" | "shutdown" => {
            let flag = if action == "restart" { "/r" } else { "/s" };
            std::process::Command::new("shutdown")
                .args([flag, "/t", "0"])
                .spawn()?;
        }
        other => return Err(AppError::Other(format!("unknown action {other}"))),
    }
    Ok(())
}

#[tauri::command]
pub fn audio_sessions() -> Vec<(u32, f32)> {
    #[cfg(windows)]
    {
        focus_sys::audio::sessions()
            .into_iter()
            .map(|s| (s.pid, s.volume))
            .collect()
    }
    #[cfg(not(windows))]
    Vec::new()
}

#[tauri::command]
pub fn set_app_volume(pid: u32, level: f32) -> CmdResult<()> {
    #[cfg(windows)]
    focus_sys::audio::set_volume(pid, level).map_err(|e| AppError::Other(e.to_string()))?;
    #[cfg(not(windows))]
    let _ = (pid, level);
    Ok(())
}
