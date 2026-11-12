app-name = Focus Launcher
lang-title = Choisis ta langue
welcome-title = Bienvenue
welcome-body = Tout ici fonctionne à la manette, à la souris ou au clavier. Rien ne change sur ton PC sans ton accord.
welcome-autostart = Ouvrir à l'ouverture de session
welcome-sim = Les sessions démarrent en simulation : tu vois ce qui changerait, et rien n'est touché.
welcome-continue = Continuer

filter-all = Tout
filter-games = Jeux
filter-software = Logiciels
filter-social = Social

top-add = Ajouter
top-stores = Boutiques
top-settings = Paramètres
top-power = Alimentation

home-resume = Reprendre
home-empty-title = Rien ici pour l'instant
home-empty-body = Les jeux installés apparaissent tout seuls. Tu peux aussi ajouter un programme.
home-played = { $hours ->
    [one] { $hours } heure d'utilisation
   *[other] { $hours } heures d'utilisation
}
home-running = En cours

detail-play = Jouer
detail-open = Ouvrir
detail-favorite = Ajouter aux favoris
detail-unfavorite = Retirer des favoris
detail-remove = Retirer de la bibliothèque
detail-launches = { $count ->
    [one] { $count } lancement
   *[other] { $count } lancements
}
detail-source = Origine : { $store }

prep-title = Préparation
prep-simulation = Simulation : rien ne sera modifié.
prep-real = Ces changements seront annulés quand tu auras fini.
prep-nothing = Rien à changer pour cette session.
prep-start = Lancer
prep-cancel = Annuler
prep-throttle = Ralentir en arrière-plan : { $name }
prep-suspend = Mettre en pause : { $name }
prep-close = Fermer : { $name }
prep-service = Arrêter pour l'instant : { $name }
prep-awake = Garder le PC éveillé
prep-power = Mode d'alimentation hautes performances
prep-ask = À toi de choisir : { $name }
prep-no-guardian = Les changements de services sont ignorés : le service d'aide n'est pas installé.

end-title = Session terminée
end-time = { $minutes ->
    [one] { $minutes } minute
   *[other] { $minutes } minutes
}
end-restored = Tout a été restauré.
end-problem = Certains éléments n'ont pas pu être restaurés. Les détails sont dans le journal.
end-back = Retour à la bibliothèque

panel-title = Panneau rapide
panel-return = Y retourner
panel-close-entry = Le fermer
panel-library = Retour à la bibliothèque
panel-none = Rien n'est en cours

close-title = Fermer { $title } ?
close-warning = La progression non sauvegardée sera perdue.
close-social = Un appel ou une diffusion peut s'arrêter.
close-no = Non
close-yes = Oui, fermer
close-waiting = Fermeture en cours. Il te demande peut-être d'enregistrer.
close-force = Forcer la fermeture
close-force-sure = Forcer la fermeture peut faire perdre du travail. Tu es sûr ?

set-title = Paramètres
set-language = Langue
set-lang-status = État de la traduction : { $status }
status-reviewed = relue
status-draft = à relire
set-autostart = Ouvrir à l'ouverture de session
set-real = Optimisation réelle
set-real-body = Tant que Focus est ouvert, ce qui ne sert pas au jeu est mis de côté : programmes ralentis, tâches de fond de Windows en pause, puissance maximale. Tout revient quand tu quittes Focus.
set-real-confirm = Autoriser de vrais changements sur ce PC pendant les sessions ?
set-keep-social = Ne pas toucher aux applications sociales
set-awake = Garder le PC éveillé pendant une session
set-high-perf = Mode d'alimentation hautes performances
set-big-picture = Bloquer l'autre mode plein écran de Steam
set-sounds = Sons
set-motion = Réduire les animations
set-theme = Thème selon l'heure
set-glyphs = Icônes des boutons
glyph-auto = Automatique
glyph-playStation = Formes
glyph-xbox = Lettres
set-detect = Détecter la manette
set-guardian = Service d'aide
set-guardian-body = Utile seulement pour mettre en pause des services Windows. Windows demande l'autorisation une fois, et cette fenêtre se valide à la souris ou au clavier.
set-guardian-on = Installé
set-guardian-off = Non installé
set-guardian-install = Installer
set-analysis = Analyse du système
state-on = Activé
state-off = Désactivé
yes = Oui
no = Non

