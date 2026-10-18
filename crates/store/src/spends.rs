use acvc_domain::{Error, Result, Spend, SpendStore};
use rusqlite::params;

use crate::error::sql_err;
use crate::graph::Graph;

impl SpendStore for Graph {
    fn insert_spend(
        &self,
        campaign: &str,
        agent: &str,
        tokens: u64,
        note: &str,
        now: i64,
    ) -> Result<()> {
        let tokens = i64::try_from(tokens).map_err(|_| Error::new("token count is too large"))?;
        let conn = self.lock()?;
        conn.execute(
            "INSERT INTO spends(campaign, agent, tokens, note, created_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![campaign, agent, tokens, note, now],
        )
        .map_err(|err| sql_err(err, "insert spend").ctx("campaign", campaign))?;
        Ok(())
    }

    fn spends_of(&self, campaign: &str) -> Result<Vec<Spend>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare(
                "SELECT agent, tokens, note, created_at FROM spends WHERE campaign = ?1 ORDER BY created_at, id",
            )
            .map_err(|err| sql_err(err, "list spends"))?;
        let rows = stmt
            .query_map([campaign], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            })
            .map_err(|err| sql_err(err, "list spends"))?;
        let pairs: Vec<(String, i64, String, i64)> = rows
            .collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "list spends"))?;
        pairs
            .into_iter()
            .map(|(agent, tokens, note, created_at)| {
                Ok(Spend {
                    agent,
                    tokens: u64::try_from(tokens)
                        .map_err(|_| Error::new("stored token count is negative"))?,
                    note,
                    created_at,
                })
            })
            .collect()
    }
}
