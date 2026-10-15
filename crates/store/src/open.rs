use std::fs;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use acvc_domain::{Error, Result};

use crate::clock::SystemClock;
use crate::db::Db;
use crate::git::GitProcess;
use crate::graph::Graph;
use crate::objects::Objects;
use crate::scan::WorkspaceFs;

pub struct Opened {
    pub repo: PathBuf,
    pub objects: Arc<Objects>,
    pub graph: Arc<Graph>,
    pub files: Arc<WorkspaceFs>,
    pub clock: Arc<SystemClock>,
    pub git: Arc<GitProcess>,
}

pub fn init_repo(repo: &Path) -> Result<Opened> {
    let acvc = repo.join(".acvc");
    if acvc.exists() {
        return Err(Error::new("repo already initialized").ctx("path", repo.display().to_string()));
    }
    fs::create_dir_all(&acvc).map_err(|err| crate::error::io_err(err, "create", &acvc))?;
    open_acvc(repo)
}

pub fn open_repo(repo: &Path) -> Result<Opened> {
    let database = repo.join(".acvc").join("graph.sqlite");
    if !database.is_file() {
        return Err(Error::new("acvc repo not found").ctx("path", repo.display().to_string()));
    }
    open_acvc(repo)
}

pub fn find_repo(start: &Path) -> Result<PathBuf> {
    let mut current =
        fs::canonicalize(start).map_err(|err| crate::error::io_err(err, "canonicalize", start))?;
    loop {
        if current.join(".acvc").join("graph.sqlite").is_file() {
            return Ok(current);
        }
        if !current.pop() {
            break;
        }
    }
    Err(Error::new("acvc repo not found").ctx("start", start.display().to_string()))
}

fn open_acvc(repo: &Path) -> Result<Opened> {
    let acvc = repo.join(".acvc");
    let db = Arc::new(Db::open(&acvc.join("graph.sqlite"))?);
    let objects = Arc::new(Objects::open(Arc::clone(&db), acvc)?);
    Ok(Opened {
        repo: repo.to_path_buf(),
        objects,
        graph: Arc::new(Graph::new(db)),
        files: Arc::new(WorkspaceFs),
        clock: Arc::new(SystemClock::default()),
        git: Arc::new(GitProcess),
    })
}
