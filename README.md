<h1 align="center">Focus Launcher</h1>

<p align="center">
  <strong>Ton PC Windows se comporte comme une console.</strong><br>
  Tu l'allumes, tu prends la manette, tu joues.
</p>

<p align="center">
  <img alt="Windows 11" src="https://img.shields.io/badge/Windows-11-0b0d10">
  <img alt="Écrit en Rust" src="https://img.shields.io/badge/%C3%A9crit%20en-Rust-e9a23b">
  <img alt="Licence GPL-3.0" src="https://img.shields.io/badge/licence-GPL--3.0-6fd39a">
  <img alt="11 langues" src="https://img.shields.io/badge/langues-11-aeb2bb">
</p>

<p align="center">
  <a href="#installer">Installer</a> ·
  <a href="#ce-que-fait-focus">Ce que fait Focus</a> ·
  <a href="#tes-garanties">Tes garanties</a> ·
  <a href="#un-problème-">Un problème ?</a> ·
  <a href="#english">English</a>
</p>

---

Focus Launcher est fait pour quelqu'un qui vient d'une console et qui ne veut pas apprendre Windows. Tant que Focus est ouvert, le bureau disparaît, tout ce qui ne sert pas au jeu est mis de côté, et tout se pilote à la manette. Quand tu le quittes, ton Windows revient exactement comme avant.

> **État du projet : en développement.** Focus marche sur le PC de son auteur, mais aucune version stable n'est encore publiée. Plusieurs fonctions récentes n'ont été vérifiées que par des tests automatiques. Les points ouverts sont listés sans fard dans [`AUDIT.md`](AUDIT.md).

## Ce que fait Focus

| | |
| --- | --- |
| **Mode console** | Le bureau de Windows, la barre des tâches et les notifications disparaissent tant que Focus est ouvert. La touche Windows et le bouton central de la manette ramènent à Focus. |
| **Tous tes jeux au même endroit** | Steam, Epic Games, GOG, EA, Ubisoft Connect, Battle.net et Xbox sont détectés tout seuls. Tu peux ajouter n'importe quel programme à la manette. |
| **Trois façons de piloter** | Manette (PlayStation ou Xbox), souris, clavier : chaque écran marche avec les trois. |
| **Un PC allégé pour le jeu** | Les programmes inutiles sont ralentis, les tâches de fond de Windows sont mises en pause, la puissance passe au maximum. Focus te montre ce qui a changé, avant et maintenant. |
| **Menu en jeu** | Depuis un jeu : y retourner, le fermer proprement, faire une capture d'écran, régler le Wi-Fi ou le Bluetooth. |
| **Téléchargements** | La progression des jeux que Steam télécharge, et ce que Focus installe. |
| **Mise à jour intégrée** | Focus se met à jour tout seul depuis ce dépôt. Un installeur qui ne porte pas sa signature est refusé. |
| **11 langues** | Français, anglais, espagnol, allemand, italien, portugais du Brésil, russe, japonais, coréen, chinois simplifié et arabe (de droite à gauche). |

## Tes garanties

- **Tout est rendu.** Chaque changement est noté sur le disque *avant* d'être fait, puis défait quand tu quittes Focus. Si Focus plante, une sentinelle le défait à sa place ; sinon, le démarrage suivant s'en charge.
- **Rien de permanent.** Aucun réglage du système n'est modifié pour de bon. Un simple redémarrage du PC rend le bureau.
- **Rien n'est fermé de force.** Une application qui te demande d'enregistrer reste ouverte.
- **Tes jeux ne sont jamais touchés.** Aucune injection, et Focus n'ouvre jamais le processus d'un jeu protégé par un anticheat.
- **Une liste de protection compilée dans le logiciel** : Windows, l'antivirus, les pilotes, les anticheats et les boutiques ne peuvent pas être visés, même par un fichier de réglages modifié.
- **Aucun identifiant lu ni gardé**, aucune clé cachée dans le logiciel.

