# Journal des décisions

Une entrée par choix structurant (v4.5). Format : contexte, options, choix, écarté, preuve.

## 1. Cadre d'interface

- **Contexte** : Rust imposé, droite-gauche et toutes les écritures exigés, fluidité et faible consommation pendant un jeu.
- **Options** : Slint (100 % Rust natif), Tauri 2 + Leptos (Rust en WebAssembly, rendu WebView2), egui, wgpu fait main.
- **Choix** : Tauri 2 + Leptos.
- **Écarté** : Slint, son ticket RTL #2294 est toujours ouvert (vérifié avec agent-reach). egui : pas de mise en forme bidirectionnelle complète ni d'accessibilité aboutie. wgpu fait main : il faudrait réécrire saisie, texte et accessibilité.
- **Preuve** : WebView2 157 présent sur la machine. Rendu stoppé pendant un jeu en cachant la fenêtre.

## 2. Droits administrateur

- **Options** : tout en administrateur (v1) ; petit assistant élevé à chaque session (v3) ; service installé une fois.
- **Choix** : service `focus-guardian`, facultatif.
- **Écarté** : l'administrateur pour tout, car les jeux en héritent. L'assistant à chaque session affiche une fenêtre Windows impossible à valider à la manette.
- **Garde-fous** : liste compilée de 8 services, canal nommé réservé aux utilisateurs interactifs, restauration à la déconnexion du client.

## 3. Navigation aux trois entrées

- **Options** : graphe de focus déclaré par écran ; navigation spatiale sur de vrais boutons.
- **Choix** : navigation spatiale (`focus-core::spatial`) sur des `button`.
- **Pourquoi** : un seul chemin pour la manette, le clavier, la souris et les lecteurs d'écran. Le droite-gauche fonctionne sans cas spécial, le raisonnement étant géométrique.

## 4. Focus Input sans DS4Windows

