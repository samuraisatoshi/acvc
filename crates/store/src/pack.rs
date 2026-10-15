// obsidian-vault/12-Scope-Docs/CD-ACVC-08-packfile.md
// Blob records only. Trees stay in objects.log so record never opens the pack.

use std::collections::BTreeMap;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::Path;

use acvc_domain::{Hash, PackReport, Result};
use flate2::Compression;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use rusqlite::params;

use crate::db::meta_set;
use crate::error::{io_err, sql_err};
use crate::objects::{self, Located, Objects};

const PACK_HEADER: &[u8; 8] = b"ACVCPAK1";
const LOG_HEADER: &[u8; 8] = b"ACVCLOG1";
const RECORD_HEAD: u64 = 32 + 1 + 8 + 8;

impl Objects {
    pub(crate) fn pack_blobs(&self) -> Result<PackReport> {
        let _gate = self.lock_gate()?;
        let report = self.append_cold_blobs()?;
        self.rewrite_hot_log()?;
        self.reconcile_locked()?;
        Ok(report)
    }

    pub(crate) fn reconcile_locked(&self) -> Result<()> {
        let log_at = index_log(&self.log_path())?;
        let pack_at = index_pack(&self.pack_path())?;
        let conn = self.db.lock()?;
        let tx = conn
            .unchecked_transaction()
            .map_err(|err| sql_err(err, "pack reconcile"))?;
        for (hash, row) in &log_at {
            upsert(&tx, hash, row.offset, row.len, row.kind, 0, 0)?;
        }
        for (hash, row) in &pack_at {
            if log_at.contains_key(hash) {
                continue;
            }
            upsert(&tx, hash, row.offset, row.len, row.kind, 1, row.clen)?;
        }
        let len = fs::metadata(self.log_path())
            .map_err(|err| io_err(err, "stat log", &self.log_path()))?
            .len();
        meta_set(&tx, "log_end", &len.to_string())?;
        tx.commit().map_err(|err| sql_err(err, "pack reconcile"))?;
        Ok(())
    }

    fn append_cold_blobs(&self) -> Result<PackReport> {
        let already = index_pack(&self.pack_path())?;
        let conn = self.db.lock()?;
        let mut stmt = conn
            .prepare("SELECT hash, offset, len FROM objects WHERE loc = 0 AND kind = 0")
            .map_err(|err| sql_err(err, "list cold blobs"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)? as u64,
                    row.get::<_, i64>(2)? as u64,
                ))
            })
            .map_err(|err| sql_err(err, "list cold blobs"))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|err| sql_err(err, "list cold blobs"))?;
        drop(stmt);
        drop(conn);
        let mut report = PackReport {
            objects: 0,
            raw_bytes: 0,
            packed_bytes: 0,
        };
        if rows.is_empty() {
            return Ok(report);
        }
        let mut file = open_pack(&self.pack_path())?;
        for (hex, offset, len) in rows {
            let hash = Hash::parse(&hex)?;
            if let Some(row) = already.get(&hash) {
                mark_packed(&self.db, &hash, row.offset, row.len, row.clen)?;
                continue;
            }
            let (kind, payload) = objects::read_record(&self.log_path(), offset, len)?;
            if kind != 0 || Hash::blob(&payload) != hash {
                return Err(acvc_domain::Error::new("cold blob mismatch").ctx("hash", hex));
            }
            let compressed = deflate(&payload)?;
            let at = file
                .seek(SeekFrom::End(0))
                .map_err(|err| io_err(err, "seek pack", &self.pack_path()))?;
            write_pack_record(&mut file, &hash, payload.len() as u64, &compressed)?;
            mark_packed(
                &self.db,
                &hash,
                at,
                payload.len() as u64,
                compressed.len() as u64,
            )?;
            report.objects += 1;
            report.raw_bytes += payload.len() as u64;
            report.packed_bytes += compressed.len() as u64;
        }
        file.sync_all()
            .map_err(|err| io_err(err, "sync pack", &self.pack_path()))?;
        Ok(report)
    }

    fn rewrite_hot_log(&self) -> Result<()> {
        let conn = self.db.lock()?;
        let mut stmt = conn
            .prepare("SELECT hash, offset, len, kind FROM objects WHERE loc = 0")
            .map_err(|err| sql_err(err, "list hot objects"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)? as u64,
                    row.get::<_, i64>(2)? as u64,
                    row.get::<_, i64>(3)? as u8,
                ))
            })
            .map_err(|err| sql_err(err, "list hot objects"))?
            .collect::<std::result::Result<Vec<_>, _>>()
            .map_err(|err| sql_err(err, "list hot objects"))?;
        drop(stmt);
        drop(conn);
        let next = self.acvc.join("objects.log.next");
        let mut out = File::create(&next).map_err(|err| io_err(err, "create log", &next))?;
        out.write_all(LOG_HEADER)
            .map_err(|err| io_err(err, "write log", &next))?;
        for (hex, offset, len, kind) in rows {
            let (actual, payload) = objects::read_record(&self.log_path(), offset, len)?;
            if actual != kind {
                return Err(acvc_domain::Error::new("hot object kind mismatch").ctx("hash", hex));
            }
            out.write_all(&[kind])
                .map_err(|err| io_err(err, "write log", &next))?;
            out.write_all(&(payload.len() as u64).to_le_bytes())
                .map_err(|err| io_err(err, "write log", &next))?;
            out.write_all(&payload)
                .map_err(|err| io_err(err, "write log", &next))?;
        }
        out.sync_all()
            .map_err(|err| io_err(err, "sync log", &next))?;
        drop(out);
        fs::rename(&next, self.log_path())
            .map_err(|err| io_err(err, "rename log", &self.log_path()))
    }
}

