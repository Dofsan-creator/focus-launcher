//! Commandes appelées par l'interface.

use crate::boot_guard::{BootStatus, UiStarted};
use crate::console::{Console, Wanted};
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
    // Un jeu garde le premier plan : sans cela, Échap et le clavier vont au jeu.
    #[cfg(windows)]
    std::thread::spawn(|| {
        for _ in 0..3 {
            std::thread::sleep(std::time::Duration::from_millis(80));
            if focus_sys::console::bring_own_to_front() {
                break;
            }
        }
    });
}

/// La fenêtre de Focus est-elle affichée (par-dessus un jeu, par exemple) ?
pub fn launcher_visible(app: &AppHandle) -> bool {
    app.get_webview_window("main")
        .and_then(|window| window.is_visible().ok())
        .unwrap_or(false)
}

/// Appelée par l'interface à son démarrage : la preuve que le WebAssembly
/// tourne. Elle désarme la fermeture du garde-fou (`boot_guard::watch`).
#[tauri::command]
pub fn boot_info(boot: State<'_, BootStatus>, ui: State<'_, UiStarted>) -> BootStatus {
    if ui.mark() {
        tracing::info!("interface started");
    }
    *boot
}

#[tauri::command]
pub fn get_settings(state: Shared<'_>) -> Settings {
    state.settings()
}

#[tauri::command(async)]
pub fn set_settings(state: Shared<'_>, settings: Settings) -> CmdResult<()> {
    state.save_settings(settings)
}

/// Ligne de bibliothèque, avec ce qu'on peut afficher comme images.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    #[serde(flatten)]
    merged: MergedEntry,
    art: focus_core::art::ArtInfo,
}

#[tauri::command(async)]
pub fn list_entries(state: Shared<'_>) -> CmdResult<Vec<LibraryItem>> {
    let data = state.data_dir().to_path_buf();
    Ok(merge_duplicates(state.repo().all()?)
        .into_iter()
        .map(|merged| LibraryItem {
            art: crate::art::info(&data, &merged.entry.id),
            merged,
        })
        .collect())
}

/// Relit toutes les boutiques, dans un fil pour ne pas bloquer l'interface.
#[tauri::command]
pub async fn refresh_library(app: AppHandle, state: Shared<'_>) -> CmdResult<Vec<String>> {
    let shared = state.inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let failed = scan_into(&shared, &app);
        crate::art::refresh(&shared, &app);
        failed
    })
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

#[tauri::command(async)]
pub fn play(app: AppHandle, state: Shared<'_>, id: String) -> CmdResult<()> {
    let handle = app.clone();
    state.play(&id, move |end| {
        if let Some(window) = handle.get_webview_window("main") {
            let _ = window.set_always_on_top(false);
        }
        show_launcher(&handle);
        let _ = handle.emit("session-ended", end);
    })?;
    // Écran de chargement : Focus reste devant jusqu’à ce que le jeu soit vraiment
    // là, puis s’efface. Une fenêtre cachée ne dessine plus : c’est la pause du rendu.
    let shared = state.inner().clone();
    std::thread::spawn(move || {
        #[cfg(windows)]
        wait_for_start(&shared);
        // Le jeu s'est fermé avant d'apparaître (plantage) : l'écran de fin reste.
        if shared.running().is_none() {
            return;
        }
        // L’interface quitte l’écran de chargement avant que la fenêtre se cache :
        // au retour (bouton central), c’est l’accueil ou le menu qui s’affiche.
        let _ = app.emit("session-started", ());
        std::thread::sleep(std::time::Duration::from_millis(150));
        hide_launcher(&app);
    });
    Ok(())
}

