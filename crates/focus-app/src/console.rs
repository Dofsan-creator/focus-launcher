//! Mode console : tant que le launcher est ouvert, Windows change d'état. La barre
//! des tâches disparaît, les notifications se taisent, le PC reste éveillé.
//!
//! Chaque changement est noté sur le disque **avant** d'être fait (règle 1). Il est
//! défait à la fermeture ; si le launcher plante, une sentinelle le défait ; à
//! défaut, le prochain démarrage le défait. Ni registre, ni remplacement de
//! l'Explorateur (v4.3, section 2).

use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};
use std::sync::{Mutex, MutexGuard, PoisonError};
use std::time::{SystemTime, UNIX_EPOCH};

const STATE_FILE: &str = "console-state.json";
/// La session de concentration finit seule après ce délai, même sans nous.
#[cfg(windows)]
const FOCUS_MINUTES: i64 = 6 * 60;

/// Ce qui a été changé, tel que noté dans le journal.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ConsoleState {
    /// Barres des tâches masquées.
    pub taskbar_hidden: bool,
    /// Session de concentration lancée par nous.
    pub focus_session: bool,
    /// Début, en secondes Unix.
    pub since: i64,
    /// Réglages du compte changés : chemin, nom, valeur d’origine (`None` : absente).
    pub registry: Vec<(String, String, Option<u32>)>,
    /// Animations de Windows coupées ; vaut l’état d’origine.
    pub animations: Option<bool>,
    /// Bureau de Windows (Explorateur) arrêté par nous : à relancer.
    pub shell_stopped: bool,
}

/// Réglages du compte (HKCU) changés en mode console, avec l’accord de l’utilisateur
/// du 2026-11-08 : enregistrement de jeu, capture, transparence, mode Jeu.
const USER_TWEAKS: [(&str, &str, u32); 4] = [
    ("System\\GameConfigStore", "GameDVR_Enabled", 0),
    ("Software\\Microsoft\\Windows\\CurrentVersion\\GameDVR", "AppCaptureEnabled", 0),
    ("Software\\Microsoft\\Windows\\CurrentVersion\\Themes\\Personalize", "EnableTransparency", 0),
    ("Software\\Microsoft\\GameBar", "AutoGameModeEnabled", 1),
];

/// Ce que l'utilisateur a choisi pour le mode console.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Wanted {
    /// Arrêter le bureau de Windows tant que Focus est ouvert.
    pub stop_shell: bool,
    /// Masquer la barre des tâches.
    pub hide_taskbar: bool,
    /// Mettre les notifications en sourdine.
    pub silence: bool,
    /// Garder le PC éveillé.
    pub keep_awake: bool,
    /// Réglages de performance du compte et effets visuels (niveau d’optimisation choisi).
    pub tweaks: bool,
}

fn state_file(data: &Path) -> PathBuf {
    data.join(STATE_FILE)
}

fn read(data: &Path) -> Option<ConsoleState> {
    let text = std::fs::read_to_string(state_file(data)).ok()?;
    serde_json::from_str(&text).ok()
}

