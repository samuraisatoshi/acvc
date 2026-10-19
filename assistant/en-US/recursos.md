# Resources

A person who has never seen ACVC needs one sentence before these names. ACVC keeps the attempts that should not become commits. Each attempt is the idea, the files, and the numbers. The attempts nobody has beaten on every number still compete. `promote` copies the approved files into a Git checkout that already exists. The names below are what the commands and the JSON use.

## What a hypothesis is

The identifier is the BLAKE3 hash of the canonical bytes: tree, parents, edge, premise, causes, revision, intent, and agent. The campaign name, the date, the measurements, and the token spend do not enter the hash. The same hypothesis is the same node.

Edges:

- `seed` — the only origin.
- `mutate` — the files changed and the belief stands. This is the default for `record`.
- `revert` — the step goes back to an earlier hypothesis.
- `revise_premise` — the belief itself changed.
- `synthesize` — two or more parents.

Status, as JSON values:

- `on_frontier` — measured, and no other hypothesis beats it.
- `superseded` — measured, and another one on the frontier is better.
- `rejected` — someone recorded the motive.
- `unmeasured` — there is no complete trial yet for the campaign schema.

Leaving the frontier does not record a rejection. If there is no motive, the reading names who won, the metric, and that the motive was not recorded. In en-US that sentence is `Motive was not recorded.`

The text a person sees follows the computer language (`LC_ALL`, then `LC_MESSAGES`, then `LANG`). The four JSON values above do not change. An agent reads the `status` field, not the translated sentence.

## Where the files live

The store is `.acvc/` in the `--repo` directory: an append-only log, a pack, and SQLite. The seeded folder keeps `.acvc-workspace` and `.acvc-index`.

The scan does not read `.gitignore`. It skips `.git`, `.acvc`, `target`, `node_modules`, `.build`, and `DerivedData`. A `.acvcignore` at the root of the seeded folder lists relative prefixes; that file does not enter the hash. A symlink during the scan is an error. A hardlink is forbidden.

## Commands

`--repo` points at the store. `--format` is `json` (the default) or `text`.

Read, before acting:

- `context` — the campaign map: problem, causes, agents, interactions, tokens, age, and branches.
- `guide` — the commands plus the map of the open repository.
- `frontier` — what still competes.
- `why` — one hypothesis: cause, measurement, who beat it, parents, and children.
- `recall` — search by the premise. Returns status and parent. An accent is part of the search.
- `compare` — two measured hypotheses: which stays and which files changed.
- `file-diff` — the text of one path between two hypotheses.
- `status` — the working folder against the materialized hypothesis.
- `backtrack` — return to a parent.

Write:

- `init` — creates the store.
- `campaign create NAME --metric name:higher|lower` — the schema. There can be more than one metric.
- `seed` — the first hypothesis of the campaign. A second origin is refused. The next path is `record`.
- `record` — the folder changed. Without `--parent`, the parent is the hypothesis the workspace points at, and the workspace then points at the new hypothesis. Default edge: `mutate`.
- `score --metric name=number` — appends a trial on the same variant. The identifier does not change. It does not erase the earlier trials.
- `reject --reason "..."` — an explicit act. The motive stays in the graph.
- `spend --tokens N` — a side ledger. It does not enter the hash, it is not a metric, and it does not lock `record`. Zero tokens is refused.
- `materialize` — copies the hypothesis into a folder.
- `promote` — deposits the winning variant into a checkout that already exists, or into a folder. It does not merge.
- `pack` — compacts cold blobs. It does not change the reading of status or record.

Viewers, all of them call the executable:

- `web` — page at `127.0.0.1:9420`.
- `tui` — terminal.
- `mcp` — stdio server. It does not open SQLite. Each tool re-executes the binary.
- `obsidian` — installs the viewer in the vault. The Obsidian command is Open ACVC.

Remote: `serve`, `push`, `fetch`. The remote does not check out or merge. It does not resend an object the other side already has.

## Frontier

Every complete measurement that nobody beats enters. Two incomparable measurements of the same hypothesis both stay. A strictly worse measurement leaves the frontier and remains in the history. Do not mix a metric from one trial with a metric from another. A rejected hypothesis stays out. Two incomparable hypotheses both remain. Do not merge the campaign to force a winner.
