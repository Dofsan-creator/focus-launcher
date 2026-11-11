//! HidHide (Nefarius, licence MIT) : masque la vraie manette PlayStation aux jeux
//! pendant que Focus Input leur présente une manette Xbox virtuelle. Sans cela, le
//! jeu voit les deux et les commandes se doublent.
//!
//! Tout passe par l'outil officiel `HidHideCLI.exe`, sans élévation. Ce qui est
//! masqué est noté par l'appelant avant tout changement, puis rendu à la fin de la
//! session, après un plantage ou au démarrage suivant. Seules les manettes Sony
//! sont visées ; Focus reste autorisé à les lire.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::path::{Path, PathBuf};

const CREATE_NO_WINDOW: u32 = 0x0800_0000;

/// Ce que Focus a changé dans HidHide, pour le défaire.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Hidden {
    /// Chemins d'instance ajoutés par Focus à la liste masquée.
    pub paths: Vec<String>,
    /// Le masquage était déjà actif avant Focus.
    pub was_cloaked: bool,
}

/// `HidHideCLI.exe`, s'il est installé (dossier actuel, puis ancien dossier `x64`).
pub fn cli() -> Option<PathBuf> {
    let base = std::env::var_os("ProgramFiles").map(PathBuf::from)?;
    let dir = base.join("Nefarius Software Solutions").join("HidHide");
    [dir.join("HidHideCLI.exe"), dir.join("x64").join("HidHideCLI.exe")]
        .into_iter()
        .find(|path| path.is_file())
}

/// HidHide est-il utilisable ?
pub fn present() -> bool {
    cli().is_some()
}

fn run(cli: &Path, args: &[String]) -> Option<String> {
    let mut command = std::process::Command::new(cli);
    command.args(args);
    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        command.creation_flags(CREATE_NO_WINDOW);
    }
    let output = command.output().ok()?;
    if !output.status.success() {
        tracing::warn!("hidhide {:?} failed: {}", args, String::from_utf8_lossy(&output.stderr));
        return None;
    }
    Some(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Chemins d'instance des manettes Sony dans l'inventaire `--dev-gaming`. La forme
/// exacte du JSON n'est pas figée : tout objet portant `deviceInstancePath` compte.
pub fn playstation_paths(inventory: &str) -> Vec<String> {
    fn walk(value: &Value, out: &mut Vec<String>) {
        match value {
            Value::Array(items) => items.iter().for_each(|item| walk(item, out)),
            Value::Object(map) => {
                if let Some(Value::String(path)) = map.get("deviceInstancePath") {
                    let vendor = map.get("vendor").and_then(Value::as_str).unwrap_or_default();
                    let lower = path.to_lowercase();
                    // USB : `VID_054C` ; Bluetooth : `VID&0002054C`.
                    let sony = lower.contains("054c") || vendor.to_lowercase().contains("sony");
                    if sony && !out.contains(path) {
                        out.push(path.clone());
                    }
                }
                map.values().for_each(|child| walk(child, out));
            }
            _ => {}
        }
    }
    let mut out = Vec::new();
    if let Ok(value) = serde_json::from_str::<Value>(inventory) {
        walk(&value, &mut out);
    }
    out
}

/// Ce qui serait masqué maintenant (à noter dans le journal avant `hide`).
pub fn plan() -> Option<Hidden> {
    let cli = cli()?;
    let inventory = run(&cli, &["--dev-gaming".to_owned()])?;
    let already = run(&cli, &["--dev-list".to_owned()]).unwrap_or_default();
    let paths: Vec<String> = playstation_paths(&inventory)
        .into_iter()
        .filter(|path| !already.contains(path.as_str()))
        .collect();
    let state = run(&cli, &["--cloak-state".to_owned()]).unwrap_or_default();
    Some(Hidden {
        paths,
        was_cloaked: state.contains("--cloak-on"),
    })
}

/// Masque les manettes du plan aux autres programmes ; `own_exe` (Focus) les voit
/// toujours. Une seule commande : le pilote est écrit une fois, à la sortie de l'outil.
pub fn hide(hidden: &Hidden, own_exe: &Path) -> bool {
    let Some(cli) = cli() else { return false };
    let mut args = vec!["--app-reg".to_owned(), own_exe.to_string_lossy().into_owned()];
    for path in &hidden.paths {
        args.push("--dev-hide".to_owned());
        args.push(path.clone());
    }
    args.push("--cloak-on".to_owned());
    run(&cli, &args).is_some()
}

/// Rend ce que `hide` a masqué, et coupe le masquage s'il était coupé avant.
pub fn restore(hidden: &Hidden) -> bool {
    let Some(cli) = cli() else { return false };
    let mut args = Vec::new();
    for path in &hidden.paths {
        args.push("--dev-unhide".to_owned());
        args.push(path.clone());
    }
    if !hidden.was_cloaked {
        args.push("--cloak-off".to_owned());
    }
    args.is_empty() || run(&cli, &args).is_some()
}

#[cfg(test)]
mod tests {
    use super::playstation_paths;

    #[test]
    fn sony_pads_should_be_found_over_usb_and_bluetooth_only() {
        let inventory = r#"[
            {"friendlyName": "Wireless Controller", "devices": [
                {"present": true, "gamingDevice": true, "vendor": "Sony Interactive Entertainment",
                 "deviceInstancePath": "HID\\VID_054C&PID_09CC&MI_03\\7&1a2b&0&0000"}]},
            {"friendlyName": "Wireless Controller BT", "devices": [
                {"vendor": "", "deviceInstancePath": "HID\\{00001124-0000-1000-8000-00805F9B34FB}_VID&0002054C_PID&09CC\\8&abc&0&0000"}]},
            {"friendlyName": "Xbox Controller", "devices": [
                {"vendor": "Microsoft", "deviceInstancePath": "HID\\VID_045E&PID_0B12&IG_00\\9&def&0&0000"}]}
        ]"#;
        let paths = playstation_paths(inventory);
        assert_eq!(paths.len(), 2);
        assert!(paths.iter().all(|path| path.to_lowercase().contains("054c")));
    }

    #[test]
    fn unreadable_inventory_should_hide_nothing() {
        assert!(playstation_paths("not json").is_empty());
    }
}
