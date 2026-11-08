//! Écrans de l'interface. Aucun texte en dur : tout passe par les traductions.

use crate::api::{
    self, invoke, CatalogItem, ControllerStatus, DirItem, Merged, Preview, SystemReport,
};
use crate::app::{after_render, Ctx, Device, Screen, TextEdit};
use crate::i18n::{use_i18n, LANGS};
use crate::keyboard;
use crate::nav;
use focus_core::model::{EntryKind, Store};
use focus_core::plan::Step;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{json, Value};

fn ctx() -> Ctx {
    use_context::<Ctx>().unwrap_or_else(|| unreachable!("Ctx is provided by App"))
}

fn hue(title: &str) -> u32 {
    title
        .chars()
        .fold(7_u32, |sum, c| sum.wrapping_mul(31).wrapping_add(c as u32))
        % 360
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

/// Entrées du filtre courant.
pub fn visible_entries(ctx: Ctx) -> Vec<Merged> {
    let filter = ctx.filter.get();
    ctx.entries
        .get()
        .into_iter()
        .filter(|item| filter.is_none_or(|kind| item.entry.kind == kind))
        .collect()
}

#[component]
pub fn CurrentScreen() -> impl IntoView {
    let ctx = ctx();
    move || match ctx.screens.with(|list| list.last().cloned()) {
        Some(Screen::Language) => view! { <LanguageScreen/> }.into_any(),
        Some(Screen::Welcome) => view! { <WelcomeScreen/> }.into_any(),
        Some(Screen::Home) => view! { <HomeScreen/> }.into_any(),
        Some(Screen::Detail(id)) => view! { <DetailScreen id=id/> }.into_any(),
        Some(Screen::Prepare(id)) => view! { <PrepareScreen id=id/> }.into_any(),
        Some(Screen::Ended(end)) => view! { <EndedScreen end=end/> }.into_any(),
        Some(Screen::Settings) => view! { <SettingsScreen/> }.into_any(),
        Some(Screen::Analysis) => view! { <AnalysisScreen/> }.into_any(),
        Some(Screen::AddFile) => view! { <AddFileScreen/> }.into_any(),
        Some(Screen::Stores) => view! { <StoresScreen/> }.into_any(),
        Some(Screen::Power) => view! { <PowerScreen/> }.into_any(),
        None => ().into_any(),
    }
}

#[component]
fn LanguageScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let items = LANGS
        .iter()
        .map(|lang| {
            let auto = lang.code == i18n.lang.get_untracked();
            view! {
                <button class="f" lang=lang.code data-auto=auto.then_some("") on:click=move |_| {
                    i18n.lang.set(lang.code);
                    ctx.update_settings(|s| s.language = Some(lang.code.to_owned()));
                    ctx.replace(Screen::Welcome);
                }>{lang.name}</button>
            }
        })
        .collect_view();
    view! {
        <main class="screen">
            <h1>{move || i18n.t("lang-title")}</h1>
            <div class="lang-grid">{items}</div>
        </main>
    }
}

#[component]
fn WelcomeScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let autostart = RwSignal::new(true);
    view! {
        <main class="screen">
            <h1>{move || i18n.t("welcome-title")}</h1>
            <p>{move || i18n.t("welcome-body")}</p>
            <p>{move || i18n.t("welcome-sim")}</p>
            <div class="stack">
                <Toggle label="welcome-autostart" value=Signal::derive(move || autostart.get())
                    on_change=Callback::new(move |_| autostart.update(|on| *on = !*on))/>
                <button class="f primary" data-auto="" on:click=move |_| {
                    let enabled = autostart.get_untracked();
                    spawn_local(async move {
                        let _ = invoke::<Value>("set_autostart", json!({ "enabled": enabled })).await;
                    });
                    ctx.update_settings(|s| s.first_run_done = true);
                    ctx.home();
                }>{move || i18n.t("welcome-continue")}</button>
            </div>
        </main>
    }
}

