//! Écrans de l'interface, direction « Lumen ». Aucun texte en dur : tout passe
//! par les traductions. Tout élément interactif est un `button.f`.

use crate::api::{gb, invoke, CatalogItem, ConsoleReport, ControllerStatus, DirItem, Merged, Preview, SystemReport};
use crate::app::{after_render, ctx, Ctx, Device, Screen, TextEdit};
use crate::connect::ConnectionsScreen;
use crate::i18n::{use_i18n, LANGS};
use crate::icons::Icon;
use crate::keyboard;
use crate::nav;
use crate::setup::SetupScreen;
use focus_core::art::ArtKind;
use focus_core::model::{EntryKind, Store};
use focus_core::plan::Step;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{json, Value};
use wasm_bindgen::JsCast;

/// Teinte stable tirée du titre, quand une entrée n'a ni image ni couleur.
pub fn hue(title: &str) -> u32 {
    title
        .chars()
        .fold(7_u32, |sum, c| sum.wrapping_mul(31).wrapping_add(c as u32))
        % 360
}

/// Couleur d'accent d'une entrée.
pub fn accent_of(item: &Merged) -> String {
    item.art
        .accent
        .clone()
        .unwrap_or_else(|| format!("hsl({} 60% 58%)", hue(&item.entry.title)))
}

fn store_name(store: Store) -> &'static str {
    match store {
        Store::Steam => "Steam",
        Store::Epic => "Epic Games",
        Store::Gog => "GOG",
        Store::Ea => "EA",
        Store::Ubisoft => "Ubisoft",
        Store::BattleNet => "Battle.net",
        Store::Xbox => "Xbox",
        Store::Manual => "PC",
    }
}

/// Entrée mise en avant : la fiche ouverte, sinon la sélection de l'accueil.
fn focus_entry(ctx: Ctx) -> Option<Merged> {
    let wanted = ctx.screens.with(|list| match list.last() {
        Some(Screen::Detail(id)) | Some(Screen::Prepare(id)) => Some(id.clone()),
        _ => None,
    });
    if let Some(id) = wanted {
        return ctx
            .entries
            .with(|list| list.iter().find(|item| item.entry.id == id).cloned());
    }
    let list = ctx.visible.get();
    let index = ctx.selected.get().clamp(0, (list.len() as i32 - 1).max(0));
    list.get(index as usize).cloned()
}

fn css_url(url: Option<String>) -> String {
    url.map(|url| format!("url(\"{url}\")")).unwrap_or_default()
}

/// Fond plein écran : l'image du jeu mis en avant, en fondu entre deux couches.
#[component]
pub fn Backdrop() -> impl IntoView {
    let ctx = ctx();
    let layers = [RwSignal::new(String::new()), RwSignal::new(String::new())];
    let front = RwSignal::new(0_usize);
    let accent = RwSignal::new(String::from("#e9a23b"));
    // Le fond ne suit la sélection qu'une fois qu'elle s'arrête : faire défiler la
    // rangée ne décode plus une grande image (ni ne recalcule la couleur de toute la
    // page) à chaque cran.
    let generation = StoredValue::new(0_u32);
    Effect::new(move |previous: Option<String>| {
        let item = focus_entry(ctx);
        let color = item.as_ref().map(accent_of);
        let url = css_url(item.and_then(|item| {
            item.art_url(ArtKind::Hero)
                .or_else(|| item.art_url(ArtKind::Cover))
        }));
        if previous.as_ref() != Some(&url) {
            generation.update_value(|value| *value = value.wrapping_add(1));
            let wanted = generation.get_value();
            let next_url = url.clone();
            set_timeout(
                move || {
                    if generation.get_value() != wanted {
                        return;
                    }
                    if let Some(color) = color {
                        // Tous les écrans prennent la couleur du jeu mis en avant.
                        if let Some(root) = web_sys::window()
                            .and_then(|w| w.document())
                            .and_then(|d| d.document_element())
                            .and_then(|root| root.dyn_into::<web_sys::HtmlElement>().ok())
                        {
                            let _ = root.style().set_property("--accent", &color);
                        }
                        accent.set(color);
                    }
                    let next = 1 - front.get_untracked();
                    layers[next].set(next_url);
                    front.set(next);
                },
                std::time::Duration::from_millis(if previous.is_none() { 0 } else { 180 }),
            );
        }
        url
    });
    let quiet = move || {
        ctx.screens
            .with(|list| !matches!(list.last(), Some(Screen::Home) | Some(Screen::Detail(_))))
    };
    view! {
        <div class="backdrop" class:quiet=quiet style:--accent=move || accent.get()>
            <div class="layer" class:on=move || front.get() == 0
                style:background-image=move || layers[0].get()></div>
            <div class="layer" class:on=move || front.get() == 1
                style:background-image=move || layers[1].get()></div>
            <div class="shade"></div>
        </div>
    }
}

#[component]
pub fn CurrentScreen() -> impl IntoView {
    let ctx = ctx();
    move || match ctx.screens.with(|list| list.last().cloned()) {
        Some(Screen::Setup(step)) => view! { <SetupScreen step=step/> }.into_any(),
        Some(Screen::Home) => view! { <HomeScreen/> }.into_any(),
        Some(Screen::Detail(id)) => view! { <DetailScreen id=id/> }.into_any(),
        Some(Screen::Prepare(id)) => view! { <PrepareScreen id=id/> }.into_any(),
        Some(Screen::Ended(end)) => view! { <EndedScreen end=end/> }.into_any(),
        Some(Screen::Settings) => view! { <SettingsScreen/> }.into_any(),
        Some(Screen::Connections) => view! { <ConnectionsScreen/> }.into_any(),
        Some(Screen::Analysis) => view! { <AnalysisScreen/> }.into_any(),
        Some(Screen::Profile) => view! { <ProfileScreen/> }.into_any(),
        Some(Screen::AddFile) => view! { <AddFileScreen/> }.into_any(),
        Some(Screen::Stores) => view! { <StoresScreen/> }.into_any(),
        Some(Screen::Power) => view! { <PowerScreen/> }.into_any(),
        None => ().into_any(),
    }
}

/// Interrupteur : un bouton qui annonce son état.
#[component]
pub fn Toggle(
    label: &'static str,
    value: Signal<bool>,
    on_change: Callback<()>,
    #[prop(optional)] auto: bool,
) -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <button class="f toggle" aria-pressed=move || value.get().to_string()
            data-auto=auto.then_some("") on:click=move |_| on_change.run(())>
            <span class="label">{move || i18n.t(label)}</span>
            <span class="switch" class:on=move || value.get()><span class="knob"></span></span>
        </button>
    }
}

/// Interrupteur lié à un réglage booléen.
#[macro_export]
macro_rules! setting_toggle {
    ($ctx:expr, $label:literal, $field:ident) => {{
        let ctx = $ctx;
        view! {
            <$crate::screens::Toggle label=$label
                value=Signal::derive(move || ctx.settings.with(|s| s.as_ref().is_some_and(|s| s.$field)))
                on_change=Callback::new(move |_| ctx.update_settings(|s| s.$field = !s.$field))/>
        }
    }};
}

/// Barre d'état : réseau, manette, heure.
#[component]
pub fn StatusBar() -> impl IntoView {
    let ctx = ctx();
    view! {
        <div class="status">
            {move || match ctx.net.with(|net| net.kind.clone()).as_str() {
                "wifi" => view! { <Icon name="wifi" level=4/> }.into_any(),
                "ethernet" => view! { <Icon name="ethernet"/> }.into_any(),
                _ => view! { <Icon name="offline"/> }.into_any(),
            }}
            {move || ctx.battery.get().map(|level| view! {
                <span class="pad-battery"><Icon name="gamepad"/><Icon name="battery" level=level/></span>
            })}
            <span class="clock">{move || ctx.clock.get()}</span>
        </div>
    }
}

