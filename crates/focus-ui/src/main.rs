//! Interface de Focus Launcher, écrite en Rust et compilée en WebAssembly.

mod api;
mod app;
mod i18n;
mod keyboard;
mod nav;
mod screens;
mod sound;

fn main() {
    leptos::mount::mount_to_body(app::App);
}
