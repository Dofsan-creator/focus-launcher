//! Journal de session `session.lock.json`, écrit de façon atomique.

use crate::plan::Step;
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// État d'origine capturé avant l'action.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Original {
    /// Rien à conserver.
    None,
    /// Schéma d'alimentation actif (GUID).
    PowerScheme {
        /// GUID du schéma.
        guid: String,
    },
    /// Classe de priorité d'origine.
    Priority {
        /// Valeur Win32 de la classe.
        class: u32,
    },
    /// Le service tournait.
    ServiceRunning,
    /// Ligne de commande pour relancer le processus.
    Relaunch {
        /// Chemin de l'exécutable.
        path: String,
        /// Arguments.
        args: Vec<String>,
    },
}

/// Avancement d'une étape.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum StepState {
    /// Prévue, rien n'est encore fait.
    Planned,
    /// L'état d'origine est capturé : l'action a pu avoir lieu.
    Captured,
    /// Appliquée.
    Applied,
    /// Restaurée.
    Reverted,
}

/// Une ligne du journal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JournalEntry {
    /// Étape.
    pub step: Step,
    /// Avancement.
    pub state: StepState,
    /// État d'origine.
    pub original: Original,
}

/// Contenu du journal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Journal {
    /// Version du format.
    pub version: u32,
    /// Identifiant de l'entrée lancée.
    pub entry_id: String,
    /// Début de session, en secondes Unix.
    pub started_at: i64,
    /// Lignes, dans l'ordre d'application.
    pub entries: Vec<JournalEntry>,
}

/// Erreur du journal.
#[derive(Debug, Error)]
pub enum JournalError {
    /// Lecture ou écriture.
    #[error("journal I/O error: {0}")]
    Io(#[from] std::io::Error),
    /// Contenu illisible.
    #[error("corrupt journal: {0}")]
    Corrupt(#[from] serde_json::Error),
}

/// Stockage du journal à un emplacement donné.
#[derive(Debug, Clone)]
pub struct JournalStore {
    path: PathBuf,
}

impl JournalStore {
    /// Crée un accès au journal situé à `path`.
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Chemin du journal.
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Écrit par fichier temporaire puis renommage, pour qu'une coupure de
    /// courant ne laisse jamais un journal à moitié écrit.
    pub fn write(&self, journal: &Journal) -> Result<(), JournalError> {
        let bytes = serde_json::to_vec_pretty(journal)?;
        let tmp = self.path.with_extension("json.tmp");
        let mut file = fs::File::create(&tmp)?;
        file.write_all(&bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&tmp, &self.path)?;
        Ok(())
    }

    /// Lit le journal s'il existe.
    pub fn read(&self) -> Result<Option<Journal>, JournalError> {
        match fs::read(&self.path) {
            Ok(bytes) => Ok(Some(serde_json::from_slice(&bytes)?)),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(err) => Err(err.into()),
        }
    }

    /// Supprime le journal : la session est intégralement restaurée.
    pub fn clear(&self) -> Result<(), JournalError> {
        match fs::remove_file(&self.path) {
            Ok(()) => Ok(()),
            Err(err) if err.kind() == std::io::ErrorKind::NotFound => Ok(()),
            Err(err) => Err(err.into()),
        }
    }

    /// Met de côté un journal illisible pour analyse, sans jamais planter.
    pub fn quarantine(&self) -> Result<(), JournalError> {
        fs::rename(&self.path, self.path.with_extension("json.corrupt"))?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> Journal {
        Journal {
            version: 1,
            entry_id: "steam:440".to_owned(),
            started_at: 100,
            entries: vec![JournalEntry {
                step: Step::StopService {
                    name: "WSearch".to_owned(),
                },
                state: StepState::Applied,
                original: Original::ServiceRunning,
            }],
        }
    }

    #[test]
    fn journal_should_read_back_identical() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = JournalStore::new(dir.path().join("session.lock.json"));
        store.write(&sample())?;
        assert_eq!(store.read()?, Some(sample()));
        Ok(())
    }

    #[test]
    fn missing_journal_should_read_as_none() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = JournalStore::new(dir.path().join("session.lock.json"));
        assert_eq!(store.read()?, None);
        Ok(())
    }

    #[test]
    fn corrupt_journal_should_return_an_error_not_panic() -> Result<(), Box<dyn std::error::Error>>
    {
        let dir = tempfile::tempdir()?;
        let path = dir.path().join("session.lock.json");
        fs::write(&path, "{ cut in the mid")?;
        assert!(matches!(
            JournalStore::new(path).read(),
            Err(JournalError::Corrupt(_))
        ));
        Ok(())
    }

    #[test]
    fn clear_should_succeed_twice() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = JournalStore::new(dir.path().join("session.lock.json"));
        store.write(&sample())?;
        store.clear()?;
        store.clear()?;
        assert_eq!(store.read()?, None);
        Ok(())
    }

    #[test]
    fn write_should_leave_no_temporary_file() -> Result<(), Box<dyn std::error::Error>> {
        let dir = tempfile::tempdir()?;
        let store = JournalStore::new(dir.path().join("session.lock.json"));
        store.write(&sample())?;
        assert_eq!(fs::read_dir(dir.path())?.count(), 1);
        Ok(())
    }
}
