use std::path::Path;

use acvc_domain::{Hash, Result, plan_changes};

use crate::query::metrics_of;
use crate::{App, StatusReport};

impl App {
    pub fn workspace_variant(&self, workspace: &Path) -> Result<Hash> {
        Ok(self.ports.marker.load(workspace)?.variant)
    }

    pub fn status(&self, workspace: &Path) -> Result<StatusReport> {
        let marker = self.ports.marker.load(workspace)?;
        let stored = self.ports.variants.get_variant(&marker.variant)?;
        let schema = self.ports.campaigns.schema_of(&marker.campaign)?;
        let trials = self.ports.trials.trials_of(&marker.variant)?;
        let metrics = metrics_of(&trials, &schema).unwrap_or_default();
        let on_frontier = self
            .frontier(&marker.campaign)?
            .iter()
            .any(|row| row.variant == marker.variant);
        let index = self.ports.index.load(workspace)?;
        let listed = self.ports.scan.list_files(workspace)?;
        let plan = plan_changes(&index, &listed);
        Ok(StatusReport {
            campaign: marker.campaign,
            variant: marker.variant,
            claim: stored.variant.premise.claim,
            edge: stored.variant.edge,
            metrics,
            on_frontier,
            clean: plan.clean.len(),
            added: plan.added.into_iter().map(|file| file.path).collect(),
            deleted: plan.deleted,
            dirty: plan.dirty.into_iter().map(|file| file.path).collect(),
            hashed: 0,
        })
    }
}
