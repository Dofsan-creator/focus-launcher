//! Sources des images d'une entrée, en lecture seule, et préparation pour l'écran.
//!
//! Steam garde jaquettes, fonds et logos sur le disque. Epic ne garde que les
//! adresses de ses images (`catcache.bin`) : le téléchargement se fait ailleurs,
//! vers une liste d'hôtes fixée. Chaque image est réduite et réencodée une seule
//! fois, pour que l'interface décode peu et reste fluide.

use base64::Engine;
use focus_core::art::{ArtInfo, ArtKind, ART_VERSION};
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use thiserror::Error;

/// Erreur de préparation d'une image : l'entrée garde alors son icône ou sa couleur.
#[derive(Debug, Error)]
pub enum ArtError {
    /// Fichier.
    #[error(transparent)]
    Io(#[from] std::io::Error),
    /// Image illisible ou impossible à encoder.
    #[error("image: {0}")]
    Image(String),
}

impl From<image::ImageError> for ArtError {
    fn from(err: image::ImageError) -> Self {
        Self::Image(err.to_string())
    }
}

/// D'où vient une image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Source {
    /// Fichier local de la boutique, copié et réduit.
    File(PathBuf),
    /// Adresse HTTPS d'une boutique, téléchargée par l'application.
    Url(String),
}

/// Sources trouvées pour une entrée. L'icône vient toujours de l'exécutable.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Sources {
    /// Jaquette verticale.
    pub cover: Option<Source>,
    /// Image de fond.
    pub hero: Option<Source>,
    /// Logo.
    pub logo: Option<Source>,
}

impl Sources {
    /// Source d'une sorte d'image.
    pub fn get(&self, kind: ArtKind) -> Option<&Source> {
        match kind {
            ArtKind::Cover => self.cover.as_ref(),
            ArtKind::Hero => self.hero.as_ref(),
            ArtKind::Logo => self.logo.as_ref(),
            ArtKind::Icon => None,
        }
    }
}

fn collect_files(dir: &Path, depth: usize, out: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            if depth > 0 {
                collect_files(&path, depth - 1, out);
            }
        } else {
            out.push(path);
        }
    }
}

/// Images locales d'un jeu Steam, dans `appcache/librarycache`.
pub fn steam_sources(steam_root: &Path, appid: &str) -> Sources {
    let cache = steam_root.join("appcache").join("librarycache");
    let mut files = Vec::new();
    // Format récent : un dossier par jeu, images rangées dans des sous-dossiers.
    collect_files(&cache.join(appid), 2, &mut files);
    let find = |names: &[&str]| -> Option<Source> {
        names.iter().find_map(|name| {
            let nested = files.iter().find(|path| {
                path.file_name()
                    .is_some_and(|file| file.to_string_lossy().eq_ignore_ascii_case(name))
            });
            // Ancien format : fichiers à plat préfixés par le numéro du jeu.
            let flat = cache.join(format!("{appid}_{name}"));
            nested
                .cloned()
                .or_else(|| flat.is_file().then_some(flat))
                .map(Source::File)
        })
    };
    Sources {
        cover: find(&[
            "library_capsule.jpg",
            "library_600x900_2x.jpg",
            "library_600x900.jpg",
        ]),
        hero: find(&["library_hero.jpg", "library_hero_2x.jpg", "library_header.jpg", "header.jpg"]),
        logo: find(&["logo.png", "logo_2x.png"]),
    }
}

#[derive(Deserialize)]
struct EpicManifest {
    #[serde(rename = "AppName", default)]
    app_name: String,
    #[serde(rename = "CatalogItemId", default)]
    catalog_item_id: String,
}

#[derive(Deserialize)]
struct CatalogItem {
    #[serde(default)]
    id: String,
    #[serde(rename = "keyImages", default)]
    key_images: Vec<KeyImage>,
}

#[derive(Deserialize)]
struct KeyImage {
    #[serde(rename = "type", default)]
    kind: String,
    #[serde(default)]
    url: String,
}

fn pick(images: &[KeyImage], kinds: &[&str]) -> Option<Source> {
    kinds.iter().find_map(|kind| {
        images
            .iter()
            .find(|image| image.kind == *kind && image.url.starts_with("https://"))
            .map(|image| Source::Url(image.url.clone()))
    })
}

