# Focus Launcher : correctif v4.4 (toutes les langues, Claude Code et skills, installation depuis le lanceur)

Ce correctif **complète** la v4.3. En cas de conflit, **ce fichier prime** sur la v4.3. Il change trois choses : **toutes les langues deviennent une exigence** (section 1), le projet est **développé avec Claude Code, avec des skills** (section 2), et les logiciels sociaux **s'installent depuis le lanceur**, comme les boutiques de jeux (section 3).

---

## 1. Toutes les langues : une exigence, pas un objectif

Cette section **remplace la ligne « Honnêteté sur toutes les langues » de la v4.3 (section 4)**. Le français et l'anglais complets « à la livraison », avec les autres « ensuite », ne suffit plus.

**Règle :** le launcher **ne sort pas** tant que **chaque langue de la liste de référence** n'est pas présente et complète.

| Point | Décision |
| --- | --- |
| Liste de référence | **Toutes les langues d'affichage de l'OS visé**, plus toute langue demandée ensuite. L'agent la fige dans `docs/langues.md` au début du projet, avec le code de chaque langue |
| « Complète » | **100 % des clés** de traduction présentes, aucune clé brute ni texte anglais de repli à l'écran. Vérifié par un test automatique, langue par langue |
| Qualité | Chaque langue porte un statut dans `docs/langues.md` : **validée** (relue par une personne compétente) ou **à relire** (traduction produite par l'agent, non relue). Ce statut est **visible dans les Paramètres**, pas sur l'écran de premier choix. Aucune traduction non relue n'est présentée comme validée |
| Écritures | Polices couvrant **tous les alphabets** de la liste (latin, cyrillique, grec, arabe, hébreu, chinois, japonais, coréen, indiens, thaï…). Droite-gauche pris en charge. Test de rendu sur une capture interne par écriture, **non livrée** |
| Saisie à la manette | Clavier à l'écran avec **au moins une disposition par écriture**, à défaut la disposition du clavier de l'OS |
| Ajout d'une langue | Un fichier, **aucun changement de code** (v4.3). Le skill `traduction-langues` (section 2) encadre le travail |

**Franchise sur la qualité :** « toutes les langues » est tenable pour la **présence** et la **complétude**. La **qualité** d'une traduction produite sans relecteur humain ne peut pas être garantie. D'où le statut « à relire », qui permet à la communauté de corriger sans bloquer la sortie.

---

## 2. Travailler avec Claude Code, et fournir des skills

L'agent de développement est **Claude Code**. L'utilisateur lui fournit des **skills**, c'est-à-dire des dossiers d'instructions que Claude Code charge quand la tâche correspond.

| Point | Décision |
| --- | --- |
| Emplacement | **`.claude/skills/<nom>/SKILL.md`** dans le dépôt (skills de projet), un dossier par skill. Les fichiers annexes (modèles, scripts, exemples) vont dans le même dossier |
| Format | `SKILL.md` commence par un en-tête avec **`name`** et **`description`**. La description dit **quand** utiliser le skill, avec les mots que l'utilisateur emploierait. Chaque `SKILL.md` reste **sous 500 lignes** : le détail va dans des fichiers annexes |
| `CLAUDE.md` | Fichier à la racine, **toujours chargé**, qui contient seulement les règles valables partout : la règle en une phrase (v4.2, section 1), les termes interdits, « tout aux trois entrées » (v4.3), « toutes les langues » (section 1), et la liste des skills. Le reste va dans les skills |
| Avant chaque tâche | L'agent **lit le skill correspondant** avant de commencer, et note dans `PROGRESS.md` quels skills il a utilisés |
| Skills de l'utilisateur | L'agent **ne les modifie ni ne les supprime jamais**. S'ils contredisent un fichier de correctif, l'agent le note dans `PROGRESS.md` sous « à trancher », continue avec l'interprétation **la moins risquable**, et ne bloque pas le travail |
| Effets de bord | Les skills qui **modifient le système** (installation d'une application, entrée de démarrage de session) sont marqués **`disable-model-invocation: true`** : l'agent ne les déclenche pas de lui-même, l'utilisateur les appelle |

**Skills proposés** (l'utilisateur peut les fournir, les adapter ou les remplacer) :

| Skill | Sert à |
| --- | --- |
| `traduction-langues` | Ajouter, compléter et vérifier une langue : clés, ICU, droite-gauche, statut « à relire », mise à jour de `docs/langues.md` |
| `termes-constructeurs` | Liste des termes interdits (v4.2, section 4) et lancement du test automatique sur les fichiers de langue |
| `revue-originalite` | Remplir `docs/originality-review.md` : repris, différent, jugement, **sans jamais livrer de capture** |
| `trois-entrees` | Liste de contrôle manette, souris, clavier pour chaque écran, et test de navigation |
| `catalogue-installation` | Ajouter une application au catalogue (section 3) et vérifier sa source officielle. **`disable-model-invocation: true`** pour les essais d'installation réels |
| `demarrage-session` | Mettre en place et tester le lancement à l'ouverture de session, y compris après redémarrage. **`disable-model-invocation: true`** |
| `suivi-progress` | Format de `PROGRESS.md` et vérification des conditions d'arrêt |

---

## 3. Installer les logiciels sociaux depuis le lanceur

**Principe :** Discord et les autres applications sociales s'installent **comme les boutiques de jeux** (Steam, Epic Games, Electronic Arts, et de la même façon GOG, Battle.net, Ubisoft Connect), **depuis le launcher**, sans quitter l'interface.

1. **Un seul mécanisme** pour les boutiques et les logiciels sociaux : un **catalogue** d'applications installables.
2. **Affichage :** une application du catalogue **non installée** apparaît comme une entrée « Installer » dans son filtre (**Social** ou **Jeux**) et dans la fonction **Boutiques**. Une fois installée, elle devient une entrée normale de la bibliothèque, avec « Ouvrir ».
3. **Source, dans cet ordre :** le **gestionnaire de paquets de l'OS** (sous Windows, winget) ; à défaut, la **page de téléchargement officielle de l'éditeur**, ouverte dans le navigateur. **Jamais** de miroir, de site tiers ni d'installeur livré avec le launcher.
4. **Confirmation avant toute installation :** nom, **éditeur**, source, taille si connue. Annulable.
5. **Progression** dans la section **Téléchargements** du launcher, avec message clair en cas d'échec et possibilité de réessayer ou d'ouvrir la page officielle.
6. **Autorisation de l'OS :** une installation peut déclencher la fenêtre d'autorisation du système, que le launcher **ne contrôle pas** et qui peut **ne pas répondre à la manette**. Le launcher **prévient avant** : « utilise le clavier ou la souris pour autoriser ».
7. **Catalogue :** fichier de données dans le dépôt, maintenu avec le projet. Pour chaque application : identifiant, nom, catégorie, identifiant du gestionnaire de paquets, page officielle, éditeur attendu. **L'agent vérifie chaque identifiant** avant de l'inscrire et ne l'invente jamais. Pas d'ajout de sources par l'utilisateur en version 1 (sécurité).
8. **Twitch** s'utilise surtout par le web : il reste une **entrée de type site web** (v4.3). L'agent vérifie s'il existe un installeur officiel à jour avant de l'ajouter au catalogue.
9. **Jamais** : publicité, mise en avant payante, lien d'affiliation, classement commercial.
10. **Hors périmètre :** installer les **jeux** eux-mêmes (c'est le rôle de chaque boutique) et désinstaller (renvoi vers les réglages du système).

---

## 4. Plan de travail et conditions d'arrêt

**Phase 0 (avant tout le reste) :** créer `CLAUDE.md`, le dossier `.claude/skills/` et les skills de la section 2 que l'utilisateur a fournis.
**Phase 8 ter :** inclut maintenant **toutes** les langues de `docs/langues.md`.
**Phase 8 quater :** catalogue et installation depuis le launcher (section 3).

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 30 (**remplace** celle de la v4.3) | **Chaque langue de `docs/langues.md`** est présente avec 100 % des clés, le choix de langue fonctionne aux trois entrées, et un test de rendu couvre chaque écriture et le droite-gauche | Test automatique par langue + captures internes non livrées |
| 33 | `docs/langues.md` liste **toutes** les langues de référence avec leur statut (validée ou à relire), et le statut est visible dans les Paramètres | Lecture du fichier et de l'écran |
| 34 | `CLAUDE.md` et `.claude/skills/` existent, chaque `SKILL.md` a `name` et `description` et fait **moins de 500 lignes**, et `PROGRESS.md` note les skills utilisés | Lecture des fichiers |
| 35 | L'installation depuis le launcher a été testée avec **au moins deux applications par le gestionnaire de paquets et une par la page officielle**, avec confirmation, progression et échec simulé | Résultats notés dans `PROGRESS.md` |
| 36 | **Aucun installeur** n'est dans le dépôt, et chaque source du catalogue est **officielle et vérifiée** | Lecture du catalogue et de la liste des fichiers |

---

## 5. À confirmer

Rien de nouveau. La plateforme supposée reste **Windows** (v4.3, section 7). Si elle change, seuls le mécanisme de démarrage (v4.3, section 2) et le gestionnaire de paquets (section 3) changent.

*Rappel juridique : consigne de prudence, pas un avis. Avant la première publication, faire relire apparence, noms, sons et usage des marques (y compris les noms Discord, Steam, Epic Games, Electronic Arts dans le catalogue) par un juriste.*
