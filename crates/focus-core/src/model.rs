//! Modèles partagés entre le moteur, la bibliothèque et l'interface.

use serde::{Deserialize, Serialize};

/// Type d'une entrée : il pilote le profil d'optimisation par défaut.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum EntryKind {
    /// Jeu.
    Game,
    /// Logiciel (3D, montage, développement).
    Software,
    /// Application sociale (voix, diffusion, musique).
    Social,
    /// Autre : aucune optimisation.
    Other,
}

/// Origine d'une entrée.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Store {
    /// Steam.
    Steam,
    /// Epic Games.
    Epic,
    /// GOG.
    Gog,
    /// EA App.
    Ea,
    /// Ubisoft Connect.
    Ubisoft,
    /// Battle.net.
    BattleNet,
    /// Xbox et Microsoft Store.
    Xbox,
    /// Ajout manuel.
    Manual,
}

/// Comment démarrer une entrée.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "type")]
pub enum LaunchTarget {
    /// Exécutable, raccourci ou script local.
    Executable {
        /// Chemin complet.
        path: String,
        /// Arguments.
        args: Vec<String>,
        /// Dossier de travail.
        working_dir: Option<String>,
    },
    /// Adresse de protocole gérée par une boutique (`steam://…`).
    Protocol {
        /// Adresse complète.
        uri: String,
    },
    /// Application empaquetée du Microsoft Store.
    Packaged {
        /// Identifiant AUMID.
        aumid: String,
    },
    /// Site web ouvert en fenêtre d'application du navigateur.
    Website {
        /// Adresse.
        url: String,
    },
}

/// Statistiques d'utilisation d'une entrée.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// Temps total en secondes.
    pub total_seconds: u64,
    /// Dernière session, en secondes Unix.
    pub last_played: Option<i64>,
    /// Nombre de lancements.
    pub launches: u32,
}

/// Une fiche de la bibliothèque.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    /// Identifiant stable, par exemple `steam:440`.
    pub id: String,
    /// Titre affiché.
    pub title: String,
    /// Type de l'entrée.
    pub kind: EntryKind,
    /// Origine.
    pub store: Store,
    /// Comment la démarrer.
    pub target: LaunchTarget,
    /// Dossier d'installation, s'il est connu.
    pub install_dir: Option<String>,
    /// Image locale.
    pub cover: Option<String>,
    /// Favori.
    pub favorite: bool,
    /// Statistiques.
    pub stats: Stats,
}

/// Un processus observé. Le couple pid et heure de création l'identifie sans
/// ambiguïté : Windows réutilise les pid.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProcessInfo {
    /// Identifiant.
    pub pid: u32,
    /// Heure de création (FILETIME).
    pub start_time: u64,
    /// Nom interne de l'image, sans `.exe`.
    pub name: String,
    /// Chemin de l'exécutable, si lisible.
    pub path: Option<String>,
    /// Mémoire utilisée en octets.
    pub memory: u64,
}

/// État d'un service Windows.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServiceState {
    /// En cours d'exécution.
    Running,
    /// Arrêté.
    Stopped,
    /// État transitoire.
    Other,
}

/// Un service observé, toujours désigné par son nom interne.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceInfo {
    /// Nom interne, indépendant de la langue de Windows.
    pub name: String,
    /// Nom affiché.
    pub display_name: String,
    /// État.
    pub state: ServiceState,
}

/// Instantané du système.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    /// Processus.
    pub processes: Vec<ProcessInfo>,
    /// Services.
    pub services: Vec<ServiceInfo>,
}
