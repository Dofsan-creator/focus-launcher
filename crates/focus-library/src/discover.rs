//! Assemble les scanners sur la machine réelle. Lecture seule du registre.

use crate::scanners::{self, StartApp, UninstallRecord};
use focus_core::model::{Entry, Store};
use serde::Serialize;
use std::path::PathBuf;

/// Résultat du scan d'une boutique.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StoreScan {
    /// Préfixe des identifiants (`steam`, `epic`…).
    pub prefix: &'static str,
    /// Le client ou ses données ont été trouvés.
    pub present: bool,
    /// Entrées trouvées.
    pub entries: Vec<Entry>,
    /// Erreur à afficher, avec proposition d'ajout manuel.
    pub error: Option<String>,
}

fn outcome(prefix: &'static str, result: Result<Vec<Entry>, crate::ScanError>) -> StoreScan {
    match result {
        Ok(entries) => StoreScan {
            prefix,
            present: true,
            entries,
            error: None,
        },
        Err(err) => StoreScan {
            prefix,
            present: false,
            entries: Vec::new(),
            error: Some(err.to_string()),
        },
    }
}

/// Analyse toutes les boutiques. Chacune est isolée : une panne n'empêche pas les autres.
pub fn scan_all(start_apps: &[StartApp]) -> Vec<StoreScan> {
    let records = registry::uninstall_records();
    let mut scans = Vec::new();
    match registry::steam_root() {
        Some(root) => scans.push(outcome("steam", scanners::scan_steam(&root))),
        None => scans.push(absent("steam")),
    }
    let epic = program_data().join("Epic/EpicGamesLauncher/Data/Manifests");
    scans.push(if epic.is_dir() {
        outcome("epic", scanners::scan_epic(&epic))
    } else {
        absent("epic")
    });

    let gog_dirs = registry::gog_dirs();
    let mut gog = Vec::new();
    for dir in &gog_dirs {
        match scanners::scan_gog_dir(dir) {
            Ok(Some(entry)) => gog.push(entry),
            Ok(None) => {}
            Err(err) => tracing::warn!("gog: {err}"),
        }
    }
    scans.push(StoreScan {
        prefix: "gog",
        present: !gog_dirs.is_empty(),
        entries: gog,
        error: None,
    });

    let ubisoft: Vec<Entry> = registry::ubisoft_installs()
        .iter()
        .map(|(id, dir)| scanners::ubisoft_entry(id, dir))
        .collect();
    scans.push(StoreScan {
        prefix: "ubisoft",
        present: !ubisoft.is_empty(),
        entries: ubisoft,
        error: None,
    });

    let ea = scanners::scan_publisher(&records, Store::Ea, "electronic arts", "ea");
    scans.push(StoreScan {
        prefix: "ea",
        present: !ea.is_empty(),
        entries: ea,
        error: None,
    });
    let battle = scanners::scan_publisher(&records, Store::BattleNet, "blizzard", "battlenet");
    scans.push(StoreScan {
        prefix: "battlenet",
        present: !battle.is_empty(),
        entries: battle,
        error: None,
    });

    let mut xbox = Vec::new();
    for root in xbox_roots() {
        if let Ok(found) = scanners::scan_xbox(&root, start_apps) {
            xbox.extend(found);
        }
    }
    scans.push(StoreScan {
        prefix: "xbox",
        present: !xbox.is_empty(),
        entries: xbox,
        error: None,
    });
    scans
}

fn absent(prefix: &'static str) -> StoreScan {
    StoreScan {
        prefix,
        present: false,
        entries: Vec::new(),
        error: None,
    }
}

/// Dossier `ProgramData` de Windows.
pub fn program_data() -> PathBuf {
    std::env::var_os("ProgramData").map_or_else(|| PathBuf::from("C:/ProgramData"), PathBuf::from)
}

/// Programmes installés : nom en minuscules et dossier (registre, lecture seule).
pub fn installed_locations() -> Vec<(String, String)> {
    registry::uninstall_records()
        .into_iter()
        .map(|record| (record.name.to_lowercase(), record.install_location))
        .collect()
}

/// Noms des programmes installés (registre, lecture seule).
pub fn installed_names() -> Vec<String> {
    registry::uninstall_records()
        .into_iter()
        .map(|record| record.name.to_lowercase())
        .collect()
}

