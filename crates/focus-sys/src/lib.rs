//! Implémentation Windows des opérations de Focus Launcher.
//!
//! Aucune écriture dans le registre. Aucun handle n'est ouvert sur un jeu de
//! boutique (SPEC D7). Le code de décision reste dans `focus-core`.
#![deny(missing_docs)]
#![cfg(windows)]

pub mod audio;
pub mod launch;
pub mod ops;
pub mod report;
pub mod scan;
pub mod shell;
pub mod winget;

mod wide;
