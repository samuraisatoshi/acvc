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

const SKIP_DIRS: &[&str] = &[".git", ".acvc", "target", "node_modules"];
const SKIP_FILES: &[&str] = &[".acvc-workspace", ".acvc-index"];

impl DirectoryScan for WorkspaceFs {
    fn list_files(&self, root: &Path) -> Result<Vec<ListedFile>> {
        let mut files = Vec::new();
        walk(root, root, &mut files)?;
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

fn walk(root: &Path, dir: &Path, out: &mut Vec<ListedFile>) -> Result<()> {
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
        if meta.is_dir() {
            if SKIP_DIRS.contains(&name.as_str()) {
                continue;
            }
            walk(root, &path, out)?;
            continue;
        }
        if !meta.is_file() || SKIP_FILES.contains(&name.as_str()) {
            continue;
        }
        let relative = path
            .strip_prefix(root)
            .map_err(|_| Error::new("path escapes root").ctx("path", path.display().to_string()))?;
        let relative = relative
            .to_str()
            .ok_or_else(|| Error::new("path is not utf-8").ctx("path", path.display().to_string()))?
            .replace('\\', "/");
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
