//! Stockage SQLite de la bibliothèque.

use focus_core::model::{Entry, EntryKind};
use rusqlite::{params, Connection, OptionalExtension};
use std::path::Path;

/// Base de la bibliothèque.
pub struct Repository {
    conn: Connection,
}

/// Erreur de la base.
pub type RepoError = rusqlite::Error;

/// Résultat des opérations de base.
pub type RepoResult<T> = Result<T, RepoError>;

impl Repository {
    /// Ouvre ou crée la base.
    pub fn open(path: &Path) -> RepoResult<Self> {
        Self::init(Connection::open(path)?)
    }

    /// Base en mémoire, pour les tests.
    pub fn in_memory() -> RepoResult<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> RepoResult<Self> {
        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS entries (
                id TEXT PRIMARY KEY,
                json TEXT NOT NULL,
                source TEXT NOT NULL
             );
             CREATE TABLE IF NOT EXISTS settings (key TEXT PRIMARY KEY, value TEXT NOT NULL);",
        )?;
        Ok(Self { conn })
    }

    /// Ajoute ou remplace une entrée manuelle.
    pub fn save(&self, entry: &Entry) -> RepoResult<()> {
        self.put(entry, "manual")
    }

    fn put(&self, entry: &Entry, source: &str) -> RepoResult<()> {
        self.conn.execute(
            "INSERT INTO entries (id, json, source) VALUES (?1, ?2, ?3)
             ON CONFLICT(id) DO UPDATE SET json = excluded.json",
            params![entry.id, to_json(entry)?, source],
        )?;
        Ok(())
    }

    /// Met à jour une entrée existante sans changer son origine : un favori posé
    /// sur un jeu de boutique doit rester géré par la synchronisation.
    pub fn save_keeping_source(&self, entry: &Entry) -> RepoResult<()> {
        self.conn.execute(
            "UPDATE entries SET json = ?2 WHERE id = ?1",
            params![entry.id, to_json(entry)?],
        )?;
        Ok(())
    }

    /// Lit une entrée.
    pub fn get(&self, id: &str) -> RepoResult<Option<Entry>> {
        self.conn
            .query_row("SELECT json FROM entries WHERE id = ?1", [id], |row| {
                row.get::<_, String>(0)
            })
            .optional()?
            .map(|json| from_json(&json))
            .transpose()
    }

    /// Toutes les entrées, les plus récentes en premier.
    pub fn all(&self) -> RepoResult<Vec<Entry>> {
        let mut stmt = self.conn.prepare("SELECT json FROM entries")?;
        let rows = stmt.query_map([], |row| row.get::<_, String>(0))?;
        let mut entries = rows
            .map(|row| row.and_then(|json| from_json(&json)))
            .collect::<RepoResult<Vec<_>>>()?;
        entries.sort_by(|a, b| {
            b.stats
                .last_played
                .cmp(&a.stats.last_played)
                .then_with(|| a.title.to_lowercase().cmp(&b.title.to_lowercase()))
        });
        Ok(entries)
    }

    /// Supprime une entrée.
    pub fn delete(&self, id: &str) -> RepoResult<()> {
        self.conn
            .execute("DELETE FROM entries WHERE id = ?1", [id])?;
        Ok(())
    }

    /// Synchronise une boutique : ajoute les nouveautés, garde favoris et
    /// statistiques, retire les jeux désinstallés. Les entrées manuelles et les
    /// autres boutiques ne sont jamais touchées.
    pub fn sync_store(&mut self, prefix: &str, found: Vec<Entry>) -> RepoResult<()> {
        let source = format!("scan:{prefix}");
        let tx = self.conn.transaction()?;
        for mut entry in found.iter().cloned() {
            let previous = tx
                .query_row(
                    "SELECT json FROM entries WHERE id = ?1",
                    [&entry.id],
                    |row| row.get::<_, String>(0),
                )
                .optional()?;
            if let Some(json) = previous {
                let old = from_json(&json)?;
                entry.favorite = old.favorite;
                entry.stats = old.stats;
                entry.cover = old.cover.or(entry.cover);
                if old.kind != EntryKind::Game {
                    entry.kind = old.kind;
                }
            }
            tx.execute(
                "INSERT INTO entries (id, json, source) VALUES (?1, ?2, ?3)
                 ON CONFLICT(id) DO UPDATE SET json = excluded.json",
                params![entry.id, to_json(&entry)?, source],
            )?;
        }
        let existing: Vec<String> = {
            let mut stmt = tx.prepare("SELECT id FROM entries WHERE source = ?1")?;
            let rows = stmt.query_map([&source], |row| row.get(0))?;
            rows.collect::<RepoResult<_>>()?
        };
        for id in existing {
            if !found.iter().any(|entry| entry.id == id) {
                tx.execute("DELETE FROM entries WHERE id = ?1", [id])?;
            }
        }
        tx.commit()
    }

    /// Enregistre la fin d'une session.
    pub fn record_session(&self, id: &str, seconds: u64, ended_at: i64) -> RepoResult<()> {
        if let Some(mut entry) = self.get(id)? {
            entry.stats.total_seconds += seconds;
            entry.stats.last_played = Some(ended_at);
            entry.stats.launches += 1;
            self.conn.execute(
                "UPDATE entries SET json = ?2 WHERE id = ?1",
                params![entry.id, to_json(&entry)?],
            )?;
        }
        Ok(())
    }

    /// Lit un réglage.
    pub fn setting(&self, key: &str) -> RepoResult<Option<String>> {
        self.conn
            .query_row("SELECT value FROM settings WHERE key = ?1", [key], |row| {
                row.get(0)
            })
            .optional()
    }

    /// Écrit un réglage.
    pub fn set_setting(&self, key: &str, value: &str) -> RepoResult<()> {
        self.conn.execute(
            "INSERT INTO settings (key, value) VALUES (?1, ?2)
             ON CONFLICT(key) DO UPDATE SET value = excluded.value",
            params![key, value],
        )?;
        Ok(())
    }
}

