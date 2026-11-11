//! Réglages de l'utilisateur, stockés dans SQLite.

use focus_input::modes::Mode;
use serde::{Deserialize, Serialize};

/// Style des icônes de boutons.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum GlyphStyle {
    /// Selon la manette détectée.
    Auto,
    /// Formes géométriques.
    PlayStation,
    /// Lettres.
    Xbox,
}

/// Niveau d'optimisation pendant une session, choisi au premier démarrage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Optimization {
    /// Simulation : rien n'est touché.
    #[default]
    Off,
    /// Profil « Concentration » : applications d'arrière-plan ralenties.
    Balanced,
    /// Profil « Maximal » : tous les leviers autorisés, puis restauration.
    Max,
}

/// Réglages. Les valeurs par défaut sont les plus prudentes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Settings {
    /// Langue choisie. `None` tant que le premier démarrage n'est pas fini.
    pub language: Option<String>,
    /// L'assistant du premier démarrage est terminé.
    pub first_run_done: bool,
    /// Ancien interrupteur, gardé pour relire les anciens réglages.
    pub real_optimization: bool,
    /// Niveau d'optimisation. `Off` tant que l'utilisateur n'a pas accepté.
    pub optimization: Optimization,
    /// Passer en performances élevées pendant une session.
    pub high_performance: bool,
    /// Empêcher la veille pendant une session.
    pub keep_awake: bool,
    /// Garder Discord et les applications sociales intouchées.
    pub keep_social: bool,
    /// Bloquer Big Picture tant que le launcher est actif.
    pub block_big_picture: bool,
    /// Windows passe en mode console tant que le launcher est ouvert.
    pub console_mode: bool,
    /// Mode console : le bureau de Windows (Explorateur) est arrêté tant que Focus
    /// est ouvert, puis relancé (décision 23).
    pub stop_shell: bool,
    /// Mode console : barre des tâches masquée.
    pub hide_taskbar: bool,
    /// Mode console : notifications en sourdine.
    pub silence_notifications: bool,
    /// Style des icônes.
    pub glyphs: GlyphStyle,
    /// Mode de manette imposé.
    pub controller_mode: Option<Mode>,
    /// Sons de navigation.
    pub sounds: bool,
    /// Musique d'ambiance.
    pub music: bool,
    /// Vibration de la manette à la navigation.
    pub vibration: bool,
    /// Réduire les animations.
    pub reduce_motion: bool,
    /// Thème selon l'heure.
    pub auto_theme: bool,
    /// Délai avant l'ouverture à l'ouverture de session, en secondes.
    pub autostart_delay: u32,
    /// À l'ouverture de session, démarrer réduit.
    pub start_minimized: bool,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            language: None,
            first_run_done: false,
            real_optimization: false,
            optimization: Optimization::Off,
            high_performance: false,
            keep_awake: true,
            keep_social: true,
            block_big_picture: true,
            console_mode: true,
            stop_shell: true,
            hide_taskbar: true,
            silence_notifications: true,
            glyphs: GlyphStyle::Auto,
            controller_mode: None,
            sounds: true,
            music: true,
            vibration: true,
            reduce_motion: false,
            auto_theme: false,
            autostart_delay: 3,
            start_minimized: false,
        }
    }
}

impl Settings {
    /// Niveau effectif, en relisant l'ancien interrupteur si besoin.
    pub fn level(&self) -> Optimization {
        match self.optimization {
            Optimization::Off if self.real_optimization => Optimization::Balanced,
            level => level,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_install_should_change_nothing_until_the_user_agrees() {
        assert_eq!(Settings::default().level(), Optimization::Off);
    }

    #[test]
    fn old_settings_should_keep_their_real_optimization() -> Result<(), serde_json::Error> {
        let old: Settings = serde_json::from_str(r#"{"realOptimization":true,"language":"fr"}"#)?;
        assert_eq!(old.level(), Optimization::Balanced);
        assert!(old.console_mode, "missing fields take their defaults");
        Ok(())
    }
}
