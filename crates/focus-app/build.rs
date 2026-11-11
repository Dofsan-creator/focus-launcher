fn main() {
    // Focus demande les droits administrateur (SPEC D5, choix de l'utilisateur).
    let windows = tauri_build::WindowsAttributes::new().app_manifest(include_str!("app.manifest"));
    if let Err(err) = tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows)) {
        println!("cargo:warning=tauri build failed: {err}");
        std::process::exit(1);
    }
}
