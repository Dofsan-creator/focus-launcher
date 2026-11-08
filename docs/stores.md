# Boutiques : détection et lancement

État au 2026-11-07. « Testé » signifie testé sur de faux dossiers ; le test sur ta machine est la condition 17.

| Boutique | Détection (lecture seule) | Lancement | Testé |
| --- | --- | --- | --- |
| Steam | `HKCU\Software\Valve\Steam\SteamPath`, `libraryfolders.vdf`, `appmanifest_*.acf` | `steam://rungameid/<id>` | Faux dossiers : bibliothèques multiples, outils filtrés, fichiers cassés |
| Epic Games | `ProgramData\Epic\EpicGamesLauncher\Data\Manifests\*.item`, catégorie `games`, installations incomplètes exclues | `com.epicgames.launcher://apps/<id>?action=launch&silent=true` | Faux manifestes |
| GOG | `HKLM\SOFTWARE\GOG.com\Games` (vue 32 bits), `goggame-<id>.info`, tâche principale | Exécutable principal | Faux dossiers, DLC exclus |
| Ubisoft Connect | `HKLM\SOFTWARE\Ubisoft\Launcher\Installs` | `uplay://launch/<id>/0` | Nom tiré du dossier |
| EA app | Clés de désinstallation (éditeur Electronic Arts) + meilleur exécutable | Exécutable | Faux dossiers. Format EA instable : méthode de repli |
| Battle.net | Clés de désinstallation (éditeur Blizzard) + meilleur exécutable | Exécutable | Comme EA |
| Xbox et Microsoft Store | `<disque>:\XboxGames\*\Content\MicrosoftGame.config` + `Get-StartApps` | `shell:AppsFolder\<AUMID>`, jamais l'exécutable | Faux dossier |

## Démarrage réduit des clients

Non vérifié par client. À noter ici après essai : option de démarrage réduit, ou repli par minimisation de la fenêtre.

## Big Picture

Détection par titre de fenêtre, puis `steam://close/bigpicture`. **Ce lien n'est pas documenté par Valve** : il peut cesser de fonctionner. Un échec n'arrête jamais le launcher. Le blocage d'une ouverture par le bouton central de la manette n'est pas garanti.