/// Adresses des images Epic, par `AppName`, lues dans les manifestes et le
/// cache du catalogue. Rien n'est téléchargé ici.
pub fn epic_sources(manifests: &Path, catcache: &Path) -> HashMap<String, Sources> {
    let mut apps_by_item: HashMap<String, Vec<String>> = HashMap::new();
    if let Ok(entries) = std::fs::read_dir(manifests) {
        for entry in entries.flatten() {
            let path = entry.path();
            if !path.extension().is_some_and(|ext| ext.eq_ignore_ascii_case("item")) {
                continue;
            }
            let Ok(text) = std::fs::read_to_string(&path) else {
                continue;
            };
            if let Ok(manifest) = serde_json::from_str::<EpicManifest>(&text) {
                if !manifest.app_name.is_empty() && !manifest.catalog_item_id.is_empty() {
                    apps_by_item
                        .entry(manifest.catalog_item_id)
                        .or_default()
                        .push(manifest.app_name);
                }
            }
        }
    }
    let mut found = HashMap::new();
    let Ok(raw) = std::fs::read(catcache) else {
        return found;
    };
    let compact: Vec<u8> = raw.into_iter().filter(|b| !b.is_ascii_whitespace()).collect();
    let Ok(json) = base64::engine::general_purpose::STANDARD.decode(compact) else {
        return found;
    };
    let Ok(items) = serde_json::from_slice::<Vec<CatalogItem>>(&json) else {
        return found;
    };
    for item in items {
        let Some(apps) = apps_by_item.get(&item.id) else {
            continue;
        };
        let sources = Sources {
            cover: pick(
                &item.key_images,
                &["DieselGameBoxTall", "OfferImageTall", "Thumbnail"],
            ),
            hero: pick(
                &item.key_images,
                &["DieselGameBox", "OfferImageWide", "DieselStoreFrontWide", "Featured"],
            ),
            logo: pick(&item.key_images, &["DieselGameBoxLogo", "ProductLogo"]),
        };
        for app in apps {
            found.insert(app.clone(), sources.clone());
        }
    }
    found
}

/// Taille maximale d'une sorte d'image : assez pour un écran 1080p, pas plus.
fn bounds(kind: ArtKind) -> (u32, u32) {
    match kind {
        ArtKind::Cover => (480, 720),
        ArtKind::Hero => (1920, 1080),
        ArtKind::Logo => (800, 400),
        ArtKind::Icon => (256, 256),
    }
}

/// Réduit et réencode une image décodée.
pub fn encode(kind: ArtKind, image: &DynamicImage) -> Result<Vec<u8>, ArtError> {
    let (max_w, max_h) = bounds(kind);
    let (w, h) = image.dimensions();
    let resized = if w > max_w || h > max_h {
        image.resize(max_w, max_h, FilterType::CatmullRom)
    } else {
        image.clone()
    };
    let mut out = Vec::new();
    match kind {
        ArtKind::Cover | ArtKind::Hero => {
            let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut out, 86);
            DynamicImage::ImageRgb8(resized.to_rgb8()).write_with_encoder(encoder)?;
        }
        ArtKind::Logo | ArtKind::Icon => {
            let encoder = image::codecs::png::PngEncoder::new(&mut out);
            DynamicImage::ImageRgba8(resized.to_rgba8()).write_with_encoder(encoder)?;
        }
    }
    Ok(out)
}

/// Image depuis des pixels RGBA (icône lue par le shell).
pub fn from_rgba(width: u32, height: u32, pixels: Vec<u8>) -> Option<DynamicImage> {
    image::RgbaImage::from_raw(width, height, pixels).map(DynamicImage::ImageRgba8)
}

/// Encode une image RGBA en PNG, sans réduction (captures d’écran).
pub fn png(width: u32, height: u32, pixels: Vec<u8>) -> Option<Vec<u8>> {
    let image = from_rgba(width, height, pixels)?;
    let mut out = Vec::new();
    image.write_with_encoder(image::codecs::png::PngEncoder::new(&mut out)).ok()?;
    Some(out)
}

/// Décode des octets d'image.
pub fn decode(bytes: &[u8]) -> Result<DynamicImage, ArtError> {
    Ok(image::load_from_memory(bytes)?)
}

fn hsv(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;
    let hue = if delta <= f32::EPSILON {
        0.0
    } else if (max - r).abs() <= f32::EPSILON {
        60.0 * ((g - b) / delta).rem_euclid(6.0)
    } else if (max - g).abs() <= f32::EPSILON {
        60.0 * ((b - r) / delta + 2.0)
    } else {
        60.0 * ((r - g) / delta + 4.0)
    };
    let saturation = if max <= f32::EPSILON { 0.0 } else { delta / max };
    (hue, saturation, max)
}

fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h / 60.0).rem_euclid(2.0) - 1.0).abs());
    let m = l - c / 2.0;
    let (r, g, b) = match (h / 60.0) as u32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };
    let byte = |v: f32| ((v + m).clamp(0.0, 1.0) * 255.0).round() as u8;
    (byte(r), byte(g), byte(b))
}

/// Couleur dominante d'une image, ajustée pour rester lisible sur fond sombre.
/// `None` pour une image presque grise : l'interface garde alors son accent.
pub fn accent(image: &DynamicImage) -> Option<String> {
    let small = image.thumbnail(48, 48).to_rgba8();
    // 24 tranches de teinte, pondérées par saturation et luminosité.
    let mut bins = [0.0_f32; 24];
    let mut hues = [0.0_f32; 24];
    let mut total = 0.0_f32;
    for pixel in small.pixels() {
        let [r, g, b, a] = pixel.0;
        if a < 128 {
            continue;
        }
        let (h, s, v) = hsv(
            f32::from(r) / 255.0,
            f32::from(g) / 255.0,
            f32::from(b) / 255.0,
        );
        if s < 0.2 || v < 0.2 {
            continue;
        }
        let weight = s * v;
        let bin = ((h / 15.0) as usize).min(23);
        bins[bin] += weight;
        hues[bin] += h * weight;
        total += weight;
    }
    let (best, weight) = bins
        .iter()
        .copied()
        .enumerate()
        .max_by(|a, b| a.1.total_cmp(&b.1))?;
    let pixels = small.width() * small.height();
    if weight <= 0.0 || total < pixels as f32 * 0.02 {
        return None;
    }
    let hue = hues[best] / weight;
    let (r, g, b) = hsl_to_rgb(hue, 0.7, 0.58);
    Some(format!("#{r:02x}{g:02x}{b:02x}"))
}

/// Description enregistrée à côté des images.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Meta {
    /// Ce qui existe.
    #[serde(flatten)]
    pub info: ArtInfo,
    /// Dernière tentative, en secondes Unix : évite de retélécharger sans cesse.
    pub attempted: i64,
}

const META_FILE: &str = "meta.json";

/// Lit la description d'un dossier d'images.
pub fn read_meta(dir: &Path) -> Option<Meta> {
    let text = std::fs::read_to_string(dir.join(META_FILE)).ok()?;
    serde_json::from_str(&text).ok()
}

/// Écrit la description d'un dossier d'images.
pub fn write_meta(dir: &Path, meta: &Meta) -> Result<(), ArtError> {
    let text = serde_json::to_string(meta).map_err(|e| ArtError::Image(e.to_string()))?;
    std::fs::write(dir.join(META_FILE), text)?;
    Ok(())
}