/// Phrases simples qui disent ce que le mode console a changé sur le PC. Aucun mot
/// technique : quelqu'un qui vient d'une console doit comprendre du premier coup.
fn profile_lines(i18n: crate::i18n::I18n, report: &ConsoleReport) -> Vec<String> {
    let mut lines = Vec::new();
    if report.shell_stopped {
        lines.push(i18n.t("profile-shell"));
    }
    if report.freed_gb() >= 0.1 {
        lines.push(i18n.ts("profile-freed", "gb", &format!("{:.1}", report.freed_gb())));
    }
    if report.full_power() {
        lines.push(i18n.t("profile-power"));
    }
    for (count, key) in [(report.slowed, "profile-slowed"), (report.closed, "profile-closed"), (report.paused, "profile-paused")] {
        if count > 0 {
            lines.push(i18n.tn(key, "count", count));
        }
    }
    lines
}

/// Pastille de l'accueil : dit si le PC est en mode console, et ouvre le détail.
#[component]
fn ProfileChip() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let on = move || ctx.profile.with(Option::is_some);
    let label = move || {
        i18n.t(if ctx.profile_busy.get() {
            "profile-busy"
        } else if on() {
            "profile-on"
        } else {
            "profile-off"
        })
    };
    let gain = move || {
        ctx.profile
            .with(|report| report.as_ref().map(ConsoleReport::freed_gb))
            .filter(|freed| *freed >= 0.1)
            .map(|freed| format!("+{freed:.1} {}", i18n.t("unit-gb")))
    };
    view! {
        <button class="f profile" class:on=on class:busy=move || ctx.profile_busy.get()
            on:click=move |_| ctx.push(Screen::Profile)>
            <Icon name="bolt"/><span>{label}</span>
            {move || gain().map(|text| view! { <span class="gain">{text}</span> })}
        </button>
    }
}

/// Annonce du changement de profil : elle descend quand le PC vient de passer en mode
/// console, liste ce qui a changé, puis repart seule.
#[component]
pub fn ProfileFlash() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    move || {
        if !ctx.flash.get() {
            return None;
        }
        let report = ctx.profile.get()?;
        let lines = profile_lines(i18n, &report)
            .into_iter()
            .take(5)
            .map(|line| view! { <li><Icon name="check"/>{line}</li> })
            .collect_view();
        Some(view! {
            <div class="flash" role="status">
                <Icon name="bolt"/>
                <div><strong>{i18n.t("profile-flash")}</strong><ul>{lines}</ul></div>
            </div>
        })
    }
}

/// Question posée une fois : des applications sont encore ouvertes, faut-il les
/// fermer ? Aucune n'est forcée : celle qui propose d'enregistrer reste ouverte.
#[component]
pub fn AppsDialog() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let closing = RwSignal::new(false);
    let close = move |_| {
        closing.set(true);
        spawn_local(async move {
            let report = invoke::<Option<ConsoleReport>>("close_open_apps", json!({})).await.ok().flatten();
            closing.set(false);
            ctx.apps_ask.set(false);
            ctx.dialog_open.set(false);
            if let Some(report) = report {
                crate::app::show_profile(ctx, Some(report));
            }
            after_render(nav::focus_first);
        });
    };
    view! {
        <Show when=move || ctx.apps_ask.get() && ctx.dialog_open.get()>
            {move || {
                let apps = ctx.profile.with(|report| report.as_ref().map(|report| report.apps.clone()).unwrap_or_default());
                let count = apps.len() as u64;
                view! {
                    <div class="dialog"><div class="card">
                        <h2>{i18n.tn("apps-title", "count", count)}</h2>
                        <p class="apps-names">{apps.join(" · ")}</p>
                        <p>{move || i18n.t(if closing.get() { "apps-closing" } else { "apps-body" })}</p>
                        <div class="row">
                            <button class="f" disabled=move || closing.get() on:click=move |_| ctx.back()>
                                {move || i18n.t("apps-keep")}</button>
                            <button class="f primary" data-auto="" disabled=move || closing.get() on:click=close>
                                {move || i18n.t("apps-close")}</button>
                        </div>
                    </div></div>
                }
            }}
        </Show>
    }
}

/// Écran du profil : ce que Focus fait pour le joueur, avant et maintenant.
#[component]
fn ProfileScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let ask = move |_| {
        ctx.apps_ask.set(true);
        ctx.dialog_open.set(true);
        after_render(nav::focus_first);
    };
    view! {
        <main class="screen sheet-screen">
            <h1>{move || i18n.t("profile-title")}</h1>
            {move || match ctx.profile.get() {
                None => view! { <p class="lead">{i18n.t("profile-none")}</p> }.into_any(),
                Some(report) => {
                    let unit = i18n.t("unit-gb");
                    let freed = report.freed_gb();
                    let free_share = (report.memory_after * 100).checked_div(report.memory_total).unwrap_or(0);
                    let memory_now = format!("{} {unit}", gb(report.memory_after));
                    let memory_before = (freed >= 0.1).then(|| format!("{} {unit}", gb(report.memory_before)));
                    let memory_gain = (freed >= 0.1).then(|| format!("+{freed:.1} {unit}"));
                    let power_now = report.power_after.as_deref().map(|plan| i18n.t(ConsoleReport::power_key(plan)));
                    let power_before = report
                        .power_before
                        .as_deref()
                        .filter(|plan| Some(*plan) != report.power_after.as_deref())
                        .map(|plan| i18n.t(ConsoleReport::power_key(plan)));
                    let lines = profile_lines(i18n, &report)
                        .into_iter()
                        .map(|line| view! { <li><Icon name="check"/>{line}</li> })
                        .collect_view();
                    let apps = report.apps.len() as u64;
                    view! {
                        <p class="lead">{i18n.t(if report.optimized() { "profile-lead" } else { "profile-lead-off" })}</p>
                        <div class="compare">
                            <div class="card">
                                <span class="label">{i18n.t("profile-memory")}</span>
                                <div class="values">
                                    <span class="now">{memory_now}</span>
                                    {memory_before.map(|text| view! { <span class="before">{text}</span> })}
                                    {memory_gain.map(|text| view! { <span class="gain">{text}</span> })}
                                </div>
                                <div class="meter"><span style:width=format!("{free_share}%")></span></div>
                            </div>
                            {power_now.map(|now| view! {
                                <div class="card">
                                    <span class="label">{i18n.t("profile-power-label")}</span>
                                    <div class="values">
                                        <span class="now">{now}</span>
                                        {power_before.map(|text| view! { <span class="before">{text}</span> })}
                                    </div>
                                </div>
                            })}
                        </div>
                        <ul class="summary">{lines}</ul>
                        {(apps > 0).then(|| view! {
                            <div class="row">
                                <button class="f pill" on:click=ask>
                                    <Icon name="close"/><span>{i18n.tn("apps-title", "count", apps)}</span>
                                </button>
                            </div>
                        })}
                    }.into_any()
                }
            }}
            <div class="row">
                <button class="f primary" data-auto="" on:click=move |_| ctx.back()>{move || i18n.t("hint-back")}</button>
                <button class="f pill" on:click=move |_| {
                    ctx.set_section.set("opt");
                    ctx.push(Screen::Settings);
                }><Icon name="settings"/><span>{move || i18n.t("set-section-opt")}</span></button>
            </div>
        </main>
    }
}

