//! Un scanner par boutique, en lecture seule, testé sur de faux dossiers.

use crate::exe_score;
use crate::vdf::{self, Value};
use crate::ScanError;
use focus_core::model::{Entry, EntryKind, LaunchTarget, Stats, Store};
use serde::Deserialize;
use std::fs;
use std::path::{Path, PathBuf};

fn read(path: &Path) -> Result<String, ScanError> {
    let bytes = fs::read(path).map_err(|source| ScanError::Io {
        path: path.display().to_string(),
        source,
    })?;
    Ok(String::from_utf8_lossy(&bytes).into_owned())
}

fn entry(
    id: String,
    title: String,
    store: Store,
    target: LaunchTarget,
    dir: Option<String>,
) -> Entry {
    Entry {
        id,
        title,
        kind: EntryKind::Game,
        store,
        target,
        install_dir: dir,
        cover: None,
        favorite: false,
        stats: Stats::default(),
    }
}

// ---- Steam ----

/// Outils Steam à ne pas montrer comme jeux.
const STEAM_TOOLS: &[&str] = &[
    "steamworks common redistributables",
    "spacewar",
    "proton",
    "steam linux runtime",
];

/// Dossiers de bibliothèques déclarés dans `libraryfolders.vdf`.
pub fn steam_libraries(steam_root: &Path) -> Result<Vec<PathBuf>, ScanError> {
    let file = steam_root.join("steamapps").join("libraryfolders.vdf");
    let doc =
        vdf::parse(&read(&file)?).ok_or_else(|| ScanError::Format(file.display().to_string()))?;
    let folders = doc
        .get("libraryfolders")
        .ok_or_else(|| ScanError::Format(file.display().to_string()))?;
    let mut paths = vec![steam_root.to_path_buf()];
    if let Value::Block(map) = folders {
        for value in map.values() {
            if let Some(path) = value.get("path").and_then(Value::text) {
                let path = PathBuf::from(path);
                if !paths.contains(&path) {
                    paths.push(path);
                }
            }
        }
    }
    Ok(paths)
}

/// Lit les `appmanifest_*.acf` de toutes les bibliothèques.
pub fn scan_steam(steam_root: &Path) -> Result<Vec<Entry>, ScanError> {
    let mut entries = Vec::new();
    for library in steam_libraries(steam_root)? {
        let apps = library.join("steamapps");
        let Ok(dir) = fs::read_dir(&apps) else {
            continue;
        };
        for file in dir.flatten() {
            let path = file.path();
            let name = file.file_name().to_string_lossy().to_lowercase();
            if !(name.starts_with("appmanifest_") && name.ends_with(".acf")) {
                continue;
            }
            match steam_manifest(&path, &apps) {
                Ok(Some(found)) => entries.push(found),
                Ok(None) => {}
                Err(err) => tracing::warn!("skipping steam manifest: {err}"),
            }
        }
    }
    Ok(entries)
}

fn steam_manifest(path: &Path, apps: &Path) -> Result<Option<Entry>, ScanError> {
    let format = || ScanError::Format(path.display().to_string());
    let doc = vdf::parse(&read(path)?).ok_or_else(format)?;
    let state = doc.get("AppState").ok_or_else(format)?;
    let field = |key: &str| state.get(key).and_then(Value::text);
    let (Some(appid), Some(title)) = (field("appid"), field("name")) else {
        return Err(format());
    };
    if STEAM_TOOLS
        .iter()
        .any(|tool| title.to_lowercase().starts_with(tool))
    {
        return Ok(None);
    }
    let dir = field("installdir").map(|name| apps.join("common").join(name).display().to_string());
    Ok(Some(entry(
        format!("steam:{appid}"),
        title.to_owned(),
        Store::Steam,
        LaunchTarget::Protocol {
            uri: format!("steam://rungameid/{appid}"),
        },
        dir,
    )))
}

// ---- Epic ----

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct EpicManifest {
    display_name: String,
    app_name: String,
    install_location: String,
    #[serde(default)]
    app_categories: Vec<String>,
    #[serde(default, rename = "bIsIncompleteInstall")]
    incomplete: bool,
}

