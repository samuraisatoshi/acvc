use std::collections::BTreeSet;

use crate::{EntryKind, Error, Hash, Result, TREE_MODE, TreeEntry, canonical_tree, valid_relative};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlannedFile {
    pub path: String,
    pub mode: u32,
    pub hash: Hash,
}

pub fn rebuild_merkle(
    prev_root: Option<Hash>,
    files: &[PlannedFile],
    dirty: &[String],
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
    put: &mut dyn FnMut(&[TreeEntry]) -> Result<Hash>,
) -> Result<Hash> {
    for file in files {
        if !valid_relative(&file.path) {
            return Err(Error::new("invalid relative path").ctx("path", file.path.clone()));
        }
    }
    rebuild("", prev_root, files, dirty, load, put)
}

fn rebuild(
    dir: &str,
    prev: Option<Hash>,
    files: &[PlannedFile],
    dirty: &[String],
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
    put: &mut dyn FnMut(&[TreeEntry]) -> Result<Hash>,
) -> Result<Hash> {
    if !is_dirty(dir, dirty)
        && let Some(hash) = prev
    {
        return Ok(hash);
    }
    let prev_entries = if let Some(hash) = prev {
        load(&hash)?
    } else {
        Vec::new()
    };
    let mut subdirs = BTreeSet::new();
    for file in files {
        if let Some(name) = child_dir(dir, &file.path) {
            subdirs.insert(name);
        }
    }
    for entry in &prev_entries {
        if entry.kind == EntryKind::Tree {
            subdirs.insert(entry.name.clone());
        }
    }
    let mut entries = Vec::new();
    for name in subdirs {
        let child = join(dir, &name);
        if !has_files(&child, files) {
            continue;
        }
        let prev_child = prev_entries
            .iter()
            .find(|entry| entry.name == name && entry.kind == EntryKind::Tree)
            .map(|entry| entry.hash);
        let hash = rebuild(&child, prev_child, files, dirty, load, put)?;
        entries.push(TreeEntry {
            name,
            mode: TREE_MODE,
            kind: EntryKind::Tree,
            hash,
        });
    }
    for file in files {
        if let Some(name) = child_file(dir, &file.path) {
            entries.push(TreeEntry {
                name,
                mode: file.mode,
                kind: EntryKind::Blob,
                hash: file.hash,
            });
        }
    }
    entries.sort_by(|left, right| left.name.cmp(&right.name));
    if let Some(hash) = prev
        && trees_equal(&prev_entries, &entries)
    {
        return Ok(hash);
    }
    let _ = canonical_tree(&entries)?;
    put(&entries)
}

fn trees_equal(left: &[TreeEntry], right: &[TreeEntry]) -> bool {
    let mut left = left.to_vec();
    let mut right = right.to_vec();
    left.sort_by(|a, b| a.name.cmp(&b.name));
    right.sort_by(|a, b| a.name.cmp(&b.name));
    left == right
}

fn is_dirty(dir: &str, dirty: &[String]) -> bool {
    dirty.iter().any(|path| covers(dir, path))
}

fn has_files(dir: &str, files: &[PlannedFile]) -> bool {
    files.iter().any(|file| covers(dir, &file.path))
}

fn covers(dir: &str, path: &str) -> bool {
    if dir.is_empty() {
        return true;
    }
    path == dir || path.starts_with(&format!("{dir}/"))
}

fn join(dir: &str, name: &str) -> String {
    if dir.is_empty() {
        name.to_string()
    } else {
        format!("{dir}/{name}")
    }
}

fn relative<'a>(dir: &str, path: &'a str) -> Option<&'a str> {
    if dir.is_empty() {
        Some(path)
    } else {
        path.strip_prefix(dir)?.strip_prefix('/')
    }
}

fn child_dir(dir: &str, path: &str) -> Option<String> {
    let rest = relative(dir, path)?;
    let (name, tail) = rest.split_once('/')?;
    if name.is_empty() || tail.is_empty() {
        None
    } else {
        Some(name.to_string())
    }
}

fn child_file(dir: &str, path: &str) -> Option<String> {
    let rest = relative(dir, path)?;
    if rest.is_empty() || rest.contains('/') {
        None
    } else {
        Some(rest.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tree_hash;
    use std::cell::RefCell;
    use std::collections::HashMap;

    fn remember(
        stored: &RefCell<HashMap<Hash, Vec<TreeEntry>>>,
        entries: &[TreeEntry],
    ) -> Result<Hash> {
        let hash = tree_hash(entries).unwrap();
        stored.borrow_mut().insert(hash, entries.to_vec());
        Ok(hash)
    }

    #[test]
    fn clean_sibling_is_not_loaded_or_rewritten() {
        let one = Hash::blob(b"one");
        let two = Hash::blob(b"two");
        let one_changed = Hash::blob(b"one-v2");
        let stored = RefCell::new(HashMap::new());
        let base_files = vec![
            PlannedFile {
                path: "a/one.txt".into(),
                mode: 0o100644,
                hash: one,
            },
            PlannedFile {
                path: "b/two.txt".into(),
                mode: 0o100644,
                hash: two,
            },
        ];
        let dirty: Vec<String> = base_files.iter().map(|file| file.path.clone()).collect();
        let root = rebuild_merkle(
            None,
            &base_files,
            &dirty,
            &mut |_| unreachable!(),
            &mut |entries| remember(&stored, entries),
        )
        .unwrap();
        let sibling = stored
            .borrow()
            .get(&root)
            .unwrap()
            .iter()
            .find(|entry| entry.name == "b")
            .unwrap()
            .hash;
        let mut loads = Vec::new();
        let mut puts: Vec<Vec<String>> = Vec::new();
        let next = vec![
            PlannedFile {
                path: "a/one.txt".into(),
                mode: 0o100644,
                hash: one_changed,
            },
            PlannedFile {
                path: "b/two.txt".into(),
                mode: 0o100644,
                hash: two,
            },
        ];
        let new_root = rebuild_merkle(
            Some(root),
            &next,
            &["a/one.txt".into()],
            &mut |hash| {
                loads.push(*hash);
                stored
                    .borrow()
                    .get(hash)
                    .cloned()
                    .ok_or_else(|| Error::new("missing tree"))
            },
            &mut |entries| {
                puts.push(entries.iter().map(|entry| entry.name.clone()).collect());
                remember(&stored, entries)
            },
        )
        .unwrap();
        assert_ne!(new_root, root);
        assert!(!loads.contains(&sibling));
        assert!(
            !puts
                .iter()
                .any(|names| names == &vec!["two.txt".to_string()])
        );
        let new_sibling = stored
            .borrow()
            .get(&new_root)
            .unwrap()
            .iter()
            .find(|entry| entry.name == "b")
            .unwrap()
            .hash;
        assert_eq!(new_sibling, sibling);
    }
}