#[component]
fn TopBar() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let tabs = [
        (None, "filter-all"),
        (Some(EntryKind::Game), "filter-games"),
        (Some(EntryKind::Software), "filter-software"),
        (None, "filter-stores"),
        (Some(EntryKind::Social), "filter-social"),
    ]
    .into_iter()
    .map(|(kind, label)| {
        view! {
            <button class="f tab"
                class:active=move || if label == "filter-stores" { ctx.stores_tab.get() } else { !ctx.stores_tab.get() && ctx.filter.get() == kind }
                on:click=move |_| {
                    ctx.stores_tab.set(label == "filter-stores");
                    if label != "filter-stores" {
                        ctx.filter.set(kind);
                    }
                    ctx.selected.set(0);
                }>{move || i18n.t(label)}</button>
        }
    })
    .collect_view();
    let icon_button = move |icon: &'static str, label: &'static str, screen: Screen| {
        view! {
            <button class="f round" title=move || i18n.t(label) aria-label=move || i18n.t(label)
                on:click=move |_| ctx.push(screen.clone())><Icon name=icon/></button>
        }
    };
    view! {
        <nav class="topbar">
            <div class="tabs">{tabs}</div>
            <div class="actions">
                <ProfileChip/>
                <button class="f round" title=move || i18n.t("top-search") aria-label=move || i18n.t("top-search")
                    on:click=move |_| ctx.open_search()><Icon name="search"/></button>
                {icon_button("add", "top-add", Screen::AddFile)}
                {icon_button("store", "top-stores", Screen::Stores)}
                {icon_button("settings", "top-settings", Screen::Settings)}
                {icon_button("power", "top-power", Screen::Power)}
                <StatusBar/>
            </div>
        </nav>
    }
}

/// Jaquette de la rangée. Seule la sélection est dans le parcours de la manette.
#[component]
fn Tile(index: usize, item: Merged) -> impl IntoView {
    let ctx = ctx();
    let selected = Memo::new(move |_| ctx.selected.get() == index as i32);
    let cover = item.art_url(ArtKind::Cover);
    let icon = item.art_url(ArtKind::Icon);
    let accent = accent_of(&item);
    let (id, title) = (item.entry.id, item.entry.title);
    let label = title.clone();
    let art = match cover {
        Some(url) => view! { <img class="cover" src=url alt="" decoding="async" draggable="false"/> }.into_any(),
        None => view! {
            <span class="placeholder">
                {icon.map(|url| view! { <img class="app-icon" src=url alt="" decoding="async" draggable="false"/> })}
                <span class="name">{title}</span>
            </span>
        }
        .into_any(),
    };
    view! {
        <button class="f tile" class:selected=move || selected.get() data-card=index
            data-auto=move || selected.get().then_some("")
            data-skip=move || (!selected.get()).then_some("")
            tabindex=move || if selected.get() { "0" } else { "-1" }
            style:--tile-accent=accent aria-label=label
            on:click=move |_| {
                if selected.get_untracked() {
                    ctx.play(id.clone());
                } else {
                    ctx.selected.set(index as i32);
                    after_render(nav::focus_first);
                }
            }>
            {art}
        </button>
    }
}

#[component]
fn HomeScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let hero = move || {
        focus_entry(ctx).map(|item| {
            let logo = item.art_url(ArtKind::Logo);
            let title = item.entry.title.clone();
            let hours = item.entry.stats.total_seconds / 3600;
            let store = store_name(item.entry.store);
            let game = item.entry.kind == EntryKind::Game;
            let (play_id, more_id) = (item.entry.id.clone(), item.entry.id);
            let heading = match logo {
                Some(url) => view! { <img class="logo" src=url alt=title decoding="async"/> }.into_any(),
                None => view! { <h1 class="title">{title}</h1> }.into_any(),
            };
            view! {
                <section class="hero-info">
                    {heading}
                    <p class="meta">
                        <span class="chip">{store}</span>
                        <span>{i18n.tn("home-played", "hours", hours)}</span>
                    </p>
                    <div class="actions">
                        <button class="f play" on:click=move |_| ctx.play(play_id.clone())>
                            <Icon name="play"/>
                            <span>{i18n.t(if game { "detail-play" } else { "detail-open" })}</span>
                        </button>
                        <button class="f round" title=i18n.t("hint-options") aria-label=i18n.t("hint-options")
                            on:click=move |_| ctx.push(Screen::Detail(more_id.clone()))>
                            <Icon name="more"/>
                        </button>
                    </div>
                </section>
            }
        })
    };
    let searching = move || !ctx.search.get().is_empty();
    let tiles = move || ctx.visible.get().into_iter().enumerate().collect::<Vec<_>>();
    let tile_key = |(index, item): &(usize, Merged)| (item.entry.id.clone(), *index, item.art.cover, item.art.icon);
    view! {
        <main class="screen home">
            <TopBar/>
            <Show when=searching>
                <p class="search-line"><Icon name="search"/>
                    {move || i18n.ts("home-searching", "text", &ctx.search.get())}</p>
            </Show>
            <Show when=move || ctx.stores_tab.get()><StoreShelf/></Show>
            <Show when=move || !ctx.stores_tab.get()>
            <Show when=move || !ctx.visible.with(Vec::is_empty)
                fallback=move || view! {
                    <section class="empty">
                        <h1>{move || i18n.t("home-empty-title")}</h1>
                        <p>{move || i18n.t("home-empty-body")}</p>
                        <button class="f primary" data-auto="" on:click=move |_| ctx.push(Screen::AddFile)>
                            <Icon name="add"/><span>{move || i18n.t("top-add")}</span>
                        </button>
                    </section>
                }>
                {hero}
                <section class="shelf">
                    <div class="track" style:--sel=move || ctx.selected.get().to_string()>
                        <For each=tiles key=tile_key children=move |(index, item)| view! { <Tile index=index item=item/> }/>
                    </div>
                </section>
            </Show>
            </Show>
        </main>
    }
}

/// Filtre « Stores » : les boutiques installées s’ouvrent, les autres s’installent.
#[component]
fn StoreShelf() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let open = move |item: crate::api::CatalogItem| {
        if item.installed {
            spawn_local(async move {
                if invoke::<Value>("open_store", json!({ "id": item.id })).await.is_err() {
                    ctx.toast.set(Some("err-generic".to_owned()));
                }
            });
        } else {
            ctx.push(Screen::Stores);
        }
    };
    let selected = move || {
        let list = ctx.stores.get();
        let index = ctx.selected.get().clamp(0, (list.len() as i32 - 1).max(0));
        list.get(index as usize).cloned()
    };
    view! {
        {move || selected().map(|item| {
            let installed = item.installed;
            let action = item.clone();
            view! {
                <section class="hero-info">
                    <h1 class="title">{item.name.clone()}</h1>
                    <p class="meta"><span class="chip">{item.publisher}</span>
                        <span>{move || i18n.t(if installed { "store-installed" } else { "store-install" })}</span></p>
                    <div class="actions">
                        <button class="f play" on:click=move |_| open(action.clone())>
                            <Icon name=if installed { "play" } else { "add" }/>
                            <span>{move || i18n.t(if installed { "store-open" } else { "store-install" })}</span>
                        </button>
                    </div>
                </section>
            }
        })}
        <section class="shelf">
            <div class="track" style:--sel=move || ctx.selected.get().to_string()>
                {move || ctx.stores.get().into_iter().enumerate().map(|(index, item)| {
                    let is_selected = move || ctx.selected.get() == index as i32;
                    let accent = format!("hsl({} 55% 50%)", hue(&item.name));
                    let (name, installed, icon) = (item.name.clone(), item.installed, item.icon_url());
                    view! {
                        <button class="f tile" class:selected=is_selected data-card=index
                            data-auto=move || is_selected().then_some("")
                            data-skip=move || (!is_selected()).then_some("")
                            style:--tile-accent=accent
                            on:click=move |_| {
                                if is_selected() { open(item.clone()); } else { ctx.selected.set(index as i32); after_render(nav::focus_first); }
                            }>
                            <span class="placeholder">
                                {match icon {
                                    Some(url) => view! { <img class="app-icon" src=url alt="" decoding="async"/> }.into_any(),
                                    None => view! { <Icon name="store"/> }.into_any(),
                                }}
                                <span class="name">{name}</span>
                                {installed.then(|| view! { <span class="badge">{move || i18n.t("store-installed")}</span> })}</span>
                        </button>
                    }
                }).collect_view()}
            </div>
        </section>
    }
}

