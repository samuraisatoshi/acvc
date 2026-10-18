use std::fs::{self, File};
use std::path::Path;
use std::time::UNIX_EPOCH;

use acvc_domain::{
    DirectoryScan, Error, FileStat, Fingerprint, ListedFile, Result, valid_relative,
};
use memmap2::Mmap;

use crate::error::io_err;

#[derive(Debug, Default)]
pub struct WorkspaceFs;

const SKIP_DIRS: &[&str] = &[
    ".git",
    ".acvc",
    "target",
    "node_modules",
    ".build",
    "DerivedData",
];
const SKIP_FILES: &[&str] = &[".acvc-workspace", ".acvc-index", ".acvcignore"];

impl DirectoryScan for WorkspaceFs {
    fn list_files(&self, root: &Path) -> Result<Vec<ListedFile>> {
        let rules = ignore_rules(root)?;
        let mut files = Vec::new();
        walk(root, root, &rules, &mut files)?;
        files.sort_by(|left, right| left.path.cmp(&right.path));
        Ok(files)
    }

    fn read_file(&self, path: &Path) -> Result<Vec<u8>> {
        fs::read(path).map_err(|err| io_err(err, "read", path))
    }

    fn metadata(&self, path: &Path) -> Result<FileStat> {
        let meta = fs::symlink_metadata(path).map_err(|err| io_err(err, "stat", path))?;
        if meta.file_type().is_symlink() {
            return Err(
                Error::new("symlink is not supported").ctx("path", path.display().to_string())
            );
        }
        Ok(stat_of(&meta))
    }
}

fn ignore_rules(root: &Path) -> Result<Vec<String>> {
    let path = root.join(".acvcignore");
    if !path.is_file() {
        return Ok(Vec::new());
    }
    let text = fs::read_to_string(&path).map_err(|err| io_err(err, "read ignore", &path))?;
    let mut rules = Vec::new();
    for (number, line) in text.lines().enumerate() {
        let line = line.trim().trim_end_matches('/');
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        if !valid_relative(line) {
            return Err(Error::new("invalid ignore path")
                .ctx("line", (number + 1).to_string())
                .ctx("path", line));
        }
        rules.push(line.to_string());
    }
    Ok(rules)
}

fn relative_of(root: &Path, path: &Path) -> Result<String> {
    let relative = path
        .strip_prefix(root)
        .map_err(|_| Error::new("path escapes root").ctx("path", path.display().to_string()))?;
    relative
        .to_str()
        .map(|value| value.replace('\\', "/"))
        .ok_or_else(|| Error::new("path is not utf-8").ctx("path", path.display().to_string()))
}

fn ignored(path: &str, rules: &[String]) -> bool {
    rules
        .iter()
        .any(|rule| path == rule || path.starts_with(&format!("{rule}/")))
}

fn walk(root: &Path, dir: &Path, rules: &[String], out: &mut Vec<ListedFile>) -> Result<()> {
    let entries = fs::read_dir(dir).map_err(|err| io_err(err, "read dir", dir))?;
    for entry in entries {
        let entry = entry.map_err(|err| io_err(err, "read dir", dir))?;
        let name = entry.file_name().into_string().map_err(|_| {
            Error::new("file name is not utf-8").ctx("path", entry.path().display().to_string())
        })?;
        let path = entry.path();
        let meta = fs::symlink_metadata(&path).map_err(|err| io_err(err, "stat", &path))?;
        if meta.file_type().is_symlink() {
            return Err(
                Error::new("symlink is not supported").ctx("path", path.display().to_string())
            );
        }
        let relative = relative_of(root, &path)?;
        if ignored(&relative, rules) {
            continue;
        }
        if meta.is_dir() {
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(root, &path, rules, out)?;
            continue;
        }
        if !meta.is_file() || SKIP_FILES.contains(&name.as_str()) {
            continue;
        }
        if !valid_relative(&relative) {
            return Err(Error::new("invalid relative path").ctx("path", relative));
        }
        out.push(ListedFile {
            path: relative,
            stat: stat_of(&meta),
        });
    }
    Ok(())
}

