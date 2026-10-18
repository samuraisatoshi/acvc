use serde_json::{Value, json};

pub fn document() -> (Value, String) {
    let value = json!({
        "tool": "acvc",
        "read_first": "context",
        "assistant": acvc_text::tr("assistant_pack"),
        "example": acvc_text::tr("guide_example"),
        "commands": [
            {"name": "context", "when": acvc_text::tr("when_context")},
            {"name": "guide", "when": acvc_text::tr("when_guide")},
            {"name": "assistant", "when": acvc_text::tr("when_assistant")},
            {"name": "frontier", "when": acvc_text::tr("when_frontier")},
            {"name": "why", "when": acvc_text::tr("when_why")},
            {"name": "recall", "when": acvc_text::tr("when_recall")},
            {"name": "seed", "when": acvc_text::tr("when_seed")},
            {"name": "record", "when": acvc_text::tr("when_record")},
            {"name": "score", "when": acvc_text::tr("when_score")},
            {"name": "reject", "when": acvc_text::tr("when_reject")},
            {"name": "spend", "when": acvc_text::tr("when_spend")},
            {"name": "compare", "when": acvc_text::tr("when_compare")},
            {"name": "web", "when": acvc_text::tr("when_web")},
            {"name": "obsidian", "when": acvc_text::tr("when_obsidian")}
        ],
        "status": {
            "na fronteira": acvc_text::tr("status_frontier_why"),
            "superada": acvc_text::tr("status_beaten_why"),
            "rejeitada": acvc_text::tr("status_rejected_why"),
            "sem medição": acvc_text::tr("status_unmeasured_why")
        }
    });
    (
        value,
        format!(
            "{}\n{}",
            acvc_text::tr("guide_text"),
            acvc_text::tr("assistant_pack")
        ),
    )
}
