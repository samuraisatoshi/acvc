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
fn pack_keeps_one_blob_and_leaves_status_and_record_on_the_log() {
    let root = scratch("pack");
    let repo = root.join("repo");
    let source = root.join("source");
    fs::create_dir_all(source.join("a")).unwrap();
    fs::create_dir_all(source.join("b")).unwrap();
    let shared = "a".repeat(4096);
    let other = "b".repeat(2048);
    fs::write(source.join("a/one.txt"), &shared).unwrap();
    fs::write(source.join("b/two.txt"), &shared).unwrap();
    fs::write(source.join("b/three.txt"), &other).unwrap();

    let app = init_at(&repo).unwrap();
    app.create_campaign(
        "engine",
        &[MetricSpec {
            name: "tests_passed".into(),
            direction: Direction::Higher,
        }],
    )
    .unwrap();
    let seeded = app
        .seed(SeedRequest {
            campaign: "engine".into(),
            path: source.clone(),
            claim: "base".into(),
            assumptions: Vec::new(),
            intent: String::new(),
            agent: "agent".into(),
        })
        .unwrap();

    let packed = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(&repo)
        .arg("pack")
        .output()
        .unwrap();
    assert!(
        packed.status.success(),
        "{}",
        String::from_utf8_lossy(&packed.stderr)
    );
    let stdout = String::from_utf8(packed.stdout).unwrap();
    assert!(stdout.contains("\"objects\":2"), "{stdout}");
    assert!(stdout.contains("\"raw_bytes\":6144"), "{stdout}");

    let log = fs::read(repo.join(".acvc/objects.log")).unwrap();
    assert!(
        !log.windows(shared.len())
            .any(|chunk| chunk == shared.as_bytes())
    );

    let app = open_at(&repo).unwrap();
    let copy = root.join("copy");
    app.materialize(&seeded.variant, &copy).unwrap();
    assert_eq!(fs::read(copy.join("a/one.txt")).unwrap(), shared.as_bytes());
    assert_eq!(fs::read(copy.join("b/two.txt")).unwrap(), shared.as_bytes());

    fs::write(repo.join(".acvc/objects.pack"), b"NOTAPACK").unwrap();
    let app = open_at(&repo).unwrap();
    let status = app.status(&source).unwrap();
    assert_eq!(status.clean, 3);
    assert_eq!(status.hashed, 0);

    fs::write(source.join("a/one.txt"), "changed").unwrap();
    app.record(RecordRequest {
        workspace: source.clone(),
        edge: EdgeKind::Mutate,
        claim: "edit".into(),
        assumptions: Vec::new(),
        revises: None,
        intent: String::new(),
        agent: "agent".into(),
        parents: Vec::new(),
    })
    .unwrap();
    let _ = fs::remove_dir_all(repo.join(".acvc/cache"));
    let again = root.join("again");
    assert!(app.materialize(&seeded.variant, &again).is_err());
}
