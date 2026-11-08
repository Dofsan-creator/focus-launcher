# Focus Launcher

A full-screen launcher for Windows 11 that you drive with a controller, a mouse or a keyboard. It shows your games, software and social apps, and can make temporary, reversible optimizations during a session. Written in Rust (Tauri 2 + Leptos). Licence: GPL-3.0-or-later.

Lanceur plein écran pour Windows 11, piloté à la manette, à la souris ou au clavier. Il affiche tes jeux, logiciels et applications sociales, et peut optimiser temporairement le PC pendant une session, puis tout restaurer. Écrit en Rust (Tauri 2 + Leptos). Licence : GPL-3.0 ou ultérieure.

## Ce qu'il fait / What it does

- Détecte les jeux installés de Steam, Epic Games, GOG, EA, Ubisoft Connect, Battle.net et Xbox, en lecture seule.
- Ajoute n'importe quel programme à la manette, avec un explorateur intégré. Les fichiers sont lus, jamais exécutés pour être inspectés.
- Prépare une session : par défaut en **simulation**, rien n'est touché. L'optimisation réelle (applications d'arrière-plan ralenties, puis restaurées) ne s'active qu'avec ton accord.
- Panneau rapide pendant un jeu (Back + Start maintenus une seconde) : revenir, fermer proprement, régler.
- 11 langues, dont l'arabe en droite à gauche.

## Sécurité / Safety

Journal écrit avant toute modification, liste de protection compilée, restauration automatique après un plantage, aucune écriture dans le registre, aucune injection dans un jeu. Voir `SPEC.md`.

## Compiler / Build

```text
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cargo install tauri-cli --version "^2" --locked
cd crates/focus-app
cargo tauri build
```

Tests : `cargo test --workspace --exclude focus-ui`.

## Documents

`SPEC.md` (décisions), `PROGRESS.md` (avancement), `AUDIT.md` (points ouverts), `docs/` (langues, licences, design, boutiques, manettes).
