use super::{seed, why};
use acvc_app::{BeatenBy, HeavyPath, MetricGap, SeedReport, WhyReport};
use acvc_domain::{Direction, EdgeKind, Hash};

#[test]
fn seed_text_names_the_heaviest_path() {
    acvc_text::install(acvc_text::Lang::PtBr);
    let report = SeedReport {
        variant: Hash::parse(&"ab".repeat(32)).unwrap(),
        tree: Hash::parse(&"cd".repeat(32)).unwrap(),
        files: 1,
        hashed: 1,
        heaviest: vec![HeavyPath {
            path: "note.txt".into(),
            bytes: 2,
        }],
    };
    let text = seed(&report).1;
    assert!(text.contains("files 1 hashed 1"), "{text}");
    assert!(text.contains("pesou note.txt 2"), "{text}");
}

#[test]
fn beaten_text_says_the_motive_was_not_recorded() {
    acvc_text::install(acvc_text::Lang::PtBr);
    let id = Hash::parse(&"ab".repeat(32)).unwrap();
    let report = WhyReport {
        variant: id,
        campaign: "evolucao".into(),
        edge: EdgeKind::Seed,
        claim: "frase humana".into(),
        assumptions: Vec::new(),
        intent: String::new(),
        agent: "researcher".into(),
        parents: Vec::new(),
        children: Vec::new(),
        child_claims: Vec::new(),
        trials: Vec::new(),
        rejection: None,
        on_frontier: false,
        beaten_by: vec![BeatenBy {
            variant: id,
            claim: "sessão gravada".into(),
            gaps: vec![MetricGap {
                name: "uso_real".into(),
                direction: Direction::Higher,
                winner: 5.0,
                loser: 3.0,
            }],
        }],
        parent_claims: Vec::new(),
    };
    let text = why(&report).1;
    assert!(text.contains("Superada por sessão gravada."), "{text}");
    assert!(text.contains("uso_real 3 contra 5"), "{text}");
    assert!(text.contains("Motivo não gravado."), "{text}");
    assert!(!text.contains("Rejeitada"), "{text}");
    let mut recorded = report;
    recorded.rejection = Some("a frase não servia".into());
    let rejected = why(&recorded).1;
    assert!(rejected.contains("Rejeitada: a frase não servia"), "{rejected}");
    assert!(!rejected.contains("Motivo não gravado."), "{rejected}");
}

#[test]
fn english_text_names_the_missing_motive() {
    acvc_text::install(acvc_text::Lang::EnUs);
    let report = WhyReport {
        variant: Hash::parse(&"ab".repeat(32)).unwrap(),
        campaign: "evolucao".into(),
        edge: EdgeKind::Seed,
        claim: "human sentence".into(),
        assumptions: Vec::new(),
        intent: String::new(),
        agent: "researcher".into(),
        parents: Vec::new(),
        children: Vec::new(),
        child_claims: Vec::new(),
        trials: Vec::new(),
        rejection: None,
        on_frontier: false,
        beaten_by: Vec::new(),
        parent_claims: Vec::new(),
    };
    let text = why(&report).1;
    assert!(text.contains("Motive was not recorded."), "{text}");
    assert!(!text.contains("Motivo não gravado."), "{text}");
}
