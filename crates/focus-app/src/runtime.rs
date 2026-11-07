//! État de l'application et déroulement d'une session.

use crate::settings::Settings;
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
const POLL: Duration = Duration::from_secs(2);

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
    /// Pid direct, s'il y en a un.
    #[serde(skip)]
    pub pid: Option<u32>,
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

/// État partagé.
pub struct App {
    repo: Mutex<Repository>,
    settings: Mutex<Settings>,
    running: Mutex<Option<Running>>,
    /// Actif pendant une session : le rendu et la manette passent en retrait.
    pub in_session: AtomicBool,
    data_dir: PathBuf,
    journal: JournalStore,
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
            data_dir: data_dir.to_path_buf(),
            journal: JournalStore::new(data_dir.join("session.lock.json")),
        })
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

    /// Règles de l'utilisateur, ou celles par défaut à la première utilisation.
    pub fn rules(&self) -> Result<RuleFile, AppError> {
        let path = self.data_dir.join("optimizer-rules.json");
        match std::fs::read_to_string(&path) {
            Ok(text) => Ok(RuleFile::parse(&text)?),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
                std::fs::write(&path, DEFAULT_RULES)?;
                Ok(RuleFile::parse(DEFAULT_RULES)?)
            }
            Err(err) => Err(err.into()),
        }
    }

    /// Termine une session interrompue. S'exécute à chaque démarrage, avant tout.
    pub fn recover_previous_session(&self) {
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

    /// Construit le plan d'une entrée : c'est le mode simulation.
    pub fn plan_for(&self, entry: &Entry) -> Result<Plan, AppError> {
        let file = self.rules()?;
        let settings = self.settings();
        let profile = file
            .profiles
            .first()
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
            keep_awake: optimize && settings.keep_awake,
            high_performance: optimize && settings.high_performance,
        };
        #[cfg(windows)]
        let snapshot = focus_sys::scan::snapshot();
        #[cfg(not(windows))]
        let snapshot = focus_core::model::Snapshot::default();
        let mut plan = build_plan(&snapshot, &file, profile, &ctx, options);
        plan.simulation_only = !settings.real_optimization;
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
                    .map(exe_names)
                    .unwrap_or_default(),
                real: false,
                pid: None,
            });
        }
        let result = self.start_session(&entry);
        match result {
            Ok(mut ops) => {
                self.in_session.store(true, Ordering::SeqCst);
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
        let real = !plan.simulation_only;
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
        match focus_sys::launch::start(&entry.target) {
            Ok(child) => {
                let pid = child.as_ref().map(std::process::Child::id);
                if let (Some(pid), true) = (pid, real && may_boost(entry)) {
                    if let Err(err) = focus_sys::ops::boost(pid) {
                        tracing::warn!("priority not raised: {err}");
                    }
                }
                if let Some(running) = lock(&self.running).as_mut() {
                    running.real = real;
                    running.pid = pid;
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
        let running = self.running();
        wait_for_exit(running.as_ref());
        let real = running.as_ref().is_some_and(|r| r.real);
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
fn wait_for_exit(running: Option<&Running>) {
    let Some(running) = running else { return };
    let alive = |running: &Running| {
        let pid_alive = running
            .pid
            .is_some_and(|pid| focus_sys::scan::process_start_time(pid).is_some());
        pid_alive || focus_sys::scan::any_running(&running.watched)
    };
    let start = Instant::now();
    while !alive(running) {
        if running.watched.is_empty() && running.pid.is_none() {
            // Rien à surveiller : la session dure jusqu'au retour manuel.
            std::thread::sleep(POLL);
            continue;
        }
        if start.elapsed() > GAME_APPEAR_TIMEOUT {
            return;
        }
        std::thread::sleep(POLL);
    }
    while alive(running) {
        std::thread::sleep(POLL);
    }
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
    walk(Path::new(dir), 3, &mut names);
    names
}
