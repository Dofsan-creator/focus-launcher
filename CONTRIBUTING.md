# Contributing

1. Read `SPEC.md` and `CLAUDE.md`. They hold the rules that do not change.
2. Every interactive element must work with a controller, a mouse and a keyboard.
3. No user-visible text in code: add a key to `locales/en.ftl` and to every other locale file.
4. Before a pull request: `cargo test --workspace --exclude focus-ui`, both clippy commands from `CLAUDE.md` and `cargo fmt --all`.
5. Translations: see `docs/langues.md`. A machine translation stays marked "to be reviewed".
6. Never copy code from DS4Windows or any GPL-incompatible project. List new dependencies in `docs/licenses.md`.

By contributing you agree that your work is licensed under GPL-3.0-or-later.