struct Indexed {
    offset: u64,
    len: u64,
    kind: u8,
    clen: u64,
}

pub(crate) fn read_packed(path: &Path, located: &Located) -> Result<(u8, Vec<u8>)> {
    let mut file = File::open(path).map_err(|err| io_err(err, "open pack", path))?;
    file.seek(SeekFrom::Start(located.offset))
        .map_err(|err| io_err(err, "seek pack", path))?;
    let mut hash_buf = [0u8; 32];
    file.read_exact(&mut hash_buf)
        .map_err(|err| io_err(err, "read pack", path))?;
    let mut kind = [0u8; 1];
    file.read_exact(&mut kind)
        .map_err(|err| io_err(err, "read pack", path))?;
    let mut raw_buf = [0u8; 8];
    file.read_exact(&mut raw_buf)
        .map_err(|err| io_err(err, "read pack", path))?;
    let mut comp_buf = [0u8; 8];
    file.read_exact(&mut comp_buf)
        .map_err(|err| io_err(err, "read pack", path))?;
    let raw_len = u64::from_le_bytes(raw_buf);
    let comp_len = u64::from_le_bytes(comp_buf);
    if raw_len != located.len || comp_len != located.clen || kind[0] != located.kind {
        return Err(acvc_domain::Error::new("pack record mismatch"));
    }
    let mut compressed = vec![0u8; comp_len as usize];
    file.read_exact(&mut compressed)
        .map_err(|err| io_err(err, "read pack", path))?;
    let payload = inflate(&compressed, raw_len as usize)?;
    Ok((kind[0], payload))
}

fn mark_packed(
    db: &crate::db::Db,
    hash: &Hash,
    offset: u64,
    raw_len: u64,
    comp_len: u64,
) -> Result<()> {
    let conn = db.lock()?;
    let changed = conn
        .execute(
            "UPDATE objects SET offset = ?2, len = ?3, kind = 0, loc = 1, clen = ?4 WHERE hash = ?1",
            params![hash.to_hex(), offset as i64, raw_len as i64, comp_len as i64],
        )
        .map_err(|err| sql_err(err, "mark packed"))?;
    if changed != 1 {
        return Err(
            acvc_domain::Error::new("packed blob missing from index").ctx("hash", hash.to_hex())
        );
    }
    Ok(())
}

fn upsert(
    tx: &rusqlite::Transaction<'_>,
    hash: &Hash,
    offset: u64,
    len: u64,
    kind: u8,
    loc: u8,
    clen: u64,
) -> Result<()> {
    tx.execute(
        "INSERT INTO objects(hash, offset, len, kind, loc, clen) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
         ON CONFLICT(hash) DO UPDATE SET offset = excluded.offset, len = excluded.len,
         kind = excluded.kind, loc = excluded.loc, clen = excluded.clen",
        params![
            hash.to_hex(),
            offset as i64,
            len as i64,
            kind as i64,
            loc as i64,
            clen as i64
        ],
    )
    .map_err(|err| sql_err(err, "pack upsert"))?;
    Ok(())
}

fn open_pack(path: &Path) -> Result<File> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
        .map_err(|err| io_err(err, "open pack", path))?;
    if file
        .metadata()
        .map_err(|err| io_err(err, "stat pack", path))?
        .len()
        == 0
    {
        file.write_all(PACK_HEADER)
            .map_err(|err| io_err(err, "write pack", path))?;
        return Ok(file);
    }
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic)
        .map_err(|err| io_err(err, "read pack", path))?;
    if &magic != PACK_HEADER {
        return Err(
            acvc_domain::Error::new("bad pack magic").ctx("path", path.display().to_string())
        );
    }
    Ok(file)
}

