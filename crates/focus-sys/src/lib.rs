//! Implémentation Windows des opérations de Focus Launcher.
//!
//! Aucune écriture dans le registre. Aucun handle n'est ouvert sur un jeu de
//! boutique (SPEC D7). Le code de décision reste dans `focus-core`.
#![deny(missing_docs)]
#![cfg(windows)]

pub mod audio;
pub mod connect;
pub mod console;
pub mod hidhide;
pub mod http;
pub mod icon;
pub mod launch;
pub mod ops;
pub mod pointer;
pub mod report;
pub mod scan;
pub mod shell;
pub mod unelevated;
pub mod winget;

mod wide;
