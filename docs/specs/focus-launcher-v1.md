# Focus Launcher : lanceur de jeux plein écran avec mode concentration

## 1. Vision du projet

Focus Launcher est un logiciel Windows qui combine deux choses :

1. **Un lanceur plein écran façon console** (inspiré de PlayStation 5, Xbox Series X et Steam Big Picture), navigable à la manette, avec une bibliothèque de jeux.
2. **Un moteur d'optimisation** qui, au clic sur « Jouer », coupe temporairement les processus et services non essentiels, lance le jeu en priorité haute, puis remet tout en place à la fermeture du jeu.

Ce n'est pas un émulateur, et ce n'est pas seulement un optimiseur : c'est une expérience de console immersive sur PC. Focus Launcher se comporte comme le système d'une console (plein écran, manette, ambiance sonore, installation des stores, bibliothèque unifiée) tout en restant une application Windows ordinaire. L'optimisation de session est l'une de ses fonctions, au service de l'immersion (voir la section 6.5).

### Objectifs

- Une interface plein écran, fluide, utilisable entièrement à la manette.
- Une bibliothèque unifiée qui charge les jeux de Steam, Epic Games, EA App, GOG, Ubisoft Connect et Battle.net, avec ajout manuel possible (sections 5 et 6.7).
- Un mode concentration sûr : ne jamais casser Windows, toujours pouvoir tout restaurer.
- Une analyse automatique du système pour proposer ce qui peut être coupé.

* Une installation des clients de stores depuis le launcher, avec stores qui tournent en arrière-plan réduit (section 6.6).
* Une immersion totale : démarrage façon console, ambiance sonore, détection de la manette PlayStation ou Xbox avec affichage direct des bonnes icônes (section 6.5).
* Un blocage de Big Picture de Steam tant que Focus Launcher est actif (section 6.5).

### Hors périmètre (pour la version 1)

- Émulation de consoles.
- Modification du registre ou du noyau.
- Overclocking ou réglages matériels.

## 2. Choix techniques

| Élément | Choix | Raison |
| --- | --- | --- |
| Langage | C# | Excellent accès aux processus et services Windows |
| Plateforme | .NET 8 (LTS) | Moderne, performant, bien supporté |
| Interface | WPF ou WinUI 3 | Plein écran, animations, support manette |
| Manette | Bibliothèque XInput (ex. SharpDX.XInput ou Vortice.XInput) | Support natif des manettes Xbox et compatibles |
| Stockage | SQLite ou fichiers JSON | Bibliothèque de jeux et profils |
| Journalisation | Serilog | Traçabilité de tout ce qui est coupé et restauré |
| Droits | Manifeste administrateur | Nécessaire pour arrêter certains services |

## 3. Architecture générale

Le logiciel est découpé en quatre projets principaux (plus deux projets de tests) dans une même solution Visual Studio :

- **FocusLauncher.Core** : modèles, interfaces et logique métier, sans dépendance à l'interface.
- **FocusLauncher.Optimizer** : le moteur qui analyse et coupe les processus et services.
- **FocusLauncher.Library** : gestion de la bibliothèque de jeux et du lancement.
- **FocusLauncher.UI** : l'interface plein écran et la gestion de la manette.

### Flux principal

1. L'utilisateur sélectionne un jeu dans le menu et appuie sur « Jouer ».
2. L'Optimizer prend un **instantané** de l'état du système (processus et services en cours).
3. Il compare avec la liste blanche (jamais touché) et la liste de coupure (peut être coupé).
4. Il arrête les éléments autorisés et **enregistre exactement ce qu'il a arrêté**.
5. Le jeu est lancé, sa priorité est montée à haute.
6. Le logiciel surveille le processus du jeu.
7. À la fermeture du jeu, l'Optimizer **restaure** tout ce qu'il avait arrêté, dans l'ordre inverse.
8. En cas de plantage du lanceur, une restauration automatique se fait au prochain démarrage grâce au journal de session.

## 4. Le moteur d'optimisation (FocusLauncher.Optimizer)

Le moteur fait trois choses : analyser, couper, restaurer. Il ne doit jamais couper quelque chose sans l'avoir enregistré avant.

### 4.1 Analyse du système

Au premier lancement, et à la demande, le moteur scanne la machine et produit un rapport :

- Liste des processus en cours, avec mémoire et processeur utilisés (classe `System.Diagnostics.Process`).
- Liste des services Windows et leur état (classe `System.ServiceProcess.ServiceController`).
- Programmes lancés au démarrage de Windows.
- Configuration matérielle de base : processeur, mémoire, carte graphique (requêtes WMI).
- Schéma d'alimentation actif.

Le rapport classe chaque élément dans une de ces catégories :

| Catégorie | Signification | Action |
| --- | --- | --- |
| Protégé | Système Windows, pilotes, sécurité, le lanceur lui-même | Jamais touché |
| Coupable sans risque | Mises à jour d'applications, clients de chat, synchronisation cloud, lanceurs inutiles | Arrêté pendant le jeu |
| Incertain | Programme inconnu | Proposé à l'utilisateur, jamais coupé automatiquement |

### 4.2 Liste blanche (jamais touché)

Le moteur refuse de couper, quoi qu'il arrive :

- Tous les processus système : `System`, `smss`, `csrss`, `wininit`, `winlogon`, `services`, `lsass`, `svchost` (jamais arrêté : un seul processus svchost héberge souvent plusieurs services, donc la coupure se fait au niveau du service et jamais en tuant le processus), `explorer`, `dwm`.
- Les services critiques : réseau de base, audio Windows, gestionnaire de périphériques, Windows Defender (on ne le désactive pas, par sécurité).
- Les pilotes et logiciels de carte graphique, de manette et de périphériques.
- Les clients de jeu nécessaires au jeu choisi (Steam, Epic, EA App, etc.), détectés selon l'origine du jeu.
- Le lanceur lui-même et ses composants.

### 4.3 Liste de coupure (modifiable par l'utilisateur)

Stockée dans un fichier `optimizer-rules.json`, avec des profils. Exemples de cibles courantes :

- Navigateurs web ouverts en arrière-plan (après avertissement, pour éviter la perte de données).
- Clients de messagerie et de chat (Discord peut être conservé par un réglage).
- Synchronisation cloud (OneDrive, Dropbox, Google Drive).
- Mises à jour automatiques d'applications tierces.
- Services Windows non essentiels pendant le jeu : indexation de recherche, télémétrie, Windows Update (arrêt temporaire du service uniquement, jamais de désactivation durable ni de modification de ses réglages), impression en arrière-plan si aucune imprimante n'est utilisée.

Chaque règle contient : nom du processus ou service, type, raison, niveau de risque, et un indicateur « actif par défaut ».

### 4.4 Coupure et restauration

Principe de sécurité absolu : **tout ce qui est coupé est inscrit dans un journal de session avant la coupure.**

1. Créer le fichier `session.lock.json` avec la liste prévue.
2. Pour chaque service : lire son état de démarrage et son état actuel, puis l'arrêter (`ServiceController.Stop`).
3. Pour chaque processus : tenter une fermeture propre (`CloseMainWindow`), puis `Kill` seulement après un délai et si la règle l'autorise.
4. Lancer le jeu, puis augmenter sa priorité (`ProcessPriorityClass.High`, jamais `RealTime`).
5. À la fermeture du jeu : relancer les services arrêtés, puis les processus qui doivent être relancés (chemin et arguments enregistrés).
6. Supprimer `session.lock.json` quand tout est restauré.
7. Au démarrage du lanceur : si `session.lock.json` existe, c'est qu'il y a eu un plantage, donc **restauration automatique immédiate**.

### 4.5 Options supplémentaires du mode concentration

- Activer le Mode Jeu de Windows et l'assistant de concentration (ne pas déranger).
- Passer temporairement au schéma d'alimentation « Performances élevées », puis restaurer l'ancien.
- Vider la mémoire en attente (option avancée, désactivée par défaut).
- Affecter les cœurs du processeur au jeu (`ProcessorAffinity`, option avancée).
- Mode « simulation » : affiche ce qui serait coupé sans rien faire. **Obligatoire pour la première utilisation.**

## 5. La bibliothèque de jeux (FocusLauncher.Library)

### 5.1 Modèle de données

Chaque jeu est représenté par une fiche contenant :

- Un identifiant unique, un titre, une jaquette (image locale ou téléchargée).
- Le type de source : exécutable local, Steam, Epic Games, GOG, EA App, Ubisoft Connect, ou autre.
- Le chemin de l'exécutable ou l'identifiant du jeu sur sa plateforme.
- Les arguments de lancement éventuels.
- Le profil d'optimisation à utiliser (par défaut ou personnalisé).
- Les statistiques : temps de jeu total, dernière session, nombre de lancements.

