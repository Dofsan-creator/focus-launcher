# Charte graphique : direction « Lumen »

Inspirée du comportement des consoles (l’image du jeu domine, une seule rangée, un appui pour jouer), exécutée à notre façon. Décision 9 de `docs/decisions.md`.

## Écrans

| Écran | Signature |
| --- | --- |
| Démarrage | Un anneau dont un quart prend la couleur d’accent tourne, puis s’efface (1,1 s). |
| Premier démarrage | Huit étapes : langue, mode console, manette, connexions, bibliothèque, performances, son, prêt. Points de progression en haut. |
| Accueil | L’image de fond du jeu sélectionné remplit l’écran (fondu entre deux couches). Logo ou titre à gauche, bouton Jouer à la couleur du jeu. Rangée de jaquettes verticales en bas ; la piste glisse, la sélection reste à gauche et grandit de 12 %. |
| Lancement | La jaquette s’avance sur l’image du jeu assombrie, puis la fenêtre se cache. |
| Panneau rapide | Feuille qui monte du bas : reprendre ou fermer le jeu, Wi-Fi, Bluetooth, réglages, alimentation. |
| Connexions | Onglets Wi-Fi et Bluetooth, interrupteur de radio, listes de réseaux et d’appareils. |

## Jetons (`crates/focus-ui/style.css`)

| Jeton | Valeur |
| --- | --- |
| `--bg` | `#08090c` (clair : `#eef0f4`) |
| `--accent` | couleur dominante de l’image du jeu, sinon `#e9a23b` |
| `--fast` / `--mid` / `--slow` | 140 / 260 / 520 ms, 1 ms si « réduire les animations » |
| Courbes | `--ease` (sortie douce), `--spring` (léger rebond pour le focus) |

Focus : anneau clair de 3 px et halo de la couleur d’accent. Seuls `transform` et `opacity` sont animés.

## Icônes et sons

Icônes dessinées pour le projet (`crates/focus-ui/src/icons.rs`), traits sur une grille de 24. Sons et musique d’ambiance synthétisés (`sound.rs`), aucun fichier audio.

## Vocabulaire

« Panneau rapide », « Mode console », « Connexions ». Aucun terme de constructeur.
