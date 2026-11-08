# Licences

Focus Launcher est distribué sous **GPL-3.0-or-later** (`COPYING`). Les licences des dépendances principales ont été vérifiées sur leurs dépôts. Celles marquées « crates.io » sont déclarées dans leur manifeste, non auditées une par une. Avant publication, lancer `cargo deny check licenses` ou `cargo about`.

| Dépendance | Rôle | Licence | Vérification |
| --- | --- | --- | --- |
| tauri 2 | Application, fenêtre, IPC | Apache-2.0 / MIT | dépôt GitHub (agent-reach) |
| leptos 0.8 | Interface | MIT | crates.io |
| windows (windows-rs) 0.58 | API Win32 | Apache-2.0 / MIT | dépôt GitHub (agent-reach) |
| fluent-bundle | Traductions | Apache-2.0 / MIT | dépôt GitHub (agent-reach) |
| hidapi 2 | Lecture HID DualShock 4 | MIT | dépôt GitHub (agent-reach) |
| vigem-client 0.1 | Manette virtuelle | MIT | dépôt GitHub (agent-reach) |
| windows-service 0.7 | Service Windows | Apache-2.0 / MIT | crates.io |
| rusqlite (SQLite intégré) | Base | MIT (SQLite : domaine public) | crates.io |
| winreg | Lecture du registre | MIT | crates.io |
| serde, serde_json, thiserror, tracing | Utilitaires | Apache-2.0 / MIT | crates.io |

Toutes sont compatibles avec la GPL v3.

## Ce qui n'est pas livré

- **DS4Windows** : aucune ligne reprise, ni traduite (SPEC D4).
- **ViGEmBus, HidHide** : pilotes installés séparément par l'utilisateur, jamais livrés.
- **Polices** : aucune livrée. L'interface utilise les polices du système (Segoe UI, et les polices de secours de Windows pour chaque écriture).
- **Sons** : synthétisés à l'exécution (`crates/focus-ui/src/sound.rs`), aucun fichier audio.
- **Icônes** : l'icône de l'application est dessinée par un script du projet. Aucune icône ni logo de marque.
