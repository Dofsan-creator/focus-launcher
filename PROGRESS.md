# Avancement

Mis à jour le 2026-11-08. Skills utilisés : `find-skills` (recherche et installation des skills), `agent-reach` (vérification des dépôts et licences sur GitHub et le web), `rust-best-practices`, `tauri-v2`.

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
| 10. Lumen, mode console, connexions | Terminé (code), vérifié en partie | Design Lumen, images des jeux (Steam, Epic, icônes), mode console avec sentinelle, Wi-Fi et Bluetooth, assistant en 8 étapes, 3 niveaux d’optimisation. Vu sur ce PC le 2026-11-08 : jaquettes et fond affichés, barre des tâches masquée puis rendue après un plantage simulé |
| 9. Livraison | En cours | Installeur NSIS par `cargo tauri build`. Écran noir de la version publiée corrigé le 2026-11-08 (AUDIT n° 18). Signature non faite |

## Vérifications lancées (résultats réels)

- `cargo test --workspace --exclude focus-ui` : **166 réussis, 0 échec** (2026-11-08, après Lumen).
- `crates/focus-app/tests/csp.rs` relancé sur l'ancienne politique : échoue, comme attendu. Le test attrape bien l'écran noir.
- Exécutable publié lancé sur ce PC le 2026-11-08 : interface affichée (capture d'écran), `interface started` au journal, compteur du garde-fou effacé après 30 s.
- `cargo clippy --workspace --exclude focus-ui --all-targets -- -D warnings` : 0 avertissement.
- `cargo clippy -p focus-ui --target wasm32-unknown-unknown -- -D warnings` : 0 avertissement.
- `cargo test -p focus-core --test locales` : 11 langues complètes, aucun terme interdit.

## Conditions d'arrêt

Légende : ✅ prouvé par test ou commande, 🧑 à faire par toi (matériel ou jugement humain), ⚠️ ouvert.

| # | Condition | État |
| --- | --- | --- |
| 1 | Compile sans avertissement bloquant | ✅ |
| 2 | Tous les tests passent | ✅ 166/166 |
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
| 17 | Jeux détectés sans action, au démarrage à froid | ✅ sur ce PC le 2026-11-08 : Yu-Gi-Oh! Master Duel (Steam), Fortnite et Delta Force (Epic), visibles sur l'accueil de l'exécutable publié (capture). La vérification précédente (« lancé 10 s, journal sans erreur ») n'avait pas vu l'écran noir (AUDIT n° 18) |
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

## Mode console v2 (2026-11-11, décision 23)

Écrit et testé hors machine réelle : bureau arrêté puis relancé, lancement par jeton, balayage du niveau Maximal, question unique sur les applications ouvertes, pastille et écran du profil, musique générée, 27 textes dans les 11 langues. 179 tests, lint propre. Reste à voir dans l'exécutable publié (AUDIT n° 45 et 47).

## Bilan

- **Ce qui marche** : tout le moteur de décision et de restauration, prouvé par des tests qui simulent plantages, pid réutilisés et journaux corrompus.
- **Ce qui est fragile** : tout ce qui dépend du comportement réel de tiers (formats EA et Battle.net, lien de fermeture de Big Picture, Steam Input en Bluetooth, jeux en plein écran exclusif). Et la version publiée, qui ne se comporte pas comme `cargo tauri dev` ni comme le navigateur (politique de sécurité) : un installeur n'est prêt qu'une fois lancé et vu à l'écran, avec `interface started` au journal.
- **Ce qu'on améliorerait** : intégrer PresentMon pour les mesures, ajouter HidHide pour supprimer la double saisie de Focus Input, ajouter des jaquettes par SteamGridDB avec la clé de l'utilisateur.
