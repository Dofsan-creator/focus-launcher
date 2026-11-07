//! Fichier de règles `optimizer-rules.json` : chargement et validation stricte.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Cible d'une règle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RuleType {
    /// Processus, par nom d'image.
    Process,
    /// Service, par nom interne.
    Service,
}

/// Action d'une règle, du moins au plus risqué.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum RuleAction {
    /// Mode efficacité (EcoQoS) et priorité basse. Rien n'est fermé.
    Throttle,
    /// Suspension, avec reprise garantie.
    Suspend,
    /// Fermeture propre, avec relance si `restoreAfterGame`.
    Close,
    /// Arrêt temporaire d'un service.
    Stop,
    /// Demander à l'utilisateur.
    AskUser,
}

/// Niveau de risque déclaré.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Risk {
    /// Faible.
    Low,
    /// Moyen.
    Medium,
    /// Élevé.
    High,
}

/// Une règle d'optimisation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Rule {
    /// Processus ou service.
    #[serde(rename = "type")]
    pub rule_type: RuleType,
    /// Nom interne de la cible.
    pub name: String,
    /// Action.
    pub action: RuleAction,
    /// Relancer après la session.
    pub restore_after_game: bool,
    /// Autoriser l'arrêt forcé après la fermeture propre.
    #[serde(default)]
    pub allow_force: bool,
    /// Risque.
    pub risk: Risk,
    /// Raison lisible.
    pub reason: String,
}

/// Un profil de règles.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct Profile {
    /// Nom.
    pub name: String,
    /// Si vrai, le profil n'agit jamais pour de vrai.
    pub simulation_only: bool,
    /// Règles.
    pub rules: Vec<Rule>,
}

/// Contenu de `optimizer-rules.json`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct RuleFile {
    /// Version du format.
    pub version: u32,
    /// Profils.
    pub profiles: Vec<Profile>,
    /// Protections ajoutées par l'utilisateur. Elles s'ajoutent à la liste
    /// compilée, elles ne la remplacent jamais.
    #[serde(default)]
    pub never_touch: Vec<String>,
}

/// Erreur de chargement des règles.
#[derive(Debug, Error)]
pub enum RuleError {
    /// JSON illisible ou champ inconnu.
    #[error("invalid rule file: {0}")]
    Parse(#[from] serde_json::Error),
    /// Version non prise en charge.
    #[error("unsupported rule file version {0}")]
    Version(u32),
    /// Règle incohérente.
    #[error("invalid rule `{name}`: {reason}")]
    Invalid {
        /// Cible de la règle.
        name: String,
        /// Problème.
        reason: &'static str,
    },
}

impl RuleFile {
    /// Analyse et valide le fichier. Un fichier invalide est refusé en entier.
    pub fn parse(json: &str) -> Result<Self, RuleError> {
        let file: Self = serde_json::from_str(json)?;
        if file.version != 1 {
            return Err(RuleError::Version(file.version));
        }
        for rule in file.profiles.iter().flat_map(|profile| &profile.rules) {
            rule.validate()?;
        }
        Ok(file)
    }
}

impl Rule {
    fn validate(&self) -> Result<(), RuleError> {
        let invalid = |reason| {
            Err(RuleError::Invalid {
                name: self.name.clone(),
                reason,
            })
        };
        if self.name.trim().is_empty() {
            return invalid("empty name");
        }
        if self.reason.trim().is_empty() {
            return invalid("empty reason");
        }
        let wrong_target = matches!(
            (self.rule_type, self.action),
            (
                RuleType::Service,
                RuleAction::Close | RuleAction::Suspend | RuleAction::Throttle
            ) | (RuleType::Process, RuleAction::Stop)
        );
        if wrong_target {
            return invalid("action does not match target type");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"{
      "version": 1,
      "profiles": [{ "name": "p", "simulationOnly": true, "rules": [
        { "type": "process", "name": "OneDrive", "action": "close",
          "restoreAfterGame": true, "risk": "low", "reason": "sync" },
        { "type": "service", "name": "WSearch", "action": "stop",
          "restoreAfterGame": true, "risk": "low", "reason": "index" }
      ]}],
      "neverTouch": ["explorer"]
    }"#;

    #[test]
    fn parse_should_accept_the_documented_format() -> Result<(), RuleError> {
        assert_eq!(RuleFile::parse(SAMPLE)?.profiles[0].rules.len(), 2);
        Ok(())
    }

    #[test]
    fn parse_should_reject_unknown_fields() {
        let json = SAMPLE.replace("\"neverTouch\"", "\"allowEverything\"");
        assert!(RuleFile::parse(&json).is_err());
    }

    #[test]
    fn parse_should_reject_broken_json() {
        assert!(RuleFile::parse("{ nope").is_err());
    }

    #[test]
    fn parse_should_reject_future_versions() {
        let json = SAMPLE.replace("\"version\": 1", "\"version\": 2");
        assert!(matches!(RuleFile::parse(&json), Err(RuleError::Version(2))));
    }

    #[test]
    fn parse_should_reject_stopping_a_process() {
        let json = SAMPLE.replace("\"action\": \"close\"", "\"action\": \"stop\"");
        assert!(matches!(
            RuleFile::parse(&json),
            Err(RuleError::Invalid { .. })
        ));
    }
}
