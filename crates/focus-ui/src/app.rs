//! Racine de l'application : état partagé et routage des entrées.

use crate::api::{self, invoke, ConsoleReport, Merged, NetStatus, Preview, RunningEntry, SessionEnd, Settings};
use crate::i18n::{best_match, I18n, LANGS};
use crate::nav;
use crate::screens::*;
use crate::sound::{self, Tone};
use focus_core::model::EntryKind;
use focus_core::spatial::Direction;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{json, Value};
use std::time::Duration;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

/// Nombre d'étapes de l'assistant du premier démarrage.
pub const SETUP_STEPS: u8 = 8;

/// Écrans. L'ordre d'arrivée forme la pile de retour.
#[derive(Clone, PartialEq)]
pub enum Screen {
    Setup(u8),
    Home,
    Detail(String),
    Prepare(String),
    Ended(SessionEnd),
    Settings,
    Connections,
    Analysis,
    Profile,
    Downloads,
    Help,
    AddFile,
    Stores,
    Power,
}

/// Saisie au clavier à l'écran en cours.
#[derive(Clone)]
pub struct TextEdit {
    pub value: RwSignal<String>,
    /// Mot de passe : caractères masqués.
    pub secret: bool,
}

/// Dernier périphérique utilisé : il décide des symboles d'aide.
#[derive(Clone, Copy, PartialEq)]
pub enum Device {
    Pad,
    Keyboard,
    Mouse,
}

/// État partagé.
#[derive(Clone, Copy)]
pub struct Ctx {
    pub screens: RwSignal<Vec<Screen>>,
    pub settings: RwSignal<Option<Settings>>,
    pub entries: RwSignal<Vec<Merged>>,
    pub visible: Memo<Vec<Merged>>,
    pub filter: RwSignal<Option<EntryKind>>,
    /// Filtre « Stores » : les boutiques à la place des jeux.
    pub stores_tab: RwSignal<bool>,
    /// Boutiques du catalogue, installées ou non.
    pub stores: RwSignal<Vec<api::CatalogItem>>,
    pub search: RwSignal<String>,
    pub selected: RwSignal<i32>,
    pub panel: RwSignal<bool>,
    pub running: RwSignal<Option<RunningEntry>>,
    pub launching: RwSignal<Option<String>>,
    pub device: RwSignal<Device>,
    pub family: RwSignal<String>,
    pub battery: RwSignal<Option<u8>>,
    pub net: RwSignal<NetStatus>,
    pub clock: RwSignal<String>,
    pub con_tab: RwSignal<&'static str>,
    /// Rubrique ouverte des paramètres, gardée au retour d’un dialogue ou d’une page.
    pub set_section: RwSignal<&'static str>,
    /// Choix d'ouverture automatique de l'assistant (coché par défaut, v4.3).
    pub setup_autostart: RwSignal<bool>,
    pub toast: RwSignal<Option<String>>,
    pub dialog_open: RwSignal<bool>,
    pub editing: RwSignal<Option<TextEdit>, LocalStorage>,
    /// Bilan du mode console : `None` quand Windows est dans son état normal.
    pub profile: RwSignal<Option<ConsoleReport>>,
    /// Le PC est en train de passer en mode console.
    pub profile_busy: RwSignal<bool>,
    /// Annonce du changement de profil, affichée quelques secondes.
    pub flash: RwSignal<bool>,
    /// Question « fermer les applications ouvertes ? » en cours.
    pub apps_ask: RwSignal<bool>,
    /// Effet plein écran du changement de profil : le niveau qui vient de s'activer.
    pub fx: RwSignal<Option<String>>,
    /// Tutoriel « le bouton central ouvre Steam » en cours.
    pub guide_ask: RwSignal<bool>,
    /// Mise à jour de Focus disponible.
    pub update: RwSignal<Option<api::UpdateInfo>>,
    /// Téléchargement de la mise à jour, en pourcentage.
    pub update_progress: RwSignal<Option<u8>>,
}

