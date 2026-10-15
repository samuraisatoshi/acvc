use crate::{EntryKind, Hash, Result, TreeEntry};

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct TreeDiff {
    pub added: Vec<String>,
    pub removed: Vec<String>,
    pub modified: Vec<String>,
}

pub fn diff_trees(
    left: Hash,
    right: Hash,
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
) -> Result<TreeDiff> {
    let mut diff = TreeDiff::default();
    walk("", left, right, load, &mut diff)?;
    diff.added.sort();
    diff.removed.sort();
    diff.modified.sort();
    Ok(diff)
}

fn walk(
    prefix: &str,
    left: Hash,
    right: Hash,
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
    diff: &mut TreeDiff,
) -> Result<()> {
    if left == right {
        return Ok(());
    }
    let left_entries = load(&left)?;
    let right_entries = load(&right)?;
    let mut names = Vec::new();
    for entry in left_entries.iter().chain(right_entries.iter()) {
        if !names.contains(&entry.name.as_str()) {
            names.push(entry.name.as_str());
        }
    }
    names.sort_unstable();
    for name in names {
        let path = if prefix.is_empty() {
            name.to_string()
        } else {
            format!("{prefix}/{name}")
        };
        let left_entry = left_entries.iter().find(|entry| entry.name == name);
        let right_entry = right_entries.iter().find(|entry| entry.name == name);
        match (left_entry, right_entry) {
            (Some(old), None) => collect(&path, old, load, &mut diff.removed)?,
            (None, Some(new)) => collect(&path, new, load, &mut diff.added)?,
            (Some(old), Some(new))
                if old.kind != new.kind
                    || (old.kind == EntryKind::Blob && old.hash != new.hash) =>
            {
                diff.modified.push(path);
            }
            (Some(old), Some(new)) if old.kind == EntryKind::Tree => {
                walk(&path, old.hash, new.hash, load, diff)?;
            }
            _ => {}
        }
    }
    Ok(())
}

fn collect(
    path: &str,
    entry: &TreeEntry,
    load: &mut dyn FnMut(&Hash) -> Result<Vec<TreeEntry>>,
    into: &mut Vec<String>,
) -> Result<()> {
    match entry.kind {
        EntryKind::Blob => into.push(path.to_string()),
        EntryKind::Tree => {
            for child in load(&entry.hash)? {
                let child_path = format!("{path}/{}", child.name);
                collect(&child_path, &child, load, into)?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{EntryKind, Error, TREE_MODE, tree_hash};
    use std::collections::HashMap;

    #[test]
    fn equal_subtree_is_not_loaded() {
        let shared_blob = Hash::blob(b"same");
        let shared = tree_hash(&[TreeEntry {
            name: "same.txt".into(),
            mode: 0o100644,
            kind: EntryKind::Blob,
            hash: shared_blob,
        }])
        .unwrap();
        let left_blob = Hash::blob(b"left");
        let right_blob = Hash::blob(b"right");
        let left = tree_hash(&[
            TreeEntry {
                name: "keep".into(),
                mode: TREE_MODE,
                kind: EntryKind::Tree,
                hash: shared,
            },
            TreeEntry {
                name: "side.txt".into(),
                mode: 0o100644,
                kind: EntryKind::Blob,
                hash: left_blob,
            },
        ])
        .unwrap();
        let right = tree_hash(&[
            TreeEntry {
                name: "keep".into(),
                mode: TREE_MODE,
                kind: EntryKind::Tree,
                hash: shared,
            },
            TreeEntry {
                name: "side.txt".into(),
                mode: 0o100644,
                kind: EntryKind::Blob,
                hash: right_blob,
            },
        ])
        .unwrap();
        let mut trees = HashMap::new();
        trees.insert(
            left,
            vec![
                TreeEntry {
                    name: "keep".into(),
                    mode: TREE_MODE,
                    kind: EntryKind::Tree,
                    hash: shared,
                },
                TreeEntry {
                    name: "side.txt".into(),
                    mode: 0o100644,
                    kind: EntryKind::Blob,
                    hash: left_blob,
                },
            ],
        );
        trees.insert(
            right,
            vec![
                TreeEntry {
                    name: "keep".into(),
                    mode: TREE_MODE,
                    kind: EntryKind::Tree,
                    hash: shared,
                },
                TreeEntry {
                    name: "side.txt".into(),
                    mode: 0o100644,
                    kind: EntryKind::Blob,
                    hash: right_blob,
                },
            ],
        );
        let mut loads = Vec::new();
        let diff = diff_trees(left, right, &mut |hash| {
            loads.push(*hash);
            trees
                .get(hash)
                .cloned()
                .ok_or_else(|| Error::new("missing"))
        })
        .unwrap();
        assert_eq!(diff.modified, vec!["side.txt".to_string()]);
        assert!(!loads.contains(&shared));
    }
}
