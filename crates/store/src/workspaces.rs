use std::path::{Path, PathBuf};

use acvc_domain::{Hash, Result, WorkspaceRegistry};
use rusqlite::params;

use crate::error::sql_err;
use crate::graph::Graph;

impl WorkspaceRegistry for Graph {
    fn register(&self, variant: &Hash, path: &Path) -> Result<()> {
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO workspaces(variant, path) VALUES (?1, ?2) ON CONFLICT(variant) DO UPDATE SET path = excluded.path",
            params![variant.to_hex(), path.to_string_lossy().to_string()],
        )
        .map_err(|err| sql_err(err, "register workspace"))?;
        Ok(())
    }

    fn find(&self, variant: &Hash) -> Result<Option<PathBuf>> {
        let conn = self.lock()?;
        match conn.query_row(
            "SELECT path FROM workspaces WHERE variant = ?1",
            [variant.to_hex()],
            |row| row.get::<_, String>(0),
        ) {
            Ok(path) => Ok(Some(PathBuf::from(path))),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(sql_err(err, "workspace")),
        }
    }
}
