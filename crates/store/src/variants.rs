use acvc_domain::{Error, Hash, Premise, Result, StoredVariant, Variant, VariantStore};
use rusqlite::{Connection, params};

use crate::error::sql_err;
use crate::graph::Graph;
use crate::recall::{body_of, upsert_recall};

impl VariantStore for Graph {
    fn insert_variant(&self, campaign: &str, variant: &Variant, now: i64) -> Result<Hash> {
        variant.validate()?;
        let id = variant.id()?;
        let conn = self.lock()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|err| sql_err(err, "variant tx"))?;
        if let Some(existing) = campaign_of(&tx, &id)? {
            if existing != campaign {
                return Err(Error::new("variant belongs to another campaign")
                    .ctx("variant", id.to_hex())
                    .ctx("campaign", existing));
            }
            return Ok(id);
        }
        let found: i64 = tx
            .query_row(
                "SELECT COUNT(*) FROM campaigns WHERE name = ?1",
                [campaign],
                |row| row.get(0),
            )
            .map_err(|err| sql_err(err, "campaign lookup"))?;
        if found == 0 {
            return Err(Error::new("campaign not found").ctx("campaign", campaign));
        }
        tx.execute(
            "INSERT INTO variants(id, campaign, tree, edge, claim, intent, agent, created_at, revises) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
            params![
                id.to_hex(),
                campaign,
                variant.tree.to_hex(),
                variant.edge as u8 as i64,
                variant.premise.claim,
                variant.intent,
                variant.agent,
                now,
                variant.premise.revises.map(|hash| hash.to_hex()),
            ],
        )
        .map_err(|err| sql_err(err, "insert variant"))?;
        for (ord, parent) in variant.parents.iter().enumerate() {
            tx.execute(
                "INSERT INTO parents(variant, ord, parent) VALUES (?1, ?2, ?3)",
                params![id.to_hex(), ord as i64, parent.to_hex()],
            )
            .map_err(|err| sql_err(err, "insert parent"))?;
        }
        for (ord, assumption) in variant.premise.assumptions.iter().enumerate() {
            tx.execute(
                "INSERT INTO assumptions(variant, ord, text) VALUES (?1, ?2, ?3)",
                params![id.to_hex(), ord as i64, assumption],
            )
            .map_err(|err| sql_err(err, "insert assumption"))?;
        }
        upsert_recall(&tx, &id.to_hex(), campaign, &body_of(variant, None))?;
        tx.commit().map_err(|err| sql_err(err, "commit variant"))?;
        Ok(id)
    }

    fn get_variant(&self, id: &Hash) -> Result<StoredVariant> {
        let conn = self.lock()?;
        load_variant(&conn, &id.to_hex())
    }

    fn resolve(&self, prefix: &str) -> Result<Hash> {
        let prefix = prefix.trim().to_ascii_lowercase();
        if prefix.len() < 8 || prefix.len() > 64 || !prefix.chars().all(|ch| ch.is_ascii_hexdigit())
        {
            return Err(
                Error::new("variant prefix must be 8 to 64 hex chars").ctx("prefix", prefix)
            );
        }
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id FROM variants WHERE id LIKE ?1 || '%' ORDER BY id LIMIT 2")
            .map_err(|err| sql_err(err, "resolve"))?;
        let rows = stmt
            .query_map(params![prefix], |row| row.get::<_, String>(0))
            .map_err(|err| sql_err(err, "resolve"))?;
        let ids: Vec<String> = rows
            .collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "resolve"))?;
        match ids.as_slice() {
            [] => Err(Error::new("variant not found").ctx("prefix", prefix)),
            [one] => Hash::parse(one),
            _ => Err(Error::new("variant prefix is ambiguous").ctx("prefix", prefix)),
        }
    }

    fn children(&self, id: &Hash) -> Result<Vec<Hash>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT variant FROM parents WHERE parent = ?1 ORDER BY variant")
            .map_err(|err| sql_err(err, "children"))?;
        let rows = stmt
            .query_map([id.to_hex()], |row| row.get::<_, String>(0))
            .map_err(|err| sql_err(err, "children"))?;
        let ids: Vec<String> = rows
            .collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "children"))?;
        ids.iter().map(|hex| Hash::parse(hex)).collect()
    }

    fn variants_in(&self, campaign: &str) -> Result<Vec<StoredVariant>> {
        let conn = self.lock()?;
        let mut stmt = conn
            .prepare("SELECT id FROM variants WHERE campaign = ?1 ORDER BY created_at, id")
            .map_err(|err| sql_err(err, "variants"))?;
        let rows = stmt
            .query_map([campaign], |row| row.get::<_, String>(0))
            .map_err(|err| sql_err(err, "variants"))?;
        let ids: Vec<String> = rows
            .collect::<rusqlite::Result<_>>()
            .map_err(|err| sql_err(err, "variants"))?;
        ids.iter().map(|id| load_variant(&conn, id)).collect()
    }
}

