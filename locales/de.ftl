app-name = Focus Launcher
lang-title = Wähle deine Sprache
welcome-title = Willkommen
welcome-body = Alles hier funktioniert mit Controller, Maus oder Tastatur. Ohne deine Zustimmung ändert sich nichts an deinem PC.
welcome-autostart = Beim Anmelden öffnen
welcome-sim = Sitzungen starten als Simulation: Du siehst, was sich ändern würde, und nichts wird angefasst.
welcome-continue = Weiter

filter-all = Alle
filter-games = Spiele
filter-software = Programme
filter-social = Sozial

top-add = Hinzufügen
top-stores = Stores
top-settings = Einstellungen
top-power = Ein/Aus

home-resume = Fortsetzen
home-empty-title = Noch nichts hier
home-empty-body = Installierte Spiele erscheinen von selbst. Du kannst auch ein Programm hinzufügen.
home-played = { $hours ->
    [one] { $hours } Stunde genutzt
   *[other] { $hours } Stunden genutzt
}
home-running = Läuft

detail-play = Spielen
detail-open = Öffnen
detail-favorite = Als Favorit markieren
detail-unfavorite = Favorit entfernen
detail-remove = Aus der Bibliothek entfernen
detail-launches = { $count ->
    [one] { $count } Start
   *[other] { $count } Starts
}
detail-source = Quelle: { $store }

prep-title = Vorbereitung
prep-simulation = Simulation: Es wird nichts geändert.
prep-real = Diese Änderungen werden rückgängig gemacht, wenn du fertig bist.
prep-nothing = Für diese Sitzung gibt es nichts zu ändern.
prep-start = Starten
prep-cancel = Abbrechen
prep-throttle = Im Hintergrund drosseln: { $name }
prep-suspend = Pausieren: { $name }
prep-close = Schließen: { $name }
prep-service = Vorübergehend anhalten: { $name }
prep-awake = PC wach halten
prep-power = Energiesparplan Höchstleistung
prep-ask = Deine Entscheidung: { $name }
prep-no-guardian = Dienständerungen werden übersprungen: Der Hilfsdienst ist nicht installiert.

end-title = Sitzung beendet
end-time = { $minutes ->
    [one] { $minutes } Minute
   *[other] { $minutes } Minuten
}
end-restored = Alles wurde wiederhergestellt.
end-problem = Einige Elemente konnten nicht wiederhergestellt werden. Details stehen im Protokoll.
end-back = Zurück zur Bibliothek

panel-title = Schnellleiste
panel-return = Dorthin zurück
panel-close-entry = Schließen
panel-library = Zurück zur Bibliothek
panel-none = Es läuft nichts

close-title = { $title } schließen?
close-warning = Nicht gespeicherter Fortschritt geht verloren.
close-social = Ein Anruf oder Stream kann abbrechen.
close-no = Nein
close-yes = Ja, schließen
close-waiting = Wird geschlossen. Vielleicht wirst du ums Speichern gebeten.
close-force = Beenden erzwingen
close-force-sure = Erzwungenes Beenden kann Arbeit kosten. Bist du sicher?

set-title = Einstellungen
set-language = Sprache
set-lang-status = Stand der Übersetzung: { $status }
status-reviewed = geprüft
status-draft = zu prüfen
set-autostart = Beim Anmelden öffnen
set-real = Echte Optimierung
set-real-body = Solange Focus geöffnet ist, wird alles beiseitegelegt, was ein Spiel nicht braucht: Programme gebremst, Hintergrundaufgaben von Windows pausiert, volle Leistung. Alles kommt zurück, wenn du Focus beendest.
set-real-confirm = Echte Änderungen an diesem PC während der Sitzungen erlauben?
set-keep-social = Soziale Apps nicht anfassen
set-awake = PC während einer Sitzung wach halten
set-high-perf = Energiesparplan Höchstleistung
set-big-picture = Den anderen Vollbildmodus von Steam blockieren
set-sounds = Töne
set-motion = Animationen reduzieren
set-theme = Design nach Tageszeit
set-glyphs = Tastensymbole
glyph-auto = Automatisch
glyph-playStation = Formen
glyph-xbox = Buchstaben
set-detect = Controller erkennen
set-guardian = Hilfsdienst
set-guardian-body = Nur nötig, um Windows-Dienste anzuhalten. Windows fragt einmal nach der Erlaubnis, und dieses Fenster braucht Maus oder Tastatur.
set-guardian-on = Installiert
set-guardian-off = Nicht installiert
set-guardian-install = Installieren
set-analysis = Systemanalyse
state-on = Ein
state-off = Aus
yes = Ja
no = Nein