/// Écran de lancement, comme sur une console : Focus reste en fond tant que le jeu
/// n'occupe pas l'écran. Les fenêtres intermédiaires (anticheat, lanceur où il faut
/// cliquer) passent devant Focus, jamais devant le bureau. Aucun processus n'est
/// ouvert : seulement des noms et des fenêtres (SPEC D7).
#[cfg(windows)]
fn wait_for_start(state: &App) {
    use std::time::{Duration, Instant};
    /// Part de l'écran à partir de laquelle une fenêtre est le jeu, pas un lanceur.
    const GAME_COVERAGE: f32 = 0.6;
    const LIMIT: Duration = Duration::from_secs(180);
    let started = Instant::now();
    let own = std::process::id();
    let mut raised: Vec<isize> = Vec::new();
    focus_sys::console::allow_any_foreground();
    while started.elapsed() < LIMIT {
        let Some(running) = state.running() else { return };
        let processes = focus_sys::scan::process_ids();
        let name_of = |pid: u32| processes.iter().find(|(id, _)| *id == pid).map(|(_, name)| name.clone());
        let mut pids: Vec<u32> = processes
            .iter()
            .filter(|(_, name)| running.watched.contains(name))
            .map(|(pid, _)| *pid)
            .collect();
        pids.extend(running.pid);
        let windows = focus_sys::console::shown_windows(&pids);
        let is_launcher = |pid: u32| name_of(pid).is_some_and(|name| crate::runtime::is_launcher(&name));
        // Le jeu occupe l'écran : Focus peut s'effacer. Un lanceur ne compte pas, même
        // avec une grande fenêtre transparente.
        if let Some(game) = windows
            .iter()
            .find(|window| window.coverage >= GAME_COVERAGE && !is_launcher(window.pid))
        {
            if let Some(name) = name_of(game.pid) {
                state.set_main(name);
            }
            std::thread::sleep(Duration::from_millis(300));
            return;
        }
        // Jeu lancé par sa boutique sous un autre nom : une grande fenêtre étrangère
        // au premier plan. Le bureau, les boutiques et Focus ne comptent pas.
        if let Some(front) = focus_sys::console::foreground_window() {
            let name = name_of(front.pid).unwrap_or_default();
            let ignored = front.pid == own
                || crate::runtime::is_launcher(&name)
                || name == "explorer"
                || STORE_PROCESSES.contains(&name.as_str())
                || name.contains("webhelper");
            if !ignored && front.coverage >= GAME_COVERAGE && started.elapsed() > Duration::from_secs(2) {
                state.set_main(name);
                std::thread::sleep(Duration::from_millis(300));
                return;
            }
        }
        // Boutique qui s'ouvre en grand pendant le lancement (Steam qui démarre, ses
        // publicités) : réduite, l'écran de lancement reste visible. Ses petites
        // fenêtres de progression restent affichées.
        let stores: Vec<u32> = processes
            .iter()
            .filter(|(_, name)| STORE_PROCESSES.contains(&name.as_str()))
            .map(|(pid, _)| *pid)
            .collect();
        for window in focus_sys::console::shown_windows(&stores) {
            if window.coverage >= 0.2 {
                focus_sys::console::minimize_window(window.hwnd);
            }
        }
        // Lanceur ou anticheat ouvert derrière Focus : on le montre, une seule fois.
        for window in &windows {
            if !raised.contains(&window.hwnd) {
                raised.push(window.hwnd);
                focus_sys::console::bring_to_front(window.hwnd);
            }
        }
        std::thread::sleep(Duration::from_millis(250));
    }
}

/// Ouvre une boutique déjà installée, par son adresse officielle.
#[tauri::command(async)]
pub fn open_store(id: String) -> CmdResult<()> {
    STORE_OPEN.store(true, std::sync::atomic::Ordering::SeqCst);
    let uri = match id.as_str() {
        "steam" => "steam://open/main",
        "epic" => "com.epicgames.launcher://store",
        "gog" => "goggalaxy://openGalaxyClient",
        "ubisoft" => "uplay://",
        "battlenet" => "battlenet://",
        "ea" => "origin2://",
        "discord" => "discord://",
        _ => return Err(AppError::UnknownEntry(id)),
    };
    #[cfg(windows)]
    focus_sys::launch::start(&LaunchTarget::Protocol { uri: uri.to_owned() })
        .map_err(|e| AppError::Other(e.to_string()))?;
    #[cfg(not(windows))]
    let _ = uri;
    Ok(())
}

/// Steam ouvre-t-il son interface au bouton central ? Lecture seule ; `None` sans Steam.
#[tauri::command]
pub async fn steam_guide_status() -> Option<bool> {
    tauri::async_runtime::spawn_blocking(|| {
        focus_library::scanners::steam_guide_focus(&discover::steam_root()?)
    })
    .await
    .ok()
    .flatten()
}

