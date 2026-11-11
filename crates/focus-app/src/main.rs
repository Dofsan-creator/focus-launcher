#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::path::PathBuf;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        // Sentinelle : un second exemplaire discret qui attend la fin du premier.
        Some("--sentinel") => {
            let pid = args.get(1).and_then(|pid| pid.parse().ok());
            if let (Some(pid), Some(dir)) = (pid, args.get(2)) {
                focus_app::sentinel(pid, &PathBuf::from(dir));
            }
        }
        // Sortie de secours de la v4.3 : coupe l'ouverture automatique, sans rien ouvrir.
        Some("--sans-demarrage-auto") => focus_app::disable_autostart(),
        _ => focus_app::run(focus_app::Launch {
            autostart: args.iter().any(|arg| arg == "--autostart"),
        }),
    }
}
