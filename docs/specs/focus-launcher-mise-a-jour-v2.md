# Focus Launcher : mise à jour v2 (sections de remplacement)

Ce fichier **remplace** les sections suivantes du document principal : 6.3 bis (tableau des niveaux), 6.8 (en entier), 7 quinquies (en entier), la tâche 6 de la phase 7, et ajoute la section 6.10 (immersion couplée aux optimisations). Tout le reste du document reste valable.

---

## 0. Ce qui a été vérifié, et ce qui reste à vérifier

| Point | Statut | Détail |
| --- | --- | --- |
| DS4Windows est sous GPL v3 | Vérifié | Licence GPL v3 depuis la version 3.2.19. |
| DS4Windows d'origine (Ryochan7) | Vérifié | Dépôt archivé le 1er janvier 2024, dernière version 3.3.3 (« EOL »). Des forks continuent (.NET 8). **L'agent doit choisir un fork actif et vérifier que sa licence est bien GPL v3.** |
| ViGEmBus (pilote de manette virtuelle) | Vérifié | Projet retiré et archivé le 2 novembre 2023, plus aucune mise à jour. Licence BSD-3-Clause. Des binaires signés restent disponibles et DS4Windows continue de s'en servir. |
| Remplaçants de ViGEmBus | Partiellement vérifié | Un fork récent (Thrum) utilise un autre moteur (VIIPER, pilote usbip-win2) que ses propres auteurs classent « expérimental ». **Aucun remplaçant n'est prêt pour la production.** |
| Steam Input et DS4Windows en même temps | Vérifié (forums) | Double saisie, manette non vue par Steam tant que DS4Windows tourne, jeux qui affichent les icônes Xbox. Le contournement connu des joueurs : arrêter le service DS4Windows avant de lancer le jeu. |
| Fermer Big Picture sans toucher à Steam | À vérifier | Le lien `steam://close/bigpicture` apparaît dans des configurations de streaming. Aucun document officiel trouvé. Les anciens noms de classe de fenêtre (années 2012) ne sont pas fiables. |
| Xbox mode de Windows 11 | Vérifié | Déployé sur les PC avec la mise à jour de mai 2026. Réglage « Choisir l'application d'accueil » pour remplacer le bureau. |
| Disponibilité d'Xbox mode en Côte d'Ivoire | **À vérifier** | La documentation Microsoft indique « marchés pris en charge ». Le pays n'est peut-être pas dans la liste. |
| Enregistrer Focus Launcher comme application d'accueil | À vérifier | Des outils tiers disent le faire « par l'API officielle ». Je n'ai pas trouvé la documentation officielle. |
| Mode efficacité (EcoQoS) via API Windows | Vérifié | `SetProcessInformation` avec `ProcessPowerThrottling`, documenté par Microsoft. |

---

## 1. Décision de licence : Focus Launcher en GPL v3

**Remplace la section 7 quinquies.**

Décision de l'utilisateur : le projet sera open source, et le code de DS4Windows sera intégré nativement.

Conséquences, à appliquer sans exception :

1. **Focus Launcher est publié sous GPL v3** (ou GPL v3 ou ultérieure). « Open source » seul ne suffit pas : la licence doit être compatible avec la GPL v3.
2. Le dépôt contient un fichier `COPYING` (texte GPL v3) et chaque fichier issu de DS4Windows garde son en-tête de copyright d'origine.
3. Tout fichier modifié est signalé comme modifié, avec la date.
4. Le code source complet correspondant à chaque version distribuée est publié.
5. Toutes les dépendances doivent être compatibles GPL v3 (MIT, BSD, Apache 2.0 le sont). Les noter dans `docs/licenses.md` : nom, licence, version, adresse.
6. `docs/licenses.md` contient aussi : le fork de DS4Windows retenu, son adresse, son commit d'origine, la liste des fichiers repris.
7. Les jaquettes, sons, polices et icônes restent soumis à leurs propres licences (voir section 6.4). La GPL ne les couvre pas.
8. Ce texte est une consigne de prudence, pas un avis juridique. Faire relire par un juriste avant la première publication.

---

## 2. Section 6.8 réécrite : Focus Input intégré et basculement

### 2.1 Les trois modes de manette (réglage dans Paramètres)

| Mode | Principe | Quand l'utiliser |
| --- | --- | --- |
| 1. Natif | Lecture directe : XInput pour la Xbox, lecture HID pour la DualShock 4. Aucun pilote ajouté | Par défaut, dans les menus |
| 2. Steam Input | Steam traduit la manette en manette Xbox virtuelle, avec ses profils | Jeux Steam, quand l'utilisateur veut les profils Steam |
| 3. **Focus Input** | Moteur de mapping **intégré** à Focus Launcher (code issu de DS4Windows). Lit la manette physique, crée au besoin une manette virtuelle, fournit les profils | Jeux qui ne comprennent que XInput, hors Steam |

