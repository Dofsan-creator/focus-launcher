# Focus Launcher : correctif v4.1 (jeux non Steam et logiciels)

Ce correctif **remplace la section 2 de la v4** (recherche d'exécutables et raccourci automatique) et **modifie l'objectif du projet** : Focus Launcher accueille désormais **des jeux et des logiciels** (par exemple de la 3D), avec les optimisations adaptées à chacun.

---

## 1. Changement de périmètre

La section 1 du document principal parle d'un lanceur de **jeux**. Elle devient : *un lanceur plein écran, piloté à la manette, pour les jeux et les logiciels de l'utilisateur, avec un mode d'optimisation adapté à chaque type d'usage.*

Conséquences :

- Une fiche s'appelle **« entrée »**, pas « jeu », dans le code et dans les textes traduits (section 1.2 de la v3).
- Chaque entrée a un **type** : **Jeu**, **Logiciel**, **Autre**.
- L'accueil propose un filtre ou une rangée **« Logiciels »**, en plus de la bibliothèque de jeux.
- **Le type Logiciel n'est pas une bibliothèque de boutique** : il n'y a pas de jaquette officielle, l'icône de l'exécutable est utilisée (ou une image choisie par l'utilisateur).

---

## 2. Section 2 de la v4 remplacée : ajout direct d'un exécutable

### 2.1 Deux façons d'ajouter, toutes les deux à la manette

| Méthode | Quand | Parcours |
| --- | --- | --- |
| **A. Choisir un fichier** (principale) | L'utilisateur sait quel fichier lancer : jeu non Steam, copie portable, logiciel 3D, outil | **Ajouter** → explorateur intégré → il **sélectionne l'exécutable** d'un appui → la fiche et le raccourci sont créés **tout de suite** |
| **B. Chercher dans un dossier** (assistée) | Il connaît le dossier, pas le bon fichier | Même explorateur, le launcher propose **les trois exécutables les plus probables** (score de la v4, section 2.2) |

Fichiers acceptés : `.exe`, raccourcis Windows `.lnk`, `.bat` et `.cmd`. Les applications du Microsoft Store se choisissent dans la liste des applications installées (pas par leur fichier, protégé).

**Pour la méthode A, aucun score ni aucune déduction** : le fichier choisi par l'utilisateur est le bon. Le score ne sert qu'à la méthode B.

### 2.2 Ce que le launcher fait à la sélection

1. **Lit seulement les informations du fichier** (nom du produit, éditeur, version, icône). **Il ne l'exécute jamais pour l'inspecter.**
2. Propose un **titre** (nom du produit, sinon nom du dossier), modifiable au clavier virtuel.
3. Demande le **type** (Jeu, Logiciel, Autre) : trois choix à la manette. Le type pilote le profil d'optimisation par défaut (section 3).
4. Enregistre le **dossier de travail** (celui de l'exécutable) et les **arguments** éventuels.
5. Extrait l'**icône** ; pour un jeu, cherche en plus la jaquette (6.15 de la v3).
6. **Crée la fiche dans le launcher** : elle est **accessible immédiatement** depuis l'accueil, sans redémarrage.
7. Propose le raccourci Windows (`.lnk` dans le Menu Démarrer), **désactivé tant que l'utilisateur ne l'accepte pas** (v4, section 2.3).

### 2.3 Ajout par lot

« Ajouter plusieurs logiciels » : l'utilisateur coche plusieurs fichiers dans l'explorateur, tous sont ajoutés en une fois, chacun avec son type.

### 2.4 Règles de sécurité

- Un fichier `.bat` ou `.cmd` est affiché **en clair** (chemin complet) avant d'être enregistré.
- **Jamais d'exécution** d'un fichier pour deviner ce qu'il est.
- Pas de parcours du dossier protégé `WindowsApps`.
- Les chemins et noms restent **locaux** : rien n'est envoyé nulle part.

---

## 3. Profils d'optimisation par type d'entrée

**Un logiciel de 3D n'est pas un jeu.** Fermer des processus pendant un rendu ou une sauvegarde peut **faire perdre du travail**. Le profil par défaut dépend donc du type.

| | **Jeu** | **Logiciel** | **Autre** |
| --- | --- | --- | --- |
| Profil par défaut | Concentration (v3) | **Performances sans rien fermer** | Aucune optimisation |
| Fermeture de processus | Selon les règles | **Aucune par défaut** | Aucune |
| Mode efficacité des processus d'arrière-plan | Oui | **Oui** | Non |
| Priorité de l'application | Au-dessus de la normale | Au-dessus de la normale (**jamais temps réel**) | Normale |
| Schéma d'alimentation « Performances élevées » | Oui | **Oui** | Non |
| Anti-veille pendant l'utilisation | Oui | **Oui** (un rendu long ne doit pas être interrompu par la veille) | Non |
| Arrêt temporaire de services (indexation, mises à jour) | Oui | **Désactivé par défaut**, activable | Non |
| Suspension de processus | Oui | **Non** | Non |
| Synchronisation cloud | Selon les règles | **Jamais touchée par défaut** (risque de fichier à moitié synchronisé) | Non |
| Mesure du gain | Images (PresentMon) | Temps réel de l'opération (voir 3.2) | Aucune |

**Règles pour les logiciels :**

1. **Ne jamais fermer ni suspendre un autre logiciel de création ou de calcul**, ni un logiciel connu pour avoir un travail non sauvegardé. Ils sont dans une **liste de protection par défaut** (autres logiciels 3D, CAO, montage, bases de données, outils de sauvegarde).
2. **Avertissement avant toute fermeture** dans un profil Logiciel, même si l'utilisateur l'a activée : « un travail non sauvegardé peut être perdu ».
3. **Pas de mode concentration plein écran imposé** : un logiciel 3D reste au premier plan en fenêtre normale ; **le launcher se met en retrait** (v2, section 5.1 : pause du rendu).
4. L'utilisateur peut **créer son propre profil** par logiciel, avec la même simulation obligatoire qu'en 4.5.

### 3.1 La manette pendant l'utilisation d'un logiciel

Un logiciel 3D se pilote au clavier et à la souris. Par défaut, **pour le type Logiciel, le mode de manette est « Aucun »** (v2, section 2.4, réglage par entrée) : pas de manette virtuelle ajoutée, pas de masquage de la manette physique. Le combo du centre de contrôle (v3, section 3.1) reste disponible pour **revenir au launcher**.

### 3.2 Mesurer le gain pour un logiciel

PresentMon mesure les images affichées par une fenêtre qui utilise DirectX, OpenGL ou Vulkan : utile pour **la fluidité d'un aperçu 3D**. Il **ne mesure pas** la durée d'un **rendu final** qui n'affiche pas d'images.

- Aperçu 3D : PresentMon, comme pour un jeu (protocole A/B de la v3, section 4.1).
- Rendu ou calcul : **chronométrage de la même opération** avec et sans profil, trois essais chacun. **À vérifier par l'agent** : comment relever la durée proprement (fin du processus, ou mesure manuelle saisie par l'utilisateur).
- Le gain peut être nul : c'est une réponse valide, et le profil reste alors **désactivé**.

---

## 4. Fermeture d'un logiciel (centre de contrôle, v3 section 3.4)

Même chaîne que pour un jeu, avec trois différences :

1. Le message dit **« Fermer l'application »**, avec l'avertissement sur le travail non sauvegardé.
2. **La fermeture douce est la seule proposée par défaut.** La fermeture forcée demande une **deuxième confirmation** séparée.
3. Si l'application affiche sa propre boîte « Enregistrer avant de quitter ? », le launcher **la laisse apparaître et attend** : **il ne force jamais pendant qu'une boîte de dialogue de l'application est ouverte.**

---

## 5. Textes et statistiques

- « Temps de jeu » devient **« temps d'utilisation »** pour les logiciels.
- Les textes traduits utilisent « application » ou « entrée » quand le type n'est pas connu.
- L'écran Analyse du système (4.1) indique, pour un logiciel, **ce qui serait touché et ce qui est protégé**, en langage clair.

---

## 6. Plan de travail et conditions d'arrêt

**Phase 6** : méthode A (choix direct d'un fichier), méthode B, ajout par lot, types d'entrée, icônes. Tests avec un faux dossier contenant un `.exe`, un `.lnk`, un `.bat`, des noms avec accents et un chemin très long.

**Phase 5 et 8** : profils par type (section 3), liste de protection par défaut des logiciels de création, test qui échoue si un profil Logiciel ferme un processus protégé.

**Conditions d'arrêt à ajouter (suite des n° 17 à 21 de la v4) :**

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 22 | Un logiciel (par exemple de 3D) s'ajoute **à la manette seule**, en choisissant directement son exécutable, et apparaît **tout de suite** dans le launcher | Test manuel de l'utilisateur |
| 23 | Un profil Logiciel **ne ferme ni ne suspend** aucun processus protégé, ni pendant un rendu, ni pendant un enregistrement | Test automatique et relecture du journal |
| 24 | La fermeture d'un logiciel attend sa propre boîte « Enregistrer ? » sans la forcer | Test manuel avec un logiciel qui affiche cette boîte |

---

## 7. Questions ouvertes

1. **Logiciels** : tu veux que le launcher les mette dans une rangée à part sur l'accueil, ou mélangés avec les jeux et filtrables ?
2. **Autres usages** que la 3D que tu veux couvrir (montage vidéo, développement, bureautique) ? Chacun peut avoir son profil par défaut.
