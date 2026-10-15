use acvc_domain::{EdgeKind, ScoreRelation};
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, Borders, Paragraph, Wrap};

use crate::model::{Model, Screen, short};

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
    let campaign = model.selected_campaign().unwrap_or("sem campanha");
    let mut spans = vec![Span::raw(format!(" {campaign}  "))];
    for (screen, name) in SCREENS {
        let style = if *screen == model.screen {
            Style::default().add_modifier(Modifier::REVERSED)
        } else {
            Style::default()
        };
        spans.push(Span::styled(format!(" {name} "), style));
    }
    Line::from(spans)
}

const SCREENS: &[(Screen, &str)] = &[
    (Screen::Campaigns, "Campanhas"),
    (Screen::Frontier, "Fronteira"),
    (Screen::Lineage, "Linhagem"),
    (Screen::Detail, "Premissa"),
    (Screen::Diff, "Comparar"),
    (Screen::Recall, "Busca"),
];

fn help(model: &Model) -> &'static str {
    if model.typing {
        return "Enter busca. Esc cancela. Backspace apaga.";
    }
    match model.screen {
        Screen::Campaigns => "Enter abre a fronteira. ← → troca a tela. q sai.",
        Screen::Frontier => "Enter abre a premissa. d compara duas. ← → troca a tela. q sai.",
        Screen::Lineage => "Enter abre o parente. ← → troca a tela. q sai.",
        Screen::Detail => "A hipótese e as medições. ← → troca a tela. q sai.",
        Screen::Diff => "Enter mostra o arquivo. ← → troca a tela. q sai.",
        Screen::Recall => "/ busca. Enter procura. ← → troca a tela. q sai.",
    }
}

fn body(model: &Model) -> Vec<Line<'static>> {
    let mut lines = match model.screen {
        Screen::Campaigns => choices(&model.campaigns, model.campaign_idx, "Nenhuma campanha."),
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
            Line::from("Nenhuma hipótese pontuada."),
            Line::from("Ela aparece aqui depois de acvc score."),
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
            "Abra uma hipótese na fronteira para ver de onde ela veio.",
        )];
    };
    let mut lines = Vec::new();
    if detail.parents.is_empty() && detail.children.is_empty() {
        lines.push(Line::from("Esta hipótese começou a campanha."));
        lines.push(Line::from("Não há variante anterior nem derivada."));
        return lines;
    }
    for (index, parent) in detail.parents.iter().enumerate() {
        lines.push(marked(
            index == model.lineage_idx,
            format!("anterior  {}", short(parent)),
        ));
    }
    for (index, child) in detail.children.iter().enumerate() {
        let slot = detail.parents.len() + index;
        lines.push(marked(
            slot == model.lineage_idx,
            format!("derivada  {}", short(child)),
        ));
    }
    lines
}

fn detail(model: &Model) -> Vec<Line<'static>> {
    let Some(detail) = &model.detail else {
        return vec![Line::from("Abra uma hipótese na fronteira.")];
    };
    let mut lines = vec![
        Line::from(Span::styled(
            detail.claim.clone(),
            Style::default().add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(origin(detail.edge)),
        Line::from(format!("Agente  {}", agent_name(&detail.agent))),
    ];
    if !detail.intent.is_empty() {
        lines.push(Line::from(format!("Intenção  {}", detail.intent)));
    }
    if !detail.assumptions.is_empty() {
        lines.push(Line::from(format!(
            "Suposições  {}",
            detail.assumptions.join("; ")
        )));
    }
    if let Some(reason) = &detail.rejection {
        lines.push(Line::from(format!("Fora da fronteira  {reason}")));
    }
    lines.push(Line::from(""));
    lines.push(Line::from("Pontuação"));
    if detail.trials.is_empty() {
        lines.push(Line::from("    ainda sem medição"));
    }
    for trial in &detail.trials {
        lines.push(Line::from(format!("    {}", metrics(&trial.metrics))));
    }
    lines.push(Line::from(""));
    lines.push(Line::from(format!(
        "Pais {}    Filhas {}",
        count_label(detail.parents.len()),
        count_label(detail.children.len())
    )));
    lines.push(Line::from(format!("id {}", short(&detail.variant))));
    lines
}

fn diff(model: &Model) -> Vec<Line<'static>> {
    let Some(compare) = &model.compare else {
        return vec![
            Line::from("Nada comparado ainda."),
            Line::from("Na fronteira, aperte d em duas hipóteses."),
        ];
    };
    let mut lines = vec![
        Line::from(dominance(compare.dominance)),
        Line::from(format!(
            "{}  →  {}",
            short(&compare.left),
            short(&compare.right)
        )),
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
    let mut query = format!("Busca  {}", model.recall_query);
    if model.typing {
        query.push('▌');
    }
    let mut lines = vec![Line::from(query)];
    if model.recall_hits.is_empty() {
        lines.push(Line::from(
            "Escreva uma palavra da premissa e aperte Enter.",
        ));
        return lines;
    }
    for (index, hit) in model.recall_hits.iter().enumerate() {
        lines.push(marked(index == model.recall_idx, hit.claim.clone()));
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
        "nenhum".into()
    } else {
        count.to_string()
    }
}

fn agent_name(agent: &str) -> &str {
    if agent.is_empty() {
        "não informado"
    } else {
        agent
    }
}

fn origin(edge: EdgeKind) -> String {
    let text = match edge {
        EdgeKind::Seed => "Começou a campanha. Não há hipótese anterior.",
        EdgeKind::Mutate => "Mudança a partir de outra hipótese.",
        EdgeKind::Revert => "Volta para uma hipótese anterior.",
        EdgeKind::RevisePremise => "A premissa foi reescrita.",
        EdgeKind::Synthesize => "Síntese de várias hipóteses.",
    };
    format!("Origem  {text}")
}

fn dominance(relation: ScoreRelation) -> String {
    match relation {
        ScoreRelation::LeftDominates => {
            "A da esquerda é melhor ou igual em tudo, e melhor em algo."
        }
        ScoreRelation::RightDominates => {
            "A da direita é melhor ou igual em tudo, e melhor em algo."
        }
        ScoreRelation::Tie => "Empate. Nenhuma é melhor.",
        ScoreRelation::Incomparable => "Cada uma ganha numa métrica diferente.",
        ScoreRelation::Incomplete => "Falta medição para decidir.",
    }
    .into()
}
