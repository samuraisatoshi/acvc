use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/acvc-tests")
        .join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

fn run(repo: &Path, args: &[&str]) -> std::process::Output {
    Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(repo)
        .args(args)
        .output()
        .unwrap()
}

#[test]
fn one_campaign_is_the_default_and_a_repeat_says_so() {
    let root = scratch("defaults");
    let repo = root.join("repo");
    let source = root.join("source");
    fs::create_dir_all(&source).unwrap();
    fs::write(source.join("note.txt"), "hello").unwrap();

    let init = run(&repo, &["init"]);
    assert!(init.status.success(), "{}", stderr(&init));
    let created = run(
        &repo,
        &[
            "campaign",
            "create",
            "engine",
            "--metric",
            "tests_passed:higher",
        ],
    );
    assert!(created.status.success(), "{}", stderr(&created));
    let again = run(
        &repo,
        &[
            "campaign",
            "create",
            "engine",
            "--metric",
            "tests_passed:higher",
        ],
    );
    assert!(!again.status.success());
    assert!(
        stderr(&again).contains("campaign already exists"),
        "{}",
        stderr(&again)
    );

    let seeded = run(
        &repo,
        &[
            "seed",
            "--path",
            source.to_str().unwrap(),
            "--premise",
            "primeira",
        ],
    );
    assert!(seeded.status.success(), "{}", stderr(&seeded));
    let frontier = run(&repo, &["frontier"]);
    assert!(frontier.status.success(), "{}", stderr(&frontier));
    assert!(String::from_utf8_lossy(&frontier.stdout).contains("\"campaign\":\"engine\""));
}

fn stderr(output: &std::process::Output) -> String {
    String::from_utf8_lossy(&output.stderr).into_owned()
}
