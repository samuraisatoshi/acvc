use acvc_app::CompareReport;
use acvc_domain::{EdgeKind, ScoreRelation};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::model::{Model, Screen};

pub fn draw(frame: &mut Frame, model: &Model) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(5),
            Constraint::Length(3),
        ])
        .split(frame.area());
    frame.render_widget(
        Paragraph::new(tabs(model)).block(Block::default().borders(Borders::ALL)),
        chunks[0],
    );
    frame.render_widget(
        Paragraph::new(body(model))
            .wrap(Wrap { trim: false })
            .block(Block::default().borders(Borders::ALL)),
        chunks[1],
    );
    frame.render_widget(
        Paragraph::new(help(model)).block(Block::default().borders(Borders::ALL)),
        chunks[2],
    );
}

fn tabs(model: &Model) -> Line<'static> {
    let campaign = model.selected_campaign().unwrap_or(acvc_text::tr("no_campaign"));
    let mut spans = vec![Span::raw(format!(" {campaign}  "))];
    for screen in SCREENS {
        let name = screen_name(*screen);
        let style = if *screen == model.screen {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };
        spans.push(Span::styled(format!(" {name} "), style));
    }
    Line::from(spans)
}

const SCREENS: &[Screen] = &[
    Screen::Campaigns,
    Screen::Frontier,
    Screen::Lineage,
    Screen::Detail,
    Screen::Diff,
    Screen::Recall,
];

fn screen_name(screen: Screen) -> &'static str {
    match screen {
        Screen::Campaigns => acvc_text::tr("campaigns"),
        Screen::Frontier => acvc_text::tr("frontier_screen"),
        Screen::Lineage => acvc_text::tr("lineage_screen"),
        Screen::Detail => acvc_text::tr("premise_screen"),
        Screen::Diff => acvc_text::tr("compare_screen"),
        Screen::Recall => acvc_text::tr("recall_screen"),
    }
}

fn help(model: &Model) -> &'static str {
    if model.typing {
        return acvc_text::tr("help_search");
    }
    match model.screen {
        Screen::Campaigns => acvc_text::tr("help_campaigns"),
        Screen::Frontier => acvc_text::tr("help_frontier"),
        Screen::Lineage => acvc_text::tr("help_lineage"),
        Screen::Detail => acvc_text::tr("help_detail"),
        Screen::Diff => acvc_text::tr("help_diff"),
        Screen::Recall => acvc_text::tr("help_recall"),
    }
}

fn body(model: &Model) -> Vec<Line<'static>> {
    let mut lines = match model.screen {
        Screen::Campaigns => choices(&model.campaigns, model.campaign_idx, acvc_text::tr("no_campaigns")),
        Screen::Frontier => frontier(model),
        Screen::Lineage => lineage(model),
        Screen::Detail => detail(model),
        Screen::Diff => diff(model),
        Screen::Recall => recall(model),
    };
    if !model.message.is_empty() {
        lines.insert(0, Line::from(model.message.clone()));
        lines.insert(1, Line::from(""));
    }
    lines
}

fn choices(items: &[String], selected: usize, empty: &str) -> Vec<Line<'static>> {
    if items.is_empty() {
        return vec![Line::from(empty.to_string())];
    }
    items
        .iter()
        .enumerate()
        .map(|(index, item)| marked(index == selected, item.clone()))
        .collect()
}

fn frontier(model: &Model) -> Vec<Line<'static>> {
    if model.frontier.is_empty() {
        return vec![
            Line::from(acvc_text::tr("no_scored")),
            Line::from(acvc_text::tr("appears_after_score")),
        ];
    }
    let mut lines = Vec::new();
    for (index, row) in model.frontier.iter().enumerate() {
        lines.push(marked(index == model.frontier_idx, row.claim.clone()));
        lines.push(Line::from(format!("    {}", metrics(&row.metrics))));
    }
    lines
}

fn lineage(model: &Model) -> Vec<Line<'static>> {
    let Some(detail) = &model.detail else {
        return vec![Line::from(
            acvc_text::tr("open_lineage"),
        )];
    };
    let mut lines = Vec::new();
    if detail.parents.is_empty() && detail.children.is_empty() {
        lines.push(Line::from(acvc_text::tr("opened_campaign")));
        lines.push(Line::from(acvc_text::tr("no_before_after")));
        return lines;
    }
    for (index, claim) in detail.parent_claims.iter().enumerate() {
        lines.push(marked(
            index == model.lineage_idx,
            acvc_text::fill("came_from_short", &[("claim", claim.as_str())]),
        ));
    }
    for (index, claim) in detail.child_claims.iter().enumerate() {
        let slot = detail.parent_claims.len() + index;
        lines.push(marked(
            slot == model.lineage_idx,
            acvc_text::fill("then_short", &[("claim", claim.as_str())]),
        ));
    }
    lines
}