#[component]
fn Toggle(label: &'static str, value: Signal<bool>, on_change: Callback<()>) -> impl IntoView {
    let i18n = use_i18n();
    view! {
        <button class="f" aria-pressed=move || value.get().to_string() on:click=move |_| on_change.run(())>
            <span>{move || i18n.t(label)}</span>" · "
            <span class="muted">{move || i18n.t(if value.get() { "state-on" } else { "state-off" })}</span>
        </button>
    }
}

#[component]
fn HomeScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let tabs = [
        (None, "filter-all"),
        (Some(EntryKind::Game), "filter-games"),
        (Some(EntryKind::Software), "filter-software"),
        (Some(EntryKind::Social), "filter-social"),
    ];
    let tab_views = tabs
        .into_iter()
        .map(|(kind, label)| {
            view! {
                <button class="f tab" class:active=move || ctx.filter.get() == kind on:click=move |_| {
                    ctx.filter.set(kind);
                    ctx.selected.set(0);
                }>{move || i18n.t(label)}</button>
            }
        })
        .collect_view();
    let clock = RwSignal::new(String::new());
    let tick = move || {
        let now = js_sys::Date::new_0();
        clock.set(format!("{:02}:{:02}", now.get_hours(), now.get_minutes()));
    };
    tick();
    let _ = set_interval_with_handle(tick, std::time::Duration::from_secs(20));
    let selected_entry = move || {
        let list = visible_entries(ctx);
        let index = ctx.selected.get().clamp(0, (list.len() as i32 - 1).max(0));
        list.get(index as usize).cloned()
    };
    let cards = move || {
        let list = visible_entries(ctx);
        let selected = ctx.selected.get().clamp(0, (list.len() as i32 - 1).max(0));
        list.into_iter()
            .enumerate()
            .filter(|(index, _)| (*index as i32 - selected).abs() <= 6)
            .map(|(index, item)| {
                let offset = index as i32 - selected;
                // Position sur l'arc : rayon de 110 % de la hauteur d'écran, centre sous l'écran.
                let angle = offset as f64 * 11.0;
                let radius = 110.0;
                let x = angle.to_radians().sin() * radius;
                let y = (1.0 - angle.to_radians().cos()) * radius;
                let scale = if offset == 0 {
                    1.2
                } else {
                    0.92 - f64::from(offset.abs()) * 0.03
                };
                let opacity = 1.0 - f64::from(offset.abs()) * 0.13;
                let style = format!(
                    "--x:{x:.2}vh;--y:{y:.2}vh;--a:{angle}deg;--s:{scale};--o:{opacity};--h:{};z-index:{}",
                    hue(&item.entry.title),
                    20 - offset.abs()
                );
                let id = item.entry.id.clone();
                let title = item.entry.title;
                view! {
                    <button class="f card" style=style data-card=index data-auto=(offset == 0).then_some("")
                        tabindex=if offset == 0 { "0" } else { "-1" }
                        on:click=move |_| {
                            if offset == 0 {
                                ctx.push(Screen::Detail(id.clone()));
                            } else {
                                ctx.selected.set(index as i32);
                                after_render(nav::focus_first);
                            }
                        }>
                        <span>{title}</span>
                    </button>
                }
            })
            .collect_view()
    };
    let background = move || {
        selected_entry()
            .map_or(32, |item| hue(&item.entry.title))
            .to_string()
    };
    view! {
        <main class="screen home" style:--hue=background>
            <nav class="topbar">
                {tab_views}
                <span class="spacer"></span>
                <span class="clock">{move || clock.get()}</span>
                <button class="f" on:click=move |_| ctx.push(Screen::AddFile)>{move || i18n.t("top-add")}</button>
                <button class="f" on:click=move |_| ctx.push(Screen::Stores)>{move || i18n.t("top-stores")}</button>
                <button class="f" on:click=move |_| ctx.push(Screen::Settings)>{move || i18n.t("top-settings")}</button>
                <button class="f" on:click=move |_| ctx.push(Screen::Power)>{move || i18n.t("top-power")}</button>
            </nav>
            {move || match selected_entry() {
                Some(item) => {
                    let hours = item.entry.stats.total_seconds / 3600;
                    let resume = ctx.selected.get() == 0 && item.entry.stats.last_played.is_some();
                    view! {
                        <section class="hero">
                            <Show when=move || resume><span class="badge">{move || i18n.t("home-resume")}</span></Show>
                            <h1>{item.entry.title.clone()}</h1>
                            <p>{store_name(item.entry.store)}" · "{move || i18n.tn("home-played", "hours", hours)}</p>
                        </section>
                    }.into_any()
                }
                None => view! {
                    <section class="empty">
                        <h1>{move || i18n.t("home-empty-title")}</h1>
                        <p>{move || i18n.t("home-empty-body")}</p>
                    </section>
                }.into_any(),
            }}
            <div class="stage"><div class="iris"></div>{cards}</div>
        </main>
    }
}

