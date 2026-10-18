use std::cmp::Ordering;
use std::fmt::Write;

use acvc_app::{
    BacktrackReport, BeatenBy, CompareReport, FrontierRow, MaterializeReport, PromoteReport,
    RecallHit, RecordReport, ScoreReport, SeedReport, StatusReport, TransferReport, WhyReport,
};
use acvc_domain::{Direction, EdgeKind, Hash, MetricSpec, PackReport, ScoreRelation};
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
    let heaviest: Vec<Value> = report
        .heaviest
        .iter()
        .map(|item| json!({ "path": item.path, "bytes": item.bytes }))
        .collect();
    let value = json!({
        "variant": hex(&report.variant),
        "tree": hex(&report.tree),
        "files": report.files,
        "hashed": report.hashed,
        "heaviest": heaviest,
    });
    let mut text = format!(
        "variant {}\nfiles {} hashed {}",
        hex(&report.variant),
        report.files,
        report.hashed
    );
    for item in &report.heaviest {
        text.push_str(&format!(
            "\n{}",
            acvc_text::fill(
                "weighed",
                &[("path", item.path.as_str()), ("bytes", &item.bytes.to_string())]
            )
        ));
    }
    (value, text)
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

pub fn frontier(campaign: &str, rows: &[FrontierRow], schema: &[MetricSpec]) -> (Value, String) {
    let variants: Vec<Value> = rows.iter().map(|row| json!({ "variant": hex(&row.variant), "claim": row.claim, "metrics": row.metrics })).collect();
    let value = json!({ "campaign": campaign, "variants": variants });
    let text = if rows.is_empty() {
        acvc_text::tr("no_frontier").to_string()
    } else {
        rows.iter()
            .map(|row| format!("{}\n{}", row.claim, metric_line(&row.metrics, schema)))
            .collect::<Vec<_>>()
            .join("\n")
    };
    (value, text)
}

