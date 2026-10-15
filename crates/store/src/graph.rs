use std::sync::Arc;

use acvc_domain::Result;

use crate::db::Db;

pub struct Graph {
    db: Arc<Db>,
}

impl Graph {
    pub(crate) fn new(db: Arc<Db>) -> Self {
        Self { db }
    }

    pub(crate) fn lock(&self) -> Result<std::sync::MutexGuard<'_, rusqlite::Connection>> {
        self.db.lock()
    }
}
