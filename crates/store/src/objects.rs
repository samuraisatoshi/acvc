use std::fs::{self, File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use acvc_domain::{BlobCache, Error, Hash, ObjectStore, Result, TreeEntry};
use rusqlite::params;

use crate::db::{Db, meta_get, meta_set};
use crate::error::{io_err, sql_err};

const HEADER: &[u8; 8] = b"ACVCLOG1";
const MAX_OBJECT: u64 = 512 * 1024 * 1024;

pub struct Objects {
    pub(crate) db: Arc<Db>,
    pub(crate) acvc: PathBuf,
    gate: Mutex<()>,
}

pub(crate) struct Located {
    pub offset: u64,
    pub len: u64,
    pub kind: u8,
    pub loc: u8,
    pub clen: u64,
}

impl Objects {
    pub(crate) fn open(db: Arc<Db>, acvc: PathBuf) -> Result<Self> {
        let objects = Self {
            db,
            acvc,
            gate: Mutex::new(()),
        };
        objects.sync()?;
        Ok(objects)
    }

    pub(crate) fn log_path(&self) -> PathBuf {
        self.acvc.join("objects.log")
    }

    pub(crate) fn pack_path(&self) -> PathBuf {
        self.acvc.join("objects.pack")
    }

    pub(crate) fn lock_gate(&self) -> Result<std::sync::MutexGuard<'_, ()>> {
        self.gate
            .lock()
            .map_err(|_| Error::new("object log lock poisoned"))
    }

    fn sync(&self) -> Result<()> {
        let _gate = self.lock_gate()?;
        ensure_header(&self.log_path())?;
        let len = fs::metadata(self.log_path())
            .map_err(|err| io_err(err, "stat log", &self.log_path()))?
            .len();
        let conn = self.db.lock()?;
        let start = meta_get(&conn, "log_end")?
            .and_then(|value| value.parse::<u64>().ok())
            .unwrap_or(HEADER.len() as u64);
        drop(conn);
        if start > len {
            return self.reconcile_locked();
        }
        let end = scan_log(&self.log_path(), start, &self.db)?;
        let conn = self.db.lock()?;
        meta_set(&conn, "log_end", &end.to_string())
    }

    fn put_raw(&self, kind: u8, payload: &[u8], hash: Hash) -> Result<()> {
        let _gate = self.lock_gate()?;
        let hex = hash.to_hex();
        let conn = self.db.lock()?;
        let exists: bool = conn
            .query_row("SELECT 1 FROM objects WHERE hash = ?1", [&hex], |_| Ok(()))
            .optional_row()
            .map_err(|err| sql_err(err, "object lookup"))?
            .is_some();
        if exists {
            return Ok(());
        }
        drop(conn);
        let offset = append_record(&self.log_path(), kind, payload)?;
        let conn = self.db.lock()?;
        conn.execute(
            "INSERT OR IGNORE INTO objects(hash, offset, len, kind) VALUES (?1, ?2, ?3, ?4)",
            params![hex, offset as i64, payload.len() as i64, kind as i64],
        )
        .map_err(|err| sql_err(err, "object insert"))?;
        let end = offset + 1 + 8 + payload.len() as u64;
        meta_set(&conn, "log_end", &end.to_string())
    }

    pub(crate) fn locate(&self, hash: &Hash) -> Result<Located> {
        let conn = self.db.lock()?;
        let found = conn
            .query_row(
                "SELECT offset, len, kind, loc, clen FROM objects WHERE hash = ?1",
                [hash.to_hex()],
                |row| {
                    Ok(Located {
                        offset: row.get::<_, i64>(0)? as u64,
                        len: row.get::<_, i64>(1)? as u64,
                        kind: row.get::<_, i64>(2)? as u8,
                        loc: row.get::<_, i64>(3)? as u8,
                        clen: row.get::<_, i64>(4)? as u64,
                    })
                },
            )
            .optional_located()
            .map_err(|err| sql_err(err, "object read"))?;
        found.ok_or_else(|| Error::new("object not found").ctx("hash", hash.to_hex()))
    }

    fn read_any(&self, hash: &Hash) -> Result<(u8, Vec<u8>)> {
        let _gate = self.lock_gate()?;
        let located = self.locate(hash)?;
        let (kind, payload) = if located.loc == 0 {
            read_record(&self.log_path(), located.offset, located.len)?
        } else {
            crate::pack::read_packed(&self.pack_path(), &located)?
        };
        verify(hash, located.kind, kind, &payload)?;
        Ok((kind, payload))
    }

    fn read_log_only(&self, hash: &Hash) -> Result<(u8, Vec<u8>)> {
        let _gate = self.lock_gate()?;
        let located = self.locate(hash)?;
        if located.loc != 0 {
            return Err(Error::new("object is packed").ctx("hash", hash.to_hex()));
        }
        let (kind, payload) = read_record(&self.log_path(), located.offset, located.len)?;
        verify(hash, located.kind, kind, &payload)?;
        Ok((kind, payload))
    }
}

