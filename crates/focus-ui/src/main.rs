//! Interface de Focus Launcher, écrite en Rust et compilée en WebAssembly.

mod api;
mod app;
mod connect;
mod i18n;
mod icons;
mod keyboard;
mod nav;
mod screens;
mod setup;
mod sound;

fn main() {
    // Un panic arrête toute l'interface : on écrit au moins sa cause dans la console.
    std::panic::set_hook(Box::new(|info| {
        web_sys::console::error_1(&format!("focus-ui panic: {info}").into());
    }));
    leptos::mount::mount_to_body(app::App);
}
