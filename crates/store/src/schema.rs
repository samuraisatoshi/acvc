pub(crate) const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS meta (
  key TEXT PRIMARY KEY,
  value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS campaigns (
  name TEXT PRIMARY KEY,
  created_at INTEGER NOT NULL
);
CREATE TABLE IF NOT EXISTS metric_specs (
  campaign TEXT NOT NULL,
  ord INTEGER NOT NULL,
  name TEXT NOT NULL,
  direction INTEGER NOT NULL,
  PRIMARY KEY (campaign, name),
  FOREIGN KEY (campaign) REFERENCES campaigns(name)
);
CREATE TABLE IF NOT EXISTS objects (
  hash TEXT PRIMARY KEY,
  offset INTEGER NOT NULL,
  len INTEGER NOT NULL,
  kind INTEGER NOT NULL,
  loc INTEGER NOT NULL DEFAULT 0,
  clen INTEGER NOT NULL DEFAULT 0
);
CREATE TABLE IF NOT EXISTS variants (
  id TEXT PRIMARY KEY,
  campaign TEXT NOT NULL,
  tree TEXT NOT NULL,
  edge INTEGER NOT NULL,
  claim TEXT NOT NULL,
  intent TEXT NOT NULL,
  agent TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  revises TEXT,
  FOREIGN KEY (campaign) REFERENCES campaigns(name)
);
CREATE TABLE IF NOT EXISTS parents (
  variant TEXT NOT NULL,
  ord INTEGER NOT NULL,
  parent TEXT NOT NULL,
  PRIMARY KEY (variant, ord),
  FOREIGN KEY (variant) REFERENCES variants(id)
);
CREATE TABLE IF NOT EXISTS assumptions (
  variant TEXT NOT NULL,
  ord INTEGER NOT NULL,
  text TEXT NOT NULL,
  PRIMARY KEY (variant, ord),
  FOREIGN KEY (variant) REFERENCES variants(id)
);
CREATE TABLE IF NOT EXISTS trials (
  id TEXT PRIMARY KEY,
  variant TEXT NOT NULL,
  nonce INTEGER NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (variant) REFERENCES variants(id)
);
CREATE TABLE IF NOT EXISTS trial_metrics (
  trial TEXT NOT NULL,
  name TEXT NOT NULL,
  value REAL NOT NULL,
  PRIMARY KEY (trial, name),
  FOREIGN KEY (trial) REFERENCES trials(id)
);
CREATE TABLE IF NOT EXISTS rejections (
  variant TEXT PRIMARY KEY,
  reason TEXT NOT NULL,
  created_at INTEGER NOT NULL,
  FOREIGN KEY (variant) REFERENCES variants(id)
);
CREATE TABLE IF NOT EXISTS workspaces (
  variant TEXT PRIMARY KEY,
  path TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS recall_doc (
  id INTEGER PRIMARY KEY,
  variant TEXT NOT NULL UNIQUE,
  campaign TEXT NOT NULL,
  body TEXT NOT NULL
);
CREATE VIRTUAL TABLE IF NOT EXISTS recall_fts USING fts5(
  body,
  content='recall_doc',
  content_rowid='id',
  tokenize='unicode61'
);
CREATE INDEX IF NOT EXISTS idx_variants_campaign ON variants(campaign);
CREATE INDEX IF NOT EXISTS idx_parents_parent ON parents(parent);
CREATE INDEX IF NOT EXISTS idx_trials_variant ON trials(variant, created_at);
";
