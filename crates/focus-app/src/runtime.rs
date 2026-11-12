//! État de l'application et déroulement d'une session.

use crate::settings::{Optimization, Settings};
use focus_core::engine::SessionContext;
use focus_core::journal::JournalStore;
use focus_core::model::{Entry, EntryKind, LaunchTarget, Store};
use focus_core::plan::{build_plan, Plan, PlanOptions, Step};
use focus_core::rules::{RuleError, RuleFile};
use focus_library::repo::Repository;
use serde::Serialize;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use thiserror::Error;

const DEFAULT_RULES: &str = include_str!("../data/optimizer-rules.default.json");
const SETTINGS_KEY: &str = "settings";
/// Temps laissé à une boutique pour démarrer le vrai jeu.
const GAME_APPEAR_TIMEOUT: Duration = Duration::from_secs(180);
const POLL: Duration = Duration::from_secs(1);

/// Erreur renvoyée à l'interface sous forme de texte anglais pour les journaux.
#[derive(Debug, Error)]
pub enum AppError {
    /// Base de données.
    #[error("database: {0}")]
    Db(#[from] rusqlite_error::Error),
    /// Règles invalides.
    #[error(transparent)]
    Rules(#[from] RuleError),
    /// Entrée inconnue.
    #[error("unknown entry {0}")]
    UnknownEntry(String),
    /// Fichier.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Une session tourne déjà.
    #[error("a session is already running")]
    Busy,
    /// Autre.
    #[error("{0}")]
    Other(String),
}

/// Alias : le type d'erreur reste caché derrière `focus-library`.
mod rusqlite_error {
    pub use focus_library::repo::RepoError as Error;
}

impl Serialize for AppError {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

/// Session en cours.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Running {
    /// Entrée.
    pub entry_id: String,
    /// Titre.
    pub title: String,
    /// Type.
    pub kind: EntryKind,
    /// Noms d'exécutables suivis, sans handle sur le jeu (SPEC D7).
    #[serde(skip)]
    pub watched: Vec<String>,
    /// La session a appliqué un plan réel.
    pub real: bool,
    /// Boutique : Steam Input prend le relais pour les jeux Steam.
    #[serde(skip)]
    pub store: Store,
    /// Pid direct, s'il y en a un.
    #[serde(skip)]
    pub pid: Option<u32>,
    /// Exécutable qui a affiché le jeu à l'écran : la session finit avec lui, même
    /// si un lanceur reste ouvert.
    #[serde(skip)]
    pub main: Option<String>,
    /// Un anticheat est installé avec le jeu : aucun arrêt forcé du jeu (SPEC D7).
    #[serde(skip)]
    pub anticheat: bool,
    /// Programmes qui tournaient déjà au lancement : leur fenêtre n'est jamais prise
    /// pour celle du jeu (un terminal ou un navigateur resté ouvert).
    #[serde(skip)]
    pub before: Vec<String>,
}

/// Fin de session, envoyée à l'interface.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionEnd {
    /// Entrée.
    pub entry_id: String,
    /// Titre.
    pub title: String,
    /// Durée en secondes.
    pub seconds: u64,
    /// Restauration complète.
    pub restored: bool,
    /// Problèmes rencontrés.
    pub failures: Vec<String>,
}

/// Ce que le mode console a changé, mesuré avant et après : c'est ce que l'interface
/// montre pour que le changement de profil se voie.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConsoleReport {
    /// Niveau appliqué.
    pub level: Optimization,
    /// Bureau de Windows arrêté.
    pub shell_stopped: bool,
    /// Mode d'alimentation avant (identifiant Windows), s'il a changé.
    pub power_before: Option<String>,
    /// Mode d'alimentation maintenant.
    pub power_after: Option<String>,
    /// Mémoire vive totale, en octets.
    pub memory_total: u64,
    /// Mémoire disponible avant.
    pub memory_before: u64,
    /// Mémoire disponible après.
    pub memory_after: u64,
    /// Programmes d'arrière-plan ralentis.
    pub slowed: usize,
    /// Programmes fermés.
    pub closed: usize,
    /// Services Windows mis en pause.
    pub paused: usize,
    /// Programmes dont la mémoire a été rendue.
    pub trimmed: usize,
    /// Étapes qui n'ont pas pu se faire.
    pub failed: usize,
    /// Applications encore ouvertes : fermées seulement si l'utilisateur le demande.
    pub apps: Vec<String>,
    /// Durée du passage, en millisecondes.
    pub millis: u64,
}

