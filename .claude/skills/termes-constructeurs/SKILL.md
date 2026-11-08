---
name: termes-constructeurs
description: Use when naming a feature or screen, writing interface text, or reviewing translations for console manufacturer terms.
---
# Termes interdits (v4.2, section 4)

Interdits dans l'interface : Control Center, Quick Resume, Switcher, HOME, Guide (comme nom de fonction), Cards, Quick Settings, Centre de contrôle, Big Picture.
Permis dans la documentation pour décrire une compatibilité : « manette PlayStation », « manette Xbox ».

Le test `no_language_should_contain_a_manufacturer_term` de `crates/focus-core/tests/locales.rs` échoue si un terme apparaît. Pour ajouter un terme, l'ajouter à `FORBIDDEN`.