/// Dialogue de confirmation : « Non » est proposé par défaut.
#[component]
pub fn Confirm(
    question: Signal<String>,
    #[prop(optional)] detail: Option<&'static str>,
    on_yes: Callback<()>,
    #[prop(optional)] danger: bool,
) -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    view! {
        <div class="dialog"><div class="card">
            <h2>{move || question.get()}</h2>
            {detail.map(|key| view! { <p>{move || i18n.t(key)}</p> })}
            <div class="row">
                <button class="f" data-auto="" on:click=move |_| ctx.back()>{move || i18n.t("no")}</button>
                <button class="f" class:danger=danger class:primary=!danger
                    on:click=move |_| on_yes.run(())>{move || i18n.t("yes")}</button>
            </div>
        </div></div>
    }
}

#[component]
fn DetailScreen(id: String) -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let lookup = id.clone();
    let item = Memo::new(move |_| {
        ctx.entries
            .with(|list| list.iter().find(|item| item.entry.id == lookup).cloned())
    });
    let asking = RwSignal::new(false);
    let update = RwSignal::new(false);
    let (check_id, update_id) = (id.clone(), id.clone());
    spawn_local(async move {
        update.set(invoke::<Option<bool>>("update_status", json!({ "id": check_id })).await.ok().flatten().unwrap_or(false));
    });
    let (play_id, fav_id, del_id) = (id.clone(), id.clone(), id);
    let favorite = move || item.with(|item| item.as_ref().is_some_and(|item| item.entry.favorite));
    let question = Signal::derive(move || {
        let title = item.with(|item| item.as_ref().map(|i| i.entry.title.clone()).unwrap_or_default());
        i18n.ts("detail-remove-confirm", "title", &title)
    });
    view! {
        <main class="screen detail">
            {move || item.get().map(|item| {
                let entry = item.entry.clone();
                let hours = entry.stats.total_seconds / 3600;
                let launches = u64::from(entry.stats.launches);
                let heading = match item.art_url(ArtKind::Logo) {
                    Some(url) => view! { <img class="logo" src=url alt=entry.title.clone()/> }.into_any(),
                    None => view! { <h1 class="title">{entry.title.clone()}</h1> }.into_any(),
                };
                view! {
                    <div class="hero-info">
                        {heading}
                        <p class="meta">
                            <span class="chip">{store_name(entry.store)}</span>
                            <span>{i18n.tn("home-played", "hours", hours)}</span>
                            <span>{i18n.tn("detail-launches", "count", launches)}</span>
                        </p>
                    </div>
                }
            })}
            <Show when=move || update.get()>
                <p class="warn status-line"><Icon name="refresh"/>{move || i18n.t("detail-update")}</p>
            </Show>
            <div class="row actions">
                <Show when=move || update.get()>
                    {
                        let id = update_id.clone();
                        view! {
                            <button class="f pill" on:click=move |_| {
                                let id = id.clone();
                                spawn_local(async move {
                                    let _ = invoke::<Value>("open_updates", json!({ "id": id })).await;
                                });
                            }><Icon name="refresh"/><span>{move || i18n.t("detail-update-open")}</span></button>
                        }
                    }
                </Show>
                <button class="f play" data-auto="" on:click=move |_| ctx.play(play_id.clone())>
                    <Icon name="play"/>
                    <span>{move || {
                        let game = item.with(|i| i.as_ref().is_some_and(|i| i.entry.kind == EntryKind::Game));
                        i18n.t(if game { "detail-play" } else { "detail-open" })
                    }}</span>
                </button>
                <button class="f pill" on:click=move |_| {
                    let (id, value) = (fav_id.clone(), !favorite());
                    spawn_local(async move {
                        let _ = invoke::<Value>("set_favorite", json!({ "id": id, "favorite": value })).await;
                        ctx.reload();
                    });
                }>
                    {move || if favorite() { view! { <Icon name="star-full"/> } } else { view! { <Icon name="star"/> } }}
                    <span>{move || i18n.t(if favorite() { "detail-unfavorite" } else { "detail-favorite" })}</span>
                </button>
                <button class="f pill danger" on:click=move |_| {
                    asking.set(true);
                    ctx.dialog_open.set(true);
                    after_render(nav::focus_first);
                }><Icon name="trash"/><span>{move || i18n.t("detail-remove")}</span></button>
            </div>
            <Show when=move || asking.get() && ctx.dialog_open.get()>
                <Confirm question=question danger=true on_yes=Callback::new({
                    let id = del_id.clone();
                    move |_| {
                        let id = id.clone();
                        asking.set(false);
                        ctx.dialog_open.set(false);
                        spawn_local(async move {
                            let _ = invoke::<Value>("remove_entry", json!({ "id": id })).await;
                            ctx.reload();
                            ctx.home();
                        });
                    }
                })/>
            </Show>
        </main>
    }
}

fn step_text(i18n: crate::i18n::I18n, step: &Step) -> String {
    match step {
        Step::KeepAwake => i18n.t("prep-awake"),
        Step::HighPerformancePower => i18n.t("prep-power"),
        Step::Throttle { process } => i18n.ts("prep-throttle", "name", &process.name),
        Step::Suspend { process } => i18n.ts("prep-suspend", "name", &process.name),
        Step::Close { process, .. } => i18n.ts("prep-close", "name", &process.name),
        Step::StopService { name } => i18n.ts("prep-service", "name", name),
    }
}

#[component]
fn PrepareScreen(id: String) -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let preview = RwSignal::new(None::<Preview>);
    let request = id.clone();
    spawn_local(async move {
        preview.set(invoke("preview_session", json!({ "id": request })).await.ok());
        after_render(nav::focus_first);
    });
    view! {
        <main class="screen sheet-screen">
            <h1>{move || i18n.t("prep-title")}</h1>
            {move || preview.get().map(|preview| {
                let empty = preview.plan.steps.is_empty();
                let lines = preview.plan.steps.clone().into_iter()
                    .map(|step| view! { <li>{move || step_text(i18n, &step)}</li> })
                    .collect_view();
                // Un navigateur ouvre un processus par onglet : une seule ligne par nom.
                let mut names: Vec<_> = preview.plan.ask_user.clone();
                names.sort_by(|a, b| a.name.cmp(&b.name));
                names.dedup_by(|a, b| a.name == b.name);
                let asks = names.into_iter()
                    .map(|ask| view! { <li>{move || i18n.ts("prep-ask", "name", &ask.name)}</li> })
                    .collect_view();
                view! {
                    <p>{move || i18n.t(if preview.simulation { "prep-simulation" } else { "prep-real" })}</p>
                    <Show when=move || !preview.guardian && !preview.simulation>
                        <p class="warn">{move || i18n.t("prep-no-guardian")}</p>
                    </Show>
                    <Show when=move || empty><p>{move || i18n.t("prep-nothing")}</p></Show>
                    <ul class="steps-list">{lines}{asks}</ul>
                }
            })}
            <div class="row">
                <button class="f play" data-auto="" on:click=move |_| ctx.launch(id.clone())>
                    <Icon name="play"/><span>{move || i18n.t("prep-start")}</span>
                </button>
                <button class="f pill" on:click=move |_| ctx.back()>{move || i18n.t("prep-cancel")}</button>
            </div>
        </main>
    }
}

#[component]
fn EndedScreen(end: crate::api::SessionEnd) -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    // Comme sur une console : l’écran de fin s’efface seul vers la bibliothèque.
    let id = end.entry_id.clone();
    set_timeout(move || {
        if ctx.screens.with_untracked(|list| matches!(list.last(), Some(Screen::Ended(_)))) {
            ctx.home();
        }
    }, std::time::Duration::from_secs(6));
    let cover = ctx.entries.with_untracked(|list| {
        list.iter().find(|item| item.entry.id == id).and_then(|item| item.art_url(ArtKind::Cover).or_else(|| item.art_url(ArtKind::Icon)))
    });
    let minutes = end.seconds / 60;
    let restored = end.restored;
    view! {
        <main class="screen sheet-screen">
            {cover.map(|url| view! { <img class="end-cover" src=url alt=""/> })}
            <h1>{move || i18n.t("end-title")}</h1>
            <h2>{end.title}</h2>
            <p class="muted pulse">{move || i18n.t("end-auto")}</p>
            <p>{move || i18n.tn("end-time", "minutes", minutes)}</p>
            <p class:ok=restored class:warn=!restored>
                {move || i18n.t(if restored { "end-restored" } else { "end-problem" })}
            </p>
            <div class="row">
                <button class="f primary" data-auto="" on:click=move |_| ctx.home()>
                    <Icon name="library"/><span>{move || i18n.t("end-back")}</span>
                </button>
            </div>
        </main>
    }
}