/// Installation lancée depuis Focus, montrée sur l'écran Téléchargements.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InstallJob {
    /// Identifiant du catalogue.
    pub id: String,
    /// Nom affiché.
    pub name: String,
    /// `running`, `done` ou `failed`.
    pub state: &'static str,
}

/// Entrée fictive qui représente Focus lui-même pour le plan du mode console.
const CONSOLE_ENTRY: &str = "focus:console";

fn console_entry() -> Entry {
    Entry {
        id: CONSOLE_ENTRY.to_owned(),
        title: "Focus".to_owned(),
        kind: EntryKind::Game,
        store: Store::Manual,
        target: LaunchTarget::Protocol { uri: String::new() },
        install_dir: None,
        cover: None,
        favorite: false,
        stats: focus_core::model::Stats::default(),
    }
}

/// Noms des applications ouvertes, une ligne par programme.
/// Le nom lisible du programme (« Google Chrome ») plutôt que celui de son fichier.
fn app_names(apps: &[focus_core::model::ProcessInfo]) -> Vec<String> {
    let readable = |process: &focus_core::model::ProcessInfo| -> Option<String> {
        #[cfg(windows)]
        return process.path.as_deref().and_then(focus_sys::scan::display_name);
        #[cfg(not(windows))]
        {
            let _ = process;
            None
        }
    };
    let mut names: Vec<String> = apps
        .iter()
        .map(|process| readable(process).unwrap_or_else(|| process.name.clone()))
        .collect();
    names.sort();
    names.dedup();
    names
}

/// État partagé.
pub struct App {
    repo: Mutex<Repository>,
    settings: Mutex<Settings>,
    running: Mutex<Option<Running>>,
    /// Actif pendant une session : le rendu et la manette passent en retrait.
    pub in_session: AtomicBool,
    /// Souris à la manette : le stick déplace le curseur, les boutons cliquent.
    pub pointer: AtomicBool,
    data_dir: PathBuf,
    journal: JournalStore,
    /// Plan appliqué pour toute la durée du mode console (tout ce qui est inutile au jeu).
    #[cfg(windows)]
    console_plan: Mutex<Option<focus_sys::ops::WinOps>>,
    /// Dernier bilan du mode console, pour l'interface.
    report: Mutex<Option<ConsoleReport>>,
    /// Installations lancées depuis Focus depuis son ouverture.
    installs: Mutex<Vec<InstallJob>>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    // Un fil mort en tenant le verrou ne doit jamais bloquer la restauration.
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}

impl App {
    /// Ouvre les données de l'application.
    pub fn open(data_dir: &Path) -> Result<Self, AppError> {
        let repo = Repository::open(&data_dir.join("library.db"))?;
        let settings = repo
            .setting(SETTINGS_KEY)?
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default();
        Ok(Self {
            repo: Mutex::new(repo),
            settings: Mutex::new(settings),
            running: Mutex::new(None),
            in_session: AtomicBool::new(false),
            pointer: AtomicBool::new(false),
            data_dir: data_dir.to_path_buf(),
            journal: JournalStore::new(data_dir.join("session.lock.json")),
            #[cfg(windows)]
            console_plan: Mutex::new(None),
            report: Mutex::new(None),
            installs: Mutex::new(Vec::new()),
        })
    }

    /// Dossier des données de l'application.
    pub fn data_dir(&self) -> &Path {
        &self.data_dir
    }

