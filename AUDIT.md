# Audit

Une ligne par constat (v4, section 5). Gravité : critique, haute, moyenne, basse.

| # | Date | Gravité | Constat | Décision |
| --- | --- | --- | --- | --- |
| 1 | 2026-11-07 | critique | v4.4 exige toutes les langues d'affichage de Windows (environ 110) avant la sortie. 11 sont livrées, couvrant chaque famille d'écriture | **À trancher par l'utilisateur** : sortir avec 11, ou traduire les autres d'abord (un fichier chacune, statut « à relire ») |
| 2 | 2026-11-07 | haute | La coupure réelle n'a jamais tourné sur une vraie machine | Volontaire (règle 8). À faire avec toi, en commençant par un seul élément inoffensif |
| 3 | 2026-11-07 | haute | `focus-guardian` jamais installé ni testé en vrai | Même raison : il s'installe avec élévation et touche aux services |
| 4 | 2026-11-07 | haute, intégré | Focus Input sans HidHide : le jeu peut voir la manette physique et la virtuelle | CLI de HidHide intégrée le 2026-11-09 (n° 42) ; 🧑 HidHide à installer depuis le catalogue puis essai |
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
| 18 | 2026-11-08 | critique | Écran noir au lancement de la version publiée : la politique de sécurité (`connect-src ipc: http://ipc.localhost`) bloquait le téléchargement du `.wasm`. `cargo tauri dev` et le navigateur n'appliquent pas cette politique, d'où un défaut resté invisible | **Corrigé** : `'self'` ajouté à `connect-src`, test `crates/focus-app/tests/csp.rs` (rouge sur l'ancienne valeur). Interface vue sur l'exécutable publié. Sans interface après 60 s : erreur au journal, fermeture, démarrage compté comme raté (décision 8) |
| 19 | 2026-11-08 | moyenne | L'argument `--autostart` du raccourci de démarrage n'est jamais lu : pas de délai de 3 s, pas d'option « démarrer réduit », pas de sortie de secours `--sans-demarrage-auto` (v4.3, section 2) | **Corrigé** le 2026-11-08 : délai réglable, démarrage réduit, `--sans-demarrage-auto` |
| 20 | 2026-11-08 | moyenne | « Supprimer » sur la fiche d'une entrée agit sans confirmation : un appui de trop à la manette suffit | **Corrigé** : confirmation, « Non » par défaut |
| 21 | 2026-11-08 | basse | L'horloge de l'accueil crée un minuteur à chaque retour à l'accueil et ne l'arrête jamais (`set_interval_with_handle`, poignée jetée) | **Corrigé** : une seule horloge pour toute l'application |

| 22 | 2026-11-08 | moyenne | Mode console : la session de concentration de Windows (Ne pas déranger) est refusée aux applications non autorisées par Microsoft (« Feature … not available », 0x80070005). Les notifications ne sont donc pas coupées | Barre des tâches masquée et PC éveillé fonctionnent. Couper les notifications autrement demanderait le registre (interdit) : à trancher par l’utilisateur |
| 23 | 2026-11-08 | haute | Connexion Wi-Fi avec mot de passe, appairage Bluetooth, assistant du premier démarrage et vibrations jamais essayés à la main | Code et listes vérifiés sur ce PC ; essais réels à faire par l’utilisateur |
| 24 | 2026-11-08 | moyenne | Les jaquettes Epic sont téléchargées depuis le CDN d’Epic (3 hôtes autorisés, HTTPS) : première préparation d’environ 20 s | Mise en cache définitive ; sans réseau, l’icône ou la couleur du jeu s’affiche |

| 25 | 2026-11-08 | haute | Touche Windows : crochet posé (journal), mais non prouvé, car les essais venaient d’un terminal administrateur que Windows cache aux programmes ordinaires. Si le menu Démarrer s’ouvre pendant le mode console, la barre des tâches peut rester masquée après la sortie | Essai à la main par l’utilisateur ; repli : la sentinelle rend la barre deux fois |
| 26 | 2026-11-08 | moyenne, en cours | Plan console Maximal (fermeture de 20 programmes, 8 services) | Niveau « Maximal » choisi par l'utilisateur : exécuté et rendu à chaque ouverture et fermeture depuis le 2026-11-09 (journal) ; la relance de Game Bar échoue (paquet Windows, « Accès refusé »), à revoir |

| 27 | 2026-11-08 | moyenne | Mise à jour : seul Steam l’indique sur le disque ; Epic et les autres boutiques ne sont pas vérifiés | Bouton « Mettre à jour » pour Steam ; ouvrir la boutique pour les autres |
| 28 | 2026-11-08 | moyenne | Capture d’écran : un jeu en plein écran exclusif peut donner une image noire | Les jeux en plein écran sans bordure sont capturés |
| 29 | 2026-11-08 | haute | Menu latéral en jeu, écran de chargement jusqu’au jeu, écran de fin, réduction des boutiques : jamais essayés avec un vrai jeu | Essai par l’utilisateur |
| 30 | 2026-11-09 | critique, corrigé | Retour (B, O, Échap) arrêtait toute l’interface en quittant une page : verrou de `screens` tenu par un `if let` (Rust 2021) puis écriture, WebAssembly s’arrêtait | Vérifié dans le navigateur et dans l’exécutable publié ; panics désormais écrits dans la console |
| 31 | 2026-11-09 | haute, corrigé | Démarrage de 17 à 21 s : mode console et plan Maximal avant l’affichage, commandes synchrones sur le fil de la fenêtre | Interface en 2,3 s mesurée au journal ; plan fini derrière |
| 32 | 2026-11-09 | moyenne, à essayer | En jeu, Retour et Échap dans le menu latéral : actions de manette transmises quand Focus est affiché, premier plan repris par `AttachThreadInput` | Essai par l’utilisateur avec un vrai jeu |
| 33 | 2026-11-09 | haute, corrigé | Manette PlayStation vue deux fois dans les jeux hors Steam (manette virtuelle sans HidHide) | Natif par défaut sans HidHide ; test `ds4_without_hidhide_should_stay_native_to_avoid_double_pads` ; 🧑 à confirmer en jeu |
| 34 | 2026-11-09 | haute, corrigé | Bureau visible pendant l'anticheat ou le lanceur ; pas de retour après fermeture (Fortnite, Wuthering Waves) | Vérifié avec Yu-Gi-Oh! Master Duel : écran de lancement jusqu'au jeu, fermeture par le menu puis « Session terminée » seule ; 🧑 à confirmer sur Fortnite et Wuthering Waves |
| 35 | 2026-11-09 | haute, corrigé | Deux Focus ouverts en même temps ; fenêtre cachée impossible à fermer sans le Gestionnaire des tâches | Instance unique vérifiée : le second lancement ramène la fenêtre |
| 36 | 2026-11-09 | moyenne, ouvert | Deux instances cachées n'ont pas répondu à WM_CLOSE pendant un essai (cause non prouvée, pas de débogueur sur ce PC) | À surveiller ; l'instance unique évite la situation |
| 37 | 2026-11-09 | moyenne, corrigé | Navigation lente partout : sur un écran à 200 % (rendu 3840×2160), voile du fond teinté par la couleur du jeu repeint en plein écran à chaque cran, filtre `drop-shadow` sur le logo, ombres de boutons animées, double attente d'image | Focus déplacé en environ 20 ms dans le navigateur ; 🧑 ressenti à confirmer à la manette |
| 38 | 2026-11-09 | basse, corrigé | Lanceur ajouté à la main nommé « launcher » | Nom tiré du dossier pour les noms génériques ; entrée Wuthering Waves renommée |
| 39 | 2026-11-09 | haute, corrigé | Lanceur de Wuthering Waves pris pour le jeu (« game window shown by launcher_main ») : bureau visible, raccourci de fermeture visant le lanceur | Un exécutable « launcher » n'est jamais le jeu ; fermeture douce sur le jeu à l'écran ; 🧑 à confirmer |
| 40 | 2026-11-09 | haute, corrigé | `has_anticheat` ne regardait que la racine : ACE (Wuthering Waves) et EasyAntiCheat (Fortnite) non vus, un jeu manuel pouvait être ouvert (priorité, arrêt forcé) malgré SPEC D7 | Recherche sur 6 niveaux, 3 tests ; vérifié sur les vrais dossiers : Wuthering Waves et Fortnite vrais, Master Duel faux |
| 41 | 2026-11-09 | haute, corrigé | Jeux gacha en administrateur (`Client-Win64-Shipping.exe` : requireAdministrator) : Windows refuse WM_CLOSE et le premier plan à Focus (UIPI) | Focus en administrateur (décision 21) ; programmes ordinaires lancés avec les droits normaux, vérifié (S-1-16-8192) ; 🧑 fermeture et bouton central à confirmer en jeu |
| 42 | 2026-11-09 | haute, corrigé (à installer) | Manette PlayStation inutilisable dans les jeux sans prise en charge, ou en double avec Focus Input | HidHide piloté par son outil officiel, journalisé et rendu (fin, démarrage, sentinelle) ; mode Auto/Natif/Focus Input dans les paramètres ; ViGEmBus et HidHide au catalogue ; 🧑 installer HidHide puis essayer |

| 43 | 2026-11-11 | haute, corrigé | Ouverture de Focus : 23 à 27 s pour appliquer le plan, dont 20 s à attendre la fermeture de Game Bar (sans fenêtre) ; sa relance refusée laissait le journal sur le disque (« recovery incomplete » à chaque démarrage) | Échec immédiat sans fenêtre ; une application du Windows Store n'est plus relancée par son chemin ; 2 tests |
| 44 | 2026-11-11 | haute, corrigé | Un jeu lancé pendant le passage en mode console appliquait une seconde session par-dessus le plan (journal du 2026-11-10, 07:47 et 08:37) | Verrou tenu pendant tout le passage : le lancement attend |
| 45 | 2026-11-11 | haute, à essayer | Mode console v2 (décision 23) : arrêt et relance de l'Explorateur, lancement par jeton sans bureau, balayage, annonce du profil, musique | 🧑 Jamais vu dans l'exécutable publié. Premier essai ensemble : ouvrir Focus, vérifier que le bureau disparaît, lancer un jeu Steam et un jeu ajouté à la main, quitter, vérifier que le bureau revient. Secours : Ctrl+Maj+Échap, Fichier, Exécuter, `explorer.exe` |
| 46 | 2026-11-11 | moyenne, ouvert | Focus s'est arrêté deux fois sans sortie normale le 2026-11-10 (07:46 et 08:36, dont une pendant Fortnite) : « console mode undone after an unexpected exit » | Cause non trouvée. La sentinelle relance maintenant toujours le bureau |
| 47 | 2026-11-11 | moyenne, ouvert | Sans bureau, les jeux du Microsoft Store et de l'application Xbox peuvent mal se lancer | 🧑 À essayer ; sinon désactiver « Masquer complètement le bureau Windows » dans les paramètres |

| 48 | 2026-11-12 | haute, à essayer | Mise à jour, signalement et écran Téléchargements (décision 24) écrits et testés hors machine réelle | 🧑 Publier une première version signée, puis une seconde, et vérifier que Focus la propose et l'installe ; vérifier que la fiche GitHub s'ouvre, bureau arrêté |

| 49 | 2026-11-12 | critique, ouvert | Le PC a redémarré sans s'éteindre proprement quatre fois le 2026-11-12 (06:30, 07:13, 08:02, 10:36, événement Kernel-Power 41), sans vidage mémoire. Focus ne tournait que la première fois, au lancement d'un jeu à anticheat avec la manette masquée par HidHide. Les arrêts « inexpliqués de Focus » du 2026-11-10 (n° 46) sont sans doute les mêmes | Cause non trouvée : matériel, alimentation, pilote ou conflit HidHide et anticheat. 🧑 Dire si ces redémarrages sont volontaires ; Focus Input n'est plus choisi tout seul (décision 25) |
| 50 | 2026-11-12 | haute, corrigé | Premier essai réel (décision 25) : manette restée masquée après l'arrêt du PC, terminal pris pour le jeu, journal bloqué, changement de niveau sans effet avant redémarrage de Focus, sélecteur irrégulier | Corrigé et testé (190 tests) ; 🧑 à revoir à l'écran |

## Contradictions des specs, réglées par la règle la plus prudente

Priorité haute (v1) contre au-dessus de la normale (v4.1) ; installation des jeux (v4) contre hors périmètre (v4.4) ; suspension en v1 (v2) contre reportée (v4.3) ; interface toujours native (v2) contre lancement via Steam (v1). Le détail est dans `SPEC.md`, section 2.
