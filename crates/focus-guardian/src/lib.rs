//! Logique du service élevé `focus-guardian`.
//!
//! Le protocole est volontairement minuscule : une demande est un nom de service,
//! vérifié contre une liste compilée. Un programme local ne peut donc ni arrêter
//! un service arbitraire ni rien exécuter. Si le client disparaît, tout est
//! restauré : l'interface peut planter sans laisser Windows coupé (SPEC D6).
#![deny(missing_docs)]

use focus_core::journal::{Journal, JournalEntry, JournalError, JournalStore, Original, StepState};
use focus_core::plan::Step;
use focus_core::whitelist;
use serde::{Deserialize, Serialize};

/// Nom du service Windows.
pub const SERVICE_NAME: &str = "FocusGuardian";
/// Canal nommé.
pub const PIPE_NAME: &str = r"\\.\pipe\focus-guardian";

/// Seuls services que le gardien accepte d'arrêter. Compilé, jamais lu d'un fichier.
pub const ALLOWED_SERVICES: &[&str] = &[
    "wsearch",
    "sysmain",
    "wuauserv",
    "usosvc",
    "dosvc",
    "bits",
    "spooler",
    "diagtrack",
];

/// Demande d'un client.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "op")]
pub enum Request {
    /// Arrêter un service.
    Stop {
        /// Nom interne.
        name: String,
    },
    /// Redémarrer un service arrêté par le gardien.
    Start {
        /// Nom interne.
        name: String,
    },
    /// Tout restaurer : fin normale de session.
    RestoreAll,
}

/// Réponse du gardien.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
pub enum Response {
    /// Succès.
    Ok,
    /// Refus ou échec, en anglais pour les journaux.
    Error {
        /// Motif.
        message: String,
    },
}

/// Accès réel ou simulé au gestionnaire de services.
pub trait ServiceBackend {
    /// Le service tourne-t-il ?
    fn is_running(&mut self, name: &str) -> Result<bool, String>;
    /// Arrête le service et attend.
    fn stop(&mut self, name: &str) -> Result<(), String>;
    /// Démarre le service.
    fn start(&mut self, name: &str) -> Result<(), String>;
}

/// Le gardien accepte-t-il de toucher à ce service ?
pub fn is_allowed(name: &str) -> bool {
    let key = name.trim().to_lowercase();
    ALLOWED_SERVICES.contains(&key.as_str()) && !whitelist::is_protected_service(&key)
}

/// État du gardien : son propre journal, dans un dossier réservé aux administrateurs.
pub struct Guardian<B> {
    backend: B,
    store: JournalStore,
}

impl<B: ServiceBackend> Guardian<B> {
    /// Crée le gardien.
    pub fn new(backend: B, store: JournalStore) -> Self {
        Self { backend, store }
    }

    fn journal(&self) -> Result<Journal, JournalError> {
        Ok(self.store.read()?.unwrap_or(Journal {
            version: 1,
            entry_id: "guardian".to_owned(),
            started_at: 0,
            entries: Vec::new(),
        }))
    }

    /// Traite une demande.
    pub fn handle(&mut self, request: &Request) -> Response {
        let result = match request {
            Request::Stop { name } => self.stop(name),
            Request::Start { name } => self.start(name),
            Request::RestoreAll => self.restore_all(),
        };
        match result {
            Ok(()) => Response::Ok,
            Err(message) => Response::Error { message },
        }
    }

    fn stop(&mut self, name: &str) -> Result<(), String> {
        if !is_allowed(name) {
            return Err(format!("service not allowed: {name}"));
        }
        if !self.backend.is_running(name)? {
            return Ok(());
        }
        let mut journal = self.journal().map_err(|e| e.to_string())?;
        journal.entries.push(JournalEntry {
            step: Step::StopService {
                name: name.to_owned(),
            },
            state: StepState::Captured,
            original: Original::ServiceRunning,
        });
        // Journalisé avant d'agir : un arrêt non noté est impossible.
        self.store.write(&journal).map_err(|e| e.to_string())?;
        self.backend.stop(name)
    }

    fn start(&mut self, name: &str) -> Result<(), String> {
        let mut journal = self.journal().map_err(|e| e.to_string())?;
        let known = |entry: &JournalEntry| matches!(&entry.step, Step::StopService { name: n } if n == name);
        if !journal.entries.iter().any(known) {
            return Err(format!("service was not stopped by guardian: {name}"));
        }
        self.backend.start(name)?;
        journal.entries.retain(|entry| !known(entry));
        self.save(&journal)
    }

    fn save(&self, journal: &Journal) -> Result<(), String> {
        if journal.entries.is_empty() {
            self.store.clear()
        } else {
            self.store.write(journal)
        }
        .map_err(|e| e.to_string())
    }