- **Options** : intégrer DS4Windows (C#, GPL v3) ; le piloter ; écrire le nôtre.
- **Choix** : écrit pour le projet (`focus-input::ds4`, `focus_input`), manette virtuelle par `vigem-client` (MIT).
- **Pourquoi** : un code C# ne s'intègre pas à Rust, et une traduction resterait une œuvre dérivée.

## 5. Traductions

- **Options** : `.resx` (spec v3, .NET), ICU MessageFormat, Project Fluent.
- **Choix** : Fluent. Pluriels et paramètres comme ICU, bibliothèque Rust maintenue sous Apache-2.0 (vérifié).

## 6. Suivi du jeu sans toucher au jeu

- **Options** : ouvrir le processus du jeu (priorité, état) ; suivre par nom d'exécutable.
- **Choix** : suivi par nom (`scan::process_names`, sans aucun handle). Priorité relevée seulement pour un jeu lancé directement, sans anticheat détecté.

## 7. Design

- **Options** : pistes Lentille, Iris, Pellicule (v4).
- **Choix** : Iris. Jeux sur un arc, diaphragme à 8 lames autour de la sélection, lancement par fermeture de l'iris.
- **Pourquoi** : la plus éloignée des rangées droites des consoles, et sans flou (coût GPU nul en arrière-plan).

## 8. Interface qui ne démarre pas (écran noir)

- **Contexte** : la version publiée restait noire (AUDIT n° 18). Le processus vivait, le journal était vide, et une personne à la manette ne pouvait pas sortir d'une fenêtre plein écran sans bordure. Avec le lancement automatique, ce serait à chaque ouverture de session.
- **Options** : journaliser seulement ; n'afficher la fenêtre qu'une fois l'interface prête ; journaliser, compter le démarrage comme raté et fermer la version publiée après 60 s (option absente des specs) ; message système natif.
- **Choix** : la troisième. Le premier appel `boot_info` de l'interface sert de preuve, sans nouvelle commande. Le compteur du garde-fou n'est effacé qu'une fois l'interface démarrée : deux écrans noirs de suite coupent le lancement automatique, comme deux plantages (v4.3).
- **Écarté** : journaliser seulement laisse la personne bloquée. Cacher la fenêtre transforme l'écran noir en processus invisible. Le message natif serait une chaîne hors Fluent, impossible à traduire depuis le cœur. En développement, la fenêtre reste ouverte pour les outils de débogage.
- **Preuve** : 5 tests dans `boot_guard`, test `crates/focus-app/tests/csp.rs` rouge sur l'ancienne politique puis vert.

## 9. Design « Lumen » (remplace Iris, à la demande de l’utilisateur)

- **Contexte** : l’utilisateur n’aime pas Iris et demande de s’inspirer fortement de la PS5. La règle v4.2 reste : comportements oui, apparence non.
- **Options** : garder Iris ; copier la disposition PS5 (écarté : règle v4.2 et droit des marques) ; « Lumen », comportements PS5 avec une mise en scène propre.
- **Choix** : Lumen. Repris : l’image du jeu remplit l’écran, une seule rangée, la sélection grandit, lancement en un appui, panneau rapide qui monte du bas, fondus. Différent : rangée de jaquettes verticales en bas (et non tuiles carrées en haut), logo du jeu à gauche, couleur d’accent tirée de l’image du jeu, icônes dessinées pour le projet, sons synthétisés.
- **Fluidité** : seulement `transform` et `opacity` animés, une seule piste promue (la rangée glisse d’un bloc), liste à clés (`For`) pour ne jamais recréer les jaquettes, images réduites une fois côté Rust.

## 10. Images des jeux

- **Options** : SteamGridDB (clé requise) ; protocole `asset:` sur des chemins (expose le disque) ; caches locaux des boutiques + icône de l’exécutable, servis par un protocole à clé.
- **Choix** : Steam (jaquette, fond, logo locaux), Epic (adresses de `catcache.bin`, téléchargées en HTTPS vers 3 hôtes autorisés), icône haute définition de tout exécutable par le shell de Windows. Protocole `focusart` : clé et sorte seulement, jamais un chemin.
- **Preuve** : tests `focus-core::art`, `focus-library::art`, `focus-app::art` (clés hostiles refusées).

## 11. Mode console de Windows

- **Options** : remplacer l’Explorateur (interdit v4.3) ; masquage automatique de la barre par l’Explorateur (réglage persistant, perdu si plantage) ; masquer les fenêtres de barre des tâches + session de concentration Windows + PC éveillé, le temps que Focus est ouvert.
- **Choix** : la troisième. Journal `console-state.json` écrit avant tout changement ; défait à la fermeture, par une sentinelle (second exemplaire qui attend la fin du premier) si Focus plante, ou au démarrage suivant. La session de concentration finit seule après 6 h. Aucune écriture dans le registre.

## 12. Wi-Fi et Bluetooth

- **Options** : API WLAN native (profils XML à fabriquer, risque d’injection avec SSID et mot de passe) ; ouvrir les Paramètres de Windows (sort de l’expérience console) ; WinRT `Windows.Devices.*`.
- **Choix** : WinRT, vérifié sur ce PC (accès « Allowed » sans paquet). Appels bloquants dans un fil neuf. Mot de passe jamais gardé ni journalisé.

## 13. « Toutes les optimisations »

- **Choix** : trois niveaux choisis au premier démarrage, avec accord explicite pour un niveau réel (règle 8) : Désactivées, Équilibrées (profil Concentration), Maximales (profil Maximal : 15 leviers de plus, dont la pause de Windows Update par le gardien, hautes performances et PC éveillé). Tout reste journalisé et restauré.
- **Écarté** : purge de la mémoire en attente et réglages du registre (règles 3 et D15).

## 14. Windows devient une console (demande du 2026-11-08)

- **Ajouts** : touche Windows et bouton central (PS, Xbox) = retour au launcher ; fenêtres des autres programmes réduites puis rendues ; applications lancées en plein écran sans bordure (logiciels, applications sociales, jeux lancés directement sans anticheat ; jamais un jeu de boutique) ; blocage de Big Picture vérifié chaque seconde ; boutiques déjà installées signalées.
- **Plan console** : au lancement, avec l’accord du premier démarrage, le profil choisi s’applique pour toute la durée où Focus est ouvert ; au niveau Maximal, 20 programmes d’arrière-plan sont fermés puis relancés à la sortie, 8 services mis en pause par le gardien. Journal avant chaque coupure, restauration à la sortie, par le gardien, par la sentinelle ou au démarrage suivant.
- **Écarté (registre, règle D15) mais demandé par la communauté** : désactiver Game DVR et la barre de jeu par le registre, Win32PrioritySeparation, NetworkThrottlingIndex, SystemResponsiveness, réglages `bcdedit` du minuteur, désactivation de VBS. À rouvrir seulement si l’utilisateur change la règle.
- **Bouton central Xbox** : fonction XInput non documentée (ordinal 100, bit 0x0400), utilisée par SDL et Chromium.

## 15. Expérience de console et fichier « Instructions Optimisation »

- **Chargement** : comme sur une console, l’écran de chargement (jaquette, titre, barre) reste affiché jusqu’à ce que le jeu tourne vraiment (45 s au plus), puis Focus s’efface.
- **Filtres** : Tout, Jeux, Logiciels, Stores, Social. Les boutiques installées s’ouvrent depuis Focus, les autres s’installent.
- **Registre** (accord de l’utilisateur, D15) : Game DVR, capture, transparence coupés, mode Jeu activé, le temps du mode console. Animations de Windows coupées sans écriture dans le profil.
- **Fichier d’instructions** : repris : priorité (au-dessus de la normale dans les menus, en dessous pendant un jeu), mémoire libérée (Focus au lancement d’un jeu, arrière-plan au niveau Maximal), effets visuels coupés, LTO déjà actif. Écarté : priorité « temps réel » (gèle Windows et prend les ressources du jeu), tampon de console `WriteConsoleOutputW` (Focus n’est pas un programme en mode texte), VBS (sécurité, redémarrage).

## 16. Règles d’une console appliquées à Focus

| Règle d’une console | Dans Focus |
| --- | --- |
| Le bouton central ramène toujours au système | Bouton PS ou Xbox, touche Windows : menu latéral en jeu, accueil sinon |
| Le système ne montre jamais le bureau | Mode console : barre des tâches cachée, fenêtres réduites, boutiques réduites, Big Picture fermé |
| Un écran de chargement, puis le jeu | Jaquette et barre jusqu’à l’apparition du jeu |
| Fin de partie : retour à l’accueil | Écran de fin sur la jaquette, retour seul après 6 s |
| Une partie à la fois | Lancement refusé si une session tourne déjà |
| Menu en jeu : capture, fermer, réglages | Menu latéral : revenir, capture d’écran (Images\Focus), fermer, Wi-Fi, Bluetooth, réglages, alimentation |
| Mode repos | « Mettre en veille » dans Alimentation |
| Mises à jour visibles | Jeux Steam : « Mise à jour disponible », bouton qui ouvre la boutique |
| Prérequis installés par le système | Visual C++, DirectX, .NET dans le catalogue |

Les boutiques ouvertes depuis Focus sont réduites, jamais fermées, quand on revient au launcher : un jeu qui en dépend continue.

## 17. Réactivité : rien de lourd sur le fil principal (2026-11-09)

Constat : 17 à 21 s entre « console mode on » et « interface started » au journal ; Retour sans effet en jeu ; page des paramètres qui se fige.

| Option | Pour | Contre |
| --- | --- | --- |
| A. Garder le plan avant l’affichage | Windows déjà prêt quand l’interface paraît | 20 s d’écran vide |
| B. Fenêtre d’abord, mode console et plan dans un fil à part | Fenêtre immédiate, animation pendant le plan | Les autres fenêtres se réduisent une seconde après |
| C. Plan allégé au démarrage, complet au premier jeu | Démarrage rapide | Le PC n’est pas en état console tant qu’on n’a pas joué |

Choix : B. Les commandes Tauri lourdes passent en `async` (une commande synchrone tourne sur le fil de la fenêtre et la fige). En jeu, la manette envoie toutes les actions à Focus quand sa fenêtre est affichée, plus seulement le bouton central ; Retour sur le menu latéral rend la main au jeu. La fenêtre reprend le clavier par `AttachThreadInput` + `SetForegroundWindow`, sinon Échap part au jeu.

## 18. Manettes natives et réglage du bouton central de Steam (2026-11-09)

Constat : ViGEmBus installé, HidHide absent. Pendant un jeu hors Steam, Focus Input créait une manette Xbox virtuelle à côté de la manette PlayStation réelle : le jeu voyait deux manettes. Les manettes USB, et la manette Xbox en Bluetooth, sont reconnues nativement.

| Option | Pour | Contre |
| --- | --- | --- |
| A. Natif par défaut ; Focus Input automatique seulement avec HidHide | Plus de doublon, rien à installer | Un jeu sans prise en charge PlayStation demande le choix manuel |
| B. Installer HidHide | Doublon supprimé | Pilote système, installation élevée, hors du cadre accordé |
| C. Garder Focus Input | Symboles Xbox partout | Commandes en double |

Choix : A. Le choix explicite de l’utilisateur reste respecté (`resolve_mode`).

Steam : la solution trouvée par l’utilisateur (désactiver « Le bouton Guide place Steam au premier plan ») est la clé `Controller_CheckGuideButton` de `localconfig.vdf`. Options : écrire la clé (fichier d’un tiers, réécrit par Steam tant qu’il tourne, risque de corruption) ou la lire et guider. Choix : lecture seule, avertissement dans Paramètres › Manette et bouton qui ouvre `steam://settings/controller`. Le blocage de l’autre mode plein écran de Steam reste en place en secours.

## 19. Écran de lancement jusqu'au jeu, fin de session fiable (2026-11-09)

Constats (essais de l'utilisateur, Fortnite et Wuthering Waves) : bureau visible pendant l'anticheat ou le lanceur ; pas de retour automatique après fermeture. Causes : Focus se cachait dès qu'une fenêtre du jeu apparaissait ; le suivi par noms gardait la session ouverte avec `EpicWebHelper` (lanceur Epic) et `msedgewebview2` (Windows), et ne voyait pas `Client-Win64-Shipping` (profondeur 4).

| Option | Pour | Contre |
| --- | --- | --- |
| A. Suivre les processus par chemin (dossier du jeu) | Exact | Ouvre un handle sur chaque processus, jeux compris : contraire à SPEC D7 |
| B. Noms filtrés : assistants écartés, noms déjà lancés avant le jeu écartés, profondeur 6, et exécutable « principal » retenu quand sa fenêtre couvre l'écran | Sans handle, couvre lanceurs et anticheats | Heuristique (seuil de 60 % de l'écran) |
| C. Fin de session manuelle seulement | Simple | Contraire à la demande |

Choix : B. Focus reste en fond (écran de lancement) ; les fenêtres de lanceur et d'anticheat sont ramenées devant lui ; les grandes fenêtres de boutique sont réduites ; Focus ne se cache qu'une fois le jeu à au moins 60 % de l'écran. La session finit 3 s après la fin de l'exécutable principal, même si le lanceur reste ouvert. Un seul Focus à la fois (`tauri-plugin-single-instance`) : relancer Focus ramène la fenêtre cachée.

## 20. Manette universelle (HidHide) et jeux en administrateur (2026-11-09)

Manette : Focus Input (manette Xbox virtuelle) marche dans tous les jeux, mais sans HidHide le jeu voit aussi la vraie manette. Options : A. natif seulement (manette morte dans les jeux sans prise en charge PlayStation) ; B. HidHide piloté par `HidHideCLI.exe` (sans élévation, documenté), masquage juste avant le lancement, journal `hidhide.json`, restauration en fin de session, au démarrage et par la sentinelle ; C. pilote maison (hors de portée). Choix : B, installé par l'utilisateur depuis le catalogue ; choix manuel Auto/Natif/Focus Input en plus.

Jeux en administrateur : Windows (UIPI) empêche un programme normal de fermer leur fenêtre ou de leur prendre le premier plan. Options : A. Focus toujours en administrateur (UAC à chaque ouverture, tout ce qu'il lance hérite des droits) ; B. assistant élevé lancé à la demande, une invite UAC par session, limité à fermer le jeu de la session ; C. rien (fermeture par le menu du jeu). Décision laissée à l'utilisateur ; en attendant, Focus rend la main au jeu après la demande de fermeture et écrit le refus au journal.

## 21. Focus en administrateur (2026-11-09)

Décision de l'utilisateur, entre deux options présentées : assistant élevé à la demande, ou Focus toujours en administrateur. Choix de l'utilisateur : toujours en administrateur. SPEC D5 révisée.

Mise en œuvre, pour limiter le risque relevé par la v3 (« un jeu lancé depuis un processus élevé hérite de ces droits ») :

| Point | Choix |
| --- | --- |
| Manifeste | `requireAdministrator` ajouté au manifeste par défaut de Tauri (`app.manifest`, en ASCII : un caractère accentué fait refuser l'exécutable, erreur 14001) |
| Lancements | Un exécutable dont le manifeste exige les droits est lancé directement, sans nouvelle invite. Tout le reste passe par l'Explorateur (`IShellDispatch2::ShellExecute` du bureau) et tourne avec les droits normaux, vérifié : enfant en S-1-16-8192, Focus en S-1-16-12288 |
| Ouverture à la session | Le dossier Démarrage ne lance pas un programme élevé : tâche planifiée `Focus Launcher`, à l'ouverture de session de l'utilisateur, privilèges les plus élevés, sans invite. L'ancien raccourci est retiré |
| Invite manuelle | Une invite UAC à l'ouverture manuelle de Focus ; elle ne se valide pas à la manette (v4, 6.15) |

## 22. Installeur complet pour un autre PC (2026-11-09)

| Point | Options | Choix |
| --- | --- | --- |
| Visual C++ | Redistributable dans l'installeur ; lien statique (`+crt-static`) | Lien statique (`.cargo/config.toml`) : `focus-guardian` importait `vcruntime140.dll`, absent d'un Windows neuf ; l'UCRT fait partie de Windows 10 et 11 |
| WebView2 | Amorce téléchargée (Internet requis) ; amorce intégrée ; installeur hors ligne (+130 Mo) | Hors ligne, silencieux : marche sans Internet, ne s'installe que s'il manque |
| Portée | Utilisateur courant ; tous les utilisateurs | Tous les utilisateurs (`Program Files`), cohérent avec SPEC D5 |
| Langues | Anglais seul ; les 11 langues de Focus | Les 11, avec choix au démarrage de l'installeur |
| Désinstallation | Laisser la tâche et le service ; les retirer | Crochets NSIS : arrêt du service pendant une mise à jour, retrait de la tâche `Focus Launcher` et du service à la désinstallation |
| Guardian | Compilé dans `target/release` (verrouillé par le service de ce PC) ; cible explicite | `--target x86_64-pc-windows-msvc` : dossier à part, plus de conflit avec le service installé |

Non inclus : HidHide et ViGEmBus (pilotes, redémarrage, installés depuis le catalogue par winget) ; signature de l'exécutable (SmartScreen préviendra).

## 23. Mode console v2 : bureau arrêté, balayage, profil visible (2026-11-11)

Demande de l'utilisateur : « transformer le bureau en console, supprimer tout ce qui est inutile en arrière, créer un script d'optimisation spéciale, supprimer toute interaction avec le bureau », pour quelqu'un qui vient d'une console et ne connaît rien au PC. Les trois choix ci-dessous ont été faits par lui le 2026-11-11.

| Point | Options | Choix |
| --- | --- | --- |
| Bureau de Windows | Explorateur masqué (avant) ; Explorateur arrêté tant que Focus est ouvert ; Focus désigné comme bureau au démarrage (registre Winlogon) ; compte Windows dédié | **Arrêté puis relancé.** Aucun registre, rien de permanent : un redémarrage du PC rend le bureau. Noté au journal avant l'arrêt ; relancé à la sortie, par la sentinelle si Focus plante, et au démarrage suivant. Écarté : le registre Winlogon (écran noir si Focus ne démarre pas) |
| Liste blanche | Retirer `explorer` de la liste compilée ; arrêt hors du moteur de règles | **Hors du moteur** : `explorer` reste protégé contre tout fichier de règles (règle 2). Seul le mode console l'arrête, par un chemin dédié et journalisé |
| Lancements sans bureau | Tout lancer avec les droits de Focus ; tâche planifiée ; jeton d'un programme ordinaire de la session | **Jeton** de `sihost` ou équivalent (`CreateProcessWithTokenW`), refusé s'il est élevé. L'ancienne voie par l'Explorateur sert encore quand il tourne. Le bureau lui-même est relancé ainsi, donc sans droits administrateur |
| « Script d'optimisation » | Script PowerShell permanent ; profil intégré en Rust | **Profil intégré** : journalisé, rendu à la sortie, conforme à « Rust partout » |
| Arrière-plan (niveau Maximal) | Liste connue seulement ; tout fermer ; balayage | **Balayage** : tout programme du compte sans fenêtre, hors dossier de Windows, est ralenti (mode efficacité, priorité basse), jamais fermé. Les applications à fenêtre sont proposées à la fermeture, en une seule question |
| Fermeture des applications | Forcée ; douce | **Douce, jamais forcée** : une application qui demande d'enregistrer reste ouverte. C'est pourquoi « Les fermer » est le choix par défaut du dialogue. Pas de relance à la sortie : l'utilisateur a choisi de les fermer |
| Exclusions du balayage | — | Liste blanche, boutiques, applications sociales gardées, logiciels de création, outils de manette et de ventilation (`is_device_process`), et tout programme situé dans le dossier d'un jeu de la bibliothèque (aucun handle sur un jeu ni son anticheat, SPEC D7) |
| Voir le changement | Message au journal ; pastille et annonce | **Pastille** sur l'accueil, **annonce** qui descend avec ce qui a changé, **écran** avant et maintenant (mémoire libre, puissance). Mots simples, aucun terme technique |
| Musique | Nappe fixe de quatre sinusoïdes (avant) ; musique générée | Accords tenus qui se fondent, notes de cloche espacées, gamme pentatonique sans dissonance, réverbération. Les sons de navigation sont dans la même gamme |

Défauts corrigés au passage, lus dans le journal du 2026-11-10 : 20 s perdues à chaque ouverture à attendre la fermeture de Game Bar (processus sans fenêtre) ; relance de Game Bar refusée, donc journal jamais effacé et « recovery incomplete » à chaque démarrage ; un jeu lancé pendant le passage en mode console appliquait un second plan par-dessus ; les programmes relancés à la sortie héritaient des droits administrateur de Focus.

Preuve : 179 tests verts, lint sans avertissement (cœur, Windows, application, interface WebAssembly). **Non prouvé sur une vraie machine** : l'arrêt et la relance de l'Explorateur, le lancement par jeton, l'annonce à l'écran et la musique n'ont pas encore été vus ni entendus dans l'exécutable publié.

## 24. Dépôt public, mise à jour, signalement, téléchargements (2026-11-12)

| Point | Options | Choix |
| --- | --- | --- |
| Dépôt | Privé ; public | **Public** (`Dofsan-creator/focus-launcher`), choisi par l'utilisateur : un dépôt privé exigerait une clé d'accès dans le logiciel (règle 7). Les adresses personnelles des commits ont été remplacées par l'adresse anonyme de GitHub avant le premier envoi |
| Mise à jour | Ouvrir la page des versions ; téléchargement maison vérifié par empreinte ; module de mise à jour de Tauri, signé | **Module Tauri signé** : l'installeur doit porter la signature de la clé de Focus, sinon il est refusé, même si le compte GitHub était compromis. Clé privée hors dépôt (`~/.tauri/focus-launcher.key`). Avant d'installer, Focus rend à Windows son état normal |
| Signalement | Envoi direct par l'API (clé interdite) ; service relais ; fiche GitHub préremplie | **Fiche préremplie** ouverte dans le navigateur : aucune clé, rien n'est envoyé sans que l'utilisateur relise. Le rapport (version, Windows, 12 derniers avertissements) est nettoyé du nom et du dossier de l'utilisateur et gardé dans `reports/`. Limite : il faut un compte GitHub |
| Téléchargements | Fenêtre de chaque boutique ; lecture sur le disque | **Steam lu sur le disque** (`appmanifest_*.acf`, sans clé ni réseau), installations lancées depuis Focus, mise à jour de Focus. Epic et les autres n'écrivent pas leur progression de façon lisible : un bouton ouvre leur page |

Non prouvé : aucune version n'est encore publiée, donc la mise à jour n'a jamais été reçue ni installée ; l'ouverture de la fiche dans le navigateur n'a pas été vue, bureau arrêté.

## 25. Leçons du premier essai réel du mode console v2 (2026-11-12)

Essai fait avec l'utilisateur sur l'exécutable publié. Vu au journal : bureau arrêté, lancement de Steam par jeton sans bureau, puissance « performances élevées », 30 programmes ralentis, 4 services en pause, plan appliqué en 3,5 s au second passage.

| Constat | Options | Choix |
| --- | --- | --- |
| Manette restée masquée après un arrêt brutal du PC (HidHide actif, journal jamais rendu) | Rendre au démarrage de Windows par le guardian ; ne plus masquer tout seul | **« Auto » ne choisit plus Focus Input** : la manette reste native, Focus Input se demande dans les paramètres. Une manette invisible est pire qu'une manette mal reconnue par un vieux jeu |
| Sélecteur tantôt lent, tantôt trop rapide | Garder ; ralentir ; ralentir et filtrer | Répétition à 120 ms (au lieu de 65), accélérée à 75 ms après 8 pas ; un même geste reçu de la manette **et** du clavier en moins de 160 ms ne compte qu'une fois (Steam convertit la croix en touches fléchées) |
| Changement de profil peu visible | Annonce seule ; effet plein écran | **Effet plein écran** (couleur propre au profil, trait de lumière, nom en grand, trois notes), trait de couleur permanent en haut de l'écran, et le changement de niveau s'applique tout de suite au lieu d'attendre un redémarrage de Focus |
| Bouton central capté par Steam | Modifier le réglage de Steam (interdit, règle 3) ; tutoriel | **Tutoriel en trois étapes** ouvert une fois sur l'accueil, avec un bouton vers le bon réglage de Steam et un bouton qui vérifie |
| Terminal pris pour le jeu | — | Un programme qui tournait avant le lancement n'est jamais pris pour le jeu |
| Journal bloqué par des programmes que Windows refuse d'ouvrir | — | Rien à rendre à un programme qu'on n'a pas pu ralentir |
| Installeur | Aspect par défaut ; images de Focus | Bandeau et panneau latéral dessinés pour Focus, icône de Focus |

## 26. Tests dans le navigateur (2026-11-12)

Méthode : l'interface servie avec ses données de démonstration, pilotée au clavier dans une fenêtre Edge par `agent-desktop` (arbre d'accessibilité de Windows), puis par le protocole de débogage d'un Edge sans fenêtre pour lire la console et capturer chaque étape. La démonstration suit désormais le niveau choisi, et `?accueil` dans l'adresse saute l'assistant.

| Constat | Correction |
| --- | --- |
| Dans l'assistant, la flèche du bas allait sur « Retour » au lieu de « Suivant » : un appui de trop faisait reculer | En changeant de rangée, la navigation va sur le bouton principal de la rangée |
| Le bouton « Jouer » n'avait pas d'anneau de focus (son halo l'écrasait) | Anneau ajouté au halo |
| Blocs collés sur les écrans Profil, Téléchargements, Aide | Espacement commun des écrans |
| Au démarrage, l'annonce du profil et la question sur les applications se chevauchaient, puis le tutoriel Steam s'enchaînait | Une chose à la fois : la question attend la fin de l'annonce ; le tutoriel attend l'accueil et ne revient qu'une fois par semaine |
| Textes de l'ancien mode console (confirmation des performances, assistant, paramètres) | Réécrits dans les 11 langues ; l'interrupteur « Masquer complètement le bureau » ajouté à l'assistant |
| Les applications ouvertes s'affichaient sous leur nom technique (« chrome · notepad ») | Nom lu dans la description de l'exécutable (« Google Chrome », « Bloc-notes »), sans ouvrir le processus |

Vérifié sans plantage ni erreur de console : assistant complet, accueil, rangée, pastille et écran du profil, question des applications, Téléchargements, Aide, effet plein écran du changement de profil.

## 27. WebView2 n'est plus embarqué dans l'installeur (2026-11-12)

| Point | Options | Choix |
| --- | --- | --- |
| WebView2 | Installeur hors ligne embarqué (décision 22, +210 Mo) ; amorce embarquée ; amorce téléchargée à l'installation | **Téléchargée seulement s'il manque.** Windows 11, exigé par Focus (SPEC D14), contient déjà WebView2 : rien n'est téléchargé dans le cas normal. L'installeur passe d'environ 212 Mo à quelques Mo, et chaque mise à jour depuis Focus aussi |
| Déclencheur | — | Le DNS de la box de l'utilisateur ne résout pas `msedge.sf.dl.delivery.mp.microsoft.com`, que Tauri interroge pour embarquer WebView2 : l'installeur ne pouvait plus être construit sur ce PC |
| Ce qu'on perd | — | Un Windows 11 sans WebView2 (supprimé à la main) et sans Internet ne pourra pas installer Focus. Cas jugé marginal |

## 28. Souris à la manette (2026-11-12)

| Point | Options | Choix |
| --- | --- | --- |
| Besoin | — | Les lanceurs de jeux, les fenêtres de connexion et les boutiques attendent souvent un clic : sans bureau ni souris, un joueur à la manette restait bloqué |
| Déclenchement | Combinaison de boutons cachée ; tuile du panneau rapide ; automatique | **Tuile du panneau rapide**, et **automatique** pendant l'écran de lancement quand Focus fait passer devant une fenêtre de lanceur. Éteinte dès que le jeu occupe l'écran : la manette n'agit jamais sur le curseur en pleine partie |
| Commandes | — | Stick gauche : curseur (zone morte, courbe au carré pour la précision) ; bouton du bas : clic ; droite : clic droit ; haut : clavier à l'écran de Windows ; stick droit : défilement ; bouton central : retour à Focus. Les noms sont donnés par position, pas par lettre ni par forme, pour toutes les manettes |
| Envoi | Pilote de souris virtuelle ; `SendInput` | **`SendInput`** : rien à installer ; Focus étant administrateur, il atteint aussi les lanceurs qui le sont |
| Clavier | Clavier de Focus ; clavier de Windows | **Clavier de Windows** (`osk.exe`, ouvert par le shell) : il écrit dans n'importe quelle fenêtre |
| Sécurité | — | Un clic tenu est toujours relâché quand le mode s'éteint |

Logique pure dans `focus-input::pointer` (7 tests), envoi dans `focus-sys::pointer`. Non essayé avec une vraie manette.

## 29. Interface pour le canapé, volume général (2026-11-12)

| Point | Choix |
| --- | --- |
| Démarrage | Animation de 5,5 s ramenée à 2,6 s |
| Taille du texte | Texte de base porté à 1,25 % de la largeur (24 px sur un écran 1920, au lieu de 20), pour une télé vue de loin |
| Écran « Préparation » | Retiré : il listait des programmes et disait « À toi de choisir » sans offrir de choix. « Jouer » lance directement ; les applications à fermer sont proposées par une seule question à l'ouverture de Focus |
| Volume | Ligne de volume général dans le panneau rapide, par pas de 10 % (`IAudioEndpointVolume`, documenté), qui rétablit le son s'il était coupé |

Vérifié dans le navigateur : panneau rapide, volume de 60 % à 70 %, texte agrandi. Le choix de la sortie son (télé, casque) reste à faire : Windows ne le propose pas par une interface documentée.

## 30. Jaquettes sans clé par le magasin public de Steam (2026-11-12)

| Point | Options | Choix |
| --- | --- | --- |
| Source | SteamGridDB (clé personnelle) ; IGDB (compte) ; magasin public de Steam | **Magasin public de Steam** : recherche par titre, puis chemins d'images par son service public. Aucune clé, aucun compte |
| Pour qui | — | Tout jeu sans jaquette : ajouté à la main, autre boutique, ou jeu Steam dont le cache local est incomplet |
| Justesse | Titre approché ; titre exact | **Titre exact** (lettres et chiffres seulement, « Battlefield™ 6 » = « Battlefield 6 ») : pas de jaquette plutôt qu'une jaquette d'un autre jeu |
| Chemins | Ancien chemin fixe ; chemin versionné | **Versionné** : pour les jeux récents, l'ancien chemin renvoie une image vide (vu pour Battlefield 6 et Wuthering Waves) |
| Vie privée | — | Seul le titre du jeu part vers Steam, en HTTPS, vers trois hôtes listés dans le code |

Vérifié en vrai : la jaquette de Wuthering Waves est trouvée par son titre et téléchargée (test ponctuel `steam_store`). Le cache des images est recalculé une fois (version 3).

## Propositions de changement (à trancher par l'utilisateur)

1. Livrer d'abord les 11 langues actuelles, puis les autres par la communauté (v4.4 demande toutes les langues de Windows avant la sortie).
2. Remplacer Tab sur l'accueil (changement de filtre, v4.3) par une touche dédiée, pour garder Tab à la navigation standard des lecteurs d'écran.
