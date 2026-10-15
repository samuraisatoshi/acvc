//! Application services. Depends only on domain ports.

#![forbid(unsafe_code)]

mod exchange;
mod materialize;
mod pack;
mod pathutil;
mod place;
mod promote;
mod query;
mod record;
mod reports;
mod score;
mod seed;
mod status;

use std::path::{Path, PathBuf};
use std::sync::Arc;

use acvc_domain::{
    BlobCache, CampaignStore, CheckoutWrite, Clock, DirectoryScan, GitPort, IndexIo, MarkerIo,
    ObjectStore, RecallStore, TrialStore, VariantStore, WorkspaceRegistry,
};

pub use exchange::{AcceptReport, ObjectBytes, Offer};
pub use record::RecordRequest;
pub use reports::*;
pub use seed::SeedRequest;

pub struct Ports {
    pub repo: PathBuf,
    pub objects: Arc<dyn ObjectStore>,
    pub cache: Arc<dyn BlobCache>,
    pub campaigns: Arc<dyn CampaignStore>,
    pub variants: Arc<dyn VariantStore>,
    pub trials: Arc<dyn TrialStore>,
    pub recall: Arc<dyn RecallStore>,
    pub workspaces: Arc<dyn WorkspaceRegistry>,
    pub scan: Arc<dyn DirectoryScan>,
    pub checkout: Arc<dyn CheckoutWrite>,
    pub index: Arc<dyn IndexIo>,
    pub marker: Arc<dyn MarkerIo>,
    pub clock: Arc<dyn Clock>,
    pub git: Arc<dyn GitPort>,
}

pub struct App {
    pub(crate) ports: Ports,
}

impl App {
    pub fn new(ports: Ports) -> Self {
        Self { ports }
    }

    pub fn repo(&self) -> &Path {
        &self.ports.repo
    }

    pub(crate) fn repo_string(&self) -> acvc_domain::Result<String> {
        self.ports
            .repo
            .to_str()
            .map(str::to_string)
            .ok_or_else(|| acvc_domain::Error::new("repo path is not utf-8"))
    }
}