pub fn recall(hits: &[RecallHit]) -> (Value, String) {
    let items: Vec<Value> = hits
        .iter()
        .map(|hit| {
            json!({
                "variant": hex(&hit.variant),
                "claim": hit.claim,
                "status": hit.status,
                "parents": hit.parents.iter().map(|parent| json!({
                    "variant": hex(&parent.variant),
                    "claim": parent.claim,
                })).collect::<Vec<_>>(),
            })
        })
        .collect();
    let value = json!({ "hits": items });
    let text = if hits.is_empty() {
        acvc_text::tr("no_recall").to_string()
    } else {
        hits.iter()
            .map(|hit| {
                let from = if hit.parents.is_empty() {
                    acvc_text::tr("opened_campaign").to_string()
                } else {
                    acvc_text::fill(
                        "came_from",
                        &[(
                            "names",
                            &hit.parents
                                .iter()
                                .map(|parent| parent.claim.as_str())
                                .collect::<Vec<_>>()
                                .join("; "),
                        )],
                    )
                };
                format!(
                    "{}\n{}\n{from}",
                    hit.claim,
                    acvc_text::status_label(&hit.status)
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    };
    (value, text)
}

pub fn compare(report: &CompareReport) -> (Value, String) {
    let value = json!({
        "left": hex(&report.left),
        "right": hex(&report.right),
        "added": report.diff.added,
        "removed": report.diff.removed,
        "modified": report.diff.modified,
        "metrics": report.metrics.iter().map(|delta| json!({ "name": delta.name, "direction": delta.direction.as_str(), "left": delta.left, "right": delta.right })).collect::<Vec<_>>(),
        "left_claim": report.left_claim,
        "right_claim": report.right_claim,
        "dominance": relation(report.dominance),
    });
    let verdict = match report.dominance {
        ScoreRelation::LeftDominates => format!(
            "{} {}",
            acvc_text::fill("stays", &[("claim", report.left_claim.as_str())]),
            decided(true, report)
        ),
        ScoreRelation::RightDominates => format!(
            "{} {}",
            acvc_text::fill("stays", &[("claim", report.right_claim.as_str())]),
            decided(false, report)
        ),
        ScoreRelation::Tie => acvc_text::fill(
            "tie_both",
            &[
                ("left", report.left_claim.as_str()),
                ("right", report.right_claim.as_str()),
            ],
        ),
        ScoreRelation::Incomparable => format!(
            "{} {}",
            acvc_text::tr("none_stays"),
            report
                .metrics
                .iter()
                .map(|delta| {
                    acvc_text::fill(
                        "against",
                        &[
                            ("name", delta.name.as_str()),
                            ("loser", &number(delta.left)),
                            ("winner", &number(delta.right)),
                            ("sense", sense(delta.direction)),
                        ],
                    )
                })
                .collect::<Vec<_>>()
                .join("; ")
        ),
        ScoreRelation::Incomplete => acvc_text::tr("missing_measure").to_string(),
    };
    (
        value,
        format!(
            "{verdict}\n{}",
            acvc_text::fill(
                "different_files",
                &[("count", &report.diff.modified.len().to_string())]
            )
        ),
    )
}

fn decided(left_wins: bool, report: &CompareReport) -> String {
    report
        .metrics
        .iter()
        .filter_map(|delta| {
            let (win, lose) = if left_wins {
                (delta.left, delta.right)
            } else {
                (delta.right, delta.left)
            };
            let (Some(win), Some(lose)) = (win, lose) else {
                return None;
            };
            let better = match delta.direction {
                Direction::Higher => win.partial_cmp(&lose) == Some(Ordering::Greater),
                Direction::Lower => lose.partial_cmp(&win) == Some(Ordering::Greater),
            };
            better.then(|| {
                acvc_text::fill(
                    "against",
                    &[
                        ("name", delta.name.as_str()),
                        ("loser", &number(Some(lose))),
                        ("winner", &number(Some(win))),
                        ("sense", sense(delta.direction)),
                    ],
                )
            })
        })
        .collect::<Vec<_>>()
        .join("; ")
}

fn number(value: Option<f64>) -> String {
    value
        .map(count)
        .unwrap_or_else(|| acvc_text::tr("no_measurement").to_string())
}

pub(crate) fn count(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        value.to_string()
    }
}

fn sense(direction: Direction) -> &'static str {
    match direction {
        Direction::Higher => acvc_text::tr("higher_better"),
        Direction::Lower => acvc_text::tr("lower_better"),
    }
}

fn metric_line(metrics: &std::collections::BTreeMap<String, f64>, schema: &[MetricSpec]) -> String {
    if metrics.is_empty() {
        return acvc_text::tr("no_measurement").to_string();
    }
    metrics
        .iter()
        .map(|(name, value)| {
            let direction = schema
                .iter()
                .find(|spec| spec.name == *name)
                .map(|spec| sense(spec.direction))
                .unwrap_or(acvc_text::tr("metric_of_campaign"));
            format!("{name} {} ({direction})", count(*value))
        })
        .collect::<Vec<_>>()
        .join("; ")
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
        "on_frontier": report.on_frontier,
        "beaten_by": report.beaten_by.iter().map(beaten_json).collect::<Vec<_>>(),
        "parent_claims": report.parent_claims,
        "child_claims": report.child_claims,
    });
    let mut text = format!("{}\n", report.claim);
    if !report.assumptions.is_empty() {
        let _ = writeln!(
            text,
            "{}",
            acvc_text::fill("cause", &[("items", &report.assumptions.join(" · "))])
        );
    }
    if !report.intent.is_empty() {
        let _ = writeln!(
            text,
            "{}",
            acvc_text::fill("intent", &[("intent", report.intent.as_str())])
        );
    }
    let measured = report.trials.first().map(|trial| {
        trial
            .metrics
            .iter()
            .map(|(name, value)| format!("{name} {}", count(*value)))
            .collect::<Vec<_>>()
            .join("; ")
    });
    let _ = writeln!(
        text,
        "{}",
        acvc_text::fill(
            "measurement",
            &[(
                "text",
                &measured.unwrap_or_else(|| acvc_text::tr("no_measurement").to_string()),
            )]
        )
    );
    if let Some(reason) = &report.rejection {
        let _ = writeln!(
            text,
            "{}",
            acvc_text::fill("rejected", &[("reason", reason.as_str())])
        );
    } else if report.on_frontier {
        let _ = writeln!(text, "{}", acvc_text::tr("on_frontier"));
    } else {
        let _ = writeln!(text, "{}", acvc_text::tr("motive_unrecorded"));
    }
    for winner in &report.beaten_by {
        let _ = writeln!(text, "{}", beaten_line(winner));
    }
    if report.parent_claims.is_empty() {
        let _ = writeln!(text, "{}", acvc_text::tr("opened_campaign"));
    } else {
        let _ = writeln!(
            text,
            "{}",
            acvc_text::fill(
                "came_from",
                &[("names", &report.parent_claims.join("; "))]
            )
        );
    }
    let _ = writeln!(
        text,
        "{}",
        acvc_text::fill("agent", &[("agent", report.agent.as_str())])
    );
    (value, text)
}

pub(crate) fn beaten_json(winner: &BeatenBy) -> Value {
    json!({
        "variant": hex(&winner.variant),
        "claim": winner.claim,
        "gaps": winner.gaps.iter().map(|gap| json!({
            "name": gap.name,
            "direction": gap.direction.as_str(),
            "winner": gap.winner,
            "loser": gap.loser,
        })).collect::<Vec<_>>(),
    })
}

pub(crate) fn beaten_line(winner: &BeatenBy) -> String {
    let gaps = winner
        .gaps
        .iter()
        .map(|gap| {
            acvc_text::fill(
                "against",
                &[
                    ("name", gap.name.as_str()),
                    ("loser", &count(gap.loser)),
                    ("winner", &count(gap.winner)),
                    ("sense", sense(gap.direction)),
                ],
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    format!(
        "{} {gaps}",
        acvc_text::fill("beaten_by", &[("claim", winner.claim.as_str())])
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

#[cfg(test)]
#[path = "present_text_test.rs"]
mod present_text_test;
