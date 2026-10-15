use std::path::Path;

use acvc_domain::{Hash, Result, Variant};

use crate::place::write_tree;
use crate::{App, MaterializeReport};

impl App {
    pub fn materialize(&self, variant: &Hash, dest: &Path) -> Result<MaterializeReport> {
        self.ports.checkout.prepare_empty_dir(dest)?;
        let stored = self.ports.variants.get_variant(variant)?;
        let parent = self.parent_checkout(&stored.variant)?;
        let files = write_tree(self, stored.variant.tree, dest, parent)?;
        let planned = files
            .iter()
            .map(|file| acvc_domain::PlannedFile {
                path: file.path.clone(),
                mode: file.mode,
                hash: file.hash,
            })
            .collect::<Vec<_>>();
        self.persist_workspace(dest, &stored.campaign, stored.variant.id()?, &planned)?;
        Ok(MaterializeReport {
            variant: *variant,
            dest: dest.to_path_buf(),
            files: files.len(),
        })
    }

    fn parent_checkout(&self, variant: &Variant) -> Result<Option<(Hash, std::path::PathBuf)>> {
        let Some(parent) = variant.parents.first() else {
            return Ok(None);
        };
        let Some(path) = self.ports.workspaces.find(parent)? else {
            return Ok(None);
        };
        if self.ports.scan.metadata(&path).is_err() {
            return Ok(None);
        }
        let stored = self.ports.variants.get_variant(parent)?;
        Ok(Some((stored.variant.tree, path)))
    }
}
