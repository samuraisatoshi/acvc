use acvc_domain::{
    CampaignStore, Direction, MetricSpec, Result, validate_campaign_name, validate_schema,
};
use rusqlite::params;

use crate::error::sql_err;
use crate::graph::Graph;

impl CampaignStore for Graph {
    fn insert_campaign(&self, name: &str, schema: &[MetricSpec], now: i64) -> Result<()> {
        validate_campaign_name(name)?;
        validate_schema(schema)?;
        let conn = self.lock()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|err| sql_err(err, "campaign tx"))?;
        tx.execute(
            "INSERT INTO campaigns(name, created_at) VALUES (?1, ?2)",
            params![name, now],
        )
        .map_err(|err| campaign_conflict(err, name))?;
        for (ord, spec) in schema.iter().enumerate() {
            tx.execute(
                "INSERT INTO metric_specs(campaign, ord, name, direction) VALUES (?1, ?2, ?3, ?4)",
                params![name, ord as i64, spec.name, spec.direction as u8 as i64],
            )
            .map_err(|err| sql_err(err, "insert metric"))?;
        }
        tx.commit().map_err(|err| sql_err(err, "commit campaign"))?;
        Ok(())
    }

    fn list_campaigns(&self) -> Result<Vec<String>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT name FROM campaigns ORDER BY name")
            .map_err(|err| sql_err(err, "list campaigns"))?;
        let rows = stmt
            .query_map([], |row| row.get(0))
            .map_err(|err| sql_err(err, "list campaigns"))?;
        rows.collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "list campaigns"))
    }

    fn schema_of(&self, name: &str) -> Result<Vec<MetricSpec>> {
        let conn = self.lock()?;
        let found: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM campaigns WHERE name = ?1",
                [name],
                |row| row.get(0),
            )
            .map_err(|err| sql_err(err, "campaign lookup"))?;
        if found == 0 {
            return Err(acvc_domain::Error::new("campaign not found").ctx("campaign", name));
        }
        let mut stmt = conn
            .prepare("SELECT name, direction FROM metric_specs WHERE campaign = ?1 ORDER BY ord")
            .map_err(|err| sql_err(err, "schema"))?;
        let rows = stmt
            .query_map([name], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, u8>(1)?))
            })
            .map_err(|err| sql_err(err, "schema"))?;
        let pairs: Vec<(String, u8)> = rows
            .collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "schema"))?;
        pairs
            .into_iter()
            .map(|(metric, direction)| {
                Ok(MetricSpec {
                    name: metric,
                    direction: Direction::from_u8(direction)?,
                })
            })
            .collect()
    }

    fn opened_at(&self, name: &str) -> Result<i64> {
        let conn = self.lock()?;
        conn.query_row(
            "SELECT created_at FROM campaigns WHERE name = ?1",
            [name],
            |row| row.get(0),
        )
        .map_err(|err| match err {
            rusqlite::Error::QueryReturnedNoRows => {
                acvc_domain::Error::new("campaign not found").ctx("campaign", name)
            }
            other => sql_err(other, "campaign opened"),
        })
    }
}

fn campaign_conflict(err: rusqlite::Error, name: &str) -> acvc_domain::Error {
    let taken = matches!(
        &err,
        rusqlite::Error::SqliteFailure(code, _)
            if code.code == rusqlite::ErrorCode::ConstraintViolation
    );
    if taken {
        return acvc_domain::Error::new("campaign already exists").ctx("campaign", name);
    }
    sql_err(err, "insert campaign").ctx("campaign", name)
}
