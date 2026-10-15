use std::collections::BTreeMap;

use acvc_domain::{Error, Hash, Result, StoredTrial, Trial, TrialStore};
use rusqlite::{Connection, params};

use crate::error::sql_err;
use crate::graph::Graph;
use crate::recall::{body_of, upsert_recall};
use crate::variants::load_variant;

impl TrialStore for Graph {
    fn insert_trial(&self, trial: &Trial, now: i64) -> Result<Hash> {
        let id = trial.id();
        let nonce = i64::try_from(trial.nonce)
            .map_err(|_| Error::new("trial nonce does not fit sqlite"))?;
        let conn = self.lock()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|err| sql_err(err, "trial tx"))?;
        let variant = trial.variant.to_hex();
        let exists: i64 = tx
            .query_row(
                "SELECT COUNT(*) FROM variants WHERE id = ?1",
                [&variant],
                |row| row.get(0),
            )
            .map_err(|err| sql_err(err, "trial variant"))?;
        if exists == 0 {
            return Err(Error::new("variant not found").ctx("variant", variant));
        }
        let already: i64 = tx
            .query_row(
                "SELECT COUNT(*) FROM trials WHERE id = ?1",
                [id.to_hex()],
                |row| row.get(0),
            )
            .map_err(|err| sql_err(err, "trial lookup"))?;
        if already > 0 {
            return Ok(id);
        }
        tx.execute(
            "INSERT INTO trials(id, variant, nonce, created_at) VALUES (?1, ?2, ?3, ?4)",
            params![id.to_hex(), variant, nonce, now],
        )
        .map_err(|err| sql_err(err, "insert trial"))?;
        for (name, value) in &trial.metrics {
            tx.execute(
                "INSERT INTO trial_metrics(trial, name, value) VALUES (?1, ?2, ?3)",
                params![id.to_hex(), name, value],
            )
            .map_err(|err| sql_err(err, "insert trial metric"))?;
        }
        tx.commit().map_err(|err| sql_err(err, "commit trial"))?;
        Ok(id)
    }

    fn trials_of(&self, variant: &Hash) -> Result<Vec<StoredTrial>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id, nonce, created_at FROM trials WHERE variant = ?1 ORDER BY created_at DESC, id DESC")
            .map_err(|err| sql_err(err, "trials"))?;
        let rows = stmt
            .query_map([variant.to_hex()], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            })
            .map_err(|err| sql_err(err, "trials"))?;
        let header: Vec<(String, i64, i64)> = rows
            .collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "trials"))?;
        header
            .into_iter()
            .map(|(id, nonce, created_at)| load_trial(&conn, variant, &id, nonce, created_at))
            .collect()
    }

    fn reject(&self, variant: &Hash, reason: &str, now: i64) -> Result<()> {
        if reason.trim().is_empty() {
            return Err(Error::new("rejection reason is empty"));
        }
        let conn = self.lock()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|err| sql_err(err, "reject tx"))?;
        let stored = load_variant(&tx, &variant.to_hex())?;
        tx.execute(
            "INSERT INTO rejections(variant, reason, created_at) VALUES (?1, ?2, ?3) ON CONFLICT(variant) DO UPDATE SET reason = excluded.reason, created_at = excluded.created_at",
            params![variant.to_hex(), reason, now],
        )
        .map_err(|err| sql_err(err, "reject"))?;
        upsert_recall(
            &tx,
            &variant.to_hex(),
            &stored.campaign,
            &body_of(&stored.variant, Some(reason)),
        )?;
        tx.commit().map_err(|err| sql_err(err, "commit reject"))?;
        Ok(())
    }

    fn rejection_of(&self, variant: &Hash) -> Result<Option<String>> {
        let conn = self.lock()?;
        match conn.query_row(
            "SELECT reason FROM rejections WHERE variant = ?1",
            [variant.to_hex()],
            |row| row.get(0),
        ) {
            Ok(reason) => Ok(Some(reason)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(sql_err(err, "rejection")),
        }
    }
}

fn load_trial(
    conn: &Connection,
    variant: &Hash,
    id: &str,
    nonce: i64,
    created_at: i64,
) -> Result<StoredTrial> {
    let nonce = u64::try_from(nonce)
        .map_err(|_| Error::new("stored nonce is negative").ctx("trial", id))?;
    let mut stmt = conn
        .prepare("SELECT name, value FROM trial_metrics WHERE trial = ?1 ORDER BY name")
        .map_err(|err| sql_err(err, "trial metrics"))?;
    let rows = stmt
        .query_map([id], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, f64>(1)?))
        })
        .map_err(|err| sql_err(err, "trial metrics"))?;
    let pairs: Vec<(String, f64)> = rows
        .collect::<rusqlite::Result<_>>()
        .map_err(|err| sql_err(err, "trial metrics"))?;
    let trial = Trial {
        variant: *variant,
        nonce,
        metrics: pairs.into_iter().collect::<BTreeMap<_, _>>(),
    };
    if trial.id().to_hex() != id {
        return Err(Error::new("stored trial id does not match canonical id").ctx("trial", id));
    }
    Ok(StoredTrial { trial, created_at })
}
