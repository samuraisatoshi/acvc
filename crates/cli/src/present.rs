use acvc_app::{
    BacktrackReport, CompareReport, FrontierRow, MaterializeReport, PromoteReport, RecallHit,
    RecordReport, ScoreReport, SeedReport, StatusReport, TransferReport, WhyReport,
};
use acvc_domain::{EdgeKind, Hash, PackReport, ScoreRelation};
use serde_json::{Value, json};

use crate::args::Format;

pub fn emit(format: Format, value: Value, text: String) {
    match format {
        Format::Json => println!("{value}"),
        Format::Text => println!("{text}"),
    }
}

pub fn hex(hash: &Hash) -> String {
    hash.to_hex()
}

fn edge(kind: EdgeKind) -> &'static str {
    kind.as_str()
}

fn relation(relation: ScoreRelation) -> &'static str {
    match relation {
        ScoreRelation::LeftDominates => "left",
        ScoreRelation::RightDominates => "right",
        ScoreRelation::Tie => "tie",
        ScoreRelation::Incomparable => "incomparable",
        ScoreRelation::Incomplete => "incomplete",
    }
}

pub fn seed(report: &SeedReport) -> (Value, String) {
    let value = json!({ "variant": hex(&report.variant), "tree": hex(&report.tree), "files": report.files, "hashed": report.hashed });
    (
        value,
        format!(
            "variant {}\nfiles {} hashed {}",
            hex(&report.variant),
            report.files,
            report.hashed
        ),
    )
}

pub fn status(report: &StatusReport) -> (Value, String) {
    let value = json!({
        "campaign": report.campaign,
        "variant": hex(&report.variant),
        "claim": report.claim,
        "edge": edge(report.edge),
        "metrics": report.metrics,
        "on_frontier": report.on_frontier,
        "clean": report.clean,
        "added": report.added,
        "deleted": report.deleted,
        "dirty": report.dirty,
        "hashed": report.hashed,
    });
    (
        value,
        format!(
            "{}\nclean {} dirty {} added {} deleted {}",
            report.claim,
            report.clean,
            report.dirty.len(),
            report.added.len(),
            report.deleted.len()
        ),
    )
}

pub fn record(report: &RecordReport) -> (Value, String) {
    let value = json!({ "variant": hex(&report.variant), "tree": hex(&report.tree), "hashed": report.hashed, "reused": report.reused, "edge": edge(report.edge) });
    (
        value,
        format!(
            "variant {}\nhashed {} reused {}",
            hex(&report.variant),
            report.hashed,
            report.reused
        ),
    )
}

pub fn score(report: &ScoreReport) -> (Value, String) {
    let value = json!({ "trial": hex(&report.trial), "variant": hex(&report.variant) });
    (value, format!("trial {}", hex(&report.trial)))
}

pub fn materialize(report: &MaterializeReport) -> (Value, String) {
    let value =
        json!({ "variant": hex(&report.variant), "dest": report.dest, "files": report.files });
    (
        value,
        format!(
            "materialized {} files into {}",
            report.files,
            report.dest.display()
        ),
    )
}

pub fn frontier(campaign: &str, rows: &[FrontierRow]) -> (Value, String) {
    let variants: Vec<Value> = rows.iter().map(|row| json!({ "variant": hex(&row.variant), "claim": row.claim, "metrics": row.metrics })).collect();
    let value = json!({ "campaign": campaign, "variants": variants });
    let text = rows
        .iter()
        .map(|row| format!("{} {}", hex(&row.variant), row.claim))
        .collect::<Vec<_>>()
        .join("\n");
    (value, text)
}

pub fn recall(hits: &[RecallHit]) -> (Value, String) {
    let items: Vec<Value> = hits
        .iter()
        .map(|hit| json!({ "variant": hex(&hit.variant), "claim": hit.claim }))
        .collect();
    let value = json!({ "hits": items });
    let text = hits
        .iter()
        .map(|hit| format!("{} {}", hex(&hit.variant), hit.claim))
        .collect::<Vec<_>>()
        .join("\n");
    (value, text)
}

pub fn compare(report: &CompareReport) -> (Value, String) {
    let value = json!({
        "left": hex(&report.left),
        "right": hex(&report.right),
        "added": report.diff.added,
        "removed": report.diff.removed,
        "modified": report.diff.modified,
        "metrics": report.metrics.iter().map(|delta| json!({ "name": delta.name, "left": delta.left, "right": delta.right })).collect::<Vec<_>>(),
        "dominance": relation(report.dominance),
    });
    (
        value,
        format!(
            "dominance {}\nmodified {}",
            relation(report.dominance),
            report.diff.modified.len()
        ),
    )
}

pub fn file_diff(left: &Hash, right: &Hash, path: &str, diff: &str) -> (Value, String) {
    (
        json!({ "left": hex(left), "right": hex(right), "path": path, "diff": diff }),
        diff.to_string(),
    )
}

pub fn why(report: &WhyReport) -> (Value, String) {
    let value = json!({
        "variant": hex(&report.variant),
        "campaign": report.campaign,
        "edge": edge(report.edge),
        "claim": report.claim,
        "assumptions": report.assumptions,
        "intent": report.intent,
        "agent": report.agent,
        "parents": report.parents.iter().map(hex).collect::<Vec<_>>(),
        "children": report.children.iter().map(hex).collect::<Vec<_>>(),
        "trials": report.trials.iter().map(|trial| json!({ "id": hex(&trial.id), "metrics": trial.metrics })).collect::<Vec<_>>(),
        "rejection": report.rejection,
    });
    (
        value,
        format!(
            "{} {}\n{}",
            edge(report.edge),
            hex(&report.variant),
            report.claim
        ),
    )
}

pub fn backtrack(report: &BacktrackReport) -> (Value, String) {
    let value = json!({
        "from": hex(&report.from),
        "parent": report.parent.as_ref().map(hex),
        "parents": report.parents.iter().map(hex).collect::<Vec<_>>(),
        "edge": edge(report.edge),
        "claim": report.claim,
        "rejection": report.rejection,
        "materialized": report.materialized,
    });
    (
        value,
        format!(
            "parent {}",
            report
                .parent
                .as_ref()
                .map(hex)
                .unwrap_or_else(|| "none".into())
        ),
    )
}

pub fn listening(addr: &str) -> (Value, String) {
    (json!({ "listening": addr }), format!("listening {addr}"))
}

pub fn transfer(report: &TransferReport) -> (Value, String) {
    let value = json!({
        "variant": hex(&report.variant),
        "objects_sent": report.objects_sent,
        "objects_skipped": report.objects_skipped,
        "payload_bytes": report.payload_bytes,
        "graph": report.graph,
    });
    (
        value,
        format!(
            "variant {} sent {} skipped {}",
            hex(&report.variant),
            report.objects_sent,
            report.objects_skipped
        ),
    )
}

pub fn pack(report: &PackReport) -> (Value, String) {
    let value = json!({
        "objects": report.objects,
        "raw_bytes": report.raw_bytes,
        "packed_bytes": report.packed_bytes,
    });
    (value, format!("packed {} objects", report.objects))
}

pub fn promote(report: &PromoteReport) -> (Value, String) {
    let value = json!({ "variant": hex(&report.variant), "dest": report.dest, "files": report.files, "git": report.git });
    (value, format!("promoted {} files", report.files))
}
