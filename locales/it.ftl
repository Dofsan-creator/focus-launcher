app-name = Focus Launcher
lang-title = Scegli la tua lingua
welcome-title = Benvenuto
welcome-body = Qui tutto funziona con controller, mouse o tastiera. Sul tuo PC non cambia nulla senza il tuo consenso.
welcome-autostart = Apri all'accesso
welcome-sim = Le sessioni partono in simulazione: vedi cosa cambierebbe e nulla viene toccato.
welcome-continue = Continua

filter-all = Tutto
filter-games = Giochi
filter-software = Programmi
filter-social = Social

top-add = Aggiungi
top-stores = Negozi
top-settings = Impostazioni
top-power = Alimentazione

home-resume = Riprendi
home-empty-title = Ancora niente qui
home-empty-body = I giochi installati compaiono da soli. Puoi anche aggiungere un programma.
home-played = { $hours ->
    [one] { $hours } ora di utilizzo
   *[other] { $hours } ore di utilizzo
}
home-running = In corso

detail-play = Gioca
detail-open = Apri
detail-favorite = Aggiungi ai preferiti
detail-unfavorite = Rimuovi dai preferiti
detail-remove = Rimuovi dalla libreria
detail-launches = { $count ->
    [one] { $count } avvio
   *[other] { $count } avvii
}
detail-source = Origine: { $store }

prep-title = Preparazione
prep-simulation = Simulazione: non verrà cambiato nulla.
prep-real = Queste modifiche saranno annullate quando avrai finito.
prep-nothing = Niente da cambiare per questa sessione.
prep-start = Avvia
prep-cancel = Annulla
prep-throttle = Rallenta in background: { $name }
prep-suspend = Metti in pausa: { $name }
prep-close = Chiudi: { $name }
prep-service = Ferma per ora: { $name }
prep-awake = Tieni il PC sveglio
prep-power = Piano energetico prestazioni elevate
prep-ask = Decidi tu: { $name }
prep-no-guardian = Le modifiche ai servizi sono saltate: il servizio di supporto non è installato.

end-title = Sessione terminata
end-time = { $minutes ->
    [one] { $minutes } minuto
   *[other] { $minutes } minuti
}
end-restored = Tutto è stato ripristinato.
end-problem = Alcuni elementi non sono stati ripristinati. I dettagli sono nel registro.
end-back = Torna alla libreria

panel-title = Pannello rapido
panel-return = Torna lì
panel-close-entry = Chiudilo
panel-library = Torna alla libreria
panel-none = Niente in corso

close-title = Chiudere { $title }?
close-warning = I progressi non salvati andranno persi.
close-social = Una chiamata o una diretta potrebbe interrompersi.
close-no = No
close-yes = Sì, chiudi
close-waiting = In chiusura. Potrebbe chiederti di salvare.
close-force = Forza la chiusura
close-force-sure = Forzare la chiusura può far perdere lavoro. Sei sicuro?

set-title = Impostazioni
set-language = Lingua
set-lang-status = Stato della traduzione: { $status }
status-reviewed = revisionata
status-draft = da revisionare
set-autostart = Apri all'accesso
set-real = Ottimizzazione reale
set-real-body = Disattivata: solo simulazione. Attivata: le app in background vengono rallentate durante una sessione e poi ripristinate.
set-real-confirm = Consentire modifiche reali a questo PC durante le sessioni?
set-keep-social = Non toccare le app social
set-awake = Tieni il PC sveglio durante una sessione
set-high-perf = Piano energetico prestazioni elevate
set-big-picture = Blocca l'altra modalità a schermo intero di Steam
set-sounds = Suoni
set-motion = Riduci le animazioni
set-theme = Tema in base all'ora
set-glyphs = Icone dei pulsanti
glyph-auto = Automatico
glyph-playStation = Forme
glyph-xbox = Lettere
set-detect = Rileva controller
set-guardian = Servizio di supporto
set-guardian-body = Serve solo per mettere in pausa i servizi di Windows. Windows chiede il permesso una volta, e quella finestra richiede mouse o tastiera.
set-guardian-on = Installato
set-guardian-off = Non installato
set-guardian-install = Installa
set-analysis = Analisi del sistema
state-on = Attivo
state-off = Disattivo
yes = Sì
no = No

ctl-family = Controller: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = Non rilevato
ctl-mode = Modalità: { $mode }
mode-auto = Auto
mode-native = Nativa
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = Nessuna
ctl-connected = Controller collegato: { $family }

an-title = Analisi del sistema
an-processes = Processi: { $count }
an-services = Servizi: { $count }
an-memory = Memoria: { $gb } GB
an-cpu = Processore: { $name }
an-top = I più esigenti in memoria
an-loading = Lettura del sistema in corso. Nulla viene modificato.

add-title = Aggiungi un programma
add-body = Scegli un file. Viene letto, mai eseguito.
add-up = Cartella superiore
add-suggest = Suggerisci da questa cartella
add-suggested = File più probabili
add-name = Nome
add-kind = Tipo
add-save = Aggiungi alla libreria
add-script = È uno script. Controlla il percorso completo prima di aggiungerlo.
kind-game = Gioco
kind-software = Programma
kind-social = Social
kind-other = Altro

