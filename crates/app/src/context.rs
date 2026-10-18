use std::collections::{BTreeMap, BTreeSet};

use acvc_domain::{EdgeKind, Error, Hash, MetricSpec, Result, Spend};

use crate::query::{metrics_of, who_beats};
use crate::{App, BeatenBy};

pub struct ContextNode {
    pub variant: Hash,
    pub claim: String,
    pub agent: String,
    pub edge: EdgeKind,
    pub assumptions: Vec<String>,
    pub intent: String,
    pub created_at: i64,
    pub status: &'static str,
    pub metrics: BTreeMap<String, f64>,
    pub rejection: Option<String>,
    pub beaten_by: Vec<BeatenBy>,
    pub children: Vec<ContextNode>,
}

pub struct ContextReport {
    pub campaign: String,
    pub opened_at: i64,
    pub now: i64,
    pub schema: Vec<MetricSpec>,
    pub agents: Vec<String>,
    pub variants: usize,
    pub trials: usize,
    pub rejections: usize,
    pub edges: BTreeMap<String, usize>,
    pub tokens: u64,
    pub spends: Vec<Spend>,
    pub graph: Vec<ContextNode>,
}

struct Row {
    claim: String,
    agent: String,
    edge: EdgeKind,
    assumptions: Vec<String>,
    intent: String,
    parents: Vec<Hash>,
    created_at: i64,
    status: &'static str,
    metrics: BTreeMap<String, f64>,
    rejection: Option<String>,
    beaten_by: Vec<BeatenBy>,
}

impl App {
    pub fn context(&self, campaign: &str) -> Result<ContextReport> {
        let schema = self.ports.campaigns.schema_of(campaign)?;
        let opened_at = self.ports.campaigns.opened_at(campaign)?;
        let stored = self.ports.variants.variants_in(campaign)?;
        let frontier = self.frontier(campaign)?;
        let on_frontier: BTreeSet<Hash> = frontier.into_iter().map(|row| row.variant).collect();
        let mut agents = BTreeSet::new();
        let mut edges = BTreeMap::new();
        let mut trials = 0usize;
        let mut rejections = 0usize;
        let mut rows = BTreeMap::new();
        for item in stored {
            let id = item.variant.id()?;
            let trial_rows = self.ports.trials.trials_of(&id)?;
            trials += trial_rows.len();
            let rejection = self.ports.trials.rejection_of(&id)?;
            if rejection.is_some() {
                rejections += 1;
            }
            let metrics = metrics_of(&trial_rows, &schema).unwrap_or_default();
            let status = status_of(
                rejection.is_some(),
                on_frontier.contains(&id),
                !metrics.is_empty(),
            );
            *edges
                .entry(item.variant.edge.as_str().to_string())
                .or_insert(0) += 1;
            agents.insert(item.variant.agent.clone());
            rows.insert(
                id,
                Row {
                    claim: item.variant.premise.claim,
                    agent: item.variant.agent,
                    edge: item.variant.edge,
                    assumptions: item.variant.premise.assumptions,
                    intent: item.variant.intent,
                    parents: item.variant.parents,
                    created_at: item.created_at,
                    status,
                    metrics,
                    rejection,
                    beaten_by: Vec::new(),
                },
            );
        }
        let winners: Vec<(Hash, String, BTreeMap<String, f64>)> = rows
            .iter()
            .filter(|(_, row)| row.status == "na fronteira")
            .map(|(id, row)| (*id, row.claim.clone(), row.metrics.clone()))
            .collect();
        let winner_refs: Vec<(Hash, &str, &BTreeMap<String, f64>)> = winners
            .iter()
            .map(|(id, claim, metrics)| (*id, claim.as_str(), metrics))
            .collect();
        for (id, row) in rows.iter_mut() {
            if row.status != "superada" {
                continue;
            }
            row.beaten_by = who_beats(&schema, Some(&row.metrics), &winner_refs)?
                .into_iter()
                .filter(|winner| winner.variant != *id)
                .collect();
        }
        let spends = self.ports.spends.spends_of(campaign)?;
        let tokens = spends.iter().map(|spend| spend.tokens).sum();
        for spend in &spends {
            agents.insert(spend.agent.clone());
        }
        let graph = tree(&rows)?;
        Ok(ContextReport {
            campaign: campaign.to_string(),
            opened_at,
            now: self.ports.clock.now_millis(),
            schema,
            agents: agents.into_iter().collect(),
            variants: rows.len(),
            trials,
            rejections,
            edges,
            tokens,
            spends,
            graph,
        })
    }

    pub fn spend(&self, campaign: &str, agent: &str, tokens: u64, note: &str) -> Result<u64> {
        if tokens == 0 {
            return Err(Error::new("tokens must be greater than zero"));
        }
        if agent.is_empty() {
            return Err(Error::new("agent is empty"));
        }
        self.ports.campaigns.schema_of(campaign)?;
        self.ports.spends.insert_spend(
            campaign,
            agent,
            tokens,
            note,
            self.ports.clock.now_millis(),
        )?;
        Ok(self
            .ports
            .spends
            .spends_of(campaign)?
            .iter()
            .map(|spend| spend.tokens)
            .sum())
    }
}

fn status_of(rejected: bool, frontier: bool, measured: bool) -> &'static str {
    if rejected {
        "rejeitada"
    } else if frontier {
        "na fronteira"
    } else if measured {
        "superada"
    } else {
        "sem medição"
    }
}

fn tree(rows: &BTreeMap<Hash, Row>) -> Result<Vec<ContextNode>> {
    let ids: BTreeSet<Hash> = rows.keys().copied().collect();
    let mut children: BTreeMap<Hash, Vec<Hash>> = BTreeMap::new();
    let mut roots = Vec::new();
    for (id, row) in rows {
        let mut linked = false;
        for parent in &row.parents {
            if ids.contains(parent) {
                children.entry(*parent).or_default().push(*id);
                linked = true;
            }
        }
        if !linked {
            roots.push(*id);
        }
    }
    let by_time = |id: &Hash| {
        rows.get(id)
            .map(|row| (row.created_at, *id))
            .unwrap_or((0, *id))
    };
    roots.sort_by_key(by_time);
    for kids in children.values_mut() {
        kids.sort_by_key(by_time);
    }
    roots
        .into_iter()
        .map(|id| node(id, rows, &children, &mut Vec::new()))
        .collect()
}

fn node(
    id: Hash,
    rows: &BTreeMap<Hash, Row>,
    children: &BTreeMap<Hash, Vec<Hash>>,
    stack: &mut Vec<Hash>,
) -> Result<ContextNode> {
    if stack.contains(&id) {
        return Err(Error::new("variant graph has a cycle").ctx("variant", id.to_hex()));
    }
    let row = rows
        .get(&id)
        .ok_or_else(|| Error::new("variant disappeared").ctx("variant", id.to_hex()))?;
    stack.push(id);
    let mut kids = Vec::new();
    if let Some(list) = children.get(&id) {
        for child in list {
            kids.push(node(*child, rows, children, stack)?);
        }
    }
    stack.pop();
    Ok(ContextNode {
        variant: id,
        claim: row.claim.clone(),
        agent: row.agent.clone(),
        edge: row.edge,
        assumptions: row.assumptions.clone(),
        intent: row.intent.clone(),
        created_at: row.created_at,
        status: row.status,
        metrics: row.metrics.clone(),
        rejection: row.rejection.clone(),
        beaten_by: row.beaten_by.clone(),
        children: kids,
    })
}