#[component]
fn DetailScreen(id: String) -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let lookup = id.clone();
    let item = Memo::new(move |_| {
        ctx.entries
            .get()
            .into_iter()
            .find(|item| item.entry.id == lookup)
    });
    let (play_id, fav_id, del_id) = (id.clone(), id.clone(), id);
    let favorite = move || item.get().is_some_and(|item| item.entry.favorite);
    view! {
        <main class="screen">
            {move || item.get().map(|item| {
                let entry = item.entry;
                let hours = entry.stats.total_seconds / 3600;
                let launches = u64::from(entry.stats.launches);
                let store = store_name(entry.store);
                view! {
                    <h1>{entry.title}</h1>
                    <p>{move || i18n.ts("detail-source", "store", store)}" · "
                       {move || i18n.tn("home-played", "hours", hours)}" · "
                       {move || i18n.tn("detail-launches", "count", launches)}</p>
                }
            })}
            <div class="stack">
                <button class="f primary" data-auto="" on:click=move |_| ctx.push(Screen::Prepare(play_id.clone()))>
                    {move || {
                        let game = item.get().is_some_and(|i| i.entry.kind == EntryKind::Game);
                        i18n.t(if game { "detail-play" } else { "detail-open" })
                    }}
                </button>
                <button class="f" on:click=move |_| {
                    let (id, value) = (fav_id.clone(), !favorite());
                    spawn_local(async move {
                        let _ = invoke::<Value>("set_favorite", json!({ "id": id, "favorite": value })).await;
                        ctx.reload();
                    });
                }>{move || i18n.t(if favorite() { "detail-unfavorite" } else { "detail-favorite" })}</button>
                <button class="f danger" on:click=move |_| {
                    let id = del_id.clone();
                    spawn_local(async move {
                        let _ = invoke::<Value>("remove_entry", json!({ "id": id })).await;
                        ctx.reload();
                        ctx.home();
                    });
                }>{move || i18n.t("detail-remove")}</button>
            </div>
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
        preview.set(
            invoke("preview_session", json!({ "id": request }))
                .await
                .ok(),
        );
        after_render(nav::focus_first);
    });
    view! {
        <main class="screen">
            <h1>{move || i18n.t("prep-title")}</h1>
            {move || preview.get().map(|preview| {
                let steps = preview.plan.steps.clone();
                let empty = steps.is_empty();
                let lines = steps.into_iter()
                    .map(|step| view! { <li>{move || step_text(i18n, &step)}</li> })
                    .collect_view();
                let asks = preview.plan.ask_user.into_iter()
                    .map(|ask| view! { <li>{move || i18n.ts("prep-ask", "name", &ask.name)}</li> })
                    .collect_view();
                view! {
                    <p>{move || i18n.t(if preview.simulation { "prep-simulation" } else { "prep-real" })}</p>
                    <Show when=move || !preview.guardian && !preview.simulation>
                        <p class="warn">{move || i18n.t("prep-no-guardian")}</p>
                    </Show>
                    <Show when=move || empty><p>{move || i18n.t("prep-nothing")}</p></Show>
                    <ul>{lines}{asks}</ul>
                }
            })}
            <div class="row">
                <button class="f primary" data-auto="" on:click=move |_| ctx.launch(id.clone())>
                    {move || i18n.t("prep-start")}
                </button>
                <button class="f" on:click=move |_| ctx.back()>{move || i18n.t("prep-cancel")}</button>
            </div>
        </main>
    }
}

