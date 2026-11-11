# Focus Launcher : spécification consolidée

Ce document **fait foi**. Il fusionne les specs v1 à v4.5 (conservées dans `docs/specs/`) et applique le passage à Rust. En cas de contradiction, ce fichier prime ; les détails non repris ici restent ceux des specs d'origine.

## 1. Produit

Un lanceur plein écran pour Windows 11, piloté à la manette, à la souris et au clavier, pour les jeux, logiciels et applications sociales de l'utilisateur. Il se comporte comme le système d'une console tout en restant une application ordinaire, désinstallable proprement. Son moteur de session optimise temporairement le PC et restaure tout ensuite.

## 2. Décisions

| # | Sujet | Décision | Raison |
| --- | --- | --- | --- |
| D1 | Langage | Rust partout, y compris la logique d'interface | Exigence de l'utilisateur |
| D2 | Interface | Tauri 2 + Leptos 0.8 (Rust compilé en WebAssembly), HTML et CSS pour le rendu | Le droite-gauche, toutes les écritures et l'accessibilité viennent de WebView2. Slint est écarté : RTL non livré (ticket #2294 ouvert) |
| D3 | Licence | GPL-3.0-or-later, conservée | Décision v2 de l'utilisateur |
| D4 | Focus Input | Écrit pour ce projet à partir des formats publics. **Aucun code DS4Windows**, y compris traduit | DS4Windows est en C#. Écriture propre, plus maintenable |
| D5 | Droits | **Révisé le 2026-11-09 par l'utilisateur** : l'interface tourne **en administrateur** (manifeste `requireAdministrator`), pour fermer les jeux qui tournent en administrateur et passer devant eux. Seuls les programmes qui exigent ces droits en héritent ; tout le reste (boutiques, navigateur, jeux ordinaires, applications relancées) est lancé avec les droits normaux par l'Explorateur. Ouverture à la session par une tâche planifiée « privilèges les plus élevés », sans invite. `focus-guardian` reste le seul à arrêter des services | Une seule invite Windows à l'ouverture manuelle de Focus, aucune à chaque « Jouer » |
| D6 | Récupération | Portée par `focus-guardian` (restauration si le client disparaît) **et** au démarrage de l'interface. Elle s'exécute même si le lancement automatique a été désactivé par le garde-fou | Corrige le trou v1 + v4.3 |
| D7 | Anticheats | Aucun handle ouvert sur le processus d'un jeu de boutique, ou d'un jeu où un anticheat est détecté. Priorité relevée seulement pour un jeu lancé directement sans anticheat | Étend la règle « pas d'injection » |
| D8 | Priorité | « Au-dessus de la normale » (v4.1). Jamais temps réel | Tranche v1 contre v4.1, option la plus prudente |
| D9 | Condition « état identique » | Compare **uniquement ce que le launcher a touché** | Windows démarre et arrête des choses tout seul |
| D10 | Installation des jeux | Hors périmètre (v4.4 prime sur v4). Les boutiques et applications s'installent, pas les jeux | Contradiction résolue |
| D11 | Suspension | Module présent, désactivé par défaut, jamais pour Social ni Logiciel. Mise en pause d'une entrée : v1.1 | Tranche v2, v4.1, v4.3 |
| D12 | Design | Piste **Iris** : jeux sur un arc, transitions en diaphragme, sans flou | Originale, peu coûteuse en GPU |
| D13 | Traductions | Project Fluent (équivalent ICU : pluriels, genres, paramètres), un fichier par langue | Bibliothèque Rust maintenue, Apache-2.0 |
| D14 | Windows | Windows 11 requis, Windows 10 en meilleur effort non testé | Fin de support d'octobre 2025 |
| D15 | Registre | Modifié le 2026-11-08 avec l’accord de l’utilisateur : seulement des valeurs du compte (HKCU) pour le jeu, notées avant d’être changées et rendues à la sortie du mode console. Aucune écriture HKLM. Démarrage par raccourci dans le dossier Démarrage | Demande explicite de l’utilisateur ; prudence v1 9.3 conservée pour le système |
| D16 | Modules actifs par défaut | Aucun module actif sans gain mesuré. Le mode simulation est imposé à la première utilisation | Règle v3 |
| D17 | Masquage de manette | Le launcher se déclare dans HidHide (par sa CLI) avant de masquer, sinon il perd lui-même la manette | Défaut v2 corrigé |

| D18 | Bureau de Windows | Décidé le 2026-11-11 par l'utilisateur : en mode console, l'Explorateur est **arrêté** tant que Focus est ouvert, puis relancé (sortie, sentinelle, démarrage suivant). Jamais remplacé : aucun réglage de Windows ne désigne Focus comme bureau. Sans bureau, Focus lance les programmes avec le jeton d'un programme ordinaire de la session | Supprimer toute interaction avec le bureau sans rien de permanent (décision 23) |
| D19 | Balayage (niveau Maximal) | Tout programme du compte qui n'est ni protégé ni visé par une règle est ralenti s'il n'a pas de fenêtre, proposé à la fermeture s'il en a une. Fermeture douce seulement, après accord. Jamais un jeu de la bibliothèque, une boutique, un outil de manette ou de ventilation | « Supprimer tout ce qui est inutile en arrière » sans risque pour le travail en cours |
| D20 | Public | Quelqu'un qui vient d'une console et ne connaît rien au PC : mots simples, bons réglages par défaut, une question à la fois | Exigence de l'utilisateur du 2026-11-11 |

## 3. Architecture

| Crate | Rôle | Dépend de |
| --- | --- | --- |
| `focus-core` | Modèles, liste blanche, règles, plan, journal, session, récupération. Pur, testable, compile en WebAssembly | rien de Windows |
| `focus-library` | Scanners de boutiques, score d'exécutable, base SQLite, catalogue | core |
| `focus-input` | Actions, répétition, combos, parseur DualShock 4, XInput, modes de manette | core |
| `focus-sys` | Implémentation Windows : processus, services, alimentation, EcoQoS, audio, raccourcis | core |
| `focus-guardian` | Service Windows élevé, canal nommé, liste compilée, restauration | core, sys |
| `focus-app` | Application Tauri : commandes, état, fenêtre | tous |
| `focus-ui` | Interface Leptos (WebAssembly) | core |

## 4. Règles non négociables

1. Tout ce qui est coupé est journalisé **avant** la coupure.
2. La liste blanche est compilée. Aucun fichier ne peut la contourner.
3. Registre : seulement les valeurs du compte listées en D15, journalisées et rendues. Aucune injection, aucun fichier de boutique modifié.
4. Aucun identifiant de boutique lu ou stocké.
5. Aucune chaîne d'interface en dur, aucun terme de constructeur dans les traductions.
6. Chaque écran est utilisable aux trois entrées.
7. Aucun installeur, logo tiers ni clé d'API dans le dépôt.
8. La première coupure réelle demande l'accord explicite de l'utilisateur.

## 5. Conditions d'arrêt

Les 40 conditions des specs restent valables, sauf la n° 4 (D9) et la n° 26 (suspendue). Leur état est suivi dans `PROGRESS.md`. Celles qui exigent du matériel ou un humain ne peuvent être cochées que par l'utilisateur.
