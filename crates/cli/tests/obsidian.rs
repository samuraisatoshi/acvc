use std::fs;
use std::path::PathBuf;
use std::process::Command;

use serde_json::Value;

fn scratch(name: &str) -> PathBuf {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../../target/acvc-tests")
        .join(name);
    let _ = fs::remove_dir_all(&path);
    fs::create_dir_all(&path).unwrap();
    path
}

#[test]
fn obsidian_installs_the_viewer_and_keeps_other_plugins() {
    let root = scratch("obsidian");
    let repo = root.join("repo");
    let vault = root.join("vault");
    fs::create_dir_all(vault.join(".obsidian")).unwrap();
    fs::write(
        vault.join(".obsidian/community-plugins.json"),
        "[\"other\"]",
    )
    .unwrap();
    let init = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(&repo)
        .arg("init")
        .output()
        .unwrap();
    assert!(
        init.status.success(),
        "{}",
        String::from_utf8_lossy(&init.stderr)
    );
    let installed = Command::new(env!("CARGO_BIN_EXE_acvc"))
        .arg("--repo")
        .arg(&repo)
        .args(["obsidian", "--vault"])
        .arg(&vault)
        .output()
        .unwrap();
    assert!(
        installed.status.success(),
        "{}",
        String::from_utf8_lossy(&installed.stderr)
    );
    let plugin = vault.join(".obsidian/plugins/acvc");
    let manifest = fs::read_to_string(plugin.join("manifest.json")).unwrap();
    assert!(manifest.contains("\"id\": \"acvc\""), "{manifest}");
    let main = fs::read_to_string(plugin.join("main.js")).unwrap();
    assert!(main.contains("Qual fica"), "viewer text missing");
    let settings: Value =
        serde_json::from_str(&fs::read_to_string(plugin.join("data.json")).unwrap()).unwrap();
    assert_eq!(settings["repo"], repo.display().to_string());
    assert!(!settings["binary"].as_str().unwrap_or("").is_empty());
    let enabled: Vec<String> = serde_json::from_str(
        &fs::read_to_string(vault.join(".obsidian/community-plugins.json")).unwrap(),
    )
    .unwrap();
    assert_eq!(enabled, ["other".to_string(), "acvc".to_string()]);
}