/// Choix du niveau d'optimisation, avec accord explicite pour un niveau réel (règle 8).
#[component]
pub fn OptimizationChoice() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let pending = RwSignal::new(None::<&'static str>);
    let current = move || ctx.settings.with(|s| s.as_ref().map(|s| s.optimization.clone()).unwrap_or_default());
    let options = [("off", "opt-off", "opt-off-body"), ("balanced", "opt-balanced", "opt-balanced-body"), ("max", "opt-max", "opt-max-body")]
        .into_iter()
        .map(|(level, title, body)| view! {
            <button class="f option" aria-pressed=move || (current() == level).to_string()
                data-auto=(level == "max").then_some("")
                on:click=move |_| {
                    if level == "off" {
                        ctx.update_settings(|s| s.optimization = level.to_owned());
                    } else if current() != level {
                        pending.set(Some(level));
                        ctx.dialog_open.set(true);
                        after_render(nav::focus_first);
                    }
                }>
                <span class="option-title"><Icon name="bolt"/>{move || i18n.t(title)}</span>
                <span class="option-body">{move || i18n.t(body)}</span>
            </button>
        })
        .collect_view();
    view! {
        <div class="options">{options}</div>
        <Show when=move || pending.get().is_some() && ctx.dialog_open.get()>
            <Confirm question=Signal::derive(move || i18n.t("set-real-confirm")) detail="set-real-body"
                on_yes=Callback::new(move |_| {
                    if let Some(level) = pending.get_untracked() {
                        ctx.update_settings(|s| s.optimization = level.to_owned());
                    }
                    pending.set(None);
                    ctx.back();
                })/>
        </Show>
    }
}

/// Après un changement du mode console, Windows suit tout de suite.
pub fn apply_console() {
    spawn_local(async move {
        let _ = invoke::<Value>("apply_console", json!({})).await;
    });
}

#[component]
fn SettingsScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let autostart = RwSignal::new(false);
    let guardian = RwSignal::new(false);
    let status = RwSignal::new(None::<ControllerStatus>);
    let detect = move || {
        spawn_local(async move {
            status.set(invoke("controller_status", json!({})).await.ok());
        });
    };
    detect();
    let steam_guide = RwSignal::new(None::<bool>);
    spawn_local(async move {
        steam_guide.set(invoke("steam_guide_status", json!({})).await.ok().flatten());
    });
    spawn_local(async move {
        autostart.set(invoke("set_autostart", json!({ "enabled": null })).await.unwrap_or(false));
        guardian.set(invoke("guardian_status", json!({})).await.unwrap_or(false));
    });
    let console_toggle = move |label: &'static str, read: fn(&crate::api::Settings) -> bool, write: fn(&mut crate::api::Settings)| {
        view! {
            <Toggle label=label value=Signal::derive(move || ctx.settings.with(|s| s.as_ref().is_some_and(read)))
                on_change=Callback::new(move |_| {
                    ctx.update_settings(write);
                    apply_console();
                })/>
        }
    };
    let lang_buttons = move || LANGS.iter().map(|lang| view! {
        <button class="f chip-button" lang=lang.code aria-pressed=move || (i18n.lang.get() == lang.code).to_string()
            on:click=move |_| {
                i18n.lang.set(lang.code);
                ctx.update_settings(|s| s.language = Some(lang.code.to_owned()));
            }>{lang.name}</button>
    }).collect_view();
    let glyphs = move || ["auto", "playStation", "xbox"].map(|style| view! {
        <button class="f chip-button" aria-pressed=move || ctx.settings.with(|s| s.as_ref().is_some_and(|s| s.glyphs == style)).to_string()
            on:click=move |_| ctx.update_settings(|s| s.glyphs = style.to_owned())>
            {move || i18n.t(&format!("glyph-{style}"))}
        </button>
    }).into_iter().collect_view();
    let section = ctx.set_section;
    let nav = [
        ("console", "display", "set-section-console"),
        ("opt", "bolt", "set-section-opt"),
        ("pad", "gamepad", "set-glyphs"),
        ("sound", "volume", "set-section-sound"),
        ("network", "wifi", "con-title"),
        ("language", "globe", "set-language"),
        ("system", "info", "set-analysis"),
    ]
    .into_iter()
    .map(|(key, icon, label)| view! {
        <button class="f nav-item" class:active=move || section.get() == key
            data-auto=move || (section.get() == key).then_some("")
            on:click=move |_| section.set(key) on:focus=move |_| section.set(key)>
            <Icon name=icon/><span>{move || i18n.t(label)}</span>
        </button>
    })
    .collect_view();
    view! {
        <main class="screen settings">
            <h1>{move || i18n.t("set-title")}</h1>
            <div class="settings-layout">
            <nav class="settings-nav">{nav}</nav>
            <div class="settings-body">
            <Show when=move || section.get() == "console">
            <section class="group">
                <h2><Icon name="display"/>{move || i18n.t("set-section-console")}</h2>
                <p>{move || i18n.t("set-console-body")}</p>
                {console_toggle("set-console", |s| s.console_mode, |s| s.console_mode = !s.console_mode)}
                {console_toggle("set-stop-shell", |s| s.stop_shell, |s| s.stop_shell = !s.stop_shell)}
                {console_toggle("set-hide-taskbar", |s| s.hide_taskbar, |s| s.hide_taskbar = !s.hide_taskbar)}
                {console_toggle("set-silence", |s| s.silence_notifications, |s| s.silence_notifications = !s.silence_notifications)}
                <Toggle label="set-autostart" value=Signal::derive(move || autostart.get())
                    on_change=Callback::new(move |_| {
                        let wanted = !autostart.get_untracked();
                        spawn_local(async move {
                            autostart.set(invoke("set_autostart", json!({ "enabled": wanted })).await.unwrap_or(false));
                        });
                    })/>
                {crate::setting_toggle!(ctx, "set-start-minimized", start_minimized)}
            </section>
            </Show>
            <Show when=move || section.get() == "opt">
            <section class="group">
                <h2><Icon name="bolt"/>{move || i18n.t("set-section-opt")}</h2>
                <OptimizationChoice/>
                {crate::setting_toggle!(ctx, "set-keep-social", keep_social)}
                {crate::setting_toggle!(ctx, "set-awake", keep_awake)}
                {crate::setting_toggle!(ctx, "set-high-perf", high_performance)}
                {crate::setting_toggle!(ctx, "set-big-picture", block_big_picture)}
                <p>{move || i18n.t("set-guardian-body")}</p>
                <button class="f pill" on:click=move |_| {
                    spawn_local(async move {
                        let _ = invoke::<Value>("install_guardian", json!({})).await;
                    });
                }>{move || i18n.t(if guardian.get() { "set-guardian-on" } else { "set-guardian-install" })}</button>
            </section>
            </Show>
            <Show when=move || section.get() == "pad">
            <section class="group">
                <h2><Icon name="gamepad"/>{move || i18n.t("set-glyphs")}</h2>
                <div class="row">{glyphs}</div>
                <div class="row">{move || [None, Some("native"), Some("focusInput")].map(|mode| view! {
                    <button class="f chip-button"
                        aria-pressed=move || ctx.settings.with(|s| s.as_ref().is_some_and(|s| s.controller_mode.as_deref() == mode)).to_string()
                        on:click=move |_| {
                            ctx.update_settings(|s| s.controller_mode = mode.map(str::to_owned));
                            detect();
                        }>
                        {move || i18n.t(&format!("mode-{}", mode.unwrap_or("auto")))}
                    </button>
                }).into_iter().collect_view()}</div>
                {crate::setting_toggle!(ctx, "set-vibration", vibration)}
                <button class="f pill" on:click=move |_| detect()>{move || i18n.t("set-detect")}</button>
                {move || status.get().map(|status| {
                    let family = i18n.t(&format!("family-{}", status.family));
                    let mode = i18n.t(&format!("mode-{}", status.mode));
                    view! { <p>{i18n.ts("ctl-family", "family", &family)}" · "{i18n.ts("ctl-mode", "mode", &mode)}</p> }
                })}
                <p class="muted">{move || i18n.t("ctl-native-note")}</p>
                {move || steam_guide.get().map(|focuses| if focuses {
                    view! {
                        <p class="warn">{move || i18n.t("steam-guide-on")}</p>
                        <button class="f pill" on:click=move |_| {
                            spawn_local(async move {
                                let _ = invoke::<Value>("open_steam_controller_settings", json!({})).await;
                            });
                        }>{move || i18n.t("steam-guide-open")}</button>
                    }.into_any()
                } else {
                    view! { <p class="ok">{move || i18n.t("steam-guide-off")}</p> }.into_any()
                })}
            </section>
            </Show>
            <Show when=move || section.get() == "sound">
            <section class="group">
                <h2><Icon name="volume"/>{move || i18n.t("set-section-sound")}</h2>
                {crate::setting_toggle!(ctx, "set-sounds", sounds)}
                {crate::setting_toggle!(ctx, "set-music", music)}
                {crate::setting_toggle!(ctx, "set-motion", reduce_motion)}
                {crate::setting_toggle!(ctx, "set-theme", auto_theme)}
            </section>
            </Show>
            <Show when=move || section.get() == "network">
            <section class="group">
                <h2><Icon name="wifi" level=4/>{move || i18n.t("con-title")}</h2>
                <div class="row">
                    <button class="f pill" on:click=move |_| { ctx.con_tab.set("wifi"); ctx.push(Screen::Connections); }>
                        <Icon name="wifi" level=4/><span>{move || i18n.t("con-wifi")}</span></button>
                    <button class="f pill" on:click=move |_| { ctx.con_tab.set("bluetooth"); ctx.push(Screen::Connections); }>
                        <Icon name="bluetooth"/><span>{move || i18n.t("con-bt")}</span></button>
                </div>
            </section>
            </Show>
            <Show when=move || section.get() == "language">
            <section class="group">
                <h2><Icon name="globe"/>{move || i18n.t("set-language")}</h2>
                <p>{move || {
                    let reviewed = i18n.current().is_some_and(|lang| lang.reviewed);
                    let status = i18n.t(if reviewed { "status-reviewed" } else { "status-draft" });
                    i18n.ts("set-lang-status", "status", &status)
                }}</p>
                <div class="lang-grid">{lang_buttons}</div>
            </section>
            </Show>
            <Show when=move || section.get() == "system">
            <section class="group">
                <h2><Icon name="info"/>{move || i18n.t("set-analysis")}</h2>
                <button class="f pill" on:click=move |_| ctx.push(Screen::Analysis)>{move || i18n.t("set-analysis")}</button>
            </section>
            </Show>
            </div>
            </div>
        </main>
    }
}

