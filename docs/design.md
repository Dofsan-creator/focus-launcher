# Charte graphique : direction « Iris »

Le nom « Focus » invite à la photographie. La métaphore retenue est le diaphragme : il s'ouvre sur un jeu et se ferme pour le lancer.

## Écrans

| Écran | Signature |
| --- | --- |
| Démarrage | Un anneau de 8 lames tourne de 45° puis s'efface (900 ms). |
| Accueil | Les jeux reposent sur un **arc** dont le centre est sous l'écran. La sélection est au sommet, entourée d'un anneau à 8 lames. Le fond prend la teinte du jeu sélectionné. |
| Lancement | L'écran se ferme en losange vers le centre (420 ms), comme un iris qui se ferme. |
| Panneau rapide | Panneau latéral côté fin de ligne (droite en LTR, gauche en RTL). |

## Jetons (`crates/focus-ui/style.css`)

| Jeton | Sombre | Clair |
| --- | --- | --- |
| `--bg` | `#0b0d10` | `#efeae0` |
| `--accent` | `#e9a23b` (ambre) | `#a2600c` |
| `--danger` | `#e5604d` | idem |
| `--fast` / `--slow` | 160 / 280 ms | 1 ms si « réduire les animations » |
| Courbe | `cubic-bezier(0.3, 1.25, 0.45, 1)`, léger dépassement | |

Taille de texte : `clamp(16px, 1.25vw, 24px)`, lisible depuis un canapé. Un seul élément focalisé à la fois : bordure ambre et agrandissement de 3 %.

## Icônes de boutons

Formes ✕ ○ pour le style « Formes », lettres A B pour le style « Lettres ». Ce sont des caractères Unicode, pas des icônes de constructeur.

## Sons

Trois tons synthétisés (déplacement 520 Hz, validation 760 Hz, retour 340 Hz), désactivables.

## Vocabulaire

« Panneau rapide » (pas « Control Center »), « Reprendre » (pas « Quick Resume »).
