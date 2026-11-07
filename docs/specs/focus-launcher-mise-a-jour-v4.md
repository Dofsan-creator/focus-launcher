# Focus Launcher : mise à jour v4 (bibliothèque unifiée, design original, manette intégrale, audit par l'agent)

Ce fichier complète les v2 et v3. Il **remplace** : la section 6.7 (chargement des bibliothèques), la section 6.4 (inspirations et direction artistique) pour tout ce qui touche à l'apparence, et il **ajoute** : 6.7 bis (recherche d'exécutables), 6.3 ter (manette intégrale), 9.5 (mandat d'audit de l'agent de code).

Les mots du brief ont été compris ainsi : « Ping » = Epic Games, « équitables » = exécutables, « loupe » = launcher (la loupe est aussi une piste de design, voir section 3), « NBR code » = l'agent de code.

---

## 0. Ce qui a été vérifié

| Point | Statut | Détail |
| --- | --- | --- |
| Détection locale des jeux installés par boutique | Vérifié (plusieurs projets open source) | Sources : voir tableau de la section 1.1. **Les formats changent sans préavis** : chaque scanner reste isolé et testé. |
| Playnite et ses extensions de bibliothèque | Vérifié | Licence MIT. Intégrations pour Steam, Epic, GOG, Origin et EA, Ubisoft Connect, Battle.net, Xbox, Amazon, itch.io, Humble. **Meilleure référence de code**, réutilisable avec la mention de la licence MIT (compatible GPL v3). |
| Installer un jeu Steam par commande | Vérifié | `steam://install/<id>` est documenté par Valve. `steam://run/<id>` installe le jeu si nécessaire. Valve précise que certaines commandes ne fonctionnent plus et que la liste est incomplète. |
| Liste des jeux possédés Steam | Vérifié, avec limites | L'interface web de Steam (`GetOwnedGames`) demande **une clé d'API** et **un profil dont les détails de jeux sont publics**. Des utilisateurs signalent des réponses vides malgré un profil public. La clé ne doit pas être partagée. |
| Jeux possédés des autres boutiques | Vérifié, avec limites | Playnite précise qu'une connexion au compte est nécessaire pour importer toute la bibliothèque de GOG, Origin et Battle.net. |
| Installer Epic, GOG, Amazon sans leur client | Vérifié (existence) | Heroic (GPL v3) s'appuie sur Legendary (Epic), gogdl (GOG) et Nile (Amazon). **Un site de comparaison signale Legendary comme abandonné** : l'état actuel est à vérifier. Ce sont des clients non officiels : risque vis-à-vis des conditions des boutiques. |
| Installer un jeu Epic par le client Epic | Non confirmé | Le protocole de **lancement** est connu. **Le protocole d'installation n'a pas été retrouvé** : à vérifier par l'agent. |
| Jeux Xbox et Microsoft Store | Vérifié | Les jeux peuvent être dans un dossier protégé (`WindowsApps`) ou, avec l'application Xbox récente, dans un dossier choisi (souvent `XboxGames`). **Ne jamais prendre possession du dossier protégé ni copier l'exécutable** : le jeu se lance par le Menu Démarrer ou l'application Xbox. |

---

## 1. Section 6.7 réécrite : bibliothèque unifiée

L'objectif : **ouvrir le launcher et voir tous ses jeux, sans rien faire**. Quatre niveaux, du plus automatique au plus manuel.

### 1.1 Niveau A : détection automatique des jeux installés

