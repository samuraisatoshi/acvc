//! Driven adapters: object log, SQLite, filesystem, and git.

mod campaigns;
mod checkout;
mod clock;
mod db;
mod error;
mod git;
mod graph;
mod index_io;
mod objects;
mod open;
mod pack;
mod recall;
mod recall_store;
mod scan;
mod schema;
mod spends;
mod trials;
mod variants;
mod workspaces;

pub use clock::SystemClock;
pub use git::GitProcess;
pub use graph::Graph;
pub use objects::Objects;
pub use open::{Opened, find_repo, init_repo, open_repo};
pub use scan::WorkspaceFs;