ctl-family = Controller: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = Nicht erkannt
ctl-mode = Modus: { $mode }
mode-auto = Auto
mode-native = Nativ
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = Keiner
ctl-connected = Controller verbunden: { $family }

an-title = Systemanalyse
an-processes = Prozesse: { $count }
an-services = Dienste: { $count }
an-memory = Arbeitsspeicher: { $gb } GB
an-cpu = Prozessor: { $name }
an-top = Größter Speicherverbrauch
an-loading = Dein System wird gelesen. Es wird nichts geändert.

add-title = Programm hinzufügen
add-body = Wähle eine Datei. Sie wird gelesen, nie ausgeführt.
add-up = Übergeordneter Ordner
add-suggest = Aus diesem Ordner vorschlagen
add-suggested = Wahrscheinlichste Dateien
add-name = Name
add-kind = Typ
add-save = Zur Bibliothek hinzufügen
add-script = Das ist ein Skript. Prüfe den vollständigen Pfad, bevor du es hinzufügst.
kind-game = Spiel
kind-software = Programm
kind-social = Sozial
kind-other = Sonstiges

store-title = Stores und Apps
store-install = Installieren
store-page = Offizielle Seite
store-confirm = { $name } von { $publisher } installieren?
store-source = Quelle: Windows-Paketverwaltung
store-uac = Windows fragt eventuell nach der Erlaubnis. Dieses Fenster braucht Maus oder Tastatur.
store-progress = { $name } wird installiert
store-done = Installation abgeschlossen.
store-failed = Die Installation ist fehlgeschlagen. Du kannst die offizielle Seite nutzen.

power-title = Ein/Aus
power-quit = Zum Desktop
power-restart = Neu starten
power-shutdown = Herunterfahren
power-sure = Bist du sicher?

kbd-done = Fertig
kbd-space = Leertaste
kbd-delete = Löschen

hint-select = Wählen
hint-back = Zurück
hint-panel = Schnellleiste