| Boutique | Source locale (lecture seule) | Lancement | À vérifier par l'agent |
| --- | --- | --- | --- |
| Steam | Dossier Steam trouvé dans le registre, `libraryfolders.vdf` (toutes les bibliothèques, tous les disques), `appmanifest_*.acf` | `steam://rungameid/<id>` | Filtrer les outils (redistribuables, Spacewar, SDK) |
| Epic Games | `LauncherInstalled.dat` (JSON) en priorité, sinon `Manifests\*.item` | `com.epicgames.launcher://apps/<id>?action=launch&silent=true` | Exclure moteur, plugins, éditeurs, DLC |
| GOG | Registre (clés `GOG.com\Games`), base de GOG Galaxy si présente, fichiers `goggame-<id>.info` | Exécutable principal du jeu | Exclure les DLC |
| EA App | Registre et fichiers de configuration locaux | Protocole de l'EA App | **Format instable**, repli sur un dossier d'installation choisi par l'utilisateur |
| Ubisoft Connect | Registre (clés `Ubisoft\Launcher\Installs`, vues 32 et 64 bits) | `uplay://launch/<id>` | Format des fichiers de configuration |
| Battle.net | Base `product.db` de l'agent Battle.net (format binaire) ou clés de désinstallation du registre | `battlenet://<id>` | Exclure les entrées internes |
| Xbox et Microsoft Store | Registre des applications empaquetées, `appxmanifest.xml` ou `MicrosoftGame.config`, dossier `XboxGames` | Menu Démarrer ou application Xbox (**jamais l'exécutable copié**) | Variantes selon l'édition de Windows |
| Autres (itch.io, Amazon, Humble) | Version ultérieure | | |

**Règles :**

1. **Lecture seule.** Aucun fichier ni clé de boutique n'est modifié.
2. **Un scanner par boutique**, isolé, avec un test sur de faux dossiers, et un **échec propre** : si un format change, le launcher l'indique et propose le niveau B.
3. **Analyse automatique** au démarrage, **en arrière-plan** (l'écran d'accueil s'ouvre tout de suite et les jeux apparaissent au fil de la détection), puis surveillance des dossiers de manifestes pour voir un nouveau jeu installé **sans redémarrer**.
4. **Un interrupteur par boutique** dans Paramètres, tout à la manette.
5. **Dossiers analysés choisis par l'utilisateur**, en plus des dossiers connus, sur **tous les disques**.
6. **Fusion des doublons** : un jeu présent dans deux boutiques apparaît une fois, avec choix de la source au lancement.
7. **Jeux installés par le launcher lui-même** : dès qu'une installation se termine, le jeu apparaît sans action.
8. Les noms internes sont utilisés pour reconnaître les jeux, pas les noms affichés (section 1.2 de la v3).
9. **Ne jamais parcourir de force le dossier protégé `WindowsApps`.**

### 1.2 Niveau B : ajout manuel et recherche d'exécutables

Voir la section 2 de ce fichier.

### 1.3 Niveau C : bibliothèque complète (jeux possédés, pas encore installés)

**Option activée par l'utilisateur, jamais par défaut.** Les jeux non installés apparaissent grisés avec un bouton **Installer**.

| Boutique | Méthode | Version |
| --- | --- | --- |
| Steam | Interface web de Steam avec **la clé d'API de l'utilisateur** et son identifiant Steam. **Prérequis affichés clairement** : profil et détails des jeux publics. Un mode « hors ligne » reste possible sans clé | **1.1**, après la version 1 |
| Epic, GOG, Amazon | Connexion au compte. À étudier avec les outils de Heroic (GPL v3, compatible) | **Ultérieure**, après analyse des conditions des boutiques |
| EA, Ubisoft, Battle.net | Aucune méthode stable trouvée | Ultérieure |

**Règles :**

- La clé d'API est **fournie par chaque utilisateur** et stockée localement. **Jamais dans le dépôt, jamais envoyée ailleurs.** Le launcher ne stocke aucun mot de passe de boutique (6.6).
- **Saisie de la clé à la manette** : bouton « Coller depuis le presse-papiers », import depuis un fichier de configuration, ou clavier virtuel. Le QR code vers le téléphone est **écarté en version 1** : il demande un petit serveur réseau local, donc une surface d'attaque.
- Un échec de lecture (réponse vide, profil privé) donne un message clair qui explique comment corriger, pas une liste vide silencieuse.
- La liste des jeux possédés est **stockée localement**, avec un bouton « Tout effacer ».

### 1.4 Niveau D : installer depuis le launcher

| Boutique | Méthode version 1.1 | Remarque |
| --- | --- | --- |
| Steam | `steam://install/<id>` : Steam télécharge en arrière-plan réduit (6.5) | Les commandes de Valve ne sont pas toutes garanties : **tester** |
| Epic, GOG, EA, Ubisoft, Battle.net, Xbox | Ouvrir la page du jeu dans le client de la boutique, en arrière-plan réduit | Protocole d'installation d'Epic **à vérifier** |
| Epic, GOG, Amazon sans leur client | **Ultérieure**, avec les outils de Heroic | Clients non officiels : l'utilisateur est prévenu |

- **Confirmation à la manette** avant toute installation, avec le nom du jeu et sa boutique.
- **Progression** : lecture de l'état de téléchargement quand la boutique le permet (par exemple l'avancement noté dans les fichiers de manifestes de Steam, **à vérifier**), sinon un indicateur « en cours dans le client ».
- À la fin : le niveau A détecte le jeu et il apparaît tout seul.
- Le choix du **dossier et du disque** d'installation est proposé quand la boutique le permet.

---

## 2. Section 6.7 bis : recherche d'exécutables et raccourci automatique

Pour un jeu qu'aucune boutique ne déclare (jeu sans installateur, copie portable, ancien jeu), tout se fait **à la manette**, **sans l'explorateur de Windows**.

### 2.1 Parcours

1. **Ajouter un jeu** → **Chercher dans un dossier** ou **Chercher sur tout un disque**.
2. **Explorateur intégré** au launcher, piloté à la manette (disques, dossiers, retour avec le bouton B).
3. Le launcher analyse **sans rien exécuter** : il lit seulement les informations du fichier (nom du produit, éditeur, taille, signature).
4. Il propose **les trois exécutables les plus probables**, avec icône et chemin.
5. L'utilisateur valide d'un appui, ou choisit un autre fichier.
6. Le launcher **crée la fiche du jeu et le raccourci**, puis ajoute la jaquette.

### 2.2 Choix de l'exécutable : un score, pas une règle fixe

Le score favorise :

- un nom proche de celui du dossier ;
- l'exécutable le plus volumineux du dossier ;
- les structures connues : un exécutable à côté d'un dossier `<nom>_Data` (jeux Unity), un exécutable dans `Binaries\Win64` (jeux Unreal) ;
- un nom de produit lisible dans les informations du fichier ;
- une version 64 bits ;
- une application à interface graphique plutôt qu'en console.

Le score pénalise (**sans jamais exclure** : certains jeux s'appellent vraiment `Launcher.exe`) : `unins*`, `setup*`, `install*`, `vc_redist*`, `dxsetup*`, gestionnaires de plantage (`UnityCrashHandler*`, `CrashReportClient*`), services d'anticheat (`EasyAntiCheat*`, `BEService*`).

Limites : profondeur de recherche réglable (3 niveaux par défaut), recherche **annulable**, aucun parcours du dossier protégé `WindowsApps`.

### 2.3 Raccourci automatique

| Raccourci | Défaut | Détail |
| --- | --- | --- |
| Fiche dans le launcher | **Toujours** | Titre, jaquette, chemin, dossier de travail, arguments |
| Raccourci Windows (`.lnk`) dans un dossier « Focus Launcher » du Menu Démarrer | **Proposé** | L'utilisateur choisit. Le raccourci lance le jeu **par le launcher** pour profiter du mode concentration |
| Raccourci sur le Bureau | Désactivé | À activer par l'utilisateur |
| Ajout dans Steam comme jeu non-Steam | **Interdit** | Modifierait des fichiers de Steam (6.7, lecture seule) |

Le titre est proposé automatiquement (informations du fichier ou nom du dossier) et se corrige au clavier virtuel. Recherche de jaquette et gestion hors ligne : voir 6.15 de la v3.

---

## 3. Section 6.4 réécrite : une identité visuelle originale

**Règle d'or :** le launcher reprend les **conventions d'usage** des consoles (un élément sélectionné, un bouton Retour, une navigation prévisible), mais **aucun élément d'apparence** d'un autre launcher. Un principe d'ergonomie est libre. Une mise en scène graphique précise est protégée.

### 3.1 Ce qui doit être original

| Élément | Exigence |
| --- | --- |
| **Silhouette de l'écran d'accueil** | Pas une simple rangée de tuiles droites avec fond flou. La façon dont les jeux sont disposés et défilent est propre à Focus Launcher |
| Mouvement | Propre courbe d'animation, propre effet de sélection, propre transition de lancement |
| Icônes | Dessinées pour le projet, y compris les icônes de boutons |
| Palette et typographie | Propres au projet, polices sous licence libre (OFL) listées dans `docs/licenses.md` |
| Sons | Créés pour le projet ou sous licence libre (CC0 ou équivalent) |
| Noms des écrans | Propres au projet |
| Animation de démarrage | Propre au projet |

### 3.2 Trois pistes, toutes tirées du nom « Focus »

Le nom invite à une métaphore photographique. C'est un **fil conducteur original**, utile pour que tout le reste cohère.

| Piste | Idée | Défilement | Attention |
| --- | --- | --- | --- |
| **A. Lentille** | Les jaquettes flottent sur un fond sombre. Une **loupe circulaire** suit la sélection, l'agrandit et l'affine. Les autres restent légèrement en retrait | Le joystick déplace la loupe, la croix directionnelle saute de jeu en jeu | Le flou est coûteux pour le GPU : **repli sans flou** (opacité et taille) à mesurer |
| **B. Iris** | Les jeux sont disposés sur un **arc**. Chaque transition ouvre ou ferme un **diaphragme**. Un anneau d'ouverture entoure le jeu choisi | Défilement le long de l'arc, avec inertie | L'arc doit rester lisible avec beaucoup de jeux : prévoir des collections |
| **C. Pellicule** | Les jeux sont des **images de pellicule** avec numéros de vue. Le sélecteur est un **viseur** en coins. Le défilement évoque une pellicule qu'on fait avancer, avec un petit clic de vibration à chaque image | Rangées horizontales | Garder des repères clairs pour ne pas se perdre dans une grande bibliothèque |

Ces trois pistes **conservent le défilement horizontal des jeux** que tu veux, avec un habillage propre.

### 3.3 Procédure obligatoire (avant les vues de la phase 7)

1. L'agent produit **une maquette de l'écran d'accueil par piste** (images fixes) et te les présente.
2. **Tu choisis** une piste, ou un mélange, et tu demandes les ajustements.
3. L'agent écrit `docs/design.md` : palette, polices, tailles, courbes d'animation, jeu d'icônes, sons.
4. **Revue d'originalité** : l'agent compare, **pour son seul contrôle interne**, avec des captures de Playnite en plein écran, de Steam Big Picture, de l'interface Xbox et de celle de la PlayStation 5. Il note les différences dans `docs/originality-review.md`. **Ces captures ne sont ni livrées ni décalquées.**
5. Si la comparaison montre un air de famille trop fort (même disposition, mêmes mouvements), **il modifie son travail** et recommence.
6. **Le nom « Focus Launcher »** : avant toute publication, vérifier qu'aucun logiciel connu ne porte un nom trop proche. Pas d'hypothèse.

---

## 4. Section 6.3 ter : manette intégrale

**Aucune fonction du launcher ne doit demander un clavier ou une souris.** Le clavier et la souris restent un secours, jamais un passage obligé.

### 4.1 Tout ce qui doit être possible à la manette

Parcourir et rechercher la bibliothèque, lancer et **fermer** un jeu, régler le son et la langue, **choisir un dossier et un exécutable** (explorateur intégré, section 2), **saisir du texte** (clavier virtuel multi-dispositions), **saisir ou coller la clé d'API**, installer une boutique ou un jeu, **confirmer ou refuser les demandes** (droits administrateur, installation, fermeture forcée), **lire et fermer les messages d'erreur**, régler tous les paramètres, quitter, redémarrer, éteindre, **assistant du premier démarrage**.

Avec la manette débranchée : message clair, clavier et souris prennent le relais, retour automatique à la manette dès sa reconnexion.

### 4.2 Règles de conception

- **Jamais de piège de focus** : à tout moment, un bouton permet de sortir (Retour), et **chaque élément cliquable est atteignable** depuis n'importe quel autre.
- Les actions **destructrices ou irréversibles** (fermeture forcée, effacement) demandent un **maintien du bouton** ou une confirmation avec « Non » présélectionné.
- Les boîtes de dialogue de Windows (droits administrateur, explorateur de fichiers) **ne sont pas pilotables à la manette** : le launcher prévient avant de les ouvrir, et son propre explorateur remplace celui de Windows. Pour le contrôle de compte Windows, voir 6.15 de la v3 (assistant séparé).
- Mêmes actions, mêmes boutons, **partout**.

### 4.3 Test automatique obligatoire

L'agent écrit un test `FocusGraphTests` qui, pour chaque vue :

1. envoie des actions simulées (Haut, Bas, Gauche, Droite, Valider, Retour, Options) à la couche `InputActions` ;
2. vérifie que **tous les éléments interactifs sont atteignables** en un nombre raisonnable de pressions ;
3. vérifie que **Retour sort toujours** de la vue ;
4. vérifie l'**absence de piège de focus** et d'élément inaccessible ;
5. s'exécute **dans chaque langue de test, y compris celle de droite à gauche**.

En plus : une **liste de contrôle manuelle**, à faire par l'utilisateur **sans toucher au clavier ni à la souris**, de l'ouverture du launcher à la fermeture d'un jeu.

---

## 5. Section 9.5 : mandat d'audit et d'amélioration de l'agent de code

**Nouveau.** L'agent de code n'est pas qu'un exécutant. Il est responsable de la **qualité de l'ensemble**.

### 5.1 Principe

Avant de commencer, **après chaque phase**, et **avant la fin**, l'agent fait une **ronde d'audit** : il analyse le projet comme un relecteur exigeant, repère **ce qui manque, ce qui bloque, ce qui est imparfait**, puis **l'ajoute et l'améliore**. Un point oublié par ce document **n'est pas une excuse** pour ne pas le traiter.

### 5.2 Le fichier AUDIT.md

Tenu à la racine, une ligne par constat : date, phase, **gravité** (critique, haute, moyenne, basse), description, **décision** (corrigé, reporté avec raison, à demander à l'utilisateur), commit associé.

### 5.3 Liste de contrôle de la ronde d'audit

| Domaine | Questions |
| --- | --- |
| Compilation et tests | Tout passe ? Des chemins du code ne sont-ils pas testés ? |
| Sécurité du système | La liste blanche est-elle respectée partout ? Toute action a-t-elle sa restauration ? |
| Récupération | Que se passe-t-il si tout plante à cet instant précis ? |
| Boutiques | Chaque scanner échoue-t-il proprement ? |
| Manette | `FocusGraphTests` passe ? Existe-t-il un écran qui demande un clavier ? |
| Langues | Reste-t-il du texte en dur ? Le miroir droite à gauche tient-il ? |
| Performances | Les budgets sont-ils tenus (60 images par seconde, mémoire, consommation pendant un jeu) ? |
| Confidentialité | Aucun identifiant, clé ou chemin personnel ne sort de l'ordinateur ni du dépôt ? |
| Licences | Chaque dépendance et chaque fichier repris est-il listé avec sa licence ? |
| Design | La revue d'originalité est-elle à jour ? |
| Documentation | Un nouvel utilisateur peut-il installer et utiliser sans aide ? |
| Livraison | Installation et désinstallation propres ? Signature du code ? |
| Ce document | Une section est-elle contradictoire, dépassée ou incomplète ? **Le signaler dans `AUDIT.md`**, appliquer la règle la plus prudente, et demander. |

### 5.4 Ce que l'agent peut ajouter seul, et ce qu'il doit demander

| L'agent le fait seul | L'agent demande d'abord |
| --- | --- |
| Tests manquants, gestion d'erreurs, journalisation | Toute action qui **modifie le système** (déjà listé en 7 ter) |
| Documentation, traductions manquantes en anglais | Une **nouvelle dépendance** ou une licence incertaine |
| Corrections de bugs et de performances mesurées | Un **élargissement du périmètre** (nouvelle fonction non prévue) |
| Accessibilité et robustesse de la manette | Toute décision de **design** visible par l'utilisateur |
| Nettoyage du code, noms, structure | Tout ce qui **contredit** ce document |

### 5.5 Garde-fous

- **Une amélioration n'affaiblit jamais une règle de sécurité** (liste blanche, restauration, lecture seule, pas de registre, pas d'injection dans un jeu).
- Les garde-fous de la boucle (5 tentatives, aucun test supprimé ou contourné) **restent en vigueur**.
- **Chaque ajout est consigné** dans `PROGRESS.md` et `AUDIT.md`, avec un commit séparé.
- **Revue à froid** : à la fin des phases 8 bis et 9, l'utilisateur peut lancer une **nouvelle session** de l'agent, sans l'historique, avec pour seule consigne de lire le dépôt et ce document, et d'écrire un rapport de ce qui manque. Les écarts entrent dans `AUDIT.md`.

---

## 6. Changements dans le plan de travail

**Phase 6** : niveau A de la section 1 (toutes les boutiques du tableau, chacune isolée et testée), recherche d'exécutables et raccourcis (section 2), installation d'un jeu Steam (section 1.4). Le niveau C (jeux possédés) devient la **phase 6 bis**, après la version 1.

**Phase 7** : revue des trois pistes de design et maquettes **avant** les vues (section 3.3), explorateur intégré et clavier virtuel, `FocusGraphTests` (section 4.3).

**Phase 9 et fin de chaque phase** : ronde d'audit (section 5).

**Conditions d'arrêt à ajouter (suite des n° 12 à 16 de la v3) :**

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 17 | Après un lancement à froid, les jeux installés de chaque boutique présente apparaissent **sans action** de l'utilisateur | Test sur le PC réel, résultats dans `PROGRESS.md` |
| 18 | Un jeu absent de toutes les boutiques s'ajoute **à la manette seule**, avec fiche et raccourci | Test manuel de l'utilisateur |
| 19 | `FocusGraphTests` passe pour chaque vue et chaque langue de test | `dotnet test` |
| 20 | La revue d'originalité est faite et validée par l'utilisateur | Lecture de `docs/originality-review.md` |
| 21 | `AUDIT.md` ne contient **aucun point critique ou haut ouvert** | Relecture du fichier |

---

## 7. Questions ouvertes

1. **Piste de design** : Lentille, Iris, Pellicule, ou un mélange ? (Ou aucune des trois : décris-moi ce que tu veux ressentir.)
2. **Jeux possédés Steam** : tu acceptes le principe d'une clé d'API fournie par chaque utilisateur, avec les prérequis de profil public ?
3. **Epic, GOG, Amazon sans leur client** : on étudie les outils de Heroic pour une version ultérieure, malgré le risque vis-à-vis des conditions des boutiques ?
4. **Raccourci Windows** : proposé par défaut dans le Menu Démarrer, comme prévu en 2.3 ?