/// Lit les manifestes `.item`. Seuls les jeux complets sont gardés : le moteur,
/// les plugins et les DLC ne portent pas la catégorie `games`.
pub fn scan_epic(manifests: &Path) -> Result<Vec<Entry>, ScanError> {
    let dir = fs::read_dir(manifests).map_err(|source| ScanError::Io {
        path: manifests.display().to_string(),
        source,
    })?;
    let mut entries = Vec::new();
    for file in dir.flatten() {
        let path = file.path();
        if !path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("item"))
        {
            continue;
        }
        let Ok(text) = read(&path) else { continue };
        match serde_json::from_str::<EpicManifest>(&text) {
            Ok(item) if !item.incomplete && item.app_categories.iter().any(|c| c == "games") => {
                entries.push(entry(
                    format!("epic:{}", item.app_name),
                    item.display_name,
                    Store::Epic,
                    LaunchTarget::Protocol {
                        uri: format!(
                            "com.epicgames.launcher://apps/{}?action=launch&silent=true",
                            item.app_name
                        ),
                    },
                    Some(item.install_location),
                ));
            }
            Ok(_) => {}
            Err(err) => tracing::warn!("skipping epic manifest {}: {err}", path.display()),
        }
    }
    Ok(entries)
}

// ---- GOG ----

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GogInfo {
    game_id: String,
    name: String,
    #[serde(default)]
    play_tasks: Vec<GogTask>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct GogTask {
    #[serde(default)]
    is_primary: bool,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    arguments: Option<String>,
    #[serde(default)]
    working_dir: Option<String>,
}

/// Lit le fichier `goggame-<id>.info` d'un dossier de jeu. Les DLC n'ont pas de tâche principale.
pub fn scan_gog_dir(dir: &Path) -> Result<Option<Entry>, ScanError> {
    let list = fs::read_dir(dir).map_err(|source| ScanError::Io {
        path: dir.display().to_string(),
        source,
    })?;
    for file in list.flatten() {
        let name = file.file_name().to_string_lossy().to_lowercase();
        if !(name.starts_with("goggame-") && name.ends_with(".info")) {
            continue;
        }
        let path = file.path();
        let info: GogInfo = serde_json::from_str(&read(&path)?)
            .map_err(|_| ScanError::Format(path.display().to_string()))?;
        let Some(task) = info.play_tasks.iter().find(|task| task.is_primary) else {
            continue;
        };
        let Some(relative) = &task.path else { continue };
        return Ok(Some(entry(
            format!("gog:{}", info.game_id),
            info.name,
            Store::Gog,
            LaunchTarget::Executable {
                path: dir.join(relative).display().to_string(),
                args: task
                    .arguments
                    .iter()
                    .flat_map(|a| a.split_whitespace())
                    .map(str::to_owned)
                    .collect(),
                working_dir: Some(
                    dir.join(task.working_dir.as_deref().unwrap_or(""))
                        .display()
                        .to_string(),
                ),
            },
            Some(dir.display().to_string()),
        )));
    }
    Ok(None)
}

// ---- Ubisoft ----

/// Fabrique une entrée à partir d'une clé `Ubisoft\Launcher\Installs\<id>`.
pub fn ubisoft_entry(id: &str, install_dir: &str) -> Entry {
    let title = Path::new(install_dir.trim_end_matches(['/', '\\']))
        .file_name()
        .map_or_else(
            || format!("Ubisoft {id}"),
            |n| n.to_string_lossy().into_owned(),
        );
    entry(
        format!("ubisoft:{id}"),
        title,
        Store::Ubisoft,
        LaunchTarget::Protocol {
            uri: format!("uplay://launch/{id}/0"),
        },
        Some(install_dir.to_owned()),
    )
}

// ---- EA et Battle.net ----

/// Clé de désinstallation Windows, lue en seule lecture.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UninstallRecord {
    /// Identifiant de la clé.
    pub key: String,
    /// Nom affiché.
    pub name: String,
    /// Éditeur.
    pub publisher: String,
    /// Dossier d'installation.
    pub install_location: String,
}

/// Applications de boutique à ne pas lister comme jeux.
const NOT_GAMES: &[&str] = &["ea app", "battle.net", "origin"];

