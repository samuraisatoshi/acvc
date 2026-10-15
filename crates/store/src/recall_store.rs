use acvc_domain::{Hash, RecallStore, Result};
use rusqlite::params;

use crate::error::sql_err;
use crate::graph::Graph;
use crate::recall::fts_query;

impl RecallStore for Graph {
    fn recall(&self, campaign: &str, query: &str) -> Result<Vec<Hash>> {
        let match_query = fts_query(query)?;
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT d.variant FROM recall_doc d JOIN recall_fts ON recall_fts.rowid = d.id WHERE recall_fts MATCH ?1 AND d.campaign = ?2 ORDER BY d.variant",
            )
            .map_err(|err| sql_err(err, "recall"))?;
        let rows = stmt
            .query_map(params![match_query, campaign], |row| {
                row.get::<_, String>(0)
            })
            .map_err(|err| sql_err(err, "recall"))?;
        let ids: Vec<String> = rows
            .collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "recall"))?;
        ids.iter().map(|hex| Hash::parse(hex)).collect()
    }
}
