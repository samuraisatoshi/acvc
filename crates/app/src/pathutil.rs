use std::path::{Path, PathBuf};

pub(crate) fn join_rel(root: &Path, relative: &str) -> PathBuf {
    let mut path = root.to_path_buf();
    for part in relative.split('/') {
        path.push(part);
    }
    path
}
