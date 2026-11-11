//! Assistant du premier démarrage : l'utilisateur règle son launcher, étape par
//! étape, à la manette, au clavier ou à la souris.

use crate::api::invoke;
use crate::app::{ctx, Ctx, Screen, SETUP_STEPS};
use crate::i18n::{use_i18n, LANGS};
use crate::icons::Icon;
use crate::screens::{accent_of, OptimizationChoice, Toggle};
use focus_core::art::ArtKind;
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{json, Value};

/// Fin de l'assistant : réglages enregistrés, puis Windows passe en mode console.
fn finish(ctx: Ctx, lang: &'static str) {
    ctx.settings.update(|settings| {
        if let Some(settings) = settings {
            settings.first_run_done = true;
            settings.language.get_or_insert_with(|| lang.to_owned());
        }
    });
    let settings = ctx.settings.get_untracked();
    let autostart = ctx.setup_autostart.get_untracked();
    spawn_local(async move {
        // Dans l'ordre : le cœur doit connaître les réglages avant d'appliquer le mode console.
        let _ = invoke::<Value>("set_settings", json!({ "settings": settings })).await;
        let _ = invoke::<Value>("set_autostart", json!({ "enabled": autostart })).await;
        let _ = invoke::<Value>("apply_console", json!({})).await;
    });
    ctx.home();
}

fn languages(ctx: Ctx) -> AnyView {
    let i18n = use_i18n();
    let buttons = LANGS
        .iter()
        .map(|lang| {
            let auto = lang.code == i18n.lang.get_untracked();
            view! {
                <button class="f chip-button" lang=lang.code data-auto=auto.then_some("")
                    aria-pressed=move || (i18n.lang.get() == lang.code).to_string()
                    on:click=move |_| {
                        i18n.lang.set(lang.code);
                        ctx.update_settings(|s| s.language = Some(lang.code.to_owned()));
                        ctx.replace(Screen::Setup(1));
                    }>{lang.name}</button>
            }
        })
        .collect_view();
    view! { <div class="lang-grid">{buttons}</div> }.into_any()
}

fn console_step(ctx: Ctx) -> AnyView {
    view! {
        <div class="stack">
            {crate::setting_toggle!(ctx, "set-console", console_mode)}
            {crate::setting_toggle!(ctx, "set-hide-taskbar", hide_taskbar)}
            {crate::setting_toggle!(ctx, "set-silence", silence_notifications)}
        </div>
    }
    .into_any()
}

fn pad_step(ctx: Ctx) -> AnyView {
    let i18n = use_i18n();
    let glyphs = ["auto", "playStation", "xbox"]
        .map(|style| {
            view! {
                <button class="f chip-button"
                    aria-pressed=move || ctx.settings.with(|s| s.as_ref().is_some_and(|s| s.glyphs == style)).to_string()
                    on:click=move |_| ctx.update_settings(|s| s.glyphs = style.to_owned())>
                    {move || i18n.t(&format!("glyph-{style}"))}
                </button>
            }
        })
        .into_iter()
        .collect_view();
    view! {
        <div class="stack">
            <p class="status-line"><Icon name="gamepad"/>{move || {
                let family = i18n.t(&format!("family-{}", ctx.family.get()));
                i18n.ts("ctl-family", "family", &family)
            }}</p>
            <h2>{move || i18n.t("set-glyphs")}</h2>
            <div class="row">{glyphs}</div>
            {crate::setting_toggle!(ctx, "set-vibration", vibration)}
        </div>
    }
    .into_any()
}

fn net_step(ctx: Ctx) -> AnyView {
    let i18n = use_i18n();
    view! {
        <div class="stack">
            <p class="status-line">{move || {
                let net = ctx.net.get();
                match net.kind.as_str() {
                    "wifi" => i18n.ts("con-connected-to", "name", &net.ssid.unwrap_or_default()),
                    "ethernet" => i18n.t("con-ethernet"),
                    _ => i18n.t("con-offline"),
                }
            }}</p>
            <div class="row">
                <button class="f pill" on:click=move |_| {
                    ctx.con_tab.set("wifi");
                    ctx.push(Screen::Connections);
                }><Icon name="wifi" level=4/><span>{move || i18n.t("setup-open-wifi")}</span></button>
                <button class="f pill" on:click=move |_| {
                    ctx.con_tab.set("bluetooth");
                    ctx.push(Screen::Connections);
                }><Icon name="bluetooth"/><span>{move || i18n.t("setup-open-bt")}</span></button>
            </div>
        </div>
    }
    .into_any()
}

