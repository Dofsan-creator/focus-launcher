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
set-real-body = Aus: nur Simulation. Ein: Hintergrund-Apps werden während einer Sitzung gedrosselt und danach wiederhergestellt.
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
