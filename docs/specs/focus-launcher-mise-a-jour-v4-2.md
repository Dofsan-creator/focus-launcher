# Focus Launcher : correctif v4.2 (inspiration des consoles, exécution originale)

Ce correctif **complète** la section 6.4 (inspirations) et la section 3 de la v4 (identité visuelle originale). Il ajoute la **Nintendo Switch** aux références et transforme les trois consoles en une liste de **comportements à reprendre**, pas d'éléments graphiques à copier.

---

## 1. La règle, en une phrase

**On reprend les comportements et les principes d'usage des trois consoles. On ne reprend aucun élément d'apparence, de nom, de son ni d'icône.**

Les deux demandes de l'utilisateur vont ensemble : s'inspirer des consoles pour que l'usage soit naturel, et avoir un design original, pas un copier-coller. Un comportement (« appuyer sur un bouton ouvre un menu rapide ») est une idée libre. Une mise en scène précise (disposition, animations, icônes, noms) est protégée.

---

## 2. Ce qui a été vérifié sur chaque console

| Console | Comportement vérifié | Source |
| --- | --- | --- |
| **Switch** | Le bouton HOME **suspend** le logiciel et affiche le menu d'accueil, à tout moment | Aide officielle Nintendo |
| **Switch** | Pour fermer : depuis le menu, sélectionner le logiciel en cours (repéré par une marque « en cours »), bouton X, puis **Fermer** avec confirmation. **Avertissement : la progression non sauvegardée est perdue** | Aide officielle Nintendo |
| **Switch** | Un **appui long d'au moins une seconde** sur HOME ouvre des **réglages rapides** sur le côté droit (veille, volume, luminosité, mode avion). On en sort avec B ou HOME | Aide officielle Nintendo |
| **Switch** | Accueil : **une rangée de jeux**, et une **rangée de fonctions système** en bas ; l'ordre des icônes est modifiable | Aide officielle Nintendo |
| **PS5** | Un seul appui sur le bouton PS ouvre un **centre de commande par-dessus** ce qu'on fait : fonctions en bas, **cartes** d'activité au-dessus | Support PlayStation |
| **PS5** | Le centre est **personnalisable** : on réordonne et on masque les icônes (**certaines ne peuvent pas être masquées**) | Support PlayStation |
| **PS5** | Un **sélecteur** liste les jeux et applications récents, permet de passer de l'un à l'autre ou de **fermer** un jeu | Support PlayStation |
| **PS5** | Deux pages principales sur l'accueil : **Jeux** et **Médias** | Presse spécialisée |
| **PS5** | Une page par jeu (« hub ») : actualités, activités, contenus | Presse spécialisée |
| **Xbox** | **Reprise rapide** : les jeux restent **suspendus** en arrière-plan et reprennent instantanément | Presse spécialisée |
| **Xbox** | La reprise rapide a posé des problèmes avec certains jeux (en ligne) : Microsoft a ajouté un réglage **par jeu** pour la désactiver | Presse spécialisée |
| **Xbox** | Un menu **par-dessus le jeu** (guide), des **groupes** pour organiser jeux et applications, un **thème clair ou sombre automatique** selon l'heure | Presse spécialisée |

---

## 3. Comportements à reprendre, et comment on les adapte

| Comportement | Origine | Dans Focus Launcher | Particularité |
| --- | --- | --- | --- |
| **Retour rapide au launcher sans fermer** | Les trois | Le combo de la v3 (section 3.1) ramène au launcher, le jeu **reste ouvert** | Acquis en v3 |
| **Mise en pause d'une application** (suspension) | Switch, Xbox | Option **« Mettre en pause »** par entrée : suspension de l'arbre de processus (module de la v3, section 4.3) | **Désactivée par défaut.** Peut gêner les jeux en ligne et certains anticheats, comme Xbox l'a constaté. **Ne survit pas à un redémarrage du PC**, contrairement à une console. **À tester** avec des jeux variés |
| **Réglages rapides par appui long** | Switch | **Appui long (1 seconde) sur le combo** : panneau latéral avec volume, état de la manette et du mode, veille | La luminosité d'un écran de PC n'est pas fiable par logiciel : **à étudier**, sinon retirer. On en sort avec Retour |
| **Panneau rapide par-dessus** | PS5, Xbox | Le panneau de la v3, section 3.2 | Voir la règle de nommage, section 4 |
| **Sélecteur d'entrées en cours** | PS5, Switch | Liste des entrées lancées par le launcher, avec **passer à** et **fermer** | Passer à = amener la fenêtre au premier plan. Fermer = chaîne de la v3, section 3.4 |
| **Panneau personnalisable** | PS5 | L'utilisateur **réordonne et masque** les fonctions. Les **essentielles ne se masquent pas** (retour au launcher, fermer, veille) | Un écran de personnalisation à la manette |
| **Deux pages d'accueil** | PS5 | **Jeux** et **Logiciels**, changement avec L1 et R1 | Répond à la question 1 de la v4.1 : **recommandation**, tu peux garder une seule page filtrable |
| **Page par entrée** | PS5 | La fiche de la v3, section 2.2 | Sans actualités ni activités en version 1 |
| **Groupes et collections** | Xbox | Collections de la v3, section 2.1 | |
| **Thème clair ou sombre automatique** | Xbox | Option **thème selon l'heure** | Sombre par défaut (6.4) |
| **Rangée de fonctions système** | Switch, PS5 | Accès à Bibliothèque, Boutiques, Téléchargements, Paramètres, Veille | **Placement propre à la piste de design choisie**, pas en bas par automatisme |
| **Confirmation avant fermeture, avec avertissement** | Switch | Déjà prévue (v3, 3.4 et v4.1, section 4) | |

