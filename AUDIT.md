# Audit

Une ligne par constat (v4, section 5). Gravité : critique, haute, moyenne, basse.

| # | Date | Gravité | Constat | Décision |
| --- | --- | --- | --- | --- |
| 1 | 2026-11-07 | critique | v4.4 exige toutes les langues d'affichage de Windows (environ 110) avant la sortie. 11 sont livrées, couvrant chaque famille d'écriture | **À trancher par l'utilisateur** : sortir avec 11, ou traduire les autres d'abord (un fichier chacune, statut « à relire ») |
| 2 | 2026-11-07 | haute | La coupure réelle n'a jamais tourné sur une vraie machine | Volontaire (règle 8). À faire avec toi, en commençant par un seul élément inoffensif |
| 3 | 2026-11-07 | haute | `focus-guardian` jamais installé ni testé en vrai | Même raison : il s'installe avec élévation et touche aux services |
| 4 | 2026-11-07 | haute | Focus Input sans HidHide : le jeu peut voir la manette physique et la virtuelle | Repli prévu par la v2 (cas 7). Intégrer la CLI de HidHide, après vérification de sa documentation officielle |
| 5 | 2026-11-07 | haute | Gains non mesurés (PresentMon non intégré) | Aucun levier n'est actif sans ton accord (simulation par défaut). Intégration de PresentMon à faire (MIT) |
| 6 | 2026-11-07 | moyenne | `steam://close/bigpicture` non documenté par Valve | Échec sans conséquence, documenté dans `docs/stores.md` |
| 7 | 2026-11-07 | moyenne | Le panneau rapide par-dessus un jeu en plein écran exclusif n'est pas testé | Repli : le combo ramène la fenêtre du launcher au premier plan |
| 8 | 2026-11-07 | moyenne | Jaquettes : couleur générée à partir du titre, pas d'image | SteamGridDB avec la clé de l'utilisateur, à ajouter |
| 9 | 2026-11-07 | moyenne | Pas de recherche texte ni de collections personnelles sur l'accueil | À ajouter : le clavier à l'écran existe déjà |
| 10 | 2026-11-07 | moyenne | Installation : progression par message seulement, pas d'écran Téléchargements | À ajouter |
| 11 | 2026-11-07 | moyenne | Clavier à l'écran : pas de saisie chinoise par pinyin ni de conversion japonaise en kanji | Les dispositions du clavier physique restent possibles |
| 12 | 2026-11-07 | moyenne | `vigem-client` : dernier push en 2023, et ViGEmBus est archivé | L'abstraction de manette virtuelle permet de remplacer le pilote |
| 13 | 2026-11-07 | basse | Volume par application : commandes prêtes, aucune interface | À ajouter au panneau rapide |
| 14 | 2026-11-07 | basse | Licences des dépendances non auditées une par une | Lancer `cargo deny check licenses` avant publication |
| 15 | 2026-11-07 | basse | Exécutables non signés : SmartScreen préviendra | Étudier SignPath pour les projets open source |
| 16 | 2026-11-07 | basse | Tab change de filtre sur l'accueil (v4.3), au détriment des lecteurs d'écran | Proposition dans `docs/decisions.md` |
| 17 | 2026-11-07 | basse | Le nom « Focus Launcher » n'a pas été vérifié contre d'autres logiciels | Avant publication |

## Contradictions des specs, réglées par la règle la plus prudente

Priorité haute (v1) contre au-dessus de la normale (v4.1) ; installation des jeux (v4) contre hors périmètre (v4.4) ; suspension en v1 (v2) contre reportée (v4.3) ; interface toujours native (v2) contre lancement via Steam (v1). Le détail est dans `SPEC.md`, section 2.
