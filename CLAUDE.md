# Focus Launcher : règles permanentes

`SPEC.md` fait foi. Les specs d'origine sont dans `docs/specs/`.

## Règles valables partout

- **Rust partout.** Interface : Tauri 2 + Leptos (WebAssembly). Pas de code C# repris.
- **Inspiration des consoles, exécution originale.** On reprend des comportements, jamais une apparence, un nom, un son ou une icône.
- **Termes interdits dans l'interface** : Control Center, Quick Resume, Switcher, HOME, Quick Settings, Centre de contrôle, Big Picture. Le test `crates/focus-core/tests/locales.rs` les cherche.
- **Trois entrées** : chaque écran se pilote à la manette, à la souris et au clavier. Tout élément interactif est un `button.f`.
- **Toutes les langues** : aucune chaîne en dur dans l'interface, une langue = un fichier `locales/*.ftl`.
- **Sécurité** : journal avant toute coupure, liste blanche compilée, registre limité aux valeurs du compte de SPEC D15 (journalisées, rendues), aucune injection, aucun handle sur un jeu de boutique, aucune coupure réelle sans accord de l'utilisateur.
- **À chaque phase** : analyser 2 à 4 options, choisir, écrire pourquoi dans `docs/decisions.md` (v4.5).

## Commandes

- Tests : `cargo test --workspace --exclude focus-ui` (terminal administrateur : `focus-launcher` exige l'élévation, SPEC D5)
- Lint : `cargo clippy --workspace --exclude focus-ui --all-targets -- -D warnings` et `cargo clippy -p focus-ui --target wasm32-unknown-unknown -- -D warnings`
- Interface seule (navigateur, données de démonstration) : `cd crates/focus-ui && trunk serve`
- Application : `cd crates/focus-app && cargo tauri dev`
- Installeur : `cd crates/focus-app && cargo tauri build`

## Skills du projet (`.claude/skills/`)

`analyse-de-phase`, `traduction-langues`, `termes-constructeurs`, `revue-originalite`, `trois-entrees`, `suivi-progress`, `catalogue-installation` (appel manuel), `demarrage-session` (appel manuel). Skills externes installés : `rust-best-practices`, `tauri-v2`, `find-skills`.

## Suivi

`PROGRESS.md` (phases et conditions d'arrêt), `AUDIT.md` (constats), `docs/decisions.md` (choix).
