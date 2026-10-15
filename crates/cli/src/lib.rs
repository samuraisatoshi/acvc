//! Composition root.
#![forbid(unsafe_code)]

mod args;
mod dispatch;
mod locate;
mod mcp;
mod mcp_tools;
mod present;
mod remote;
mod web;
mod web_page;
mod wire;

use std::path::Path;
use std::sync::Arc;

use acvc_app::{App, Ports};
use acvc_domain::{
    BlobCache, CampaignStore, CheckoutWrite, Clock, DirectoryScan, GitPort, IndexIo, MarkerIo,
    ObjectStore, RecallStore, Result, TrialStore, VariantStore, WorkspaceRegistry,
};
use acvc_store::{
    GitProcess, Graph, Objects, Opened, SystemClock, WorkspaceFs, find_repo, init_repo, open_repo,
};

pub fn init_at(path: &Path) -> Result<App> {
    std::fs::create_dir_all(path).map_err(|err| {
        acvc_domain::Error::new(err.to_string())
            .ctx("op", "create")
            .ctx("path", path.display().to_string())
    })?;
    Ok(compose(init_repo(path)?))
}

pub fn open_at(path: &Path) -> Result<App> {
    Ok(compose(open_repo(path)?))
}

pub fn find_at(start: &Path) -> Result<App> {
    open_at(&find_repo(start)?)
}

pub fn run() -> Result<()> {
    dispatch::dispatch(args::parse()?)
}

fn compose(opened: Opened) -> App {
    let objects: Arc<Objects> = opened.objects;
    let graph: Arc<Graph> = opened.graph;
    let files: Arc<WorkspaceFs> = opened.files;
    let clock: Arc<SystemClock> = opened.clock;
    let git: Arc<GitProcess> = opened.git;
    let objects_store: Arc<dyn ObjectStore> = objects.clone();
    let cache: Arc<dyn BlobCache> = objects;
    let campaigns: Arc<dyn CampaignStore> = graph.clone();
    let variants: Arc<dyn VariantStore> = graph.clone();
    let trials: Arc<dyn TrialStore> = graph.clone();
    let recall: Arc<dyn RecallStore> = graph.clone();
    let workspaces: Arc<dyn WorkspaceRegistry> = graph;
    let scan: Arc<dyn DirectoryScan> = files.clone();
    let checkout: Arc<dyn CheckoutWrite> = files.clone();
    let index: Arc<dyn IndexIo> = files.clone();
    let marker: Arc<dyn MarkerIo> = files;
    let clock: Arc<dyn Clock> = clock;
    let git: Arc<dyn GitPort> = git;
    App::new(Ports {
        repo: opened.repo,
        objects: objects_store,
        cache,
        campaigns,
        variants,
        trials,
        recall,
        workspaces,
        scan,
        checkout,
        index,
        marker,
        clock,
        git,
    })
}
