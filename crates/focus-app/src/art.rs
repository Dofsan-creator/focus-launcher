//! Images de la bibliothèque : préparation en fond et service à l'interface.
//!
//! Les images sont rangées dans `<données>/art/<clé>/`. L'interface les demande
//! par le protocole `focusart` avec une clé et une sorte, jamais par un chemin :
//! rien d'autre du disque ne peut être lu par ce biais.

use crate::runtime::App;
use focus_core::art::{art_key, is_valid_key, ArtInfo, ArtKind, ART_VERSION};
use focus_core::model::{Entry, LaunchTarget, Store};
use focus_library::art::{self as library_art, Meta, Source, Sources};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use tauri::{AppHandle, Emitter};

/// Hôtes d'images autorisés : les CDN d'Epic, rien d'autre.
#[cfg(windows)]
const IMAGE_HOSTS: &[&str] = &[
    "cdn1.epicgames.com",
    "cdn2.unrealengine.com",
    "cdn1.unrealengine.com",
];
#[cfg(windows)]
const MAX_IMAGE_BYTES: u32 = 12 * 1024 * 1024;
/// Après un échec, nouvelle tentative le lendemain seulement.
const RETRY_AFTER: i64 = 24 * 3600;

static RUNNING: AtomicBool = AtomicBool::new(false);

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |elapsed| elapsed.as_secs() as i64)
}

fn art_root(data: &Path) -> PathBuf {
    data.join("art")
}

fn entry_dir(data: &Path, id: &str) -> PathBuf {
    art_root(data).join(art_key(id))
}

/// Ce que l'interface peut afficher pour une entrée.
pub fn info(data: &Path, id: &str) -> ArtInfo {
    library_art::read_info(&entry_dir(data, id))
}

fn sources_for(entry: &Entry, steam: Option<&Path>, epic: &HashMap<String, Sources>) -> Sources {
    match entry.store {
        Store::Steam => match (steam, entry.id.strip_prefix("steam:")) {
            (Some(root), Some(appid)) => library_art::steam_sources(root, appid),
            _ => Sources::default(),
        },
        Store::Epic => entry
            .id
            .strip_prefix("epic:")
            .and_then(|app| epic.get(app))
            .cloned()
            .unwrap_or_default(),
        _ => Sources::default(),
    }
}

/// Fichier dont on lit l'icône : la cible directe, sinon l'exécutable le plus
/// probable du dossier d'installation.
fn icon_file(entry: &Entry) -> Option<PathBuf> {
    if let LaunchTarget::Executable { path, .. } = &entry.target {
        let path = PathBuf::from(path);
        if path.is_file() {
            return Some(path);
        }
    }
    let dir = entry.install_dir.as_deref()?;
    focus_library::exe_score::best_candidates(
        Path::new(dir),
        focus_library::exe_score::DEFAULT_DEPTH,
        1,
        &|| false,
    )
    .into_iter()
    .next()
    .map(|candidate| candidate.path)
}

fn fetch(source: &Source) -> Option<Vec<u8>> {
    match source {
        Source::File(path) => std::fs::read(path).ok(),
        #[cfg(windows)]
        Source::Url(url) => match focus_sys::http::download(url, IMAGE_HOSTS, MAX_IMAGE_BYTES) {
            Ok(bytes) => Some(bytes),
            Err(err) => {
                tracing::warn!("image not downloaded: {err}");
                None
            }
        },
        #[cfg(not(windows))]
        Source::Url(_) => None,
    }
}

fn save(dir: &Path, kind: ArtKind, bytes: &[u8]) -> bool {
    std::fs::write(dir.join(kind.file_name()), bytes).is_ok()
}

/// Prépare les images d'une entrée. Vrai si quelque chose a changé.
fn prepare(data: &Path, entry: &Entry, steam: Option<&Path>, epic: &HashMap<String, Sources>) -> bool {
    let dir = entry_dir(data, &entry.id);
    let previous = library_art::read_meta(&dir);
    let up_to_date = |meta: &Meta| {
        meta.info.version == ART_VERSION
            && (meta.info.cover || now() - meta.attempted < RETRY_AFTER)
    };
    if previous.as_ref().is_some_and(up_to_date) {
        return false;
    }
    if std::fs::create_dir_all(&dir).is_err() {
        return false;
    }
    let sources = sources_for(entry, steam, epic);
    let mut info = ArtInfo {
        version: ART_VERSION,
        ..ArtInfo::default()
    };
    for kind in [ArtKind::Cover, ArtKind::Hero, ArtKind::Logo] {
        let Some(bytes) = sources.get(kind).and_then(fetch) else {
            continue;
        };
        let prepared = library_art::decode(&bytes)
            .and_then(|image| Ok((library_art::encode(kind, &image)?, image)));
        match prepared {
            Ok((encoded, image)) if save(&dir, kind, &encoded) => {
                info.set(kind, true);
                if info.accent.is_none() && kind != ArtKind::Logo {
                    info.accent = library_art::accent(&image);
                }
            }
            Ok(_) => {}
            Err(err) => tracing::warn!("{} {}: {err}", entry.id, kind.slug()),
        }
    }
    if let Some(file) = icon_file(entry) {
        #[cfg(windows)]
        match focus_sys::icon::file_icon(&file, 256) {
            Ok(icon) => {
                let image = library_art::from_rgba(icon.width, icon.height, icon.pixels);
                if let Some(image) = image {
                    if let Ok(encoded) = library_art::encode(ArtKind::Icon, &image) {
                        if save(&dir, ArtKind::Icon, &encoded) {
                            info.icon = true;
                            if info.accent.is_none() {
                                info.accent = library_art::accent(&image);
                            }
                        }
                    }
                }
            }
            Err(err) => tracing::warn!("icon of {}: {err}", entry.id),
        }
        #[cfg(not(windows))]
        let _ = file;
    }
    let changed = previous.is_none_or(|old| old.info != info);
    if let Err(err) = library_art::write_meta(&dir, &Meta { info, attempted: now() }) {
        tracing::warn!("art meta not written: {err}");
    }
    changed
}

