# DualShock 4 en Bluetooth avec Steam Input

Ce guide sert au mode « Steam Input ». En mode Natif, Focus Launcher lit déjà la DualShock 4 directement, sans Steam.

## 1. Appairer la manette

1. Ouvre **Paramètres Windows → Bluetooth et appareils → Ajouter un appareil → Bluetooth**.
2. Maintiens **Share** et le bouton central jusqu'à ce que la barre lumineuse clignote vite.
3. Choisis **Wireless Controller**.

## 2. Activer la prise en charge dans Steam

1. Steam → **Paramètres → Manette**.
2. Active la prise en charge des manettes PlayStation.

## 3. Ajouter Focus Launcher à Steam (facultatif)

Focus Launcher ne modifie jamais les fichiers de Steam : cet ajout se fait à la main.

1. Steam → **Jeux → Ajouter un jeu non-Steam à ma bibliothèque**.
2. Choisis `Focus Launcher.exe`.
3. Lance Focus Launcher depuis Steam : Steam présente alors la manette comme une manette Xbox.

## Pièges

- **Double saisie** : n'utilise pas DS4Windows en même temps que Steam Input.
- **Icônes** : quand Steam traduit la manette, Focus Launcher peut voir une manette Xbox. Règle **Paramètres → Icônes des boutons → Formes**.
- **Steam fermé** : Focus Launcher repasse seul en mode Natif.

## À vérifier sur ta machine

- Si Steam masque la manette physique, la détection automatique peut échouer : c'est pour cela que le réglage manuel des icônes existe.
- Certaines sources signalent qu'en Bluetooth, il faut parfois éteindre et rallumer la manette pour que Steam la voie après un changement de mode.
