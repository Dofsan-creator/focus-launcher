# Focus Launcher : correctif v4.3 (décisions, démarrage auto, apps gaming, langues, entrées)

Ce correctif **complète** la v4.2. Il tranche les questions ouvertes (section 1) et ajoute quatre blocs : lancement à l'ouverture de session (2), optimisation pour les applications de gaming (3), langues (4), clavier, souris et manette partout (5). En cas de conflit, **ce fichier prime** sur la v4.2.

---

## 1. Décisions sur les questions ouvertes de la v4.2

| Question | Décision | Raison |
| --- | --- | --- |
| Pages d'accueil | **Une seule page filtrable** : Tout, Jeux, Logiciels, Social. L1 et R1 changent de filtre (clavier : Tab et Maj+Tab, souris : onglets cliquables) | Plus simple à rendre fluide aux trois entrées. Deux pages restent possibles plus tard |
| Mise en pause | **Reportée à la v1.1.** Le module de la v3 (4.3) est préparé mais **non exposé**. Quand elle arrivera : désactivée par défaut, et **jamais proposée par défaut aux applications sociales** (section 3) | Risque pour les jeux en ligne et les anticheats. Le retour rapide au launcher sans fermer le jeu couvre déjà le besoin |
| Réglages rapides | **Volume, volume par application (à étudier), état de la manette et du mode, veille.** La luminosité est **retirée** | Non fiable par logiciel sur PC |
| Nom « Centre de contrôle » | **Nom de travail interne : « panneau rapide »** jusqu'au choix de la piste de design, puis nom propre (v4.2, section 4) | Trop proche de la PS5 |

Conséquence : la **condition d'arrêt 26** (v4.2) est **suspendue** jusqu'à l'exposition de la mise en pause.

---

## 2. Lancement à l'ouverture de session

**Règle :** l'utilisateur choisit, et peut changer d'avis à tout moment. Deux états : **automatique** (le launcher s'ouvre à l'ouverture de la session) ou **manuel** (il ne s'ouvre que si l'utilisateur le lance).

| Point | Décision |
| --- | --- |
| Valeur par défaut | **Automatique, cochée**, proposée dans l'assistant du premier démarrage (section 4) avec une case claire à décocher |
| Où le changer ensuite | Paramètres, entrée « Lancer à l'ouverture de session », accessible **aux trois entrées** (section 5) |
| Mécanisme | Celui de l'OS pour **la session de l'utilisateur courant**, **sans droits administrateur**. Sous Windows : entrée de démarrage de l'utilisateur. L'agent confirme la plateforme visée dans le dépôt |
| Ce qu'on ne fait pas | **On ne remplace jamais le shell du système** (l'explorateur, le bureau). Le launcher est une application comme une autre |
| Délai | Court délai réglable (3 secondes par défaut) pour laisser la session finir de se charger |
| Mode au lancement | Plein écran par défaut. Option « démarrer réduit » |
| Garde-fou | Si le launcher **plante deux fois de suite au démarrage**, le lancement automatique se **désactive seul** et un message l'explique au prochain lancement manuel |
| Sortie de secours | Argument de ligne de commande `--sans-demarrage-auto`, et **sortie toujours possible** (Retour, Échap, combo manette) |
| Décocher | Retire l'entrée de démarrage. **Cocher** la recrée. L'état affiché reflète **l'état réel du système**, pas un simple réglage mémorisé |

---

## 3. Optimisation pour les applications de gaming (Twitch, Discord, etc.)

**Principe :** ces applications tournent **à côté** des jeux (voix, diffusion, musique, boutiques). Le launcher doit les lancer, y revenir et en repartir **sans jamais les interrompre**.

1. **Filtre « Social »** et collection par défaut, remplie par **détection automatique** de ce qui est installé : Discord, OBS, Spotify, TeamSpeak et équivalents. Boutiques de jeux (Steam, Epic, GOG, Battle.net, EA, Ubisoft) dans le filtre **Jeux**, pas Social.
2. **Twitch et autres services web** : entrée de type **site web**, ouverte dans une fenêtre d'application du navigateur par défaut. **Aucun navigateur ni client n'est intégré** dans le launcher.
3. **Retour sans fermer** : le combo ramène au launcher, l'application **continue** (voix Discord, diffusion, musique). « Passer à » la remet au premier plan (sélecteur d'entrées en cours, v4.2).
4. **Jamais suspendues, jamais fermées par erreur** : même quand la mise en pause existera, elle est **interdite par défaut** pour la catégorie Social. La fermeture demande une confirmation qui **prévient qu'une communication ou une diffusion peut s'arrêter**.
5. **Volume par application** dans le panneau rapide (par exemple baisser le jeu, garder la voix). **À étudier** : si non fiable, le retirer.
6. **Mode curseur à la manette** : ces applications ne sont pas faites pour la manette. Un combo active un **curseur piloté par le stick** (A = clic, B = retour, gâchettes = défilement), avec **clavier à l'écran** (section 5). Version 1 minimale, **à tester**.
7. **Icônes** : **aucun logo de marque tiers n'est livré** dans le dépôt. Les icônes sont **extraites localement** des applications installées, avec une icône générique de secours.
8. **Hors périmètre v1** : amis, statut en ligne, messages, flux de diffusion dans le launcher.