pub(crate) fn load_variant(conn: &Connection, id: &str) -> Result<StoredVariant> {
    let row = conn.query_row(
        "SELECT campaign, tree, edge, claim, intent, agent, created_at, revises FROM variants WHERE id = ?1",
        [id],
        |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, u8>(2)?,
                row.get::<_, String>(3)?,
                row.get::<_, String>(4)?,
                row.get::<_, String>(5)?,
                row.get::<_, i64>(6)?,
                row.get::<_, Option<String>>(7)?,
            ))
        },
    );
    let (campaign, tree, edge, claim, intent, agent, created_at, revises) = match row {
        Ok(value) => value,
        Err(rusqlite::Error::QueryReturnedNoRows) => {
            return Err(Error::new("variant not found").ctx("variant", id));
        }
        Err(err) => return Err(sql_err(err, "load variant")),
    };
    let mut parents_stmt = conn
        .prepare("SELECT parent FROM parents WHERE variant = ?1 ORDER BY ord")
        .map_err(|err| sql_err(err, "parents"))?;
    let parent_rows = parents_stmt
        .query_map([id], |row| row.get::<_, String>(0))
        .map_err(|err| sql_err(err, "parents"))?;
    let parent_hex: Vec<String> = parent_rows
        .collect::<rusqlite::Result<_>>()
        .map_err(|err| sql_err(err, "parents"))?;
    let parents = parent_hex
        .iter()
        .map(|hex| Hash::parse(hex))
        .collect::<Result<Vec<_>>>()?;
    let mut assumptions_stmt = conn
        .prepare("SELECT text FROM assumptions WHERE variant = ?1 ORDER BY ord")
        .map_err(|err| sql_err(err, "assumptions"))?;
    let assumption_rows = assumptions_stmt
        .query_map([id], |row| row.get::<_, String>(0))
        .map_err(|err| sql_err(err, "assumptions"))?;
    let assumptions = assumption_rows
        .collect::<rusqlite::Result<_>>()
        .map_err(|err| sql_err(err, "assumptions"))?;
    let revises = match revises {
        Some(hex) => Some(Hash::parse(&hex)?),
        None => None,
    };
    Ok(StoredVariant {
        campaign,
        created_at,
        variant: Variant {
            tree: Hash::parse(&tree)?,
            parents,
            edge: acvc_domain::EdgeKind::from_u8(edge)?,
            premise: Premise {
                claim,
                assumptions,
                revises,
            },
            intent,
            agent,
        },
    })
}

fn campaign_of(conn: &Connection, id: &Hash) -> Result<Option<String>> {
    match conn.query_row(
        "SELECT campaign FROM variants WHERE id = ?1",
        [id.to_hex()],
        |row| row.get(0),
    ) {
        Ok(name) => Ok(Some(name)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(err) => Err(sql_err(err, "variant campaign")),
    }
}
