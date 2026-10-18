use std::collections::BTreeMap;
use std::path::Path;

use std::cmp::Ordering;

use acvc_domain::{
    Direction, Error, Hash, MetricSpec, Result, ScoreRelation, ScoredVariant, StoredTrial,
    compare_scores, diff_trees, frontier, is_complete, lookup_file, unified,
};

use crate::{
    App, BacktrackReport, BeatenBy, CompareReport, FrontierRow, MetricDelta, MetricGap, RecallHit,
    TrialView, WhyReport,
};

impl App {
    pub fn campaigns(&self) -> Result<Vec<String>> {
        self.ports.campaigns.list_campaigns()
    }

    pub fn schema(&self, campaign: &str) -> Result<Vec<MetricSpec>> {
        self.ports.campaigns.schema_of(campaign)
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
            let rejected = self.ports.trials.rejection_of(&id)?.is_some();
            for trial in undominated_complete(&trials, &schema) {
                scored.push(ScoredVariant {
                    variant: id,
                    metrics: trial.trial.metrics.clone(),
                    rejected,
                });
            }
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
        let schema = self.ports.campaigns.schema_of(campaign)?;
        let frontier_rows = self.frontier(campaign)?;
        let mut hits = Vec::new();
        for id in self.ports.recall.recall(campaign, query)? {
            let stored = self.ports.variants.get_variant(&id)?;
            let trials = self.ports.trials.trials_of(&id)?;
            let rejection = self.ports.trials.rejection_of(&id)?;
            let on_frontier = frontier_rows.iter().any(|row| row.variant == id);
            let measured = metrics_of(&trials, &schema).is_some();
            let status = if rejection.is_some() {
                "rejeitada"
            } else if on_frontier {
                "na fronteira"
            } else if measured {
                "superada"
            } else {
                "sem medição"
            };
            let mut parents = Vec::new();
            for parent in &stored.variant.parents {
                let claim = self
                    .ports
                    .variants
                    .get_variant(parent)?
                    .variant
                    .premise
                    .claim;
                parents.push(crate::RecallParent {
                    variant: *parent,
                    claim,
                });
            }
            hits.push(RecallHit {
                variant: id,
                claim: stored.variant.premise.claim,
                status: status.to_string(),
                parents,
            });
        }
        Ok(hits)
    }

    pub fn why(&self, variant: &Hash) -> Result<WhyReport> {
        let stored = self.ports.variants.get_variant(variant)?;
        let trials = self.ports.trials.trials_of(variant)?;
        let schema = self.ports.campaigns.schema_of(&stored.campaign)?;
        let metrics = metrics_of(&trials, &schema);
        let frontier_rows = self.frontier(&stored.campaign)?;
        let on_frontier = frontier_rows.iter().any(|row| row.variant == *variant);
        let rejection = self.ports.trials.rejection_of(variant)?;
        let beaten_by = if rejection.is_none() && !on_frontier {
            who_beats(
                &schema,
                metrics.as_ref(),
                &frontier_rows
                    .iter()
                    .map(|row| (row.variant, row.claim.as_str(), &row.metrics))
                    .collect::<Vec<_>>(),
            )?
        } else {
            Vec::new()
        };
        let mut parent_claims = Vec::new();
        for parent in &stored.variant.parents {
            parent_claims.push(self.claim_of(parent)?);
        }
        let children = self.ports.variants.children(variant)?;
        let mut child_claims = Vec::new();
        for child in &children {
            child_claims.push(self.claim_of(child)?);
        }
        Ok(WhyReport {
            variant: *variant,
            campaign: stored.campaign,
            edge: stored.variant.edge,
            claim: stored.variant.premise.claim,
            assumptions: stored.variant.premise.assumptions,
            intent: stored.variant.intent,
            agent: stored.variant.agent,
            parents: stored.variant.parents,
            children,
            child_claims,
            trials: trials
                .into_iter()
                .map(|trial| TrialView {
                    id: trial.trial.id(),
                    metrics: trial.trial.metrics,
                })
                .collect(),
            rejection,
            on_frontier,
            beaten_by,
            parent_claims,
        })
    }

