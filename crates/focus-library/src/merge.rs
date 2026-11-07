//! Fusion des doublons : un jeu présent dans deux boutiques apparaît une fois.

use focus_core::model::Entry;
use serde::Serialize;
use std::collections::BTreeMap;

/// Une ligne d'affichage : une fiche, et ses autres sources.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MergedEntry {
    /// Fiche principale.
    pub entry: Entry,
    /// Autres fiches du même jeu, choisies au lancement.
    pub alternates: Vec<Entry>,
}

/// Clé de comparaison : casse, ponctuation et symboles de marque ignorés.
pub fn title_key(title: &str) -> String {
    title
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect()
}

/// Regroupe par titre, en gardant l'ordre de la première apparition.
pub fn merge_duplicates(entries: Vec<Entry>) -> Vec<MergedEntry> {
    let mut order = Vec::new();
    let mut groups: BTreeMap<String, Vec<Entry>> = BTreeMap::new();
    for entry in entries {
        let mut key = title_key(&entry.title);
        if key.is_empty() {
            key = entry.id.clone();
        }
        if !groups.contains_key(&key) {
            order.push(key.clone());
        }
        groups.entry(key).or_default().push(entry);
    }
    order
        .into_iter()
        .filter_map(|key| {
            let mut group = groups.remove(&key)?.into_iter();
            Some(MergedEntry {
                entry: group.next()?,
                alternates: group.collect(),
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use focus_core::model::{EntryKind, LaunchTarget, Stats, Store};

    fn game(id: &str, title: &str, store: Store) -> Entry {
        Entry {
            id: id.to_owned(),
            title: title.to_owned(),
            kind: EntryKind::Game,
            store,
            target: LaunchTarget::Protocol { uri: String::new() },
            install_dir: None,
            cover: None,
            favorite: false,
            stats: Stats::default(),
        }
    }

    #[test]
    fn same_game_on_two_stores_should_merge() {
        let merged = merge_duplicates(vec![
            game("steam:1", "The Witcher® 3: Wild Hunt", Store::Steam),
            game("gog:2", "The Witcher 3 - Wild Hunt", Store::Gog),
        ]);
        assert_eq!((merged.len(), merged[0].alternates.len()), (1, 1));
    }

    #[test]
    fn different_games_should_stay_apart() {
        let merged = merge_duplicates(vec![
            game("a", "Doom", Store::Steam),
            game("b", "Doom 3", Store::Steam),
        ]);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn titles_in_non_latin_scripts_should_not_collapse() {
        let merged = merge_duplicates(vec![
            game("a", "ペルソナ", Store::Steam),
            game("b", "龙之信条", Store::Steam),
        ]);
        assert_eq!(merged.len(), 2);
    }

    #[test]
    fn symbol_only_titles_should_not_collapse_together() {
        let merged = merge_duplicates(vec![
            game("a", "!!!", Store::Steam),
            game("b", "???", Store::Gog),
        ]);
        assert_eq!(merged.len(), 2);
    }
}