#[component]
fn AnalysisScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let report = RwSignal::new(None::<SystemReport>);
    spawn_local(async move {
        report.set(invoke("system_report", json!({})).await.ok().flatten());
    });
    view! {
        <main class="screen sheet-screen">
            <h1>{move || i18n.t("an-title")}</h1>
            {move || match report.get() {
                None => view! { <p>{move || i18n.t("an-loading")}</p> }.into_any(),
                Some(report) => {
                    let gb = (report.hardware.memory / (1 << 30)).to_string();
                    let cpu = report.hardware.cpu.clone();
                    let top = report.top.iter().map(|(name, memory)| view! {
                        <li><span class="mono">{name.clone()}</span>" · "{memory / (1 << 20)}" MB"</li>
                    }).collect_view();
                    view! {
                        <p>{i18n.ts("an-cpu", "name", &cpu)}</p>
                        <p>{i18n.ts("an-memory", "gb", &gb)}</p>
                        <p>{i18n.tn("an-processes", "count", report.processes)}</p>
                        <p>{i18n.tn("an-services", "count", report.services)}</p>
                        <h2>{i18n.t("an-top")}</h2>
                        <ul class="steps-list">{top}</ul>
                    }.into_any()
                }
            }}
            <div class="row">
                <button class="f primary" data-auto="" on:click=move |_| ctx.back()>{move || i18n.t("hint-back")}</button>
            </div>
        </main>
    }
}

fn parent_of(path: &str) -> Option<String> {
    let trimmed = path.trim_end_matches('\\');
    let cut = trimmed.rfind('\\')?;
    let parent = &trimmed[..cut];
    Some(if parent.len() == 2 {
        format!("{parent}\\")
    } else {
        parent.to_owned()
    })
}

fn title_from(path: &str) -> String {
    let mut parts = path.trim_end_matches('\\').rsplit('\\');
    let name = parts.next().unwrap_or(path);
    let stem = name.rsplit_once('.').map_or(name, |(stem, _)| stem);
    // Un lanceur s'appelle souvent « launcher » ou « start » : le dossier porte le
    // vrai nom du jeu (`Wuthering Waves\\launcher.exe`).
    if is_generic_name(stem) {
        if let Some(folder) = parts.next().filter(|folder| !folder.ends_with(':')) {
            return folder.to_owned();
        }
    }
    stem.to_owned()
}

/// Nom d'exécutable qui ne dit rien du jeu.
fn is_generic_name(stem: &str) -> bool {
    const GENERIC: [&str; 10] =
        ["launcher", "launch", "start", "game", "client", "play", "bootstrapper", "run", "main", "app"];
    let lower = stem.to_lowercase();
    let base = lower.trim_end_matches(|c: char| c.is_ascii_digit() || c == '_' || c == '-');
    GENERIC.contains(&lower.as_str()) || GENERIC.contains(&base)
}