fn library_step(ctx: Ctx) -> AnyView {
    let i18n = use_i18n();
    let preview = move || {
        ctx.entries
            .get()
            .into_iter()
            .take(8)
            .map(|item| {
                let url = item.art_url(ArtKind::Cover).or_else(|| item.art_url(ArtKind::Icon));
                let title = item.entry.title.clone();
                view! {
                    <span class="mini-tile" style:--tile-accent=accent_of(&item)>
                        {match url {
                            Some(url) => view! { <img src=url alt="" decoding="async"/> }.into_any(),
                            None => view! { <span class="name">{title}</span> }.into_any(),
                        }}
                    </span>
                }
            })
            .collect_view()
    };
    view! {
        <div class="stack">
            <p>{move || i18n.tn("setup-lib-body", "count", ctx.entries.with(Vec::len) as u64)}</p>
            <div class="mini-shelf">{preview}</div>
            <div class="row">
                <button class="f pill" on:click=move |_| ctx.push(Screen::AddFile)>
                    <Icon name="add"/><span>{move || i18n.t("top-add")}</span>
                </button>
            </div>
        </div>
    }
    .into_any()
}

fn comfort_step(ctx: Ctx) -> AnyView {
    view! {
        <div class="stack">
            {crate::setting_toggle!(ctx, "set-sounds", sounds)}
            {crate::setting_toggle!(ctx, "set-music", music)}
            {crate::setting_toggle!(ctx, "set-motion", reduce_motion)}
            <Toggle label="set-autostart" value=Signal::derive(move || ctx.setup_autostart.get())
                on_change=Callback::new(move |_| ctx.setup_autostart.update(|on| *on = !*on))/>
        </div>
    }
    .into_any()
}

fn done_step(ctx: Ctx) -> AnyView {
    let i18n = use_i18n();
    view! {
        <ul class="summary">
            <li><Icon name="display"/>{move || {
                let on = ctx.settings.with(|s| s.as_ref().is_some_and(|s| s.console_mode));
                format!("{} · {}", i18n.t("set-section-console"), i18n.t(if on { "state-on" } else { "state-off" }))
            }}</li>
            <li><Icon name="bolt"/>{move || {
                let level = ctx.settings.with(|s| s.as_ref().map(|s| s.optimization.clone()).unwrap_or_default());
                format!("{} · {}", i18n.t("set-section-opt"), i18n.t(&format!("opt-{level}")))
            }}</li>
        </ul>
    }
    .into_any()
}

#[component]
pub fn SetupScreen(step: u8) -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let last = step + 1 >= SETUP_STEPS;
    let (title, body) = match step {
        0 => ("lang-title", None),
        1 => ("setup-console-title", Some("setup-console-body")),
        2 => ("setup-pad-title", Some("setup-pad-body")),
        3 => ("setup-net-title", Some("setup-net-body")),
        4 => ("setup-lib-title", None),
        5 => ("setup-opt-title", Some("setup-opt-body")),
        6 => ("setup-comfort-title", Some("setup-comfort-body")),
        _ => ("setup-done-title", Some("setup-done-body")),
    };
    let content = match step {
        0 => languages(ctx),
        1 => console_step(ctx),
        2 => pad_step(ctx),
        3 => net_step(ctx),
        4 => library_step(ctx),
        5 => view! { <OptimizationChoice/> }.into_any(),
        6 => comfort_step(ctx),
        _ => done_step(ctx),
    };
    // La langue et le choix d'optimisation proposent eux-mêmes un bouton par défaut.
    let next_is_default = !matches!(step, 0 | 5);
    let dots = (0..SETUP_STEPS)
        .map(|index| view! { <span class="dot" class:on={index <= step}></span> })
        .collect_view();
    view! {
        <main class="screen setup">
            <div class="setup-head">
                <span class="brand">{move || i18n.t("app-name")}</span>
                <div class="dots" aria-label=move || i18n.tn("setup-step", "current", u64::from(step) + 1)>{dots}</div>
            </div>
            <section class="setup-body">
                <h1>{move || i18n.t(title)}</h1>
                {body.map(|key| view! { <p class="lead">{move || i18n.t(key)}</p> })}
                {content}
            </section>
            <div class="setup-nav">
                <Show when=move || step != 0>
                    <button class="f pill" on:click=move |_| ctx.back()>
                        <Icon name="back"/><span>{move || i18n.t("setup-back")}</span>
                    </button>
                </Show>
                <button class="f primary" data-auto=next_is_default.then_some("") on:click=move |_| {
                    if last {
                        finish(ctx, i18n.lang.get_untracked());
                    } else {
                        ctx.replace(Screen::Setup(step + 1));
                    }
                }>
                    <span>{move || i18n.t(if last { "setup-start" } else { "setup-next" })}</span>
                    <Icon name="next"/>
                </button>
            </div>
        </main>
    }
}