fn write(data: &Path, state: &ConsoleState) -> std::io::Result<()> {
    let text = serde_json::to_string(state).map_err(std::io::Error::other)?;
    std::fs::write(state_file(data), text)
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

/// Défait ce qu'un journal décrit. Sert à la fermeture, à la sentinelle et au démarrage.
pub fn undo(state: &ConsoleState) {
    #[cfg(windows)]
    {
        if state.taskbar_hidden {
            focus_sys::console::set_taskbars_visible(true);
        }
        if state.focus_session {
            focus_sys::console::end_focus_session();
        }
        for (path, name, original) in &state.registry {
            focus_sys::console::set_user_dword(path, name, *original);
        }
        if !state.registry.is_empty() {
            focus_sys::console::broadcast_settings();
        }
        if let Some(enabled) = state.animations {
            focus_sys::console::set_animations(enabled);
        }
        // En dernier : le bureau repart avec les réglages déjà rendus.
        if state.shell_stopped && !focus_sys::console::start_shell() {
            tracing::error!("windows desktop not restarted");
        }
    }
    #[cfg(not(windows))]
    let _ = state;
}


/// Au démarrage : un journal resté sur le disque vient d'un plantage. On défait.
pub fn recover(data: &Path) {
    if let Some(state) = read(data) {
        undo(&state);
        tracing::warn!("console mode undone after an unexpected exit");
    }
    let _ = std::fs::remove_file(state_file(data));
}

/// Mode console en cours.
pub struct Console {
    data: PathBuf,
    active: Mutex<Option<ConsoleState>>,
    #[cfg(windows)]
    awake: Mutex<Option<focus_sys::console::KeepAwake>>,
    /// Fenêtres réduites à l’entrée, rendues à la sortie.
    #[cfg(windows)]
    minimized: Mutex<Vec<isize>>,
    /// Touche Windows détournée vers le launcher.
    #[cfg(windows)]
    win_key: Mutex<Option<focus_sys::console::WinKeyGuard>>,
}

impl Console {
    /// Aucun changement tant que `enter` n'est pas appelé.
    pub fn new(data: &Path) -> Self {
        Self {
            data: data.to_path_buf(),
            active: Mutex::new(None),
            #[cfg(windows)]
            awake: Mutex::new(None),
            #[cfg(windows)]
            minimized: Mutex::new(Vec::new()),
            #[cfg(windows)]
            win_key: Mutex::new(None),
        }
    }

    /// Le mode console est-il actif ?
    pub fn is_active(&self) -> bool {
        lock(&self.active).is_some()
    }

    /// Le bureau de Windows est-il arrêté par le mode console ?
    pub fn shell_stopped(&self) -> bool {
        lock(&self.active).as_ref().is_some_and(|state| state.shell_stopped)
    }

    /// Remasque la barre des tâches si Windows l’a remontrée, sous le même verrou
    /// que la sortie du mode console.
    pub fn keep_taskbar_hidden(&self) {
        let active = lock(&self.active);
        #[cfg(windows)]
        if active.as_ref().is_some_and(|state| state.taskbar_hidden)
            && focus_sys::console::any_taskbar_visible()
        {
            focus_sys::console::set_taskbars_visible(false);
        }
        drop(active);
    }

    /// Fait passer Windows en mode console. Sans journal écrit, rien ne change.
    pub fn enter(&self, wanted: Wanted, on_home: impl Fn() + Send + 'static) {
        self.leave();
        let mut state = ConsoleState {
            taskbar_hidden: wanted.hide_taskbar,
            focus_session: wanted.silence,
            since: now(),
            // Noté avant l'arrêt : un plantage juste après relance quand même le bureau.
            shell_stopped: cfg!(windows) && wanted.stop_shell,
            ..ConsoleState::default()
        };
        #[cfg(windows)]
        if wanted.tweaks {
            // Les valeurs d’origine sont notées avant tout changement.
            state.registry = USER_TWEAKS
                .iter()
                .map(|(path, name, _)| ((*path).to_owned(), (*name).to_owned(), focus_sys::console::user_dword(path, name)))
                .collect();
            state.animations = Some(true);
        }
        if let Err(err) = write(&self.data, &state) {
            tracing::error!("console journal not written, nothing changed: {err}");
            return;
        }
        #[cfg(windows)]
        {
            if state.taskbar_hidden {
                focus_sys::console::set_taskbars_visible(false);
            }
            if wanted.tweaks {
                for (path, name, value) in USER_TWEAKS {
                    focus_sys::console::set_user_dword(path, name, Some(value));
                }
                focus_sys::console::broadcast_settings();
                let before = focus_sys::console::set_animations(false);
                state.animations = Some(before);
                let _ = write(&self.data, &state);
            }
            if state.focus_session && !focus_sys::console::start_focus_session(FOCUS_MINUTES) {
                // Rien n'a été lancé : rien à arrêter plus tard.
                state.focus_session = false;
                let _ = write(&self.data, &state);
            }
            if wanted.keep_awake {
                *lock(&self.awake) = Some(focus_sys::console::KeepAwake::start());
            }
            // Windows passe au second plan : autres fenêtres réduites, touche Windows = launcher.
            *lock(&self.minimized) = focus_sys::console::minimize_others();
            // Plus de bureau : ni barre des tâches, ni menu Démarrer, ni icônes.
            if state.shell_stopped && !focus_sys::console::stop_shell() {
                tracing::warn!("windows desktop still running");
            }
            *lock(&self.win_key) = focus_sys::console::WinKeyGuard::start(on_home);
        }
        tracing::info!("console mode on: {state:?}");
        *lock(&self.active) = Some(state);
    }

    /// Rend à Windows son état normal.
    pub fn leave(&self) {
        // Le verrou reste tenu pendant la remise en état : la tâche qui remasque la
        // barre des tâches ne peut pas passer entre les deux.
        let mut active = lock(&self.active);
        let state = active.take();
        #[cfg(windows)]
        {
            lock(&self.awake).take();
            lock(&self.win_key).take();
            focus_sys::console::restore_windows(&std::mem::take(&mut *lock(&self.minimized)));
        }
        if let Some(state) = state {
            undo(&state);
            tracing::info!("console mode off");
        }
        drop(active);
        let _ = std::fs::remove_file(state_file(&self.data));
    }
}

/// Lance la sentinelle : un second exemplaire discret du launcher qui attend la
/// fin de celui-ci et défait le mode console s'il est resté actif.
pub fn spawn_sentinel(data: &Path) {
    let Ok(exe) = std::env::current_exe() else {
        return;
    };
    let spawned = std::process::Command::new(exe)
        .arg("--sentinel")
        .arg(std::process::id().to_string())
        .arg(data)
        .spawn();
    if let Err(err) = spawned {
        tracing::warn!("sentinel not started: {err}");
    }
}

/// Corps de la sentinelle.
pub fn sentinel(pid: u32, data: &Path) {
    #[cfg(windows)]
    focus_sys::console::wait_for_exit(pid);
    #[cfg(not(windows))]
    let _ = pid;
    recover(data);
    // La manette PlayStation masquée pour Focus Input redevient visible.
    crate::runtime::pads_restore(data);
    // Filet de sécurité : la barre des tâches revient toujours quand Focus se ferme.
    #[cfg(windows)]
    {
        // Jamais de PC sans bureau une fois Focus fermé, quoi que dise le journal.
        focus_sys::console::start_shell();
        focus_sys::console::set_taskbars_visible(true);
        // L’Explorateur peut la remasquer en refermant son menu : seconde passe.
        std::thread::sleep(std::time::Duration::from_millis(1500));
        focus_sys::console::set_taskbars_visible(true);
    }
    // Ce qui avait été coupé pour le jeu repart aussi.
    #[cfg(windows)]
    {
        let services = crate::guardian_client::GuardianClient::connect()
            .map(|client| Box::new(client) as Box<dyn focus_sys::ops::ServiceControl>);
        let mut ops = focus_sys::ops::WinOps::new(services);
        let journal = focus_core::journal::JournalStore::new(data.join("session.lock.json"));
        if let Err(err) = focus_core::session::recover(&mut ops, &journal) {
            tracing::error!("sentinel recovery failed: {err}");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOTHING: Wanted = Wanted {
        stop_shell: false,
        hide_taskbar: false,
        silence: false,
        keep_awake: false,
        tweaks: false,
    };

    #[test]
    fn journal_should_exist_while_active_and_vanish_after() -> Result<(), std::io::Error> {
        let data = tempfile::tempdir()?;
        let console = Console::new(data.path());
        console.enter(NOTHING, || {});
        assert!(console.is_active());
        assert!(state_file(data.path()).is_file());
        console.leave();
        assert!(!console.is_active());
        assert!(!state_file(data.path()).exists());
        Ok(())
    }

    #[test]
    fn leftover_journal_should_be_cleared_at_start() -> Result<(), std::io::Error> {
        let data = tempfile::tempdir()?;
        write(data.path(), &ConsoleState::default())?;
        recover(data.path());
        assert!(!state_file(data.path()).exists());
        Ok(())
    }

    #[test]
    fn corrupt_journal_should_be_cleared_without_panicking() -> Result<(), std::io::Error> {
        let data = tempfile::tempdir()?;
        std::fs::write(state_file(data.path()), "{not json")?;
        recover(data.path());
        assert!(!state_file(data.path()).exists());
        Ok(())
    }
}