fn detail(model: &Model) -> Vec<Line<'static>> {
    let Some(detail) = &model.detail else {
        return vec![Line::from(acvc_text::tr("open_hypothesis"))];
    };
    let mut lines = vec![
        Line::from(Span::styled(
            detail.claim.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(origin(detail.edge)),
        Line::from(acvc_text::fill(
            "agent_gap",
            &[("agent", agent_name(&detail.agent))],
        )),
    ];
    if !detail.intent.is_empty() {
        lines.push(Line::from(acvc_text::fill(
            "intent_gap",
            &[("intent", detail.intent.as_str())],
        )));
    }
    if !detail.assumptions.is_empty() {
        lines.push(Line::from(acvc_text::fill(
            "assumptions_gap",
            &[("items", &detail.assumptions.join("; "))],
        )));
    }
    if let Some(reason) = &detail.rejection {
        lines.push(Line::from(acvc_text::fill(
            "rejected_gap",
            &[("reason", reason.as_str())],
        )));
    } else if detail.on_frontier {
        lines.push(Line::from(acvc_text::tr("on_frontier")));
    } else {
        lines.push(Line::from(acvc_text::tr("motive_unrecorded")));
    }
    for beaten in &detail.beaten_by {
        lines.push(Line::from(beaten_sentence(beaten)));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(acvc_text::tr("score_heading")));
    if detail.trials.is_empty() {
        lines.push(Line::from(acvc_text::tr("still_unmeasured")));
    }
    for trial in &detail.trials {
        lines.push(Line::from(format!("    {}", metrics(&trial.metrics))));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(acvc_text::fill(
        "came_then",
        &[
            ("from", &count_label(detail.parents.len())),
            ("after", &count_label(detail.children.len())),
        ],
    )));
    lines
}

fn diff(model: &Model) -> Vec<Line<'static>> {
    let Some(compare) = &model.compare else {
        return vec![
            Line::from(acvc_text::tr("nothing_compared")),
            Line::from(acvc_text::tr("frontier_hint")),
        ];
    };
    let mut lines = vec![
        Line::from(verdict(compare)),
        Line::from(compare.left_claim.clone()),
        Line::from(compare.right_claim.clone()),
    ];
    for delta in &compare.metrics {
        lines.push(Line::from(format!(
            "{}  {} → {}",
            delta.name,
            number_opt(delta.left),
            number_opt(delta.right)
        )));
    }
    for (index, path) in model.diff_paths.iter().enumerate() {
        lines.push(marked(index == model.diff_idx, path.clone()));
    }
    if !model.file_text.is_empty() {
        lines.push(Line::from(""));
        for line in model.file_text.lines() {
            lines.push(Line::from(line.to_string()));
        }
    }
    lines
}

fn recall(model: &Model) -> Vec<Line<'static>> {
    let mut query = acvc_text::fill("search_label", &[("query", model.recall_query.as_str())]);
    if model.typing {
        query.push('▌');
    }
    let mut lines = vec![Line::from(query)];
    if model.recall_hits.is_empty() {
        lines.push(Line::from(
            acvc_text::tr("recall_prompt"),
        ));
        return lines;
    }
    for (index, hit) in model.recall_hits.iter().enumerate() {
        lines.push(marked(index == model.recall_idx, hit.claim.clone()));
        let origin = hit
            .parents
            .first()
            .map(|parent| acvc_text::fill("came_from_one", &[("claim", parent.claim.as_str())]))
            .unwrap_or_else(|| acvc_text::tr("opened_short").to_string());
        lines.push(Line::from(format!(
            "    {}  {origin}",
            acvc_text::status_label(&hit.status)
        )));
    }
    lines
}

fn marked(selected: bool, text: String) -> Line<'static> {
    let style = if selected {
        Style::default().add_modifier(Modifier::REVERSED)
    } else {
        Style::default()
    };
    Line::from(Span::styled(format!(" {text} "), style))
}

fn beaten_sentence(winner: &acvc_app::BeatenBy) -> String {
    let gaps = winner
        .gaps
        .iter()
        .map(|gap| {
            let sense = match gap.direction {
                acvc_domain::Direction::Higher => acvc_text::tr("higher_better"),
                acvc_domain::Direction::Lower => acvc_text::tr("lower_better"),
            };
            acvc_text::fill(
                "against",
                &[
                    ("name", gap.name.as_str()),
                    ("loser", &number(gap.loser)),
                    ("winner", &number(gap.winner)),
                    ("sense", sense),
                ],
            )
        })
        .collect::<Vec<_>>()
        .join("; ");
    if gaps.is_empty() {
        acvc_text::fill("beaten_by_gap", &[("claim", winner.claim.as_str())])
    } else {
        format!(
            "{} {gaps}",
            acvc_text::fill("beaten_by_gap", &[("claim", winner.claim.as_str())])
        )
    }
}

fn metrics(values: &std::collections::BTreeMap<String, f64>) -> String {
    values
        .iter()
        .map(|(name, value)| format!("{name} {}", number(*value)))
        .collect::<Vec<_>>()
        .join("    ")
}

