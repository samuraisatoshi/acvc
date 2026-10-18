use acvc_domain::{Error, Result, Variant};
use rusqlite::{Connection, params};

use crate::error::sql_err;

pub(crate) fn body_of(variant: &Variant, rejection: Option<&str>) -> String {
    let mut body = String::new();
    body.push_str(&variant.premise.claim);
    body.push('\n');
    for assumption in &variant.premise.assumptions {
        body.push_str(assumption);
        body.push('\n');
    }
    body.push_str(&variant.intent);
    body.push('\n');
    body.push_str(&variant.agent);
    body.push('\n');
    if let Some(reason) = rejection {
        body.push_str(reason);
        body.push('\n');
    }
    body
}

pub(crate) fn fts_query(query: &str) -> Result<String> {
    let parts: Vec<String> = query
        .split_whitespace()
        .map(|token| {
            token
                .chars()
                .filter(|ch| ch.is_alphanumeric() || *ch == '_' || *ch == '-')
                .collect::<String>()
        })
        .filter(|token| !token.is_empty())
        .map(|token| format!("\"{token}\"*"))
        .collect();
    if parts.is_empty() {
        return Err(Error::new("empty recall query"));
    }
    Ok(parts.join(" AND "))
}

pub(crate) fn upsert_recall(
    conn: &Connection,
    variant_id: &str,
    campaign: &str,
    body: &str,
) -> Result<()> {
    let existing = match conn.query_row(
        "SELECT id, body FROM recall_doc WHERE variant = ?1",
        [variant_id],
        |row| Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?)),
    ) {
        Ok(value) => Some(value),
        Err(rusqlite::Error::QueryReturnedNoRows) => None,
        Err(err) => return Err(sql_err(err, "recall lookup")),
    };
    if let Some((id, old_body)) = existing {
        conn.execute(
            "INSERT INTO recall_fts(recall_fts, rowid, body) VALUES ('delete', ?1, ?2)",
            params![id, old_body],
        )
        .map_err(|err| sql_err(err, "recall delete"))?;
        conn.execute(
            "UPDATE recall_doc SET body = ?1, campaign = ?2 WHERE id = ?3",
            params![body, campaign, id],
        )
        .map_err(|err| sql_err(err, "recall update"))?;
        conn.execute(
            "INSERT INTO recall_fts(rowid, body) VALUES (?1, ?2)",
            params![id, body],
        )
        .map_err(|err| sql_err(err, "recall index"))?;
        return Ok(());
    }
    conn.execute(
        "INSERT INTO recall_doc(variant, campaign, body) VALUES (?1, ?2, ?3)",
        params![variant_id, campaign, body],
    )
    .map_err(|err| sql_err(err, "recall insert"))?;
    let id = conn.last_insert_rowid();
    conn.execute(
        "INSERT INTO recall_fts(rowid, body) VALUES (?1, ?2)",
        params![id, body],
    )
    .map_err(|err| sql_err(err, "recall index"))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{fts_query, upsert_recall};
    use crate::schema::SCHEMA;
    use rusqlite::Connection;

    #[test]
    fn accented_word_matches_the_indexed_premise() {
        let conn = Connection::open_in_memory().unwrap();
        conn.execute_batch(SCHEMA).unwrap();
        let variant = "ab".repeat(32);
        upsert_recall(&conn, &variant, "engine", "cada sessão grava a hipótese\n").unwrap();
        let query = fts_query("sessão").unwrap();
        let hit: String = conn
            .query_row(
                "SELECT d.variant FROM recall_doc d JOIN recall_fts ON recall_fts.rowid = d.id WHERE recall_fts MATCH ?1",
                [query],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(hit, variant);
        assert!(fts_query("!!").is_err());
        let missing = fts_query("xyzzy").unwrap();
        let count: i64 = conn
            .query_row(
                "SELECT count(*) FROM recall_doc d JOIN recall_fts ON recall_fts.rowid = d.id WHERE recall_fts MATCH ?1",
                [missing],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(count, 0);
    }
}
