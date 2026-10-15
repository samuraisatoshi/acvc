//! Domain model and ports. No IO.

#![forbid(unsafe_code)]

mod campaign;
mod codec;
mod diff;
mod error;
mod exchange;
mod hash;
mod index;
mod line_diff;
mod marker;
mod merkle;
mod pareto;
mod plan;
mod ports;
mod premise;
mod tree;
mod trial;
mod variant;

pub use campaign::{Direction, MetricSpec, validate_campaign_name, validate_schema};
pub use diff::{TreeDiff, diff_trees};
pub use error::{Error, Result};
pub use exchange::missing_hashes;
pub use hash::Hash;
pub use index::{FileStat, Fingerprint, IndexEntry, decode_index, encode_index, same_stat};
pub use line_diff::unified;
pub use marker::Marker;
pub use merkle::{PlannedFile, rebuild_merkle};
pub use pareto::{ScoreRelation, ScoredVariant, compare_scores, frontier};
pub use plan::{ListedFile, Plan, plan_changes};
pub use ports::{
    BlobCache, CampaignStore, CheckoutWrite, Clock, DirectoryScan, GitPort, IndexIo, MarkerIo,
    ObjectStore, PackReport, RecallStore, TrialStore, VariantStore, WorkspaceRegistry,
};
pub use premise::Premise;
pub use tree::{
    EXEC_MODE, EntryKind, FILE_MODE, TREE_MODE, TreeEntry, TreeFile, canonical_tree, decode_tree,
    lookup_file, tree_hash, valid_relative, walk_files,
};
pub use trial::{StoredTrial, Trial, is_complete, validate_score};
pub use variant::{EdgeKind, StoredVariant, Variant};
