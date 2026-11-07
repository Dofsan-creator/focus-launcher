# Focus Launcher : mise à jour v3 (public mondial, bibliothèque façon console, outils open source)

Ce fichier complète `focus-launcher-mise-a-jour-v2.md`. Il **remplace** : la section 6.9 (optimisations et cadres graphiques), la section 6.2 écran « Accueil » et « Fiche du jeu », le point « Menu rapide en jeu » de la section 6.5. Il **ajoute** : 6.11 (langues et public mondial), 6.12 (bibliothèque façon console), 6.13 (centre de contrôle et fermeture du jeu), 6.14 (modules d'optimisation), 6.15 (distribution mondiale). Tout le reste reste valable.

---

## 0. Corrections et vérifications

| Point | Statut | Détail |
| --- | --- | --- |
| Public | **Corrigé** | Le logiciel s'adresse au monde entier, pas à un pays. La v2 évoquait la disponibilité d'Xbox mode « dans le pays de l'utilisateur » : cela devient une **détection à l'exécution**, jamais une hypothèse. |
| Playnite | Vérifié | Licence MIT, mode plein écran à la manette, interface traduisible, thèmes. Sa documentation indique que la personnalisation complète des boutons manette est prévue pour Playnite 11. **Référence principale, réutilisable avec mention de la licence MIT** (compatible GPL v3). |
| Touche Guide (bouton central) en jeu | Vérifié | La lecture passe par une fonction XInput **non documentée** (ordinal 100), utilisée par Chromium, SDL, Godot et PPSSPP. Un projet note que Steam « s'accapare » ce bouton. **Le combo par défaut ne doit donc pas dépendre de Guide.** |
| Combo Back + Start | Vérifié (usage) | Choisi par des extensions Playnite pour garder Guide libre ; fonctionne même avec un jeu Steam en fenêtre. |
| Fermeture forcée d'un jeu | Vérifié (forums) | Les communautés de frontends déconseillent la fermeture brutale (risque de corruption de sauvegarde). **Fermeture douce d'abord, forcée en dernier recours.** |
| PresentMon (Intel) | Vérifié | Licence MIT. Mesure les durées d'images (CPU, GPU, affichage) pour DirectX, OpenGL et Vulkan, via ETW. Demande les droits administrateur ou l'appartenance au groupe « Performance Log Users ». |
| Magpie | Vérifié | GPL v3. Agrandissement de fenêtre (algorithmes FSR, Anime4K, etc.), interface traduite via Weblate. |
| OptiScaler | Vérifié | GPL v3. Remplace l'agrandisseur d'un jeu (DLSS, FSR, XeSS) en interceptant ses appels. **S'injecte dans le jeu : risque avec les anticheats.** |
| GameShift (lhceist41) | Vérifié | MIT, C#, WPF, réversible. Apparu en mars 2026, peu utilisé à la date indexée. **Référence d'idées uniquement.** Certains réglages touchent au registre ou à la résolution du minuteur (exclus en version 1). |
| NotCPUCores | Vérifié | Open source, règle priorité et cœurs. Son auteur dit lui-même que ces outils n'apportent qu'un petit gain. **Licence à vérifier avant toute reprise de code.** |
| Process Lasso | Vérifié | **Propriétaire** (versions gratuite et Pro). Plusieurs dépôts GitHub portant son nom ressemblent à des copies douteuses (l'un parle d'une touche pour ouvrir un « menu trainer »). **Ne rien télécharger ni reprendre de ces dépôts.** |
| ReShade, LibreHardwareMonitor | Non confirmé | Licences non confirmées par mes recherches. **À vérifier avant tout usage.** |
| Avalonia + SkiaSharp | Partiellement vérifié | Avalonia dessine tout via Skia, avec Direct2D possible sous Windows. Une discussion officielle indique qu'Avalonia 12.1 fonctionne avec SkiaSharp 4 à condition de le référencer explicitement. |

---

## 1. Section 6.11 : public mondial et langues

**Principe : aucune chaîne de texte visible dans le code.** Toute phrase affichée vient d'un fichier de traduction. Aucun texte français, anglais ou autre n'est écrit en dur.

### 1.1 Mise en œuvre

| Sujet | Règle |
| --- | --- |
| Langue source | **Anglais** (référence pour les traducteurs). Documentation en anglais et en français au minimum. |
| Format | Fichiers `.resx` par langue (`Strings.fr.resx`, `Strings.ar.resx`...). Il existe des bibliothèques pour WPF et Avalonia qui changent de langue **en direct, sans redémarrer**, avec clés vérifiées à la compilation et signalement des traductions manquantes. **L'agent choisit après un test**, et note sa licence dans `docs/licenses.md`. |
| Langue au premier démarrage | Langue de Windows, modifiable à la manette dans Paramètres, **jamais en saisie de texte obligatoire**. |
| Langues du lancement | Au minimum : anglais, français, **une langue écrite de droite à gauche (arabe)** et **une langue asiatique (chinois simplifié ou japonais)**, pour valider la mise en page. Les autres arrivent par la communauté. |
| Plateforme de traduction | Étudier **Weblate** (utilisé par Magpie) ou Crowdin. **À vérifier :** conditions gratuites pour projets open source. |
| Qualité | Une traduction automatique n'est qu'un brouillon : **relecture par un locuteur natif obligatoire** avant de l'intégrer. Marquer chaque langue « relue » ou « brouillon » dans `docs/languages.md`. |

### 1.2 Pièges à traiter dès la phase 1

1. **Aucune phrase construite par assemblage** (« Jeu » + « en cours »). L'ordre des mots change selon la langue : utiliser des phrases entières avec paramètres.
2. **Pluriels** : prévoir les formes selon la langue (un fichier `.resx` simple ne les gère pas, l'agent doit choisir une solution).
3. **Droite à gauche** : tout l'écran (défilement des jaquettes, menus, icônes de retour) doit se **refléter** en arabe et en hébreu.
4. **Polices** : prévoir une police de secours pour le chinois, le japonais, le coréen, l'arabe, le cyrillique et l'indien. Polices libres de droits uniquement (section 6.4).
5. **Clavier virtuel** : plusieurs dispositions (AZERTY, QWERTY, QWERTZ, cyrillique, arabe), plus un moyen de saisir du japonais, chinois et coréen. À étudier : le clavier virtuel de Windows ou un clavier maison.
6. **Dates, heures, nombres, durées de jeu** : toujours via la culture de l'utilisateur.
7. **Règles de l'optimiseur** : les règles et la liste blanche utilisent le **nom interne** des services et des processus, **jamais le nom affiché**, qui change selon la langue de Windows.
8. **Dossiers Windows** (Démarrage, Documents) : toujours via les API de dossiers connus, jamais par un chemin écrit en dur ou un nom traduit. Prévoir des noms d'utilisateur avec accents ou caractères non latins.
9. **Journaux et messages techniques** : toujours en anglais, pour pouvoir les partager dans les tickets.
10. **Test de pseudo-localisation** : une langue factice avec textes allongés et accents, pour détecter les textes coupés avant d'avoir de vraies traductions.

### 1.3 Matériel et système variés (public mondial)

- Plusieurs écrans, résolutions, mises à l'échelle (DPI), HDR et fréquences d'affichage : l'agent permet de **choisir l'écran** du launcher.
- Éditions de Windows sans winget : prévoir le repli sur l'installateur officiel (section 6.6).
- Boutiques à variantes régionales (par exemple versions locales de clients) : **détecter, ne pas supposer**.
- **Windows 10** : la prise en charge officielle de Microsoft s'est terminée en octobre 2025. **Décision à prendre** : Windows 11 requis, ou Windows 10 en « meilleur effort ».

---

## 2. Section 6.12 : bibliothèque façon console (remplace 6.2, écrans 1 et 2)

L'objectif est l'**ergonomie** d'une console de salon, sans copier d'icône, de logo ni de police de Sony, Microsoft ou Valve (section 6.4).

### 2.1 Écran d'accueil

1. **Rangée horizontale de jaquettes**, le jeu sélectionné s'agrandit et son fond occupe l'écran, en fondu.
2. **Défilement fluide** à la croix directionnelle ou au joystick, avec **inertie** et **répétition** quand la direction est maintenue. Les gâchettes (L1, R1 ou LB, RB) changent de rangée ou de collection.
3. **Premier élément : « Reprendre »** : le dernier jeu joué, avec le temps de jeu et la dernière session.
4. **Rangées par catégorie** : Récents, Favoris, Collections de l'utilisateur, Par boutique, Tous les jeux.
5. **Barre du haut** : heure, état de la manette et de sa batterie (quand disponible), réseau, accès à Recherche, Boutiques, Téléchargements, Paramètres, Alimentation.
6. **Recherche** à la manette avec clavier virtuel (section 1.2), filtres et tri (nom, dernier joué, temps de jeu, boutique).
7. **Performances** : seules les jaquettes visibles et leurs voisines sont chargées en mémoire (virtualisation), à une résolution adaptée à l'écran.

### 2.2 Fiche du jeu

- Jaquette, titre, boutique d'origine, temps de jeu, dernière session, nombre de lancements.
- Bouton **Jouer** (par défaut), **Options** : profil d'optimisation, mode manette (section 2.3 de la v2), arguments de lancement, ajout aux favoris et collections, choix de la boutique si doublon.
- Sans réseau ou sans clé d'API : image de remplacement générée localement (titre sur fond de couleur).

### 2.3 États d'un jeu

Installé, en cours de lancement, en cours, en cours de fermeture, introuvable (dossier supprimé). **Version 1 : jeux installés uniquement** (section 6.7).

### 2.4 Retour sensoriel

Sons de navigation, de validation et d'erreur, vibration légère, **tous réglables et coupables**. Option « réduire les animations » obligatoire (accessibilité).

---

## 3. Section 6.13 : centre de contrôle et fermeture du jeu

Remplace le point « Menu rapide en jeu » de la section 6.5. L'utilisateur le veut dans le logiciel, avec fermeture automatique du jeu.

### 3.1 Ouverture pendant le jeu

| Élément | Règle |
| --- | --- |
| Combo par défaut | **Back + Start maintenus environ 1 seconde** (Share + Options sur DualShock 4). Réglable. |
| Bouton Guide ou PS | **Option, désactivée par défaut** : lecture non documentée, souvent prise par Steam ou la Game Bar. |
| Lecture de la manette hors focus | XInput lit la manette même quand le jeu a le focus. **À vérifier par l'agent.** Une note du code de SDL indique que la variante « XInput sur Windows.Gaming.Input » n'offre pas d'entrée en arrière-plan : ne pas l'utiliser. |
| Mode Focus Input (DualShock 4) | La lecture HID peut échouer si le jeu ouvre la manette de façon exclusive. **À tester.** |
| Mode Steam Input | Steam peut consommer certains boutons. **À tester avec le combo choisi.** |

### 3.2 Contenu du centre de contrôle

Reprendre le jeu, Retourner au launcher (le jeu reste ouvert), **Fermer le jeu**, volume, état de la manette et du mode actif, heure, réseau, quitter vers le bureau, redémarrer, éteindre. Tout se pilote à la manette.

### 3.3 Affichage par-dessus le jeu

- **Fenêtre séparée au premier plan.** **Interdit : injecter une bibliothèque ou accrocher le jeu** (risque de blocage par les anticheats et de sanction sur le compte du joueur).
- Les jeux en **plein écran exclusif** peuvent masquer cette fenêtre. Chaîne de repli, à tester :
  1. fenêtre au premier plan ;
  2. si invisible : vibration et son, puis retour automatique du launcher au premier plan ;
  3. sinon : le combo ramène au launcher.
- **Matrice de test obligatoire** : plein écran sans bordure, plein écran exclusif (DirectX 11, DirectX 12, Vulkan), jeu avec anticheat, jeu en fenêtre.

### 3.4 Fermeture du jeu, étape par étape

1. **Confirmation** à la manette, bouton « Non » sélectionné par défaut, avec avertissement sur la progression non sauvegardée.
2. **Fermeture douce** : demande de fermeture aux fenêtres des processus du jeu (arbre identifié par le `ProcessWatcher`), puis attente (10 secondes par défaut, réglable).
3. Si le jeu est encore là : message « Le jeu ne répond pas » avec **Attendre** ou **Forcer la fermeture**.
4. **Fermeture forcée de l'arbre de processus du jeu uniquement.** Jamais le client du store, sauf réglage explicite (lien avec la section 4.2).
5. **Restauration de la session** d'optimisation (4.4), du mode manette (v2, section 2.4) et des sons. Écran de fin de session.
6. Tout est consigné dans le journal.

**Cas à traiter :** jeu avec processus intermédiaire ou lanceur propre, jeu qui affiche sa propre confirmation, jeu qui se minimise au lieu de se fermer, plantage du jeu pendant la fermeture, deux appuis rapides sur le combo.

**Option :** « Quand je lance un autre jeu, proposer de fermer le jeu en cours ».

---

## 4. Section 6.14 : optimisations modulaires et catalogue open source

Remplace la section 6.9. Chaque optimisation est un **module** :

```text
IOptimizationModule
  Analyze()    : que ferait-il ? (mode simulation)
  Apply()      : applique, après écriture dans session.lock.json
  Revert()     : restaure l'état d'origine exact
  Measure()    : fournit les chiffres avant et après
  Risk, RequiresAdmin, TouchesRegistry, Description (traduite)
```

Chaque module a un interrupteur, un test, une version simulation et une fonction inverse. **Aucun module ne s'active par défaut tant que son gain n'est pas mesuré.**

### 4.1 Mesure du gain réel (condition d'arrêt n° 11 de la v2)

- Intégrer **PresentMon** (MIT) comme outil de mesure : durées d'images médiane, 95e et 99e centile, pendant une session avec puis sans modules. **Il mesure le jeu, pas le launcher.**
- Il demande les droits administrateur ou le groupe « Performance Log Users » : le launcher l'explique et gère le refus sans planter.
- Protocole A/B : trois sessions de même durée et même scène avec les modules, trois sans. Résultats consignés dans `PROGRESS.md` et dans l'écran Analyse. **Le gain peut être nul : c'est une réponse valide.**

### 4.2 Catalogue des outils open source

| Outil | Licence | Rôle | Comment l'utiliser | Verdict |
| --- | --- | --- | --- | --- |
| **PresentMon** | MIT | Mesure des images | Outil de mesure intégré ou lancé en processus séparé | **Oui, version 1** |
| **Playnite** | MIT | Bibliothèque, plein écran, thèmes, traductions | Étudier l'architecture, reprendre du code avec mention MIT | **Oui, référence principale** |
| **DS4Windows (fork actif)** | GPL v3 | Mapping manette | Intégré (v2, section 1) | Oui |
| **Magpie** | GPL v3 | Agrandissement de fenêtre pour jeux en fenêtre ou anciens | **Programme séparé installé par l'utilisateur**, que le launcher détecte et lance par jeu | Option, version ultérieure |
| **OptiScaler** | GPL v3 | Remplacement du DLSS, FSR, XeSS | **Ne pas livrer ni lancer automatiquement** : s'injecte dans le jeu, risque anticheat. Au mieux un lien dans la documentation | Non par défaut |
| **ReShade** | À vérifier | Filtres graphiques | Même risque d'injection | Non par défaut |
| **GameShift** | MIT | Optimiseur réversible (C#) | **Lire pour s'inspirer**, ne pas reprendre ses réglages de registre ni de minuteur | Référence |
| **NotCPUCores** | Open source, licence à vérifier | Priorité et cœurs | Idée de l'interface, après vérification de la licence | Référence |
| **LibreHardwareMonitor** | À vérifier | Capteurs (température, charge) | Pour afficher l'état du système dans le centre de contrôle | À étudier |
| **Process Lasso** | **Propriétaire** | Priorité, cœurs | **Non intégrable.** Se méfier des dépôts GitHub prétendus | Non |

Règle générale : **les outils externes ne sont jamais livrés avec le launcher sans analyse de licence.** Ils sont détectés, présentés par le launcher avec leur site officiel, et lancés **seulement après confirmation** de l'utilisateur.

### 4.3 Modules intégrés de la version 1 (rappel et ajouts)

Ceux de la v2, section 5.1 (mode efficacité EcoQoS, baisse de priorité, pause du rendu du launcher, volume des autres applications, anti-veille, schéma d'alimentation, suspension de processus) auxquels s'ajoute :

| Module | Moyen | Risque |
| --- | --- | --- |
| **Répartition sur les cœurs (CPU Sets)** | API Windows des ensembles de cœurs pour processeurs hybrides | Moyen. **Mesurer d'abord**, version ultérieure |
| **Mise en veille des mises à jour d'applications** | Arrêt temporaire des services (4.3) | Faible, déjà prévu |

### 4.4 Règles d'or

1. **Pas de registre en version 1** (9.3). Les modules qui en ont besoin sont décrits mais désactivés.
2. **Aucun module n'injecte du code dans un jeu.**
3. **Les outils de type « FPS booster » promettent plus qu'ils ne donnent.** L'interface n'affiche **jamais** de promesse chiffrée : seulement les mesures de l'utilisateur.
4. Un mode avancé (registre avec point de restauration, sauvegarde de chaque clé) peut être étudié **plus tard**, seulement si l'utilisateur le décide (question 2).

---

## 5. Graphismes du launcher

| Cadre | État | Remarque |
| --- | --- | --- |
| **WPF** | Choix de départ | Mature, utilisé par Playnite, intégration Windows simple. Rendu moins moderne. |
| **Avalonia (Skia ou Direct2D)** | Candidat | Dessine toute l'interface lui-même, licence ouverte. Les développeurs de Playnite jugeaient l'expérience de développement insuffisante pour leur projet (section 6.9 d'origine). |
| ProGPU, Zenith.NET | **Exclus** | Statut « preview » ou version 0.0.x. |

**Procédure inchangée et obligatoire :** prototype de l'écran d'accueil, **comparaison WPF et Avalonia** avec les mêmes jaquettes, mesure de la fluidité (60 images par seconde), de la mémoire et de **la consommation pendant un jeu**. Le critère décisif est la consommation du launcher **en arrière-plan**, pas son esthétique.

Techniques à appliquer quel que soit le cadre :

- Virtualisation de la rangée de jaquettes, images réduites à la taille d'affichage et mises en cache disque.
- **Pas de fond vidéo en version 1** (coût GPU), option ultérieure.
- **Arrêt complet du rendu** quand un jeu a le focus (v2, section 5.1), reprise au retour.
- Pas d'animation qui tourne sans raison. Option « réduire les animations ».
- Tout est mesuré dans le prototype avant la phase 7.

---

## 6. Section 6.15 : distribution mondiale open source

| Sujet | Règle |
| --- | --- |
| Signature du code | Les exécutables non signés déclenchent SmartScreen et des antivirus, d'autant plus pour un logiciel qui arrête des services. **Étudier la signature gratuite pour projets open source** (par exemple SignPath, dont la liste de projets inclut OptiScaler). **À vérifier :** conditions d'éligibilité. |
| Droits administrateur | **Ne pas lancer toute l'interface en administrateur** : un jeu lancé depuis un processus élevé héritera de ces droits, ce qui est une mauvaise pratique. **Recommandation à valider par un test** : interface sans élévation + petit assistant élevé, uniquement pour arrêter et relancer des services. |
| Dépôt | `README` (anglais et français), `LICENSE`/`COPYING`, `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `docs/languages.md`. |
| Jaquettes | Clés d'API **fournies par chaque utilisateur** (SteamGridDB, IGDB) : ne jamais publier de clé dans le dépôt. |
| Confidentialité | Aucune télémétrie par défaut. Si elle existe un jour : désactivée par défaut, expliquée dans chaque langue. |
| Mises à jour | À étudier. Jamais de mise à jour silencieuse d'un logiciel qui touche au système sans que l'utilisateur le sache. |
| Marques | Aucun nom, logo ni icône de Sony, Microsoft ou Valve dans l'interface ni dans le nom du projet. « Compatible avec » est permis, « officiel » ne l'est pas. |

---

## 7. Changements dans le plan de travail

**Phase 0** : ajouter `CONTRIBUTING.md`, `CODE_OF_CONDUCT.md`, `SECURITY.md`, `docs/languages.md`.

**Phase 1** : ajouter le système de traduction **dès le squelette** (anglais en langue source, pseudo-localisation, test qui échoue si une chaîne est écrite en dur dans une vue).

**Phase 3** : les règles et la liste blanche utilisent les noms internes (section 1.2, point 7), avec un test sur un Windows dont la langue n'est pas l'anglais.

**Phase 7** : bibliothèque façon console (section 2), clavier virtuel multi-dispositions, miroir droite à gauche, comparaison WPF et Avalonia (section 5).

**Nouvelle phase 8 bis : centre de contrôle et fermeture du jeu** (section 3). C'est la phase la plus risquée du projet : elle ne démarre qu'après la validation de la phase 8, avec la matrice de test de la section 3.3 et les replis décrits.

**Phase 8** : modules d'optimisation de la section 4, avec mesure PresentMon.

**Phase 9** : signature du code, relecture juridique des licences, premières langues relues.

**Conditions d'arrêt à ajouter (suite des n° 9 à 11 de la v2) :**

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 12 | L'application démarre et se parcourt entièrement en anglais, français, une langue de droite à gauche et une langue asiatique, sans texte coupé ni chaîne manquante | Test de pseudo-localisation et relecture de captures |
| 13 | Le centre de contrôle s'ouvre ou se replie proprement pour chaque cas de la matrice de la section 3.3 | Résultats dans `PROGRESS.md` |
| 14 | La fermeture du jeu passe les cas de la section 3.4 et restaure session et manette | Test manuel, trois fois de suite |
| 15 | Chaque module de la section 4 a une mesure A/B consignée, les modules sans gain mesuré restent désactivés par défaut | Relecture de `PROGRESS.md` |
| 16 | Aucune interface ni aucun module n'injecte de code dans un jeu | Revue de code et recherche automatique |

---

## 8. Questions ouvertes

1. **Windows 10** : requis Windows 11, ou meilleur effort sur Windows 10 ?
2. **Mode avancé avec registre** (point de restauration, sauvegarde de chaque clé) : on l'étudie pour une version ultérieure, ou on l'exclut définitivement ?
3. Les **jaquettes** : ta propre clé d'API pour tester, et chaque utilisateur la sienne ensuite. D'accord ?
4. Les **langues du lancement** : anglais et français garantis, arabe et chinois ou japonais pour valider la mise en page, le reste par la communauté. D'accord ?