#[component]
fn AddFileScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let folder = RwSignal::new(None::<String>);
    let items = RwSignal::new(Vec::<DirItem>::new());
    let suggested = RwSignal::new(Vec::<String>::new());
    let chosen = RwSignal::new(None::<String>);
    let title = RwSignal::new(String::new());
    let kind = RwSignal::new(EntryKind::Game);
    let open = move |path: Option<String>| {
        folder.set(path.clone());
        suggested.set(Vec::new());
        spawn_local(async move {
            let list = match path {
                Some(path) => invoke("list_dir", json!({ "path": path })).await,
                None => invoke("list_drives", json!({})).await,
            };
            items.set(list.unwrap_or_default());
            after_render(nav::focus_first);
        });
    };
    open(None);
    let choose = move |path: String| {
        title.set(title_from(&path));
        chosen.set(Some(path));
        after_render(nav::focus_first);
    };
    let kinds = [
        (EntryKind::Game, "kind-game"),
        (EntryKind::Software, "kind-software"),
        (EntryKind::Social, "kind-social"),
        (EntryKind::Other, "kind-other"),
    ].map(|(value, label)| view! {
        <button class="f chip-button" aria-pressed=move || (kind.get() == value).to_string() on:click=move |_| kind.set(value)>
            {move || i18n.t(label)}
        </button>
    }).into_iter().collect_view();
    view! {
        <main class="screen sheet-screen">
            <h1>{move || i18n.t("add-title")}</h1>
            <Show when=move || chosen.get().is_none()
                fallback=move || {
                    let path = chosen.get().unwrap_or_default();
                    let lower = path.to_lowercase();
                    let script = lower.ends_with(".bat") || lower.ends_with(".cmd");
                    let shown = path.clone();
                    view! {
                        <p class="mono">{shown}</p>
                        <Show when=move || script><p class="warn">{move || i18n.t("add-script")}</p></Show>
                        <div class="stack">
                            <button class="f pill" data-auto="" on:click=move |_| ctx.editing.set(Some(TextEdit { value: title, secret: false }))>
                                {move || format!("{} · {}", i18n.t("add-name"), title.get())}
                            </button>
                            <h2>{move || i18n.t("add-kind")}</h2>
                            <div class="row">{kinds.clone()}</div>
                            <button class="f primary" on:click=move |_| {
                                let (path, name, value) = (path.clone(), title.get_untracked(), kind.get_untracked());
                                spawn_local(async move {
                                    let _ = invoke::<Value>("add_file", json!({ "path": path, "title": name, "kind": value })).await;
                                    let _ = invoke::<Value>("refresh_library", json!({})).await;
                                    ctx.reload();
                                    ctx.home();
                                });
                            }><Icon name="add"/><span>{move || i18n.t("add-save")}</span></button>
                        </div>
                    }
                }>
                <p>{move || i18n.t("add-body")}</p>
                <p class="mono">{move || folder.get().unwrap_or_default()}</p>
                <div class="row">
                    <Show when=move || folder.get().is_some()>
                        <button class="f pill" on:click=move |_| open(folder.get_untracked().and_then(|p| parent_of(&p)))>
                            <Icon name="back"/><span>{move || i18n.t("add-up")}</span>
                        </button>
                        <button class="f pill" on:click=move |_| {
                            let path = folder.get_untracked().unwrap_or_default();
                            spawn_local(async move {
                                suggested.set(invoke("suggest_executables", json!({ "path": path })).await.unwrap_or_default());
                                after_render(nav::focus_first);
                            });
                        }><Icon name="search"/><span>{move || i18n.t("add-suggest")}</span></button>
                    </Show>
                </div>
                <Show when=move || !suggested.get().is_empty()>
                    <h2>{move || i18n.t("add-suggested")}</h2>
                    <div class="list">
                        {move || suggested.get().into_iter().map(|path| {
                            let (shown, value) = (path.clone(), path);
                            view! { <button class="f row-item mono" on:click=move |_| choose(value.clone())>{shown}</button> }
                        }).collect_view()}
                    </div>
                </Show>
                <div class="list">
                    {move || items.get().into_iter().map(|item| {
                        let DirItem { name, path, is_dir } = item;
                        view! {
                            <button class="f row-item" on:click=move |_| {
                                if is_dir { open(Some(path.clone())) } else { choose(path.clone()) }
                            }>{if is_dir { "▸ " } else { "" }}{name}</button>
                        }
                    }).collect_view()}
                </div>
            </Show>
        </main>
    }
}

