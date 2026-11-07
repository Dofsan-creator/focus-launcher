//! Racine de l'application : état partagé et routage des entrées.

use crate::api::{self, invoke, RunningEntry, SessionEnd, Settings};
use crate::i18n::{best_match, I18n, LANGS};
use crate::nav;
use crate::screens::*;
use crate::sound::{self, Tone};
use focus_core::model::EntryKind;
use focus_core::spatial::Direction;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{json, Value};
use std::cell::Cell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::KeyboardEvent;

/// Écrans. L'ordre d'arrivée forme la pile de retour.
#[derive(Clone, PartialEq)]
pub enum Screen {
    Language,
    Welcome,
    Home,
    Detail(String),
    Prepare(String),
    Ended(SessionEnd),
    Settings,
    Analysis,
    AddFile,
    Stores,
    Power,
}

/// Saisie au clavier à l'écran en cours.
#[derive(Clone)]
pub struct TextEdit {
    pub value: RwSignal<String>,
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
    pub entries: RwSignal<Vec<api::Merged>>,
    pub filter: RwSignal<Option<EntryKind>>,
    pub selected: RwSignal<i32>,
    pub panel: RwSignal<bool>,
    pub running: RwSignal<Option<RunningEntry>>,
    pub device: RwSignal<Device>,
    pub family: RwSignal<String>,
    pub toast: RwSignal<Option<String>>,
    pub wipe: RwSignal<bool>,
    pub dialog_open: RwSignal<bool>,
    pub editing: RwSignal<Option<TextEdit>, LocalStorage>,
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
        if self.editing.read_untracked().is_some() {
            self.editing.set(None);
        } else if self.dialog_open.get_untracked() {
            self.dialog_open.set(false);
        } else if self.panel.get_untracked() {
            self.panel.set(false);
        } else if self.screens.read_untracked().len() > 1 {
            self.screens.update(|list| {
                list.pop();
            });
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

    /// Lance une entrée.
    pub fn launch(&self, id: String) {
        let ctx = *self;
        ctx.wipe.set(true);
        set_timeout(
            move || ctx.wipe.set(false),
            std::time::Duration::from_millis(500),
        );
        spawn_local(async move {
            let started = invoke::<Value>("play", json!({ "id": id })).await;
            match started {
                Ok(_) => {
                    ctx.home();
                    refresh_running(ctx);
                }
                Err(_) => ctx.toast.set(Some(use_err())),
            }
        });
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
}

fn use_err() -> String {
    "err-generic".to_owned()
}

/// Lance `work` une fois le DOM à jour.
pub fn after_render(work: impl FnOnce() + 'static) {
    request_animation_frame(move || request_animation_frame(work));
}

fn refresh_running(ctx: Ctx) {
    spawn_local(async move {
        if let Ok(running) = invoke::<Option<RunningEntry>>("running_entry", json!({})).await {
            ctx.running.set(running);
        }
    });
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

fn route_action(ctx: Ctx, action: &str) {
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
        "pagePrev" => cycle_filter(ctx, -1),
        "pageNext" => cycle_filter(ctx, 1),
        "menu" | "quickPanel" | "quickSettings" => toggle_panel(ctx),
        _ => {}
    }
}

fn toggle_panel(ctx: Ctx) {
    refresh_running(ctx);
    ctx.panel.update(|open| *open = !*open);
    after_render(nav::focus_first);
}

fn step(ctx: Ctx, direction: Direction) {
    if nav::move_focus(direction) {
        ctx.tone(Tone::Move);
    }
}

/// Sur l'arc, gauche et droite suivent l'ordre des jeux, y compris en droite-gauche.
fn horizontal(ctx: Ctx, direction: Direction) {
    let on_card = nav::active_card().is_some();
    if on_card {
        let rtl = ctx_rtl();
        let forward = (direction == Direction::Right) != rtl;
        let count = visible_entries(ctx).len() as i32;
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

fn ctx_rtl() -> bool {
    use_context::<I18n>()
        .and_then(|i18n| i18n.current())
        .is_some_and(|lang| lang.rtl)
}

fn cycle_filter(ctx: Ctx, delta: i32) {
    if ctx.screens.read_untracked().last() != Some(&Screen::Home) {
        return;
    }
    let order = [
        None,
        Some(EntryKind::Game),
        Some(EntryKind::Software),
        Some(EntryKind::Social),
    ];
    let current = ctx.filter.get_untracked();
    let index = order.iter().position(|item| *item == current).unwrap_or(0) as i32;
    let next = (index + delta).rem_euclid(order.len() as i32) as usize;
    ctx.filter.set(order[next]);
    ctx.selected.set(0);
    ctx.tone(Tone::Move);
    after_render(nav::focus_first);
}

fn on_key(ctx: Ctx, event: &KeyboardEvent) {
    ctx.device.set(Device::Keyboard);
    let key = event.key();
    let handled = match key.as_str() {
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
        "Tab"
            if ctx.screens.read_untracked().last() == Some(&Screen::Home)
                && !ctx.panel.get_untracked() =>
        {
            cycle_filter(ctx, if event.shift_key() { -1 } else { 1 });
            true
        }
        "F1" => {
            toggle_panel(ctx);
            true
        }
        _ => false,
    };
    if handled {
        event.prevent_default();
    }
}

#[component]
pub fn App() -> impl IntoView {
    let system = web_sys::window()
        .and_then(|w| w.navigator().language())
        .unwrap_or_default();
    let i18n = I18n::new(best_match(&system));
    provide_context(i18n);
    let ctx = Ctx {
        screens: RwSignal::new(Vec::new()),
        settings: RwSignal::new(None),
        entries: RwSignal::new(Vec::new()),
        filter: RwSignal::new(None),
        selected: RwSignal::new(0),
        panel: RwSignal::new(false),
        running: RwSignal::new(None),
        device: RwSignal::new(Device::Mouse),
        family: RwSignal::new("other".to_owned()),
        toast: RwSignal::new(None),
        wipe: RwSignal::new(false),
        dialog_open: RwSignal::new(false),
        editing: RwSignal::new_local(None),
    };
    provide_context(ctx);

    Effect::new(move |_| {
        let lang = i18n.current();
        if let Some(root) = web_sys::window()
            .and_then(|w| w.document())
            .and_then(|d| d.document_element())
        {
            let _ = root.set_attribute("lang", lang.map_or("en", |l| l.code));
            let _ = root.set_attribute(
                "dir",
                if lang.is_some_and(|l| l.rtl) {
                    "rtl"
                } else {
                    "ltr"
                },
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
            set_timeout(
                move || ctx.toast.set(None),
                std::time::Duration::from_secs(4),
            );
        }
        previous.unwrap_or(())
    });

    spawn_local(async move {
        let settings: Option<Settings> = invoke("get_settings", json!({})).await.ok();
        if let Some(settings) = settings {
            apply_appearance(&settings);
            if let Some(code) = settings.language.as_deref() {
                if let Some(lang) = LANGS.iter().find(|lang| lang.code == code) {
                    i18n.lang.set(lang.code);
                }
            }
            let start = if settings.language.is_none() {
                Screen::Language
            } else if !settings.first_run_done {
                Screen::Welcome
            } else {
                Screen::Home
            };
            ctx.settings.set(Some(settings));
            ctx.screens.set(vec![start]);
        }
        ctx.reload();
        after_render(nav::focus_first);
        let boot: Value = invoke("boot_info", json!({})).await.unwrap_or_default();
        if boot["autostartDisabled"] == json!(true) {
            ctx.toast.set(Some("autostart-disabled".to_owned()));
        }
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
            ctx.toast
                .set(Some(i18n.ts("ctl-connected", "family", &name)));
        }
        ctx.family.set(family);
    });
    api::listen("library-changed", move |_| ctx.reload());
    api::listen("session-ended", move |value| {
        if let Ok(end) = serde_wasm_bindgen::from_value::<SessionEnd>(value) {
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
        ctx.toast.set(Some(
            if ok { "store-done" } else { "store-failed" }.to_owned(),
        ));
        ctx.reload();
    });

    let keydown = Rc::new(Cell::new(None::<Closure<dyn FnMut(KeyboardEvent)>>));
    let closure = Closure::<dyn FnMut(KeyboardEvent)>::new(move |event| on_key(ctx, &event));
    let mouse = Closure::<dyn FnMut()>::new(move || ctx.device.set(Device::Mouse));
    if let Some(window) = web_sys::window() {
        let _ =
            window.add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
        let _ =
            window.add_event_listener_with_callback("mousemove", mouse.as_ref().unchecked_ref());
    }
    keydown.set(Some(closure));
    std::mem::forget(keydown);
    mouse.forget();

    view! {
        <div class="boot"></div>
        <CurrentScreen/>
        <Show when=move || ctx.panel.get()><QuickPanel/></Show>
        <TextOverlay/>
        <Show when=move || ctx.wipe.get()><div class="iris-wipe"></div></Show>
        <Toast/>
        <Hints/>
    }
}