/// Prépare les images manquantes de toute la bibliothèque. Tourne dans un fil de
/// fond ; un seul passage à la fois. L'interface est prévenue s'il y a du neuf.
pub fn refresh(state: &App, app: &AppHandle) {
    if RUNNING.swap(true, Ordering::SeqCst) {
        return;
    }
    let entries = state.repo().all().unwrap_or_default();
    let data = state.data_dir().to_path_buf();
    let steam = focus_library::discover::steam_root();
    let epic_data = focus_library::discover::program_data().join("Epic/EpicGamesLauncher/Data");
    let epic = library_art::epic_sources(
        &epic_data.join("Manifests"),
        &epic_data.join("Catalog/catcache.bin"),
    );
    let mut changed = false;
    for entry in &entries {
        changed |= prepare(&data, entry, steam.as_deref(), &epic);
    }
    RUNNING.store(false, Ordering::SeqCst);
    if changed {
        let _ = app.emit("library-changed", "art");
    }
}

/// Icône d’une boutique installée, lue dans son programme et gardée en cache.
pub fn store_icon(data: &Path, id: &str, exe: Option<PathBuf>) -> ArtInfo {
    let key = format!("store:{id}");
    let dir = entry_dir(data, &key);
    let info = library_art::read_info(&dir);
    if info.icon {
        return info;
    }
    tracing::info!("store icon {id}: {exe:?}");
    #[cfg(windows)]
    if let Some(exe) = exe.filter(|exe| exe.is_file()) {
        let icon = focus_sys::icon::file_icon(&exe, 256);
        if let Err(err) = &icon {
            tracing::warn!("store icon {id} not read: {err}");
        }
        if let Ok(icon) = icon {
            if let Some(image) = library_art::from_rgba(icon.width, icon.height, icon.pixels) {
                if let Ok(encoded) = library_art::encode(ArtKind::Icon, &image) {
                    if std::fs::create_dir_all(&dir).is_ok() && save(&dir, ArtKind::Icon, &encoded) {
                        let fresh = ArtInfo { icon: true, accent: library_art::accent(&image), version: ART_VERSION, ..ArtInfo::default() };
                        let _ = library_art::write_meta(&dir, &Meta { info: fresh.clone(), attempted: now() });
                        return fresh;
                    }
                }
            }
        }
    }
    #[cfg(not(windows))]
    let _ = exe;
    info
}

/// Répond à une demande `focusart` : `/<clé>/<sorte>`.
pub fn respond(data: &Path, path: &str) -> tauri::http::Response<Vec<u8>> {
    let mut parts = path.trim_start_matches('/').split('/');
    let key = parts.next().unwrap_or_default();
    let kind = parts.next().and_then(ArtKind::from_slug);
    let builder = tauri::http::Response::builder();
    let response = match kind {
        Some(kind) if is_valid_key(key) && parts.next().is_none() => {
            match std::fs::read(art_root(data).join(key).join(kind.file_name())) {
                Ok(bytes) => builder
                    .status(200)
                    .header("Content-Type", kind.mime())
                    .header("Cache-Control", "max-age=31536000, immutable")
                    .body(bytes),
                Err(_) => builder.status(404).body(Vec::new()),
            }
        }
        _ => builder.status(400).body(Vec::new()),
    };
    response.unwrap_or_else(|_| tauri::http::Response::new(Vec::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_known_keys_and_kinds_should_be_served() -> Result<(), Box<dyn std::error::Error>> {
        let data = tempfile::tempdir()?;
        let dir = entry_dir(data.path(), "steam:440");
        std::fs::create_dir_all(&dir)?;
        std::fs::write(dir.join("cover.jpg"), b"jpeg")?;
        let key = art_key("steam:440");
        let ok = respond(data.path(), &format!("/{key}/cover"));
        assert_eq!(ok.status(), 200);
        assert_eq!(ok.body(), b"jpeg");
        assert_eq!(respond(data.path(), &format!("/{key}/hero")).status(), 404);
        for hostile in ["/../secret/cover", "/a/b/cover", &format!("/{key}/../../x"), "/"] {
            assert_ne!(respond(data.path(), hostile).status(), 200, "{hostile}");
        }
        Ok(())
    }
}