err-generic = Etwas ist schiefgelaufen. Details stehen im Protokoll.
autostart-disabled = Der Launcher konnte zweimal nicht starten, daher wurde das Öffnen beim Anmelden ausgeschaltet.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = Suchen
hint-options = Optionen
home-searching = Ergebnisse für „{ $text }“
detail-remove-confirm = { $title } aus der Bibliothek entfernen?
set-section-console = Konsolenmodus
set-console = Windows in den Konsolenmodus versetzen, solange Focus geöffnet ist
set-console-body = Desktop, Taskleiste und Benachrichtigungen verschwinden. Beende Focus, um dein gewohntes Windows zurückzubekommen.
set-hide-taskbar = Taskleiste ausblenden
set-silence = Benachrichtigungen stummschalten
set-start-minimized = Bei Sitzungsbeginn minimiert starten
set-section-opt = Leistung
set-section-sound = Ton und Anzeige
set-music = Hintergrundmusik
set-vibration = Controller-Vibration
opt-off = Aus
opt-off-body = Simulation: Du siehst, was sich ändern würde, nichts wird angefasst.
opt-balanced = Ausgewogen
opt-balanced-body = Hintergrund-Apps werden während einer Sitzung gebremst und danach wiederhergestellt.
opt-max = Maximal
opt-max-body = Alle erlaubten Hebel: Apps gebremst, Updates pausiert, Höchstleistung. Danach wird alles wiederhergestellt.
power-quit-body = Windows kehrt in den Normalzustand zurück.
power-restart-body = Startet den PC neu.
power-shutdown-body = Schaltet den PC aus.
con-title = Verbindungen
con-wifi = WLAN
con-bt = Bluetooth
con-scan = Erneut suchen
con-searching = Suche läuft …
con-connected = Verbunden
con-connected-to = Verbunden mit { $name }
con-connect = Verbinden
con-disconnected = Getrennt.
con-password = Passwort
con-none = Kein Netzwerk gefunden.
con-ethernet = Kabelverbindung
con-offline = Nicht verbunden
con-result-success = Verbunden.
con-result-badPassword = Falsches Passwort.
con-result-timeout = Das Netzwerk hat nicht geantwortet.
con-result-failed = Verbindung fehlgeschlagen.
con-result-notFound = Netzwerk nicht gefunden. Suche erneut.
con-result-noAdapter = Kein WLAN-Adapter gefunden.
bt-paired = Deine Geräte
bt-search = Geräte suchen
bt-pair = Koppeln
bt-pair-hint = Versetze das Gerät in den Kopplungsmodus und starte dann die Suche.
bt-forget-confirm = { $name } entfernen?
bt-forgotten = Gerät entfernt.
bt-result-success = Gerät gekoppelt.
bt-result-failed = Kopplung fehlgeschlagen.
setup-step = Schritt { $current }
setup-next = Weiter
setup-back = Zurück
setup-start = Los geht’s
setup-console-title = Dein PC wird zur Konsole
setup-console-body = Solange Focus geöffnet ist, tritt Windows zurück: kein Desktop, keine Taskleiste, keine Benachrichtigungen. Beende Focus und alles ist wieder normal.
setup-pad-title = Dein Controller
setup-pad-body = Wähle, wie Tasten angezeigt werden. Focus funktioniert auch mit Maus und Tastatur.
setup-net-title = Bleib verbunden
setup-net-body = WLAN und Bluetooth sind immer nur eine Taste entfernt, auch während eines Spiels.
setup-lib-title = Deine Bibliothek
setup-lib-body = { $count ->
    [one] { $count } Spiel oder App auf diesem PC gefunden.
   *[other] { $count } Spiele und Apps auf diesem PC gefunden.
}
setup-opt-title = Leistung während der Sitzungen
setup-opt-body = Focus kann deinen PC beim Spielen entlasten und danach alles zurücksetzen. Du entscheidest und kannst es später ändern.
setup-comfort-title = Ton und Komfort
setup-comfort-body = Stell die Atmosphäre ein. Öffnet sich Focus bei Sitzungsbeginn, startet dein PC wie eine Konsole.
setup-done-title = Alles bereit
setup-done-body = Deine Bibliothek wartet. Drücke „Los geht’s“, um einzutreten.
setup-open-wifi = WLAN einrichten
setup-open-bt = Controller koppeln
store-installed = Installiert
filter-stores = Stores
launch-starting = Wird gestartet …
store-open = Öffnen
panel-screenshot = Bildschirmfoto
shot-saved = Bildschirmfoto in Bilder\Focus gespeichert.
detail-update = Update verfügbar
detail-update-open = Aktualisieren
power-sleep = Energie sparen
power-sleep-body = Der PC geht in den Ruhezustand, wie eine Konsole.
end-auto = Zurück zur Bibliothek …
con-ethernet-on = Per Kabel verbunden (Ethernet)
ctl-native-note = Außerhalb von Steam ersetzt Focus Input den PlayStation-Controller durch einen Xbox-Controller, den jedes Spiel versteht. Gegen doppelte Eingaben braucht es HidHide (im Focus-Store, Treiber): Focus verbirgt dann den echten Controller während des Spiels und gibt ihn danach zurück. Ohne HidHide lässt „Auto“ den Controller unverändert.
steam-guide-on = In Steam holt die Mitteltaste des Controllers Steam noch vor deine Spiele. In Steam: Einstellungen, Controller, Erweiterte Einstellungen, die Option der Guide-Taste, die Steam in den Vordergrund holt, und die Guide-Tastenkombinationen ausschalten.
steam-guide-off = Steam überlässt die Mitteltaste Focus.
steam-guide-open = Steam-Controllereinstellungen öffnen

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = PC wird vorbereitet…
profile-on = Konsolenmodus
profile-off = Normalmodus
unit-gb = GB
profile-flash = Dein PC ist im Konsolenmodus
profile-shell = Windows-Desktop ausgeblendet: Er kommt zurück, wenn du Focus beendest
profile-freed = { $gb } GB Arbeitsspeicher für deine Spiele freigegeben
profile-power = Volle Leistung eingeschaltet
profile-slowed = { $count ->
    [one] { $count } unnötiges Programm gebremst
   *[other] { $count } unnötige Programme gebremst
}
profile-closed = { $count ->
    [one] { $count } Programm geschlossen
   *[other] { $count } Programme geschlossen
}
profile-paused = { $count ->
    [one] { $count } Hintergrundaufgabe von Windows pausiert
   *[other] { $count } Hintergrundaufgaben von Windows pausiert
}
profile-title = Was Focus für dich tut
profile-lead = Solange Focus geöffnet ist, verhält sich dein PC wie eine Konsole: Alles, was ein Spiel nicht braucht, wird beiseitegelegt. Alles kommt zurück, wenn du Focus beendest.
profile-lead-off = Der Desktop ist ausgeblendet, aber dein PC ist noch nicht fürs Spielen entlastet. Schalte die Leistung in den Einstellungen ein.
profile-none = Dein PC ist im normalen Zustand. Schalte den Konsolenmodus in den Einstellungen ein, damit er sich wie eine Konsole verhält.
profile-memory = Freier Arbeitsspeicher für deine Spiele
profile-power-label = Leistung
power-plan-balanced = Ausbalanciert
power-plan-high = Maximal
power-plan-saver = Energiesparen
power-plan-other = Benutzerdefiniert
apps-title = { $count ->
    [one] { $count } App ist noch geöffnet
   *[other] { $count } Apps sind noch geöffnet
}
apps-body = Sie zu schließen macht deinen PC frei fürs Spiel. Fragt eine davon nach dem Speichern, bleibt sie offen: Nichts geht verloren.
apps-closing = Wird geschlossen…
apps-close = Schließen
apps-keep = Offen lassen
set-stop-shell = Windows-Desktop vollständig ausblenden

