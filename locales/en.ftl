# Focus Launcher, langue source. Une langue = un fichier, aucun code à changer.

app-name = Focus Launcher
lang-title = Choose your language
welcome-title = Welcome
welcome-body = Everything here works with a controller, a mouse or a keyboard. Nothing on your PC changes until you allow it.
welcome-autostart = Open at session start
welcome-sim = Sessions start in simulation: you see what would change, and nothing is touched.
welcome-continue = Continue

filter-all = All
filter-games = Games
filter-software = Software
filter-social = Social

top-add = Add
top-stores = Stores
top-settings = Settings
top-power = Power

home-resume = Resume
home-empty-title = Nothing here yet
home-empty-body = Installed games appear on their own. You can also add a program.
home-played = { $hours ->
    [one] { $hours } hour used
   *[other] { $hours } hours used
}
home-running = Running

detail-play = Play
detail-open = Open
detail-favorite = Favorite
detail-unfavorite = Remove favorite
detail-remove = Remove from library
detail-launches = { $count ->
    [one] { $count } launch
   *[other] { $count } launches
}
detail-source = Source: { $store }

prep-title = Getting ready
prep-simulation = Simulation: nothing will be changed.
prep-real = These changes will be undone when you finish.
prep-nothing = Nothing to change for this session.
prep-start = Start
prep-cancel = Cancel
prep-throttle = Slow down in the background: { $name }
prep-suspend = Pause: { $name }
prep-close = Close: { $name }
prep-service = Stop for now: { $name }
prep-awake = Keep the PC awake
prep-power = High performance power plan
prep-ask = Your choice: { $name }
prep-no-guardian = Service changes are skipped: the helper service is not installed.

end-title = Session finished
end-time = { $minutes ->
    [one] { $minutes } minute
   *[other] { $minutes } minutes
}
end-restored = Everything was restored.
end-problem = Some items could not be restored. Details are in the log.
end-back = Back to library

panel-title = Quick panel
panel-return = Return to it
panel-close-entry = Close it
panel-library = Back to library
panel-none = Nothing is running

close-title = Close { $title }?
close-warning = Progress that is not saved will be lost.
close-social = A call or a stream may stop.
close-no = No
close-yes = Yes, close
close-waiting = Waiting for it to close. It may be asking you to save.
close-force = Force close
close-force-sure = Force closing can lose work. Are you sure?

set-title = Settings
set-language = Language
set-lang-status = Translation status: { $status }
status-reviewed = reviewed
status-draft = to be reviewed
set-autostart = Open at session start
set-real = Real optimization
set-real-body = Off: simulation only. On: background apps are slowed down during a session, then restored.
set-real-confirm = Allow real changes to this PC during sessions?
set-keep-social = Leave social apps alone
set-awake = Keep the PC awake during a session
set-high-perf = High performance power plan
set-big-picture = Block the other full-screen mode of Steam
set-sounds = Sounds
set-motion = Reduce animations
set-theme = Theme follows the time of day
set-glyphs = Button icons
glyph-auto = Automatic
glyph-playStation = Shapes
glyph-xbox = Letters
set-detect = Detect controller
set-guardian = Helper service
set-guardian-body = Needed only to pause Windows services. Windows asks for permission once, and that window needs a mouse or a keyboard.
set-guardian-on = Installed
set-guardian-off = Not installed
set-guardian-install = Install
set-analysis = System analysis
state-on = On
state-off = Off
yes = Yes
no = No

ctl-family = Controller: { $family }
family-playStation = PlayStation
family-xbox = Xbox
family-other = Not detected
ctl-mode = Mode: { $mode }
mode-auto = Auto
mode-native = Native
mode-steamInput = Steam Input
mode-focusInput = Focus Input
mode-none = None
ctl-connected = Controller connected: { $family }

an-title = System analysis
an-processes = Processes: { $count }
an-services = Services: { $count }
an-memory = Memory: { $gb } GB
an-cpu = Processor: { $name }
an-top = Using the most memory
an-loading = Reading your system. Nothing is changed.

add-title = Add a program
add-body = Pick a file. It is read, never run.
add-up = Up one folder
add-suggest = Suggest from this folder
add-suggested = Most likely files
add-name = Name
add-kind = Type
add-save = Add to library
add-script = This is a script. Check its full path before adding it.
kind-game = Game
kind-software = Software
kind-social = Social
kind-other = Other

store-title = Stores and apps
store-install = Install
store-page = Official page
store-confirm = Install { $name } from { $publisher }?
store-source = Source: Windows package manager
store-uac = Windows may ask for permission. That window needs a mouse or a keyboard.
store-progress = Installing { $name }
store-done = Installation finished.
store-failed = Installation failed. You can try the official page.

power-title = Power
power-quit = Exit to desktop
power-restart = Restart
power-shutdown = Shut down
power-sure = Are you sure?

kbd-done = Done
kbd-space = Space
kbd-delete = Delete

hint-select = Select
hint-back = Back
hint-panel = Quick panel

err-generic = Something went wrong. Details are in the log.
autostart-disabled = The launcher failed to start twice, so opening at session start was turned off.