Il n'y a **plus de programme DS4Windows séparé** : plus de fenêtre, plus de processus à démarrer ni à fermer.

### 2.2 Architecture

Dans `FocusLauncher.Input` (nouveau projet) :

- `IControllerSource` : produit les états de la manette (Natif, Steam Input, Focus Input l'implémentent).
- `IVirtualControllerBackend` : crée une manette virtuelle. **Une seule implémentation en version 1 : ViGEmBus.** Cette couche d'abstraction existe parce que ViGEmBus est retiré (voir section 0) : si un remplaçant fiable apparaît, il s'ajoute sans toucher au reste.
- `ControllerModeManager` : décide du mode actif, gère le basculement, journalise chaque transition.
- `DeviceHider` : masque et démasque la manette physique (via HidHide, installé séparément). Son état est **toujours restauré**.

Règles :

- **Focus Launcher ne dépend pas de ViGEmBus pour démarrer.** Si le pilote est absent, les modes 1 et 2 fonctionnent, le mode 3 indique clairement ce qui manque et renvoie vers la source officielle. Jamais d'installation de pilote sans confirmation explicite de l'utilisateur.
- L'interface du launcher lui-même fonctionne toujours en mode Natif, quel que soit le mode choisi pour les jeux.

### 2.3 Détection de la manette et choix automatique

Au démarrage et à chaque branchement :

1. Lister les manettes et leur famille (PlayStation, Xbox, autre), selon la section 6.5.
2. Proposer le mode adapté : Xbox → Natif. DualShock 4 → Steam Input si Steam est ouvert et réglé pour, sinon Focus Input, sinon Natif.
3. Le choix de l'utilisateur est mémorisé et **prime toujours** sur la détection automatique.
4. Bouton « Détecter la manette » dans Paramètres, avec résultat en clair.
5. Un réglage **par jeu** : mode forcé (Natif, Steam Input, Focus Input, Aucun).

### 2.4 Séquence de basculement au clic sur « Jouer »

La séquence est écrite pour éviter la double saisie, qui est le piège documenté par les joueurs.

**Cas A : jeu lancé par Steam, mode Steam Input**

1. Journal de session : écrire l'état courant (mode, manette masquée ou non).
2. Arrêter Focus Input (arrêt du moteur dans le même processus).
3. Supprimer la manette virtuelle.
4. **Démasquer la manette physique** (HidHide). Sans cela, Steam ne la voit pas.
5. Attendre que Windows ré-énumère la manette (délai maximal, avec message en cas d'échec).
6. Vérifier que Steam est en cours d'exécution. Sinon : avertissement clair, repli sur le mode Natif.
7. Lancer le jeu. Steam Input prend le relais.
8. Surveillance : détecter une double saisie (deux manettes actives pour une seule physique) et afficher un avertissement.
9. À la fermeture du jeu : relancer Focus Input, recréer la manette virtuelle, remasquer la manette, effacer le journal.

**Cas B : jeu hors Steam, mode Focus Input**

1. Focus Input reste actif, la manette virtuelle reste présente, la physique reste masquée.
2. Le jeu ne voit que la manette virtuelle.
3. À la fermeture du jeu : rien à changer.

**Cas C : jeu Steam, mais l'utilisateur veut Focus Input**

Steam Input doit être désactivé pour ce jeu dans Steam. Le launcher le signale avant le lancement. Il **ne modifie jamais** les fichiers ni le registre de Steam.

### 2.5 Pièges à tester explicitement (liste de contrôle pour l'agent)

| # | Cas | Résultat attendu |
| --- | --- | --- |
| 1 | DualShock 4 en Bluetooth : Focus Input actif, puis lancement d'un jeu Steam | Steam voit la manette, aucune double saisie |
| 2 | Même cas : la manette doit-elle être éteinte et rallumée pour que Steam la voie ? Des sources signalent ce besoin en Bluetooth | **Mesurer et documenter.** Si c'est le cas, prévoir une déconnexion et reconnexion automatiques de la manette ou un message à l'écran |
| 3 | Retour au launcher après la fermeture du jeu | Focus Input reprend, la manette reste masquée une seule fois, aucune double saisie |
| 4 | Plantage du lanceur pendant un jeu | Au redémarrage : la manette physique est démasquée et la manette virtuelle supprimée (même principe que `session.lock.json`, section 4.4) |
| 5 | Xbox filaire + DualShock 4 en même temps | Règle : la dernière manette utilisée devient la manette active, documentée |
| 6 | Steam fermé | Repli automatique sur le mode Natif, sans erreur |
| 7 | HidHide absent | Focus Input fonctionne sans masquage, avec un avertissement de double saisie possible |
| 8 | ViGEmBus absent | Modes 1 et 2 intacts, mode 3 indiqué comme indisponible |
| 9 | Icônes : croix, rond, carré, triangle sur la DualShock 4 dans chaque mode | Icônes PlayStation (en mode Steam Input, via le réglage manuel de la section 6.3 bis) |

Chaque transition de mode est consignée dans le journal de session et restaurée en cas de plantage.

---

## 3. Patch de la section 6.3 bis

Le tableau des « trois niveaux » est remplacé par :

| Niveau | Principe | Version |
| --- | --- | --- |
| 1. XInput direct | Manettes Xbox et compatibles | 1 |
| 2. Compatibilité avec Steam Input | Focus Launcher ajouté à Steam comme jeu non-Steam | 1 |
| 3. API Steam Input intégrée | Appel direct de ISteamInput (SDK Valve) | Ultérieure, après vérification des conditions de Valve |
| Focus Input | Moteur intégré, voir section 2 de ce fichier | 1 |

Les règles de la 6.3 bis sur la double saisie, les icônes PlayStation et les tests avec les deux manettes de l'utilisateur restent valables.

---

## 4. Section 6.5 : amendement sur le « mode shell »

**Nouveau.** Windows 11 propose désormais **Xbox mode**, qui remplace le bureau par une application d'accueil de jeux, avec un réglage officiel « Choisir l'application d'accueil » et une entrée dans la vue des tâches et la Game Bar. Cela offre une voie officielle pour ce que le document interdisait (remplacer l'explorateur par modification du registre).

Consignes :

1. **Le remplacement de l'explorateur par le registre reste interdit** (sections 6.5 et 9.3).
2. L'agent **étudie** l'enregistrement de Focus Launcher comme application d'accueil de Xbox mode, **uniquement par l'interface officielle de Windows**. Il cherche la documentation officielle Microsoft avant tout code, et il la cite dans `docs/references.md`.
3. Conditions à vérifier avant de promettre quoi que ce soit : version de Windows requise (24H2 ou plus d'après la documentation), disponibilité dans le pays de l'utilisateur, différences selon les éditions de Windows.
4. **Si Xbox mode n'est pas disponible** sur le PC de l'utilisateur, le démarrage via le dossier Démarrage de l'utilisateur (déjà prévu en version 1) reste la solution.
5. Xbox mode est un concurrent possible de Focus Launcher. Ne pas supposer qu'il est présent. Détecter, ne pas dépendre.

---

## 5. Nouvelle section 6.10 : immersion couplée aux optimisations

Principe : chaque levier d'optimisation doit **aussi servir l'immersion**, être **réversible**, ne **pas écrire dans le registre**, et être **mesuré** avant d'être gardé (section 11). Les gains ne sont **pas démontrés** : ce sont des hypothèses à mesurer.

### 5.1 Leviers retenus pour la version 1

| Levier | Moyen | Réversible | Registre | Bénéfice immersion | À mesurer |
| --- | --- | --- | --- | --- | --- |
| **Mode efficacité des processus d'arrière-plan** | `SetProcessInformation` avec `ProcessPowerThrottling` (EcoQoS) + priorité en dessous de la normale | Oui, état d'origine enregistré | Non | Aucun processus fermé, donc aucune perte de données, et le PC reste silencieux | Mémoire, charge processeur, fluidité |
| **Le launcher se met lui-même en mode efficacité pendant le jeu** | Même API, appliquée à Focus Launcher | Oui | Non | Le launcher « disparaît » : plus d'animation, plus de fond vidéo | Consommation du launcher (critère de la section 11) |
| **Pause du rendu du launcher** | Arrêt des animations, du fond dynamique et de la musique d'ambiance quand le jeu a le focus, reprise au retour | Oui | Non | Transition nette entre menu et jeu | Mémoire graphique, charge processeur |
| **Baisse du volume des autres applications** | API audio Windows par application (sessions audio) | Oui, volumes d'origine enregistrés | Non | Une option douce à la place de fermer Discord ou un navigateur | Subjectif, à tester |
| **Empêcher la veille et l'écran de veille pendant la session** | `SetThreadExecutionState` (s'annule seul à la fermeture du processus) | Oui, automatique | Non | Aucune interruption en plein jeu | Aucun |
| **Schéma d'alimentation** | Lecture, passage en performances élevées, restauration | Oui | Non (déjà prévu, 4.5) | Fréquence stable | Température, bruit, images par seconde |
| **Suspension de processus lourds** | Suspension puis reprise, au lieu de la fermeture | Oui, reprise garantie par le journal | Non | Pas de perte d'état | Mémoire, stabilité. **Tester soigneusement**, certains programmes ne supportent pas la suspension |
| **Plein écran du launcher** | Fenêtre sans bordure au premier plan, couvrant la barre des tâches | Oui | Non | Aucune fenêtre Windows visible (section 6.5) | Aucun |
| **Couplage du son** | Musique des menus coupée au lancement du jeu, son de fin de session au retour | Oui | Non | Ambiance console | Aucun |
| **Vibration au lancement et au retour** | Retour sensoriel de la manette | Oui | Non | Immersion | Aucun |

### 5.2 Leviers à ne pas faire en version 1, et pourquoi

| Levier | Raison |
| --- | --- |
| Mode Jeu de Windows, assistant de concentration, notifications | Aucune interface officielle simple trouvée : passe par le registre. **Exclu en version 1 (9.3).** Les notifications peuvent donc apparaître en jeu : l'indiquer honnêtement à l'utilisateur |
| Réglage graphique par application, accélération matérielle du GPU | Registre |
| Résolution du minuteur système | Gain contesté, risque d'effets de bord |
| Vidage de la mémoire en attente | Gain non démontré, droits élevés. Reste désactivé par défaut (4.5) |
| Priorité temps réel | Interdite (4.4) |
| Désactivation durable de services | Interdite (9.3) |

### 5.3 Règle de sécurité propre au mode efficacité

Microsoft indique que ce mode est destiné aux tâches qui ne participent pas à l'expérience au premier plan, et qu'il ne doit pas être appliqué aux tâches critiques pour les performances. Donc :

- Ne **jamais** l'appliquer au jeu, à son client de store, ni aux processus de la liste blanche (4.2).
- Ne **jamais** l'appliquer au pilote de manette ni au moteur Focus Input.
- Enregistrer l'état d'origine de chaque processus **avant** modification, dans `session.lock.json`.

### 5.4 Ordre de prudence (mise à jour de la section 6.9)

1. Mode efficacité et baisse de priorité (le moins risqué).
2. Baisse du volume des autres applications.
3. Suspension de processus.
4. Fermeture propre (déjà prévue).
5. Fermeture forcée, seulement si la règle l'autorise.

---

## 6. Changements dans le plan de travail

**Phase 0** : ajouter `COPYING` (GPL v3) et `docs/licenses.md` avec le fork de DS4Windows choisi.

**Phase 7, tâche 6 remplacée par :**

6. Gestion de la déconnexion de la manette en cours d'utilisation. Nouveau projet `FocusLauncher.Input` : trois modes (Natif, Steam Input, Focus Input), abstraction `IVirtualControllerBackend` avec ViGEmBus comme seule implémentation, `DeviceHider`, basculement selon la section 2.4, liste de contrôle de la section 2.5, guide `docs/steam-input.md` et `docs/focus-input.md`. Aucun pilote installé sans confirmation.

**Phase 8** : ajouter la section 6.10 (leviers de la section 5.1) au flux « Jouer », chaque levier avec son interrupteur, son test, sa fonction de restauration et sa mesure.

**Phase 9** : ajouter la relecture juridique des licences avant publication.

**Conditions d'arrêt (7 bis), ajouter :**

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 9 | La liste de contrôle de la section 2.5 est entièrement passée avec la DualShock 4 en Bluetooth et la Xbox filaire | Test manuel de l'utilisateur, résultats dans `PROGRESS.md` |
| 10 | Après un plantage en pleine session, la manette physique est démasquée et la manette virtuelle supprimée | Test de plantage, trois fois de suite |
| 11 | Chaque levier de la section 5.1 a une mesure consignée, et seuls ceux avec un gain mesuré restent actifs par défaut | Relecture de `PROGRESS.md` |

---

## 7. Questions ouvertes pour l'utilisateur

1. Quel fork de DS4Windows reprendre ? L'agent propose, tu valides après lecture de l'activité et de la licence.
2. Acceptes-tu que le mode 3 dépende de ViGEmBus (pilote retiré mais fonctionnel) en version 1, avec l'abstraction prévue pour le remplacer plus tard ?
3. Xbox mode est-il disponible sur ton PC ? Si oui, veux-tu que l'agent étudie l'enregistrement de Focus Launcher comme application d'accueil ?
