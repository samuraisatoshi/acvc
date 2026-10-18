use std::fmt::Write;

use acvc_app::ContextNode;
use acvc_app::ContextReport;
use serde_json::{Value, json};

use crate::present::{beaten_json, beaten_line, count, hex};

pub fn context(report: &ContextReport) -> (Value, String) {
    let problem = problem_of(&report.graph);
    let need = need_of(&report.graph);
    let open_for = open_for(report.opened_at, report.now);
    let tokens_label = tokens_label(report.tokens);
    let value = json!({
        "campaign": report.campaign,
        "opened_at": report.opened_at,
        "open_for": open_for,
        "problem": problem,
        "need": need,
        "agents": report.agents,
        "interactions": {
            "variants": report.variants,
            "trials": report.trials,
            "rejections": report.rejections,
            "edges": report.edges,
        },
        "tokens": report.tokens,
        "tokens_label": tokens_label,
        "spends": report.spends.iter().map(|spend| json!({
            "agent": spend.agent,
            "tokens": spend.tokens,
            "note": spend.note,
            "created_at": spend.created_at,
        })).collect::<Vec<_>>(),
        "schema": report.schema.iter().map(|spec| json!({
            "name": spec.name,
            "direction": spec.direction.as_str(),
        })).collect::<Vec<_>>(),
        "graph": report.graph.iter().map(node_json).collect::<Vec<_>>(),
    });
    let mut text = String::new();
    let _ = writeln!(
        text,
        "{}",
        acvc_text::fill("campaign_line", &[("name", report.campaign.as_str())])
    );
    let _ = writeln!(text, "{}", acvc_text::fill("problem", &[("text", &problem)]));
    if !need.is_empty() {
        let _ = writeln!(text, "{}", acvc_text::fill("need", &[("text", &need)]));
    }
    let owner = if report.agents.is_empty() {
        acvc_text::tr("nobody").to_string()
    } else {
        report.agents.join(", ")
    };
    let _ = writeln!(text, "{}", acvc_text::fill("owner", &[("name", &owner)]));
    let _ = writeln!(
        text,
        "{}",
        acvc_text::fill(
            "opened_summary",
            &[
                ("age", &open_for),
                ("variants", &report.variants.to_string()),
                ("trials", &report.trials.to_string()),
                ("rejections", &report.rejections.to_string()),
                ("tokens", &tokens_label),
            ]
        )
    );
    let _ = writeln!(text, "{}", acvc_text::tr("map_label"));
    write_tree(&mut text, &report.graph, 0);
    (value, text)
}

fn node_json(node: &ContextNode) -> Value {
    json!({
        "variant": hex(&node.variant),
        "claim": node.claim,
        "agent": node.agent,
        "edge": node.edge.as_str(),
        "assumptions": node.assumptions,
        "intent": node.intent,
        "created_at": node.created_at,
        "status": node.status,
        "metrics": node.metrics,
        "rejection": node.rejection,
        "beaten_by": node.beaten_by.iter().map(beaten_json).collect::<Vec<_>>(),
        "children": node.children.iter().map(node_json).collect::<Vec<_>>(),
    })
}

fn problem_of(graph: &[ContextNode]) -> String {
    match graph {
        [] => acvc_text::tr("no_hypothesis_yet").to_string(),
        [one] => one.claim.clone(),
        many => {
            let claims: Vec<&str> = many.iter().map(|node| node.claim.as_str()).collect();
            acvc_text::fill(
                "many_origins",
                &[
                    ("count", &many.len().to_string()),
                    ("claims", &claims.join("; ")),
                ],
            )
        }
    }
}

fn need_of(graph: &[ContextNode]) -> String {
    let mut parts = Vec::new();
    collect_causes(graph, &mut parts);
    if parts.len() == 1 {
        parts.pop().unwrap_or_default()
    } else {
        String::new()
    }
}

fn collect_causes(nodes: &[ContextNode], parts: &mut Vec<String>) {
    for node in nodes {
        for assumption in &node.assumptions {
            if !parts.contains(assumption) {
                parts.push(assumption.clone());
            }
        }
        collect_causes(&node.children, parts);
    }
}

fn open_for(opened: i64, now: i64) -> String {
    let minutes = (now - opened).max(0) / 60_000;
    if minutes < 1 {
        acvc_text::tr("ago_under_minute").to_string()
    } else if minutes < 60 {
        acvc_text::fill("ago_minutes", &[("count", &minutes.to_string())])
    } else if minutes < 48 * 60 {
        acvc_text::fill("ago_hours", &[("count", &(minutes / 60).to_string())])
    } else {
        acvc_text::fill("ago_days", &[("count", &(minutes / (24 * 60)).to_string())])
    }
}

fn tokens_label(tokens: u64) -> String {
    if tokens == 0 {
        acvc_text::tr("no_tokens").to_string()
    } else {
        acvc_text::fill("tokens_recorded", &[("tokens", &tokens.to_string())])
    }
}

fn write_tree(out: &mut String, nodes: &[ContextNode], depth: usize) {
    let pad = "  ".repeat(depth);
    for node in nodes {
        let metrics = if node.metrics.is_empty() {
            acvc_text::tr("no_measurement").to_string()
        } else {
            node.metrics
                .iter()
                .map(|(name, value)| format!("{name} {}", count(*value)))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let _ = writeln!(
            out,
            "{pad}- {} · {}",
            acvc_text::status_label(node.status),
            node.claim
        );
        let _ = writeln!(
            out,
            "{pad}  {} · {} · {metrics}",
            node.agent,
            node.edge.as_str()
        );
        if !node.assumptions.is_empty() {
            let _ = writeln!(
                out,
                "{pad}  {}",
                acvc_text::fill("cause", &[("items", &node.assumptions.join(" · "))])
            );
        }
        if let Some(reason) = &node.rejection {
            let _ = writeln!(
                out,
                "{pad}  {}",
                acvc_text::fill("rejection_label", &[("reason", reason.as_str())])
            );
        }
        for winner in &node.beaten_by {
            let _ = writeln!(out, "{pad}  {}", beaten_line(winner));
        }
        write_tree(out, &node.children, depth + 1);
    }
}
