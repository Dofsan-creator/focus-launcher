//! Application Focus Launcher : relie la bibliothèque, le moteur et l'interface.

mod boot_guard;
mod commands;
mod guardian_client;
mod pad_thread;
mod runtime;
mod settings;
mod watchers;

use runtime::App;
use std::sync::Arc;
use tauri::Manager;

/// Lance l'application.
pub fn run() {
    let result = tauri::Builder::default()
        .setup(|app| {
            let data = app.path().app_local_data_dir()?;
            std::fs::create_dir_all(data.join("logs"))?;
            let writer = tracing_appender::rolling::daily(data.join("logs"), "launcher.log");
            let _ = tracing_subscriber::fmt().with_writer(writer).with_ansi(false).try_init();

            let shared = Arc::new(App::open(&data)?);
            shared.recover_previous_session();
            let boot = boot_guard::on_start(&data);
            app.manage(shared.clone());
            app.manage(boot);
            pad_thread::spawn(app.handle().clone(), shared.clone());
            watchers::spawn(app.handle().clone(), shared);
            if let Some(window) = app.get_webview_window("main") {
                window.show()?;
            }
            Ok(())
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
        ])
        .run(tauri::generate_context!());
    if let Err(err) = result {
        tracing::error!("application failed: {err}");
    }
}
