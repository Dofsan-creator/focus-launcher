//! Modes de manette et séquence de basculement au lancement.

use crate::pad::Family;
use serde::{Deserialize, Serialize};

/// Mode de manette.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Mode {
    /// Lecture directe, sans pilote.
    Native,
    /// Traduction par Steam.
    SteamInput,
    /// Moteur intégré avec manette virtuelle.
    FocusInput,
    /// Aucune manette ajoutée (logiciels).
    None,
}

/// Ce qui est disponible sur la machine.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Environment {
    /// Steam tourne.
    pub steam_running: bool,
    /// Le pilote de manette virtuelle est présent.
    pub vigem_present: bool,
    /// HidHide est présent.
    pub hidhide_present: bool,
}

/// Choisit le mode. Le choix de l'utilisateur prime toujours, sauf s'il est
/// impossible : on retombe alors sur Natif, sans erreur.
///
/// Sans HidHide, la manette virtuelle s'ajoute à la vraie au lieu de la remplacer :
/// le jeu voit deux manettes et les commandes se doublent. Focus Input n'est donc
/// jamais choisi tout seul sans HidHide ; la manette reste en natif (USB ou
/// Bluetooth, Windows et la plupart des jeux la reconnaissent).
pub fn resolve_mode(family: Family, env: Environment, user_choice: Option<Mode>) -> Mode {
    let candidate = user_choice.unwrap_or(match family {
        Family::PlayStation if env.steam_running => Mode::SteamInput,
        Family::PlayStation if env.vigem_present && env.hidhide_present => Mode::FocusInput,
        _ => Mode::Native,
    });
    match candidate {
        Mode::SteamInput if !env.steam_running => Mode::Native,
        Mode::FocusInput if !env.vigem_present => Mode::Native,
        other => other,
    }
}

/// Une étape de basculement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Transition {
    /// Arrêter le moteur intégré.
    StopEngine,
    /// Supprimer la manette virtuelle.
    RemoveVirtualPad,
    /// Démasquer la manette physique, sinon Steam ne la voit pas.
    UnhidePhysicalPad,
    /// Attendre que Windows ré-énumère la manette.
    WaitForDevice,
    /// Déclarer le launcher dans HidHide : il doit garder la manette masquée.
    AllowLauncherInHidHide,
    /// Masquer la manette physique.
    HidePhysicalPad,
    /// Créer la manette virtuelle.
    CreateVirtualPad,
    /// Démarrer le moteur intégré.
    StartEngine,
}

/// Séquence pour passer d'un mode à l'autre. L'ordre évite la double saisie.
pub fn transitions(from: Mode, to: Mode, hidhide: bool) -> Vec<Transition> {
    use Transition::{
        AllowLauncherInHidHide, CreateVirtualPad, HidePhysicalPad, RemoveVirtualPad, StartEngine,
        StopEngine, UnhidePhysicalPad, WaitForDevice,
    };
    if from == to {
        return Vec::new();
    }
    let mut steps = Vec::new();
    if from == Mode::FocusInput {
        steps.extend([StopEngine, RemoveVirtualPad]);
        if hidhide {
            steps.extend([UnhidePhysicalPad, WaitForDevice]);
        }
    }
    if to == Mode::FocusInput {
        if hidhide {
            steps.extend([AllowLauncherInHidHide, HidePhysicalPad]);
        }
        steps.extend([CreateVirtualPad, StartEngine]);
    }
    steps
}

/// Après un plantage : manette physique visible, manette virtuelle supprimée.
pub fn crash_recovery_steps() -> [Transition; 2] {
    [Transition::RemoveVirtualPad, Transition::UnhidePhysicalPad]
}

#[cfg(test)]
mod tests {
    use super::*;

    const ALL: Environment = Environment {
        steam_running: true,
        vigem_present: true,
        hidhide_present: true,
    };
    const NOTHING: Environment = Environment {
        steam_running: false,
        vigem_present: false,
        hidhide_present: false,
    };

    #[test]
    fn xbox_pad_should_default_to_native() {
        assert_eq!(resolve_mode(Family::Xbox, ALL, None), Mode::Native);
    }

    #[test]
    fn ds4_with_steam_should_default_to_steam_input() {
        assert_eq!(
            resolve_mode(Family::PlayStation, ALL, None),
            Mode::SteamInput
        );
    }

    #[test]
    fn ds4_without_steam_should_default_to_focus_input() {
        let env = Environment {
            steam_running: false,
            ..ALL
        };
        assert_eq!(
            resolve_mode(Family::PlayStation, env, None),
            Mode::FocusInput
        );
    }

    #[test]
    fn ds4_without_hidhide_should_stay_native_to_avoid_double_pads() {
        let env = Environment {
            steam_running: false,
            vigem_present: true,
            hidhide_present: false,
        };
        assert_eq!(resolve_mode(Family::PlayStation, env, None), Mode::Native);
    }

    #[test]
    fn user_choice_should_win_over_detection() {
        assert_eq!(
            resolve_mode(Family::PlayStation, ALL, Some(Mode::Native)),
            Mode::Native
        );
    }

    #[test]
    fn closed_steam_should_fall_back_to_native() {
        assert_eq!(
            resolve_mode(Family::PlayStation, NOTHING, Some(Mode::SteamInput)),
            Mode::Native
        );
    }

    #[test]
    fn missing_driver_should_fall_back_to_native() {
        assert_eq!(
            resolve_mode(Family::PlayStation, NOTHING, Some(Mode::FocusInput)),
            Mode::Native
        );
    }

    #[test]
    fn leaving_focus_input_should_unhide_before_steam_takes_over() {
        let steps = transitions(Mode::FocusInput, Mode::SteamInput, true);
        assert_eq!(
            steps,
            [
                Transition::StopEngine,
                Transition::RemoveVirtualPad,
                Transition::UnhidePhysicalPad,
                Transition::WaitForDevice
            ]
        );
    }

    #[test]
    fn entering_focus_input_should_allow_the_launcher_before_hiding() {
        let steps = transitions(Mode::Native, Mode::FocusInput, true);
        assert_eq!(steps[0], Transition::AllowLauncherInHidHide);
    }

    #[test]
    fn focus_input_without_hidhide_should_still_work() {
        let steps = transitions(Mode::Native, Mode::FocusInput, false);
        assert_eq!(
            steps,
            [Transition::CreateVirtualPad, Transition::StartEngine]
        );
    }

    #[test]
    fn staying_in_a_mode_should_do_nothing() {
        assert!(transitions(Mode::FocusInput, Mode::FocusInput, true).is_empty());
    }
}