fn to_json(entry: &Entry) -> RepoResult<String> {
    serde_json::to_string(entry)
        .map_err(|err| rusqlite::Error::ToSqlConversionFailure(Box::new(err)))
}

fn from_json(json: &str) -> RepoResult<Entry> {
    serde_json::from_str(json).map_err(|err| {
        rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Text, Box::new(err))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use focus_core::model::{LaunchTarget, Stats, Store};

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    fn game(id: &str) -> Entry {
        Entry {
            id: id.to_owned(),
            title: id.to_owned(),
            kind: EntryKind::Game,
            store: Store::Steam,
            target: LaunchTarget::Protocol {
                uri: "steam://x".into(),
            },
            install_dir: None,
            cover: None,
            favorite: false,
            stats: Stats::default(),
        }
    }

    #[test]
    fn saved_entry_should_read_back() -> TestResult {
        let repo = Repository::in_memory()?;
        repo.save(&game("a"))?;
        assert_eq!(repo.get("a")?, Some(game("a")));
        Ok(())
    }

    #[test]
    fn sync_should_keep_favorites_and_stats() -> TestResult {
        let mut repo = Repository::in_memory()?;
        repo.sync_store("steam", vec![game("steam:1")])?;
        repo.record_session("steam:1", 60, 10)?;
        let mut favorite = repo.get("steam:1")?.ok_or("missing")?;
        favorite.favorite = true;
        repo.save_keeping_source(&favorite)?;
        repo.sync_store("steam", vec![game("steam:1")])?;
        let kept = repo.get("steam:1")?.ok_or("missing")?;
        assert_eq!((kept.favorite, kept.stats.total_seconds), (true, 60));
        Ok(())
    }

    #[test]
    fn sync_should_remove_uninstalled_games() -> TestResult {
        let mut repo = Repository::in_memory()?;
        repo.sync_store("steam", vec![game("steam:1")])?;
        repo.sync_store("steam", vec![])?;
        assert_eq!(repo.get("steam:1")?, None);
        Ok(())
    }

    #[test]
    fn sync_should_never_remove_manual_entries_or_other_stores() -> TestResult {
        let mut repo = Repository::in_memory()?;
        repo.save(&game("manual:1"))?;
        repo.sync_store("epic", vec![game("epic:1")])?;
        repo.sync_store("steam", vec![])?;
        assert_eq!(repo.all()?.len(), 2);
        Ok(())
    }

    #[test]
    fn recent_entries_should_come_first() -> TestResult {
        let repo = Repository::in_memory()?;
        repo.save(&game("a"))?;
        repo.save(&game("b"))?;
        repo.record_session("b", 1, 5)?;
        assert_eq!(repo.all()?[0].id, "b");
        Ok(())
    }

    #[test]
    fn settings_should_persist() -> TestResult {
        let repo = Repository::in_memory()?;
        repo.set_setting("lang", "fr")?;
        repo.set_setting("lang", "ar")?;
        assert_eq!(repo.setting("lang")?.as_deref(), Some("ar"));
        Ok(())
    }
}
