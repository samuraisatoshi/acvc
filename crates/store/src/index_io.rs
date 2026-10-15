use std::fs;
use std::path::Path;

use acvc_domain::{IndexEntry, IndexIo, Marker, MarkerIo, Result, decode_index, encode_index};

use crate::error::io_err;
use crate::scan::{WorkspaceFs, map_index};

impl IndexIo for WorkspaceFs {
    fn load(&self, workspace: &Path) -> Result<Vec<IndexEntry>> {
        let path = workspace.join(".acvc-index");
        let Some(map) = map_index(&path)? else {
            return Ok(Vec::new());
        };
        decode_index(&map)
    }

    fn store(&self, workspace: &Path, entries: &[IndexEntry]) -> Result<()> {
        let bytes = encode_index(entries)?;
        write_atomic(&workspace.join(".acvc-index"), &bytes)
    }
}

impl MarkerIo for WorkspaceFs {
    fn load(&self, workspace: &Path) -> Result<Marker> {
        let path = workspace.join(".acvc-workspace");
        let bytes = fs::read(&path).map_err(|err| io_err(err, "read marker", &path))?;
        Marker::decode(&bytes)
    }

    fn store(&self, workspace: &Path, marker: &Marker) -> Result<()> {
        write_atomic(&workspace.join(".acvc-workspace"), &marker.encode()?)
    }
}

fn write_atomic(path: &Path, bytes: &[u8]) -> Result<()> {
    let tmp = path.with_extension("partial");
    fs::write(&tmp, bytes).map_err(|err| io_err(err, "write", &tmp))?;
    fs::rename(&tmp, path).map_err(|err| io_err(err, "rename", path))
}