#[component]
fn EndedScreen(end: api::SessionEnd) -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let minutes = end.seconds / 60;
    let restored = end.restored;
    view! {
        <main class="screen">
            <h1>{move || i18n.t("end-title")}</h1>
            <h2>{end.title}</h2>
            <p>{move || i18n.tn("end-time", "minutes", minutes)}</p>
            <p class:ok=restored class:warn=!restored>
                {move || i18n.t(if restored { "end-restored" } else { "end-problem" })}
            </p>
            <div class="row">
                <button class="f primary" data-auto="" on:click=move |_| ctx.home()>{move || i18n.t("end-back")}</button>
            </div>
        </main>
    }
}

#[component]
fn SettingsScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let autostart = RwSignal::new(false);
    let guardian = RwSignal::new(false);
    let status = RwSignal::new(None::<ControllerStatus>);
    let confirm_real = RwSignal::new(false);
    let detect = move || {
        spawn_local(async move {
            status.set(invoke("controller_status", json!({})).await.ok());
        });
    };
    detect();
    spawn_local(async move {
        autostart.set(
            invoke("set_autostart", json!({ "enabled": null }))
                .await
                .unwrap_or(false),
        );
        guardian.set(invoke("guardian_status", json!({})).await.unwrap_or(false));
    });
    let setting = move |read: fn(&api::Settings) -> bool| {
        Signal::derive(move || ctx.settings.with(|s| s.as_ref().is_some_and(read)))
    };
    macro_rules! toggle {
        ($label:literal, $field:ident) => {
            view! { <Toggle label=$label value=setting(|s| s.$field)
            on_change=Callback::new(move |_| ctx.update_settings(|s| s.$field = !s.$field))/> }
        };
    }
    let lang_buttons = LANGS.iter().map(|lang| view! {
        <button class="f" lang=lang.code aria-pressed=move || (i18n.lang.get() == lang.code).to_string()
            on:click=move |_| {
                i18n.lang.set(lang.code);
                ctx.update_settings(|s| s.language = Some(lang.code.to_owned()));
            }>{lang.name}</button>
    }).collect_view();
    let glyphs = ["auto", "playStation", "xbox"].map(|style| view! {
        <button class="f" aria-pressed=move || ctx.settings.with(|s| s.as_ref().is_some_and(|s| s.glyphs == style)).to_string()
            on:click=move |_| ctx.update_settings(|s| s.glyphs = style.to_owned())>
            {move || i18n.t(&format!("glyph-{style}"))}
        </button>
    }).into_iter().collect_view();
    view! {
        <main class="screen">
            <h1>{move || i18n.t("set-title")}</h1>
            <div class="stack">
                <Toggle label="set-real" value=setting(|s| s.real_optimization)
                    on_change=Callback::new(move |_| {
                        if ctx.settings.with_untracked(|s| s.as_ref().is_some_and(|s| s.real_optimization)) {
                            ctx.update_settings(|s| s.real_optimization = false);
                        } else {
                            confirm_real.set(true);
                            ctx.dialog_open.set(true);
                            after_render(nav::focus_first);
                        }
                    })/>
                <p>{move || i18n.t("set-real-body")}</p>
                <Toggle label="set-autostart" value=Signal::derive(move || autostart.get())
                    on_change=Callback::new(move |_| {
                        let wanted = !autostart.get_untracked();
                        spawn_local(async move {
                            autostart.set(invoke("set_autostart", json!({ "enabled": wanted })).await.unwrap_or(false));
                        });
                    })/>
                {toggle!("set-keep-social", keep_social)}
                {toggle!("set-awake", keep_awake)}
                {toggle!("set-high-perf", high_performance)}
                {toggle!("set-big-picture", block_big_picture)}
                {toggle!("set-sounds", sounds)}
                {toggle!("set-motion", reduce_motion)}
                {toggle!("set-theme", auto_theme)}
                <h2>{move || i18n.t("set-glyphs")}</h2>
                <div class="row">{glyphs}</div>
                <h2>{move || i18n.t("set-detect")}</h2>
                <button class="f" on:click=move |_| detect()>{move || i18n.t("set-detect")}</button>
                {move || status.get().map(|status| {
                    let family = i18n.t(&format!("family-{}", status.family));
                    let mode = i18n.t(&format!("mode-{}", status.mode));
                    view! {
                        <p>{i18n.ts("ctl-family", "family", &family)}" · "{i18n.ts("ctl-mode", "mode", &mode)}</p>
                    }
                })}
                <h2>{move || i18n.t("set-guardian")}</h2>
                <p>{move || i18n.t("set-guardian-body")}</p>
                <button class="f" on:click=move |_| {
                    spawn_local(async move {
                        let _ = invoke::<Value>("install_guardian", json!({})).await;
                    });
                }>
                    {move || i18n.t(if guardian.get() { "set-guardian-on" } else { "set-guardian-install" })}
                </button>
                <button class="f" on:click=move |_| ctx.push(Screen::Analysis)>{move || i18n.t("set-analysis")}</button>
                <h2>{move || i18n.t("set-language")}</h2>
                <p>{move || {
                    let reviewed = i18n.current().is_some_and(|lang| lang.reviewed);
                    let status = i18n.t(if reviewed { "status-reviewed" } else { "status-draft" });
                    i18n.ts("set-lang-status", "status", &status)
                }}</p>
                <div class="lang-grid">{lang_buttons}</div>
            </div>
            <Show when=move || confirm_real.get() && ctx.dialog_open.get()>
                <div class="dialog"><div>
                    <h2>{move || i18n.t("set-real-confirm")}</h2>
                    <p>{move || i18n.t("set-real-body")}</p>
                    <div class="row">
                        <button class="f" data-auto="" on:click=move |_| {
                            confirm_real.set(false);
                            ctx.back();
                        }>{move || i18n.t("no")}</button>
                        <button class="f danger" on:click=move |_| {
                            confirm_real.set(false);
                            ctx.update_settings(|s| s.real_optimization = true);
                            ctx.back();
                        }>{move || i18n.t("yes")}</button>
                    </div>
                </div></div>
            </Show>
        </main>
    }
}