Le détail des règles est dans [`SPEC.md`](SPEC.md), et chaque choix est expliqué dans [`docs/decisions.md`](docs/decisions.md).

## Installer

Les versions seront publiées dans l'onglet [**Releases**](https://github.com/Dofsan-creator/focus-launcher/releases). Aucune n'y est encore : pour l'instant, Focus se compile depuis les sources (voir plus bas).

À savoir pour la première installation :

- Windows 11 est requis.
- Windows affichera un avertissement SmartScreen, parce que l'installeur n'est pas encore signé par un certificat payant. Choisis « Informations complémentaires », puis « Exécuter quand même ».
- Focus demande les droits administrateur à l'ouverture : il en a besoin pour fermer les jeux qui tournent eux-mêmes en administrateur.

Si le bureau ne revenait pas en quittant Focus : Ctrl+Maj+Échap, « Fichier », « Exécuter », puis `explorer.exe`.

## Un problème ?

Dans Focus : **Paramètres, Aide et mise à jour, Signaler un problème**. Tu décris ce qui s'est passé, et ton navigateur s'ouvre sur une fiche déjà remplie. Ton nom et ton dossier personnel sont retirés du rapport avant qu'il ne quitte ton PC.

Tu peux aussi écrire directement dans les [signalements](https://github.com/Dofsan-creator/focus-launcher/issues).

## Compiler depuis les sources

```text
rustup target add wasm32-unknown-unknown
cargo install trunk --locked
cargo install tauri-cli --version "^2" --locked
cd crates/focus-app
cargo tauri dev
```

Tests : `cargo test --workspace --exclude focus-ui`, dans un terminal administrateur.

Focus est écrit entièrement en Rust : [Tauri 2](https://tauri.app) pour l'application, [Leptos](https://leptos.dev) compilé en WebAssembly pour l'interface.

| Dossier | Rôle |
| --- | --- |
| `crates/focus-core` | Règles, liste de protection, journal et restauration. Sans dépendance à Windows, donc testable partout. |
| `crates/focus-library` | Lecture des boutiques et de la bibliothèque. |
| `crates/focus-input` | Manettes. |
| `crates/focus-sys` | Tout ce qui parle à Windows. |
| `crates/focus-guardian` | Petit service qui met en pause des tâches de Windows et les relance. |
| `crates/focus-app` | L'application. |
| `crates/focus-ui` | L'interface. |
| `locales/` | Un fichier par langue. |

## Participer

Les traductions sont le meilleur point d'entrée : une langue, c'est un fichier dans `locales/`, et un test vérifie qu'aucun texte ne manque. Les traductions marquées « à relire » attendent quelqu'un dont c'est la langue.

## Licence

[GPL-3.0 ou ultérieure](COPYING). Focus s'inspire du comportement des consoles, sans en reprendre l'apparence, les noms, les sons ni les icônes. Il n'est affilié à aucun constructeur ni à aucune boutique ; les marques citées appartiennent à leurs propriétaires.

---

## English

**Focus Launcher makes a Windows PC behave like a console.** It is built for people coming from a console who do not want to learn Windows: while Focus is open, the desktop is gone, everything a game does not need is set aside, and every screen works with a controller. When you exit, Windows comes back exactly as it was.

- Finds your Steam, Epic Games, GOG, EA, Ubisoft Connect, Battle.net and Xbox games on its own.
- Controller, mouse and keyboard on every screen.
- Slows unneeded programs, pauses Windows background tasks, switches to full power, and shows you what changed.
- Every change is written to disk before it happens and undone on exit, after a crash, or at the next start. Nothing is permanent, nothing is force-closed, and games are never touched.
- Built-in signed updates, a downloads screen and a problem report that strips your name before it leaves your PC.
- 11 languages, including right-to-left Arabic.

**Status: in development.** No stable release yet; open points are listed in [`AUDIT.md`](AUDIT.md). Written in Rust (Tauri 2 and Leptos). Licence: GPL-3.0-or-later.