    /// Restaure tout ce qui reste au journal, dans l'ordre inverse. Appelé à la
    /// déconnexion du client, au démarrage du service et à son arrêt.
    pub fn restore_all(&mut self) -> Result<(), String> {
        let mut journal = match self.store.read() {
            Ok(Some(journal)) => journal,
            Ok(None) => return Ok(()),
            Err(JournalError::Corrupt(err)) => {
                self.store.quarantine().map_err(|e| e.to_string())?;
                return Err(format!("corrupt journal quarantined: {err}"));
            }
            Err(err) => return Err(err.to_string()),
        };
        let mut failures = Vec::new();
        let mut remaining = Vec::new();
        for entry in journal.entries.drain(..).rev() {
            let Step::StopService { name } = &entry.step else {
                continue;
            };
            if let Err(err) = self.backend.start(name) {
                failures.push(format!("{name}: {err}"));
                remaining.insert(0, entry);
            }
        }
        journal.entries = remaining;
        self.save(&journal)?;
        if failures.is_empty() {
            Ok(())
        } else {
            Err(failures.join("; "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[derive(Default)]
    struct Fake {
        running: HashSet<String>,
        log: Vec<String>,
        fail_start: bool,
    }

    impl ServiceBackend for &mut Fake {
        fn is_running(&mut self, name: &str) -> Result<bool, String> {
            Ok(self.running.contains(name))
        }
        fn stop(&mut self, name: &str) -> Result<(), String> {
            self.running.remove(name);
            self.log.push(format!("stop:{name}"));
            Ok(())
        }
        fn start(&mut self, name: &str) -> Result<(), String> {
            if self.fail_start {
                return Err("nope".to_owned());
            }
            self.running.insert(name.to_owned());
            self.log.push(format!("start:{name}"));
            Ok(())
        }
    }

    fn running(names: &[&str]) -> Fake {
        Fake {
            running: names.iter().map(|n| (*n).to_owned()).collect(),
            ..Fake::default()
        }
    }

    fn store(dir: &tempfile::TempDir) -> JournalStore {
        JournalStore::new(dir.path().join("guardian.lock.json"))
    }

    fn stop(name: &str) -> Request {
        Request::Stop {
            name: name.to_owned(),
        }
    }

    #[test]
    fn protected_service_should_be_refused_even_if_listed() {
        assert!(!is_allowed("WinDefend") && !is_allowed("wscsvc"));
    }

    #[test]
    fn no_allowed_service_should_be_protected() {
        assert!(ALLOWED_SERVICES
            .iter()
            .all(|name| !whitelist::is_protected_service(name)));
    }

    #[test]
    fn unlisted_service_should_be_refused_and_left_running() -> Result<(), std::io::Error> {
        let dir = tempfile::tempdir()?;
        let mut fake = running(&["evilsvc"]);
        let response = Guardian::new(&mut fake, store(&dir)).handle(&stop("evilsvc"));
        assert!(matches!(response, Response::Error { .. }));
        assert!(fake.log.is_empty());
        Ok(())
    }

    #[test]
    fn stopped_service_should_be_journaled_first() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let mut fake = running(&["wsearch"]);
        Guardian::new(&mut fake, store(&dir)).handle(&stop("wsearch"));
        assert_eq!(store(&dir).read()?.map(|j| j.entries.len()), Some(1));
        Ok(())
    }

    #[test]
    fn already_stopped_service_should_not_be_restarted_later(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let mut fake = running(&[]);
        let mut guardian = Guardian::new(&mut fake, store(&dir));
        guardian.handle(&stop("spooler"));
        guardian.handle(&Request::RestoreAll);
        drop(guardian);
        assert!(fake.log.is_empty());
        Ok(())
    }

    #[test]
    fn client_loss_should_restore_in_reverse_order() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let mut fake = running(&["wsearch", "bits"]);
        let mut guardian = Guardian::new(&mut fake, store(&dir));
        guardian.handle(&stop("wsearch"));
        guardian.handle(&stop("bits"));
        guardian.restore_all()?;
        drop(guardian);
        assert_eq!(&fake.log[2..], ["start:bits", "start:wsearch"]);
        assert_eq!(store(&dir).read()?, None);
        Ok(())
    }

    #[test]
    fn restart_of_the_guardian_should_finish_a_previous_session(
    ) -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let mut fake = running(&["wsearch"]);
        Guardian::new(&mut fake, store(&dir)).handle(&stop("wsearch"));
        let mut rebooted = running(&[]);
        Guardian::new(&mut rebooted, store(&dir)).restore_all()?;
        assert_eq!(rebooted.log, ["start:wsearch"]);
        Ok(())
    }

    #[test]
    fn start_should_refuse_a_service_the_guardian_did_not_stop() -> Result<(), std::io::Error> {
        let dir = tempfile::tempdir()?;
        let mut fake = running(&[]);
        let response = Guardian::new(&mut fake, store(&dir)).handle(&Request::Start {
            name: "wsearch".to_owned(),
        });
        assert!(matches!(response, Response::Error { .. }));
        Ok(())
    }

    #[test]
    fn failed_restore_should_keep_the_entry_for_a_retry() -> Result<(), Box<dyn std::error::Error>>
    {
        let dir = tempfile::tempdir()?;
        let mut fake = running(&["wsearch"]);
        Guardian::new(&mut fake, store(&dir)).handle(&stop("wsearch"));
        fake.fail_start = true;
        let result = Guardian::new(&mut fake, store(&dir)).restore_all();
        assert!(result.is_err());
        assert_eq!(store(&dir).read()?.map(|j| j.entries.len()), Some(1));
        Ok(())
    }

    #[test]
    fn unknown_request_should_not_parse() {
        assert!(serde_json::from_str::<Request>(r#"{"op":"exec","cmd":"calc"}"#).is_err());
    }
}
