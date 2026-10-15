use std::path::PathBuf;

use acvc_domain::{
    EdgeKind, Error, Hash, IndexEntry, PlannedFile, Premise, Result, Variant, plan_changes,
    rebuild_merkle,
};

use crate::pathutil::join_rel;
use crate::{App, RecordReport};

pub struct RecordRequest {
    pub workspace: PathBuf,
    pub edge: EdgeKind,
    pub claim: String,
    pub assumptions: Vec<String>,
    pub revises: Option<Hash>,
    pub intent: String,
    pub agent: String,
    pub parents: Vec<Hash>,
}

impl App {
    pub fn record(&self, request: RecordRequest) -> Result<RecordReport> {
        if request.edge == EdgeKind::Seed {
            return Err(Error::new("record cannot create a seed"));
        }
        let marker = self.ports.marker.load(&request.workspace)?;
        let base = self.ports.variants.get_variant(&marker.variant)?;
        if base.campaign != marker.campaign {
            return Err(Error::new("workspace campaign does not match variant")
                .ctx("campaign", marker.campaign));
        }
        let parents = if request.parents.is_empty() {
            vec![marker.variant]
        } else {
            request.parents
        };
        for parent in &parents {
            let stored = self.ports.variants.get_variant(parent)?;
            if stored.campaign != marker.campaign {
                return Err(
                    Error::new("parent is in another campaign").ctx("parent", parent.to_hex())
                );
            }
        }
        let revises = match request.edge {
            EdgeKind::RevisePremise => Some(request.revises.unwrap_or(parents[0])),
            _ => request.revises,
        };
        let index = self.ports.index.load(&request.workspace)?;
        let listed = self.ports.scan.list_files(&request.workspace)?;
        let plan = plan_changes(&index, &listed);
        let mut files = plan.clean;
        let reused = files.len();
        let mut hashed = 0;
        let mut dirty = plan.deleted;
        for file in plan.dirty.into_iter().chain(plan.added) {
            let bytes = self
                .ports
                .scan
                .read_file(&join_rel(&request.workspace, &file.path))?;
            files.push(PlannedFile {
                path: file.path.clone(),
                mode: file.stat.mode,
                hash: self.ports.objects.put_blob(&bytes)?,
            });
            dirty.push(file.path);
            hashed += 1;
        }
        let tree = rebuild_merkle(
            Some(base.variant.tree),
            &files,
            &dirty,
            &mut |hash| self.ports.objects.get_tree(hash),
            &mut |entries| self.ports.objects.put_tree(entries),
        )?;
        let variant = Variant {
            tree,
            parents,
            edge: request.edge,
            premise: Premise {
                claim: request.claim,
                assumptions: request.assumptions,
                revises,
            },
            intent: request.intent,
            agent: request.agent,
        };
        let id = self.ports.variants.insert_variant(
            &marker.campaign,
            &variant,
            self.ports.clock.now_millis(),
        )?;
        self.persist_workspace(&request.workspace, &marker.campaign, id, &files)?;
        Ok(RecordReport {
            variant: id,
            tree,
            hashed,
            reused,
            edge: request.edge,
        })
    }

    pub(crate) fn persist_workspace(
        &self,
        workspace: &std::path::Path,
        campaign: &str,
        variant: Hash,
        files: &[PlannedFile],
    ) -> Result<()> {
        let mut entries = Vec::with_capacity(files.len());
        for file in files {
            let stat = self.ports.scan.metadata(&join_rel(workspace, &file.path))?;
            entries.push(IndexEntry {
                path: file.path.clone(),
                mode: file.mode,
                hash: file.hash,
                fingerprint: stat.fingerprint,
            });
        }
        entries.sort_by(|left, right| left.path.cmp(&right.path));
        self.ports.index.store(workspace, &entries)?;
        self.ports.marker.store(
            workspace,
            &acvc_domain::Marker {
                repo: self.repo_string()?,
                campaign: campaign.to_string(),
                variant,
            },
        )?;
        self.ports.workspaces.register(&variant, workspace)
    }
}