fn write_pack_record(file: &mut File, hash: &Hash, raw_len: u64, compressed: &[u8]) -> Result<()> {
    file.write_all(hash.as_bytes())
        .and_then(|_| file.write_all(&[0]))
        .and_then(|_| file.write_all(&raw_len.to_le_bytes()))
        .and_then(|_| file.write_all(&(compressed.len() as u64).to_le_bytes()))
        .and_then(|_| file.write_all(compressed))
        .map_err(|err| io_err(err, "write pack", Path::new("objects.pack")))?;
    Ok(())
}

fn index_pack(path: &Path) -> Result<BTreeMap<Hash, Indexed>> {
    let mut out = BTreeMap::new();
    if !path.is_file() {
        return Ok(out);
    }
    let mut file = File::open(path).map_err(|err| io_err(err, "open pack", path))?;
    let len = file
        .metadata()
        .map_err(|err| io_err(err, "stat pack", path))?
        .len();
    if len == 0 {
        return Ok(out);
    }
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic)
        .map_err(|err| io_err(err, "read pack", path))?;
    if &magic != PACK_HEADER {
        return Err(
            acvc_domain::Error::new("bad pack magic").ctx("path", path.display().to_string())
        );
    }
    let mut pos = 8u64;
    while pos + RECORD_HEAD <= len {
        file.seek(SeekFrom::Start(pos))
            .map_err(|err| io_err(err, "seek pack", path))?;
        let mut hash_buf = [0u8; 32];
        file.read_exact(&mut hash_buf)
            .map_err(|err| io_err(err, "read pack", path))?;
        let mut kind = [0u8; 1];
        file.read_exact(&mut kind)
            .map_err(|err| io_err(err, "read pack", path))?;
        let mut raw_buf = [0u8; 8];
        file.read_exact(&mut raw_buf)
            .map_err(|err| io_err(err, "read pack", path))?;
        let mut comp_buf = [0u8; 8];
        file.read_exact(&mut comp_buf)
            .map_err(|err| io_err(err, "read pack", path))?;
        let raw_len = u64::from_le_bytes(raw_buf);
        let comp_len = u64::from_le_bytes(comp_buf);
        let next = pos + RECORD_HEAD + comp_len;
        if next > len {
            return Err(acvc_domain::Error::new("truncated pack").ctx("offset", pos));
        }
        out.insert(
            Hash::from_bytes(hash_buf),
            Indexed {
                offset: pos,
                len: raw_len,
                kind: kind[0],
                clen: comp_len,
            },
        );
        pos = next;
    }
    Ok(out)
}

fn index_log(path: &Path) -> Result<BTreeMap<Hash, Indexed>> {
    let mut out = BTreeMap::new();
    let file_len = fs::metadata(path)
        .map_err(|err| io_err(err, "stat log", path))?
        .len();
    let mut pos = LOG_HEADER.len() as u64;
    while pos < file_len {
        let (kind, payload) = objects::read_record(path, pos, {
            let mut file = File::open(path).map_err(|err| io_err(err, "open log", path))?;
            file.seek(SeekFrom::Start(pos + 1))
                .map_err(|err| io_err(err, "seek log", path))?;
            let mut len_buf = [0u8; 8];
            file.read_exact(&mut len_buf)
                .map_err(|err| io_err(err, "read log", path))?;
            u64::from_le_bytes(len_buf)
        })?;
        let hash = match kind {
            0 => Hash::blob(&payload),
            1 => Hash::tree(&payload),
            other => return Err(acvc_domain::Error::new("unknown object kind").ctx("kind", other)),
        };
        out.insert(
            hash,
            Indexed {
                offset: pos,
                len: payload.len() as u64,
                kind,
                clen: 0,
            },
        );
        pos += 1 + 8 + payload.len() as u64;
    }
    Ok(out)
}

fn deflate(raw: &[u8]) -> Result<Vec<u8>> {
    let mut enc = ZlibEncoder::new(Vec::new(), Compression::default());
    enc.write_all(raw)
        .and_then(|_| enc.finish())
        .map_err(|err| acvc_domain::Error::new(err.to_string()).ctx("op", "deflate"))
}

fn inflate(compressed: &[u8], raw_len: usize) -> Result<Vec<u8>> {
    let mut dec = ZlibDecoder::new(compressed);
    let mut out = Vec::new();
    dec.read_to_end(&mut out)
        .map_err(|err| acvc_domain::Error::new(err.to_string()).ctx("op", "inflate"))?;
    if out.len() != raw_len {
        return Err(acvc_domain::Error::new("inflated length mismatch").ctx("len", out.len()));
    }
    Ok(out)
}