fn verify(hash: &Hash, expected_kind: u8, kind: u8, payload: &[u8]) -> Result<()> {
    if kind != expected_kind {
        return Err(Error::new("object kind mismatch").ctx("hash", hash.to_hex()));
    }
    let actual = match kind {
        0 => Hash::blob(payload),
        1 => Hash::tree(payload),
        _ => return Err(Error::new("unknown object kind").ctx("kind", kind)),
    };
    if actual != *hash {
        return Err(Error::new("object hash mismatch").ctx("hash", hash.to_hex()));
    }
    Ok(())
}

impl ObjectStore for Objects {
    fn put_blob(&self, content: &[u8]) -> Result<Hash> {
        let hash = Hash::blob(content);
        self.put_raw(0, content, hash)?;
        Ok(hash)
    }

    fn put_tree(&self, entries: &[TreeEntry]) -> Result<Hash> {
        let canonical = acvc_domain::canonical_tree(entries)?;
        let hash = Hash::tree(&canonical);
        self.put_raw(1, &canonical, hash)?;
        Ok(hash)
    }

    fn get_blob(&self, hash: &Hash) -> Result<Vec<u8>> {
        let (kind, payload) = self.read_any(hash)?;
        if kind != 0 {
            return Err(Error::new("object is not a blob").ctx("hash", hash.to_hex()));
        }
        Ok(payload)
    }

    fn get_tree(&self, hash: &Hash) -> Result<Vec<TreeEntry>> {
        let (kind, payload) = self.read_log_only(hash)?;
        if kind != 1 {
            return Err(Error::new("object is not a tree").ctx("hash", hash.to_hex()));
        }
        acvc_domain::decode_tree(&payload)
    }

    fn has_object(&self, hash: &Hash) -> Result<bool> {
        let conn = self.db.lock()?;
        let found = conn
            .query_row(
                "SELECT 1 FROM objects WHERE hash = ?1",
                [hash.to_hex()],
                |_| Ok(()),
            )
            .optional_row()
            .map_err(|err| sql_err(err, "object lookup"))?;
        Ok(found.is_some())
    }

    fn read_object(&self, hash: &Hash) -> Result<(u8, Vec<u8>)> {
        self.read_any(hash)
    }

    fn put_object(&self, kind: u8, bytes: &[u8]) -> Result<Hash> {
        let hash = match kind {
            0 => Hash::blob(bytes),
            1 => Hash::tree(bytes),
            other => return Err(Error::new("unknown object kind").ctx("kind", other)),
        };
        self.put_raw(kind, bytes, hash)?;
        Ok(hash)
    }

    fn pack(&self) -> Result<acvc_domain::PackReport> {
        self.pack_blobs()
    }
}

impl BlobCache for Objects {
    fn materialize_blob(&self, hash: &Hash) -> Result<PathBuf> {
        let path = cache_path(&self.acvc, hash);
        if path.is_file() {
            return Ok(path);
        }
        let bytes = self.get_blob(hash)?;
        let _gate = self.lock_gate()?;
        if path.is_file() {
            return Ok(path);
        }
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).map_err(|err| io_err(err, "create cache", parent))?;
        }
        let tmp = path.with_extension("partial");
        fs::write(&tmp, bytes).map_err(|err| io_err(err, "write cache", &tmp))?;
        let mut perms = fs::metadata(&tmp)
            .map_err(|err| io_err(err, "stat cache", &tmp))?
            .permissions();
        perms.set_readonly(true);
        fs::set_permissions(&tmp, perms).map_err(|err| io_err(err, "chmod cache", &tmp))?;
        fs::rename(&tmp, &path).map_err(|err| io_err(err, "rename cache", &path))?;
        Ok(path)
    }
}

fn cache_path(acvc: &Path, hash: &Hash) -> PathBuf {
    let hex = hash.to_hex();
    acvc.join("cache").join(&hex[..2]).join(&hex[2..])
}

fn ensure_header(path: &Path) -> Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(path)
        .map_err(|err| io_err(err, "open log", path))?;
    if file
        .metadata()
        .map_err(|err| io_err(err, "stat log", path))?
        .len()
        == 0
    {
        file.write_all(HEADER)
            .map_err(|err| io_err(err, "write log header", path))?;
        file.sync_all()
            .map_err(|err| io_err(err, "sync log", path))?;
        return Ok(());
    }
    let mut magic = [0u8; 8];
    file.read_exact(&mut magic)
        .map_err(|err| io_err(err, "read log header", path))?;
    if &magic != HEADER {
        return Err(Error::new("bad object log magic").ctx("path", path.display().to_string()));
    }
    Ok(())
}