---

## 4. Langues, avec choix au premier démarrage

| Point | Décision |
| --- | --- |
| Écran de choix | **Premier écran du premier démarrage**, avant tout le reste, utilisable aux trois entrées. Langue de l'OS **présélectionnée** |
| Affichage | Chaque langue dans **son propre nom** (Français, English, Español, Deutsch, 日本語, العربية…), pour qu'on la trouve sans la comprendre |
| Changement ensuite | Paramètres, à tout moment, **sans redémarrage** |
| Architecture | **Une langue = un fichier de traduction.** Ajouter une langue ne demande **aucun changement de code**. Chaînes en format ICU (pluriels, genres). **Aucun texte dans les images** |
| Repli | Clé manquante : anglais, jamais une clé brute à l'écran |
| Écriture | Polices de secours pour tous les alphabets. **Droite-gauche (RTL)** pris en charge dans la mise en page. Les textes peuvent grandir de 40 % sans casser l'écran |
| Honnêteté sur « toutes les langues » | **Toutes** est un objectif d'architecture, pas une promesse de traduction complète au premier jour. **Complètes à la livraison : français et anglais.** Ensuite : espagnol, allemand, italien, portugais (Brésil), japonais, coréen, chinois simplifié, russe, arabe, avec la mention **« traduction à relire »** tant qu'une personne compétente ne l'a pas validée |
| Contribution | `docs/traductions.md` explique comment ajouter ou corriger une langue |
| Règles existantes | Le test des termes de constructeurs (v4.2, section 4) s'applique à **tous** les fichiers de langue |

---

## 5. Manette, souris et clavier : tout partout

**Règle :** **chaque écran, chaque réglage et chaque action**, y compris la configuration, est faisable **avec la manette, avec la souris et avec le clavier**, sans jamais en exiger un seul.

| Sujet | Décision |
| --- | --- |
| Clavier | Flèches, Tab, Entrée, Échap, Maj+Tab. Focus **toujours visible**. Raccourci global configurable pour revenir au launcher (valeur par défaut sans conflit avec l'OS et les jeux) |
| Souris | Survol, clic, molette, glisser-déposer. Boutons cliquables pour tout ce que la manette fait par combo |
| Manette | Comme prévu en v3 et v4.2 |
| Changement automatique | L'interface détecte **le dernier périphérique utilisé** : les **symboles d'aide** (A, B, Entrée, Échap) changent en conséquence, et le **curseur de souris se masque** en mode manette |
| Personnalisation du panneau | Souris : glisser-déposer. Clavier : sélectionner, puis flèches pour déplacer, Entrée pour valider. Manette : comme en v4.2. Les fonctions essentielles ne se masquent pas |
| Saisie de texte | **Clavier à l'écran** pour la manette, avec **disposition adaptée à la langue** (AZERTY, QWERTY, etc.) |
| Accessibilité | Exposer l'interface aux lecteurs d'écran : **à étudier** |

---

## 6. Plan de travail et conditions d'arrêt

**Phase 7** : les maquettes intègrent les sections 3 à 5 (filtre Social, écran de langue, écran de démarrage).
**Phase 8 ter** : lancement à l'ouverture de session, détection des applications sociales, mode curseur, langues et clavier à l'écran, navigation aux trois entrées.

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 28 | Cocher et décocher le lancement automatique fonctionne **après un vrai redémarrage de la session**, sans droits administrateur, et le garde-fou de deux plantages agit | Résultats notés dans `PROGRESS.md` |
| 29 | Chaque écran et chaque réglage se parcourt et se modifie **aux trois entrées**, sans impasse | Liste de contrôle dans `PROGRESS.md` + test automatique de navigation au clavier |
| 30 | Le choix de langue au premier démarrage fonctionne aux trois entrées. Chaque fichier de langue a **toutes les clés**. Si une langue RTL est livrée, un test de mise en page la couvre | Test automatique + capture interne non livrée |
| 31 | Au moins **deux applications sociales** (par exemple Discord et un service web comme Twitch) ont été testées : lancement, retour au launcher **sans coupure**, retour dans l'application, fermeture avec avertissement | Résultats notés dans `PROGRESS.md` |
| 32 | **Aucun logo de marque tiers** n'est dans le dépôt | Lecture de la liste des ressources visuelles |

---

## 7. À confirmer (une seule chose)

**Plateforme visée** : j'ai supposé **Windows** d'après le reste du projet (processus, anticheats). Si une autre plateforme est visée, seule la section 2 (mécanisme de démarrage) change.

*Rappel juridique (v4.2) : consigne de prudence, pas un avis. Avant la première publication, faire relire apparence, noms, sons et usage des marques par un juriste.*
