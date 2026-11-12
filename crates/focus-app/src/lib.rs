//! Application Focus Launcher : relie la bibliothèque, le moteur et l'interface.

mod art;
mod boot_guard;
mod commands;
mod console;
mod guardian_client;
mod pad_thread;
mod runtime;
mod settings;
mod watchers;

use console::Console;
use runtime::App;
use std::path::Path;
use std::sync::Arc;
use tauri::{Manager, RunEvent, WindowEvent};

/// Options de la ligne de commande.
#[derive(Debug, Default, Clone, Copy)]
pub struct Launch {
    /// Ouvert par le raccourci de démarrage de session.
    pub autostart: bool,
}

/// Sentinelle : surveille le launcher `pid` et défait le mode console s'il plante.
pub fn sentinel(pid: u32, data: &Path) {
    console::sentinel(pid, data);
}

/// Sortie de secours : coupe l'ouverture à l'ouverture de session (v4.3).
pub fn disable_autostart() {
    #[cfg(windows)]
    if let Ok(exe) = std::env::current_exe() {
        let _ = focus_sys::shell::set_autostart(false, &exe);
    }
}

/// Lance l'application.
pub fn run(launch: Launch) {
    let builder = tauri::Builder::default()
        // Un seul Focus à la fois : deux modes console se gêneraient. Relancer Focus
        // ramène la fenêtre déjà ouverte, même cachée pendant un jeu.
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            commands::back_to_launcher(app);
        }))
        // Mises à jour signées, prises dans les versions publiées du dépôt.
        .plugin(tauri_plugin_updater::Builder::new().build())
        .register_asynchronous_uri_scheme_protocol("focusart", |ctx, request, responder| {
            let data = ctx.app_handle().path().app_local_data_dir().ok();
            let path = request.uri().path().to_owned();
            std::thread::spawn(move || {
                let response = match data {
                    Some(data) => art::respond(&data, &path),
                    None => tauri::http::Response::new(Vec::new()),
                };
                responder.respond(response);
            });
        })
        .setup(move |app| {
            let data = app.path().app_local_data_dir()?;
            std::fs::create_dir_all(data.join("logs"))?;
            let writer = tracing_appender::rolling::daily(data.join("logs"), "launcher.log");
            let _ = tracing_subscriber::fmt()
                .with_writer(writer)
                .with_ansi(false)
                .try_init();

            // Un plantage précédent a pu laisser Windows en mode console.
            console::recover(&data);
            #[cfg(windows)]
            focus_sys::console::set_own_priority(false);
            let shared = Arc::new(App::open(&data)?);
            shared.recover_previous_session();
            let boot = boot_guard::on_start(&data);
            let ui = boot_guard::UiStarted::default();
            let handle = app.handle().clone();
            boot_guard::watch(&data, ui.clone(), move || {
                // En développement, la fenêtre reste ouverte pour le débogage.
                if !cfg!(debug_assertions) {
                    handle.exit(1);
                }
            });
            let console = Console::new(&data);
            console::spawn_sentinel(&data);
            app.manage(shared.clone());
            app.manage(boot);
            app.manage(ui);
            app.manage(console);
            pad_thread::spawn(app.handle().clone(), shared.clone());
            watchers::spawn(app.handle().clone(), shared.clone());

            let settings = shared.settings();
            let minimized = launch.autostart && settings.start_minimized;
            // La fenêtre s'affiche tout de suite ; le mode console et le plan
            // d'optimisation se font derrière l'animation de démarrage.
            if !minimized {
                commands::apply_console_in_background(app.handle());
            }
            if let Some(window) = app.get_webview_window("main") {
                if launch.autostart && settings.autostart_delay > 0 {
                    // Laisse la session Windows finir de se charger (v4.3).
                    let delay = std::time::Duration::from_secs(u64::from(settings.autostart_delay.min(30)));
                    std::thread::spawn(move || {
                        std::thread::sleep(delay);
                        let _ = window.show();
                        if minimized {
                            let _ = window.minimize();
                        }
                    });
                } else {
                    window.show()?;
                    if minimized {
                        window.minimize()?;
                    }
                }
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            // Démarré réduit : Windows passe en mode console quand on ouvre le launcher.
            if let WindowEvent::Focused(true) = event {
                let app = window.app_handle();
                let console = app.state::<Console>();
                let settings = app.state::<Arc<App>>().settings();
                let wanted = settings.first_run_done && settings.console_mode;
                if wanted && !console.is_active() && !commands::console_applying() {
                    commands::apply_console_in_background(app);
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::boot_info,
            commands::get_settings,
            commands::set_settings,
            commands::list_entries,
            commands::refresh_library,
            commands::preview_session,
            commands::play,
            commands::close_running,
            commands::force_close_running,
            commands::return_to_running,
            commands::running_entry,
            commands::add_file,
            commands::remove_entry,
            commands::set_favorite,
            commands::list_drives,
            commands::list_dir,
            commands::suggest_executables,
            commands::catalog,
            commands::install_app,
            commands::open_official_page,
            commands::system_report,
            commands::set_autostart,
            commands::guardian_status,
            commands::install_guardian,
            commands::controller_status,
            commands::power_action,
            commands::set_app_volume,
            commands::audio_sessions,
            commands::net_status,
            commands::radios,
            commands::set_radio,
            commands::wifi_networks,
            commands::wifi_connect,
            commands::wifi_disconnect,
            commands::bt_paired,
            commands::bt_nearby,
            commands::bt_pair,
            commands::bt_unpair,
            commands::apply_console,
            commands::console_report,
            commands::pointer_mode,
            commands::close_open_apps,
            commands::downloads,
            commands::app_version,
            commands::report_problem,
            commands::update_check,
            commands::update_install,
            commands::open_store,
            commands::screenshot,
            commands::update_status,
            commands::open_updates,
            commands::steam_guide_status,
            commands::open_steam_controller_settings,
        ]);
    let app = match builder.build(tauri::generate_context!()) {
        Ok(app) => app,
        Err(err) => {
            tracing::error!("application failed: {err}");
            return;
        }
    };
    app.run(|handle, event| {
        // Fermeture normale, Alt+F4 ou garde-fou : Windows retrouve son état.
        if let RunEvent::Exit = event {
            if let Some(console) = handle.try_state::<Console>() {
                console.leave();
            }
            if let Some(state) = handle.try_state::<Arc<App>>() {
                state.restore_console_plan();
            }
        }
    });
}
