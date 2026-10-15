use std::path::Path;
use std::process::{Command, Stdio};

use acvc_domain::{Error, GitPort, Result};

#[derive(Debug, Default)]
pub struct GitProcess;

impl GitPort for GitProcess {
    fn is_work_tree(&self, path: &Path) -> Result<bool> {
        let output = Command::new("git")
            .arg("-C")
            .arg(path)
            .args(["rev-parse", "--is-inside-work-tree"])
            .output()
            .map_err(|err| {
                Error::new(err.to_string())
                    .ctx("op", "git rev-parse")
                    .ctx("path", path.display().to_string())
            })?;
        Ok(output.status.success() && String::from_utf8_lossy(&output.stdout).trim() == "true")
    }

    fn commit_paths(
        &self,
        repo: &Path,
        files: &[String],
        message: &str,
        author: &str,
    ) -> Result<()> {
        if files.is_empty() {
            return Err(Error::new("no files to commit").ctx("path", repo.display().to_string()));
        }
        let spec = files.iter().fold(String::new(), |mut acc, file| {
            acc.push_str(file);
            acc.push('\n');
            acc
        });
        let add = git_stdin(repo, &["add", "--pathspec-from-file=-"], spec.as_bytes())?;
        if !add.status.success() {
            return Err(Error::new("git add failed")
                .ctx("stderr", String::from_utf8_lossy(&add.stderr).trim())
                .ctx("path", repo.display().to_string()));
        }
        let diff = Command::new("git")
            .arg("-C")
            .arg(repo)
            .args(["diff", "--cached", "--quiet"])
            .status()
            .map_err(|err| Error::new(err.to_string()).ctx("op", "git diff"))?;
        match diff.code() {
            Some(0) => {
                return Err(Error::new("nothing to commit").ctx("path", repo.display().to_string()));
            }
            Some(1) => {}
            _ => return Err(Error::new("git diff failed").ctx("path", repo.display().to_string())),
        }
        let author = author.replace(['\n', '\r'], " ");
        let commit = Command::new("git")
            .arg("-C")
            .arg(repo)
            .arg("-c")
            .arg(format!("user.name={author}"))
            .arg("-c")
            .arg("user.email=acvc@localhost")
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .map_err(|err| Error::new(err.to_string()).ctx("op", "git commit"))?;
        if !commit.status.success() {
            return Err(Error::new("git commit failed")
                .ctx("stderr", String::from_utf8_lossy(&commit.stderr).trim())
                .ctx("path", repo.display().to_string()));
        }
        Ok(())
    }
}

fn git_stdin(repo: &Path, args: &[&str], stdin: &[u8]) -> Result<std::process::Output> {
    let mut child = Command::new("git")
        .arg("-C")
        .arg(repo)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| {
            Error::new(err.to_string())
                .ctx("op", "git")
                .ctx("path", repo.display().to_string())
        })?;
    if let Some(mut pipe) = child.stdin.take() {
        use std::io::Write;
        pipe.write_all(stdin)
            .map_err(|err| Error::new(err.to_string()).ctx("op", "git stdin"))?;
    }
    child
        .wait_with_output()
        .map_err(|err| Error::new(err.to_string()).ctx("op", "git wait"))
}
