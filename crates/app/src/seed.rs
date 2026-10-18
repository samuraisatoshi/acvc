use std::path::PathBuf;

use acvc_domain::{
    EdgeKind, Error, ListedFile, PlannedFile, Premise, Result, Variant, rebuild_merkle,
};

use crate::pathutil::join_rel;
use crate::{App, HeavyPath, SeedReport};

pub struct SeedRequest {
    pub campaign: String,
    pub path: PathBuf,
    pub claim: String,
    pub assumptions: Vec<String>,
    pub intent: String,
    pub agent: String,
}

impl App {
    pub fn create_campaign(&self, name: &str, schema: &[acvc_domain::MetricSpec]) -> Result<()> {
        self.ports
            .campaigns
            .insert_campaign(name, schema, self.ports.clock.now_millis())
    }

    pub fn seed(&self, request: SeedRequest) -> Result<SeedReport> {
        let listed = self.ports.scan.list_files(&request.path)?;
        let mut files = Vec::with_capacity(listed.len());
        for file in &listed {
            let bytes = self
                .ports
                .scan
                .read_file(&join_rel(&request.path, &file.path))?;
            files.push(PlannedFile {
                path: file.path.clone(),
                mode: file.stat.mode,
                hash: self.ports.objects.put_blob(&bytes)?,
            });
        }
        let dirty: Vec<String> = files.iter().map(|file| file.path.clone()).collect();
        let tree = rebuild_merkle(
            None,
            &files,
            &dirty,
            &mut |_| Err(Error::new("seed has no previous tree")),
            &mut |entries| self.ports.objects.put_tree(entries),
        )?;
        let variant = Variant {
            tree,
            parents: Vec::new(),
            edge: EdgeKind::Seed,
            premise: Premise {
                claim: request.claim,
                assumptions: request.assumptions,
                revises: None,
            },
            intent: request.intent,
            agent: request.agent,
        };
        let id = variant.id()?;
        let existing = self.ports.variants.variants_in(&request.campaign)?;
        let same = existing
            .iter()
            .any(|stored| stored.variant.id().ok() == Some(id));
        if !existing.is_empty() && !same {
            return Err(Error::new("campaign already has an origin")
                .ctx("campaign", request.campaign)
                .ctx("fix", "record from the parent workspace"));
        }
        let id = self.ports.variants.insert_variant(
            &request.campaign,
            &variant,
            self.ports.clock.now_millis(),
        )?;
        self.persist_workspace(&request.path, &request.campaign, id, &files)?;
        Ok(SeedReport {
            variant: id,
            tree,
            files: files.len(),
            hashed: files.len(),
            heaviest: heaviest(&listed),
        })
    }
}

fn heaviest(listed: &[ListedFile]) -> Vec<HeavyPath> {
    let mut ranked: Vec<&ListedFile> = listed.iter().collect();
    ranked.sort_by(|left, right| {
        right
            .stat
            .fingerprint
            .size
            .cmp(&left.stat.fingerprint.size)
            .then_with(|| left.path.cmp(&right.path))
    });
    ranked.truncate(5);
    ranked
        .into_iter()
        .map(|file| HeavyPath {
            path: file.path.clone(),
            bytes: file.stat.fingerprint.size,
        })
        .collect()
}