thread_local! {
    /// Heure d'ouverture de l'interface, pour ne rien montrer sous l'animation de démarrage.
    static STARTED: std::cell::Cell<f64> = const { std::cell::Cell::new(0.0) };
    /// La question sur les applications ouvertes n'est posée qu'une fois par ouverture.
    static APPS_ASKED: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Durée de l'animation de démarrage (voir `.boot` dans `style.css`), en millisecondes.
const BOOT_MS: f64 = 5600.0;

/// Reçoit le bilan du mode console : la pastille change, l'annonce descend, et la
/// question sur les applications ouvertes est posée une fois, sur l'accueil.
pub fn show_profile(ctx: Ctx, report: Option<ConsoleReport>) {
    ctx.profile_busy.set(false);
    let changed = ctx.profile.with_untracked(|known| *known != report);
    let level_before = ctx.profile.with_untracked(|known| known.as_ref().map(|known| known.level.clone()));
    let level_now = report.as_ref().map(|report| report.level.clone());
    ctx.profile.set(report.clone());
    // Toute l'interface prend la couleur du profil actif.
    if let Some(root) = web_sys::window().and_then(|w| w.document()).and_then(|d| d.document_element()) {
        let _ = root.set_attribute("data-level", level_now.as_deref().unwrap_or("none"));
    }
    let Some(report) = report.filter(|_| changed) else {
        return;
    };
    let wait = (BOOT_MS - (js_sys::Date::now() - STARTED.get())).max(0.0);
    set_timeout(
        move || {
            // Le profil vient de changer : effet plein écran et trois notes, puis l'annonce.
            if level_before != level_now {
                ctx.fx.set(level_now.clone());
                if ctx.settings.with_untracked(|s| s.as_ref().is_some_and(|s| s.sounds)) {
                    sound::play(Tone::Profile);
                }
                set_timeout(move || ctx.fx.set(None), Duration::from_millis(2400));
            }
            ctx.flash.set(true);
            set_timeout(move || ctx.flash.set(false), Duration::from_secs(7));
            let quiet = on_home(ctx)
                && !ctx.dialog_open.get_untracked()
                && ctx.launching.with_untracked(Option::is_none)
                && ctx.running.with_untracked(Option::is_none);
            if !report.apps.is_empty() && quiet && !APPS_ASKED.replace(true) {
                ctx.apps_ask.set(true);
                ctx.dialog_open.set(true);
                after_render(nav::focus_first);
            }
        },
        Duration::from_millis(wait as u64),
    );
}

/// L'état partagé, fourni par `App`.
pub fn ctx() -> Ctx {
    use_context::<Ctx>().unwrap_or_else(|| unreachable!("Ctx is provided by App"))
}

/// Entrées du filtre et de la recherche : les plus récentes d'abord.
fn compute_visible(
    entries: &[Merged],
    filter: Option<EntryKind>,
    search: &str,
) -> Vec<Merged> {
    let search = search.to_lowercase();
    let mut list: Vec<Merged> = entries
        .iter()
        .filter(|item| filter.is_none_or(|kind| item.entry.kind == kind))
        .filter(|item| search.is_empty() || item.entry.title.to_lowercase().contains(&search))
        .cloned()
        .collect();
    list.sort_by(|a, b| {
        b.entry
            .stats
            .last_played
            .cmp(&a.entry.stats.last_played)
            .then_with(|| b.entry.favorite.cmp(&a.entry.favorite))
            .then_with(|| a.entry.title.to_lowercase().cmp(&b.entry.title.to_lowercase()))
    });
    list
}

impl Ctx {
    /// Ouvre un écran.
    pub fn push(&self, screen: Screen) {
        self.screens.update(|list| list.push(screen));
        after_render(nav::focus_first);
    }

    /// Remplace l'écran courant.
    pub fn replace(&self, screen: Screen) {
        self.screens.update(|list| {
            list.pop();
            list.push(screen);
        });
        after_render(nav::focus_first);
    }

    /// Revient à la racine.
    pub fn home(&self) {
        self.screens.set(vec![Screen::Home]);
        after_render(nav::focus_first);
    }

    /// Bouton Retour, valable partout.
    pub fn back(&self) {
        // Lu une fois, à part : en Rust 2021, un verrou pris dans `if let` reste tenu
        // dans tous les `else`, et `screens.update` s'y bloquait (WebAssembly s'arrêtait).
        let top = self.screens.with_untracked(|list| list.last().cloned());
        let depth = self.screens.with_untracked(Vec::len);
        let editing = self.editing.with_untracked(Option::is_some);
        if editing {
            self.editing.set(None);
        } else if self.dialog_open.get_untracked() && dialog_shown() {
            self.dialog_open.set(false);
            self.apps_ask.set(false);
            self.guide_ask.set(false);
        } else if self.panel.get_untracked() {
            self.panel.set(false);
            // En jeu, fermer le menu latéral rend la main au jeu, comme sur une console.
            if self.running.with_untracked(Option::is_some) {
                spawn_local(async move {
                    let _ = invoke::<Value>("return_to_running", json!({})).await;
                });
                return;
            }
        } else if let Some(Screen::Setup(step)) = top {
            if step > 0 {
                self.replace(Screen::Setup(step - 1));
            }
            return;
        } else if depth > 1 {
            self.screens.update(|list| {
                list.pop();
            });
        } else if !self.search.get_untracked().is_empty() {
            self.search.set(String::new());
            self.selected.set(0);
        } else {
            return;
        }
        after_render(nav::focus_first);
    }

    /// Enregistre un changement de réglages.
    pub fn update_settings(&self, change: impl FnOnce(&mut Settings)) {
        self.settings.update(|settings| {
            if let Some(settings) = settings {
                change(settings);
            }
        });
        if let Some(settings) = self.settings.get_untracked() {
            apply_appearance(&settings);
            sound::set_music(settings.music);
            spawn_local(async move {
                let _ = invoke::<Value>("set_settings", json!({ "settings": settings })).await;
            });
        }
    }

    /// Recharge la bibliothèque.
    pub fn reload(&self) {
        let entries = self.entries;
        spawn_local(async move {
            if let Ok(list) = invoke("list_entries", json!({})).await {
                entries.set(list);
            }
        });
    }

    /// Lance une entrée en un appui. L'écran de préparation n'apparaît que si une
    /// session réelle demande un choix (un navigateur ouvert, par exemple).
    pub fn play(&self, id: String) {
        let ctx = *self;
        let real = ctx
            .settings
            .with_untracked(|s| s.as_ref().is_some_and(|s| s.optimization != "off"));
        if !real {
            ctx.launch(id);
            return;
        }
        spawn_local(async move {
            let preview: Option<Preview> = invoke("preview_session", json!({ "id": id })).await.ok();
            if preview.is_some_and(|p| !p.simulation && !p.plan.ask_user.is_empty()) {
                ctx.push(Screen::Prepare(id));
            } else {
                ctx.launch(id);
            }
        });
    }

    /// Lance pour de bon, après l'animation d'ouverture.
    pub fn launch(&self, id: String) {
        let ctx = *self;
        ctx.tone(Tone::Confirm);
        ctx.launching.set(Some(id.clone()));
        set_timeout(
            move || {
                spawn_local(async move {
                    match invoke::<Value>("play", json!({ "id": id })).await {
                        Ok(_) => {
                            ctx.home();
                            refresh_running(ctx);
                        }
                        Err(_) => {
                            ctx.launching.set(None);
                            ctx.toast.set(Some("err-generic".to_owned()));
                        }
                    }
                });
            },
            Duration::from_millis(250),
        );
    }

    /// Ouvre la fiche de l'entrée sélectionnée.
    pub fn open_selected(&self) {
        let list = self.visible.get_untracked();
        let index = self.selected.get_untracked().clamp(0, (list.len() as i32 - 1).max(0));
        if let Some(item) = list.get(index as usize) {
            self.push(Screen::Detail(item.entry.id.clone()));
        }
    }

    fn sounds(&self) -> bool {
        self.settings
            .read_untracked()
            .as_ref()
            .is_some_and(|s| s.sounds)
    }

    /// Joue un son si l'utilisateur le veut.
    pub fn tone(&self, tone: Tone) {
        if self.sounds() {
            sound::play(tone);
        }
    }

    /// Ouvre la recherche par le clavier à l'écran.
    pub fn open_search(&self) {
        self.editing.set(Some(TextEdit {
            value: self.search,
            secret: false,
        }));
        self.selected.set(0);
    }
}

/// Un dialogue est-il vraiment affiché ? Un drapeau resté levé ne doit jamais
/// bloquer la touche Retour.
fn dialog_shown() -> bool {
    web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.query_selector(".dialog").ok().flatten())
        .is_some()
}

