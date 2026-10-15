use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::Path;

use acvc_domain::{CheckoutWrite, Result};
use reflink_copy::reflink_or_copy;

use crate::error::io_err;
use crate::scan::WorkspaceFs;

impl CheckoutWrite for WorkspaceFs {
    fn prepare_empty_dir(&self, path: &Path) -> Result<()> {
        if path.exists() {
            let mut entries = fs::read_dir(path).map_err(|err| io_err(err, "read dir", path))?;
            if entries.next().is_some() {
                return Err(acvc_domain::Error::new("destination is not empty")
                    .ctx("path", path.display().to_string()));
            }
        } else {
            fs::create_dir_all(path).map_err(|err| io_err(err, "create", path))?;
        }
        Ok(())
    }

    fn create_dirs(&self, path: &Path) -> Result<()> {
        fs::create_dir_all(path).map_err(|err| io_err(err, "create", path))
    }

    fn place_reflink(&self, from: &Path, to: &Path, mode: u32) -> Result<()> {
        if let Some(parent) = to.parent() {
            fs::create_dir_all(parent).map_err(|err| io_err(err, "create", parent))?;
        }
        if to.exists() {
            fs::remove_file(to).map_err(|err| io_err(err, "remove", to))?;
        }
        reflink_or_copy(from, to).map_err(|err| io_err(err, "reflink", to))?;
        let mut perms = fs::metadata(to)
            .map_err(|err| io_err(err, "stat", to))?
            .permissions();
        perms.set_mode(if mode & 0o111 != 0 { 0o755 } else { 0o644 });
        fs::set_permissions(to, perms).map_err(|err| io_err(err, "chmod", to))?;
        Ok(())
    }
}