/// EA et Battle.net n'exposent pas de format local stable : on passe par les
/// clés de désinstallation et le meilleur exécutable du dossier.
pub fn scan_publisher(
    records: &[UninstallRecord],
    store: Store,
    publisher: &str,
    prefix: &str,
) -> Vec<Entry> {
    records
        .iter()
        .filter(|record| record.publisher.to_lowercase().contains(publisher))
        .filter(|record| !NOT_GAMES.contains(&record.name.to_lowercase().as_str()))
        .filter(|record| !record.install_location.is_empty())
        .filter_map(|record| {
            let root = Path::new(&record.install_location);
            let best = exe_score::best_candidates(root, exe_score::DEFAULT_DEPTH, 1, &|| false);
            let exe = best.first()?;
            Some(entry(
                format!("{prefix}:{}", record.key),
                record.name.clone(),
                store,
                LaunchTarget::Executable {
                    path: exe.path.display().to_string(),
                    args: Vec::new(),
                    working_dir: exe.path.parent().map(|p| p.display().to_string()),
                },
                Some(record.install_location.clone()),
            ))
        })
        .collect()
}

// ---- Xbox ----

/// Application du Menu Démarrer (`Get-StartApps`).
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
pub struct StartApp {
    /// Nom.
    #[serde(rename = "Name")]
    pub name: String,
    /// AUMID.
    #[serde(rename = "AppID")]
    pub app_id: String,
}

/// Nom d'identité déclaré par `MicrosoftGame.config`, sans parseur XML complet :
/// seul l'attribut `Name` de la balise `Identity` est utile.
pub fn xbox_identity(config: &str) -> Option<String> {
    let tag = &config[config.find("<Identity")?..];
    let tag = &tag[..tag.find('>')?];
    let start = tag.find(" Name=\"")? + 7;
    let rest = &tag[start..];
    Some(rest[..rest.find('"')?].to_owned())
}