ctl-family = Manette : { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = Non détectée
ctl-mode = Mode : { $mode }
mode-auto = Auto
mode-native = Natif
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = Aucun
ctl-connected = Manette connectée : { $family }

an-title = Analyse du système
an-processes = Processus : { $count }
an-services = Services : { $count }
an-memory = Mémoire : { $gb } Go
an-cpu = Processeur : { $name }
an-top = Les plus gourmands en mémoire
an-loading = Lecture de ton système. Rien n'est modifié.

add-title = Ajouter un programme
add-body = Choisis un fichier. Il est lu, jamais exécuté.
add-up = Dossier parent
add-suggest = Proposer depuis ce dossier
add-suggested = Fichiers les plus probables
add-name = Nom
add-kind = Type
add-save = Ajouter à la bibliothèque
add-script = C'est un script. Vérifie son chemin complet avant de l'ajouter.
kind-game = Jeu
kind-software = Logiciel
kind-social = Social
kind-other = Autre

store-title = Boutiques et applications
store-install = Installer
store-page = Page officielle
store-confirm = Installer { $name } de { $publisher } ?
store-source = Source : gestionnaire de paquets de Windows
store-uac = Windows peut demander une autorisation. Cette fenêtre se valide à la souris ou au clavier.
store-progress = Installation de { $name }
store-done = Installation terminée.
store-failed = L'installation a échoué. Tu peux passer par la page officielle.

power-title = Alimentation
power-quit = Quitter vers le bureau
power-restart = Redémarrer
power-shutdown = Éteindre
power-sure = Tu es sûr ?

kbd-done = Terminé
kbd-space = Espace
kbd-delete = Effacer

hint-select = Choisir
hint-back = Retour
hint-panel = Panneau rapide

err-generic = Un problème est survenu. Les détails sont dans le journal.
autostart-disabled = Le launcher n'a pas pu démarrer deux fois de suite : l'ouverture à l'ouverture de session a été désactivée.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = Rechercher
hint-options = Options
home-searching = Résultats pour « { $text } »
detail-remove-confirm = Retirer { $title } de la bibliothèque ?
set-section-console = Mode console
set-console = Passer Windows en mode console tant que Focus est ouvert
set-console-body = Le bureau, la barre des tâches et les notifications disparaissent. Quitte Focus pour retrouver ton Windows habituel.
set-hide-taskbar = Masquer la barre des tâches
set-silence = Mettre les notifications en sourdine
set-start-minimized = Démarrer réduit à l'ouverture de session
set-section-opt = Performances
set-section-sound = Son et affichage
set-music = Musique d'ambiance
set-vibration = Vibrations de la manette
opt-off = Désactivées
opt-off-body = Simulation : tu vois ce qui changerait, rien n'est touché.
opt-balanced = Équilibrées
opt-balanced-body = Les applications d'arrière-plan sont ralenties pendant une session, puis restaurées.
opt-max = Maximales
opt-max-body = Tous les leviers autorisés : applications ralenties, mises à jour en pause, hautes performances. Tout est remis en place ensuite.
power-quit-body = Windows retrouve son état normal.
power-restart-body = Redémarre le PC.
power-shutdown-body = Éteint le PC.
con-title = Connexions
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = Rechercher à nouveau
con-searching = Recherche…
con-connected = Connecté
con-connected-to = Connecté à { $name }
con-connect = Se connecter
con-disconnected = Déconnecté.
con-password = Mot de passe
con-none = Aucun réseau trouvé.
con-ethernet = Connexion filaire
con-offline = Non connecté
con-result-success = Connecté.
con-result-badPassword = Mot de passe incorrect.
con-result-timeout = Le réseau n'a pas répondu.
con-result-failed = La connexion a échoué.
con-result-notFound = Réseau introuvable. Relance la recherche.
con-result-noAdapter = Aucune carte Wi-Fi trouvée.
bt-paired = Tes appareils
bt-search = Chercher des appareils
bt-pair = Appairer
bt-pair-hint = Mets l'appareil en mode appairage, puis lance la recherche.
bt-forget-confirm = Oublier { $name } ?
bt-forgotten = Appareil oublié.
bt-result-success = Appareil appairé.
bt-result-failed = L'appairage a échoué.
setup-step = Étape { $current }
setup-next = Suivant
setup-back = Retour
setup-start = Commencer
setup-console-title = Ton PC devient une console
setup-console-body = Tant que Focus est ouvert, Windows s'efface : plus de bureau, plus de barre des tâches, plus de notifications. Quitte Focus et tout redevient normal.
setup-pad-title = Ta manette
setup-pad-body = Choisis l'affichage des boutons. Focus marche aussi à la souris et au clavier.
setup-net-title = Reste connecté
setup-net-body = Le Wi-Fi et le Bluetooth sont toujours à un bouton, même pendant un jeu.
setup-lib-title = Ta bibliothèque
setup-lib-body = { $count ->
    [one] { $count } jeu ou application trouvé sur ce PC.
   *[other] { $count } jeux et applications trouvés sur ce PC.
}
setup-opt-title = Performances pendant les sessions
setup-opt-body = Focus peut libérer ton PC pendant que tu joues, puis tout remettre en place. Tu choisis, et tu pourras changer plus tard.
setup-comfort-title = Son et confort
setup-comfort-body = Règle l'ambiance. Ouvrir Focus au démarrage de la session fait démarrer ton PC comme une console.
setup-done-title = Tout est prêt
setup-done-body = Ta bibliothèque t'attend. Appuie sur Commencer pour entrer.
setup-open-wifi = Configurer le Wi-Fi
setup-open-bt = Appairer une manette
store-installed = Installé
filter-stores = Stores
launch-starting = Lancement…
store-open = Ouvrir
panel-screenshot = Capture d’écran
shot-saved = Capture enregistrée dans Images\Focus.
detail-update = Mise à jour disponible
detail-update-open = Mettre à jour
power-sleep = Mettre en veille
power-sleep-body = Le PC se met en veille, comme le repos d’une console.
end-auto = Retour à la bibliothèque…
con-ethernet-on = Connecté par câble (Ethernet)
ctl-native-note = Hors Steam, Focus Input remplace la manette PlayStation par une manette Xbox, que tous les jeux reconnaissent. Pour éviter les commandes en double, il faut HidHide (dans la boutique de Focus, rubrique Pilotes) : Focus cache alors la vraie manette au jeu pendant la partie, puis la rend. Sans HidHide, « Auto » laisse la manette telle quelle.
steam-guide-on = Dans Steam, le bouton central de la manette ouvre encore Steam par-dessus tes jeux. Dans Steam : Paramètres, Contrôleur, Paramètres avancés, désactive « Le bouton Guide place Steam au premier plan » et « Activer la configuration du bouton guide pour les contrôleurs ».
steam-guide-off = Steam laisse le bouton central à Focus.
steam-guide-open = Ouvrir les réglages manette de Steam

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = Préparation du PC…
profile-on = Mode console
profile-off = Mode normal
unit-gb = Go
profile-flash = Ton PC est en mode console
profile-shell = Bureau Windows masqué : il revient quand tu quittes Focus
profile-freed = { $gb } Go de mémoire libérés pour tes jeux
profile-power = Puissance maximale activée
profile-slowed = { $count ->
    [one] { $count } programme inutile ralenti
   *[other] { $count } programmes inutiles ralentis
}
profile-closed = { $count ->
    [one] { $count } programme fermé
   *[other] { $count } programmes fermés
}
profile-paused = { $count ->
    [one] { $count } tâche de fond de Windows en pause
   *[other] { $count } tâches de fond de Windows en pause
}
profile-title = Ce que Focus fait pour toi
profile-lead = Tant que Focus est ouvert, ton PC se comporte comme une console : tout ce qui ne sert pas au jeu est mis de côté. Tout revient quand tu quittes Focus.
profile-lead-off = Le bureau est masqué, mais ton PC n'est pas encore allégé pour le jeu. Active les performances dans les paramètres.
profile-none = Ton PC est dans son état normal. Active le mode console dans les paramètres pour qu'il se comporte comme une console.
profile-memory = Mémoire libre pour tes jeux
profile-power-label = Puissance
power-plan-balanced = Équilibrée
power-plan-high = Maximale
power-plan-saver = Économie d'énergie
power-plan-other = Personnalisée
apps-title = { $count ->
    [one] { $count } application est encore ouverte
   *[other] { $count } applications sont encore ouvertes
}
apps-body = Les fermer libère ton PC pour le jeu. Si l'une d'elles te demande d'enregistrer, elle reste ouverte : rien n'est perdu.
apps-closing = Fermeture en cours…
apps-close = Les fermer
apps-keep = Les garder
set-stop-shell = Masquer complètement le bureau Windows

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = Téléchargements
dl-empty = Rien ne se télécharge en ce moment.
dl-steam = Jeux Steam
dl-waiting = En attente
dl-focus = Installations lancées depuis Focus
dl-state-running = Installation en cours…
dl-state-done = Installé
dl-state-failed = L'installation a échoué
dl-open-steam = Voir dans Steam
dl-open-epic = Voir dans Epic Games
dl-other = Les autres boutiques montrent leurs téléchargements dans leur propre fenêtre.
help-title = Aide et mise à jour
help-version = Focus { $version }
upd-check = Rechercher une mise à jour
upd-checking = Recherche…
upd-none = Focus est à jour.
upd-available = Focus { $version } est disponible
upd-install = Installer la mise à jour
upd-downloading = Téléchargement de la mise à jour : { $percent } %
upd-restart = Focus va se fermer, puis se rouvrir à jour.
upd-failed = La mise à jour n'a pas pu se faire. Réessaie plus tard.
rep-title = Signaler un problème
rep-body = Décris ce qui s'est passé. Ton navigateur s'ouvre ensuite sur une fiche déjà remplie : il faut un compte GitHub gratuit pour l'envoyer. Ton nom et ton dossier personnel sont retirés du rapport.
rep-describe = Décrire le problème
rep-send = Ouvrir la fiche
rep-opened = La fiche est ouverte dans ton navigateur.

# Effet de profil et tutoriel du bouton central (à relire)
fx-profile = Profil de performance
guide-title = Le bouton central de ta manette ouvre encore Steam
guide-step-1 = Ouvre les réglages de Steam avec le bouton ci-dessous, puis choisis « Contrôleur ».
guide-step-2 = Désactive « Le bouton Guide place Steam au premier plan ».
guide-step-3 = Désactive « Activer la configuration du bouton guide pour les contrôleurs », puis reviens ici.
guide-check = C'est fait, vérifier
guide-later = Plus tard
guide-show = Voir comment régler ça