/// Ouvre la page Contrôleur des paramètres de Steam, pour que l'utilisateur
/// change lui-même le réglage du bouton central.
#[tauri::command(async)]
pub fn open_steam_controller_settings() -> CmdResult<()> {
    STORE_OPEN.store(true, std::sync::atomic::Ordering::SeqCst);
    #[cfg(windows)]
    focus_sys::launch::start(&LaunchTarget::Protocol { uri: "steam://settings/controller".to_owned() })
        .map_err(|e| AppError::Other(e.to_string()))?;
    Ok(())
}

#[tauri::command]
pub fn running_entry(state: Shared<'_>) -> Option<Running> {
    state.running()
}

#[tauri::command]
pub fn return_to_running(app: AppHandle) {
    hide_launcher(&app);
}

/// Cache Focus et lui retire le premier rang : le jeu reprend sa place.
pub fn hide_launcher(app: &AppHandle) {
    if let Some(window) = app.get_webview_window("main") {
        let _ = window.set_always_on_top(false);
        let _ = window.hide();
    }
}

/// Fermeture douce : la seule demandée par défaut.
#[tauri::command]
pub fn close_running(state: Shared<'_>) -> bool {
    // Demande de fermeture à la fenêtre (comme Alt+F4) : aucun processus n'est ouvert.
    // Jeu à l'écran : seul le jeu est visé, le lanceur reste. Sinon, le lanceur.
    #[cfg(windows)]
    if let Some(running) = state.running() {
        let targets: Vec<String> = match &running.main {
            Some(main) => vec![main.clone()],
            None => running.watched.clone(),
        };
        let mut asked = running.main.is_none() && running.pid.is_some_and(focus_sys::ops::request_close);
        for (pid, name) in focus_sys::scan::process_ids() {
            if targets.contains(&name) {
                asked |= focus_sys::ops::request_close(pid);
            }
        }
        return asked;
    }
    false
}