/// Dossier d'installation de Steam, lu dans le registre (lecture seule).
pub fn steam_root() -> Option<PathBuf> {
    registry::steam_root()
}

/// `XboxGames` à la racine de chaque disque fixe.
fn xbox_roots() -> Vec<PathBuf> {
    ('C'..='Z')
        .map(|drive| PathBuf::from(format!("{drive}:\\XboxGames")))
        .filter(|path| path.is_dir())
        .collect()
}

/// Chemins surveillés pour voir un nouveau jeu sans redémarrer.
pub fn watch_paths() -> Vec<PathBuf> {
    let mut paths = vec![program_data().join("Epic/EpicGamesLauncher/Data/Manifests")];
    if let Some(root) = registry::steam_root() {
        if let Ok(libraries) = scanners::steam_libraries(&root) {
            paths.extend(
                libraries
                    .into_iter()
                    .map(|library| library.join("steamapps")),
            );
        }
    }
    paths.retain(|path| path.is_dir());
    paths
}

#[cfg(windows)]
mod registry {
    use super::UninstallRecord;
    use std::path::PathBuf;
    use winreg::enums::{
        HKEY_CURRENT_USER, HKEY_LOCAL_MACHINE, KEY_READ, KEY_WOW64_32KEY, KEY_WOW64_64KEY,
    };
    use winreg::RegKey;

    pub fn steam_root() -> Option<PathBuf> {
        let key = RegKey::predef(HKEY_CURRENT_USER)
            .open_subkey("Software\\Valve\\Steam")
            .ok()?;
        let path: String = key.get_value("SteamPath").ok()?;
        Some(PathBuf::from(path))
    }

    fn open_32(path: &str) -> Option<RegKey> {
        RegKey::predef(HKEY_LOCAL_MACHINE)
            .open_subkey_with_flags(path, KEY_READ | KEY_WOW64_32KEY)
            .ok()
    }

    pub fn gog_dirs() -> Vec<PathBuf> {
        let Some(games) = open_32("SOFTWARE\\GOG.com\\Games") else {
            return Vec::new();
        };
        games
            .enum_keys()
            .flatten()
            .filter_map(|id| games.open_subkey(id).ok())
            .filter_map(|game| game.get_value::<String, _>("path").ok())
            .map(PathBuf::from)
            .collect()
    }

    pub fn ubisoft_installs() -> Vec<(String, String)> {
        let Some(installs) = open_32("SOFTWARE\\Ubisoft\\Launcher\\Installs") else {
            return Vec::new();
        };
        installs
            .enum_keys()
            .flatten()
            .filter_map(|id| {
                let dir: String = installs
                    .open_subkey(&id)
                    .ok()?
                    .get_value("InstallDir")
                    .ok()?;
                Some((id, dir))
            })
            .collect()
    }

    pub fn uninstall_records() -> Vec<UninstallRecord> {
        const PATH: &str = "SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Uninstall";
        let mut records = Vec::new();
        for view in [KEY_WOW64_32KEY, KEY_WOW64_64KEY] {
            let Ok(root) =
                RegKey::predef(HKEY_LOCAL_MACHINE).open_subkey_with_flags(PATH, KEY_READ | view)
            else {
                continue;
            };
            for key in root.enum_keys().flatten() {
                let Ok(sub) = root.open_subkey(&key) else {
                    continue;
                };
                let value = |name: &str| sub.get_value::<String, _>(name).unwrap_or_default();
                let name = value("DisplayName");
                if !name.is_empty() {
                    records.push(UninstallRecord {
                        key,
                        name,
                        publisher: value("Publisher"),
                        install_location: value("InstallLocation"),
                    });
                }
            }
        }
        records
    }
}

#[cfg(not(windows))]
mod registry {
    use super::UninstallRecord;
    use std::path::PathBuf;

    pub fn steam_root() -> Option<PathBuf> {
        None
    }
    pub fn gog_dirs() -> Vec<PathBuf> {
        Vec::new()
    }
    pub fn ubisoft_installs() -> Vec<(String, String)> {
        Vec::new()
    }
    pub fn uninstall_records() -> Vec<UninstallRecord> {
        Vec::new()
    }
}
