use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use acvc_app::SeedRequest;
use acvc_cli::{init_at, open_at};
use acvc_domain::{Direction, EntryKind, FILE_MODE, Hash, MetricSpec, TreeEntry, canonical_tree};

fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/acvc-tests")
        .join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

struct Server(Child);

impl Drop for Server {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

fn campaign(repo: &Path) {
    let app = init_at(repo).unwrap();
    app.create_campaign(
        "engine",
        &[MetricSpec {
            name: "tests_passed".into(),
            direction: Direction::Higher,
        }],
    )
    .unwrap();
}

fn seed(repo: &Path, path: PathBuf, claim: &str) -> Hash {
    open_at(repo)
        .unwrap()
        .seed(SeedRequest {
            campaign: "engine".into(),
            path,
            claim: claim.into(),
            assumptions: Vec::new(),
            intent: String::new(),
            agent: "agent".into(),
        })
        .unwrap()
        .variant
}

fn serve(repo: &Path) -> (Server, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(repo)
        .arg("serve")
        .arg("--bind")
        .arg("127.0.0.1:0")
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap();
    let stdout = child.stdout.take().unwrap();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let mut line = String::new();
        let _ = BufReader::new(stdout).read_line(&mut line);
        let _ = tx.send(line);
    });
    let line = rx.recv_timeout(Duration::from_secs(15)).unwrap();
    let addr = line
        .split('"')
        .find(|part| part.starts_with("127.0.0.1:"))
        .unwrap_or_else(|| panic!("serve did not print an address: {line}"))
        .to_string();
    (Server(child), addr)
}

fn push(repo: &Path, addr: &str, variant: &Hash) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(repo)
        .arg("push")
        .arg("--to")
        .arg(addr)
        .arg("--variant")
        .arg(variant.to_hex())
        .arg("--campaign")
        .arg("engine")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

fn fetch(repo: &Path, addr: &str, variant: &Hash) -> String {
    let output = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(repo)
        .arg("fetch")
        .arg("--from")
        .arg(addr)
        .arg("--variant")
        .arg(variant.to_hex())
        .arg("--campaign")
        .arg("engine")
        .output()
        .unwrap();
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap()
}

#[test]
fn push_and_fetch_skip_objects_already_at_the_destination() {
    let root = scratch("remote");
    let shared = "S".repeat(4096);
    let extra = "only-on-source";
    let local_dir = root.join("local");
    let source_dir = root.join("source");
    fs::create_dir_all(&local_dir).unwrap();
    fs::create_dir_all(&source_dir).unwrap();
    fs::write(local_dir.join("same.txt"), &shared).unwrap();
    fs::write(source_dir.join("same.txt"), &shared).unwrap();
    fs::write(source_dir.join("extra.txt"), extra).unwrap();

    let dest = root.join("dest");
    let source = root.join("origin");
    campaign(&dest);
    campaign(&source);
    let local = seed(&dest, local_dir, "local");
    let variant = seed(&source, source_dir, "remote claim");

    let tree = canonical_tree(&[
        TreeEntry {
            name: "extra.txt".into(),
            mode: FILE_MODE,
            kind: EntryKind::Blob,
            hash: Hash::blob(extra.as_bytes()),
        },
        TreeEntry {
            name: "same.txt".into(),
            mode: FILE_MODE,
            kind: EntryKind::Blob,
            hash: Hash::blob(shared.as_bytes()),
        },
    ])
    .unwrap();
    let expected = (extra.len() + tree.len()) as u64;

    let (server, addr) = serve(&dest);
    let first = push(&source, &addr, &variant);
    assert!(
        first.contains(&format!("\"variant\":\"{}\"", variant.to_hex())),
        "{first}"
    );
    assert!(first.contains("\"objects_sent\":2"), "{first}");
    assert!(first.contains("\"objects_skipped\":1"), "{first}");
    assert!(
        first.contains(&format!("\"payload_bytes\":{expected}")),
        "{first}"
    );
    assert!(first.contains("\"graph\":\"inserted\""), "{first}");

    let opened = open_at(&dest).unwrap();
    let received = opened.why(&variant).unwrap();
    assert_eq!(received.claim, "remote claim");
    assert_eq!(received.variant, variant);
    let kept = opened.why(&local).unwrap();
    assert_eq!(kept.claim, "local");
    assert!(kept.parents.is_empty());
    let copy = root.join("copy");
    opened.materialize(&variant, &copy).unwrap();
    assert_eq!(fs::read(copy.join("extra.txt")).unwrap(), extra.as_bytes());
    assert_eq!(fs::read(copy.join("same.txt")).unwrap(), shared.as_bytes());

    let second = push(&source, &addr, &variant);
    assert!(second.contains("\"objects_sent\":0"), "{second}");
    assert!(second.contains("\"objects_skipped\":3"), "{second}");
    assert!(second.contains("\"payload_bytes\":0"), "{second}");
    assert!(second.contains("\"graph\":\"unchanged\""), "{second}");
    assert_eq!(open_at(&dest).unwrap().why(&local).unwrap().claim, "local");
    drop(server);

    let third = root.join("third");
    let third_dir = root.join("third-files");
    fs::create_dir_all(&third_dir).unwrap();
    fs::write(third_dir.join("same.txt"), &shared).unwrap();
    campaign(&third);
    seed(&third, third_dir, "third local");
    let (origin, origin_addr) = serve(&source);
    let fetched = fetch(&third, &origin_addr, &variant);
    assert!(
        fetched.contains(&format!("\"variant\":\"{}\"", variant.to_hex())),
        "{fetched}"
    );
    assert!(
        fetched.contains(&format!("\"payload_bytes\":{expected}")),
        "{fetched}"
    );
    assert!(fetched.contains("\"objects_skipped\":1"), "{fetched}");
    assert!(fetched.contains("\"graph\":\"inserted\""), "{fetched}");
    assert_eq!(
        open_at(&third).unwrap().why(&variant).unwrap().claim,
        "remote claim"
    );
    drop(origin);
}