### 5.2 Ajout des jeux

- **Ajout manuel** : l'utilisateur choisit un `.exe` ou un raccourci.
- **Détection automatique** : lecture des dossiers de bibliothèques Steam (fichiers `libraryfolders.vdf` et `appmanifest_*.acf`), des manifestes Epic Games (dossier `ProgramData\Epic\EpicGamesLauncher\Data\Manifests`), et du registre pour GOG.
- **Jaquettes** : récupération via une API publique comme SteamGridDB ou IGDB (clé d'API à prévoir).

### 5.3 Lancement d'un jeu

- Jeu local : `Process.Start` avec le chemin et les arguments.
- Jeu Steam : protocole `steam://rungameid/<id>`.
- Jeu Epic : protocole `com.epicgames.launcher://apps/<id>?action=launch&silent=true`.
- Le lanceur retrouve ensuite le processus réel du jeu (certains lanceurs démarrent un processus intermédiaire) en surveillant les nouveaux processus dans le dossier d'installation.
- Le temps de jeu est compté tant que ce processus existe.

## 6. L'interface plein écran (FocusLauncher.UI)

### 6.1 Principes

- Plein écran sans bordure, démarrage possible en même temps que Windows.
- Navigation 100 % manette : croix directionnelle ou joystick pour se déplacer, A pour valider, B pour revenir, Y pour les options, Start pour le menu général.
- Clavier et souris en secours.
- Animations fluides, sons de navigation discrets.

### 6.2 Écrans

1. **Accueil** : ligne horizontale de jaquettes façon PlayStation, avec les derniers jeux joués en premier et un fond qui change selon le jeu sélectionné.
2. **Fiche du jeu** : jaquette, temps de jeu, bouton Jouer, bouton Options.
3. **Préparation** : écran de transition affichant « Optimisation en cours » avec le détail de ce qui est coupé.
4. **En jeu** : le lanceur reste discret en arrière-plan et surveille le jeu.
5. **Retour** : écran de fin de session avec le temps joué et la confirmation que tout est restauré.
6. **Paramètres** : règles d'optimisation, profils, dossiers à scanner, manette, démarrage automatique.
7. **Analyse du système** : le rapport du moteur d'optimisation, avec les catégories et les cases à cocher.

### 6.3 Gestion de la manette

- Lecture de l'état de la manette via XInput (manettes Xbox et compatibles), avec une boucle d'interrogation d'environ 60 fois par seconde.
- Une couche d'abstraction transforme les entrées en actions (Valider, Retour, Haut, Bas, Gauche, Droite, Options), pour que l'interface ne dépende pas du matériel.
- Les manettes PlayStation, Switch et génériques sont prises en charge via Steam Input (voir section 6.3 bis).

### 6.3 bis. Intégration de Steam Input (modèle de manette de Steam)

**Clarification importante :** XInput est l'interface de Microsoft pour les manettes Xbox, déjà prévue en section 6.3. Ce que Steam apporte en plus s'appelle **Steam Input** : une couche de traduction qui fait passer une manette PlayStation, Switch ou générique pour une manette Xbox auprès des jeux, avec remappage des boutons, gyroscope et profils par jeu.

Focus Launcher gère les deux, selon un ordre de priorité précis. Attention : la section 6.8 définit les trois modes de manette réellement proposés à l'utilisateur (Natif, Steam Input, DS4Windows piloté) ; les trois niveaux ci-dessous décrivent le cadre technique de l'intégration de Steam Input et restent valables.

#### Les trois niveaux de support

| Niveau | Principe | Avantages | Limites |
| --- | --- | --- | --- |
| 1. XInput direct | Lecture directe des manettes Xbox et compatibles par Windows | Aucune dépendance, très léger, fonctionne sans Steam | Ne reconnaît pas les manettes PlayStation et Switch |
| 2. Compatibilité avec Steam Input (recommandé pour la version 1) | Focus Launcher se laisse traduire par Steam : on l'ajoute à Steam comme jeu non-Steam, et Steam présente toute manette comme une manette Xbox | Support PlayStation, Switch et génériques sans coder de pilote, profils de remappage de Steam | Nécessite que Steam soit installé et lancé |
| 3. API Steam Input intégrée (version ultérieure) | Appel direct de l'API ISteamInput de Valve (Steamworks SDK) | Contrôle complet, gyroscope, glyphes de boutons exacts | Le SDK officiel est en C++ (il faut un wrapper C#), et son usage pour un logiciel hors Steam est soumis aux conditions de Valve : **à vérifier avant de s'engager** |

#### Ce que doit faire l'agent de code

1. **Version 1 : niveaux 1 et 2.** Implémenter XInput direct (déjà prévu), et garantir que Focus Launcher fonctionne correctement lorsqu'il est lancé via Steam comme jeu non-Steam.
2. Détecter si Steam est installé et en cours d'exécution, et l'afficher dans l'écran Paramètres (« Steam Input disponible » ou non).
3. Prévoir une option de réglage : **Source des manettes = XInput direct, ou via Steam Input**.
4. Fournir un guide court dans `docs/steam-input.md` expliquant, étape par étape, comment ajouter Focus Launcher à Steam comme jeu non-Steam et choisir une configuration de manette.
5. Concevoir la couche `InputActions` (section 6.3) pour qu'elle ne dépende pas de la source : XInput et Steam Input alimentent les mêmes actions (Valider, Retour, Haut, Bas, Gauche, Droite, Options). Le niveau 3 pourra ainsi s'ajouter plus tard sans toucher aux vues.
6. **Ne pas implémenter le niveau 3 en version 1.** Avant de le faire, vérifier la documentation officielle de Valve et ses conditions d'utilisation, et demander l'accord de l'utilisateur.

#### Points d'attention (pièges connus)

- **Double saisie :** si Steam Input et un autre outil de traduction de manette (comme DS4Windows) sont actifs en même temps, chaque pression peut être comptée deux fois. L'application doit afficher un avertissement si elle détecte ce cas, et la documentation doit recommander d'en garder un seul.
- **Jeux lancés depuis Focus Launcher :** si Focus Launcher est lui-même lancé via Steam, Steam Input reste actif pendant le jeu. Pour certains jeux, cela peut créer un conflit avec leur propre gestion de manette. Prévoir un réglage par jeu pour le signaler.
- **Steam doit être ouvert** pour le niveau 2. Si Steam est fermé, Focus Launcher retombe automatiquement sur XInput direct, sans erreur.
- **Interaction avec le mode concentration :** Steam est un client de jeu et il est dans la liste blanche (section 4.2) quand le jeu choisi vient de Steam. Pour les autres jeux, Steam ne doit **pas** être coupé si l'option Steam Input est active, sinon la manette cesse de fonctionner en plein jeu. L'agent doit ajouter cette règle explicite et la tester.

#### Matériel de test de l'utilisateur

L'utilisateur possède deux manettes, ce qui permet de tester les deux niveaux de support :

| Manette | Connexion | Chemin de test | Résultat attendu |
| --- | --- | --- | --- |
| Xbox | Câblée en USB | Niveau 1 : XInput direct, sans Steam | Toute l'interface se pilote à la manette, sans aucun réglage |
| PlayStation (DualShock 4, confirmé par l'utilisateur) | Bluetooth, confirmé par l'utilisateur | Niveau 2 : via Steam Input, Focus Launcher ajouté à Steam comme jeu non-Steam | Steam la présente comme une manette Xbox, et l'interface réagit exactement comme avec la manette Xbox |

Consignes pour l'agent :

- La manette PlayStation est une **DualShock 4 connectée en Bluetooth** : écrire le guide `docs/steam-input.md` pour ce cas précis (appairage Bluetooth, activation de la prise en charge PlayStation dans les réglages de manette de Steam).
- Tester **sans Steam d'abord** avec la manette Xbox : c'est la référence qui prouve que XInput direct fonctionne seul.
- Tester ensuite la manette PlayStation **sans Steam Input** et constater qu'elle n'est pas reconnue par XInput direct : ce résultat est normal et attendu, il justifie le niveau 2.
- Tester enfin la manette PlayStation **avec Steam Input actif** et vérifier que les mêmes actions (Valider, Retour, directions, Options) fonctionnent.
- Vérifier le cas des **deux manettes branchées en même temps** : l'interface ne doit pas réagir en double ni planter. Décider d'une règle claire (par exemple : la dernière manette utilisée devient la manette active) et l'inscrire dans la documentation.
- Vérifier que **débrancher la manette Xbox en pleine navigation** ne plante pas l'application et qu'un message le signale.
- Si l'utilisateur utilise un logiciel comme DS4Windows, le lui faire désactiver pendant le test du niveau 2 pour éviter la double saisie.

#### Mapping des touches et icônes PlayStation (exigence de l'utilisateur)

L'utilisateur veut que sa DualShock 4 affiche et utilise **les boutons PlayStation** (croix, rond, carré, triangle), et non les lettres Xbox (A, B, X, Y).

| Action de l'interface | Bouton Xbox | Bouton DualShock 4 |
| --- | --- | --- |
| Valider | A | Croix |
| Retour | B | Rond |
| Action secondaire | X | Carré |
| Options de l'élément | Y | Triangle |
| Menu général | Start | Options |
| Page précédente et suivante | LB et RB | L1 et R1 |
| Défilement rapide | LT et RT | L2 et R2 |

**Problème technique :** quand Steam Input traduit la DualShock 4 en manette Xbox, Focus Launcher reçoit des « boutons Xbox » et ne peut pas deviner qu'une manette PlayStation est derrière. Les icônes doivent donc être gérées explicitement.

**Solution, par ordre de priorité :**

1. **Réglage manuel (obligatoire en version 1)** : dans Paramètres, option « Style des icônes de boutons » avec trois choix : Xbox, PlayStation, Automatique. Le choix est mémorisé.
2. **Détection automatique (optionnelle)** : reconnaître une DualShock 4 par ses identifiants matériels Sony. **À vérifier par l'agent** : quand Steam Input est actif, la manette physique peut être masquée par une manette virtuelle, et la détection peut alors échouer. Le réglage manuel reste la valeur sûre.
3. **Mode Automatique** : utilise la détection quand elle marche, sinon retombe sur le style Xbox.

**Consignes pour l'agent :**

- Créer un jeu d'icônes PlayStation **original ou libre de droits** (formes simples). Ne jamais copier les icônes officielles de Sony (section 6.4).
- Les vues ne nomment jamais un bouton (« A », « croix ») : elles demandent à la couche `InputActions` l'icône de l'action « Valider », et la couche renvoie la bonne selon le style choisi.
- Les bandeaux d'aide changent d'icône immédiatement quand le style change, sans redémarrage.
- Test obligatoire : DualShock 4 en Bluetooth via Steam Input, la croix valide, le rond revient en arrière, les icônes affichées sont PlayStation. Même test avec la manette Xbox et le style Xbox.
- Vérifier qu'un appui sur le bouton Options ou sur le pavé tactile ne déclenche pas l'action deux fois.
- Prévoir deux jeux d'icônes (Xbox et PlayStation) dans le fichier de thème, pour alterner entre les deux manettes.

### 6.4 Inspirations et direction artistique

L'interface s'inspire de trois références. L'objectif est de reprendre leurs **principes d'ergonomie et leur ambiance**, pas de copier leurs logos, icônes, polices ou éléments graphiques protégés : le logiciel doit avoir sa propre identité visuelle.

| Référence | Ce qu'on en retient | Comment l'appliquer dans Focus Launcher |
| --- | --- | --- |
| PlayStation 5 | Ligne horizontale de jeux, jaquette sélectionnée qui s'agrandit, fond qui change selon le jeu, ambiance sombre et fluide, transitions douces | Écran d'accueil : rangée de jaquettes avec zoom sur la sélection et arrière-plan dynamique en fondu |
| Xbox Series X | Tableau de bord en tuiles, accès rapide au dernier jeu joué, menu latéral détaillé, mise en avant claire de l'action principale, sections par catégorie | Bloc « Reprendre » en grand pour le dernier jeu, rangées par catégorie (Récents, Favoris, Tous les jeux), menu latéral pour les paramètres |
| Steam Big Picture | Mode plein écran pensé pour la manette sur PC, navigation simple, bouton de menu général, bibliothèque filtrable, clavier à l'écran | Navigation 100 % manette, menu général avec Start, filtres et tri de la bibliothèque, clavier virtuel pour la recherche |

#### Principes de design à respecter

- **Lisibilité à distance** : texte grand, contrastes forts, pensé pour un écran vu depuis un canapé.
- **Un seul élément sélectionné à la fois**, toujours très visible (agrandissement, halo ou contour).
- **Navigation prévisible** : jamais plus de trois pressions de bouton pour atteindre une action courante.
- **Retour partout** : le bouton B ramène toujours à l'écran précédent.
- **Animations courtes et fluides** (environ 150 à 300 millisecondes), avec une option pour les réduire.
- **Retour sensoriel** : sons discrets de navigation et vibration légère de la manette à la validation (réglables).
- **Thème sombre par défaut**, avec couleur d'accent configurable.
- **Performances** : l'interface doit rester fluide (60 images par seconde) et légère en mémoire, car elle tourne en parallèle du jeu.

#### Précisions pour l'agent de code

- Créer une charte graphique minimale dans `docs/design.md` avant d'écrire les vues : palette de couleurs, tailles de texte, durées d'animation, tailles de jaquettes.
- Concevoir d'abord une maquette de l'écran d'accueil et la faire valider par l'utilisateur avant de coder toutes les vues. Avant la phase 7, construire un prototype de cet écran dans le cadre graphique recommandé (section 6.9), mesurer la fluidité et la mémoire, et confirmer ou changer le choix.
- Utiliser des assets libres de droits ou créés pour le projet. Ne jamais intégrer de logo, de police ou d'icône propriétaire de Sony, Microsoft ou Valve.
- Prévoir un système de thèmes pour pouvoir changer l'apparence sans toucher au code des vues.

### 6.5 Immersion : un launcher pensé comme une console (exigence de l'utilisateur)

**Changement de cap :** Focus Launcher n'est pas seulement un optimiseur de jeux. L'objectif est une **expérience immersive de bout en bout**, où l'utilisateur oublie qu'il est sur un PC Windows. L'optimisation devient une fonction parmi d'autres, au service de cette immersion.

#### Principes d'immersion

- **Démarrage façon console :** possibilité de lancer Focus Launcher à l'ouverture de session Windows, avec une courte animation de démarrage, et d'arriver directement dans l'interface sans voir le bureau.
- **Aucune fenêtre Windows visible :** pas de barre de titre, pas de barre des tâches visible, pas de boîte de dialogue système en cours d'usage. Toute action (installer, configurer, quitter) passe par l'interface du launcher.
- **Ambiance sonore :** musique d'ambiance discrète dans les menus, sons de navigation, de validation et d'erreur, avec réglage du volume et option de coupure.
- **Transitions continues :** pas de coupure brutale entre le menu et le jeu. Écran de lancement avec la jaquette du jeu, puis fondu vers le jeu. Au retour, écran de fin de session.
- **Fond dynamique par jeu :** image ou couleur dominante du jeu sélectionné en arrière-plan.
- **Retour sensoriel :** légère vibration de la manette à la navigation et à la validation, réglable.
- **Menu rapide en jeu :** appui long sur le bouton central de la manette pour ouvrir un menu (revenir au launcher, quitter le jeu, voir l'état du système). **À vérifier par l'agent :** cela exige de capter la manette pendant que le jeu a le focus, ce qui peut entrer en conflit avec le jeu et avec Steam. À traiter comme fonction avancée, après la version 1.
- **Mode veille propre :** à la mise en veille puis au réveil, le launcher retrouve son état sans erreur.

#### Détection de la manette et affichage des bonnes touches (exigence de l'utilisateur)

Le logiciel doit **savoir quelle manette est branchée** (PlayStation, Xbox, autre) et **afficher directement les bonnes icônes**, sans que l'utilisateur ait à régler quoi que ce soit.

| Famille détectée | Icônes affichées | Méthode de détection |
| --- | --- | --- |
| PlayStation (DualShock 4, DualSense) | Croix, rond, carré, triangle | Identifiants matériels Sony |
| Xbox (360, One, Series) | A, B, X, Y | Manette lue via XInput |
| Autre ou inconnue | Style Xbox par défaut | Repli |

**Contrainte technique importante, à ne pas cacher :** quand Steam Input traduit une manette PlayStation en manette Xbox, la détection automatique peut être trompée, car Windows voit une manette Xbox virtuelle. L'agent doit donc :

1. Essayer plusieurs sources de détection dans l'ordre : liste des périphériques matériels, puis type de manette signalé par Windows, puis réglage mémorisé.
2. **Tester explicitement le cas critique :** DualShock 4 en Bluetooth avec Steam Input actif, pour voir si la vraie manette reste détectable.
3. Si la détection échoue dans ce cas, **le dire honnêtement à l'utilisateur** et se rabattre sur le réglage manuel de la section 6.3 bis, qui reste la valeur sûre.
4. Détecter aussi le **changement de manette en cours d'usage** : si l'utilisateur débranche la Xbox et allume la PlayStation, les icônes changent immédiatement.
5. Afficher brièvement le nom de la manette détectée à sa connexion (« Manette PlayStation connectée »).

Le logiciel doit pouvoir être **piloté entièrement à la manette**, y compris les écrans d'installation des stores (section 6.6), la saisie de texte (clavier virtuel) et les réglages.

#### Philosophie « OS sur une application » (exigence de l'utilisateur)

Focus Launcher doit se comporter **comme le système d'une console**, tout en restant une **application Windows ordinaire** : pas de remplacement de Windows, pas de modification du système, désinstallable proprement.

- **Mode shell optionnel :** au démarrage de Windows, Focus Launcher peut s'ouvrir en plein écran et se présenter comme l'unique interface. L'utilisateur peut quitter ce mode à tout moment (menu Quitter vers le bureau). **Version 1 : lancement via le dossier Démarrage de l'utilisateur uniquement.** Le remplacement de l'explorateur Windows comme shell système est **interdit en version 1** : il demande de modifier le registre, ce qui est exclu (section 9.3), et il peut rendre le PC inutilisable en cas d'erreur.
- **Écran d'accueil de type système :** heure, état de la manette, état du réseau, notifications, et accès direct à Bibliothèque, Boutique, Téléchargements et Paramètres.
- **Gestion de l'extinction :** menus pour quitter vers le bureau, redémarrer ou éteindre le PC, avec confirmation à la manette.
- **Aucun retour brutal à Windows :** si un jeu se ferme ou plante, l'utilisateur revient au launcher, pas au bureau.

#### Stores en arrière-plan (exigence de l'utilisateur)

Les clients de stores (Steam, Epic Games, EA App, etc.) tournent **réduits dans la zone de notification de Windows, comme en usage normal**, et non au premier plan.

- À la demande (installation, connexion, téléchargement, lancement d'un jeu), Focus Launcher démarre le client du store **en arrière-plan réduit**, sans le faire apparaître à l'écran, quand le client le permet.
- **Honnêteté technique :** chaque client a son propre comportement. Certains s'ouvrent toujours en fenêtre au démarrage. L'agent doit **tester chaque store** et noter dans `docs/stores.md` s'il peut démarrer réduit, par quelle option, et sinon utiliser un repli : minimiser sa fenêtre dès qu'elle apparaît.
- Focus Launcher ne doit **jamais** fermer un client de store dont un jeu en cours dépend (lien avec la section 4.2).

#### Big Picture de Steam : blocage tant que Focus Launcher est actif (exigence de l'utilisateur)

Tant que Focus Launcher est ouvert, **le mode Big Picture de Steam ne doit pas pouvoir s'afficher**, pour éviter deux interfaces plein écran concurrentes.

- **Méthode autorisée :** surveiller l'apparition de la fenêtre Big Picture. Dès qu'elle apparaît, la fermer ou la masquer, puis ramener Focus Launcher au premier plan.
- **Steam lui-même reste ouvert en arrière-plan** : seul son mode Big Picture est bloqué, car Steam est nécessaire pour lancer les jeux Steam et pour Steam Input.
- **Interdit :** modifier les fichiers de Steam, modifier le registre, ou empêcher Steam de démarrer.
- **Réglage obligatoire :** option « Bloquer Big Picture quand Focus Launcher est actif », **activée par défaut mais désactivable**, avec un moyen sûr de la désactiver à la manette.
- **À vérifier par l'agent :** Big Picture peut aussi être ouvert par un bouton de la manette (le bouton central) lorsque Steam Input est actif. L'agent doit tester ce cas précis avec la DualShock 4 et la manette Xbox, et documenter ce qui est possible et ce qui ne l'est pas. Si le blocage complet est impossible sans modifier Steam, **il faut le dire à l'utilisateur plutôt que de promettre l'impossible**.
- Si Steam met à jour son interface et que la détection échoue, le launcher ne doit **jamais planter** : il affiche un avertissement et continue.

### 6.6 Installation des stores depuis le launcher (exigence de l'utilisateur)

L'utilisateur doit pouvoir **installer les clients de stores depuis Focus Launcher**, sans passer par le bureau.

| Store | Installation |
| --- | --- |
| Steam | Via l'installateur officiel de Valve |
| Epic Games | Via l'installateur officiel d'Epic (paquet MSI) |
| EA App | Via l'installateur officiel d'Electronic Arts |
| GOG Galaxy, Ubisoft Connect, Battle.net | Même principe, selon disponibilité |

**Règles obligatoires pour l'agent :**

1. **Toujours utiliser les sources officielles :** télécharger l'installateur depuis le site officiel de l'éditeur, jamais depuis un site tiers. Vérifier l'adresse (HTTPS) et, si possible, la signature numérique de l'exécutable avant de le lancer.
2. **Option préférée : le gestionnaire de paquets Windows (winget),** quand il propose le store. Cela évite de coder des téléchargements fragiles. À vérifier au moment du développement : quels stores sont disponibles dans winget.
3. **Confirmation explicite de l'utilisateur** avant chaque installation, avec le nom de l'éditeur et la taille du téléchargement.
4. **Droits administrateur :** les installateurs peuvent demander une élévation Windows. L'interface doit prévenir l'utilisateur et gérer le refus sans planter.
5. **Installation silencieuse quand c'est possible,** sinon lancer l'installateur officiel avec son interface normale. Les options silencieuses varient selon le store et peuvent changer : **l'agent doit les vérifier dans la documentation officielle au moment du développement, et ne pas se fier à cette liste.**
6. **Connexion au compte :** l'utilisateur se connecte lui-même dans le client du store. **Focus Launcher ne saisit, ne stocke et ne lit jamais les identifiants ou mots de passe d'un store.**
7. Détecter les stores déjà installés pour ne pas proposer une réinstallation.
8. Écran de progression à la manette, avec possibilité d'annuler.
9. **Journaliser** chaque installation dans `logs/`.
10. Vérifier les conditions d'utilisation des stores avant toute diffusion publique de Focus Launcher, car l'automatisation de leurs clients peut être encadrée.

### 6.7 Chargement des bibliothèques de jeux (exigence de l'utilisateur)

L'utilisateur doit pouvoir **charger les jeux de toutes ses bibliothèques** dans Focus Launcher : Steam, Epic Games, EA App, GOG, Ubisoft Connect, Battle.net.

| Store | Source de détection | Remarque |
| --- | --- | --- |
| Steam | Fichiers `libraryfolders.vdf` et `appmanifest_*.acf` | Vérifié dans plusieurs sources |
| Epic Games | Manifestes `.item` dans `C:\ProgramData\Epic\EpicGamesLauncher\Data\Manifests` | Vérifié dans plusieurs sources |
| GOG | Registre (lecture seule) ou GOG Galaxy | À vérifier par l'agent |
| EA App | Fichiers de configuration locaux de l'EA App | **À vérifier par l'agent : le format a changé avec le passage d'Origin à l'EA App** |
| Ubisoft Connect | Fichiers de configuration locaux et registre (lecture seule) | À vérifier par l'agent |
| Battle.net | Fichiers de configuration locaux | À vérifier par l'agent |

**Règles :**

- **Lecture seule uniquement :** Focus Launcher ne modifie jamais les fichiers des stores.
- **Les formats des stores changent sans préavis.** Pour chaque store, un scanner isolé, avec un test, et un échec propre : si un store n'est pas lisible, le launcher l'indique sans planter et propose l'ajout manuel.
- **Jeux installés seulement en version 1.** Lister les jeux non installés (achetés mais pas téléchargés) demande l'accès aux comptes en ligne : **hors périmètre de la version 1.**
- **Lancement via le store :** un jeu Steam se lance par le protocole Steam, un jeu Epic par le protocole Epic, etc. (section 5.3). Le store tourne alors en arrière-plan (voir plus haut).
- **Fusion des doublons :** un même jeu présent dans deux stores apparaît une seule fois, avec choix de la source au lancement.
- **Actualisation :** nouvelle analyse à chaque démarrage et à la demande, avec détection des jeux ajoutés ou supprimés.
- Les jaquettes viennent des API publiques (section 5.2) ; sans clé ou sans réseau, une image de remplacement est affichée.

### 6.8 Gestion native des manettes : trois modes et basculement automatique (exigence de l'utilisateur)

L'utilisateur veut que la manette se règle **nativement dans Focus Launcher**, avec la possibilité soit de faire détecter la manette directement, soit d'utiliser Steam, et un basculement automatique quand un jeu Steam est lancé. Cette section remplace et précise les niveaux de la section 6.3 bis.

#### Les trois modes de manette (réglage dans Paramètres)

| Mode | Principe | Quand l'utiliser |
| --- | --- | --- |
| 1. Natif | Focus Launcher lit directement les manettes. Xbox via XInput. DualShock 4 par lecture HID directe (identifiants Sony), sans aucun outil externe | Par défaut, sans dépendance |
| 2. Steam Input | Steam traduit la manette (section 6.3 bis) | Quand Steam est ouvert et que l'utilisateur veut ses profils Steam |
| 3. DS4Windows piloté | Focus Launcher démarre et arrête le programme DS4Windows **installé séparément**, pour traduire la DualShock 4 en manette Xbox virtuelle | Pour les jeux qui ne comprennent que XInput |

#### Basculement automatique à l'ouverture d'un jeu Steam (exigence de l'utilisateur)

L'utilisateur décrit ce comportement : dans le launcher, DS4Windows fournit le mapping. Quand un jeu Steam est lancé, l'entrée passe sur celle de Windows (et de Steam), et le DS4Windows intégré **se ferme automatiquement** pour ne pas créer de double saisie.

1. Dans le launcher : mode 3 actif, DS4Windows démarré par Focus Launcher.
2. Au clic sur « Jouer » pour un jeu Steam : Focus Launcher **ferme proprement DS4Windows** (fermeture normale de la fenêtre d'abord, arrêt forcé seulement après un délai), puis lance le jeu, et Steam Input prend le relais.
3. À la fermeture du jeu : Focus Launcher **relance DS4Windows** automatiquement et reprend la main.
4. Si DS4Windows ne se ferme pas, ou si Steam Input ne prend pas le relais, afficher un avertissement clair plutôt qu'un échec silencieux.
5. Chaque bascule est consignée dans le journal de session, avec restauration en cas de plantage (même principe que la section 4.4).

**À vérifier par l'agent :** ce comportement dépend du bon fonctionnement de Steam Input avec la DualShock 4 en Bluetooth. Le tester explicitement, y compris le cas où Steam masque la manette physique.

#### Licence : ce qui est permis et ce qui ne l'est pas (point critique)

DS4Windows est sous licence **GNU GPL version 3** (sources officielles consultées). Il y a trois façons de l'« intégrer », et elles n'ont pas les mêmes conséquences.

| Façon d'intégrer | Conséquence légale |
| --- | --- |
| **Piloter DS4Windows comme programme séparé** (le démarrer, l'arrêter, lire son état) | **Voie recommandée.** Les deux programmes restent indépendants. Focus Launcher peut garder sa propre licence. À confirmer par un juriste si diffusion publique. |
| **Livrer DS4Windows avec Focus Launcher** (le télécharger et l'installer pour l'utilisateur) | Possible, mais il faut respecter la GPL pour DS4Windows lui-même (fournir ses sources et sa licence) et télécharger depuis la source officielle. Moins simple. |
| **Copier son code dans Focus Launcher** | **Interdit dans ce projet** : cela obligerait à publier tout Focus Launcher sous GPL v3 dès qu'il est distribué (section 7 quinquies). |

**Consigne pour l'agent :** implémenter la voie 1 (programme séparé) uniquement. L'utilisateur installe DS4Windows lui-même depuis son site officiel ; Focus Launcher détecte sa présence, le démarre et l'arrête. **Ne jamais copier de code de DS4Windows.** Pour le mode 1 (natif), écrire sa propre lecture HID DualShock 4 à partir des spécifications publiques, ou utiliser une bibliothèque sous licence permissive (MIT, Apache 2.0, BSD).

**Dépendances de DS4Windows à connaître :** il s'appuie sur des pilotes système (par exemple ViGEmBus pour créer une manette virtuelle, HidHide pour masquer la manette physique). **Focus Launcher ne doit jamais installer de pilote système sans confirmation explicite de l'utilisateur**, car un pilote mal installé peut déstabiliser Windows. Si ces pilotes manquent, le launcher l'indique et renvoie vers le site officiel.

#### Détection de la manette depuis le launcher

- Au démarrage, lister les manettes connectées et leur famille (PlayStation, Xbox, autre) selon la section 6.5.
- Proposer **automatiquement le mode adapté** : manette Xbox, mode 1 ; DualShock 4 avec DS4Windows détecté, mode 3 ; DualShock 4 avec Steam ouvert, mode 2 ; sinon mode 1.
- Un bouton « Détecter la manette » dans Paramètres relance la détection et affiche le résultat en clair.
- Le choix de l'utilisateur est mémorisé et **prime toujours** sur la détection automatique.

### 6.9 Optimisations de performance : sources open source et cadres graphiques

L'utilisateur a demandé de chercher des informations sur les optimisations et les cadres graphiques possibles dans des dépôts open source. Voici ce qui a été trouvé et comment l'utiliser. **Ces projets sont des références à étudier, pas du code à copier** (voir la section 7 quinquies sur les licences).

#### Projets de référence pour les optimisations

| Projet | Ce qu'il fait | Ce qu'on peut en apprendre | Attention |
| --- | --- | --- | --- |
| Zintom/GameOptimizer | Limite la priorité des autres processus et le nombre de cœurs qu'ils utilisent | Idée très proche du mode concentration, approche par priorité plutôt que par arrêt | Licence à vérifier avant toute réutilisation |
| charlie754/Game-Optimizer | Application de barre système qui place les jeux sur les bons cœurs via l'API Windows CPU Sets | Alternative plus propre que les masques d'affinité pour les processeurs récents | Licence à vérifier ; l'auteur signale lui-même que des antivirus peuvent réagir à ce type d'outil |
| bensheed/OpenGameBoost | Active un mode jeu à la détection d'un jeu, avec suspension de processus et gestion de l'alimentation | Détection automatique des jeux et suspension de processus | Utilise aussi des réglages de registre, **interdits en version 1** |
| theantipopau/windows11nontouchgamingoptimizer | Script d'optimisation Windows 11 (plan d'alimentation, mode jeu, services, effets visuels) | Liste d'idées d'optimisations, avec création d'un point de restauration avant modification | Beaucoup de réglages de registre et réseau : **à ne pas reprendre en version 1** |
| MobinMardi/Exoptimizer | Optimiseur centré sur un jeu précis | Idée du suivi en direct du processeur et de la mémoire | Contient des modifications de registre et un réglage de priorité temps réel, **que ce projet interdit** |

**Lecture honnête de ces sources :** plusieurs de ces outils reposent sur des modifications du registre et des réglages agressifs. Ce projet les exclut volontairement en version 1 (sections 9.3 et 4.5), car le risque pour le système est plus grand que le gain. Les gains réels de ces optimisations ne sont **pas démontrés dans les sources consultées** : l'agent doit les mesurer lui-même (section 11) avant d'affirmer qu'elles aident.

**Idées retenues pour la suite, par ordre de prudence :**

1. Baisser la **priorité** des processus d'arrière-plan plutôt que les arrêter (moins risqué, entièrement réversible).
2. Utiliser l'API **CPU Sets** de Windows pour les processeurs récents, après mesure.
3. Suspendre temporairement certains processus lourds au lieu de les fermer, avec reprise garantie.
4. Journaliser et mesurer chaque optimisation, et ne garder que celles qui apportent un gain mesuré.

#### Projets de référence pour le launcher plein écran

| Projet | Intérêt | Ce qu'on peut en apprendre |
| --- | --- | --- |
| Playnite | Gestionnaire de bibliothèque open source qui charge Steam, Epic, GOG, EA App, Battle.net et d'autres, avec mode plein écran manette et thèmes | La référence la plus proche de Focus Launcher : détection des stores, mode plein écran, système de thèmes, extensions. Son prochain grand modèle reste une interface Windows (WPF) |
| Pegasus Frontend | Interface open source personnalisable, thèmes entièrement libres, support manette | Architecture des thèmes, animations, personnalisation complète de l'écran |

**Cadres graphiques possibles pour l'interface** (choix à trancher par l'agent après un petit prototype, en s'appuyant sur ces références) :

| Cadre | Atouts | Limites |
| --- | --- | --- |
| WPF (.NET) | Mature, utilisé par Playnite, bon support des animations, intégration Windows simple | Windows uniquement, rendu moins moderne |
| WinUI 3 | Moderne, animations fluides | Écosystème plus récent, plein écran personnalisé plus délicat |
| Avalonia UI | Multi-plateforme, open source | Selon les développeurs de Playnite, l'expérience de développement n'est pas encore jugée suffisante pour leur projet |
| Pegasus (Qt/QML) | Très flexible pour des thèmes plein écran | Pile technique différente de C#, demanderait un autre langage |

**Recommandation de départ :** WPF, par cohérence avec le choix technique de la section 2 et parce que Playnite prouve qu'un launcher plein écran à la manette est réalisable avec. L'agent doit **construire un prototype de l'écran d'accueil dans le cadre choisi, mesurer la fluidité (60 images par seconde) et la mémoire utilisée**, puis confirmer ou changer ce choix avant la phase 7.

**Consigne :** tenir à jour `docs/references.md` avec chaque projet consulté, son adresse, sa licence vérifiée, et ce qui en a été retenu.

## 7. Structure du projet

```text
FocusLauncher/
  FocusLauncher.sln
  src/
    FocusLauncher.Core/
      Models/            (Game, OptimizationRule, SessionLog, SystemReport)
      Interfaces/        (IGameRepository, IOptimizer, ISystemScanner)
    FocusLauncher.Optimizer/
      Scanning/          (ProcessScanner, ServiceScanner, StartupScanner, HardwareScanner)
      Rules/             (RuleEngine, WhitelistProvider, RuleFileLoader)
      Execution/         (SessionManager, ServiceController wrapper, ProcessCloser, PowerPlanManager)
      Recovery/          (SessionJournal, CrashRecovery)
    FocusLauncher.Library/
      Scanners/          (SteamScanner, EpicScanner, GogScanner, ManualScanner)
      Launching/         (GameStarter, ProcessWatcher)
      Storage/           (SqliteGameRepository)
      Covers/            (CoverFetcher)
    FocusLauncher.UI/
      Views/             (HomeView, GameDetailView, PreparingView, SettingsView, ScanView)
      ViewModels/
      Input/             (GamepadService, InputActions)
      Assets/
  tests/
    FocusLauncher.Optimizer.Tests/
    FocusLauncher.Library.Tests/
  docs/
  optimizer-rules.json
```

## 7 bis. Mode boucle : objectif final et condition d'arrêt

L'agent de code travaille **en boucle** jusqu'à ce que l'objectif final soit atteint et prouvé par des tests. Il ne s'arrête ni parce qu'il a « fini d'écrire du code », ni parce que ça compile : il s'arrête uniquement quand tous les critères ci-dessous sont validés.

### Objectif final (définition de « terminé »)

Un logiciel Windows nommé Focus Launcher, installé et fonctionnel sur le PC de l'utilisateur, qui :

1. s'ouvre en plein écran et se pilote entièrement à la manette ;
2. affiche une bibliothèque de jeux (ajout manuel, plus détection de Steam et d'Epic Games) ;
3. au clic sur « Jouer », analyse le système, coupe temporairement les processus et services autorisés, lance le jeu en priorité haute ;
4. à la fermeture du jeu, restaure exactement l'état d'origine du système ;
5. se répare seul après un plantage grâce au journal de session ;
6. ne cause aucune erreur de compilation, aucun test en échec, aucun avertissement bloquant.

### Condition d'arrêt de la boucle

La boucle ne s'arrête que si **toutes** ces conditions sont vraies en même temps :

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 1 | La solution compile sans erreur ni avertissement bloquant | `dotnet build -warnaserror` renvoie le code 0 |
| 2 | Tous les tests automatiques passent | `dotnet test` : 0 échec, 0 test ignoré sans justification |
| 3 | Le mode simulation produit un rapport correct sur le PC réel | Relecture du rapport par l'utilisateur |
| 4 | Une session réelle coupe puis restaure : l'état avant et après est identique | Comparaison automatique des instantanés (liste des processus et services avant, après) |
| 5 | La récupération après plantage fonctionne | Test : tuer le lanceur en pleine session, relancer, vérifier la restauration |
| 6 | La manette pilote toute l'interface | Test manuel par l'utilisateur, liste de contrôle fournie |
| 7 | Aucun élément de la liste blanche n'a jamais été touché | Test automatique + relecture du journal |
| 8 | Le fichier `PROGRESS.md` indique toutes les phases terminées | Relecture du fichier |

Tant qu'une seule condition est fausse, l'agent retourne dans la boucle et corrige.

### Garde-fous contre les boucles infinies

- Maximum de **5 tentatives** pour corriger le même problème. Après la cinquième, l'agent s'arrête, explique le blocage en détail et demande de l'aide à l'utilisateur.
- Si une correction casse un test qui passait avant, l'agent annule cette correction (retour au dernier commit) avant d'essayer autre chose.
- L'agent ne supprime jamais un test, ne l'affaiblit jamais et ne le contourne jamais pour le faire passer.
- L'agent ne déclare jamais une phase terminée sans avoir exécuté les commandes de vérification et affiché leur résultat réel.

## 7 ter. Règles de la boucle de travail

Chaque phase du plan de travail suit exactement le même cycle. L'agent ne passe à la phase suivante que si le cycle est validé.

### Le cycle en huit temps

1. **Lire** : relire `PROGRESS.md`, la phase en cours et ses critères de validation.
2. **Planifier** : écrire en quelques lignes ce qui va être fait dans cette itération, et quels fichiers seront touchés.
3. **Écrire les tests d'abord** quand c'est possible (tests unitaires du moteur de règles, du journal de session, du classement).
4. **Implémenter** la plus petite portion de code qui fait avancer la phase.
5. **Compiler** : `dotnet build -warnaserror`. En cas d'erreur, corriger avant toute autre chose.
6. **Tester** : `dotnet test`. Lire les échecs en entier, ne pas deviner.
7. **Vérifier sur le système réel** quand la phase le demande, toujours en mode simulation d'abord.
8. **Enregistrer** : mettre à jour `PROGRESS.md`, faire un commit Git clair, puis retourner à l'étape 1.

### Le fichier PROGRESS.md

L'agent crée et tient à jour ce fichier à la racine du projet. Il contient pour chaque phase : son statut (non commencée, en cours, terminée, bloquée), la date, les commandes de vérification lancées avec leur résultat réel, les problèmes rencontrés et leur solution, et les décisions prises. C'est la mémoire de la boucle : si l'agent est relancé, il reprend exactement là où il s'était arrêté.

### Règles de qualité (zéro erreur)

- Aucune phase n'est « presque finie » : elle est validée ou elle ne l'est pas.
- Chaque fonction qui touche au système (arrêt de service, fermeture de processus, changement de schéma d'alimentation) a un test, une version simulation, et une fonction inverse de restauration.
- Toute exception est capturée, journalisée avec le contexte, et déclenche une restauration si une session est en cours.
- Pas de chemin codé en dur : tout passe par la configuration.
- Pas de dépendance ajoutée sans la justifier dans `PROGRESS.md`.
- Un commit Git par itération validée, pour pouvoir revenir en arrière à tout moment.
- En cas de doute sur l'effet d'une action sur le système, l'agent ne la fait pas et pose la question à l'utilisateur.

### Quand l'agent doit s'arrêter et demander

- Avant la première coupure réelle de processus ou de service.
- Avant de créer le point de restauration Windows.
- Quand un élément est classé « incertain ».
- Après 5 tentatives infructueuses sur le même problème.
- À la fin de chaque grande phase, pour présenter un résumé et obtenir l'accord de continuer.

## 7 quater. Plan de travail détaillé par phases

Chaque phase indique : l'objectif, les tâches, les fichiers à créer, les tests à écrire et le critère de validation. La phase suivante ne commence qu'après validation.

### Phase 0 : préparation de l'environnement

**Objectif :** disposer d'un poste prêt à compiler et d'un dépôt Git propre.

Tâches :

1. Vérifier la présence du SDK .NET 8 (`dotnet --version`). Sinon, l'installer et le dire à l'utilisateur.
2. Vérifier Git (`git --version`) et initialiser un dépôt dans le dossier du projet.
3. Créer un fichier `.gitignore` adapté à .NET (dossiers `bin`, `obj`, `logs`, fichiers de rapport locaux).
4. Créer `PROGRESS.md` avec la liste de toutes les phases à l'état « non commencée », ainsi que docs/licenses.md et docs/references.md (section 6.9).
5. Vérifier que l'ordinateur a assez d'espace disque libre (au moins 5 Go).

**Validation :** `dotnet --version` et `git status` fonctionnent, `PROGRESS.md` existe.

### Phase 1 : squelette de la solution

**Objectif :** une solution qui compile avec les quatre projets, la journalisation et la configuration.

Tâches :

1. Créer `FocusLauncher.sln` et les projets `FocusLauncher.Core`, `FocusLauncher.Optimizer`, `FocusLauncher.Library`, `FocusLauncher.UI`, plus les deux projets de tests.
2. Configurer les références entre projets : l'interface dépend de tout, l'optimiseur et la bibliothèque dépendent seulement du Core.
3. Activer `Nullable` et `TreatWarningsAsErrors` dans tous les projets.
4. Ajouter Serilog avec écriture dans `logs/launcher-.log` (un fichier par jour).
5. Créer le chargement de la configuration depuis un fichier `appsettings.json`.
6. Écrire un premier test trivial pour vérifier que la chaîne de test fonctionne.

**Validation :** `dotnet build -warnaserror` et `dotnet test` réussissent, un fichier de log est créé au lancement.

### Phase 2 : analyse du système en lecture seule

**Objectif :** produire `system-report.json` et `system-report.md` sans modifier quoi que ce soit.

Tâches :

1. Modèles dans Core : `ProcessInfo`, `ServiceInfo`, `StartupItem`, `HardwareInfo`, `SystemReport`.
2. `ProcessScanner` : liste des processus avec nom, identifiant, chemin de l'exécutable, mémoire utilisée, utilisateur. Gérer proprement les processus dont l'accès est refusé.
3. `ServiceScanner` : liste des services avec nom interne, nom affiché, état, type de démarrage, chemin du binaire, éditeur (pour repérer ceux qui ne sont pas Microsoft).
4. `StartupScanner` : dossier Démarrage, clés de registre `Run` en lecture seule, tâches planifiées à l'ouverture de session.
5. `HardwareScanner` : processeur, mémoire, carte graphique, via WMI en lecture seule.
6. `LauncherDetector` : détecter Steam, Epic, GOG, EA, Ubisoft, Battle.net et leurs dossiers de bibliothèques.
7. `ReportWriter` : écrire le rapport JSON et sa version lisible en Markdown.
8. Tests : utiliser des interfaces et des faux scanners pour tester le rapport sans dépendre de la machine.

**Validation :** le rapport est généré sur le PC réel, l'utilisateur le relit et le juge exact. Vérification explicite : aucun processus ni service n'a changé d'état pendant l'analyse (comparaison avant et après).

### Phase 3 : moteur de règles et mode simulation

**Objectif :** classer chaque élément et afficher ce qui serait coupé, sans rien couper.

Tâches :

1. Modèle `OptimizationRule` et chargeur de `optimizer-rules.json` avec validation stricte (un fichier invalide est refusé avec un message clair).
2. `WhitelistProvider` : liste blanche intégrée au code, **non modifiable depuis le fichier de règles**, pour qu'une erreur de configuration ne puisse jamais viser un processus critique.
3. `RuleEngine` : pour chaque élément, décider Protégé, Coupable sans risque ou Incertain. La liste blanche l'emporte toujours sur toute règle.
4. `SimulationRunner` : produit le plan de coupure (liste, raisons, risque) sans aucune action.
5. Tests unitaires obligatoires : chaque élément de la liste blanche est refusé même si une règle le vise ; un élément inconnu est classé Incertain ; un fichier de règles invalide est rejeté.
6. Générer un `optimizer-rules.json` initial adapté au PC analysé, avec une raison par règle.

**Validation :** les tests de la liste blanche passent, la simulation sur le PC réel affiche un plan que l'utilisateur approuve ligne par ligne.

### Phase 4 : journal de session et récupération après plantage

**Objectif :** garantir qu'on peut toujours revenir à l'état d'origine, **avant** d'écrire le code qui coupe quoi que ce soit.

Tâches :

1. `SessionJournal` : écrit `session.lock.json` de façon atomique (écriture dans un fichier temporaire, puis renommage) avec la liste de ce qui va être coupé, l'état d'origine de chaque élément, et l'heure.
2. `CrashRecovery` : au démarrage, si le fichier existe, lire la liste et relancer ce qui doit l'être.
3. `SystemSnapshot` : photographie de l'état (processus et services en cours) pour comparer avant et après.
4. Tests : journal écrit puis relu à l'identique ; fichier corrompu géré sans plantage ; récupération simulée avec de faux services.

**Validation :** tous les tests de récupération passent avec des faux services. Rien n'a encore été coupé sur le vrai système.

### Phase 5 : coupure et restauration réelles

**Objectif :** couper vraiment, puis restaurer exactement, avec un maximum de prudence.

Tâches :

1. Demander à l'utilisateur d'accepter la création d'un point de restauration Windows, puis le créer.
2. `ServiceStopper` et `ServiceRestorer` : arrêt et redémarrage avec délai maximal et gestion des dépendances entre services.
3. `ProcessCloser` : fermeture propre d'abord, arrêt forcé seulement si la règle l'autorise et après un délai.
4. `ProcessRelauncher` : relance avec le chemin et les arguments enregistrés.
5. `PowerPlanManager` : lecture du schéma actif, passage en performances élevées, restauration de l'ancien.
6. `SessionManager` : enchaîne tout dans l'ordre, avec restauration garantie même en cas d'exception (blocs `finally`).
7. **Premier essai réel limité à un seul élément inoffensif** choisi avec l'utilisateur, par exemple un utilitaire de synchronisation. Puis deux éléments, puis le profil complet.
8. Test de plantage : tuer le lanceur en pleine session, relancer, vérifier que tout est restauré. Mesurer le gain réel de chaque optimisation (mémoire libre, temps de lancement) et ne conserver que celles qui apportent un gain mesuré. Tester en priorité la baisse de priorité des processus d'arrière-plan, moins risquée que leur arrêt (section 6.9).

**Validation :** après une session complète, la comparaison des instantanés montre exactement le même état qu'avant. Le test de plantage réussit au moins trois fois de suite.

### Phase 6 : bibliothèque de jeux

**Objectif :** ajouter, stocker et lancer des jeux, avec suivi du temps de jeu.

Tâches :

1. Modèle `Game` et `SqliteGameRepository` (création, lecture, modification, suppression).
2. `ManualScanner` : ajout d'un `.exe` ou d'un raccourci.
3. `SteamScanner` : lecture de `libraryfolders.vdf` et des fichiers `appmanifest_*.acf`.
4. `EpicScanner` : lecture des manifestes Epic.
5. `GameStarter` : lancement selon la source (exécutable, protocole Steam, protocole Epic).
6. `ProcessWatcher` : retrouver le vrai processus du jeu et compter le temps de jeu.
7. Scanners isolés pour GOG, EA App, Ubisoft Connect et Battle.net selon la section 6.7, chacun avec un échec propre si le format a changé. Fusion des doublons entre stores. Installation des stores depuis le launcher selon la section 6.6 (sources officielles, confirmation de l'utilisateur, aucun identifiant jamais stocké), avec fichier \`docs/stores.md\` consignant ce qui a été testé pour chaque store. Tests avec de faux dossiers de bibliothèque et une base SQLite temporaire.

**Validation :** les jeux de l'utilisateur sont détectés, au moins un jeu de chaque source disponible se lance et le temps de jeu est correct.

### Phase 7 : interface plein écran

**Objectif :** une interface fluide pilotée à la manette.

Tâches :

1. `GamepadService` : lecture XInput environ 60 fois par seconde, avec anti-rebond et répétition des directions maintenues.
2. `InputActions` : couche qui traduit les boutons en actions (Valider, Retour, Haut, Bas, Gauche, Droite, Options).
3. Fenêtre plein écran sans bordure, avec l'immersion de la section 6.5 : animation de démarrage, ambiance sonore, transitions continues, écran d'accueil de type système, détection de la manette PlayStation ou Xbox avec affichage direct des bonnes icônes, clavier virtuel à la manette.
4. Vues : Accueil (ligne de jaquettes), Fiche du jeu, Préparation, Paramètres, Analyse du système.
5. Navigation complète au clavier et à la souris en secours.
6. Gestion de la déconnexion de la manette en cours d'utilisation. Trois modes de manette et basculement automatique à l'ouverture d'un jeu Steam selon la section 6.8 (DS4Windows piloté comme programme séparé uniquement, aucun code copié, aucun pilote installé sans confirmation). Intégration de Steam Input niveaux 1 et 2 selon la section 6.3 bis : détection de Steam, réglage de la source des manettes, avertissement en cas de double saisie, repli automatique sur XInput direct, et guide \`docs/steam-input.md\`.

**Validation :** L'utilisateur parcourt toute l'interface uniquement à la manette, avec la liste de contrôle fournie par l'agent, au moins avec une manette Xbox en XInput direct et une manette PlayStation via Steam Input si l'utilisateur en possède une.

### Phase 8 : intégration du flux « Jouer »

**Objectif :** relier bibliothèque, optimiseur et interface.

Tâches :

1. Bouton Jouer : simulation si première utilisation, sinon coupure réelle, puis lancement du jeu. Stores démarrés en arrière-plan réduit, et blocage de Big Picture de Steam tant que Focus Launcher est actif (section 6.5), testé avec la DualShock 4 et la manette Xbox.
2. Écran de préparation qui montre en direct ce qui est coupé.
3. Surveillance du jeu, puis restauration automatique à la fermeture.
4. Écran de fin de session : temps joué et confirmation de la restauration.
5. Cas limites : jeu qui plante, jeu qui ne se lance pas, manette débranchée, changement de manette en cours d'usage, veille du PC, deux lancements successifs rapides, store qui ne démarre pas en mode réduit, mise à jour de Steam qui change son interface.

**Validation :** dix sessions réelles d'affilée, avec des jeux différents, sans aucune erreur ni état système altéré après coup.

### Phase 9 : finitions et livraison

**Objectif :** un logiciel installable et stable.

Tâches :

1. Récupération automatique des jaquettes (avec clé d'API fournie par l'utilisateur).
2. Sons de navigation et animations de transition.
3. Option de démarrage avec Windows.
4. Installateur ou paquet autonome (`dotnet publish`).
5. Documentation utilisateur courte dans `docs/`.
6. Passage final de toutes les conditions d'arrêt du mode boucle.

**Validation :** les huit conditions du tableau d'arrêt sont vraies en même temps, et `PROGRESS.md` indique toutes les phases terminées.

### Ordre et dépendances

Les phases se font dans l'ordre. Deux points sont non négociables : la phase 4 (journal et récupération) vient **avant** la phase 5 (coupure réelle), et la liste blanche de la phase 3 doit être testée avant toute coupure. Les phases 6 et 7 peuvent avancer en parallèle une fois la phase 3 validée, mais l'intégration (phase 8) exige que toutes les précédentes soient validées.

## 7 quinquies. Réutilisation de code open source et licences

**Cas de DS4Windows** : ce logiciel est publié sous licence GNU GPL version 3 (vérifié dans les dépôts officiels). Cette licence autorise à lire, étudier et modifier le code, mais impose des conditions si on en réutilise des morceaux.

**Règles pour l'agent de code :**

1. **Étudier pour comprendre est permis** : lire le code pour comprendre comment une manette DualShock 4 communique (formats de rapports, identifiants matériels) est légitime. Les faits techniques et les idées ne sont pas protégés.
2. **Ne jamais copier-coller de code de DS4Windows** dans Focus Launcher. Copier du code GPL v3 oblige à publier tout Focus Launcher sous GPL v3, avec son code source, dès qu'il est distribué.
3. **Réécrire avec ses propres mots** : après avoir compris le principe, écrire sa propre implémentation, sans reprendre la structure ni les noms du code d'origine.
4. **Préférer une bibliothèque sous licence permissive** (MIT, Apache 2.0, BSD) quand on a besoin de code existant, et noter sa licence dans `docs/licenses.md`.
5. **Pour la version 1, DS4Windows n'est pas nécessaire** : Steam Input assure déjà la traduction de la DualShock 4.
6. Tenir à jour `docs/licenses.md` : chaque dépendance, sa licence, sa version.

Ce texte est une consigne de prudence, pas un avis juridique. Si le logiciel doit être diffusé publiquement ou vendu, faire relire les licences par un juriste.

## 8. Feuille de route

La feuille de route complète, avec les tâches, les tests et les critères de validation de chaque phase (de la phase 0 à la phase 9), se trouve dans les sections 7 bis, 7 ter et 7 quater. Cette section ne résume que l'ordre général :

Phase 0 préparation, phase 1 squelette, phase 2 analyse en lecture seule, phase 3 moteur de règles et simulation, phase 4 journal et récupération, phase 5 coupure réelle, phase 6 bibliothèque, phase 7 interface, phase 8 intégration, phase 9 finitions. En cas de contradiction, le plan détaillé des sections 7 bis à 7 quater fait foi.

## 9. Instructions pour l'outil de code qui analysera le système

Cette section s'adresse directement à l'agent ou à l'outil de code lancé sur l'ordinateur de l'utilisateur.

### 9.1 Mission

Tu vas d'abord **analyser le système en lecture seule**, puis proposer un plan, puis seulement après validation de l'utilisateur, implémenter le projet phase par phase, en mode boucle, selon le plan de travail détaillé des sections 7 bis à 7 quater, jusqu'à ce que toutes les conditions d'arrêt soient vraies.

### 9.2 Phase d'analyse (lecture seule, aucune modification)

Collecte et résume :

1. Version de Windows et édition, architecture (x64 ou ARM).
2. Processeur, quantité de mémoire, carte graphique et version du pilote.
3. Présence de .NET SDK (version), de Visual Studio ou de VS Code. Si absent, indiquer comment l'installer.
4. Processus en cours, triés par mémoire décroissante.
5. Services Windows en cours d'exécution, avec leur type de démarrage, et signaler ceux qui ne sont pas fournis par Microsoft.
6. Programmes lancés au démarrage (dossier Démarrage, clés `Run` du registre, tâches planifiées à l'ouverture de session).
7. Lanceurs de jeux installés (Steam, Epic, GOG, EA, Ubisoft, Battle.net) et leurs dossiers de bibliothèques.
8. Manettes connectées et type.
9. Schéma d'alimentation actif et réglages du Mode Jeu.

Le résultat doit être un fichier `system-report.json` et un résumé lisible `system-report.md`.

### 9.3 Règles de sécurité impératives

- **Ne rien arrêter, supprimer ou modifier pendant l'analyse.** Lecture seule uniquement.
- Ne jamais modifier le registre Windows dans la version 1 (la lecture seule est autorisée). Conséquence : l'activation du Mode Jeu et de l'assistant de concentration (section 4.5) reste en option « à étudier » et n'est pas implémentée en version 1 si elle exige une écriture dans le registre.
- Ne jamais désactiver définitivement un service : uniquement arrêt temporaire avec restauration garantie.
- Ne jamais toucher aux services de sécurité (Windows Defender, pare-feu), ni aux processus système.
- Toujours proposer le mode simulation avant la première coupure réelle.
- Demander confirmation à l'utilisateur avant chaque étape qui modifie le système.
- Créer un point de restauration Windows avant les premiers tests réels de coupure.
- Tout journaliser dans un dossier `logs/`.

### 9.4 Après l'analyse

1. Présenter le rapport à l'utilisateur en langage simple.
2. Proposer un fichier `optimizer-rules.json` initial, adapté à CE système, avec une raison pour chaque règle.
3. Attendre la validation, puis commencer la Phase 0 du plan de travail (sections 7 bis à 7 quater) et travailler en mode boucle jusqu'à la condition d'arrêt.
4. Après chaque phase : compiler, lancer les tests, mettre à jour PROGRESS.md, résumer ce qui a été fait et demander l'accord pour la suite.

## 10. Exemple de fichier de règles

```json
{
  "version": 1,
  "profiles": [
    {
      "name": "Concentration standard",
      "simulationOnly": true,
      "rules": [
        {
          "type": "process",
          "name": "OneDrive",
          "action": "close",
          "restoreAfterGame": true,
          "risk": "low",
          "reason": "Synchronisation cloud inutile pendant le jeu"
        },
        {
          "type": "service",
          "name": "WSearch",
          "action": "stop",
          "restoreAfterGame": true,
          "risk": "low",
          "reason": "Indexation de recherche, gourmande en disque"
        },
        {
          "type": "process",
          "name": "chrome",
          "action": "askUser",
          "restoreAfterGame": false,
          "risk": "medium",
          "reason": "Risque de perte d'onglets non sauvegardés"
        }
      ]
    }
  ],
  "neverTouch": ["explorer", "dwm", "csrss", "winlogon", "lsass", "MsMpEng"]
}
```

## 11. Critères de réussite

- Le jeu démarre au moins aussi vite qu'avant (mesure : temps de lancement chronométré avec et sans le mode concentration, sur trois essais), et la mémoire libre augmente mesurablement pendant la session (mesure : mémoire disponible avant et pendant le jeu, consignée dans le journal). Attention : sur un PC récent avec beaucoup de mémoire, le gain peut rester modeste, ce qui n'est pas une erreur du logiciel.
- Après chaque session, le système retrouve exactement son état d'origine.
- Un plantage du lanceur ne laisse jamais Windows dans un état coupé.
- L'interface est utilisable entièrement à la manette, sans clavier ni souris.
- Le lanceur lui-même consomme très peu de ressources pendant le jeu.

## 12. Idées pour plus tard

- Profils par jeu, avec mesure automatique des images par seconde avant et après.
- Superposition en jeu (overlay) pour afficher la mémoire et le processeur.
- Prise en charge des émulateurs de consoles dans la bibliothèque.
- Thèmes visuels et sauvegarde des paramètres dans le cloud.
- API Steam Input intégrée (niveau 3 de la section 6.3 bis) pour le gyroscope et les glyphes exacts, après vérification des conditions de Valve.
