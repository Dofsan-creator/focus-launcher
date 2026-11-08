---
name: suivi-progress
description: Use at the end of a phase, before claiming something is done, or when updating PROGRESS.md, AUDIT.md or the stop conditions.
---
# Suivi

- `PROGRESS.md` : par phase, statut, commandes lancées avec leur résultat réel, bilan (ce qui marche, ce qui est fragile, ce qu'on améliorerait), skills utilisés.
- `AUDIT.md` : une ligne par constat, avec date, gravité, décision et commit.
- Ne jamais cocher une condition qui demande du matériel ou un humain : elle reste « à faire par l'utilisateur ».
- Toujours lancer avant de conclure : `cargo test --workspace --exclude focus-ui` et les deux commandes clippy de `CLAUDE.md`.