/// Fermeture forcée de l'arbre du jeu seulement, jamais du client de boutique.
#[tauri::command(async)]
pub fn force_close_running(state: Shared<'_>) -> CmdResult<()> {
    #[cfg(windows)]
    if let Some(running) = state.running() {
        for (pid, name) in focus_sys::scan::process_ids() {
            // Jeu protégé par un anticheat : jamais de handle sur lui (SPEC D7), seul
            // son lanceur peut être arrêté de force.
            let allowed = !running.anticheat || crate::runtime::is_launcher(&name);
            if allowed
                && running.watched.contains(&name)
                && focus_core::whitelist::store_of_process(&name).is_none()
                && !focus_core::whitelist::is_protected_process(&name)
            {
                focus_sys::ops::terminate(pid).map_err(|e| AppError::Other(e.0))?;
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
#[tauri::command(async)]
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

#[tauri::command(async)]
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

#[tauri::command(async)]
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
    /// Déjà présent sur le PC.
    installed: bool,
    /// Icône lue dans le programme installé.
    art: focus_core::art::ArtInfo,
}

#[tauri::command]
pub async fn catalog(state: Shared<'_>) -> CmdResult<Vec<CatalogItem>> {
    let data = state.data_dir().to_path_buf();
    tauri::async_runtime::spawn_blocking(move || catalog_items(&data))
        .await
        .map_err(|err| AppError::Other(err.to_string()))?
}

/// Exécutable d’une boutique installée, pour son icône.
fn store_exe(id: &str, process: &str, locations: &[(String, String)], name: &str) -> Option<PathBuf> {
    let program_files = std::env::var_os("ProgramFiles(x86)").map(PathBuf::from).unwrap_or_default();
    match id {
        "steam" => return discover::steam_root().map(|root| root.join("steam.exe")),
        "epic" => {
            let base = program_files.join("Epic Games/Launcher/Portal/Binaries");
            let other = std::env::var_os("ProgramFiles").map(PathBuf::from).unwrap_or_default().join("Epic Games/Launcher/Portal/Binaries");
            let found = [&base, &other]
                .iter()
                .flat_map(|root| ["Win64", "Win32"].map(|arch| root.join(arch).join("EpicGamesLauncher.exe")))
                .find(|exe| exe.is_file());
            if let Some(exe) = found {
                return Some(exe);
            }
        }
        _ => {}
    }
    let (_, dir) = locations.iter().find(|(known, dir)| known.contains(name) && !dir.is_empty())?;
    let wanted = format!("{process}.exe");
    exe_score::best_candidates(Path::new(dir), 3, 8, &|| false)
        .into_iter()
        .map(|candidate| candidate.path)
        .find(|path| path.file_name().is_some_and(|file| file.to_string_lossy().eq_ignore_ascii_case(&wanted)))
}

fn catalog_items(data: &Path) -> CmdResult<Vec<CatalogItem>> {
    #[cfg(windows)]
    let winget = focus_sys::winget::available();
    #[cfg(not(windows))]
    let winget = false;
    let apps = catalog::load().map_err(|e| AppError::Other(e.to_string()))?;
    let names = discover::installed_names();
    let locations = discover::installed_locations();
    let program_data = discover::program_data();
    Ok(apps
        .into_iter()
        .map(|app| {
            let wanted = app.name.to_lowercase();
            let installed = match app.id.as_str() {
                "steam" => discover::steam_root().is_some(),
                "epic" => program_data.join("Epic/EpicGamesLauncher").is_dir(),
                // Pilotes de Focus Input : vus par leur présence réelle, pas par leur nom.
                #[cfg(windows)]
                "hidhide" => focus_sys::hidhide::present(),
                #[cfg(windows)]
                "vigembus" => focus_input::source::vigem_present(),
                _ => false,
            } || names.iter().any(|name| name.contains(&wanted));
            let exe = installed.then(|| store_exe(&app.id, &app.process_name, &locations, &wanted)).flatten();
            let art = if installed { crate::art::store_icon(data, &app.id, exe) } else { focus_core::art::ArtInfo::default() };
            CatalogItem { app, winget, installed, art }
        })
        .collect())
}

/// Installe par winget. L'identifiant vient du catalogue, jamais de l'interface.
#[tauri::command(async)]
pub fn install_app(app: AppHandle, state: Shared<'_>, id: String) -> CmdResult<()> {
    let apps = catalog::load().map_err(|e| AppError::Other(e.to_string()))?;
    let item = apps
        .into_iter()
        .find(|a| a.id == id)
        .ok_or(AppError::UnknownEntry(id))?;
    let shared = state.inner().clone();
    shared.track_install(&item.id, &item.name, "running");
    std::thread::spawn(move || {
        #[cfg(windows)]
        let result = focus_sys::winget::install(&item.winget_id);
        #[cfg(not(windows))]
        let result: Result<(), String> = Err("Windows only".to_owned());
        shared.track_install(&item.id, &item.name, if result.is_ok() { "done" } else { "failed" });
        let payload =
            serde_json::json!({ "id": item.id, "ok": result.is_ok(), "error": result.err() });
        tracing::info!("install finished: {payload}");
        let _ = app.emit("install-finished", payload);
    });
    Ok(())
}

/// Ouvre la page officielle, prise dans le catalogue.
#[tauri::command(async)]
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

/// Dépôt public de Focus : mises à jour et signalements.
const REPO: &str = "Dofsan-creator/focus-launcher";

/// Tout ce qui se télécharge ou s'installe : Steam (lu sur le disque) et Focus.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Downloads {
    steam: Vec<focus_library::scanners::SteamDownload>,
    installs: Vec<crate::runtime::InstallJob>,
}

#[tauri::command(async)]
pub fn downloads(state: Shared<'_>) -> Downloads {
    Downloads {
        steam: discover::steam_root()
            .map(|root| focus_library::scanners::steam_downloads(&root))
            .unwrap_or_default(),
        installs: state.installs(),
    }
}

/// Version de Focus, pour l'écran d'aide.
#[tauri::command]
pub fn app_version(app: AppHandle) -> String {
    app.package_info().version.to_string()
}

/// Dernier journal de Focus, s'il existe.
fn latest_log(data: &Path) -> Option<String> {
    let newest = std::fs::read_dir(data.join("logs"))
        .ok()?
        .flatten()
        .map(|file| file.path())
        .filter(|path| path.file_name().is_some_and(|name| name.to_string_lossy().starts_with("launcher.log")))
        .max()?;
    std::fs::read_to_string(newest).ok()
}

/// Signale un problème : le rapport est nettoyé du nom et du dossier de
/// l'utilisateur, gardé sur le disque, puis la fiche préremplie s'ouvre dans le
/// navigateur. Focus n'envoie rien lui-même : l'utilisateur relit et valide.
#[tauri::command(async)]
pub fn report_problem(app: AppHandle, state: Shared<'_>, text: String) -> CmdResult<()> {
    use focus_core::report::{issue_url, problem_lines, scrub};
    let data = state.data_dir().to_path_buf();
    let settings = state.settings();
    let log = latest_log(&data).unwrap_or_default();
    #[cfg(windows)]
    let system = focus_sys::report::hardware().os;
    #[cfg(not(windows))]
    let system = String::new();
    let body = format!(
        "### Ce qui s'est passé\n{}\n\n### Informations\n- Focus {}\n- {}\n- Optimisation : {:?}, mode console : {}, bureau masqué : {}\n\n### Derniers avertissements du journal\n```\n{}\n```\n",
        text.trim(),
        app.package_info().version,
        system,
        settings.level(),
        settings.console_mode,
        settings.stop_shell,
        problem_lines(&log, 12).join("\n"),
    );
    let profile = std::env::var("USERPROFILE").unwrap_or_default();
    let user = std::env::var("USERNAME").unwrap_or_default();
    let body = scrub(&body, &profile, &user);
    let stamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs());
    std::fs::create_dir_all(data.join("reports"))?;
    std::fs::write(data.join("reports").join(format!("report-{stamp}.txt")), &body)?;
    let first_line = scrub(text.trim().lines().next().unwrap_or_default(), &profile, &user);
    let title: String = if first_line.is_empty() {
        "Problème dans Focus".to_owned()
    } else {
        first_line.chars().take(70).collect()
    };
    #[cfg(windows)]
    {
        // Le navigateur doit pouvoir passer devant Focus.
        focus_sys::console::allow_any_foreground();
        focus_sys::launch::start(&LaunchTarget::Website { url: issue_url(REPO, &title, &body) })
            .map_err(|e| AppError::Other(e.to_string()))?;
    }
    #[cfg(not(windows))]
    let _ = (title, issue_url(REPO, "", ""));
    Ok(())
}

/// Mise à jour disponible.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateInfo {
    version: String,
    notes: String,
}

