use std::path::Path;

use acvc_domain::{Hash, Result, is_complete};

use crate::place::write_tree;
use crate::{App, PromoteReport};

impl App {
    pub fn promote_dest(&self, variant: &Hash, dest: &Path) -> Result<PromoteReport> {
        self.ports.checkout.create_dirs(dest)?;
        let stored = self.ports.variants.get_variant(variant)?;
        let files = write_tree(self, stored.variant.tree, dest, None)?;
        Ok(PromoteReport {
            variant: *variant,
            dest: dest.to_path_buf(),
            files: files.len(),
            git: false,
        })
    }

    pub fn promote_git(&self, variant: &Hash, dest: &Path) -> Result<PromoteReport> {
        if !self.ports.git.is_work_tree(dest)? {
            return Err(
                acvc_domain::Error::new("destination is not a git work tree")
                    .ctx("path", dest.display().to_string()),
            );
        }
        let stored = self.ports.variants.get_variant(variant)?;
        let files = write_tree(self, stored.variant.tree, dest, None)?;
        let schema = self.ports.campaigns.schema_of(&stored.campaign)?;
        let trials = self.ports.trials.trials_of(variant)?;
        let metrics = trials
            .iter()
            .find(|trial| is_complete(&schema, &trial.trial.metrics))
            .map(|trial| trial.trial.metrics.clone());
        let mut message = stored.variant.premise.claim.clone();
        message.push_str("\n\n");
        if let Some(metrics) = metrics {
            for (name, value) in metrics {
                message.push_str(&format!("{name}={value}\n"));
            }
        }
        message.push_str(&format!("\nvariant {}\n", stored.variant.id()?));
        let paths: Vec<String> = files.iter().map(|file| file.path.clone()).collect();
        self.ports
            .git
            .commit_paths(dest, &paths, &message, &stored.variant.agent)?;
        Ok(PromoteReport {
            variant: *variant,
            dest: dest.to_path_buf(),
            files: files.len(),
            git: true,
        })
    }
}
