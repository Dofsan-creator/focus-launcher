//! Connexions : Wi-Fi et Bluetooth, pilotables à la manette.
//! Le mot de passe Wi-Fi passe par le clavier à l'écran ; il n'est jamais gardé.

use crate::api::{invoke, BtDevice, RadioInfo, WifiNetwork};
use crate::app::{after_render, ctx, refresh_status, Screen, TextEdit};
use crate::i18n::use_i18n;
use crate::icons::Icon;
use crate::nav;
use crate::screens::{Confirm, Toggle};
use leptos::prelude::*;
use leptos::task::spawn_local;
use serde_json::{json, Value};

/// Interrupteur d'une radio, qui affiche l'état réel de Windows.
#[component]
fn RadioToggle(kind: &'static str, label: &'static str) -> impl IntoView {
    let on = RwSignal::new(false);
    let load = move || {
        spawn_local(async move {
            let radios: Vec<RadioInfo> = invoke("radios", json!({})).await.unwrap_or_default();
            on.set(radios.iter().any(|radio| radio.kind == kind && radio.on));
        });
    };
    load();
    view! {
        <Toggle label=label value=Signal::derive(move || on.get()) on_change=Callback::new(move |_| {
            let wanted = !on.get_untracked();
            spawn_local(async move {
                let _ = invoke::<Value>("set_radio", json!({ "kind": kind, "on": wanted })).await;
                load();
            });
        })/>
    }
}

#[component]
fn WifiPane() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let networks = RwSignal::new(Vec::<WifiNetwork>::new());
    let busy = RwSignal::new(false);
    let chosen = RwSignal::new(None::<WifiNetwork>);
    let password = RwSignal::new(String::new());
    let load = move |scan: bool| {
        busy.set(true);
        spawn_local(async move {
            networks.set(invoke("wifi_networks", json!({ "scan": scan })).await.unwrap_or_default());
            busy.set(false);
        });
    };
    load(true);
    let connect = move |ssid: String, secret: Option<String>| {
        busy.set(true);
        spawn_local(async move {
            let result: String = invoke("wifi_connect", json!({ "ssid": ssid, "password": secret }))
                .await
                .unwrap_or_else(|_| "failed".to_owned());
            ctx.toast.set(Some(format!("con-result-{result}")));
            chosen.set(None);
            password.set(String::new());
            refresh_status(ctx);
            load(false);
        });
    };
    let pick = move |network: WifiNetwork| {
        if network.connected {
            spawn_local(async move {
                let _ = invoke::<Value>("wifi_disconnect", json!({})).await;
                ctx.toast.set(Some("con-disconnected".to_owned()));
                refresh_status(ctx);
                load(false);
            });
        } else if network.secure {
            password.set(String::new());
            chosen.set(Some(network));
            ctx.editing.set(Some(TextEdit { value: password, secret: true }));
        } else {
            connect(network.ssid, None);
        }
    };
    view! {
        <RadioToggle kind="wifi" label="con-wifi"/>
        <div class="row">
            <button class="f pill" on:click=move |_| load(true)>
                <Icon name="refresh"/><span>{move || i18n.t("con-scan")}</span>
            </button>
            <Show when=move || busy.get()><span class="muted pulse">{move || i18n.t("con-searching")}</span></Show>
        </div>
        {move || chosen.get().map(|network| {
            let ssid = network.ssid.clone();
            view! {
                <div class="card connect-card">
                    <h2><Icon name="lock"/>{network.ssid}</h2>
                    <button class="f pill" on:click=move |_| ctx.editing.set(Some(TextEdit { value: password, secret: true }))>
                        {move || format!("{} · {}", i18n.t("con-password"), "•".repeat(password.with(|p| p.chars().count())))}
                    </button>
                    <div class="row">
                        <button class="f primary" data-auto="" on:click=move |_| connect(ssid.clone(), Some(password.get_untracked()))>
                            {move || i18n.t("con-connect")}
                        </button>
                        <button class="f pill" on:click=move |_| { chosen.set(None); after_render(nav::focus_first); }>
                            {move || i18n.t("prep-cancel")}
                        </button>
                    </div>
                </div>
            }
        })}
        <div class="list">
            {move || {
                let list = networks.get();
                if list.is_empty() && !busy.get() {
                    return view! { <p class="muted">{move || i18n.t("con-none")}</p> }.into_any();
                }
                list.into_iter().map(|network| {
                    let (ssid, bars, secure, connected) = (network.ssid.clone(), network.bars, network.secure, network.connected);
                    view! {
                        <button class="f row-item network" class:current=connected on:click=move |_| pick(network.clone())>
                            <Icon name="wifi" level=bars.max(1)/>
                            <span class="grow">{ssid}</span>
                            {secure.then(|| view! { <Icon name="lock"/> })}
                            {connected.then(|| view! { <span class="badge">{move || i18n.t("con-connected")}</span> })}
                        </button>
                    }
                }).collect_view().into_any()
            }}
        </div>
    }
}

