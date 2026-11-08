---
name: demarrage-session
description: Set up or test opening Focus Launcher at session start, including after a reboot. Manual invocation only.
disable-model-invocation: true
---
# Démarrage de session

- Mécanisme : raccourci `Focus Launcher.lnk` dans le dossier Démarrage de l'utilisateur (`focus-sys::shell::set_autostart`). Aucun registre, aucun droit administrateur, jamais de remplacement du shell.
- L'état affiché vient du système (`autostart_enabled`), pas d'un réglage mémorisé.
- Garde-fou : deux plantages de suite au démarrage désactivent le lancement automatique (`focus-app/src/boot_guard.rs`). La récupération de session tourne avant.
- Test (condition 28) : cocher, se déconnecter, se reconnecter ; décocher, recommencer. Noter le résultat dans `PROGRESS.md`.
