# Journal des décisions

Une entrée par choix structurant (v4.5). Format : contexte, options, choix, écarté, preuve.

## 1. Cadre d'interface

- **Contexte** : Rust imposé, droite-gauche et toutes les écritures exigés, fluidité et faible consommation pendant un jeu.
- **Options** : Slint (100 % Rust natif), Tauri 2 + Leptos (Rust en WebAssembly, rendu WebView2), egui, wgpu fait main.
- **Choix** : Tauri 2 + Leptos.
- **Écarté** : Slint, son ticket RTL #2294 est toujours ouvert (vérifié avec agent-reach). egui : pas de mise en forme bidirectionnelle complète ni d'accessibilité aboutie. wgpu fait main : il faudrait réécrire saisie, texte et accessibilité.
- **Preuve** : WebView2 157 présent sur la machine. Rendu stoppé pendant un jeu en cachant la fenêtre.

## 2. Droits administrateur

- **Options** : tout en administrateur (v1) ; petit assistant élevé à chaque session (v3) ; service installé une fois.
- **Choix** : service `focus-guardian`, facultatif.
- **Écarté** : l'administrateur pour tout, car les jeux en héritent. L'assistant à chaque session affiche une fenêtre Windows impossible à valider à la manette.
- **Garde-fous** : liste compilée de 8 services, canal nommé réservé aux utilisateurs interactifs, restauration à la déconnexion du client.

## 3. Navigation aux trois entrées

- **Options** : graphe de focus déclaré par écran ; navigation spatiale sur de vrais boutons.
- **Choix** : navigation spatiale (`focus-core::spatial`) sur des `button`.
- **Pourquoi** : un seul chemin pour la manette, le clavier, la souris et les lecteurs d'écran. Le droite-gauche fonctionne sans cas spécial, le raisonnement étant géométrique.

## 4. Focus Input sans DS4Windows

- **Options** : intégrer DS4Windows (C#, GPL v3) ; le piloter ; écrire le nôtre.
- **Choix** : écrit pour le projet (`focus-input::ds4`, `focus_input`), manette virtuelle par `vigem-client` (MIT).
- **Pourquoi** : un code C# ne s'intègre pas à Rust, et une traduction resterait une œuvre dérivée.

## 5. Traductions

- **Options** : `.resx` (spec v3, .NET), ICU MessageFormat, Project Fluent.
- **Choix** : Fluent. Pluriels et paramètres comme ICU, bibliothèque Rust maintenue sous Apache-2.0 (vérifié).

## 6. Suivi du jeu sans toucher au jeu

- **Options** : ouvrir le processus du jeu (priorité, état) ; suivre par nom d'exécutable.
- **Choix** : suivi par nom (`scan::process_names`, sans aucun handle). Priorité relevée seulement pour un jeu lancé directement, sans anticheat détecté.

## 7. Design

- **Options** : pistes Lentille, Iris, Pellicule (v4).
- **Choix** : Iris. Jeux sur un arc, diaphragme à 8 lames autour de la sélection, lancement par fermeture de l'iris.
- **Pourquoi** : la plus éloignée des rangées droites des consoles, et sans flou (coût GPU nul en arrière-plan).

## Propositions de changement (à trancher par l'utilisateur)

1. Livrer d'abord les 11 langues actuelles, puis les autres par la communauté (v4.4 demande toutes les langues de Windows avant la sortie).
2. Remplacer Tab sur l'accueil (changement de filtre, v4.3) par une touche dédiée, pour garder Tab à la navigation standard des lecteurs d'écran.
