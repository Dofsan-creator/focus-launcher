---
name: analyse-de-phase
description: Use before starting any phase or structural change of Focus Launcher, when choosing a library, an API or an approach, or when the user says "analyse", "meilleure option", "quelle approche".
---
# Analyse de phase (v4.5)

1. Cadrer : objectif, conditions d'arrêt concernées (`PROGRESS.md`), skills utiles.
2. Lister 2 à 4 options réalistes, dont une que les specs ne citent pas. Vérifier tout fait susceptible d'avoir changé dans la source officielle, pas de mémoire (skill agent-reach pour GitHub et le web).
3. Comparer, dans cet ordre : sécurité de l'utilisateur, règles du projet, fiabilité, expérience, simplicité, évolutivité, coût.
4. Prototype jetable si un doute réel décide du choix.
5. Écrire l'entrée dans `docs/decisions.md` : contexte, options, choix, écarté, preuve.
6. Construire, puis faire le bilan dans `PROGRESS.md`.

Une exigence explicite de l'utilisateur ne change jamais sans son accord : noter la proposition sous « Propositions de changement ».