    /// Accès à la base.
    pub fn repo(&self) -> MutexGuard<'_, Repository> {
        lock(&self.repo)
    }

    /// Réglages courants.
    pub fn settings(&self) -> Settings {
        lock(&self.settings).clone()
    }

    /// Enregistre les réglages.
    pub fn save_settings(&self, settings: Settings) -> Result<(), AppError> {
        let json = serde_json::to_string(&settings).map_err(|e| AppError::Other(e.to_string()))?;
        self.repo().set_setting(SETTINGS_KEY, &json)?;
        *lock(&self.settings) = settings;
        Ok(())
    }

    /// Session en cours.
    pub fn running(&self) -> Option<Running> {
        lock(&self.running).clone()
    }

    /// Retient l'exécutable du jeu une fois sa fenêtre à l'écran.
    pub fn set_main(&self, name: String) {
        if let Some(running) = lock(&self.running).as_mut() {
            tracing::info!("game window shown by {name}");
            running.main = Some(name);
        }
    }

    /// Règles de l'utilisateur, ou celles par défaut à la première utilisation.
    pub fn rules(&self) -> Result<RuleFile, AppError> {
        let path = self.data_dir.join("optimizer-rules.json");
        match std::fs::read_to_string(&path) {
            Ok(text) => {
                let mut file = RuleFile::parse(&text)?;
                // Un fichier d’une version précédente reçoit les profils ajoutés depuis ;
                // les règles déjà présentes ne sont jamais modifiées.
                let defaults = RuleFile::parse(DEFAULT_RULES)?;
                let missing: Vec<_> = defaults
                    .profiles
                    .into_iter()
                    .filter(|profile| file.profiles.iter().all(|own| own.name != profile.name))
                    .collect();
                if !missing.is_empty() {
                    file.profiles.extend(missing);
                    if let Ok(json) = serde_json::to_string_pretty(&file) {
                        std::fs::write(&path, json)?;
                    }
                }
                Ok(file)
            }
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                std::fs::write(&path, DEFAULT_RULES)?;
                Ok(RuleFile::parse(DEFAULT_RULES)?)
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Masque la manette PlayStation au jeu si Focus Input va la remplacer (HidHide
    /// et pilote de manette virtuelle présents, jeu hors Steam, choix « Auto » ou
    /// « Focus Input »). Le journal est écrit avant le changement (règle 1).
    #[cfg(windows)]
    fn pads_hide(&self, entry: &Entry) {
        use focus_input::modes::{resolve_mode, Environment, Mode};
        use focus_input::pad::Family;
        if entry.store == Store::Steam || !focus_sys::hidhide::present() {
            return;
        }
        let env = Environment {
            steam_running: false,
            vigem_present: focus_input::source::vigem_present(),
            hidhide_present: true,
        };
        let family = focus_input::source::detect_family();
        if family != Family::PlayStation || resolve_mode(family, env, self.settings().controller_mode) != Mode::FocusInput {
            return;
        }
        let Some(plan) = focus_sys::hidhide::plan() else { return };
        let Ok(text) = serde_json::to_string(&plan) else { return };
        if let Err(err) = std::fs::write(self.data_dir.join(PADS_JOURNAL), text) {
            tracing::error!("pad journal not written, nothing hidden: {err}");
            return;
        }
        let Ok(exe) = std::env::current_exe() else { return };
        if focus_sys::hidhide::hide(&plan, &exe) {
            tracing::info!("playstation pad hidden from the game: {:?}", plan.paths);
        }
    }

    /// Termine une session interrompue. S'exécute à chaque démarrage, avant tout.
    pub fn recover_previous_session(&self) {
        pads_restore(&self.data_dir);
        #[cfg(windows)]
        {
            let services = crate::guardian_client::GuardianClient::connect()
                .map(|client| Box::new(client) as Box<dyn focus_sys::ops::ServiceControl>);
            let mut ops = focus_sys::ops::WinOps::new(services);
            match focus_core::session::recover(&mut ops, &self.journal) {
                Ok(report) if report.failures.is_empty() => {}
                Ok(report) => tracing::error!("recovery incomplete: {:?}", report.failures),
                Err(err) => tracing::error!("recovery failed: {err}"),
            }
        }
    }

    /// Mode console : arrête ou ralentit tout ce qui ne sert pas au jeu, pour toute la
    /// durée où Focus est ouvert. Journalisé avant chaque coupure, restauré à la
    /// sortie, par le gardien si Focus disparaît, ou au démarrage suivant.
    /// Rien ne se passe sans l’accord donné au premier démarrage (règle 8).
    pub fn apply_console_plan(&self, shell_stopped: bool) -> ConsoleReport {
        let level = self.settings().level();
        let mut summary = ConsoleReport {
            level,
            shell_stopped,
            ..ConsoleReport::default()
        };
        #[cfg(windows)]
        {
            use focus_sys::ops::{ServiceControl, WinOps};
            // Le verrou est tenu pendant tout le passage : un jeu lancé à ce moment
            // attend la fin au lieu d'appliquer un second plan par-dessus.
            let mut slot = lock(&self.console_plan);
            if slot.is_some() {
                let mut report = lock(&self.report);
                if let Some(known) = report.as_mut().filter(|known| known.level == level) {
                    known.shell_stopped = shell_stopped;
                    return known.clone();
                }
                drop(report);
                // Le niveau a changé : on rend ce que l'ancien profil avait coupé, puis
                // le nouveau s'applique. C'est ce qui rend le changement réel, tout de suite.
                if let Some(mut ops) = slot.take() {
                    if let Err(err) = focus_core::session::restore(&mut ops, &self.journal) {
                        tracing::error!("previous profile not restored: {err}");
                    }
                }
            }
            let (total, before) = focus_sys::console::memory();
            (summary.memory_total, summary.memory_before, summary.memory_after) = (total, before, before);
            if slot.is_none() && level != Optimization::Off {
                let started = Instant::now();
                let pseudo = console_entry();
                if let Ok(mut plan) = self.plan_for(&pseudo) {
                    let guardian = crate::guardian_client::GuardianClient::connect()
                        .map(|client| Box::new(client) as Box<dyn ServiceControl>);
                    if guardian.is_none() {
                        plan.steps.retain(|step| !matches!(step, Step::StopService { .. }));
                    }
                    let mut ops = WinOps::new(guardian);
                    match focus_core::session::apply_plan(&plan, &pseudo.id, now(), &mut ops, &self.journal) {
                        Ok(report) => {
                            if level == Optimization::Max {
                                summary.trimmed = focus_sys::console::trim_background(|name| {
                                    focus_core::whitelist::is_protected_process(name)
                                        || focus_core::whitelist::store_of_process(name).is_some()
                                });
                            }
                            summary.failed = report.failures.len();
                            if !report.failures.is_empty() {
                                tracing::warn!("console steps failed: {:?}", report.failures);
                            }
                            summary.apps = app_names(&plan.apps);
                            self.count_changes(&ops, &mut summary);
                            summary.millis = started.elapsed().as_millis() as u64;
                            tracing::info!("console plan applied: {summary:?}");
                            *slot = Some(ops);
                        }
                        Err(err) => tracing::error!("console plan not applied: {err}"),
                    }
                }
            }
            drop(slot);
        }
        *lock(&self.report) = Some(summary.clone());
        summary
    }

    /// Compte ce qui a vraiment été fait, d'après le journal, et relit la mémoire et
    /// le mode d'alimentation.
    #[cfg(windows)]
    fn count_changes(&self, ops: &focus_sys::ops::WinOps, summary: &mut ConsoleReport) {
        use focus_core::journal::{Original, StepState};
        use focus_core::session::SystemOps;
        (summary.slowed, summary.closed, summary.paused) = (0, 0, 0);
        if let Ok(Some(journal)) = self.journal.read() {
            for entry in journal.entries.iter().filter(|entry| entry.state == StepState::Applied) {
                match (&entry.step, &entry.original) {
                    (Step::Throttle { .. }, _) => summary.slowed += 1,
                    (Step::Close { .. }, _) => summary.closed += 1,
                    (Step::StopService { .. }, _) => summary.paused += 1,
                    (Step::HighPerformancePower, Original::PowerScheme { guid }) => {
                        summary.power_before = Some(guid.clone());
                    }
                    _ => {}
                }
            }
        }
        summary.power_after = ops.active_power_scheme().ok();
        // La mémoire rendue met un instant à apparaître comme disponible.
        std::thread::sleep(Duration::from_millis(400));
        summary.memory_after = focus_sys::console::memory().1;
    }

    /// Note le début ou la fin d'une installation lancée depuis Focus.
    pub fn track_install(&self, id: &str, name: &str, state: &'static str) {
        let mut installs = lock(&self.installs);
        installs.retain(|job| job.id != id);
        installs.push(InstallJob {
            id: id.to_owned(),
            name: name.to_owned(),
            state,
        });
    }

    /// Installations lancées depuis Focus depuis son ouverture.
    pub fn installs(&self) -> Vec<InstallJob> {
        lock(&self.installs).clone()
    }

    /// Dernier bilan du mode console.
    pub fn console_report(&self) -> Option<ConsoleReport> {
        lock(&self.report).clone()
    }

    /// Ferme les applications encore ouvertes, après l'accord de l'utilisateur. Chaque
    /// fermeture est notée au journal avant d'être demandée ; aucune n'est forcée :
    /// une application qui propose d'enregistrer reste ouverte.
    pub fn close_open_apps(&self) -> Option<ConsoleReport> {
        #[cfg(windows)]
        {
            let mut slot = lock(&self.console_plan);
            let ops = slot.as_mut()?;
            let apps = self.plan_for(&console_entry()).ok()?.apps;
            let steps = focus_core::plan::close_steps(&apps);
            match focus_core::session::extend_plan(&steps, CONSOLE_ENTRY, now(), ops, &self.journal) {
                Ok(report) if !report.failures.is_empty() => {
                    tracing::warn!("some applications stayed open: {:?}", report.failures);
                }
                Ok(_) => {}
                Err(err) => tracing::error!("applications not closed: {err}"),
            }
            let mut summary = self.console_report().unwrap_or_default();
            summary.apps = self.plan_for(&console_entry()).map(|plan| app_names(&plan.apps)).unwrap_or_default();
            self.count_changes(ops, &mut summary);
            drop(slot);
            *lock(&self.report) = Some(summary.clone());
            Some(summary)
        }
        #[cfg(not(windows))]
        None
    }

    /// Remet en place tout ce que le mode console avait coupé.
    pub fn restore_console_plan(&self) {
        *lock(&self.report) = None;
        #[cfg(windows)]
        if let Some(mut ops) = lock(&self.console_plan).take() {
            match focus_core::session::restore(&mut ops, &self.journal) {
                Ok(report) if report.failures.is_empty() => tracing::info!("console plan restored"),
                Ok(report) => tracing::error!("console restore incomplete: {:?}", report.failures),
                Err(err) => tracing::error!("console restore failed: {err}"),
            }
        }
    }

    fn console_plan_active(&self) -> bool {
        #[cfg(windows)]
        {
            lock(&self.console_plan).is_some()
        }
        #[cfg(not(windows))]
        false
    }

    /// Dossiers d'installation de la bibliothèque, en minuscules avec leur barre finale.
    /// Une racine de disque n'en fait pas partie : elle couvrirait tout le PC.
    fn game_dirs(&self) -> Vec<String> {
        self.repo()
            .all()
            .unwrap_or_default()
            .into_iter()
            .filter_map(|entry| entry.install_dir)
            .map(|dir| dir.replace('/', "\\").trim_end_matches('\\').to_lowercase() + "\\")
            .filter(|dir| dir.len() > 4)
            .collect()
    }

    /// Construit le plan d'une entrée : c'est le mode simulation.
    pub fn plan_for(&self, entry: &Entry) -> Result<Plan, AppError> {
        let file = self.rules()?;
        let settings = self.settings();
        let level = settings.level();
        // « Maximal » applique tous les leviers autorisés ; à défaut, le premier profil.
        let wanted = if level == Optimization::Max { "Maximal" } else { "Concentration" };
        let profile = file
            .profiles
            .iter()
            .find(|profile| profile.name == wanted)
            .or_else(|| file.profiles.first())
            .ok_or_else(|| AppError::Other("no profile in rules".to_owned()))?;
        let ctx = SessionContext {
            kind: entry.kind,
            store: entry.store,
            steam_input_active: settings.controller_mode
                == Some(focus_input::modes::Mode::SteamInput),
            keep_social: settings.keep_social,
        };
        let optimize = matches!(entry.kind, EntryKind::Game | EntryKind::Software);
        let options = PlanOptions {
            keep_awake: optimize && (settings.keep_awake || level == Optimization::Max),
            high_performance: optimize
                && (settings.high_performance || level == Optimization::Max),
            // « Maximal » : tout ce qui n'est pas nécessaire au jeu est ralenti, et les
            // applications ouvertes sont proposées à la fermeture.
            sweep: level == Optimization::Max && entry.kind == EntryKind::Game,
        };
        #[cfg(windows)]
        let mut snapshot = focus_sys::scan::snapshot();
        #[cfg(not(windows))]
        let mut snapshot = focus_core::model::Snapshot::default();
        if options.sweep {
            // Aucun handle sur un jeu de la bibliothèque ni sur ce qui l'accompagne
            // (anticheat, lanceur) : ils sortent du plan avant tout (SPEC D7).
            let games = self.game_dirs();
            snapshot.processes.retain(|process| {
                !process.path.as_ref().is_some_and(|path| {
                    let path = path.to_lowercase();
                    games.iter().any(|dir| path.starts_with(dir))
                })
            });
        }
        let mut plan = build_plan(&snapshot, &file, profile, &ctx, options);
        plan.simulation_only = level == Optimization::Off;
        Ok(plan)
    }

    /// Prépare, optimise et lance. La surveillance continue dans un fil dédié.
    pub fn play(
        self: &std::sync::Arc<Self>,
        id: &str,
        on_end: impl FnOnce(SessionEnd) + Send + 'static,
    ) -> Result<(), AppError> {
        let entry = self
            .repo()
            .get(id)?
            .ok_or_else(|| AppError::UnknownEntry(id.to_owned()))?;
        {
            let mut running = lock(&self.running);
            if running.is_some() {
                return Err(AppError::Busy);
            }
            *running = Some(Running {
                entry_id: entry.id.clone(),
                title: entry.title.clone(),
                kind: entry.kind,
                watched: entry
                    .install_dir
                    .as_deref()
                    .map(game_exe_names)
                    .unwrap_or_default(),
                real: false,
                store: entry.store,
                pid: None,
                main: None,
                anticheat: entry.install_dir.as_deref().is_some_and(anticheat_in),
                before: running_names(),
            });
        }
        let result = self.start_session(&entry);
        match result {
            Ok(mut ops) => {
                self.in_session.store(true, Ordering::SeqCst);
                #[cfg(windows)]
                {
                    focus_sys::console::set_own_priority(true);
                    focus_sys::console::trim_self();
                }
                let app = self.clone();
                std::thread::spawn(move || {
                    let end = app.watch(&entry, &mut ops);
                    on_end(end);
                });
                Ok(())
            }
            Err(err) => {
                *lock(&self.running) = None;
                Err(err)
            }
        }
    }

    #[cfg(windows)]
    fn start_session(&self, entry: &Entry) -> Result<focus_sys::ops::WinOps, AppError> {
        use focus_core::session::apply_plan;
        use focus_sys::ops::{ServiceControl, WinOps};

        let mut plan = self.plan_for(entry)?;
        // En mode console, tout est déjà coupé : la session ne recoupe rien.
        let console_active = self.console_plan_active();
        let real = !plan.simulation_only && !console_active;
        let guardian = real
            .then(crate::guardian_client::GuardianClient::connect)
            .flatten()
            .map(|client| Box::new(client) as Box<dyn ServiceControl>);
        // Sans gardien, les arrêts de services sont simplement retirés du plan.
        if guardian.is_none() {
            plan.steps
                .retain(|step| !matches!(step, Step::StopService { .. }));
        }
        let mut ops = WinOps::new(guardian);
        if real {
            let report = apply_plan(&plan, &entry.id, now(), &mut ops, &self.journal)
                .map_err(|e| AppError::Other(e.to_string()))?;
            if !report.failures.is_empty() {
                tracing::warn!("session steps failed: {:?}", report.failures);
            }
        }
        // Manette PlayStation hors Steam : la vraie est masquée au jeu avant qu'il
        // démarre, la manette Xbox virtuelle de Focus Input la remplace.
        self.pads_hide(entry);
        match focus_sys::launch::start(&entry.target) {
            Ok(child) => {
                let pid = child.as_ref().map(std::process::Child::id);
                if let (Some(pid), true) = (pid, (real || console_active) && may_boost(entry)) {
                    if let Err(err) = focus_sys::ops::boost(pid) {
                        tracing::warn!("priority not raised: {err}");
                    }
                }
                if let Some(running) = lock(&self.running).as_mut() {
                    running.real = real;
                    running.pid = pid;
                    let fill = matches!(entry.kind, EntryKind::Software | EntryKind::Social) || may_boost(entry);
                    if fill {
                        fill_when_ready(pid, running.watched.clone());
                    }
                }
                Ok(ops)
            }
            Err(err) => {
                // Le lancement a échoué : on ne laisse rien de coupé.
                if real {
                    let _ = focus_core::session::restore(&mut ops, &self.journal);
                }
                Err(AppError::Other(err.to_string()))
            }
        }
    }

    #[cfg(not(windows))]
    fn start_session(&self, _entry: &Entry) -> Result<(), AppError> {
        Err(AppError::Other("Windows only".to_owned()))
    }

    #[cfg(windows)]
    fn watch(&self, entry: &Entry, ops: &mut focus_sys::ops::WinOps) -> SessionEnd {
        let started = Instant::now();
        wait_for_exit(self);
        pads_restore(&self.data_dir);
        // Le lanceur resté ouvert peut se remontrer à la fin du jeu : réduit, il ne
        // passe pas devant l'écran de fin.
        if let Some(running) = self.running() {
            let launchers: Vec<&str> =
                running.watched.iter().map(String::as_str).filter(|name| is_launcher(name)).collect();
            if !launchers.is_empty() {
                std::thread::sleep(Duration::from_millis(500));
                focus_sys::console::minimize_processes(&launchers);
            }
        }
        let real = self.running().is_some_and(|r| r.real);
        let (restored, failures) = if real {
            match focus_core::session::restore(ops, &self.journal) {
                Ok(report) => (report.failures.is_empty(), report.failures),
                Err(err) => (false, vec![err.to_string()]),
            }
        } else {
            (true, Vec::new())
        };
        let seconds = started.elapsed().as_secs();
        if let Err(err) = self.repo().record_session(&entry.id, seconds, now()) {
            tracing::error!("session not recorded: {err}");
        }
        *lock(&self.running) = None;
        self.in_session.store(false, Ordering::SeqCst);
        focus_sys::console::set_own_priority(false);
        SessionEnd {
            entry_id: entry.id.clone(),
            title: entry.title.clone(),
            seconds,
            restored,
            failures,
        }
    }

    #[cfg(not(windows))]
    fn watch(&self, _entry: &Entry, _ops: &mut ()) -> SessionEnd {
        unreachable!("start_session always fails outside Windows")
    }
}

/// Met l’application lancée en plein écran sans bordure dès que sa fenêtre apparaît.
/// Seule la fenêtre est touchée ; un jeu de boutique ou avec anticheat n’est jamais visé.
#[cfg(windows)]
fn fill_when_ready(pid: Option<u32>, watched: Vec<String>) {
    std::thread::spawn(move || {
        for _ in 0..40 {
            std::thread::sleep(Duration::from_millis(1500));
            let mut pids: Vec<u32> = focus_sys::scan::processes()
                .into_iter()
                .filter(|process| watched.contains(&process.name))
                .map(|process| process.pid)
                .collect();
            pids.extend(pid);
            if !pids.is_empty() && focus_sys::console::fill_screen(&pids) {
                return;
            }
        }
    });
}

/// La priorité n'est relevée que pour un jeu lancé directement, sans anticheat (SPEC D7).
#[cfg(windows)]
fn may_boost(entry: &Entry) -> bool {
    let local = matches!(entry.target, LaunchTarget::Executable { .. })
        && matches!(entry.store, Store::Manual | Store::Gog);
    local
        && entry
            .install_dir
            .as_deref()
            .is_some_and(|dir| !focus_sys::launch::has_anticheat(Path::new(dir)))
}

#[cfg(windows)]
fn wait_for_exit(state: &App) {
    /// Un jeu peut passer d'un processus à l'autre (amorce, anticheat, jeu).
    const GRACE: Duration = Duration::from_secs(3);
    let start = Instant::now();
    let mut seen = false;
    let mut gone_since: Option<Instant> = None;
    loop {
        let Some(running) = state.running() else { return };
        // Noms seulement : aucun processus du jeu n'est ouvert (SPEC D7).
        let names = focus_sys::scan::process_names();
        let alive = match &running.main {
            // Le jeu lui-même : un lanceur resté ouvert ne prolonge pas la session.
            Some(main) => names.contains(main),
            None => {
                running.pid.is_some_and(|pid| focus_sys::scan::process_start_time(pid).is_some())
                    || running.watched.iter().any(|name| names.contains(name))
            }
        };
        if alive {
            seen = true;
            gone_since = None;
        } else if seen {
            if gone_since.get_or_insert_with(Instant::now).elapsed() >= GRACE {
                return;
            }
        } else if !(running.watched.is_empty() && running.pid.is_none())
            && start.elapsed() > GAME_APPEAR_TIMEOUT
        {
            // Sans rien à suivre, la session dure jusqu'au retour manuel.
            return;
        }
        std::thread::sleep(POLL);
    }
}

/// Journal de ce que Focus a masqué dans HidHide.
const PADS_JOURNAL: &str = "hidhide.json";

/// Rend la manette masquée (fin de session, démarrage, sentinelle). Sans journal, rien.
pub fn pads_restore(data: &Path) {
    let file = data.join(PADS_JOURNAL);
    let Ok(text) = std::fs::read_to_string(&file) else { return };
    #[cfg(windows)]
    if let Ok(hidden) = serde_json::from_str::<focus_sys::hidhide::Hidden>(&text) {
        if !focus_sys::hidhide::restore(&hidden) {
            // Le journal reste : nouvel essai au prochain démarrage.
            tracing::error!("playstation pad not restored yet");
            return;
        }
        tracing::info!("playstation pad visible again");
    }
    #[cfg(not(windows))]
    let _ = text;
    let _ = std::fs::remove_file(file);
}

/// Exécutable de lanceur (`launcher.exe`, `launcher_main.exe`, `FortniteLauncher`…) :
/// sa fenêtre n'est jamais prise pour celle du jeu, même grande ou transparente.
pub fn is_launcher(name: &str) -> bool {
    name.to_lowercase().contains("launcher")
}

#[cfg(windows)]
fn anticheat_in(dir: &str) -> bool {
    focus_sys::launch::has_anticheat(Path::new(dir))
}

#[cfg(not(windows))]
fn anticheat_in(_dir: &str) -> bool {
    false
}

/// Exécutables qui accompagnent un jeu sans être le jeu : ils servent une boutique,
/// un navigateur intégré ou une installation.
const HELPER_HINTS: &[&str] = &[
    "webhelper", "webview", "crashreport", "crashhandler", "createdump", "uninst", "unins0",
    "setup", "install", "updater", "redist", "prereq", "notification_helper", "tracing_service",
    "cefsubprocess", "hpatchz",
];

/// Exécutables suivis pendant une session. Sont écartés les assistants connus et
/// tout ce qui tournait déjà avant le lancement (le `EpicWebHelper` d'Epic, le
/// `msedgewebview2` de Windows…), qui sinon garderaient la session ouverte.
fn game_exe_names(dir: &str) -> Vec<String> {
    let already = running_names();
    let mut names = exe_names(dir);
    names.retain(|name| !HELPER_HINTS.iter().any(|hint| name.contains(hint)) && !already.contains(name));
    names.sort();
    names.dedup();
    names
}

#[cfg(windows)]
fn running_names() -> Vec<String> {
    focus_sys::scan::process_names()
}

#[cfg(not(windows))]
fn running_names() -> Vec<String> {
    Vec::new()
}

/// Noms des exécutables d'un dossier, pour suivre le jeu sans ouvrir son processus.
fn exe_names(dir: &str) -> Vec<String> {
    fn walk(dir: &Path, depth: usize, out: &mut Vec<String>) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if depth > 0 {
                    walk(&path, depth - 1, out);
                }
            } else if path
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
            {
                if let Some(name) = path.file_name() {
                    out.push(focus_core::whitelist::normalize(&name.to_string_lossy()));
                }
            }
        }
    }
    let mut names = Vec::new();
    // Les jeux rangent souvent leur exécutable loin du lanceur
    // (`Jeu/Client/Binaries/Win64/…`).
    walk(Path::new(dir), 6, &mut names);
    names
}
