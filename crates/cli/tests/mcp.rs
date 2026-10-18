use std::io::{BufRead, BufReader, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

use serde_json::{Value, json};

const TOOLS: &[&str] = &[
    "init",
    "campaign_create",
    "campaign_list",
    "seed",
    "materialize",
    "status",
    "record",
    "score",
    "reject",
    "compare",
    "file_diff",
    "frontier",
    "recall",
    "why",
    "backtrack",
    "pack",
    "serve",
    "push",
    "fetch",
    "promote",
    "tui",
    "web",
    "obsidian",
    "context",
    "guide",
    "assistant",
    "spend",
];

struct Client {
    stdin: std::process::ChildStdin,
    stdout: BufReader<std::process::ChildStdout>,
    child: std::process::Child,
}

impl Drop for Client {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
    }
}

impl Client {
    fn spawn() -> Self {
        let mut child = Command::new(env!("CARGO_BIN_EXE_acvc"))
            .arg("mcp")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::inherit())
            .spawn()
            .unwrap();
        let stdin = child.stdin.take().unwrap();
        let stdout = BufReader::new(child.stdout.take().unwrap());
        Self {
            stdin,
            stdout,
            child,
        }
    }

    fn request(&mut self, id: i64, method: &str, params: Value) -> Value {
        let body = serde_json::to_vec(&json!({
            "jsonrpc": "2.0",
            "id": id,
            "method": method,
            "params": params
        }))
        .unwrap();
        self.stdin.write_all(&body).unwrap();
        self.stdin.write_all(b"\n").unwrap();
        self.stdin.flush().unwrap();
        let mut line = String::new();
        assert!(
            self.stdout.read_line(&mut line).unwrap() > 0,
            "mcp closed stdout"
        );
        serde_json::from_str(line.trim()).unwrap()
    }
}

#[test]
fn mcp_lists_every_verb_and_keeps_the_cli_error() {
    let mut client = Client::spawn();
    let hello = client.request(
        1,
        "initialize",
        json!({"protocolVersion": "2024-11-05", "capabilities": {}}),
    );
    assert_eq!(hello["result"]["serverInfo"]["name"], "acvc");

    let listed = client.request(2, "tools/list", json!({}));
    let names: Vec<String> = listed["result"]["tools"]
        .as_array()
        .unwrap()
        .iter()
        .map(|tool| tool["name"].as_str().unwrap().to_string())
        .collect();
    let expected: Vec<String> = TOOLS.iter().map(|name| (*name).to_string()).collect();
    assert_eq!(names, expected);

    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/acvc-tests/mcp");
    let _ = std::fs::remove_dir_all(&root);
    std::fs::create_dir_all(&root).unwrap();
    let repo = root.join("repo");
    let created = client.request(
        3,
        "tools/call",
        json!({"name": "init", "arguments": {"repo": repo}}),
    );
    assert_eq!(created["result"]["isError"], false);
    let text = created["result"]["content"][0]["text"].as_str().unwrap();
    assert!(text.contains("repo"), "{text}");

    let failed = client.request(
        4,
        "tools/call",
        json!({"name": "campaign_create", "arguments": {"repo": repo, "name": "nope"}}),
    );
    assert_eq!(failed["result"]["isError"], true);
    let tool_error = failed["result"]["content"][0]["text"].as_str().unwrap();
    let direct = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(&repo)
        .args(["campaign", "create", "nope"])
        .output()
        .unwrap();
    assert!(!direct.status.success());
    let cli_error = String::from_utf8(direct.stderr).unwrap();
    assert_eq!(tool_error.trim(), cli_error.trim());
    let parsed: Value = serde_json::from_str(tool_error.trim()).unwrap();
    assert!(parsed.get("error").is_some(), "{tool_error}");
    assert!(parsed.get("context").is_some(), "{tool_error}");
}