fn update_error(err: impl std::fmt::Display) -> AppError {
    AppError::Other(format!("update: {err}"))
}

/// Cherche une version plus récente dans les versions publiées du dépôt. Le fichier
/// reçu doit être signé par la clé de Focus, sinon il est refusé.
#[tauri::command]
pub async fn update_check(app: AppHandle) -> CmdResult<Option<UpdateInfo>> {
    use tauri_plugin_updater::UpdaterExt;
    let update = app.updater().map_err(update_error)?.check().await.map_err(update_error)?;
    Ok(update.map(|update| UpdateInfo {
        version: update.version,
        notes: update.body.unwrap_or_default(),
    }))
}

/// Télécharge la mise à jour (progression envoyée à l'interface), rend à Windows son
/// état normal, puis lance l'installation : Focus se ferme et rouvre à jour.
#[tauri::command]
pub async fn update_install(app: AppHandle, state: Shared<'_>, console: State<'_, Console>) -> CmdResult<()> {
    use tauri_plugin_updater::UpdaterExt;
    let Some(update) = app.updater().map_err(update_error)?.check().await.map_err(update_error)? else {
        return Ok(());
    };
    let progress = app.clone();
    let mut done = 0_u64;
    let bytes = update
        .download(
            move |chunk, total| {
                done += chunk as u64;
                let _ = progress.emit("update-progress", serde_json::json!({ "done": done, "total": total.unwrap_or(0) }));
            },
            || {},
        )
        .await
        .map_err(update_error)?;
    tracing::info!("update {} downloaded, installing", update.version);
    // L'installeur remplace Focus : rien ne doit rester coupé ni masqué.
    console.leave();
    state.restore_console_plan();
    update.install(bytes).map_err(update_error)?;
    app.restart();
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
#[tauri::command(async)]
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

#[tauri::command(async)]
pub fn guardian_status() -> bool {
    guardian_running()
}

/// Lance l'installation du gardien : Windows demandera l'autorisation une seule fois.
#[tauri::command(async)]
pub fn install_guardian() -> CmdResult<()> {
    #[cfg(windows)]
    {
        // Installé : `guardianocus-guardian.exe` à côté de Focus. En développement :
        // à côté de l'exécutable.
        let here = std::env::current_exe()?;
        let bundled = here.with_file_name("guardian").join("focus-guardian.exe");
        let exe = if bundled.is_file() { bundled } else { here.with_file_name("focus-guardian.exe") };
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
    /// Batterie d'une manette XInput sans fil, de 0 à 3.
    battery: Option<u8>,
    env: focus_input::modes::Environment,
    mode: focus_input::modes::Mode,
}

#[tauri::command]
pub async fn controller_status(state: Shared<'_>) -> CmdResult<Option<ControllerStatus>> {
    let state = state.inner().clone();
    #[cfg(windows)]
    {
      tauri::async_runtime::spawn_blocking(move || {
        let family = focus_input::source::detect_family();
        let env = focus_input::modes::Environment {
            steam_running: focus_sys::scan::is_running("steam"),
            vigem_present: focus_input::source::vigem_present(),
            hidhide_present: focus_input::source::hidhide_present(),
        };
        let mode = focus_input::modes::resolve_mode(family, env, state.settings().controller_mode);
        let battery = (0..4).find_map(focus_input::source::battery);
        Some(ControllerStatus {
            family,
            battery,
            env,
            mode,
        })
      })
      .await
      .map_err(|err| AppError::Other(err.to_string()))
    }
    #[cfg(not(windows))]
    {
        let _ = state;
        Ok(None)
    }
}

/// Quitter, redémarrer, éteindre. L'interface a déjà demandé confirmation.
#[tauri::command(async)]
pub fn power_action(app: AppHandle, state: Shared<'_>, console: State<'_, Console>, action: String) -> CmdResult<()> {
    // Windows retrouve son état normal avant de quitter, redémarrer ou éteindre.
    console.leave();
    state.restore_console_plan();
    match action.as_str() {
        "quit" => app.exit(0),
        #[cfg(windows)]
        "sleep" => {
            focus_sys::console::sleep_now();
        }
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

/// Exécute un appel WinRT bloquant dans un fil neuf, jamais dans celui de la fenêtre.
#[cfg(windows)]
async fn off_thread<T: Send + 'static>(work: impl FnOnce() -> T + Send + 'static) -> Option<T> {
    tauri::async_runtime::spawn_blocking(move || std::thread::spawn(work).join().ok())
        .await
        .ok()
        .flatten()
}

#[cfg(windows)]
fn win<T>(result: focus_sys::connect::WinResult<T>) -> CmdResult<T> {
    result.map_err(|err| AppError::Other(err.to_string()))
}

/// Connexion actuelle, pour la barre du haut.
#[tauri::command]
pub async fn net_status() -> Option<focus_sys::connect::NetStatus> {
    off_thread(focus_sys::connect::net_status).await
}

/// Radios Wi-Fi et Bluetooth.
#[tauri::command]
pub async fn radios() -> CmdResult<Vec<focus_sys::connect::RadioInfo>> {
    off_thread(|| win(focus_sys::connect::radios()))
        .await
        .unwrap_or_else(|| Ok(Vec::new()))
}

/// Allume ou éteint une radio.
#[tauri::command]
pub async fn set_radio(kind: String, on: bool) -> CmdResult<bool> {
    off_thread(move || win(focus_sys::connect::set_radio(&kind, on)))
        .await
        .unwrap_or(Ok(false))
}

/// Réseaux Wi-Fi visibles.
#[tauri::command]
pub async fn wifi_networks(scan: bool) -> CmdResult<Vec<focus_sys::connect::WifiNetwork>> {
    off_thread(move || win(focus_sys::connect::wifi_networks(scan)))
        .await
        .unwrap_or_else(|| Ok(Vec::new()))
}

/// Se connecte à un réseau Wi-Fi. Le mot de passe n'est ni journalisé ni gardé.
#[tauri::command]
pub async fn wifi_connect(ssid: String, password: Option<String>) -> CmdResult<String> {
    let result = off_thread(move || {
        win(focus_sys::connect::wifi_connect(&ssid, password.as_deref()))
    })
    .await
    .unwrap_or(Ok("failed"))?;
    Ok(result.to_owned())
}

/// Se déconnecte du Wi-Fi.
#[tauri::command]
pub async fn wifi_disconnect() -> CmdResult<()> {
    off_thread(|| win(focus_sys::connect::wifi_disconnect()))
        .await
        .unwrap_or(Ok(()))
}

/// Appareils Bluetooth appairés.
#[tauri::command]
pub async fn bt_paired() -> CmdResult<Vec<focus_sys::connect::BtDevice>> {
    off_thread(|| win(focus_sys::connect::bt_paired()))
        .await
        .unwrap_or_else(|| Ok(Vec::new()))
}

/// Appareils Bluetooth proches.
#[tauri::command]
pub async fn bt_nearby() -> CmdResult<Vec<focus_sys::connect::BtDevice>> {
    off_thread(|| win(focus_sys::connect::bt_nearby()))
        .await
        .unwrap_or_else(|| Ok(Vec::new()))
}

/// Appaire un appareil Bluetooth.
#[tauri::command]
pub async fn bt_pair(id: String) -> CmdResult<String> {
    let result = off_thread(move || win(focus_sys::connect::bt_pair(&id)))
        .await
        .unwrap_or(Ok("failed"))?;
    Ok(result.to_owned())
}

/// Oublie un appareil Bluetooth.
#[tauri::command]
pub async fn bt_unpair(id: String) -> CmdResult<String> {
    let result = off_thread(move || win(focus_sys::connect::bt_unpair(&id)))
        .await
        .unwrap_or(Ok("failed"))?;
    Ok(result.to_owned())
}

/// Applique le mode console selon les réglages : entrée, mise à jour ou sortie.
#[tauri::command]
pub fn apply_console(app: AppHandle) -> bool {
    apply_console_in_background(&app)
}

/// Un passage en mode console est en cours (il peut durer plusieurs secondes).
static CONSOLE_APPLYING: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Applique le mode console hors du fil principal : la fenêtre reste fluide pendant
/// que le plan (fermetures, services, mémoire) s'exécute. Faux si un passage est déjà
/// en cours ; il relit alors les réglages à la fin.
pub fn apply_console_in_background(app: &AppHandle) -> bool {
    use std::sync::atomic::Ordering;
    if CONSOLE_APPLYING.swap(true, Ordering::SeqCst) {
        AGAIN.store(true, Ordering::SeqCst);
        return false;
    }
    let app = app.clone();
    std::thread::spawn(move || {
        loop {
            AGAIN.store(false, Ordering::SeqCst);
            let state = app.state::<std::sync::Arc<App>>();
            apply_console_settings(&app, &state, &app.state::<Console>());
            if !AGAIN.load(Ordering::SeqCst) {
                break;
            }
        }
        CONSOLE_APPLYING.store(false, Ordering::SeqCst);
    });
    true
}

/// Un réglage a changé pendant un passage : on repasse avec les nouveaux réglages.
static AGAIN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Passage en mode console en cours ?
pub fn console_applying() -> bool {
    CONSOLE_APPLYING.load(std::sync::atomic::Ordering::SeqCst)
}

/// Met Windows dans l'état voulu par les réglages.
pub fn apply_console_settings(app: &AppHandle, state: &App, console: &Console) {
    let settings = state.settings();
    if settings.first_run_done && settings.console_mode {
        let handle = app.clone();
        // L'interface montre le passage en cours, puis son bilan.
        let _ = app.emit("console-progress", true);
        console.enter(
            Wanted {
                stop_shell: settings.stop_shell,
                hide_taskbar: settings.hide_taskbar,
                silence: settings.silence_notifications,
                keep_awake: settings.keep_awake,
                tweaks: settings.level() != crate::settings::Optimization::Off,
            },
            // Touche Windows : comme le bouton central, retour au launcher.
            move || {
                back_to_launcher(&handle);
                let _ = handle.emit("pad-action", "home");
            },
        );
        let report = state.apply_console_plan(console.shell_stopped());
        let _ = app.emit("console-report", Some(report));
    } else {
        console.leave();
        state.restore_console_plan();
        let _ = app.emit("console-report", None::<crate::runtime::ConsoleReport>);
    }
}

/// Bilan du mode console : ce qui a changé sur le PC, mesuré avant et après.
#[tauri::command]
pub fn console_report(state: Shared<'_>) -> Option<crate::runtime::ConsoleReport> {
    state.console_report()
}

/// Ferme les applications encore ouvertes. L'interface a demandé l'accord.
#[tauri::command]
pub async fn close_open_apps(app: AppHandle, state: Shared<'_>) -> CmdResult<Option<crate::runtime::ConsoleReport>> {
    let shared = state.inner().clone();
    let report = tauri::async_runtime::spawn_blocking(move || shared.close_open_apps())
        .await
        .map_err(|err| AppError::Other(err.to_string()))?;
    let _ = app.emit("console-report", report.clone());
    Ok(report)
}

/// Processus des boutiques : réduits plutôt que fermés, pour ne jamais casser un jeu.
pub const STORE_PROCESSES: &[&str] = &[
    "steam", "steamwebhelper", "epicgameslauncher", "eadesktop", "galaxyclient", "upc", "battle.net",
];

/// Une boutique a été ouverte depuis Focus et doit rester visible.
pub static STORE_OPEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// Retour au launcher (bouton central, touche Windows) : une boutique ouverte est
/// simplement réduite, jamais fermée.
pub fn back_to_launcher(app: &AppHandle) {
    if STORE_OPEN.swap(false, std::sync::atomic::Ordering::SeqCst) {
        #[cfg(windows)]
        focus_sys::console::minimize_processes(STORE_PROCESSES);
    }
    // Par-dessus un jeu : beaucoup de jeux en plein écran sans bordure se placent
    // « toujours au-dessus ». Focus prend ce rang le temps de son menu ; il le rend
    // en revenant au jeu (`hide_launcher`) ou à la fin de la session.
    let in_game = app
        .try_state::<std::sync::Arc<App>>()
        .is_some_and(|state| state.in_session.load(std::sync::atomic::Ordering::SeqCst));
    if in_game {
        if let Some(window) = app.get_webview_window("main") {
            let _ = window.set_always_on_top(true);
        }
    }
    show_launcher(app);
}

/// Capture d’écran, enregistrée dans `Images\Focus`. Renvoie le nom du fichier.
#[tauri::command]
pub async fn screenshot() -> CmdResult<String> {
    #[cfg(windows)]
    {
        tauri::async_runtime::spawn_blocking(|| {
            let image = focus_sys::console::capture_screen().ok_or_else(|| AppError::Other("capture failed".into()))?;
            let bytes = focus_library::art::png(image.width, image.height, image.pixels)
                .ok_or_else(|| AppError::Other("encode failed".into()))?;
            let base = std::env::var_os("USERPROFILE").map(PathBuf::from).unwrap_or_default();
            let dir = base.join("Pictures").join("Focus");
            std::fs::create_dir_all(&dir)?;
            let stamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_or(0, |d| d.as_secs());
            let name = format!("focus-{stamp}.png");
            std::fs::write(dir.join(&name), bytes)?;
            Ok(name)
        })
        .await
        .map_err(|err| AppError::Other(err.to_string()))?
    }
    #[cfg(not(windows))]
    Err(AppError::Other("Windows only".into()))
}

/// Un jeu attend-il une mise à jour ? Seul Steam l’indique sur le disque.
#[tauri::command(async)]
pub fn update_status(id: String) -> Option<bool> {
    let appid = id.strip_prefix("steam:")?;
    let root = discover::steam_root()?;
    focus_library::scanners::steam_update_needed(&root, appid)
}

/// Ouvre la page de téléchargements de la boutique pour mettre le jeu à jour.
#[tauri::command(async)]
pub fn open_updates(app: AppHandle, id: String) -> CmdResult<()> {
    let uri = if id.starts_with("steam:") {
        "steam://open/downloads"
    } else if id.starts_with("epic:") {
        "com.epicgames.launcher://store/library"
    } else {
        return Err(AppError::UnknownEntry(id));
    };
    STORE_OPEN.store(true, std::sync::atomic::Ordering::SeqCst);
    #[cfg(windows)]
    focus_sys::launch::start(&LaunchTarget::Protocol { uri: uri.to_owned() }).map_err(|e| AppError::Other(e.to_string()))?;
    let _ = app;
    Ok(())
}
