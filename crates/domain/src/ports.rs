use std::path::{Path, PathBuf};

use crate::{
    Hash, IndexEntry, ListedFile, Marker, MetricSpec, Result, StoredTrial, StoredVariant,
    TreeEntry, Trial, Variant,
};

pub struct PackReport {
    pub objects: u64,
    pub raw_bytes: u64,
    pub packed_bytes: u64,
}

pub trait ObjectStore: Send + Sync {
    fn put_blob(&self, content: &[u8]) -> Result<Hash>;
    fn put_tree(&self, entries: &[TreeEntry]) -> Result<Hash>;
    fn get_blob(&self, hash: &Hash) -> Result<Vec<u8>>;
    fn get_tree(&self, hash: &Hash) -> Result<Vec<TreeEntry>>;
    fn has_object(&self, hash: &Hash) -> Result<bool>;
    fn read_object(&self, hash: &Hash) -> Result<(u8, Vec<u8>)>;
    fn put_object(&self, kind: u8, bytes: &[u8]) -> Result<Hash>;
    fn pack(&self) -> Result<PackReport>;
}

pub trait BlobCache: Send + Sync {
    fn materialize_blob(&self, hash: &Hash) -> Result<PathBuf>;
}

pub trait CampaignStore: Send + Sync {
    fn insert_campaign(&self, name: &str, schema: &[MetricSpec], now: i64) -> Result<()>;
    fn list_campaigns(&self) -> Result<Vec<String>>;
    fn schema_of(&self, name: &str) -> Result<Vec<MetricSpec>>;
}

pub trait VariantStore: Send + Sync {
    fn insert_variant(&self, campaign: &str, variant: &Variant, now: i64) -> Result<Hash>;
    fn get_variant(&self, id: &Hash) -> Result<StoredVariant>;
    fn resolve(&self, prefix: &str) -> Result<Hash>;
    fn children(&self, id: &Hash) -> Result<Vec<Hash>>;
    fn variants_in(&self, campaign: &str) -> Result<Vec<StoredVariant>>;
}

pub trait TrialStore: Send + Sync {
    fn insert_trial(&self, trial: &Trial, now: i64) -> Result<Hash>;
    fn trials_of(&self, variant: &Hash) -> Result<Vec<StoredTrial>>;
    fn reject(&self, variant: &Hash, reason: &str, now: i64) -> Result<()>;
    fn rejection_of(&self, variant: &Hash) -> Result<Option<String>>;
}

pub trait RecallStore: Send + Sync {
    fn recall(&self, campaign: &str, query: &str) -> Result<Vec<Hash>>;
}

pub trait WorkspaceRegistry: Send + Sync {
    fn register(&self, variant: &Hash, path: &Path) -> Result<()>;
    fn find(&self, variant: &Hash) -> Result<Option<PathBuf>>;
}

pub trait DirectoryScan: Send + Sync {
    fn list_files(&self, root: &Path) -> Result<Vec<ListedFile>>;
    fn read_file(&self, path: &Path) -> Result<Vec<u8>>;
    fn metadata(&self, path: &Path) -> Result<crate::FileStat>;
}

pub trait CheckoutWrite: Send + Sync {
    fn prepare_empty_dir(&self, path: &Path) -> Result<()>;
    fn create_dirs(&self, path: &Path) -> Result<()>;
    fn place_reflink(&self, from: &Path, to: &Path, mode: u32) -> Result<()>;
}

pub trait IndexIo: Send + Sync {
    fn load(&self, workspace: &Path) -> Result<Vec<IndexEntry>>;
    fn store(&self, workspace: &Path, entries: &[IndexEntry]) -> Result<()>;
}

pub trait MarkerIo: Send + Sync {
    fn load(&self, workspace: &Path) -> Result<Marker>;
    fn store(&self, workspace: &Path, marker: &Marker) -> Result<()>;
}

pub trait Clock: Send + Sync {
    fn now_millis(&self) -> i64;
    fn nonce(&self) -> u64;
}

pub trait GitPort: Send + Sync {
    fn is_work_tree(&self, path: &Path) -> Result<bool>;
    fn commit_paths(
        &self,
        repo: &Path,
        files: &[String],
        message: &str,
        author: &str,
    ) -> Result<()>;
}