fn number(value: f64) -> String {
    if value.fract() == 0.0 && value.abs() < 1e15 {
        format!("{}", value as i64)
    } else {
        format!("{value}")
    }
}

fn number_opt(value: Option<f64>) -> String {
    value.map(number).unwrap_or_else(|| "-".into())
}

fn count_label(count: usize) -> String {
    if count == 0 {
        acvc_text::tr("none_count").to_string()
    } else {
        count.to_string()
    }
}

fn agent_name(agent: &str) -> &str {
    if agent.is_empty() {
        acvc_text::tr("agent_unknown")
    } else {
        agent
    }
}

fn origin(edge: EdgeKind) -> String {
    let text = match edge {
        EdgeKind::Seed => acvc_text::tr("origin_seed"),
        EdgeKind::Mutate => acvc_text::tr("origin_mutate"),
        EdgeKind::Revert => acvc_text::tr("origin_revert"),
        EdgeKind::RevisePremise => acvc_text::tr("origin_revise"),
        EdgeKind::Synthesize => acvc_text::tr("origin_synthesize"),
    };
    acvc_text::fill("origin_prefix", &[("text", text)])
}

fn verdict(compare: &CompareReport) -> String {
    match compare.dominance {
        ScoreRelation::LeftDominates => {
            acvc_text::fill("stays", &[("claim", compare.left_claim.as_str())])
        }
        ScoreRelation::RightDominates => {
            acvc_text::fill("stays", &[("claim", compare.right_claim.as_str())])
        }
        ScoreRelation::Tie => acvc_text::fill(
            "tie_both",
            &[
                ("left", compare.left_claim.as_str()),
                ("right", compare.right_claim.as_str()),
            ],
        ),
        ScoreRelation::Incomparable => acvc_text::tr("incomparable").to_string(),
        ScoreRelation::Incomplete => acvc_text::tr("missing_measure_which").to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::model::Model;
    use acvc_app::{BeatenBy, CompareReport, MetricGap, WhyReport};
    use acvc_domain::{Direction as MetricDirection, EdgeKind, Hash, ScoreRelation, TreeDiff};
    use ratatui::Terminal;
    use ratatui::backend::TestBackend;

    fn sample() -> Hash {
        Hash::parse("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa").unwrap()
    }

    fn painted(model: &Model) -> String {
        let backend = TestBackend::new(90, 24);
        let mut terminal = Terminal::new(backend).unwrap();
        terminal.draw(|frame| draw(frame, model)).unwrap();
        let buffer = terminal.backend().buffer();
        let mut text = String::new();
        for y in 0..buffer.area.height {
            for x in 0..buffer.area.width {
                text.push_str(buffer[(x, y)].symbol());
            }
            text.push('\n');
        }
        text
    }

    #[test]
    fn lineage_and_compare_name_the_premise() {
        acvc_text::install(acvc_text::Lang::PtBr);
        let mut model = Model::empty();
        model.detail = Some(WhyReport {
            variant: sample(),
            campaign: "evolucao".into(),
            edge: EdgeKind::Seed,
            claim: "frase humana".into(),
            assumptions: Vec::new(),
            intent: String::new(),
            agent: "researcher".into(),
            parents: Vec::new(),
            children: vec![sample()],
            child_claims: vec!["sessão gravada".into()],
            trials: Vec::new(),
            rejection: None,
            on_frontier: false,
            beaten_by: vec![BeatenBy {
                variant: sample(),
                claim: "sessão gravada".into(),
                gaps: vec![MetricGap {
                    name: "uso_real".into(),
                    direction: MetricDirection::Higher,
                    winner: 5.0,
                    loser: 3.0,
                }],
            }],
            parent_claims: Vec::new(),
        });
        model.screen = Screen::Detail;
        let premise = painted(&model);
        assert!(premise.contains("Motivo não gravado."), "{premise}");
        assert!(premise.contains("uso_real 3 contra 5"), "{premise}");
        assert!(!premise.contains("Rejeitada"), "{premise}");
        model.screen = Screen::Lineage;
        let lineage = painted(&model);
        assert!(lineage.contains("depois  sessão gravada"), "{lineage}");
        assert!(!lineage.contains("aaaaaaaa"), "{lineage}");

        model.compare = Some(CompareReport {
            left: sample(),
            right: sample(),
            left_claim: "frase humana".into(),
            right_claim: "sessão gravada".into(),
            diff: TreeDiff {
                added: Vec::new(),
                removed: Vec::new(),
                modified: Vec::new(),
            },
            metrics: Vec::new(),
            dominance: ScoreRelation::RightDominates,
        });
        model.screen = Screen::Diff;
        let compared = painted(&model);
        assert!(compared.contains("sessão gravada fica."), "{compared}");
        assert!(!compared.contains("esquerda"), "{compared}");
    }
}