#[component]
fn BtPane() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let paired = RwSignal::new(Vec::<BtDevice>::new());
    let nearby = RwSignal::new(Vec::<BtDevice>::new());
    let searching = RwSignal::new(false);
    let forget = RwSignal::new(None::<BtDevice>);
    let load = move || {
        spawn_local(async move {
            paired.set(invoke("bt_paired", json!({})).await.unwrap_or_default());
        });
    };
    load();
    let search = move || {
        searching.set(true);
        spawn_local(async move {
            nearby.set(invoke("bt_nearby", json!({})).await.unwrap_or_default());
            searching.set(false);
        });
    };
    let pair = move |id: String| {
        searching.set(true);
        spawn_local(async move {
            let result: String = invoke("bt_pair", json!({ "id": id })).await.unwrap_or_else(|_| "failed".to_owned());
            ctx.toast.set(Some(format!("bt-result-{result}")));
            searching.set(false);
            nearby.update(|list| list.retain(|device| result != "success" || device.id != id));
            load();
        });
    };
    let question = Signal::derive(move || {
        let name = forget.with(|device| device.as_ref().map(|d| d.name.clone()).unwrap_or_default());
        i18n.ts("bt-forget-confirm", "name", &name)
    });
    view! {
        <RadioToggle kind="bluetooth" label="con-bt"/>
        <h2>{move || i18n.t("bt-paired")}</h2>
        <div class="list">
            {move || paired.get().into_iter().map(|device| {
                let (name, connected) = (device.name.clone(), device.connected);
                view! {
                    <button class="f row-item" class:current=connected on:click=move |_| {
                        forget.set(Some(device.clone()));
                        ctx.dialog_open.set(true);
                        after_render(nav::focus_first);
                    }>
                        <Icon name="bluetooth"/>
                        <span class="grow">{name}</span>
                        {connected.then(|| view! { <span class="badge">{move || i18n.t("con-connected")}</span> })}
                    </button>
                }
            }).collect_view()}
        </div>
        <div class="row">
            <button class="f pill" on:click=move |_| search()>
                <Icon name="search"/><span>{move || i18n.t("bt-search")}</span>
            </button>
            <Show when=move || searching.get()><span class="muted pulse">{move || i18n.t("con-searching")}</span></Show>
        </div>
        <p class="muted">{move || i18n.t("bt-pair-hint")}</p>
        <div class="list">
            {move || nearby.get().into_iter().map(|device| {
                let (id, name) = (device.id, device.name);
                view! {
                    <button class="f row-item" on:click=move |_| pair(id.clone())>
                        <Icon name="add"/><span class="grow">{name}</span>
                        <span class="badge">{move || i18n.t("bt-pair")}</span>
                    </button>
                }
            }).collect_view()}
        </div>
        <Show when=move || forget.get().is_some() && ctx.dialog_open.get()>
            <Confirm question=question danger=true on_yes=Callback::new(move |_| {
                if let Some(device) = forget.get_untracked() {
                    spawn_local(async move {
                        let result: String = invoke("bt_unpair", json!({ "id": device.id })).await.unwrap_or_else(|_| "failed".to_owned());
                        ctx.toast.set(Some(if result == "success" { "bt-forgotten" } else { "bt-result-failed" }.to_owned()));
                        load();
                    });
                }
                forget.set(None);
                ctx.back();
            })/>
        </Show>
    }
}

#[component]
pub fn ConnectionsScreen() -> impl IntoView {
    let (ctx, i18n) = (ctx(), use_i18n());
    let tab = ctx.con_tab;
    view! {
        <main class="screen sheet-screen connections">
            <h1>{move || i18n.t(if tab.get() == "wifi" { "con-wifi" } else { "con-bt" })}</h1>
            <Show when=move || tab.get() == "wifi" && ctx.net.with(|net| net.kind == "ethernet")>
                <p class="status-line"><Icon name="ethernet"/>{move || i18n.t("con-ethernet-on")}</p>
            </Show>
            {move || if tab.get() == "wifi" {
                view! { <WifiPane/> }.into_any()
            } else {
                view! { <BtPane/> }.into_any()
            }}
            <div class="row">
                <button class="f pill" on:click=move |_| {
                    if ctx.screens.with_untracked(|list| list.len() > 1) {
                        ctx.back();
                    } else {
                        ctx.push(Screen::Home);
                    }
                }><Icon name="back"/><span>{move || i18n.t("hint-back")}</span></button>
            </div>
        </main>
    }
}