/// Attente non bloquante, en millisecondes.
pub async fn wait_ms(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// Lance `work` une fois le DOM à jour.
pub fn after_render(work: impl FnOnce() + 'static) {
    // Leptos met le DOM à jour avant l'image suivante : une seule attente suffit
    // (deux ajoutaient environ 16 ms à chaque déplacement).
    request_animation_frame(work);
}

fn refresh_running(ctx: Ctx) {
    spawn_local(async move {
        if let Ok(running) = invoke::<Option<RunningEntry>>("running_entry", json!({})).await {
            ctx.running.set(running);
        }
    });
}

/// État du réseau et de la manette, pour la barre du haut.
pub fn refresh_status(ctx: Ctx) {
    spawn_local(async move {
        if let Ok(Some(net)) = invoke::<Option<NetStatus>>("net_status", json!({})).await {
            ctx.net.set(net);
        }
        if let Ok(Some(status)) =
            invoke::<Option<api::ControllerStatus>>("controller_status", json!({})).await
        {
            ctx.battery.set(status.battery);
            // La manette branchée au démarrage donne tout de suite ses symboles.
            if status.family != "other" {
                ctx.family.set(status.family);
            }
        }
    });
}

/// Boutiques du catalogue, avec leur état d’installation.
pub fn refresh_stores(ctx: Ctx) {
    spawn_local(async move {
        let items: Vec<api::CatalogItem> = invoke("catalog", json!({})).await.unwrap_or_default();
        let mut stores: Vec<_> = items.into_iter().filter(|item| item.category == "store").collect();
        stores.sort_by_key(|item| !item.installed);
        ctx.stores.set(stores);
    });
}

fn now_hhmm() -> String {
    let now = js_sys::Date::new_0();
    format!("{:02}:{:02}", now.get_hours(), now.get_minutes())
}

fn apply_appearance(settings: &Settings) {
    let Some(root) = web_sys::window()
        .and_then(|w| w.document())
        .and_then(|d| d.document_element())
    else {
        return;
    };
    let _ = root.set_attribute(
        "data-motion",
        if settings.reduce_motion {
            "reduced"
        } else {
            "full"
        },
    );
    let hour = js_sys::Date::new_0().get_hours();
    let light = settings.auto_theme && (7..19).contains(&hour);
    let _ = root.set_attribute("data-theme", if light { "light" } else { "dark" });
}

fn on_home(ctx: Ctx) -> bool {
    ctx.screens.read_untracked().last() == Some(&Screen::Home) && !ctx.panel.get_untracked()
}

thread_local! {
    /// Dernier geste de navigation : son nom, sa source (manette ou clavier), son heure.
    static LAST_NAV: std::cell::RefCell<(String, u8, f64)> = const { std::cell::RefCell::new((String::new(), 0, 0.0)) };
}

/// Quand Steam tourne, il convertit la croix et les boutons de la manette en touches
/// du clavier : le même geste arrive alors deux fois, par la manette et par le
/// clavier, et le sélecteur saute une case. Le second, venu de l'autre source dans
/// l'instant, est ignoré.
fn duplicate(action: &str, source: u8) -> bool {
    let now = js_sys::Date::now();
    LAST_NAV.with(|cell| {
        let mut last = cell.borrow_mut();
        if last.0 == action && last.1 != source && now - last.2 < 160.0 {
            return true;
        }
        *last = (action.to_owned(), source, now);
        false
    })
}

const FROM_PAD: u8 = 1;
const FROM_KEYBOARD: u8 = 2;

fn route_action(ctx: Ctx, action: &str) {
    if matches!(action, "up" | "down" | "left" | "right" | "confirm" | "back") && duplicate(action, FROM_PAD) {
        return;
    }
    ctx.device.set(Device::Pad);
    match action {
        "up" => step(ctx, Direction::Up),
        "down" => step(ctx, Direction::Down),
        "left" => horizontal(ctx, Direction::Left),
        "right" => horizontal(ctx, Direction::Right),
        "confirm" => {
            ctx.tone(Tone::Confirm);
            nav::confirm();
        }
        "back" => {
            ctx.tone(Tone::Back);
            ctx.back();
        }
        "options" if on_home(ctx) && nav::active_card().is_some() => ctx.open_selected(),
        "secondary" if on_home(ctx) => ctx.open_search(),
        "pagePrev" => cycle_filter(ctx, -1),
        "pageNext" => cycle_filter(ctx, 1),
        "menu" | "quickPanel" | "quickSettings" => toggle_panel(ctx),
        "home" => press_home(ctx),
        _ => {}
    }
}

/// Bouton central (ou touche Windows) : comme sur une console.
fn press_home(ctx: Ctx) {
    if matches!(ctx.screens.read_untracked().last(), Some(Screen::Setup(_))) {
        return;
    }
    ctx.tone(Tone::Confirm);
    // Rien ne doit rester par-dessus le menu : l’écran de chargement s’efface.
    ctx.launching.set(None);
    if ctx.panel.get_untracked() {
        ctx.panel.set(false);
        after_render(nav::focus_first);
    } else if ctx.running.get_untracked().is_some() || on_home(ctx) {
        toggle_panel(ctx);
    } else {
        ctx.dialog_open.set(false);
        ctx.editing.set(None);
        ctx.home();
    }
}

fn toggle_panel(ctx: Ctx) {
    if matches!(ctx.screens.read_untracked().last(), Some(Screen::Setup(_))) {
        return;
    }
    refresh_running(ctx);
    refresh_status(ctx);
    ctx.panel.update(|open| *open = !*open);
    after_render(nav::focus_first);
}

fn step(ctx: Ctx, direction: Direction) {
    if nav::move_focus(direction) {
        ctx.tone(Tone::Move);
    }
}

/// Sur la rangée, gauche et droite suivent l'ordre des jeux, y compris en droite-gauche.
fn horizontal(ctx: Ctx, direction: Direction) {
    if nav::active_card().is_some() {
        let rtl = use_context::<I18n>()
            .and_then(|i18n| i18n.current())
            .is_some_and(|lang| lang.rtl);
        let forward = (direction == Direction::Right) != rtl;
        let count = if ctx.stores_tab.get_untracked() {
            ctx.stores.with_untracked(Vec::len)
        } else {
            ctx.visible.with_untracked(Vec::len)
        } as i32;
        let next = ctx.selected.get_untracked() + if forward { 1 } else { -1 };
        if (0..count).contains(&next) {
            ctx.selected.set(next);
            ctx.tone(Tone::Move);
            after_render(nav::focus_first);
        }
    } else {
        step(ctx, direction);
    }
}

fn cycle_filter(ctx: Ctx, delta: i32) {
    if !on_home(ctx) {
        return;
    }
    // Tout, Jeux, Logiciels, Stores, Social : `None` dans la liste veut dire Stores.
    let order = [
        Some(None),
        Some(Some(EntryKind::Game)),
        Some(Some(EntryKind::Software)),
        None,
        Some(Some(EntryKind::Social)),
    ];
    let current = if ctx.stores_tab.get_untracked() { None } else { Some(ctx.filter.get_untracked()) };
    let index = order.iter().position(|item| *item == current).unwrap_or(0) as i32;
    let next = (index + delta).rem_euclid(order.len() as i32) as usize;
    ctx.stores_tab.set(order[next].is_none());
    if let Some(filter) = order[next] {
        ctx.filter.set(filter);
    }
    ctx.selected.set(0);
    ctx.tone(Tone::Move);
    after_render(nav::focus_first);
}

fn on_key(ctx: Ctx, event: &KeyboardEvent) {
    // Le même geste vient d'arriver par la manette : cette touche est sa copie.
    let twin = match event.key().as_str() {
        "ArrowUp" => Some("up"),
        "ArrowDown" => Some("down"),
        "ArrowLeft" => Some("left"),
        "ArrowRight" => Some("right"),
        "Enter" => Some("confirm"),
        "Escape" => Some("back"),
        _ => None,
    };
    if twin.is_some_and(|action| duplicate(action, FROM_KEYBOARD)) {
        event.prevent_default();
        return;
    }
    ctx.device.set(Device::Keyboard);
    if ctx.editing.read_untracked().is_some() {
        // Le clavier physique écrit directement dans la saisie en cours.
        if let Some(edit) = ctx.editing.get_untracked() {
            let key = event.key();
            match key.as_str() {
                "Escape" | "Enter" => ctx.back(),
                "Backspace" => edit.value.update(|text| {
                    text.pop();
                }),
                _ if key.chars().count() == 1 => edit.value.update(|text| text.push_str(&key)),
                _ => return,
            }
            event.prevent_default();
        }
        return;
    }
    let handled = match event.key().as_str() {
        "ArrowUp" => {
            step(ctx, Direction::Up);
            true
        }
        "ArrowDown" => {
            step(ctx, Direction::Down);
            true
        }
        "ArrowLeft" => {
            horizontal(ctx, Direction::Left);
            true
        }
        "ArrowRight" => {
            horizontal(ctx, Direction::Right);
            true
        }
        "Escape" => {
            ctx.back();
            true
        }
        "Tab" if on_home(ctx) => {
            cycle_filter(ctx, if event.shift_key() { -1 } else { 1 });
            true
        }
        "F1" => {
            toggle_panel(ctx);
            true
        }
        "F2" | "ContextMenu" if on_home(ctx) => {
            ctx.open_selected();
            true
        }
        "F3" if on_home(ctx) => {
            ctx.open_search();
            true
        }
        _ => false,
    };
    if handled {
        event.prevent_default();
    }
}

fn listen_window<E>(name: &str, handler: impl FnMut(E) + 'static)
where
    E: wasm_bindgen::convert::FromWasmAbi + 'static,
{
    let closure = Closure::<dyn FnMut(E)>::new(handler);
    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback(name, closure.as_ref().unchecked_ref());
    }
    // L'écouteur vit aussi longtemps que l'application.
    closure.forget();
}

#[component]
pub fn App() -> impl IntoView {
    let system = web_sys::window()
        .and_then(|w| w.navigator().language())
        .unwrap_or_default();
    let i18n = I18n::new(best_match(&system));
    provide_context(i18n);
    let entries = RwSignal::new(Vec::<Merged>::new());
    let filter = RwSignal::new(None);
    let search = RwSignal::new(String::new());
    let ctx = Ctx {
        screens: RwSignal::new(Vec::new()),
        settings: RwSignal::new(None),
        entries,
        visible: Memo::new(move |_| entries.with(|list| compute_visible(list, filter.get(), &search.get()))),
        filter,
        stores_tab: RwSignal::new(false),
        stores: RwSignal::new(Vec::new()),
        search,
        selected: RwSignal::new(0),
        panel: RwSignal::new(false),
        running: RwSignal::new(None),
        launching: RwSignal::new(None),
        device: RwSignal::new(Device::Mouse),
        family: RwSignal::new("other".to_owned()),
        battery: RwSignal::new(None),
        net: RwSignal::new(NetStatus::default()),
        clock: RwSignal::new(now_hhmm()),
        con_tab: RwSignal::new("wifi"),
        set_section: RwSignal::new("console"),
        setup_autostart: RwSignal::new(true),
        toast: RwSignal::new(None),
        dialog_open: RwSignal::new(false),
        editing: RwSignal::new_local(None),
        profile: RwSignal::new(None),
        profile_busy: RwSignal::new(false),
        flash: RwSignal::new(false),
        apps_ask: RwSignal::new(false),
        fx: RwSignal::new(None),
        guide_ask: RwSignal::new(false),
        update: RwSignal::new(None),
        update_progress: RwSignal::new(None),
    };
    provide_context(ctx);
    STARTED.set(js_sys::Date::now());

    Effect::new(move |_| {
        let lang = i18n.current();
        if let Some(root) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let _ = root.set_attribute("lang", lang.map_or("en", |l| l.code));
            let _ = root.set_attribute(
                "dir",
                if lang.is_some_and(|l| l.rtl) { "rtl" } else { "ltr" },
            );
        }
    });
    Effect::new(move |_| {
        if let Some(body) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.body())
        {
            let _ = body
                .class_list()
                .toggle_with_force("pad", ctx.device.get() == Device::Pad);
        }
    });
    Effect::new(move |previous: Option<()>| {
        if ctx.toast.get().is_some() {
            set_timeout(move || ctx.toast.set(None), Duration::from_secs(4));
        }
        previous.unwrap_or(())
    });
    // La sélection reste dans la liste quand elle change de taille.
    Effect::new(move |_| {
        let count = ctx.visible.with(Vec::len) as i32;
        if ctx.selected.get_untracked() >= count {
            ctx.selected.set((count - 1).max(0));
        }
    });

    // Une seule horloge et un seul rafraîchissement d'état pour toute l'application.
    let _ = set_interval_with_handle(move || ctx.clock.set(now_hhmm()), Duration::from_secs(10));
    let _ = set_interval_with_handle(move || refresh_status(ctx), Duration::from_secs(20));
    refresh_status(ctx);

    spawn_local(async move {
        let settings: Option<Settings> = invoke("get_settings", json!({})).await.ok();
        if let Some(settings) = settings {
            apply_appearance(&settings);
            sound::set_music(settings.music);
            if let Some(code) = settings.language.as_deref() {
                if let Some(lang) = LANGS.iter().find(|lang| lang.code == code) {
                    i18n.lang.set(lang.code);
                }
            }
            let start = if settings.first_run_done {
                Screen::Home
            } else if settings.language.is_none() {
                Screen::Setup(0)
            } else {
                Screen::Setup(1)
            };
            ctx.settings.set(Some(settings));
            ctx.screens.set(vec![start]);
        }
        ctx.reload();
        refresh_stores(ctx);
        after_render(nav::focus_first);
        // Cet appel prouve au cœur que l'interface tourne : sans lui, la version
        // publiée se ferme au bout d'une minute (focus-app, `boot_guard`).
        let boot: Value = invoke("boot_info", json!({})).await.unwrap_or_default();
        if boot["autostartDisabled"] == json!(true) {
            ctx.toast.set(Some("autostart-disabled".to_owned()));
        }
        // Le passage en mode console a pu finir avant que l'interface n'écoute.
        if let Ok(Some(report)) = invoke::<Option<ConsoleReport>>("console_report", json!({})).await {
            show_profile(ctx, Some(report));
        }
    });

    // Une mise à jour de Focus ? Cherchée une fois, sans rien installer : la pastille
    // des téléchargements le signale.
    spawn_local(async move {
        wait_ms(BOOT_MS as i32 + 4000).await;
        if let Ok(Some(update)) = invoke::<Option<api::UpdateInfo>>("update_check", json!({})).await {
            ctx.toast.set(Some(i18n.ts("upd-available", "version", &update.version)));
            ctx.update.set(Some(update));
        }
    });
    // Steam capte encore le bouton central ? Le tutoriel s'ouvre une fois, sur l'accueil.
    spawn_local(async move {
        wait_ms(BOOT_MS as i32 + 9000).await;
        let grabbed = invoke::<Option<bool>>("steam_guide_status", json!({})).await.ok().flatten() == Some(true);
        if grabbed && on_home(ctx) && !ctx.dialog_open.get_untracked() && ctx.running.with_untracked(Option::is_none) {
            ctx.guide_ask.set(true);
            ctx.dialog_open.set(true);
            after_render(nav::focus_first);
        }
    });
    api::listen("update-progress", move |value| {
        let read = |key: &str| js_sys::Reflect::get(&value, &key.into()).ok().and_then(|v| v.as_f64()).unwrap_or(0.0);
        let (done, total) = (read("done"), read("total"));
        ctx.update_progress.set(Some(if total > 0.0 { (done * 100.0 / total).min(100.0) as u8 } else { 0 }));
    });
    api::listen("console-progress", move |_| ctx.profile_busy.set(true));
    api::listen("console-report", move |value| {
        show_profile(ctx, serde_wasm_bindgen::from_value::<Option<ConsoleReport>>(value).ok().flatten());
    });

    api::listen("pad-action", move |value| {
        if let Some(action) = value.as_string() {
            route_action(ctx, &action);
        }
    });
    api::listen("pad-family", move |value| {
        let family = value.as_string().unwrap_or_else(|| "other".to_owned());
        if family != "other" && family != ctx.family.get_untracked() {
            let name = i18n.t(&format!("family-{family}"));
            ctx.toast.set(Some(i18n.ts("ctl-connected", "family", &name)));
        }
        ctx.family.set(family);
    });
    api::listen("library-changed", move |_| ctx.reload());
    api::listen("session-started", move |_| {
        ctx.launching.set(None);
        ctx.panel.set(false);
        refresh_running(ctx);
    });
    api::listen("session-ended", move |value| {
        if let Ok(end) = serde_wasm_bindgen::from_value::<SessionEnd>(value) {
            ctx.launching.set(None);
            ctx.running.set(None);
            ctx.panel.set(false);
            ctx.reload();
            ctx.screens.set(vec![Screen::Home, Screen::Ended(end)]);
            after_render(nav::focus_first);
        }
    });
    api::listen("install-finished", move |value| {
        let ok = js_sys::Reflect::get(&value, &"ok".into())
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        ctx.toast.set(Some(if ok { "store-done" } else { "store-failed" }.to_owned()));
        ctx.reload();
    });

    listen_window("keydown", move |event: KeyboardEvent| on_key(ctx, &event));
    listen_window("mousemove", move |_: web_sys::MouseEvent| ctx.device.set(Device::Mouse));
    // Fenêtre cachée pendant un jeu : l'audio s'arrête, l'animation de lancement aussi.
    if let Some(document) = web_sys::window().and_then(|w| w.document()) {
        let watched = document.clone();
        let closure = Closure::<dyn FnMut()>::new(move || {
            let hidden = watched.hidden();
            sound::set_suspended(hidden);
            // L'écran de lancement n'est effacé que par « session-started » ou le
            // bouton central : une fenêtre de lanceur qui recouvre Focus rend aussi la
            // page cachée, puis visible.
            if !hidden {
                refresh_status(ctx);
            }
        });
        let _ = document.add_event_listener_with_callback(
            "visibilitychange",
            closure.as_ref().unchecked_ref(),
        );
        closure.forget();
    }

    view! {
        <Backdrop/>
        <div class="boot"><span class="boot-word">"FOCUS"</span><span class="boot-line"></span></div>
        <CurrentScreen/>
        <Show when=move || ctx.panel.get()><QuickPanel/></Show>
        <TextOverlay/>
        <LaunchOverlay/>
        <AppsDialog/>
        <GuideDialog/>
        <div class="profile-line"></div>
        <ProfileFlash/>
        <ProfileFx/>
        <Toast/>
        <Hints/>
    }
}
