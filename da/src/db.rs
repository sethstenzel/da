use anyhow::{Context, Result};
use rusqlite::{Connection, params};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

pub struct Db {
    conn: Connection,
}

#[derive(Serialize, Deserialize)]
pub struct Alias {
    pub name: String,
    pub path: String,
}

#[derive(Serialize, Deserialize)]
pub struct Command {
    pub name: String,
    pub executable: String,
}

impl Db {
    pub fn open() -> Result<Self> {
        let path = db_path();
        let conn = Connection::open(&path)
            .with_context(|| format!("failed to open database at {}", path.display()))?;

        conn.execute_batch(SCHEMA).context("failed to initialize schema")?;

        Ok(Self { conn })
    }

    // --- aliases ---

    pub fn add(&self, alias: &str, path: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO aliases (alias, path) VALUES (?1, ?2)",
                params![alias, path],
            )
            .context("failed to insert alias")?;
        Ok(())
    }

    pub fn get(&self, alias: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT path FROM aliases WHERE alias = ?1")?;
        let mut rows = stmt.query(params![alias])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self, alias: &str) -> Result<bool> {
        let changed = self
            .conn
            .execute("DELETE FROM aliases WHERE alias = ?1", params![alias])?;
        Ok(changed > 0)
    }

    pub fn list(&self) -> Result<Vec<Alias>> {
        let mut stmt = self
            .conn
            .prepare("SELECT alias, path FROM aliases ORDER BY alias")?;
        let rows = stmt.query_map([], |row| {
            Ok(Alias {
                name: row.get(0)?,
                path: row.get(1)?,
            })
        })?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn search(&self, text: &str) -> Result<Vec<Alias>> {
        let pattern = format!("%{text}%");
        let mut stmt = self.conn.prepare(
            "SELECT alias, path FROM aliases WHERE alias LIKE ?1 ORDER BY alias",
        )?;
        let rows = stmt.query_map(params![pattern], |row| {
            Ok(Alias {
                name: row.get(0)?,
                path: row.get(1)?,
            })
        })?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    // --- open commands ---

    pub fn add_command(&self, name: &str, executable: &str) -> Result<()> {
        self.conn
            .execute(
                "INSERT OR REPLACE INTO commands (name, executable) VALUES (?1, ?2)",
                params![name, executable],
            )
            .context("failed to insert command")?;
        Ok(())
    }

    pub fn get_command(&self, name: &str) -> Result<Option<String>> {
        let mut stmt = self
            .conn
            .prepare("SELECT executable FROM commands WHERE name = ?1")?;
        let mut rows = stmt.query(params![name])?;
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }

    pub fn list_commands(&self) -> Result<Vec<Command>> {
        let mut stmt = self
            .conn
            .prepare("SELECT name, executable FROM commands ORDER BY name")?;
        let rows = stmt.query_map([], |row| {
            Ok(Command {
                name: row.get(0)?,
                executable: row.get(1)?,
            })
        })?;
        rows.map(|r| r.map_err(Into::into)).collect()
    }

    pub fn delete_command(&self, name: &str) -> Result<bool> {
        let changed = self
            .conn
            .execute("DELETE FROM commands WHERE name = ?1", params![name])?;
        Ok(changed > 0)
    }
}

const SCHEMA: &str = "
    CREATE TABLE IF NOT EXISTS aliases (
        alias TEXT PRIMARY KEY NOT NULL,
        path  TEXT NOT NULL
    );
    CREATE TABLE IF NOT EXISTS commands (
        name       TEXT PRIMARY KEY NOT NULL,
        executable TEXT NOT NULL
    );
    INSERT OR IGNORE INTO commands (name, executable) VALUES ('e',    'explorer');
    INSERT OR IGNORE INTO commands (name, executable) VALUES ('code', 'code');
    INSERT OR IGNORE INTO commands (name, executable) VALUES ('nvim', 'nvim');
";

#[cfg(test)]
impl Db {
    pub fn open_in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA).context("failed to initialize schema")?;
        Ok(Self { conn })
    }
}

fn db_path() -> PathBuf {
    let mut path = dirs::data_local_dir().unwrap_or_else(|| PathBuf::from("."));
    path.push("da");
    std::fs::create_dir_all(&path).ok();
    path.push("a.db");
    path
}

#[cfg(test)]
mod tests {
    use super::*;

    fn db() -> Db {
        Db::open_in_memory().unwrap()
    }

    // --- add command ---

    #[test]
    fn add_command_and_retrieve() {
        let db = db();
        db.add_command("rider", "rider64").unwrap();
        assert_eq!(db.get_command("rider").unwrap(), Some("rider64".to_string()));
    }

    #[test]
    fn add_command_overwrites_existing() {
        let db = db();
        db.add_command("code", "code-insiders").unwrap();
        assert_eq!(db.get_command("code").unwrap(), Some("code-insiders".to_string()));
    }

    #[test]
    fn get_command_returns_none_when_not_found() {
        let db = db();
        assert_eq!(db.get_command("nonexistent").unwrap(), None);
    }

    // --- remove command ---

    #[test]
    fn delete_command_removes_it() {
        let db = db();
        db.add_command("rider", "rider64").unwrap();
        assert!(db.delete_command("rider").unwrap());
        assert_eq!(db.get_command("rider").unwrap(), None);
    }

    #[test]
    fn delete_command_returns_false_when_not_found() {
        let db = db();
        assert!(!db.delete_command("nonexistent").unwrap());
    }

    #[test]
    fn deleted_command_absent_from_list() {
        let db = db();
        db.add_command("rider", "rider64").unwrap();
        db.delete_command("rider").unwrap();
        let names: Vec<_> = db.list_commands().unwrap().into_iter().map(|c| c.name).collect();
        assert!(!names.contains(&"rider".to_string()));
    }

    // --- list commands ---

    #[test]
    fn list_commands_includes_defaults() {
        let db = db();
        let names: Vec<_> = db.list_commands().unwrap().into_iter().map(|c| c.name).collect();
        for expected in ["e", "code", "nvim"] {
            assert!(names.contains(&expected.to_string()), "missing default command: {expected}");
        }
    }

    #[test]
    fn list_commands_includes_newly_added() {
        let db = db();
        db.add_command("rider", "rider64").unwrap();
        let cmds = db.list_commands().unwrap();
        let found = cmds.iter().any(|c| c.name == "rider" && c.executable == "rider64");
        assert!(found);
    }
}
