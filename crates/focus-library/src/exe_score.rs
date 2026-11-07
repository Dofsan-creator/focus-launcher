//! Propose les exécutables les plus probables d'un dossier. Rien n'est exécuté.

use std::fs;
use std::path::{Path, PathBuf};

/// Profondeur de recherche par défaut.
pub const DEFAULT_DEPTH: usize = 3;

/// Préfixes pénalisés, jamais exclus : certains jeux s'appellent `Launcher.exe`.
const PENALIZED: &[&str] = &[
    "unins",
    "setup",
    "install",
    "vc_redist",
    "vcredist",
    "dxsetup",
    "unitycrashhandler",
    "crashreportclient",
    "crashpad",
    "easyanticheat",
    "beservice",
    "dotnet",
];

/// Un candidat noté.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Candidate {
    /// Chemin de l'exécutable.
    pub path: PathBuf,
    /// Score, plus haut est meilleur.
    pub score: i64,
    /// Taille en octets.
    pub size: u64,
}

/// Renvoie les `limit` meilleurs exécutables de `root`.
/// `cancelled` est consulté à chaque dossier pour rendre la recherche annulable.
pub fn best_candidates(
    root: &Path,
    depth: usize,
    limit: usize,
    cancelled: &dyn Fn() -> bool,
) -> Vec<Candidate> {
    let mut files = Vec::new();
    collect(root, depth, cancelled, &mut files);
    let largest = files.iter().map(|(_, size)| *size).max().unwrap_or(0);
    let folder = root.file_name().map(lossy).unwrap_or_default();
    let mut candidates: Vec<Candidate> = files
        .into_iter()
        .map(|(path, size)| Candidate {
            score: score(&path, size, largest, &folder),
            path,
            size,
        })
        .collect();
    candidates.sort_by(|a, b| b.score.cmp(&a.score).then_with(|| a.path.cmp(&b.path)));
    candidates.truncate(limit);
    candidates
}

fn lossy(name: &std::ffi::OsStr) -> String {
    name.to_string_lossy().to_lowercase()
}

fn collect(dir: &Path, depth: usize, cancelled: &dyn Fn() -> bool, out: &mut Vec<(PathBuf, u64)>) {
    if cancelled() || is_protected_dir(dir) {
        return;
    }
    let Ok(entries) = fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(meta) = entry.metadata() else { continue };
        if meta.is_dir() {
            if depth > 0 {
                collect(&path, depth - 1, cancelled, out);
            }
        } else if path
            .extension()
            .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
        {
            out.push((path, meta.len()));
        }
    }
}

/// Le dossier protégé `WindowsApps` n'est jamais parcouru.
pub fn is_protected_dir(dir: &Path) -> bool {
    dir.components()
        .any(|part| part.as_os_str().eq_ignore_ascii_case("WindowsApps"))
}

fn score(path: &Path, size: u64, largest: u64, folder: &str) -> i64 {
    let stem = path.file_stem().map(lossy).unwrap_or_default();
    let mut score = 0;
    if squash(&stem) == squash(folder) {
        score += 50;
    } else if !folder.is_empty()
        && (squash(folder).contains(&squash(&stem)) || squash(&stem).contains(&squash(folder)))
    {
        score += 25;
    }
    if size == largest {
        score += 30;
    }
    let parent = path.parent();
    if let Some(parent) = parent {
        if parent.join(format!("{stem}_Data")).is_dir() {
            score += 40; // Unity
        }
        let text = parent.to_string_lossy().to_lowercase().replace('/', "\\");
        if text.ends_with("binaries\\win64") {
            score += 30; // Unreal
        }
        if text.ends_with("\\bin") || text.ends_with("\\x64") || text.ends_with("\\win64") {
            score += 10;
        }
    }
    if PENALIZED.iter().any(|prefix| stem.starts_with(prefix)) {
        score -= 100;
    }
    score
}

fn squash(text: &str) -> String {
    text.chars().filter(|c| c.is_alphanumeric()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn touch(path: &Path, size: usize) -> std::io::Result<()> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(path, vec![0_u8; size])
    }

    fn names(root: &Path, limit: usize) -> Vec<String> {
        best_candidates(root, DEFAULT_DEPTH, limit, &|| false)
            .iter()
            .filter_map(|c| c.path.file_name().map(|n| n.to_string_lossy().into_owned()))
            .collect()
    }

    #[test]
    fn name_matching_the_folder_should_beat_a_bigger_uninstaller() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("Hades");
        touch(&root.join("Hades.exe"), 100)?;
        touch(&root.join("unins000.exe"), 500)?;
        assert_eq!(names(&root, 1), ["Hades.exe"]);
        Ok(())
    }

    #[test]
    fn unity_layout_should_win() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("Some Folder");
        touch(&root.join("Game.exe"), 10)?;
        touch(&root.join("Game_Data/x"), 1)?;
        touch(&root.join("Tool.exe"), 20)?;
        assert_eq!(names(&root, 1), ["Game.exe"]);
        Ok(())
    }

    #[test]
    fn unreal_layout_should_be_found_in_depth() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("Game");
        touch(
            &root.join("Game/Binaries/Win64/Shooter-Win64-Shipping.exe"),
            100,
        )?;
        touch(&root.join("Engine.exe"), 10)?;
        assert_eq!(names(&root, 1), ["Shooter-Win64-Shipping.exe"]);
        Ok(())
    }

    #[test]
    fn launcher_named_game_should_stay_a_candidate() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("X");
        touch(&root.join("setup.exe"), 10)?;
        assert_eq!(names(&root, 3), ["setup.exe"]);
        Ok(())
    }

    #[test]
    fn windows_apps_should_never_be_walked() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("WindowsApps");
        touch(&root.join("Game.exe"), 10)?;
        assert!(names(&root, 3).is_empty());
        Ok(())
    }

    #[test]
    fn search_should_stop_when_cancelled() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        touch(&dir.path().join("a.exe"), 1)?;
        assert!(best_candidates(dir.path(), 3, 3, &|| true).is_empty());
        Ok(())
    }

    #[test]
    fn accented_and_long_paths_should_work() -> std::io::Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().join("Jeu éléphant 日本語");
        touch(&root.join("Lancé.exe"), 1)?;
        assert_eq!(names(&root, 1), ["Lancé.exe"]);
        Ok(())
    }
}
