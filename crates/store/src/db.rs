use std::fs;
use std::path::Path;
use std::sync::{Mutex, MutexGuard};

use acvc_domain::{Error, Result};
use rusqlite::Connection;

use crate::error::sql_err;
use crate::schema::SCHEMA;

pub(crate) struct Db {
    conn: Mutex<Connection>,
}

impl Db {
    pub(crate) fn open(path: &Path) -> Result<Self> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|err| crate::error::io_err(err, "create", parent))?;
        }
        let conn = Connection::open(path).map_err(|err| sql_err(err, "open sqlite"))?;
        conn.execute_batch(
            "PRAGMA journal_mode=WAL; PRAGMA foreign_keys=ON; PRAGMA synchronous=NORMAL;",
        )
        .map_err(|err| sql_err(err, "pragma"))?;
        conn.execute_batch(SCHEMA)
            .map_err(|err| sql_err(err, "schema"))?;
        migrate(&conn)?;
        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub(crate) fn lock(&self) -> Result<MutexGuard<'_, Connection>> {
        self.conn
            .lock()
            .map_err(|_| Error::new("sqlite lock poisoned"))
    }
}

pub(crate) fn meta_get(conn: &Connection, key: &str) -> Result<Option<String>> {
    conn.query_row("SELECT value FROM meta WHERE key = ?1", [key], |row| {
        row.get(0)
    })
    .optional()
    .map_err(|err| sql_err(err, "meta get"))
}

pub(crate) fn meta_set(conn: &Connection, key: &str, value: &str) -> Result<()> {
    conn.execute(
        "INSERT INTO meta(key, value) VALUES (?1, ?2) ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        [key, value],
    )
    .map_err(|err| sql_err(err, "meta set"))?;
    Ok(())
}

fn migrate(conn: &Connection) -> Result<()> {
    for sql in [
        "ALTER TABLE objects ADD COLUMN loc INTEGER NOT NULL DEFAULT 0",
        "ALTER TABLE objects ADD COLUMN clen INTEGER NOT NULL DEFAULT 0",
    ] {
        if let Err(err) = conn.execute(sql, [])
            && !err.to_string().contains("duplicate column")
        {
            return Err(sql_err(err, "migrate objects"));
        }
    }
    Ok(())
}

trait OptionalRow {
    fn optional(self) -> rusqlite::Result<Option<String>>;
}

impl OptionalRow for rusqlite::Result<String> {
    fn optional(self) -> rusqlite::Result<Option<String>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }
}
