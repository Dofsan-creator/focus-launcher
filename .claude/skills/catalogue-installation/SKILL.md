---
name: catalogue-installation
description: Add an application to the install catalog or try a real installation. Manual invocation only.
disable-model-invocation: true
---
# Catalogue d'installation

1. Vérifier l'identifiant : `winget show --id <Id> -e`. Ne jamais l'inventer.
2. Ajouter une ligne dans `crates/focus-library/data/catalog.json` : id, nom, catégorie (`store` ou `social`), `wingetId`, page officielle en https, éditeur, processus.
3. `cargo test -p focus-library catalog`.
4. Un essai d'installation réel modifie la machine : seulement à la demande de l'utilisateur.
