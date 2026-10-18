use std::collections::BTreeMap;
use std::path::PathBuf;

use acvc_domain::{Direction, EdgeKind, Hash, ScoreRelation, TreeDiff};

#[derive(Clone, Debug)]
pub struct CampaignReport {
    pub name: String,
    pub metrics: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct HeavyPath {
    pub path: String,
    pub bytes: u64,
}

#[derive(Clone, Debug)]
pub struct SeedReport {
    pub variant: Hash,
    pub tree: Hash,
    pub files: usize,
    pub hashed: usize,
    pub heaviest: Vec<HeavyPath>,
}

#[derive(Clone, Debug)]
pub struct StatusReport {
    pub campaign: String,
    pub variant: Hash,
    pub claim: String,
    pub edge: EdgeKind,
    pub metrics: BTreeMap<String, f64>,
    pub on_frontier: bool,
    pub clean: usize,
    pub added: Vec<String>,
    pub deleted: Vec<String>,
    pub dirty: Vec<String>,
    pub hashed: usize,
}

#[derive(Clone, Debug)]
pub struct RecordReport {
    pub variant: Hash,
    pub tree: Hash,
    pub hashed: usize,
    pub reused: usize,
    pub edge: EdgeKind,
}

#[derive(Clone, Debug)]
pub struct ScoreReport {
    pub trial: Hash,
    pub variant: Hash,
}

#[derive(Clone, Debug)]
pub struct MaterializeReport {
    pub variant: Hash,
    pub dest: PathBuf,
    pub files: usize,
}

#[derive(Clone, Debug)]
pub struct FrontierRow {
    pub variant: Hash,
    pub claim: String,
    pub metrics: BTreeMap<String, f64>,
}

#[derive(Clone, Debug)]
pub struct MetricGap {
    pub name: String,
    pub direction: Direction,
    pub winner: f64,
    pub loser: f64,
}

#[derive(Clone, Debug)]
pub struct BeatenBy {
    pub variant: Hash,
    pub claim: String,
    pub gaps: Vec<MetricGap>,
}

#[derive(Clone, Debug)]
pub struct MetricDelta {
    pub name: String,
    pub direction: Direction,
    pub left: Option<f64>,
    pub right: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct CompareReport {
    pub left: Hash,
    pub right: Hash,
    pub left_claim: String,
    pub right_claim: String,
    pub diff: TreeDiff,
    pub metrics: Vec<MetricDelta>,
    pub dominance: ScoreRelation,
}

#[derive(Clone, Debug)]
pub struct RecallParent {
    pub variant: Hash,
    pub claim: String,
}

#[derive(Clone, Debug)]
pub struct RecallHit {
    pub variant: Hash,
    pub claim: String,
    pub status: String,
    pub parents: Vec<RecallParent>,
}

#[derive(Clone, Debug)]
pub struct TrialView {
    pub id: Hash,
    pub metrics: BTreeMap<String, f64>,
}

#[derive(Clone, Debug)]
pub struct WhyReport {
    pub variant: Hash,
    pub campaign: String,
    pub edge: EdgeKind,
    pub claim: String,
    pub assumptions: Vec<String>,
    pub intent: String,
    pub agent: String,
    pub parents: Vec<Hash>,
    pub children: Vec<Hash>,
    pub child_claims: Vec<String>,
    pub trials: Vec<TrialView>,
    pub rejection: Option<String>,
    pub on_frontier: bool,
    pub beaten_by: Vec<BeatenBy>,
    pub parent_claims: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct BacktrackReport {
    pub from: Hash,
    pub parent: Option<Hash>,
    pub parents: Vec<Hash>,
    pub edge: EdgeKind,
    pub claim: String,
    pub rejection: Option<String>,
    pub materialized: Option<PathBuf>,
}

#[derive(Clone, Debug)]
pub struct TransferReport {
    pub variant: Hash,
    pub objects_sent: u64,
    pub objects_skipped: u64,
    pub payload_bytes: u64,
    pub graph: String,
}

#[derive(Clone, Debug)]
pub struct PromoteReport {
    pub variant: Hash,
    pub dest: PathBuf,
    pub files: usize,
    pub git: bool,
}
