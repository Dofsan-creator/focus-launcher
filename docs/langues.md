# Langues

Une langue = un fichier `locales/<code>.ftl` (format Fluent) et une ligne dans `crates/focus-ui/src/i18n.rs`. Aucun autre code ne change. Le test `crates/focus-core/tests/locales.rs` vérifie que chaque langue a toutes les clés de l'anglais, avec les mêmes paramètres, et aucun terme de constructeur.

| Code | Langue | Écriture | Statut |
| --- | --- | --- | --- |
| `en` | English | latine | source |
| `fr` | Français | latine | à relire (écrite par l'agent, l'utilisateur est francophone) |
| `es` | Español | latine | à relire |
| `de` | Deutsch | latine | à relire |
| `it` | Italiano | latine | à relire |
| `pt-BR` | Português (Brasil) | latine | à relire |
| `ru` | Русский | cyrillique | à relire |
| `ja` | 日本語 | japonaise | à relire |
| `ko` | 한국어 | hangeul | à relire |
| `zh-CN` | 简体中文 | chinoise simplifiée | à relire |
| `ar` | العربية | arabe, droite à gauche | à relire |

« À relire » : traduction produite par l'agent, jamais présentée comme validée. Le statut s'affiche dans Paramètres.

## Écart avec la v4.4 : toutes les langues de Windows

La v4.4 bloque la sortie tant que **toutes** les langues d'affichage de Windows ne sont pas présentes. Les 11 langues ci-dessus couvrent chaque famille d'écriture difficile (latin, cyrillique, CJK, arabe RTL) et valident l'architecture. Les autres (environ 100) restent à ajouter, un fichier chacune : c'est le point ouvert « critique » n° 1 de `AUDIT.md`, laissé à ta décision.

## Ajouter une langue

1. Copier `locales/en.ftl` en `locales/<code>.ftl` et traduire chaque valeur, jamais les clés ni les `{ $paramètres }`.
2. Ajouter `lang!("<code>", "<nom dans la langue>", <rtl>, false)` dans `LANGS`.
3. Ajouter une ligne dans ce tableau et dans `SHIPPED` du test.
4. `cargo test -p focus-core --test locales`.