fn append_record(path: &Path, kind: u8, payload: &[u8]) -> Result<u64> {
    if payload.len() as u64 > MAX_OBJECT {
        return Err(Error::new("object exceeds 512MB").ctx("len", payload.len()));
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open(path)
        .map_err(|err| io_err(err, "append log", path))?;
    let offset = file
        .seek(SeekFrom::End(0))
        .map_err(|err| io_err(err, "seek log", path))?;
    file.write_all(&[kind])
        .map_err(|err| io_err(err, "write log", path))?;
    file.write_all(&(payload.len() as u64).to_le_bytes())
        .map_err(|err| io_err(err, "write log", path))?;
    file.write_all(payload)
        .map_err(|err| io_err(err, "write log", path))?;
    file.sync_all()
        .map_err(|err| io_err(err, "sync log", path))?;
    Ok(offset)
}

pub(crate) fn read_record(path: &Path, offset: u64, expected_len: u64) -> Result<(u8, Vec<u8>)> {
    if expected_len > MAX_OBJECT {
        return Err(Error::new("object exceeds 512MB").ctx("len", expected_len));
    }
    let mut file = File::open(path).map_err(|err| io_err(err, "open log", path))?;
    file.seek(SeekFrom::Start(offset))
        .map_err(|err| io_err(err, "seek log", path))?;
    let mut kind = [0u8; 1];
    file.read_exact(&mut kind)
        .map_err(|err| io_err(err, "read log", path))?;
    let mut len_buf = [0u8; 8];
    file.read_exact(&mut len_buf)
        .map_err(|err| io_err(err, "read log", path))?;
    let len = u64::from_le_bytes(len_buf);
    if len != expected_len {
        return Err(Error::new("object length mismatch").ctx("path", path.display().to_string()));
    }
    let mut payload = vec![0u8; len as usize];
    file.read_exact(&mut payload)
        .map_err(|err| io_err(err, "read log", path))?;
    Ok((kind[0], payload))
}

fn scan_log(path: &Path, mut pos: u64, db: &Db) -> Result<u64> {
    let file_len = fs::metadata(path)
        .map_err(|err| io_err(err, "stat log", path))?
        .len();
    let mut file = File::open(path).map_err(|err| io_err(err, "open log", path))?;
    while pos < file_len {
        file.seek(SeekFrom::Start(pos))
            .map_err(|err| io_err(err, "seek log", path))?;
        let mut kind = [0u8; 1];
        file.read_exact(&mut kind)
            .map_err(|err| io_err(err, "scan log", path))?;
        let mut len_buf = [0u8; 8];
        file.read_exact(&mut len_buf)
            .map_err(|err| io_err(err, "scan log", path))?;
        let len = u64::from_le_bytes(len_buf);
        if len > MAX_OBJECT || pos + 1 + 8 + len > file_len {
            return Err(Error::new("truncated object log").ctx("offset", pos));
        }
        let mut payload = vec![0u8; len as usize];
        file.read_exact(&mut payload)
            .map_err(|err| io_err(err, "scan log", path))?;
        let hash = match kind[0] {
            0 => Hash::blob(&payload),
            1 => Hash::tree(&payload),
            other => return Err(Error::new("unknown object kind").ctx("kind", other)),
        };
        let conn = db.lock()?;
        conn.execute(
            "INSERT OR IGNORE INTO objects(hash, offset, len, kind) VALUES (?1, ?2, ?3, ?4)",
            params![hash.to_hex(), pos as i64, len as i64, kind[0] as i64],
        )
        .map_err(|err| sql_err(err, "scan insert"))?;
        drop(conn);
        pos += 1 + 8 + len;
    }
    Ok(pos)
}

trait OptionalUnit {
    fn optional_row(self) -> rusqlite::Result<Option<()>>;
}

impl OptionalUnit for rusqlite::Result<()> {
    fn optional_row(self) -> rusqlite::Result<Option<()>> {
        match self {
            Ok(()) => Ok(Some(())),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }
}

trait OptionalLocated {
    fn optional_located(self) -> rusqlite::Result<Option<Located>>;
}

impl OptionalLocated for rusqlite::Result<Located> {
    fn optional_located(self) -> rusqlite::Result<Option<Located>> {
        match self {
            Ok(value) => Ok(Some(value)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(err) => Err(err),
        }
    }
}
