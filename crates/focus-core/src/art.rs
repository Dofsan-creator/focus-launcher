//! Images d'une entrée : clé de dossier et description, partagées par le cœur
//! natif et l'interface WebAssembly. L'interface ne voit jamais un chemin.

use serde::{Deserialize, Serialize};

/// Version du format du cache : la changer force un nouveau calcul.
pub const ART_VERSION: u32 = 2;

/// Sorte d'image.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ArtKind {
    /// Jaquette verticale (2:3).
    Cover,
    /// Grande image de fond, horizontale.
    Hero,
    /// Logo sur fond transparent.
    Logo,
    /// Icône carrée de l'exécutable.
    Icon,
}

impl ArtKind {
    /// Toutes les sortes, dans l'ordre de préférence pour la couleur d'accent.
    pub const ALL: [Self; 4] = [Self::Cover, Self::Hero, Self::Icon, Self::Logo];

    /// Nom dans les adresses (`/cover`).
    pub fn slug(self) -> &'static str {
        match self {
            Self::Cover => "cover",
            Self::Hero => "hero",
            Self::Logo => "logo",
            Self::Icon => "icon",
        }
    }

    /// Lit un nom d'adresse.
    pub fn from_slug(slug: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|kind| kind.slug() == slug)
    }

    /// Nom du fichier dans le cache.
    pub fn file_name(self) -> &'static str {
        match self {
            Self::Cover => "cover.jpg",
            Self::Hero => "hero.jpg",
            Self::Logo => "logo.png",
            Self::Icon => "icon.png",
        }
    }

    /// Type MIME servi.
    pub fn mime(self) -> &'static str {
        match self {
            Self::Cover | Self::Hero => "image/jpeg",
            Self::Logo | Self::Icon => "image/png",
        }
    }
}

/// Ce qui existe pour une entrée.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ArtInfo {
    /// Jaquette.
    pub cover: bool,
    /// Image de fond.
    pub hero: bool,
    /// Logo.
    pub logo: bool,
    /// Icône.
    pub icon: bool,
    /// Couleur dominante, `#rrggbb`, lisible sur fond sombre.
    pub accent: Option<String>,
    /// Version du cache, pour que le navigateur recharge après un changement.
    pub version: u32,
}

impl ArtInfo {
    /// L'image de cette sorte existe-t-elle ?
    pub fn has(&self, kind: ArtKind) -> bool {
        match kind {
            ArtKind::Cover => self.cover,
            ArtKind::Hero => self.hero,
            ArtKind::Logo => self.logo,
            ArtKind::Icon => self.icon,
        }
    }

    /// Note qu'une image existe.
    pub fn set(&mut self, kind: ArtKind, present: bool) {
        match kind {
            ArtKind::Cover => self.cover = present,
            ArtKind::Hero => self.hero = present,
            ArtKind::Logo => self.logo = present,
            ArtKind::Icon => self.icon = present,
        }
    }
}

/// FNV-1a 64 bits : identique en natif et en WebAssembly.
fn fnv1a(text: &str) -> u64 {
    text.bytes().fold(0xcbf2_9ce4_8422_2325, |hash, byte| {
        (hash ^ u64::from(byte)).wrapping_mul(0x0100_0000_01b3)
    })
}

/// Clé de dossier d'une entrée : lisible, sans caractère dangereux, unique.
pub fn art_key(id: &str) -> String {
    let readable: String = id
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '_' })
        .take(40)
        .collect();
    format!("{readable}-{:016x}", fnv1a(id))
}

/// Une clé reçue d'une adresse est-elle valide ? Rien d'autre n'est servi.
pub fn is_valid_key(key: &str) -> bool {
    !key.is_empty()
        && key.len() <= 64
        && key
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_should_be_stable_and_safe() {
        let key = art_key("steam:1449850");
        assert_eq!(key, art_key("steam:1449850"));
        assert!(key.starts_with("steam_1449850-"));
        assert!(is_valid_key(&key));
    }

    #[test]
    fn similar_ids_should_not_share_a_folder() {
        assert_ne!(art_key("steam:1"), art_key("steam_1"));
    }

    #[test]
    fn path_like_ids_should_never_escape_the_cache() {
        let key = art_key(r"manual:..\..\windows\system32\cmd.exe");
        assert!(!key.contains('.') && !key.contains('\\') && !key.contains('/'));
        assert!(is_valid_key(&key));
    }

    #[test]
    fn hostile_keys_should_be_refused() {
        for key in ["", "../x", "a/b", r"a\b", "a.png", &"x".repeat(65)] {
            assert!(!is_valid_key(key), "{key}");
        }
    }

    #[test]
    fn kinds_should_round_trip_through_their_slug() {
        for kind in ArtKind::ALL {
            assert_eq!(ArtKind::from_slug(kind.slug()), Some(kind));
        }
        assert_eq!(ArtKind::from_slug("thumbnail"), None);
    }
}
