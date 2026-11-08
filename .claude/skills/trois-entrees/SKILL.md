---
name: trois-entrees
description: Use when adding or changing any screen, dialog or control of the interface, or when the user mentions manette, clavier, souris, focus, navigation.
---
# Manette, souris, clavier

- Tout élément interactif est un `<button class="f">`. Il est cliquable à la souris, focalisable au clavier et atteint à la manette par la navigation spatiale (`focus-core::spatial`, `focus-ui/src/nav.rs`).
- Un dialogue ou un panneau utilise `.dialog`, `.panel` ou `.osk` : le focus y reste piégé, et Retour (B, Échap) le ferme.
- L'élément proposé par défaut porte `data-auto`. Les actions destructrices ont « Non » en `data-auto`.
- Aucune saisie de texte obligatoire sans le clavier à l'écran (`TextOverlay`).
- Vérifier : tests de `focus-core::spatial`, puis la liste de contrôle manuelle de `PROGRESS.md`.
