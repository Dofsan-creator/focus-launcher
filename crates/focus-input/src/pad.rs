//! État neutre d'une manette, indépendant de la source.

use serde::{Deserialize, Serialize};

/// Famille d'une manette : elle décide des icônes affichées.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Family {
    /// Xbox et compatibles.
    Xbox,
    /// DualShock 4 et DualSense.
    PlayStation,
    /// Inconnue.
    Other,
}

/// Bouton physique, dans une disposition neutre (position, pas étiquette).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Button {
    /// Bouton central (PS, Xbox) : retour au launcher.
    Guide,
    /// Bas (A, croix).
    South,
    /// Droite (B, rond).
    East,
    /// Gauche (X, carré).
    West,
    /// Haut (Y, triangle).
    North,
    /// Épaule gauche.
    LeftShoulder,
    /// Épaule droite.
    RightShoulder,
    /// Gâchette gauche.
    LeftTrigger,
    /// Gâchette droite.
    RightTrigger,
    /// Back, Share.
    Back,
    /// Start, Options.
    Start,
    /// Croix directionnelle.
    DpadUp,
    /// Croix directionnelle.
    DpadDown,
    /// Croix directionnelle.
    DpadLeft,
    /// Croix directionnelle.
    DpadRight,
}

/// État d'une manette à un instant.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PadState {
    /// Boutons enfoncés.
    pub pressed: Vec<Button>,
    /// Stick gauche horizontal, de -1 à 1.
    pub left_x: f32,
    /// Stick gauche vertical, de -1 (bas) à 1 (haut).
    pub left_y: f32,
    /// Stick droit horizontal.
    pub right_x: f32,
    /// Stick droit vertical.
    pub right_y: f32,
}

impl PadState {
    /// Ce bouton est-il enfoncé ?
    pub fn is_pressed(&self, button: Button) -> bool {
        self.pressed.contains(&button)
    }

    /// Y a-t-il une activité (pour élire la dernière manette utilisée) ?
    pub fn has_activity(&self) -> bool {
        const THRESHOLD: f32 = 0.5;
        !self.pressed.is_empty() || self.left_x.abs() > THRESHOLD || self.left_y.abs() > THRESHOLD
    }
}
