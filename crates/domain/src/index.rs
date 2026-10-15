use crate::codec::{Cursor, push_hash, push_i64, push_str, push_u32, push_u64};
use crate::{Error, Hash, Result};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fingerprint {
    pub size: u64,
    pub mtime_nsec: i64,
    pub inode: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FileStat {
    pub mode: u32,
    pub fingerprint: Fingerprint,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IndexEntry {
    pub path: String,
    pub mode: u32,
    pub hash: Hash,
    pub fingerprint: Fingerprint,
}

pub fn same_stat(entry: &IndexEntry, stat: &FileStat) -> bool {
    entry.mode == stat.mode
        && entry.fingerprint.size == stat.fingerprint.size
        && entry.fingerprint.mtime_nsec == stat.fingerprint.mtime_nsec
        && entry.fingerprint.inode == stat.fingerprint.inode
}

pub fn encode_index(entries: &[IndexEntry]) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    out.extend_from_slice(b"ACVCIDX1");
    push_u32(&mut out, entries.len() as u32);
    let mut previous: Option<&str> = None;
    for entry in entries {
        if let Some(prev) = previous
            && entry.path.as_str() <= prev
        {
            return Err(
                Error::new("index paths must be strictly sorted").ctx("path", entry.path.clone())
            );
        }
        previous = Some(entry.path.as_str());
        push_str(&mut out, &entry.path);
        push_u32(&mut out, entry.mode);
        push_hash(&mut out, &entry.hash);
        push_u64(&mut out, entry.fingerprint.size);
        push_i64(&mut out, entry.fingerprint.mtime_nsec);
        push_u64(&mut out, entry.fingerprint.inode);
    }
    Ok(out)
}

pub fn decode_index(bytes: &[u8]) -> Result<Vec<IndexEntry>> {
    if bytes.len() < 8 || &bytes[..8] != b"ACVCIDX1" {
        return Err(Error::new("bad index magic"));
    }
    let mut cursor = Cursor::new(&bytes[8..]);
    let count = cursor.u32()? as usize;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        entries.push(IndexEntry {
            path: cursor.string()?,
            mode: cursor.u32()?,
            hash: cursor.hash()?,
            fingerprint: Fingerprint {
                size: cursor.u64()?,
                mtime_nsec: cursor.i64()?,
                inode: cursor.u64()?,
            },
        });
    }
    cursor.finish()?;
    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn index_roundtrip_and_stat_match() {
        let entry = IndexEntry {
            path: "a/b.txt".into(),
            mode: 0o100644,
            hash: Hash::blob(b"x"),
            fingerprint: Fingerprint {
                size: 1,
                mtime_nsec: 2,
                inode: 3,
            },
        };
        let bytes = encode_index(std::slice::from_ref(&entry)).unwrap();
        assert_eq!(decode_index(&bytes).unwrap(), vec![entry.clone()]);
        let stat = FileStat {
            mode: entry.mode,
            fingerprint: entry.fingerprint,
        };
        assert!(same_stat(&entry, &stat));
        let mut changed = stat.clone();
        changed.fingerprint.size = 9;
        assert!(!same_stat(&entry, &changed));
    }
}
