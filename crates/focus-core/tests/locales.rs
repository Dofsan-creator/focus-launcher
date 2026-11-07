//! Vérifie toutes les langues : clés complètes, mêmes paramètres, aucun terme interdit.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

/// Termes des constructeurs, interdits dans l'interface (v4.2, section 4).
const FORBIDDEN: &[&str] = &[
    "Control Center",
    "Quick Resume",
    "Switcher",
    "HOME",
    "Quick Settings",
    "Centre de contrôle",
    "Big Picture",
    "PlayStation Store",
    "Xbox Game Bar",
];

/// Langues livrées : elles doivent toutes exister.
const SHIPPED: &[&str] = &[
    "en", "fr", "es", "de", "it", "pt-BR", "ru", "ja", "ko", "zh-CN", "ar",
];

fn locales_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../locales")
}

/// Clé → paramètres utilisés.
fn parse(text: &str) -> BTreeMap<String, BTreeSet<String>> {
    let mut messages: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut current = None::<String>;
    for line in text.lines() {
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let starts_message = line.chars().next().is_some_and(|c| c.is_ascii_lowercase());
        if starts_message {
            if let Some((key, _)) = line.split_once(" =") {
                current = Some(key.trim().to_owned());
                messages.entry(key.trim().to_owned()).or_default();
            }
        }
        if let Some(key) = &current {
            let params = messages.entry(key.clone()).or_default();
            let mut rest = line;
            while let Some(start) = rest.find("{ $") {
                let tail = &rest[start + 3..];
                let end = tail.find([' ', '}']).unwrap_or(tail.len());
                params.insert(tail[..end].to_owned());
                rest = &tail[end..];
            }
        }
    }
    messages
}

fn load(code: &str) -> String {
    fs::read_to_string(locales_dir().join(format!("{code}.ftl"))).unwrap_or_default()
}

#[test]
fn every_shipped_language_should_exist() {
    let missing: Vec<_> = SHIPPED
        .iter()
        .filter(|code| load(code).is_empty())
        .collect();
    assert!(missing.is_empty(), "missing: {missing:?}");
}

#[test]
fn every_language_should_have_every_key_with_the_same_parameters() {
    let reference = parse(&load("en"));
    assert!(reference.len() > 100);
    let mut problems = Vec::new();
    for code in SHIPPED {
        let messages = parse(&load(code));
        for (key, params) in &reference {
            match messages.get(key) {
                None => problems.push(format!("{code}: missing {key}")),
                Some(found) if found != params => {
                    problems.push(format!("{code}: {key} params {found:?}"))
                }
                Some(_) => {}
            }
        }
        for key in messages.keys().filter(|key| !reference.contains_key(*key)) {
            problems.push(format!("{code}: unknown key {key}"));
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn no_language_should_contain_a_manufacturer_term() {
    let mut problems = Vec::new();
    for code in SHIPPED {
        let text = load(code);
        for line in text.lines().filter(|line| !line.starts_with('#')) {
            for term in FORBIDDEN {
                if line.contains(term) {
                    problems.push(format!("{code}: `{term}` in `{line}`"));
                }
            }
        }
    }
    assert!(problems.is_empty(), "{}", problems.join("\n"));
}

#[test]
fn no_message_should_be_empty() {
    for code in SHIPPED {
        for line in load(code).lines() {
            assert!(
                !line.trim_end().ends_with(" ="),
                "{code}: empty message `{line}`"
            );
        }
    }
}

#[test]
fn language_list_document_should_cover_every_shipped_language() {
    let doc = fs::read_to_string(locales_dir().join("../docs/langues.md")).unwrap_or_default();
    let missing: Vec<_> = SHIPPED
        .iter()
        .filter(|code| !doc.contains(&format!("`{code}`")))
        .collect();
    assert!(missing.is_empty(), "docs/langues.md misses {missing:?}");
}
