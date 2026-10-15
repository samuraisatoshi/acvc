use crate::codec::{Cursor, push_hash, push_str, push_u8, push_u32};
use crate::{Error, Hash, Result};

pub const TREE_MODE: u32 = 0o040000;
pub const FILE_MODE: u32 = 0o100644;
pub const EXEC_MODE: u32 = 0o100755;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum EntryKind {
    Blob = 0,
    Tree = 1,
}

impl EntryKind {
    pub fn from_u8(value: u8) -> Result<Self> {
        match value {
            0 => Ok(Self::Blob),
            1 => Ok(Self::Tree),
            _ => Err(Error::new("unknown entry kind").ctx("kind", value)),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeEntry {
    pub name: String,
    pub mode: u32,
    pub kind: EntryKind,
    pub hash: Hash,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TreeFile {
    pub path: String,
    pub mode: u32,
    pub hash: Hash,
}

pub fn valid_relative(path: &str) -> bool {
    !path.is_empty()
        && !path.starts_with('/')
        && !path.starts_with('\\')
        && path
            .split('/')
            .all(|part| !part.is_empty() && part != "." && part != "..")
}

pub fn canonical_tree(entries: &[TreeEntry]) -> Result<Vec<u8>> {
    let mut ordered = entries.to_vec();
    ordered.sort_by(|left, right| left.name.cmp(&right.name));
    let mut out = Vec::new();
    push_u32(&mut out, ordered.len() as u32);
    let mut previous = "";
    for entry in &ordered {
        if entry.name.is_empty() || entry.name.contains('/') || entry.name.contains('\0') {
            return Err(Error::new("invalid tree entry name").ctx("name", entry.name.clone()));
        }
        if entry.name == previous {
            return Err(Error::new("duplicate tree entry").ctx("name", entry.name.clone()));
        }
        previous = &entry.name;
        push_str(&mut out, &entry.name);
        push_u32(&mut out, entry.mode);
        push_u8(&mut out, entry.kind as u8);
        push_hash(&mut out, &entry.hash);
    }
    Ok(out)
}

pub fn tree_hash(entries: &[TreeEntry]) -> Result<Hash> {
    Ok(Hash::tree(&canonical_tree(entries)?))
}

pub fn decode_tree(bytes: &[u8]) -> Result<Vec<TreeEntry>> {
    let mut cursor = Cursor::new(bytes);
    let count = cursor.u32()? as usize;
    let mut entries = Vec::with_capacity(count);
    for _ in 0..count {
        entries.push(TreeEntry {
            name: cursor.string()?,
            mode: cursor.u32()?,
            kind: EntryKind::from_u8(cursor.u8()?)?,
            hash: cursor.hash()?,
        });
    }
    cursor.finish()?;
    Ok(entries)
}

pub fn walk_files(
    root: Hash,
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
) -> Result<Vec<TreeFile>> {
    let mut files = Vec::new();
    walk_at("", root, load, &mut files)?;
    files.sort_by(|left, right| left.path.cmp(&right.path));
    Ok(files)
}

fn walk_at(
    prefix: &str,
    tree: Hash,
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
    files: &mut Vec<TreeFile>,
) -> Result<()> {
    for entry in load(&tree)? {
        let path = join_name(prefix, &entry.name);
        match entry.kind {
            EntryKind::Blob => files.push(TreeFile {
                path,
                mode: entry.mode,
                hash: entry.hash,
            }),
            EntryKind::Tree => walk_at(&path, entry.hash, load, files)?,
        }
    }
    Ok(())
}

pub fn lookup_file(
    root: Hash,
    path: &str,
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
) -> Result<Option<TreeFile>> {
    if !valid_relative(path) {
        return Err(Error::new("invalid relative path").ctx("path", path));
    }
    let mut current = root;
    let parts: Vec<&str> = path.split('/').collect();
    for (index, part) in parts.iter().enumerate() {
        let entries = load(&current)?;
        let found = entries.into_iter().find(|entry| entry.name == *part);
        let Some(entry) = found else {
            return Ok(None);
        };
        let last = index + 1 == parts.len();
        match entry.kind {
            EntryKind::Blob if last => {
                return Ok(Some(TreeFile {
                    path: path.to_string(),
                    mode: entry.mode,
                    hash: entry.hash,
                }));
            }
            EntryKind::Tree if !last => current = entry.hash,
            _ => return Ok(None),
        }
    }
    Ok(None)
}

pub(crate) fn join_name(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_string()
    } else {
        format!("{prefix}/{name}")
    }
}
