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
