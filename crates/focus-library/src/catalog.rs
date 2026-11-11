//! Catalogue des applications installables depuis le launcher.
//!
//! Le fichier est compilé dans le programme : aucune source ajoutée par
//! l'utilisateur en version 1. Chaque identifiant a été vérifié avec winget.

use serde::{Deserialize, Serialize};

const CATALOG_JSON: &str = include_str!("../data/catalog.json");

/// Catégorie d'une application.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Category {
    /// Boutique de jeux, filtre Jeux.
    Store,
    /// Application sociale, filtre Social.
    Social,
    /// Pilote ou bibliothèque nécessaire aux jeux (Visual C++, DirectX, .NET).
    Runtime,
}

/// Une application du catalogue.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct CatalogApp {
    /// Identifiant.
    pub id: String,
    /// Nom affiché.
    pub name: String,
    /// Catégorie.
    pub category: Category,
    /// Identifiant winget vérifié.
    pub winget_id: String,
    /// Page officielle de téléchargement.
    pub official_page: String,
    /// Éditeur attendu.
    pub publisher: String,
    /// Processus du client.
    pub process_name: String,
}

/// Charge le catalogue compilé.
pub fn load() -> Result<Vec<CatalogApp>, serde_json::Error> {
    serde_json::from_str(CATALOG_JSON)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_should_parse() -> Result<(), serde_json::Error> {
        assert!(load()?.len() >= 11);
        Ok(())
    }

    #[test]
    fn every_official_page_should_use_https() -> Result<(), serde_json::Error> {
        assert!(load()?
            .iter()
            .all(|app| app.official_page.starts_with("https://")));
        Ok(())
    }

    #[test]
    fn ids_should_be_unique() -> Result<(), serde_json::Error> {
        let apps = load()?;
        let mut ids: Vec<_> = apps.iter().map(|app| app.id.as_str()).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), apps.len());
        Ok(())
    }

    #[test]
    fn winget_ids_should_stay_plain_identifiers() -> Result<(), serde_json::Error> {
        // L'identifiant finit en argument de winget, appelé sans shell : lettres,
        // chiffres, points, et le « + » de l'identifiant officiel de Visual C++.
        let safe = |id: &str| id.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '+');
        assert!(load()?.iter().all(|app| safe(&app.winget_id)));
        Ok(())
    }
}
