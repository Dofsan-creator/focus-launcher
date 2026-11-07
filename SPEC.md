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
| D5 | Droits | L'interface tourne **sans** élévation. `focus-guardian`, service Windows optionnel installé une fois, seul à arrêter des services | Aucune fenêtre Windows à chaque « Jouer », impossible à valider à la manette |
| D6 | Récupération | Portée par `focus-guardian` (restauration si le client disparaît) **et** au démarrage de l'interface. Elle s'exécute même si le lancement automatique a été désactivé par le garde-fou | Corrige le trou v1 + v4.3 |
| D7 | Anticheats | Aucun handle ouvert sur le processus d'un jeu de boutique, ou d'un jeu où un anticheat est détecté. Priorité relevée seulement pour un jeu lancé directement sans anticheat | Étend la règle « pas d'injection » |
| D8 | Priorité | « Au-dessus de la normale » (v4.1). Jamais temps réel | Tranche v1 contre v4.1, option la plus prudente |
| D9 | Condition « état identique » | Compare **uniquement ce que le launcher a touché** | Windows démarre et arrête des choses tout seul |
| D10 | Installation des jeux | Hors périmètre (v4.4 prime sur v4). Les boutiques et applications s'installent, pas les jeux | Contradiction résolue |
| D11 | Suspension | Module présent, désactivé par défaut, jamais pour Social ni Logiciel. Mise en pause d'une entrée : v1.1 | Tranche v2, v4.1, v4.3 |
| D12 | Design | Piste **Iris** : jeux sur un arc, transitions en diaphragme, sans flou | Originale, peu coûteuse en GPU |
| D13 | Traductions | Project Fluent (équivalent ICU : pluriels, genres, paramètres), un fichier par langue | Bibliothèque Rust maintenue, Apache-2.0 |
| D14 | Windows | Windows 11 requis, Windows 10 en meilleur effort non testé | Fin de support d'octobre 2025 |
| D15 | Registre | Aucune écriture. Démarrage par raccourci dans le dossier Démarrage de l'utilisateur | Règle v1 9.3 |
| D16 | Modules actifs par défaut | Aucun module actif sans gain mesuré. Le mode simulation est imposé à la première utilisation | Règle v3 |
| D17 | Masquage de manette | Le launcher se déclare dans HidHide (par sa CLI) avant de masquer, sinon il perd lui-même la manette | Défaut v2 corrigé |

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
3. Aucune écriture dans le registre, aucune injection, aucun fichier de boutique modifié.
4. Aucun identifiant de boutique lu ou stocké.
5. Aucune chaîne d'interface en dur, aucun terme de constructeur dans les traductions.
6. Chaque écran est utilisable aux trois entrées.
7. Aucun installeur, logo tiers ni clé d'API dans le dépôt.
8. La première coupure réelle demande l'accord explicite de l'utilisateur.

## 5. Conditions d'arrêt

Les 40 conditions des specs restent valables, sauf la n° 4 (D9) et la n° 26 (suspendue). Leur état est suivi dans `PROGRESS.md`. Celles qui exigent du matériel ou un humain ne peuvent être cochées que par l'utilisateur.