/// Ce que l'interface peut afficher : seules les images présentes sur le disque
/// et du format courant comptent.
pub fn read_info(dir: &Path) -> ArtInfo {
    let Some(meta) = read_meta(dir) else {
        return ArtInfo::default();
    };
    if meta.info.version != ART_VERSION {
        return ArtInfo::default();
    }
    let mut info = meta.info;
    for kind in ArtKind::ALL {
        if info.has(kind) && !dir.join(kind.file_name()).is_file() {
            info.set(kind, false);
        }
    }
    info
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{Rgb, RgbImage};
    use tempfile::tempdir;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn png(width: u32, height: u32, color: [u8; 3]) -> Result<Vec<u8>, ArtError> {
        let image = DynamicImage::ImageRgb8(RgbImage::from_pixel(width, height, Rgb(color)));
        let mut out = Vec::new();
        image.write_with_encoder(image::codecs::png::PngEncoder::new(&mut out))?;
        Ok(out)
    }

    #[test]
    fn steam_should_find_art_in_the_new_cache_layout() -> TestResult {
        let root = tempdir()?;
        let game = root.path().join("appcache/librarycache/1449850");
        std::fs::create_dir_all(game.join("aa"))?;
        std::fs::create_dir_all(game.join("bb"))?;
        std::fs::write(game.join("aa/library_capsule.jpg"), b"x")?;
        std::fs::write(game.join("bb/library_hero.jpg"), b"x")?;
        std::fs::write(game.join("bb/logo.png"), b"x")?;
        let found = steam_sources(root.path(), "1449850");
        assert_eq!(found.cover, Some(Source::File(game.join("aa/library_capsule.jpg"))));
        assert_eq!(found.hero, Some(Source::File(game.join("bb/library_hero.jpg"))));
        assert_eq!(found.logo, Some(Source::File(game.join("bb/logo.png"))));
        Ok(())
    }

    #[test]
    fn steam_should_fall_back_to_the_old_flat_layout() -> TestResult {
        let root = tempdir()?;
        let cache = root.path().join("appcache/librarycache");
        std::fs::create_dir_all(&cache)?;
        std::fs::write(cache.join("440_library_600x900.jpg"), b"x")?;
        let found = steam_sources(root.path(), "440");
        assert_eq!(found.cover, Some(Source::File(cache.join("440_library_600x900.jpg"))));
        assert_eq!(found.hero, None);
        Ok(())
    }

    #[test]
    fn epic_should_map_app_names_to_https_images() -> TestResult {
        let dir = tempdir()?;
        let manifests = dir.path().join("Manifests");
        std::fs::create_dir_all(&manifests)?;
        std::fs::write(
            manifests.join("A.item"),
            r#"{"AppName":"Fortnite","CatalogItemId":"item1","DisplayName":"Fortnite"}"#,
        )?;
        let catalog = r#"[{"id":"item1","keyImages":[
            {"type":"DieselGameBoxTall","url":"https://cdn1.epicgames.com/tall.jpg"},
            {"type":"DieselGameBox","url":"https://cdn1.epicgames.com/wide.jpg"},
            {"type":"DieselGameBoxLogo","url":"http://insecure.example/logo.png"}]},
            {"id":"other","keyImages":[]}]"#;
        let catcache = dir.path().join("catcache.bin");
        std::fs::write(&catcache, base64::engine::general_purpose::STANDARD.encode(catalog))?;
        let found = epic_sources(&manifests, &catcache);
        let fortnite = found.get("Fortnite").ok_or("missing")?;
        assert_eq!(
            fortnite.cover,
            Some(Source::Url("https://cdn1.epicgames.com/tall.jpg".into()))
        );
        assert_eq!(
            fortnite.hero,
            Some(Source::Url("https://cdn1.epicgames.com/wide.jpg".into()))
        );
        // Une adresse non chiffrée n'est jamais retenue.
        assert_eq!(fortnite.logo, None);
        Ok(())
    }

    #[test]
    fn missing_epic_files_should_give_nothing_without_failing() {
        let found = epic_sources(Path::new("Z:/nope"), Path::new("Z:/nope.bin"));
        assert!(found.is_empty());
    }

    #[test]
    fn big_cover_should_be_reduced_within_bounds_keeping_its_shape() -> TestResult {
        let image = decode(&png(1200, 1600, [200, 40, 40])?)?;
        let out = decode(&encode(ArtKind::Cover, &image)?)?;
        assert!(out.width() <= 480 && out.height() <= 720);
        assert_eq!(out.width() * 4, out.height() * 3);
        Ok(())
    }

    #[test]
    fn accent_should_follow_the_dominant_colour() -> TestResult {
        let red = decode(&png(64, 64, [220, 30, 30])?)?;
        let accent = accent(&red).ok_or("no accent")?;
        assert!(accent.starts_with('#'), "{accent}");
        let grey = decode(&png(64, 64, [128, 128, 128])?)?;
        assert_eq!(super::accent(&grey), None);
        Ok(())
    }

    #[test]
    fn info_should_forget_files_that_disappeared() -> TestResult {
        let dir = tempdir()?;
        let mut meta = Meta::default();
        meta.info.version = ART_VERSION;
        meta.info.cover = true;
        meta.info.icon = true;
        write_meta(dir.path(), &meta)?;
        std::fs::write(dir.path().join("icon.png"), b"x")?;
        let info = read_info(dir.path());
        assert!(!info.cover);
        assert!(info.icon);
        Ok(())
    }

    #[test]
    fn old_cache_format_should_be_ignored() -> TestResult {
        let dir = tempdir()?;
        let mut meta = Meta::default();
        meta.info.version = ART_VERSION - 1;
        meta.info.icon = true;
        write_meta(dir.path(), &meta)?;
        std::fs::write(dir.path().join("icon.png"), b"x")?;
        assert_eq!(read_info(dir.path()), ArtInfo::default());
        Ok(())
    }
}
