use std::collections::HashMap;
use std::path::{Path, PathBuf};

use acvc_domain::{Hash, Result, TreeFile, walk_files};

use crate::App;
use crate::pathutil::join_rel;

pub(crate) fn write_tree(
    app: &App,
    tree: Hash,
    dest: &Path,
    parent: Option<(Hash, PathBuf)>,
) -> Result<Vec<TreeFile>> {
    let files = walk_files(tree, &mut |hash| app.ports.objects.get_tree(hash))?;
    let parent_hashes = match &parent {
        Some((parent_tree, _)) => {
            walk_files(*parent_tree, &mut |hash| app.ports.objects.get_tree(hash))?
                .into_iter()
                .map(|file| (file.path, file.hash))
                .collect::<HashMap<_, _>>()
        }
        None => HashMap::new(),
    };
    for file in &files {
        let dest_path = join_rel(dest, &file.path);
        let from_parent = parent.as_ref().and_then(|(_, workspace)| {
            if parent_hashes.get(&file.path) == Some(&file.hash) {
                let candidate = join_rel(workspace, &file.path);
                app.ports.scan.metadata(&candidate).ok().map(|_| candidate)
            } else {
                None
            }
        });
        let source = match from_parent {
            Some(path) => path,
            None => app.ports.cache.materialize_blob(&file.hash)?,
        };
        app.ports
            .checkout
            .place_reflink(&source, &dest_path, file.mode)?;
    }
    Ok(files)
}