#[component]
fn AnalysisScreen() -> impl IntoView {
    let i18n = use_i18n();
    let report = RwSignal::new(None::<SystemReport>);
    spawn_local(async move {
        report.set(invoke("system_report", json!({})).await.ok().flatten());
    });
    let back_ctx = ctx();
    view! {
        <main class="screen">
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
                        <ul>{top}</ul>
                    }.into_any()
                }
            }}
            <div class="row">
                <button class="f primary" data-auto="" on:click=move |_| back_ctx.back()>{move || i18n.t("hint-back")}</button>
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
    let name = path.rsplit('\\').next().unwrap_or(path);
    name.rsplit_once('.')
        .map_or(name, |(stem, _)| stem)
        .to_owned()
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
        <button class="f" aria-pressed=move || (kind.get() == value).to_string() on:click=move |_| kind.set(value)>
            {move || i18n.t(label)}
        </button>
    }).into_iter().collect_view();
    view! {
        <main class="screen">
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
                            <button class="f" data-auto="" on:click=move |_| ctx.editing.set(Some(TextEdit { value: title }))>
                                {move || format!("{} · {}", i18n.t("add-name"), title.get())}
                            </button>
                            <h2>{move || i18n.t("add-kind")}</h2>
                            <div class="row">{kinds.clone()}</div>
                            <button class="f primary" on:click=move |_| {
                                let (path, name, value) = (path.clone(), title.get_untracked(), kind.get_untracked());
                                spawn_local(async move {
                                    let _ = invoke::<Value>("add_file", json!({ "path": path, "title": name, "kind": value })).await;
                                    ctx.reload();
                                    ctx.home();
                                });
                            }>{move || i18n.t("add-save")}</button>
                        </div>
                    }
                }>
                <p>{move || i18n.t("add-body")}</p>
                <p class="mono">{move || folder.get().unwrap_or_default()}</p>
                <div class="row">
                    <Show when=move || folder.get().is_some()>
                        <button class="f" on:click=move |_| open(folder.get_untracked().and_then(|p| parent_of(&p)))>
                            {move || i18n.t("add-up")}
                        </button>
                        <button class="f" on:click=move |_| {
                            let path = folder.get_untracked().unwrap_or_default();
                            spawn_local(async move {
                                suggested.set(invoke("suggest_executables", json!({ "path": path })).await.unwrap_or_default());
                                after_render(nav::focus_first);
                            });
                        }>{move || i18n.t("add-suggest")}</button>
                    </Show>
                </div>
                <Show when=move || !suggested.get().is_empty()>
                    <h2>{move || i18n.t("add-suggested")}</h2>
                    <div class="list">
                        {move || suggested.get().into_iter().map(|path| {
                            let (shown, value) = (path.clone(), path);
                            view! { <button class="f mono" on:click=move |_| choose(value.clone())>{shown}</button> }
                        }).collect_view()}
                    </div>
                </Show>
                <div class="list">
                    {move || items.get().into_iter().map(|item| {
                        let DirItem { name, path, is_dir } = item;
                        view! {
                            <button class="f" on:click=move |_| {
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
        <main class="screen">
            <h1>{move || i18n.t("store-title")}</h1>
            <div class="grid">
                {move || apps.get().into_iter().map(|app| {
                    let (name, page_id, winget) = (app.name.clone(), app.id.clone(), app.winget);
                    let install = app.clone();
                    view! {
                        <div class="stack">
                            <strong>{name}</strong>
                            <span class="muted">{app.publisher}</span>
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
                            <button class="f" on:click=move |_| {
                                let id = page_id.clone();
                                spawn_local(async move {
                                    let _ = invoke::<Value>("open_official_page", json!({ "id": id })).await;
                                });
                            }>{move || i18n.t("store-page")}</button>
                        </div>
                    }
                }).collect_view()}
            </div>
            <Show when=move || asking.get().is_some() && ctx.dialog_open.get()>
                {move || asking.get().map(|app| {
                    let question = i18n.ts2("store-confirm", ("name", &app.name), ("publisher", &app.publisher));
                    let (id, name) = (app.id, app.name);
                    view! {
                        <div class="dialog"><div>
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
    view! {
        <main class="screen">
            <h1>{move || i18n.t("power-title")}</h1>
            <div class="stack">
                <button class="f" data-auto="" on:click=move |_| ask("quit")>{move || i18n.t("power-quit")}</button>
                <button class="f" on:click=move |_| ask("restart")>{move || i18n.t("power-restart")}</button>
                <button class="f" on:click=move |_| ask("shutdown")>{move || i18n.t("power-shutdown")}</button>
            </div>
            <Show when=move || pending.get().is_some() && ctx.dialog_open.get()>
                <div class="dialog"><div>
                    <h2>{move || i18n.t("power-sure")}</h2>
                    <div class="row">
                        <button class="f" data-auto="" on:click=move |_| ctx.back()>{move || i18n.t("no")}</button>
                        <button class="f danger" on:click=move |_| {
                            let action = pending.get_untracked().unwrap_or("quit");
                            spawn_local(async move {
                                let _ = invoke::<Value>("power_action", json!({ "action": action })).await;
                            });
                        }>{move || i18n.t("yes")}</button>
                    </div>
                </div></div>
            </Show>
        </main>
    }
}

/// Panneau rapide : accessible du menu ou par le combo pendant un jeu.
#[component]
pub fn QuickPanel() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let closing = RwSignal::new(0_u8);
    let running = move || ctx.running.get();
    view! {
        <div class="scrim" on:click=move |_| ctx.back()></div>
        <aside class="panel">
            <h2>{move || i18n.t("panel-title")}</h2>
            {move || match running() {
                None => view! { <p>{move || i18n.t("panel-none")}</p> }.into_any(),
                Some(entry) => {
                    let title = entry.title.clone();
                    let social = entry.kind == EntryKind::Social;
                    let question = title.clone();
                    view! {
                        <strong>{title}</strong>
                        <button class="f primary" data-auto="" on:click=move |_| {
                            ctx.panel.set(false);
                            spawn_local(async move {
                                let _ = invoke::<Value>("return_to_running", json!({})).await;
                            });
                        }>{move || i18n.t("panel-return")}</button>
                        <button class="f danger" on:click=move |_| {
                            closing.set(1);
                            ctx.dialog_open.set(true);
                            after_render(nav::focus_first);
                        }>{move || i18n.t("panel-close-entry")}</button>
                        <Show when=move || closing.get() != 0 && ctx.dialog_open.get()>
                            {
                                let question = question.clone();
                                view! {
                                    <div class="dialog"><div>
                                        <h2>{i18n.ts("close-title", "title", &question)}</h2>
                                        <p>{move || i18n.t(if social { "close-social" } else { "close-warning" })}</p>
                                        <Show when=move || closing.get() == 2>
                                            <p>{move || i18n.t("close-waiting")}</p>
                                        </Show>
                                        <Show when=move || closing.get() == 3>
                                            <p class="warn">{move || i18n.t("close-force-sure")}</p>
                                        </Show>
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
                                                        });
                                                    }>{move || i18n.t("close-yes")}</button>
                                                }.into_any(),
                                                2 => view! {
                                                    <button class="f danger" on:click=move |_| closing.set(3)>
                                                        {move || i18n.t("close-force")}
                                                    </button>
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
                    }.into_any()
                }
            }}
            <button class="f" on:click=move |_| { ctx.panel.set(false); ctx.home(); }>{move || i18n.t("panel-library")}</button>
            <button class="f" on:click=move |_| { ctx.panel.set(false); ctx.push(Screen::Settings); }>{move || i18n.t("set-title")}</button>
            <button class="f" on:click=move |_| { ctx.panel.set(false); ctx.push(Screen::Power); }>{move || i18n.t("power-title")}</button>
        </aside>
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
        let value = edit.value;
        let rows = keyboard::layout(i18n.lang.get()).iter().map(|row| {
            let keys = row.chars().map(|letter| view! {
                <button class="f key" on:click=move |_| value.update(|text| text.push(letter))>{letter.to_string()}</button>
            }).collect_view();
            view! { <div class="keys">{keys}</div> }
        }).collect_view();
        view! {
            <section class="osk">
                <div class="value">{move || value.get()}</div>
                {rows}
                <div class="keys">
                    <button class="f key" on:click=move |_| value.update(|text| text.push(' '))>{move || i18n.t("kbd-space")}</button>
                    <button class="f key" on:click=move |_| value.update(|text| { text.pop(); })>{move || i18n.t("kbd-delete")}</button>
                    <button class="f key primary" data-auto="" on:click=move |_| ctx.back()>{move || i18n.t("kbd-done")}</button>
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
            let shown = if text.contains(' ') {
                text
            } else {
                i18n.t(&text)
            };
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
        let family = ctx.family.get();
        let shapes = style == "playStation" || (style == "auto" && family == "playStation");
        match ctx.device.get() {
            Device::Keyboard => ["↵", "Esc", "F1"][slot].to_owned(),
            Device::Mouse => String::new(),
            Device::Pad if shapes => ["✕", "○", "⌂"][slot].to_owned(),
            Device::Pad => ["A", "B", "⌂"][slot].to_owned(),
        }
    };
    let hint = move |slot: usize, key: &'static str| {
        view! {
            <span class="hint">
                <span class="glyph">{move || glyph(slot)}</span>
                {move || i18n.t(key)}
            </span>
        }
    };
    view! {
        <Show when=move || ctx.device.get() != Device::Mouse>
            <footer class="hints">{hint(0, "hint-select")}{hint(1, "hint-back")}{hint(2, "hint-panel")}</footer>
        </Show>
    }
}