fn stat_of(meta: &fs::Metadata) -> FileStat {
    use std::os::unix::fs::MetadataExt;
    let mtime_nsec = meta
        .modified()
        .ok()
        .and_then(|time| time.duration_since(UNIX_EPOCH).ok())
        .map(|duration| i64::try_from(duration.as_nanos()).unwrap_or(i64::MAX))
        .unwrap_or(0);
    let mode = if meta.mode() & 0o111 != 0 {
        acvc_domain::EXEC_MODE
    } else {
        acvc_domain::FILE_MODE
    };
    FileStat {
        mode,
        fingerprint: Fingerprint {
            size: meta.len(),
            mtime_nsec,
            inode: meta.ino(),
        },
    }
}

pub(crate) fn map_index(path: &Path) -> Result<Option<Mmap>> {
    if !path.is_file() {
        return Ok(None);
    }
    let file = File::open(path).map_err(|err| io_err(err, "open index", path))?;
    // mmap is read-only and dropped after decode.
    let map = unsafe { Mmap::map(&file) }.map_err(|err| io_err(err, "mmap index", path))?;
    Ok(Some(map))
}

#[cfg(test)]
mod tests {
    use super::WorkspaceFs;
    use acvc_domain::DirectoryScan;
    use std::fs;

    #[test]
    fn build_directories_stay_out_of_the_tree() {
        let root = std::env::temp_dir().join(format!("acvc-scan-{}", std::process::id()));
        let _ = fs::remove_dir_all(&root);
        fs::create_dir_all(root.join(".build/checkouts")).unwrap();
        fs::write(root.join(".build/checkouts/lib.o"), b"obj").unwrap();
        fs::create_dir_all(root.join("App/DerivedData/Logs")).unwrap();
        fs::write(root.join("App/DerivedData/Logs/shot.png"), b"png").unwrap();
        fs::create_dir_all(root.join("Sources")).unwrap();
        fs::write(root.join("Sources/Shelf.swift"), b"struct Shelf {}").unwrap();
        fs::write(root.join(".acvc-index"), b"idx").unwrap();
        let listed = WorkspaceFs.list_files(&root).unwrap();
        let paths: Vec<_> = listed.into_iter().map(|file| file.path).collect();
        assert_eq!(paths, vec!["Sources/Shelf.swift".to_string()]);
        fs::remove_dir_all(&root).unwrap();
    }

    #[test]
    fn the_same_ignore_file_leaves_the_same_paths_out() {
        let left = std::env::temp_dir().join(format!("acvc-ignore-a-{}", std::process::id()));
        let right = std::env::temp_dir().join(format!("acvc-ignore-b-{}", std::process::id()));
        for root in [&left, &right] {
            let _ = fs::remove_dir_all(root);
            fs::create_dir_all(root.join("lab/reports")).unwrap();
            fs::write(root.join("lab/reports/shot.png"), b"png-bytes").unwrap();
            fs::write(root.join("Sources.swift"), b"struct Shelf {}").unwrap();
            fs::write(root.join(".acvcignore"), b"# relatorio\nlab/reports\n").unwrap();
        }
        let listed = |root: &std::path::Path| {
            WorkspaceFs
                .list_files(root)
                .unwrap()
                .into_iter()
                .map(|file| file.path)
                .collect::<Vec<_>>()
        };
        assert_eq!(listed(&left), listed(&right));
        assert_eq!(listed(&left), vec!["Sources.swift".to_string()]);
        fs::remove_file(left.join(".acvcignore")).unwrap();
        let open = listed(&left);
        assert!(open.contains(&"lab/reports/shot.png".to_string()));
        fs::remove_dir_all(&left).unwrap();
        fs::remove_dir_all(&right).unwrap();
    }
}
