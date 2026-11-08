---
name: traduction-langues
description: Use when adding, completing or checking a language, editing locales/*.ftl, or when the user mentions traduction, langue, RTL, arabe, pluriel.
---
# Traductions

- Source : `locales/en.ftl`. Une langue = un fichier `locales/<code>.ftl` + une ligne `lang!` dans `crates/focus-ui/src/i18n.rs` + une ligne dans `docs/langues.md` et dans `SHIPPED` (`crates/focus-core/tests/locales.rs`).
- Ne jamais traduire les clés ni les `{ $paramètres }`. Pluriels Fluent : catégories CLDR de la langue, toujours une branche `*[other]`.
- Jamais de phrase assemblée par morceaux dans le code.
- Statut « à relire » tant qu'une personne compétente n'a pas relu. Jamais « validée » pour une traduction de l'agent.
- Vérifier : `cargo test -p focus-core --test locales`.