#[component]
fn StoresScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let apps = RwSignal::new(Vec::<CatalogItem>::new());
    let asking = RwSignal::new(None::<CatalogItem>);
    spawn_local(async move {
        apps.set(invoke("catalog", json!({})).await.unwrap_or_default());
        after_render(nav::focus_first);
    });
    view! {
        <main class="screen sheet-screen">
            <h1>{move || i18n.t("store-title")}</h1>
            <div class="grid">
                {move || apps.get().into_iter().map(|app| {
                    let (name, page_id, installed) = (app.name.clone(), app.id.clone(), app.installed);
                    let winget = app.winget && !installed;
                    let install = app.clone();
                    view! {
                        <div class="card store-card">
                            <strong>{name}</strong>
                            <span class="muted">{app.publisher}</span>
                            <Show when=move || installed><span class="badge">{move || i18n.t("store-installed")}</span></Show>
                            <div class="row">
                                <Show when=move || winget>
                                    {
                                        let install = install.clone();
                                        view! {
                                            <button class="f primary" on:click=move |_| {
                                                asking.set(Some(install.clone()));
                                                ctx.dialog_open.set(true);
                                                after_render(nav::focus_first);
                                            }>{move || i18n.t("store-install")}</button>
                                        }
                                    }
                                </Show>
                                <button class="f pill" on:click=move |_| {
                                    let id = page_id.clone();
                                    spawn_local(async move {
                                        let _ = invoke::<Value>("open_official_page", json!({ "id": id })).await;
                                    });
                                }>{move || i18n.t("store-page")}</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>
            <Show when=move || asking.get().is_some() && ctx.dialog_open.get()>
                {move || asking.get().map(|app| {
                    let question = i18n.ts2("store-confirm", ("name", &app.name), ("publisher", &app.publisher));
                    let (id, name) = (app.id, app.name);
                    view! {
                        <div class="dialog"><div class="card">
                            <h2>{question}</h2>
                            <p>{move || i18n.t("store-source")}</p>
                            <p>{move || i18n.t("store-uac")}</p>
                            <div class="row">
                                <button class="f" data-auto="" on:click=move |_| ctx.back()>{move || i18n.t("no")}</button>
                                <button class="f primary" on:click=move |_| {
                                    let id = id.clone();
                                    ctx.toast.set(Some(i18n.ts("store-progress", "name", &name)));
                                    spawn_local(async move {
                                        let _ = invoke::<Value>("install_app", json!({ "id": id })).await;
                                    });
                                    ctx.back();
                                }>{move || i18n.t("yes")}</button>
                            </div>
                        </div></div>
                    }
                })}
            </Show>
        </main>
    }
}

#[component]
fn PowerScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let pending = RwSignal::new(None::<&'static str>);
    let ask = move |action| {
        pending.set(Some(action));
        ctx.dialog_open.set(true);
        after_render(nav::focus_first);
    };
    let choice = move |action: &'static str, icon: &'static str, label: &'static str, body: &'static str, auto: bool| view! {
        <button class="f option" data-auto=auto.then_some("") on:click=move |_| ask(action)>
            <span class="option-title"><Icon name=icon/>{move || i18n.t(label)}</span>
            <span class="option-body">{move || i18n.t(body)}</span>
        </button>
    };
    view! {
        <main class="screen sheet-screen">
            <h1>{move || i18n.t("power-title")}</h1>
            <div class="options">
                {choice("quit", "display", "power-quit", "power-quit-body", true)}
                {choice("sleep", "moon", "power-sleep", "power-sleep-body", false)}
                {choice("restart", "refresh", "power-restart", "power-restart-body", false)}
                {choice("shutdown", "power", "power-shutdown", "power-shutdown-body", false)}
            </div>
            <Show when=move || pending.get().is_some() && ctx.dialog_open.get()>
                <Confirm question=Signal::derive(move || i18n.t("power-sure")) danger=true
                    on_yes=Callback::new(move |_| {
                        let action = pending.get_untracked().unwrap_or("quit");
                        spawn_local(async move {
                            let _ = invoke::<Value>("power_action", json!({ "action": action })).await;
                        });
                    })/>
            </Show>
        </main>
    }
}

/// Panneau rapide : monte du bas, accessible partout et pendant un jeu.
#[component]
pub fn QuickPanel() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let closing = RwSignal::new(0_u8);
    let go = move |screen: Screen| {
        ctx.panel.set(false);
        ctx.push(screen);
    };
    let tile = move |icon: &'static str, label: &'static str, action: Callback<()>| view! {
        <button class="f qtile" on:click=move |_| action.run(())>
            <Icon name=icon/><span>{move || i18n.t(label)}</span>
        </button>
    };
    view! {
        <div class="scrim" on:click=move |_| ctx.back()></div>
        <aside class="panel sheet" class:side=move || ctx.running.with(Option::is_some)>
            <div class="sheet-head">
                <strong>{move || i18n.t("panel-title")}</strong>
                <StatusBar/>
            </div>
            {move || ctx.running.get().map(|entry| {
                let title = entry.title.clone();
                let social = entry.kind == EntryKind::Social;
                let question = title.clone();
                view! {
                    <div class="running">
                        <span class="running-title">{title}</span>
                        <button class="f play" data-auto="" on:click=move |_| {
                            ctx.panel.set(false);
                            spawn_local(async move {
                                let _ = invoke::<Value>("return_to_running", json!({})).await;
                            });
                        }><Icon name="play"/><span>{move || i18n.t("panel-return")}</span></button>
                        <button class="f pill danger" on:click=move |_| {
                            closing.set(1);
                            ctx.dialog_open.set(true);
                            after_render(nav::focus_first);
                        }><Icon name="close"/><span>{move || i18n.t("panel-close-entry")}</span></button>
                        <Show when=move || closing.get() != 0 && ctx.dialog_open.get()>
                            {
                                let question = question.clone();
                                view! {
                                    <div class="dialog"><div class="card">
                                        <h2>{i18n.ts("close-title", "title", &question)}</h2>
                                        <p>{move || i18n.t(if social { "close-social" } else { "close-warning" })}</p>
                                        <Show when=move || closing.get() == 2><p>{move || i18n.t("close-waiting")}</p></Show>
                                        <Show when=move || closing.get() == 3><p class="warn">{move || i18n.t("close-force-sure")}</p></Show>
                                        <div class="row">
                                            <button class="f" data-auto="" on:click=move |_| {
                                                closing.set(0);
                                                ctx.back();
                                            }>{move || i18n.t("close-no")}</button>
                                            {move || match closing.get() {
                                                1 => view! {
                                                    <button class="f" on:click=move |_| {
                                                        closing.set(2);
                                                        spawn_local(async move {
                                                            let _ = invoke::<Value>("close_running", json!({})).await;
                                                            // Beaucoup de jeux demandent « Quitter ? » : leur question
                                                            // serait cachée derrière Focus. On rend la main au jeu ; à
                                                            // sa fermeture, l'écran de fin revient seul.
                                                            crate::app::wait_ms(1200).await;
                                                            let still = invoke::<Option<crate::api::RunningEntry>>("running_entry", json!({}))
                                                                .await
                                                                .ok()
                                                                .flatten()
                                                                .is_some();
                                                            if still {
                                                                closing.set(0);
                                                                ctx.dialog_open.set(false);
                                                                ctx.panel.set(false);
                                                                let _ = invoke::<Value>("return_to_running", json!({})).await;
                                                            }
                                                        });
                                                    }>{move || i18n.t("close-yes")}</button>
                                                }.into_any(),
                                                2 => view! {
                                                    <button class="f danger" on:click=move |_| closing.set(3)>{move || i18n.t("close-force")}</button>
                                                }.into_any(),
                                                _ => view! {
                                                    <button class="f danger" on:click=move |_| {
                                                        spawn_local(async move {
                                                            let _ = invoke::<Value>("force_close_running", json!({})).await;
                                                        });
                                                    }>{move || i18n.t("close-force")}</button>
                                                }.into_any(),
                                            }}
                                        </div>
                                    </div></div>
                                }
                            }
                        </Show>
                    </div>
                }
            })}
            <div class="qtiles">
                <Show when=move || ctx.running.with(Option::is_some)>
                    {tile("display", "panel-screenshot", Callback::new(move |_| {
                        ctx.panel.set(false);
                        spawn_local(async move {
                            let _ = invoke::<Value>("return_to_running", json!({})).await;
                            crate::app::wait_ms(700).await;
                            let saved = invoke::<String>("screenshot", json!({})).await.is_ok();
                            ctx.toast.set(Some(if saved { "shot-saved" } else { "err-generic" }.to_owned()));
                        });
                    }))}
                </Show>
                {tile("library", "panel-library", Callback::new(move |_| { ctx.panel.set(false); ctx.home(); }))}
                {tile("wifi", "con-wifi", Callback::new(move |_| { ctx.con_tab.set("wifi"); go(Screen::Connections); }))}
                {tile("bluetooth", "con-bt", Callback::new(move |_| { ctx.con_tab.set("bluetooth"); go(Screen::Connections); }))}
                {tile("settings", "set-title", Callback::new(move |_| go(Screen::Settings)))}
                {tile("power", "power-title", Callback::new(move |_| go(Screen::Power)))}
            </div>
        </aside>
    }
}

/// Animation de lancement : la jaquette s'avance, puis la fenêtre laisse place au jeu.
#[component]
pub fn LaunchOverlay() -> impl IntoView {
    let ctx = ctx();
    move || {
        let id = ctx.launching.get()?;
        let item = ctx.entries.with(|list| list.iter().find(|item| item.entry.id == id).cloned())?;
        let background = css_url(item.art_url(ArtKind::Hero).or_else(|| item.art_url(ArtKind::Cover)));
        let front = item.art_url(ArtKind::Cover).or_else(|| item.art_url(ArtKind::Icon));
        Some(view! {
            <div class="launch" style:--accent=accent_of(&item)>
                <div class="launch-art" style:background-image=background></div>
                {front.map(|url| view! { <img class="launch-cover" src=url alt=""/> })}
                <p class="launch-title">{item.entry.title.clone()}</p>
                <p class="launch-status">{move || use_i18n().t("launch-starting")}</p>
                <span class="launch-bar"><span></span></span>
            </div>
        })
    }
}

/// Clavier à l'écran.
#[component]
pub fn TextOverlay() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    Effect::new(move |_| {
        if ctx.editing.read().is_some() {
            after_render(nav::focus_first);
        }
    });
    move || {
        ctx.editing.get().map(|edit| {
            let (value, secret) = (edit.value, edit.secret);
            let rows = keyboard::layout(i18n.lang.get()).iter().map(|row| {
                let keys = row.chars().map(|letter| view! {
                    <button class="f key" on:click=move |_| value.update(|text| text.push(letter))>{letter.to_string()}</button>
                }).collect_view();
                view! { <div class="keys">{keys}</div> }
            }).collect_view();
            view! {
                <section class="osk">
                    <div class="value">{move || {
                        let text = value.get();
                        if secret { "•".repeat(text.chars().count()) } else { text }
                    }}<span class="caret"></span></div>
                    {rows}
                    <div class="keys">
                        <button class="f key wide" on:click=move |_| value.update(|text| text.push(' '))>{move || i18n.t("kbd-space")}</button>
                        <button class="f key wide" on:click=move |_| value.update(|text| { text.pop(); })>{move || i18n.t("kbd-delete")}</button>
                        <button class="f key wide primary" data-auto="" on:click=move |_| ctx.back()>{move || i18n.t("kbd-done")}</button>
                    </div>
                </section>
            }
        })
    }
}

/// Message temporaire. Une clé de traduction connue est traduite.
#[component]
pub fn Toast() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    move || {
        ctx.toast.get().map(|text| {
            let shown = if text.contains(' ') { text } else { i18n.t(&text) };
            view! { <div class="toast" role="status">{shown}</div> }
        })
    }
}

/// Barre d'aide : les symboles suivent le dernier périphérique et la manette.
#[component]
pub fn Hints() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let glyph = move |slot: usize| -> String {
        let style = ctx
            .settings
            .with(|s| s.as_ref().map_or("auto".to_owned(), |s| s.glyphs.clone()));
        let shapes = style == "playStation" || (style == "auto" && ctx.family.get() == "playStation");
        match ctx.device.get() {
            Device::Keyboard => ["↵", "F2", "Esc", "F1"][slot].to_owned(),
            Device::Mouse => String::new(),
            Device::Pad if shapes => ["✕", "△", "○", "≡"][slot].to_owned(),
            Device::Pad => ["A", "Y", "B", "≡"][slot].to_owned(),
        }
    };
    let hint = move |slot: usize, key: &'static str| {
        view! {
            <span class="hint"><span class="glyph">{move || glyph(slot)}</span>{move || i18n.t(key)}</span>
        }
    };
    let home = move || ctx.screens.with(|list| list.last() == Some(&Screen::Home));
    view! {
        <Show when=move || ctx.device.get() != Device::Mouse>
            <footer class="hints">
                {hint(0, "hint-select")}
                <Show when=home>{hint(1, "hint-options")}</Show>
                {hint(2, "hint-back")}
                {hint(3, "hint-panel")}
            </footer>
        </Show>
    }
}
