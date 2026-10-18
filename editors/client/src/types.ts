export interface AcvcOptions {
  binary: string;
  cwd: string;
  repo?: string;
}

export interface CampaignList {
  campaigns: string[];
}

export interface Seeded {
  variant: string;
  tree: string;
  files: number;
  hashed: number;
}

export interface Recorded {
  variant: string;
  tree: string;
  hashed: number;
  reused: number;
  edge: string;
}

export interface Scored {
  trial: string;
  variant: string;
}

export interface FrontierVariant {
  variant: string;
  claim: string;
  metrics: Record<string, number>;
}

export interface Frontier {
  campaign: string;
  variants: FrontierVariant[];
}

export interface Why {
  variant: string;
  campaign: string;
  edge: string;
  claim: string;
  assumptions: string[];
  intent: string;
  agent: string;
  parents: string[];
  children: string[];
  trials: { id: string; metrics: Record<string, number> }[];
  rejection: string | null;
  on_frontier?: boolean;
  beaten_by?: { claim: string; gaps: { name: string; direction: string; winner: number; loser: number }[] }[];
  parent_claims?: string[];
  child_claims?: string[];
}

export interface Compare {
  left: string;
  right: string;
  added: string[];
  removed: string[];
  modified: string[];
  metrics: { name: string; direction?: string; left: number | null; right: number | null }[];
  dominance: string;
  left_claim?: string;
  right_claim?: string;
}

export interface Recall {
  hits: { variant: string; claim: string; status?: string; parents?: { variant: string; claim: string }[] }[];
}

export interface FileDiff {
  left: string;
  right: string;
  path: string;
  diff: string;
}
