//! Entrée manette de Focus Launcher.
//!
//! La logique (actions, répétition, combos, modes, décodage DualShock 4) est
//! pure et testée sans matériel. Tout le code est écrit pour ce projet : aucun
//! code de DS4Windows n'est repris (SPEC D4).
#![deny(missing_docs)]

pub mod actions;
pub mod ds4;
pub mod focus_input;
pub mod modes;
pub mod pad;
pub mod pointer;
#[cfg(windows)]
pub mod source;