/// Parcourt `XboxGames` et relie chaque jeu à son AUMID. L'exécutable n'est jamais
/// copié : le jeu se lance par son identité d'application.
pub fn scan_xbox(games_root: &Path, apps: &[StartApp]) -> Result<Vec<Entry>, ScanError> {
    let dir = fs::read_dir(games_root).map_err(|source| ScanError::Io {
        path: games_root.display().to_string(),
        source,
    })?;
    let mut entries = Vec::new();
    for game in dir.flatten() {
        let content = game.path().join("Content");
        let Ok(config) = read(&content.join("MicrosoftGame.config")) else {
            continue;
        };
        let Some(identity) = xbox_identity(&config) else {
            continue;
        };
        let Some(app) = apps.iter().find(|app| app.app_id.starts_with(&identity)) else {
            continue;
        };
        entries.push(entry(
            format!("xbox:{identity}"),
            app.name.clone(),
            Store::Xbox,
            LaunchTarget::Packaged {
                aumid: app.app_id.clone(),
            },
            Some(content.display().to_string()),
        ));
    }
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn write(path: &Path, text: &str) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, text)
    }

    fn steam_fixture(root: &Path, library: &Path) -> std::io::Result<()> {
        let lib = library.display().to_string().replace('\\', "\\\\");
        write(
            &root.join("steamapps/libraryfolders.vdf"),
            &format!("\"libraryfolders\" {{ \"0\" {{ \"path\" \"{lib}\" }} }}"),
        )?;
        write(
            &library.join("steamapps/appmanifest_440.acf"),
            "\"AppState\" { \"appid\" \"440\" \"name\" \"Team Fortress 2\" \"installdir\" \"TF2\" }",
        )?;
        write(
            &library.join("steamapps/appmanifest_228980.acf"),
            "\"AppState\" { \"appid\" \"228980\" \"name\" \"Steamworks Common Redistributables\" }",
        )
    }

    #[test]
    fn steam_should_find_games_on_other_libraries() -> TestResult {
        let dir = tempfile::tempdir()?;
        let (root, library) = (dir.path().join("Steam"), dir.path().join("D"));
        steam_fixture(&root, &library)?;
        let found = scan_steam(&root)?;
        assert_eq!(
            found.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(),
            ["steam:440"]
        );
        Ok(())
    }

    #[test]
    fn steam_should_launch_by_protocol() -> TestResult {
        let dir = tempfile::tempdir()?;
        let (root, library) = (dir.path().join("Steam"), dir.path().join("D"));
        steam_fixture(&root, &library)?;
        assert_eq!(
            scan_steam(&root)?[0].target,
            LaunchTarget::Protocol {
                uri: "steam://rungameid/440".into()
            }
        );
        Ok(())
    }

    #[test]
    fn broken_steam_file_should_fail_cleanly() -> TestResult {
        let dir = tempfile::tempdir()?;
        write(&dir.path().join("steamapps/libraryfolders.vdf"), "\"broken")?;
        assert!(matches!(scan_steam(dir.path()), Err(ScanError::Format(_))));
        Ok(())
    }

    #[test]
    fn broken_manifest_should_not_hide_other_games() -> TestResult {
        let dir = tempfile::tempdir()?;
        let (root, library) = (dir.path().join("Steam"), dir.path().join("D"));
        steam_fixture(&root, &library)?;
        write(&library.join("steamapps/appmanifest_9.acf"), "garbage {")?;
        assert_eq!(scan_steam(&root)?.len(), 1);
        Ok(())
    }

    #[test]
    fn epic_should_keep_only_complete_games() -> TestResult {
        let dir = tempfile::tempdir()?;
        let game = r#"{"DisplayName":"Alan Wake","AppName":"abc","InstallLocation":"C:/AW","AppCategories":["public","games"]}"#;
        let engine = r#"{"DisplayName":"Unreal","AppName":"ue","InstallLocation":"C:/UE","AppCategories":["engines"]}"#;
        let partial = r#"{"DisplayName":"Part","AppName":"p","InstallLocation":"C:/P","AppCategories":["games"],"bIsIncompleteInstall":true}"#;
        write(&dir.path().join("a.item"), game)?;
        write(&dir.path().join("b.item"), engine)?;
        write(&dir.path().join("c.item"), partial)?;
        write(&dir.path().join("d.item"), "not json")?;
        let found = scan_epic(dir.path())?;
        assert_eq!(
            found.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(),
            ["epic:abc"]
        );
        Ok(())
    }

    #[test]
    fn missing_epic_folder_should_fail_cleanly() {
        assert!(matches!(
            scan_epic(Path::new("Z:/nope")),
            Err(ScanError::Io { .. })
        ));
    }

    #[test]
    fn gog_should_use_the_primary_task() -> TestResult {
        let dir = tempfile::tempdir()?;
        write(
            &dir.path().join("goggame-123.info"),
            r#"{"gameId":"123","name":"Witcher","playTasks":[{"isPrimary":true,"path":"bin/w.exe","arguments":"-a -b"}]}"#,
        )?;
        let found = scan_gog_dir(dir.path())?.ok_or("no game")?;
        assert!(
            matches!(found.target, LaunchTarget::Executable { ref args, .. } if args.len() == 2)
        );
        Ok(())
    }

    #[test]
    fn gog_dlc_without_primary_task_should_be_ignored() -> TestResult {
        let dir = tempfile::tempdir()?;
        write(
            &dir.path().join("goggame-9.info"),
            r#"{"gameId":"9","name":"DLC","playTasks":[]}"#,
        )?;
        assert_eq!(scan_gog_dir(dir.path())?, None);
        Ok(())
    }

    #[test]
    fn ubisoft_title_should_come_from_the_folder() {
        assert_eq!(
            ubisoft_entry("42", "D:\\Ubi\\Far Cry 6\\").title,
            "Far Cry 6"
        );
    }

    #[test]
    fn publisher_scan_should_skip_the_client_itself() -> TestResult {
        let dir = tempfile::tempdir()?;
        write(&dir.path().join("FIFA/FIFA.exe"), "x")?;
        let record = |name: &str| UninstallRecord {
            key: name.to_owned(),
            name: name.to_owned(),
            publisher: "Electronic Arts".to_owned(),
            install_location: dir.path().join("FIFA").display().to_string(),
        };
        let found = scan_publisher(
            &[record("FIFA"), record("EA app")],
            Store::Ea,
            "electronic arts",
            "ea",
        );
        assert_eq!(
            found.iter().map(|e| e.title.as_str()).collect::<Vec<_>>(),
            ["FIFA"]
        );
        Ok(())
    }

    #[test]
    fn xbox_game_should_launch_by_identity_never_by_exe() -> TestResult {
        let dir = tempfile::tempdir()?;
        write(
            &dir.path().join("Halo/Content/MicrosoftGame.config"),
            r#"<Game><Identity Name="Microsoft.Halo" Publisher="CN=x"/></Game>"#,
        )?;
        let apps = [StartApp {
            name: "Halo".into(),
            app_id: "Microsoft.Halo_8wekyb!Game".into(),
        }];
        assert!(matches!(
            scan_xbox(dir.path(), &apps)?[0].target,
            LaunchTarget::Packaged { .. }
        ));
        Ok(())
    }
}