# Téléchargements, aide, mise à jour, signalement (à relire)
dl-title = Downloads
dl-empty = Im Moment wird nichts heruntergeladen.
dl-steam = Steam-Spiele
dl-waiting = Wartet
dl-focus = Aus Focus gestartete Installationen
dl-state-running = Wird installiert…
dl-state-done = Installiert
dl-state-failed = Die Installation ist fehlgeschlagen
dl-open-steam = In Steam ansehen
dl-open-epic = In Epic Games ansehen
dl-other = Andere Shops zeigen ihre Downloads in ihrem eigenen Fenster.
help-title = Hilfe und Update
help-version = Focus { $version }
upd-check = Nach einem Update suchen
upd-checking = Suche läuft…
upd-none = Focus ist aktuell.
upd-available = Focus { $version } ist verfügbar
upd-install = Update installieren
upd-downloading = Update wird heruntergeladen: { $percent } %
upd-restart = Focus schließt sich und öffnet sich dann aktualisiert wieder.
upd-failed = Das Update hat nicht geklappt. Versuch es später noch einmal.
rep-title = Ein Problem melden
rep-body = Beschreibe, was passiert ist. Danach öffnet sich dein Browser mit einem bereits ausgefüllten Formular: Zum Absenden brauchst du ein kostenloses GitHub-Konto. Dein Name und dein persönlicher Ordner werden aus dem Bericht entfernt.
rep-describe = Problem beschreiben
rep-send = Formular öffnen
rep-opened = Das Formular ist in deinem Browser geöffnet.

# Effet de profil et tutoriel du bouton central (à relire)
fx-profile = Leistungsprofil
guide-title = Die mittlere Taste deines Controllers öffnet noch Steam
guide-step-1 = Öffne die Steam-Einstellungen mit der Schaltfläche unten und wähle „Controller“.
guide-step-2 = Schalte „Guide-Taste bringt Steam in den Vordergrund“ aus.
guide-step-3 = Schalte „Guide-Tasten-Kombinationen für Controller aktivieren“ aus und komm dann hierher zurück.
guide-check = Erledigt, prüfen
guide-later = Später
guide-show = So lässt es sich beheben

# Souris à la manette (à relire)
pointer-tile = Controller als Maus
pointer-hint = Linker Stick: bewegen · unten: klicken · rechts: Rechtsklick · oben: Tastatur · mittlere Taste: fertig