store-title = Negozi e app
store-install = Installa
store-page = Pagina ufficiale
store-confirm = Installare { $name } di { $publisher }?
store-source = Origine: gestore pacchetti di Windows
store-uac = Windows potrebbe chiedere il permesso. Quella finestra richiede mouse o tastiera.
store-progress = Installazione di { $name }
store-done = Installazione completata.
store-failed = Installazione non riuscita. Puoi usare la pagina ufficiale.

power-title = Alimentazione
power-quit = Esci al desktop
power-restart = Riavvia
power-shutdown = Spegni
power-sure = Sei sicuro?

kbd-done = Fatto
kbd-space = Spazio
kbd-delete = Cancella

hint-select = Scegli
hint-back = Indietro
hint-panel = Pannello rapido

err-generic = Qualcosa è andato storto. I dettagli sono nel registro.
autostart-disabled = Il launcher non è riuscito ad avviarsi due volte, quindi l'apertura all'accesso è stata disattivata.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = Cerca
hint-options = Opzioni
home-searching = Risultati per «{ $text }»
detail-remove-confirm = Rimuovere { $title } dalla raccolta?
set-section-console = Modalità console
set-console = Passa Windows alla modalità console mentre Focus è aperto
set-console-body = La barra delle applicazioni si nasconde e le notifiche tacciono. Esci da Focus per ritrovare il solito Windows.
set-hide-taskbar = Nascondi la barra delle applicazioni
set-silence = Silenzia le notifiche
set-start-minimized = Avvia ridotto all’apertura della sessione
set-section-opt = Prestazioni
set-section-sound = Audio e schermo
set-music = Musica di sottofondo
set-vibration = Vibrazione del controller
opt-off = Disattivate
opt-off-body = Simulazione: vedi cosa cambierebbe, non viene toccato nulla.
opt-balanced = Bilanciate
opt-balanced-body = Le app in background vengono rallentate durante una sessione, poi ripristinate.
opt-max = Massime
opt-max-body = Tutte le leve consentite: app rallentate, aggiornamenti in pausa, prestazioni elevate. Poi tutto viene ripristinato.
power-quit-body = Windows torna alla normalità.
power-restart-body = Riavvia il PC.
power-shutdown-body = Spegne il PC.
con-title = Connessioni
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = Cerca di nuovo
con-searching = Ricerca in corso…
con-connected = Connesso
con-connected-to = Connesso a { $name }
con-connect = Connetti
con-disconnected = Disconnesso.
con-password = Password
con-none = Nessuna rete trovata.
con-ethernet = Connessione via cavo
con-offline = Non connesso
con-result-success = Connesso.
con-result-badPassword = Password errata.
con-result-timeout = La rete non ha risposto.
con-result-failed = Connessione non riuscita.
con-result-notFound = Rete non trovata. Ripeti la ricerca.
con-result-noAdapter = Nessun adattatore Wi-Fi trovato.
bt-paired = I tuoi dispositivi
bt-search = Cerca dispositivi
bt-pair = Associa
bt-pair-hint = Metti il dispositivo in modalità di associazione, poi avvia la ricerca.
bt-forget-confirm = Dimenticare { $name }?
bt-forgotten = Dispositivo dimenticato.
bt-result-success = Dispositivo associato.
bt-result-failed = Associazione non riuscita.
setup-step = Passo { $current }
setup-next = Avanti
setup-back = Indietro
setup-start = Inizia
setup-console-title = Il tuo PC diventa una console
setup-console-body = Finché Focus è aperto, Windows si fa da parte: niente barra delle applicazioni, niente notifiche. Esci da Focus e tutto torna normale.
setup-pad-title = Il tuo controller
setup-pad-body = Scegli come mostrare i pulsanti. Focus funziona anche con mouse e tastiera.
setup-net-title = Resta connesso
setup-net-body = Wi-Fi e Bluetooth sono sempre a un pulsante di distanza, anche durante una partita.
setup-lib-title = La tua raccolta
setup-lib-body = { $count ->
    [one] { $count } gioco o app trovato su questo PC.
   *[other] { $count } giochi e app trovati su questo PC.
}
setup-opt-title = Prestazioni durante le sessioni
setup-opt-body = Focus può alleggerire il PC mentre giochi, poi rimettere tutto a posto. Scegli tu, e potrai cambiare in seguito.
setup-comfort-title = Audio e comfort
setup-comfort-body = Regola l’atmosfera. Aprire Focus all’avvio della sessione fa partire il PC come una console.
setup-done-title = Tutto pronto
setup-done-body = La tua raccolta ti aspetta. Premi Inizia per entrare.
setup-open-wifi = Configura il Wi-Fi
setup-open-bt = Associa un controller
store-installed = Installato
filter-stores = Store
launch-starting = Avvio in corso…
store-open = Apri
panel-screenshot = Screenshot
shot-saved = Screenshot salvato in Immagini\Focus.
detail-update = Aggiornamento disponibile
detail-update-open = Aggiorna
power-sleep = Sospendi
power-sleep-body = Il PC va in sospensione, come il riposo di una console.
end-auto = Ritorno alla raccolta…
con-ethernet-on = Connesso via cavo (Ethernet)
ctl-native-note = Fuori da Steam, Focus Input sostituisce il controller PlayStation con un controller Xbox che ogni gioco riconosce. Per evitare comandi doppi serve HidHide (nel negozio di Focus, Driver): Focus nasconde il controller reale al gioco durante la partita e poi lo restituisce. Senza HidHide, «Auto» lascia il controller così com’è.
steam-guide-on = In Steam, il tasto centrale del controller porta ancora Steam sopra i tuoi giochi. In Steam: Impostazioni, Controller, Impostazioni avanzate, disattiva l’opzione del tasto Guida che porta Steam in primo piano e la configurazione delle combinazioni del tasto Guida.
steam-guide-off = Steam lascia il tasto centrale a Focus.
steam-guide-open = Apri le impostazioni controller di Steam

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = Preparazione del PC…
profile-on = Modalità console
profile-off = Modalità normale
unit-gb = GB
profile-flash = Il tuo PC è in modalità console
profile-shell = Desktop di Windows nascosto: torna quando esci da Focus
profile-freed = { $gb } GB di memoria liberati per i tuoi giochi
profile-power = Potenza massima attivata
profile-slowed = { $count ->
    [one] { $count } programma inutile rallentato
   *[other] { $count } programmi inutili rallentati
}
profile-closed = { $count ->
    [one] { $count } programma chiuso
   *[other] { $count } programmi chiusi
}
profile-paused = { $count ->
    [one] { $count } attività in background di Windows in pausa
   *[other] { $count } attività in background di Windows in pausa
}
profile-title = Cosa fa Focus per te
profile-lead = Finché Focus è aperto, il tuo PC si comporta come una console: tutto ciò che non serve al gioco viene messo da parte. Tutto torna quando esci da Focus.
profile-lead-off = Il desktop è nascosto, ma il tuo PC non è ancora alleggerito per giocare. Attiva le prestazioni nelle impostazioni.
profile-none = Il tuo PC è nel suo stato normale. Attiva la modalità console nelle impostazioni perché si comporti come una console.
profile-memory = Memoria libera per i tuoi giochi
profile-power-label = Potenza
power-plan-balanced = Bilanciata
power-plan-high = Massima
power-plan-saver = Risparmio energetico
power-plan-other = Personalizzata
apps-title = { $count ->
    [one] { $count } app è ancora aperta
   *[other] { $count } app sono ancora aperte
}
apps-body = Chiuderle libera il PC per il gioco. Se una ti chiede di salvare, resta aperta: non si perde nulla.
apps-closing = Chiusura in corso…
apps-close = Chiudile
apps-keep = Lasciale aperte
set-stop-shell = Nascondi completamente il desktop di Windows

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = Download
dl-empty = Al momento non c'è nulla in download.
dl-steam = Giochi di Steam
dl-waiting = In attesa
dl-focus = Installazioni avviate da Focus
dl-state-running = Installazione in corso…
dl-state-done = Installato
dl-state-failed = L'installazione non è riuscita
dl-open-steam = Vedi in Steam
dl-open-epic = Vedi in Epic Games
dl-other = Gli altri store mostrano i download nella propria finestra.
help-title = Aiuto e aggiornamento
help-version = Focus { $version }
upd-check = Cerca un aggiornamento
upd-checking = Ricerca in corso…
upd-none = Focus è aggiornato.
upd-available = Focus { $version } è disponibile
upd-install = Installa l'aggiornamento
upd-downloading = Download dell'aggiornamento: { $percent } %
upd-restart = Focus si chiuderà e si riaprirà aggiornato.
upd-failed = L'aggiornamento non è riuscito. Riprova più tardi.
rep-title = Segnala un problema
rep-body = Descrivi cosa è successo. Poi il browser si apre su una scheda già compilata: per inviarla serve un account GitHub gratuito. Il tuo nome e la tua cartella personale vengono rimossi dal rapporto.
rep-describe = Descrivi il problema
rep-send = Apri la scheda
rep-opened = La scheda è aperta nel browser.

# Effet de profil et tutoriel du bouton central (à relire)
fx-profile = Profilo prestazioni
guide-title = Il pulsante centrale del controller apre ancora Steam
guide-step-1 = Apri le impostazioni di Steam con il pulsante qui sotto, poi scegli «Controller».
guide-step-2 = Disattiva «Il pulsante Guida porta Steam in primo piano».
guide-step-3 = Disattiva «Abilita le combinazioni del pulsante Guida per i controller», poi torna qui.
guide-check = Fatto, verifica
guide-later = Più tardi
guide-show = Vedi come risolvere
