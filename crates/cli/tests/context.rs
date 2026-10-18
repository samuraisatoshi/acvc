use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;
use std::process::Command;

use acvc_app::{RecordRequest, SeedRequest};
use acvc_cli::{init_at, open_at};
use acvc_domain::{Direction, EdgeKind, MetricSpec};

fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/acvc-tests")
        .join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn context_reads_origin_branch_and_tokens() {
    let root = scratch("context");
    let repo = root.join("repo");
    let app = init_at(&repo).unwrap();
    app.create_campaign(
        "engine",
        &[MetricSpec {
            name: "tests_passed".into(),
            direction: Direction::Higher,
        }],
    )
    .unwrap();
    let workspace = root.join("work");
    fs::create_dir_all(&workspace).unwrap();
    fs::write(workspace.join("a.txt"), "seed").unwrap();
    let seeded = app
        .seed(SeedRequest {
            campaign: "engine".into(),
            path: workspace.clone(),
            claim: "o teste não ficava na campanha".into(),
            assumptions: vec!["a sessão terminava sem gravar a hipótese".into()],
            intent: "guardar o que já foi medido".into(),
            agent: "researcher".into(),
        })
        .unwrap();
    fs::write(workspace.join("a.txt"), "next").unwrap();
    let app = open_at(&repo).unwrap();
    let child = app
        .record(RecordRequest {
            workspace,
            edge: EdgeKind::Mutate,
            claim: "cada sessão grava a hipótese antes de seguir".into(),
            assumptions: vec!["nada obrigava o registro".into()],
            revises: None,
            intent: "não perder o caminho".into(),
            agent: "researcher".into(),
            parents: Vec::new(),
        })
        .unwrap();
    app.score(
        &seeded.variant,
        &BTreeMap::from([("tests_passed".into(), 1.0)]),
    )
    .unwrap();
    app.score(
        &child.variant,
        &BTreeMap::from([("tests_passed".into(), 2.0)]),
    )
    .unwrap();
    assert_eq!(app.spend("engine", "researcher", 1200, "revisão").unwrap(), 1200);

    let report = app.context("engine").unwrap();
    assert_eq!(report.agents, vec!["researcher".to_string()]);
    assert_eq!(report.tokens, 1200);
    assert_eq!(report.variants, 2);
    assert_eq!(report.graph.len(), 1);
    assert_eq!(report.graph[0].claim, "o teste não ficava na campanha");
    assert_eq!(report.graph[0].status, "superseded");
    assert_eq!(
        report.graph[0].beaten_by[0].claim,
        "cada sessão grava a hipótese antes de seguir"
    );
    assert_eq!(report.graph[0].beaten_by[0].gaps[0].name, "tests_passed");
    assert_eq!(report.graph[0].children.len(), 1);
    assert_eq!(
        report.graph[0].children[0].claim,
        "cada sessão grava a hipótese antes de seguir"
    );
    assert_eq!(report.graph[0].children[0].status, "on_frontier");
    assert!(report.graph[0].assumptions[0].contains("sessão"));
    let parent = app.why(&seeded.variant).unwrap();
    assert!(!parent.on_frontier);
    assert!(parent.beaten_by[0].claim.contains("cada sessão"));
    let child_why = app.why(&child.variant).unwrap();
    assert!(child_why.on_frontier);
    assert_eq!(
        child_why.parent_claims,
        vec!["o teste não ficava na campanha".to_string()]
    );
    let other = root.join("other");
    fs::create_dir_all(&other).unwrap();
    fs::write(other.join("b.txt"), "outra origem").unwrap();
    let err = app
        .seed(SeedRequest {
            campaign: "engine".into(),
            path: other,
            claim: "outra origem".into(),
            assumptions: Vec::new(),
            intent: String::new(),
            agent: "researcher".into(),
        })
        .unwrap_err();
    assert_eq!(err.message, "campaign already has an origin");
    app.score(
        &child.variant,
        &BTreeMap::from([("tests_passed".into(), 1.0)]),
    )
    .unwrap();
    let again = app.context("engine").unwrap();
    assert_eq!(again.graph[0].children[0].status, "on_frontier");
    assert_eq!(
        again.graph[0].children[0].metrics.get("tests_passed"),
        Some(&2.0)
    );
    let hits = app.recall("engine", "sessão").unwrap();
    let hit = hits
        .iter()
        .find(|hit| hit.variant == child.variant)
        .unwrap();
    assert_eq!(hit.status, "on_frontier");
    assert_eq!(hit.parents[0].claim, "o teste não ficava na campanha");
    let origin = app.recall("engine", "teste").unwrap();
    let origin = origin
        .iter()
        .find(|hit| hit.variant == seeded.variant)
        .unwrap();
    assert_eq!(origin.status, "superseded");
    assert!(origin.parents.is_empty());
    assert!(app.recall("engine", "xyzzy").unwrap().is_empty());
}

#[test]
fn guide_names_context_before_the_other_commands() {
    let isolated = scratch("guide");
    let output = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .env("LC_ALL", "pt_BR.UTF-8")
        .arg("--repo")
        .arg(&isolated)
        .arg("guide")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let text = String::from_utf8(output.stdout).unwrap();
    assert!(text.contains("\"read_first\":\"context\""), "{text}");
    assert!(text.contains("O mapa da campanha aberta"), "{text}");
    assert!(!text.contains("64 arquivos"), "{text}");
    assert!(!text.contains("o problema em uma frase"), "{text}");
}
