use std::collections::BTreeMap;
use std::path::Path;

use acvc_domain::{
    Error, Hash, MetricSpec, Result, ScoreRelation, ScoredVariant, StoredTrial, compare_scores,
    diff_trees, frontier, is_complete, lookup_file, unified,
};

use crate::{
    App, BacktrackReport, CompareReport, FrontierRow, MetricDelta, RecallHit, TrialView, WhyReport,
};

impl App {
    pub fn campaigns(&self) -> Result<Vec<String>> {
        self.ports.campaigns.list_campaigns()
    }

    pub fn resolve(&self, prefix: &str) -> Result<Hash> {
        self.ports.variants.resolve(prefix)
    }

    pub fn read_tree(&self, hash: &Hash) -> Result<Vec<acvc_domain::TreeEntry>> {
        self.ports.objects.get_tree(hash)
    }

    pub fn frontier(&self, campaign: &str) -> Result<Vec<FrontierRow>> {
        let schema = self.ports.campaigns.schema_of(campaign)?;
        let variants = self.ports.variants.variants_in(campaign)?;
        let mut scored = Vec::new();
        for stored in &variants {
            let id = stored.variant.id()?;
            let trials = self.ports.trials.trials_of(&id)?;
            let Some(trial) = latest_complete(&trials, &schema) else {
                continue;
            };
            scored.push(ScoredVariant {
                variant: id,
                metrics: trial.trial.metrics.clone(),
                rejected: self.ports.trials.rejection_of(&id)?.is_some(),
            });
        }
        let chosen = frontier(&schema, &scored)?;
        let mut rows = Vec::new();
        for item in chosen {
            let stored = variants
                .iter()
                .find(|stored| stored.variant.id().ok() == Some(item.variant))
                .ok_or_else(|| Error::new("frontier variant disappeared"))?;
            rows.push(FrontierRow {
                variant: item.variant,
                claim: stored.variant.premise.claim.clone(),
                metrics: item.metrics.clone(),
            });
        }
        Ok(rows)
    }

    pub fn recall(&self, campaign: &str, query: &str) -> Result<Vec<RecallHit>> {
        let mut hits = Vec::new();
        for id in self.ports.recall.recall(campaign, query)? {
            let stored = self.ports.variants.get_variant(&id)?;
            hits.push(RecallHit {
                variant: id,
                claim: stored.variant.premise.claim,
            });
        }
        Ok(hits)
    }

    pub fn why(&self, variant: &Hash) -> Result<WhyReport> {
        let stored = self.ports.variants.get_variant(variant)?;
        let trials = self.ports.trials.trials_of(variant)?;
        Ok(WhyReport {
            variant: *variant,
            campaign: stored.campaign,
            edge: stored.variant.edge,
            claim: stored.variant.premise.claim,
            assumptions: stored.variant.premise.assumptions,
            intent: stored.variant.intent,
            agent: stored.variant.agent,
            parents: stored.variant.parents,
            children: self.ports.variants.children(variant)?,
            trials: trials
                .into_iter()
                .map(|trial| TrialView {
                    id: trial.trial.id(),
                    metrics: trial.trial.metrics,
                })
                .collect(),
            rejection: self.ports.trials.rejection_of(variant)?,
        })
    }

    pub fn compare(&self, left: &Hash, right: &Hash) -> Result<CompareReport> {
        let left_stored = self.ports.variants.get_variant(left)?;
        let right_stored = self.ports.variants.get_variant(right)?;
        if left_stored.campaign != right_stored.campaign {
            return Err(Error::new("variants are in different campaigns"));
        }
        let schema = self.ports.campaigns.schema_of(&left_stored.campaign)?;
        let diff = diff_trees(
            left_stored.variant.tree,
            right_stored.variant.tree,
            &mut |hash| self.ports.objects.get_tree(hash),
        )?;
        let left_metrics = metrics_of(&self.ports.trials.trials_of(left)?, &schema);
        let right_metrics = metrics_of(&self.ports.trials.trials_of(right)?, &schema);
        let dominance = match (&left_metrics, &right_metrics) {
            (Some(left_values), Some(right_values)) => {
                compare_scores(&schema, left_values, right_values)?
            }
            _ => ScoreRelation::Incomplete,
        };
        let mut names: Vec<String> = schema.iter().map(|spec| spec.name.clone()).collect();
        names.sort();
        let metrics = names
            .into_iter()
            .map(|name| MetricDelta {
                left: left_metrics
                    .as_ref()
                    .and_then(|values| values.get(&name).copied()),
                right: right_metrics
                    .as_ref()
                    .and_then(|values| values.get(&name).copied()),
                name,
            })
            .collect();
        Ok(CompareReport {
            left: *left,
            right: *right,
            diff,
            metrics,
            dominance,
        })
    }

    pub fn backtrack(&self, variant: &Hash, into: Option<&Path>) -> Result<BacktrackReport> {
        let stored = self.ports.variants.get_variant(variant)?;
        if stored.variant.parents.is_empty() {
            return Err(Error::new("seed has no parent").ctx("variant", variant.to_hex()));
        }
        let parent = stored.variant.parents[0];
        let materialized = if let Some(dest) = into {
            self.materialize(&parent, dest)?;
            Some(dest.to_path_buf())
        } else {
            None
        };
        Ok(BacktrackReport {
            from: *variant,
            parent: Some(parent),
            parents: stored.variant.parents,
            edge: stored.variant.edge,
            claim: stored.variant.premise.claim,
            rejection: self.ports.trials.rejection_of(variant)?,
            materialized,
        })
    }

    pub fn file_diff(&self, left: &Hash, right: &Hash, path: &str) -> Result<String> {
        let left_tree = self.ports.variants.get_variant(left)?.variant.tree;
        let right_tree = self.ports.variants.get_variant(right)?.variant.tree;
        let left_file = lookup_file(left_tree, path, &mut |hash| {
            self.ports.objects.get_tree(hash)
        })?;
        let right_file = lookup_file(right_tree, path, &mut |hash| {
            self.ports.objects.get_tree(hash)
        })?;
        let left_text = left_file
            .map(|file| preview(self, &file.hash))
            .transpose()?;
        let right_text = right_file
            .map(|file| preview(self, &file.hash))
            .transpose()?;
        Ok(match (left_text, right_text) {
            (None, None) => {
                return Err(Error::new("path is not in either variant").ctx("path", path));
            }
            (None, Some(text)) => format!("added\n{text}"),
            (Some(text), None) => format!("removed\n{text}"),
            (Some(before), Some(after)) => unified(&before, &after),
        })
    }
}

fn latest_complete<'a>(
    trials: &'a [StoredTrial],
    schema: &[MetricSpec],
) -> Option<&'a StoredTrial> {
    trials
        .iter()
        .find(|trial| is_complete(schema, &trial.trial.metrics))
}

pub(crate) fn metrics_of(
    trials: &[StoredTrial],
    schema: &[MetricSpec],
) -> Option<BTreeMap<String, f64>> {
    latest_complete(trials, schema).map(|trial| trial.trial.metrics.clone())
}

fn preview(app: &App, hash: &Hash) -> Result<String> {
    let bytes = app.ports.objects.get_blob(hash)?;
    match std::str::from_utf8(&bytes) {
        Ok(text) if !text.contains('\0') => Ok(text.to_string()),
        _ => Ok(format!("binary {} bytes", bytes.len())),
    }
}