**Ne rien reprendre** de ce qui n'a pas de sens ici : profils d'utilisateurs, amis, trophées, magasin intégré, actualités, partage de captures, mode avion. Ils pourraient venir plus tard, mais ils ne font pas partie de la version 1.

---

## 4. Noms : ne pas réutiliser ceux des consoles

Les termes **« Control Center », « Quick Resume », « Switcher », « HOME », « Guide », « Cards », « Quick Settings »** sont ceux des constructeurs. **« Centre de contrôle »**, utilisé comme nom de travail dans la v3, est trop proche de celui de la PS5.

Consignes :

1. **Nommer chaque fonction avec des mots propres au projet**, cohérents avec la piste de design (par exemple un vocabulaire de photographie si la piste Lentille, Iris ou Pellicule est retenue).
2. Ces noms sont **traduits** (6.11) et listés dans `docs/design.md`.
3. Les termes de la liste ci-dessus **n'apparaissent dans aucun texte de l'interface**. Ils peuvent apparaître dans la documentation, quand il s'agit de décrire une compatibilité (« manette PlayStation », « manette Xbox »).
4. Un **test automatique** cherche ces termes dans les fichiers de traduction et échoue s'il en trouve un.

---

## 5. Revue d'originalité (complément de la v4, section 3.3)

Ajouter la **Nintendo Switch** aux interfaces comparées **pour le seul contrôle interne** : Playnite en plein écran, Steam Big Picture, Xbox, PS5, **Switch**. Pour chaque écran, l'agent note dans `docs/originality-review.md` :

- ce qui est repris (**un comportement**) ;
- ce qui diffère (**disposition, mouvement, icônes, noms, sons**) ;
- le jugement : « assez différent » ou « à retravailler ».

Une captures ne sont **jamais** livrées, décalquées, ni placées dans le dépôt.

**Rappel juridique :** ce texte est une consigne de prudence, pas un avis. Avant la première publication, faire relire par un juriste l'apparence, les noms et les sons.

---

## 6. Plan de travail et conditions d'arrêt

**Phase 7** : les maquettes de la v4 (section 3.3) intègrent les comportements de la section 3 de ce fichier, avec des noms propres.

**Phase 8 bis** : panneau rapide, appui long pour les réglages rapides, sélecteur d'entrées en cours, personnalisation du panneau, option de mise en pause par entrée.

**Conditions d'arrêt à ajouter (suite des n° 22 à 24 de la v4.1) :**

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 25 | Aucun terme de constructeur de la section 4 n'apparaît dans les textes de l'interface | Test automatique sur les fichiers de traduction |
| 26 | La mise en pause d'une entrée a été testée avec au moins un jeu hors ligne, un jeu en ligne et un logiciel, avec résultats notés | `PROGRESS.md` |
| 27 | La revue d'originalité couvre les cinq interfaces de référence et chaque écran principal | Lecture de `docs/originality-review.md` |

---

## 7. Questions ouvertes

1. **Deux pages d'accueil** (Jeux et Logiciels) avec L1 et R1, ou une seule page filtrable ?
2. **Mise en pause d'une application** : tu veux la fonction dès la version 1 (désactivée par défaut), ou plus tard ?
3. Les **réglages rapides** : volume, manette et veille suffisent pour la version 1 ?
