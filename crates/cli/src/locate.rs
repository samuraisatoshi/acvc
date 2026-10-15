use std::fs;
use std::path::Path;

use acvc_app::App;
use acvc_domain::{Error, Marker, Result};

use crate::{find_at, open_at};

pub fn from_repo(repo: Option<&Path>) -> Result<App> {
    match repo {
        Some(path) => open_at(path),
        None => find_at(Path::new(".")),
    }
}

pub fn from_workspace(repo: Option<&Path>, workspace: &Path) -> Result<App> {
    let marker_path = workspace.join(".acvc-workspace");
    let bytes = fs::read(&marker_path).map_err(|err| {
        if err.kind() == std::io::ErrorKind::NotFound {
            return Error::new("this directory is not a working copy")
                .ctx("op", "read marker")
                .ctx("path", marker_path.display().to_string())
                .ctx(
                    "fix",
                    "seed this directory; materialize is only a second copy",
                );
        }
        Error::new(err.to_string())
            .ctx("op", "read marker")
            .ctx("path", marker_path.display().to_string())
    })?;
    let marker = Marker::decode(&bytes)?;
    if let Some(flag) = repo {
        let flagged = fs::canonicalize(flag)
            .map_err(|err| Error::new(err.to_string()).ctx("path", flag.display().to_string()))?;
        let marked = fs::canonicalize(&marker.repo)
            .map_err(|err| Error::new(err.to_string()).ctx("path", marker.repo.clone()))?;
        if flagged != marked {
            return Err(Error::new("repo does not match workspace")
                .ctx("repo", flag.display().to_string())
                .ctx("marker", marker.repo));
        }
    }
    open_at(Path::new(&marker.repo))
}