# Lumen : accueil, mode console, connexions, premier démarrage (à relire)
top-search = Search
hint-options = Options
home-searching = Results for “{ $text }”
detail-remove-confirm = Remove { $title } from the library?
set-section-console = Console mode
set-console = Switch Windows to console mode while Focus is open
set-console-body = The taskbar hides and notifications go quiet. Exit Focus to get your usual Windows back.
set-hide-taskbar = Hide the taskbar
set-silence = Silence notifications
set-start-minimized = Start minimized at session start
set-section-opt = Performance
set-section-sound = Sound and display
set-music = Background music
set-vibration = Controller vibration
opt-off = Off
opt-off-body = Simulation: you see what would change, nothing is touched.
opt-balanced = Balanced
opt-balanced-body = Background apps are slowed down during a session, then restored.
opt-max = Maximum
opt-max-body = Every allowed lever: apps slowed, updates paused, high performance. Everything is restored afterwards.
power-quit-body = Windows returns to normal.
power-restart-body = Restarts the PC.
power-shutdown-body = Turns the PC off.
con-title = Connections
con-wifi = Wi-Fi
con-bt = Bluetooth
con-scan = Search again
con-searching = Searching…
con-connected = Connected
con-connected-to = Connected to { $name }
con-connect = Connect
con-disconnected = Disconnected.
con-password = Password
con-none = No network found.
con-ethernet = Wired connection
con-offline = Not connected
con-result-success = Connected.
con-result-badPassword = Wrong password.
con-result-timeout = The network did not answer.
con-result-failed = Connection failed.
con-result-notFound = Network not found. Search again.
con-result-noAdapter = No Wi-Fi adapter found.
bt-paired = Your devices
bt-search = Find devices
bt-pair = Pair
bt-pair-hint = Put the device in pairing mode, then search.
bt-forget-confirm = Forget { $name }?
bt-forgotten = Device forgotten.
bt-result-success = Device paired.
bt-result-failed = Pairing failed.
setup-step = Step { $current }
setup-next = Next
setup-back = Back
setup-start = Start
setup-console-title = Your PC becomes a console
setup-console-body = While Focus is open, Windows steps aside: no taskbar, no notifications. Exit Focus and everything is back to normal.
setup-pad-title = Your controller
setup-pad-body = Choose how buttons are shown. Focus also works with a mouse and a keyboard.
setup-net-title = Stay connected
setup-net-body = Wi-Fi and Bluetooth are always one button away, even during a game.
setup-lib-title = Your library
setup-lib-body = { $count ->
    [one] { $count } game or app found on this PC.
   *[other] { $count } games and apps found on this PC.
}
setup-opt-title = Performance during sessions
setup-opt-body = Focus can free up your PC while you play, then put everything back. You choose, and you can change it later.
setup-comfort-title = Sound and comfort
setup-comfort-body = Set the mood. Opening Focus at session start makes your PC start like a console.
setup-done-title = All set
setup-done-body = Your library is ready. Press Start to enter.
setup-open-wifi = Set up Wi-Fi
setup-open-bt = Pair a controller
store-installed = Installed
filter-stores = Stores
launch-starting = Starting…
store-open = Open
panel-screenshot = Screenshot
shot-saved = Screenshot saved in Pictures\Focus.
detail-update = Update available
detail-update-open = Update
power-sleep = Sleep
power-sleep-body = Puts the PC to sleep, like a console rest mode.
end-auto = Back to the library…
con-ethernet-on = Connected by cable (Ethernet)
ctl-native-note = Outside Steam, Focus Input replaces the PlayStation controller with an Xbox controller that every game understands. To avoid doubled inputs it needs HidHide (in the Focus store, Drivers): Focus then hides the real controller from the game while you play and gives it back afterwards. Without HidHide, “Auto” leaves the controller as it is.
steam-guide-on = In Steam, the controller’s center button still brings Steam over your games. In Steam: Settings, Controller, Advanced settings, turn off “Guide button focuses Steam” and “Enable guide button chord configuration for controllers”.
steam-guide-off = Steam leaves the center button to Focus.
steam-guide-open = Open Steam controller settings

# Mode console v2 : profil visible, bureau masqué, applications ouvertes (à relire)
profile-busy = Getting your PC ready…
profile-on = Console mode
profile-off = Normal mode
unit-gb = GB
profile-flash = Your PC is in console mode
profile-shell = Windows desktop hidden: it comes back when you exit Focus
profile-freed = { $gb } GB of memory freed for your games
profile-power = Full power switched on
profile-slowed = { $count ->
    [one] { $count } unneeded program slowed down
   *[other] { $count } unneeded programs slowed down
}
profile-closed = { $count ->
    [one] { $count } program closed
   *[other] { $count } programs closed
}
profile-paused = { $count ->
    [one] { $count } Windows background task paused
   *[other] { $count } Windows background tasks paused
}
profile-title = What Focus does for you
profile-lead = While Focus is open, your PC behaves like a console: everything a game does not need is set aside. It all comes back when you exit Focus.
profile-lead-off = The desktop is hidden, but your PC is not tuned for games yet. Turn on performance in the settings.
profile-none = Your PC is in its normal state. Turn on console mode in the settings to make it behave like a console.
profile-memory = Memory free for your games
profile-power-label = Power
power-plan-balanced = Balanced
power-plan-high = Maximum
power-plan-saver = Power saver
power-plan-other = Custom
apps-title = { $count ->
    [one] { $count } app is still open
   *[other] { $count } apps are still open
}
apps-body = Closing them frees your PC for the game. If one of them asks you to save, it stays open: nothing is lost.
apps-closing = Closing…
apps-close = Close them
apps-keep = Keep them
set-stop-shell = Hide the Windows desktop completely
