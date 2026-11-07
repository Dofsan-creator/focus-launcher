//! Réglages de l'utilisateur, stockés dans SQLite.

use focus_input::modes::Mode;
use serde::{Deserialize, Serialize};

/// Style des icônes de boutons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GlyphStyle {
    /// Selon la manette détectée.
    Auto,
    /// Forme géométrique façon PlayStation.
    PlayStation,
    /// Lettres façon Xbox.
    Xbox,
}

/// Réglages. Les valeurs par défaut sont les plus prudentes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    /// Langue choisie. `None` tant que le premier démarrage n'est pas fini.
    pub language: Option<String>,
    /// L'assistant du premier démarrage est terminé.
    pub first_run_done: bool,
    /// Coupures réelles autorisées. Faux : mode simulation, rien n'est touché.
    pub real_optimization: bool,
    /// Passer en performances élevées pendant une session.
    pub high_performance: bool,
    /// Empêcher la veille pendant une session.
    pub keep_awake: bool,
    /// Garder Discord et les applications sociales intouchées.
    pub keep_social: bool,
    /// Bloquer Big Picture tant que le launcher est actif.
    pub block_big_picture: bool,
    /// Style des icônes.
    pub glyphs: GlyphStyle,
    /// Mode de manette imposé.
    pub controller_mode: Option<Mode>,
    /// Musique et sons.
    pub sounds: bool,
    /// Réduire les animations.
    pub reduce_motion: bool,
    /// Thème selon l'heure.
    pub auto_theme: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: None,
            first_run_done: false,
            real_optimization: false,
            high_performance: false,
            keep_awake: true,
            keep_social: true,
            block_big_picture: true,
            glyphs: GlyphStyle::Auto,
            controller_mode: None,
            sounds: true,
            reduce_motion: false,
            auto_theme: false,
        }
    }
}
