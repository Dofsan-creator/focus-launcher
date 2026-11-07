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
set-real-body = Désactivée : simulation seulement. Activée : les applications en arrière-plan sont ralenties pendant une session, puis restaurées.
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
