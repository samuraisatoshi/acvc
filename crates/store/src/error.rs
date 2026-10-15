use std::io;
use std::path::Path;

use acvc_domain::Error;

pub(crate) fn io_err(err: io::Error, op: &str, path: &Path) -> Error {
    Error::new(err.to_string())
        .ctx("op", op)
        .ctx("path", path.display().to_string())
}

pub(crate) fn sql_err(err: rusqlite::Error, op: &str) -> Error {
    Error::new(err.to_string()).ctx("op", op)
}
