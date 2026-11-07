//! Bibliothèque de Focus Launcher : détection, stockage, catalogue.
//!
//! Chaque scanner lit en seule lecture, reçoit ses chemins en argument pour être
//! testable, et échoue proprement : un format changé n'arrête jamais le launcher.
#![deny(missing_docs)]

pub mod catalog;
pub mod discover;
pub mod exe_score;
pub mod merge;
pub mod repo;
pub mod scanners;
pub mod vdf;

use thiserror::Error;

/// Erreur d'un scanner : elle est affichée, jamais fatale.
#[derive(Debug, Error)]
pub enum ScanError {
    /// Lecture impossible.
    #[error("cannot read {path}: {source}")]
    Io {
        /// Fichier concerné.
        path: String,
        /// Cause.
        source: std::io::Error,
    },
    /// Format inattendu.
    #[error("unexpected format in {0}")]
    Format(String),
}
