use std::collections::BTreeMap;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::Command;

use acvc_app::{RecordRequest, SeedRequest};
use acvc_cli::init_at;
use acvc_domain::{Direction, EdgeKind, MetricSpec, ScoreRelation};

fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/acvc-tests")
        .join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn agent_loop_scores_variants_without_rewriting_siblings() {
    let root = scratch("loop");
    let repo = root.join("repo");
    let source = root.join("source");
    let workspace = root.join("ws");
    let promoted = root.join("promoted");
    let git_dest = root.join("git-dest");
    fs::create_dir_all(source.join("a")).unwrap();
    fs::create_dir_all(source.join("b")).unwrap();
    fs::write(source.join("a/one.txt"), "one-v0").unwrap();
    fs::write(source.join("b/two.txt"), "two").unwrap();

    let app = init_at(&repo).unwrap();
    app.create_campaign(
        "search",
        &[
            MetricSpec {
                name: "tests_passed".into(),
                direction: Direction::Higher,
            },
            MetricSpec {
                name: "latency_ms".into(),
                direction: Direction::Lower,
            },
        ],
    )
    .unwrap();
    let seeded = app
        .seed(SeedRequest {
            campaign: "search".into(),
            path: source.clone(),
            claim: "base cache".into(),
            assumptions: vec!["disk is local".into()],
            intent: "seed".into(),
            agent: "agent".into(),
        })
        .unwrap();
    let seeded_here = app.status(&source).unwrap();
    assert_eq!(seeded_here.clean, 2);
    assert!(seeded_here.dirty.is_empty());
    assert_eq!(seeded_here.hashed, 0);
    let sibling = app
        .read_tree(&seeded.tree)
        .unwrap()
        .into_iter()
        .find(|entry| entry.name == "b")
        .unwrap()
        .hash;
    app.materialize(&seeded.variant, &workspace).unwrap();

    let one = workspace.join("a/one.txt");
    let mut perms = fs::metadata(&one).unwrap().permissions();
    perms.set_mode(0o000);
    fs::set_permissions(&one, perms.clone()).unwrap();
    let clean = app.status(&workspace).unwrap();
    assert_eq!(clean.hashed, 0);
    assert!(clean.dirty.is_empty());
    assert_eq!(clean.clean, 2);
    perms.set_mode(0o644);
    fs::set_permissions(&one, perms).unwrap();

    fs::write(&one, "one-v1").unwrap();
    let dirty = app.status(&workspace).unwrap();
    assert_eq!(dirty.hashed, 0);
    assert_eq!(dirty.dirty, vec!["a/one.txt".to_string()]);

    let faster = record(&app, &workspace, "faster cache");
    assert_eq!(faster.hashed, 1);
    assert_eq!(faster.reused, 1);
    let new_sibling = app
        .read_tree(&faster.tree)
        .unwrap()
        .into_iter()
        .find(|entry| entry.name == "b")
        .unwrap()
        .hash;
    assert_eq!(new_sibling, sibling);
    app.score(&faster.variant, &scores(10.0, 40.0)).unwrap();

    fs::write(&one, "one-v2").unwrap();
    let slower = record(&app, &workspace, "slower cache");
    assert_eq!(slower.hashed, 1);
    assert_eq!(slower.reused, 1);
    app.score(&slower.variant, &scores(10.0, 50.0)).unwrap();

    let frontier = app.frontier("search").unwrap();
    assert_eq!(frontier.len(), 1);
    assert_eq!(frontier[0].variant, faster.variant);
    assert!(
        app.recall("search", "faster")
            .unwrap()
            .iter()
            .any(|hit| hit.variant == faster.variant)
    );
    assert_eq!(
        app.why(&faster.variant).unwrap().parents,
        vec![seeded.variant]
    );
    assert_eq!(
        app.backtrack(&faster.variant, None).unwrap().parent,
        Some(seeded.variant)
    );
    let compared = app.compare(&faster.variant, &slower.variant).unwrap();
    assert_eq!(compared.dominance, ScoreRelation::LeftDominates);
    assert!(
        compared
            .diff
            .modified
            .iter()
            .any(|path| path == "a/one.txt")
    );

    app.promote_dest(&faster.variant, &promoted).unwrap();
    assert_eq!(
        fs::read_to_string(promoted.join("a/one.txt")).unwrap(),
        "one-v1"
    );
    assert!(
        Command::new("git")
            .arg("init")
            .arg(&git_dest)
            .status()
            .unwrap()
            .success()
    );
    app.promote_git(&faster.variant, &git_dest).unwrap();
    let log = Command::new("git")
        .arg("-C")
        .arg(&git_dest)
        .args(["log", "-1", "--format=%s"])
        .output()
        .unwrap();
    assert!(
        String::from_utf8(log.stdout)
            .unwrap()
            .contains("faster cache")
    );
}

fn record(app: &acvc_app::App, workspace: &Path, claim: &str) -> acvc_app::RecordReport {
    app.record(RecordRequest {
        workspace: workspace.to_path_buf(),
        edge: EdgeKind::Mutate,
        claim: claim.into(),
        assumptions: Vec::new(),
        revises: None,
        intent: claim.into(),
        agent: "agent".into(),
        parents: Vec::new(),
    })
    .unwrap()
}

fn scores(tests_passed: f64, latency_ms: f64) -> BTreeMap<String, f64> {
    BTreeMap::from([
        ("tests_passed".into(), tests_passed),
        ("latency_ms".into(), latency_ms),
    ])
}
