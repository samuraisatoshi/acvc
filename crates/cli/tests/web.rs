use std::collections::BTreeMap;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use acvc_app::SeedRequest;
use acvc_cli::{init_at, open_at};
use acvc_domain::{Direction, MetricSpec};
use serde_json::Value;

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

fn http(addr: &str, path: &str) -> (u16, String) {
    let mut stream = TcpStream::connect(addr).unwrap();
    stream
        .set_read_timeout(Some(Duration::from_secs(5)))
        .unwrap();
    write!(
        stream,
        "GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n"
    )
    .unwrap();
    let mut raw = Vec::new();
    stream.read_to_end(&mut raw).unwrap();
    let text = String::from_utf8(raw).unwrap();
    let (head, body) = text.split_once("\r\n\r\n").unwrap();
    let status = head
        .split_whitespace()
        .nth(1)
        .unwrap()
        .parse::<u16>()
        .unwrap();
    (status, body.to_string())
}

#[test]
fn page_shows_frontier_premise_and_metrics_without_opening_the_store() {
    let root = scratch("web");
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
    let slow_dir = root.join("slow");
    let fast_dir = root.join("fast");
    fs::create_dir_all(&slow_dir).unwrap();
    fs::create_dir_all(&fast_dir).unwrap();
    fs::write(slow_dir.join("a.txt"), "slow").unwrap();
    fs::write(fast_dir.join("b.txt"), "fast").unwrap();
    let slow = app
        .seed(SeedRequest {
            campaign: "engine".into(),
            path: slow_dir,
            claim: "premissa lenta".into(),
            assumptions: Vec::new(),
            intent: String::new(),
            agent: "agent".into(),
        })
        .unwrap()
        .variant;
    let fast = app
        .seed(SeedRequest {
            campaign: "engine".into(),
            path: fast_dir,
            claim: "premissa rapida".into(),
            assumptions: Vec::new(),
            intent: String::new(),
            agent: "agent".into(),
        })
        .unwrap()
        .variant;
    let app = open_at(&repo).unwrap();
    app.score(&slow, &BTreeMap::from([("tests_passed".into(), 1.0)]))
        .unwrap();
    app.score(&fast, &BTreeMap::from([("tests_passed".into(), 5.0)]))
        .unwrap();

    let mut child = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(&repo)
        .arg("web")
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
        let _ = std::io::BufRead::read_line(&mut std::io::BufReader::new(stdout), &mut line);
        let _ = tx.send(line);
    });
    let line = rx.recv_timeout(Duration::from_secs(15)).unwrap();
    let addr = line
        .split('"')
        .find(|part| part.starts_with("127.0.0.1:"))
        .unwrap()
        .to_string();
    let server = Server(child);

    let (status, page) = http(&addr, "/");
    assert_eq!(status, 200);
    assert!(page.contains("Premissa"));
    assert!(page.contains("Métricas"));
    assert!(page.contains("abriu a campanha"));
    assert!(page.contains("melhor ou igual"));
    assert!(!page.contains("sqlite"));
    assert!(!page.contains("objects.log"));

    let (status, listed) = http(&addr, "/api/campaigns");
    assert_eq!(status, 200, "{listed}");
    assert!(listed.contains("\"direction\":\"higher\""), "{listed}");

    let (status, body) = http(&addr, "/api/frontier?campaign=engine");
    assert_eq!(status, 200, "{body}");
    let parsed: Value = serde_json::from_str(&body).unwrap_or_else(|err| panic!("{err}\n{body}"));
    let rows = parsed["variants"].as_array().unwrap();
    assert!(
        rows.iter().any(|row| {
            row["claim"] == "premissa rapida" && row["metrics"]["tests_passed"] == 5.0
        }),
        "{body}"
    );
    assert!(rows.iter().all(|row| row["claim"] != "premissa lenta"));
    drop(server);
}
