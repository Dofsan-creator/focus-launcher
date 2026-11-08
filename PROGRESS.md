# Avancement

Mis à jour le 2026-11-07. Skills utilisés : `find-skills` (recherche et installation des skills), `agent-reach` (vérification des dépôts et licences sur GitHub et le web), `rust-best-practices`, `tauri-v2`.

## Phases

| Phase | Statut | Preuve |
| --- | --- | --- |
| 0. Environnement | Terminée | Rust 1.99.1 (MSVC), cible wasm32, Visual Studio Build Tools 2022, Git, Trunk |
| 1. Squelette | Terminée | Espace de travail de 7 crates, journalisation `tracing`, lints `unwrap_used` et `expect_used` interdits |
| 2. Analyse en lecture seule | Terminée (code) | `focus-sys::scan`, `report` ; écran Analyse. À relire sur ton PC |
| 3. Règles et simulation | Terminée | Tests : liste blanche refusée même visée, inconnu incertain, fichier invalide rejeté |
| 4. Journal et récupération | Terminée | Tests : journal avant action, reprise après plantage, pid réutilisé ignoré, journal corrompu mis de côté |
| 5. Coupure réelle | Code fini, **jamais exécutée** | Règle 8 : la première coupure réelle demande ton accord. Point de restauration à créer avant |
| 6. Bibliothèque | Terminée (code) | 7 scanners testés sur faux dossiers, SQLite, fusion des doublons, score d'exécutable |
| 7. Interface | Terminée (code) | Iris, navigation spatiale, 11 langues, clavier à l'écran, explorateur intégré |
| 8. Flux « Jouer » | Terminé (code) | Préparation, lancement, suivi sans handle, restauration, écran de fin |
| 8 bis. Panneau rapide | Terminé (code) | Combo Back + Start, retour sans fermer, fermeture douce puis forcée avec seconde confirmation |
| 8 ter. Démarrage et langues | Terminé (code) | Raccourci de démarrage sans registre, garde-fou deux plantages, choix de langue au premier écran |
| 8 quater. Catalogue | Terminé (code) | 11 identifiants winget vérifiés, confirmation, page officielle |
| 9. Livraison | En cours | Installeur NSIS par `cargo tauri build`. Signature non faite |

## Vérifications lancées (résultats réels)

- `cargo test --workspace --exclude focus-ui` : **133 réussis, 0 échec**.
- `cargo clippy --workspace --exclude focus-ui --all-targets -- -D warnings` : 0 avertissement.
- `cargo clippy -p focus-ui --target wasm32-unknown-unknown -- -D warnings` : 0 avertissement.
- `cargo test -p focus-core --test locales` : 11 langues complètes, aucun terme interdit.

## Conditions d'arrêt

Légende : ✅ prouvé par test ou commande, 🧑 à faire par toi (matériel ou jugement humain), ⚠️ ouvert.

| # | Condition | État |
| --- | --- | --- |
| 1 | Compile sans avertissement bloquant | ✅ |
| 2 | Tous les tests passent | ✅ 133/133 |
| 3 | Rapport de simulation correct sur le PC réel | 🧑 |
| 4 | Ce qui a été touché est restauré à l'identique (D9) | ✅ en test ; 🧑 en réel |
| 5 | Récupération après plantage | ✅ en test ; 🧑 en réel, 3 fois |
| 6 | La manette pilote toute l'interface | 🧑 Xbox filaire puis DualShock 4 |
| 7 | Liste blanche jamais touchée | ✅ test sur chaque nom |
| 8 | Phases terminées dans ce fichier | ⚠️ phase 9 |
| 9 | Liste de contrôle manette v2 (DS4 Bluetooth, Xbox filaire) | 🧑 |
| 10 | Après plantage, manette physique visible et virtuelle supprimée | ✅ la manette virtuelle disparaît avec le processus ; 🧑 à vérifier |
| 11, 15 | Gain mesuré A/B pour chaque levier | ⚠️ PresentMon non intégré. Aucun levier actif sans ton accord |
| 12, 30, 33 | Langues complètes et testées | ✅ pour les 11 livrées ; ⚠️ « toutes les langues de Windows » (AUDIT n° 1) |
| 13 | Panneau rapide sur chaque type de plein écran | 🧑 matrice DX11, DX12, Vulkan, anticheat, fenêtré |
| 14 | Fermeture d'un jeu, 3 fois | 🧑 |
| 16 | Aucune injection | ✅ revue : aucune API d'injection, aucun handle sur un jeu de boutique |
| 17 | Jeux détectés sans action, au démarrage à froid | ✅ sur ce PC le 2026-11-08 : Yu-Gi-Oh! Master Duel (Steam), Fortnite et Delta Force (Epic). Exécutable lancé 10 s, 36 Mo, journal sans erreur |
| 18, 22 | Ajout à la manette seule | 🧑 |
| 19, 29 | Graphe de focus complet, trois entrées | ✅ algorithme testé, et parcours automatique dans Chrome : 9/9 éléments de l'accueil atteints aux flèches, en arabe (RTL), panneau rapide qui prend et rend le focus ; 🧑 parcours manuel de chaque écran à la manette |
| 20, 27 | Revue d'originalité validée | 🧑 sur captures |
| 21 | Aucun point critique ou haut ouvert dans `AUDIT.md` | ⚠️ |
| 23 | Un profil Logiciel ne ferme ni ne suspend rien de protégé | ✅ |
| 24 | La fermeture attend la boîte « Enregistrer ? » | ✅ la fermeture douce n'est jamais forcée sans seconde confirmation ; 🧑 |
| 25 | Aucun terme de constructeur | ✅ |
| 26 | Mise en pause testée | suspendue (v4.3) |
| 28 | Démarrage automatique après redémarrage réel | 🧑 |
| 31 | Deux applications sociales testées | 🧑 |
| 32, 36 | Aucun logo tiers, aucun installeur dans le dépôt | ✅ |
| 34 | `CLAUDE.md` et skills présents, moins de 500 lignes chacun | ✅ |
| 35 | Installation testée (2 par winget, 1 par page officielle) | 🧑 modifie la machine |
| 37–40 | Décisions et bilans écrits | ✅ `docs/decisions.md` |

## Bilan

- **Ce qui marche** : tout le moteur de décision et de restauration, prouvé par des tests qui simulent plantages, pid réutilisés et journaux corrompus.
- **Ce qui est fragile** : tout ce qui dépend du comportement réel de tiers (formats EA et Battle.net, lien de fermeture de Big Picture, Steam Input en Bluetooth, jeux en plein écran exclusif).
- **Ce qu'on améliorerait** : intégrer PresentMon pour les mesures, ajouter HidHide pour supprimer la double saisie de Focus Input, ajouter des jaquettes par SteamGridDB avec la clé de l'utilisateur.
