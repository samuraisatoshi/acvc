use std::fs;
use std::path::{Path, PathBuf};

use acvc_domain::{Error, Result};
use serde_json::json;

use crate::args::Format;
use crate::locate::from_repo;
use crate::present::emit;

const MANIFEST: &str = include_str!("../../../editors/obsidian/manifest.json");
const MAIN: &str = include_str!("../../../editors/obsidian/main.js");
const STYLES: &str = include_str!("../../../editors/obsidian/styles.css");

pub fn install(format: Format, repo: Option<&Path>, vault: Option<PathBuf>) -> Result<()> {
    let app = from_repo(repo)?;
    let repo = app.repo().to_path_buf();
    let vault = vault.unwrap_or_else(|| repo.join("obsidian-vault"));
    if !vault.is_dir() {
        return Err(Error::new("vault folder is missing")
            .ctx("vault", vault.display().to_string())
            .ctx("fix", "obsidian --vault <pasta>"));
    }
    let binary = std::env::current_exe()
        .map_err(|err| Error::new(err.to_string()).ctx("op", "current exe"))?;
    let plugin = vault.join(".obsidian/plugins/acvc");
    fs::create_dir_all(&plugin).map_err(|err| io(err, &plugin))?;
    write(&plugin.join("manifest.json"), MANIFEST.as_bytes())?;
    write(&plugin.join("main.js"), MAIN.as_bytes())?;
    write(&plugin.join("styles.css"), STYLES.as_bytes())?;
    let data = json!({
        "binary": binary,
        "repo": repo,
    });
    let settings = serde_json::to_vec_pretty(&data)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "obsidian settings"))?;
    write(&plugin.join("data.json"), &settings)?;
    enable(&vault.join(".obsidian/community-plugins.json"))?;
    emit(
        format,
        json!({
            "vault": vault,
            "plugin": plugin,
            "binary": binary,
            "repo": repo,
        }),
        format!(
            "Visor instalado em {}. No Obsidian, o comando é Open ACVC.",
            vault.display()
        ),
    );
    Ok(())
}

fn enable(path: &Path) -> Result<()> {
    let mut ids = if path.is_file() {
        let text = fs::read_to_string(path).map_err(|err| io(err, path))?;
        serde_json::from_str::<Vec<String>>(&text)
            .map_err(|err| Error::new(err.to_string()).ctx("path", path.display().to_string()))?
    } else {
        Vec::new()
    };
    if !ids.iter().any(|id| id == "acvc") {
        ids.push("acvc".to_string());
    }
    let body = serde_json::to_vec_pretty(&ids)
        .map_err(|err| Error::new(err.to_string()).ctx("op", "obsidian plugins"))?;
    write(path, &body)
}

fn write(path: &Path, bytes: &[u8]) -> Result<()> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|err| io(err, parent))?;
    }
    fs::write(path, bytes).map_err(|err| io(err, path))
}

fn io(err: std::io::Error, path: &Path) -> Error {
    Error::new(err.to_string())
        .ctx("op", "obsidian")
        .ctx("path", path.display().to_string())
}