    fn claim_of(&self, variant: &Hash) -> Result<String> {
        Ok(self
            .ports
            .variants
            .get_variant(variant)?
            .variant
            .premise
            .claim)
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
            .map(|name| {
                let direction = schema
                    .iter()
                    .find(|spec| spec.name == name)
                    .map(|spec| spec.direction)
                    .unwrap_or(Direction::Higher);
                MetricDelta {
                    left: left_metrics
                        .as_ref()
                        .and_then(|values| values.get(&name).copied()),
                    right: right_metrics
                        .as_ref()
                        .and_then(|values| values.get(&name).copied()),
                    direction,
                    name,
                }
            })
            .collect();
        Ok(CompareReport {
            left: *left,
            right: *right,
            left_claim: left_stored.variant.premise.claim,
            right_claim: right_stored.variant.premise.claim,
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

pub(crate) fn who_beats(
    schema: &[MetricSpec],
    loser: Option<&BTreeMap<String, f64>>,
    frontier_rows: &[(Hash, &str, &BTreeMap<String, f64>)],
) -> Result<Vec<BeatenBy>> {
    let Some(loser) = loser else {
        return Ok(Vec::new());
    };
    let mut beaten = Vec::new();
    for (variant, claim, metrics) in frontier_rows {
        if compare_scores(schema, metrics, loser)? != ScoreRelation::LeftDominates {
            continue;
        }
        beaten.push(BeatenBy {
            variant: *variant,
            claim: (*claim).to_string(),
            gaps: gaps(schema, metrics, loser),
        });
    }
    beaten.sort_by(|left, right| left.claim.cmp(&right.claim));
    Ok(beaten)
}

fn gaps(
    schema: &[MetricSpec],
    winner: &BTreeMap<String, f64>,
    loser: &BTreeMap<String, f64>,
) -> Vec<MetricGap> {
    let mut found = Vec::new();
    for spec in schema {
        let Some(win) = winner.get(&spec.name).copied() else {
            continue;
        };
        let Some(lose) = loser.get(&spec.name).copied() else {
            continue;
        };
        let order = match spec.direction {
            Direction::Higher => win.partial_cmp(&lose),
            Direction::Lower => lose.partial_cmp(&win),
        };
        if order == Some(Ordering::Greater) {
            found.push(MetricGap {
                name: spec.name.clone(),
                direction: spec.direction,
                winner: win,
                loser: lose,
            });
        }
    }
    found
}

fn undominated_complete<'a>(
    trials: &'a [StoredTrial],
    schema: &[MetricSpec],
) -> Vec<&'a StoredTrial> {
    let complete: Vec<usize> = trials
        .iter()
        .enumerate()
        .filter(|(_, trial)| is_complete(schema, &trial.trial.metrics))
        .map(|(index, _)| index)
        .collect();
    complete
        .iter()
        .copied()
        .filter(|index| {
            !complete.iter().any(|other| {
                *other != *index
                    && matches!(
                        compare_scores(
                            schema,
                            &trials[*other].trial.metrics,
                            &trials[*index].trial.metrics,
                        ),
                        Ok(ScoreRelation::LeftDominates)
                    )
            })
        })
        .map(|index| &trials[index])
        .collect()
}

fn best_complete<'a>(trials: &'a [StoredTrial], schema: &[MetricSpec]) -> Option<&'a StoredTrial> {
    undominated_complete(trials, schema).into_iter().next()
}

pub(crate) fn metrics_of(
    trials: &[StoredTrial],
    schema: &[MetricSpec],
) -> Option<BTreeMap<String, f64>> {
    best_complete(trials, schema).map(|trial| trial.trial.metrics.clone())
}

fn preview(app: &App, hash: &Hash) -> Result<String> {
    let bytes = app.ports.objects.get_blob(hash)?;
    match std::str::from_utf8(&bytes) {
        Ok(text) if !text.contains('\0') => Ok(text.to_string()),
        _ => Ok(format!("binary {} bytes", bytes.len())),
    }
}
