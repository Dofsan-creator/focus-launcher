# Focus Launcher : correctif v4.5 (analyse et meilleure option à chaque phase)

Ce correctif **complète** la v4.4 et s'applique à **toutes les phases**, présentes et futures. Il ne change aucune fonctionnalité : il change **la façon de travailler**.

---

## 1. La règle, en une phrase

**À chaque phase, avant de construire, Claude Code analyse les options possibles, choisit celle qui donne le meilleur résultat pour le projet, et écrit pourquoi.**

« Meilleur » veut dire meilleur **pour Focus Launcher** (ses règles, ses utilisateurs), pas le plus simple à écrire ni le plus à la mode.

---

## 2. Le déroulement de chaque phase

| Étape | Ce que Claude Code fait | Trace |
| --- | --- | --- |
| **1. Cadrer** | Relire l'objectif de la phase, les conditions d'arrêt concernées et les skills utiles (v4.4, section 2) | `PROGRESS.md` |
| **2. Chercher les options** | Lister **2 à 4 options réalistes**, y compris celle que le texte suggère et au moins une **alternative qu'il ne mentionne pas**. Pour tout fait susceptible d'avoir changé (documentation d'un outil, comportement d'une API, gestionnaire de paquets), **vérifier dans la source officielle** au lieu de se fier à la mémoire | `docs/decisions.md` |
| **3. Comparer** | Noter chaque option selon les **critères** de la section 3 | `docs/decisions.md` |
| **4. Essayer si le doute est réel** | Si un point risqué décide du choix, faire un **court prototype jetable** et noter le résultat, plutôt que de deviner | `docs/decisions.md` |
| **5. Choisir et écrire** | Retenir **une** option, écrire **pourquoi** et **ce qui a été écarté** | `docs/decisions.md` |
| **6. Construire** | Implémenter l'option retenue | Code et tests |
| **7. Faire le bilan** | Relire le résultat contre les conditions d'arrêt, noter ce qui marche, ce qui est fragile, et **ce qu'on améliorerait** | `PROGRESS.md` |

**Proportion :** une petite phase demande une analyse de **quelques lignes** ; une phase risquée (mise en pause, installation, démarrage automatique, curseur à la manette) demande une analyse **complète**. L'analyse **ne doit pas retarder** la phase : si deux options sont équivalentes, choisir la plus simple et le noter.

---

## 3. Critères de comparaison

Dans cet ordre de priorité quand ils s'opposent :

1. **Sécurité et respect de l'utilisateur** : aucune perte de données, aucune action surprenante, confirmation avant tout ce qui modifie le système.
2. **Règles du projet** : originalité et noms (v4.2), trois entrées (v4.3), toutes les langues (v4.4), sources officielles seulement (v4.4, section 3).
3. **Fiabilité** : fonctionne sur des machines et des jeux variés, échoue proprement.
4. **Expérience d'usage** : fluide, naturelle, lisible à distance de l'écran.
5. **Simplicité et testabilité** : peu de pièces, vérifiable automatiquement.
6. **Évolutivité** : n'empêche pas les versions suivantes.
7. **Coût** : temps, dépendances, poids.

---

## 4. Quand la meilleure option contredit le texte

L'agent ne suit pas aveuglément le texte et ne s'en écarte pas en silence.

- Si une option **meilleure** que celle prévue existe, l'agent l'**écrit sous « proposition de changement »** dans `docs/decisions.md` avec son gain et son risque.
- Il applique **par défaut ce que le texte demande**, sauf si l'écart est **mineur, réversible et clairement meilleur** : dans ce cas il l'applique et le signale dans `PROGRESS.md`.
- Pour tout changement qui touche une **exigence explicite de l'utilisateur** (toutes les langues, démarrage au choix de l'utilisateur, trois entrées, installation depuis le launcher), il **ne change rien** sans l'aval de l'utilisateur : il note la proposition sous « à trancher » et continue avec l'option prévue.
- Les **skills de l'utilisateur** ne sont jamais modifiés (v4.4).

---

## 5. Ajouts aux autres fichiers

- **`CLAUDE.md`** : ajouter la règle en une phrase (section 1) et le renvoi vers ce fichier.
- **Nouveau skill `analyse-de-phase`** dans `.claude/skills/` : décrit les sept étapes, fournit un **modèle d'entrée** pour `docs/decisions.md` (contexte, options, notes par critère, choix, écartées, résultat du prototype) et un modèle de bilan.
- **`docs/decisions.md`** : journal des décisions, **une entrée par phase**, conservé dans le dépôt.

---

## 6. Plan de travail et conditions d'arrêt

Cette règle s'applique **dès la Phase 0** et à chaque phase suivante.

| # | Condition | Comment la vérifier |
| --- | --- | --- |
| 37 | **Chaque phase terminée** a une entrée dans `docs/decisions.md` avec au moins deux options comparées, le choix et ce qui a été écarté | Lecture du fichier |
| 38 | Chaque phase terminée a un **bilan** dans `PROGRESS.md` (ce qui marche, ce qui est fragile, ce qu'on améliorerait) | Lecture du fichier |
| 39 | Les phases risquées (mise en pause, installation, démarrage automatique, curseur à la manette, langues) ont un **prototype ou un test** noté avant le choix final | Lecture de `docs/decisions.md` |
| 40 | Toute **proposition de changement** touchant une exigence explicite est listée sous « à trancher », sans avoir été appliquée | Lecture de `PROGRESS.md` |

---

## 7. À confirmer

Rien de nouveau.
