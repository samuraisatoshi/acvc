# ACVC: hypothesis memory for experimental work on code

For readers new to ACVC: an agent produces trials, measures results, and
abandons paths. ACVC preserves that exploration outside Git history.
`acvc promote --dest` materializes the selected files; `--git` also creates a
commit in the specified checkout.

## Abstract

Coding agents explore alternatives, backtrack, and revise hypotheses during a
session. Git records states that someone chose to preserve, but a commit does
not say which alternatives were still competing, how they were measured, or
why one was abandoned [1, 2, 4].

ACVC records this process as a campaign of hypotheses. Each hypothesis combines
a premise, its assumptions, an intent, its relationship to its parents, and a
file tree. Trials add measurements without changing hypothesis identity. The
frontier retains complete measurements that are not dominated; rejection
remains a separate act and requires an explicit reason.

Storage is local and content-addressed. Merkle trees avoid writing identical
files again [12], while Git remains responsible for the variant that will be
preserved. The frontier rule has a TLA+/TLC model in the repository. This paper
does not present a user study or an evaluation at large-monorepo scale.

## 1. Problem and scope

### 1.1 What a commit does not record

An agent may produce many versions before reaching a useful change. If every
attempt becomes a commit, decision history becomes mixed with exploration
history. If only the chosen attempt becomes a commit, the premises,
measurements, and reasons for abandoning the others disappear. Studies of
exploratory programming and notebooks describe similar disorder [5–7].

The problem addressed by ACVC is not recovering an old version. It is
preserving the relationship between a claim about the code, the files that
materialize it, and the evidence used to compare it with other claims.

### 1.2 The system's question

After several attempts, which hypotheses are still competing, and why? The
answer must survive the session that produced the attempts without turning Git
into an experiment diary.

### 1.3 Responsibility boundary

ACVC does not run tests, create isolated environments, perform merges, or
replace Git. It receives files and measurements produced by external tools.
When a hypothesis is selected, `promote` materializes its files at a destination
or creates a commit in a Git worktree, depending on the requested mode.

## 2. Conceptual model

A **campaign** represents a comparable question. It declares the metrics in
advance and the direction of each metric: `higher` when larger values are
better, and `lower` when smaller values are better.

A **hypothesis** includes:

- a premise and assumptions written in human language;
- the experiment's intent and the agent's identity;
- a file tree;
- parents and the relationship to those parents.

Relationships are `seed`, `mutate`, `revert`, `revise_premise`, and
`synthesize`. A campaign has one origin. A mutation starts from the current
hypothesis or an explicit parent; a revision references by hash the hypothesis
whose premise is being revised; a synthesis has two or more parents. Because
the relationship participates in identity, a mutation and a revision over the
same tree remain distinct hypotheses.

A **trial** measures a hypothesis. It may be complete or incomplete and does
not participate in hypothesis identity. A new trial does not erase earlier
ones.

A **rejection** records a decision and its reason. It is not automatically
inferred from a worse measurement. This separation prevents the system from
inventing a justification that nobody supplied.

## 3. Frontier semantics

A measurement enters the frontier only when it contains every metric declared
by the campaign. Incomplete measurements remain in history but do not
participate in comparison.

One measurement dominates another when it is no worse in every metric and
better in at least one [8, 16, 17]. The frontier contains every complete
measurement that no other measurement dominates. Two alternatives may remain
simultaneously: one may be faster while the other is more accurate. Metrics
from different trials are never combined into a fictitious score.

Within one hypothesis, a strictly worse complete trial ceases to represent it.
Across the campaign, all non-dominated representatives remain.

The JSON state uses four stable values:

- `on_frontier`: a non-dominated complete measurement exists;
- `superseded`: another hypothesis has a dominating measurement;
- `rejected`: someone explicitly recorded closure and its reason;
- `unmeasured`: no complete trial exists yet.

Leaving the frontier does not mean dying. Without a rejection, the reading may
show who won and on which metric, but it must state that no reason was recorded.

## 4. Identity and storage

### 4.1 Control by hash

Git stores content-addressed blobs, trees, and commits [1, 2]. A tree references
hashes; it does not contain every file's bytes again. ACVC applies the same
principle to hypotheses, using BLAKE3 [3].

Hypothesis identity is the hash of a canonical representation that includes the
tree, parents, relationship, premise, assumptions, revision, intent, and agent.
Campaign name, date, measurements, token consumption, and workspace path remain
outside that hash. Scoring a hypothesis therefore does not create another
hypothesis.

### 4.2 Trees and incremental writing

Files form a Merkle tree [12]. A file becomes a blob; a directory references
names, modes, types, and hashes. During recording, a clean path reuses its hash.
Only new or changed paths require reading and a new blob. Identical subtrees can
be compared by hash.

Storage lives in `.acvc/`. During normal writes, new blobs and trees are
appended to `objects.log`. `pack` moves cold blobs to `objects.pack` and rewrites
the hot log; trees remain in the log. SQLite stores campaigns, variants,
parents, trials, rejections, spending, and the FTS5 text index [13]. Interfaces
must not open these files directly; the executable is the public contract.

Scanning ignores fixed directories such as `.git`, `.acvc`, `target`, and
`node_modules`, and accepts exclusions in `.acvcignore`. `.gitignore` does not
define hypothesis identity.

## 5. Operational flow

A session starts by reading:

1. `context` presents the campaign and its relationships;
2. `frontier` shows what is still competing;
3. `why`, `recall`, `compare`, and `file-diff` explain existing hypotheses.

When the campaign does not exist, the user declares the metrics and records the
origin with `seed`. After changing files, the user runs `record` to create a
hypothesis linked to its parent. `score` adds a real measurement; `reject`
closes a hypothesis only when a known reason exists.

Token consumption is a side ledger maintained by `spend`. It is neither a
metric nor a budget, does not participate in identity, and must be observed
rather than estimated.

At the end, `promote --dest` materializes the hypothesis files in a directory.
`promote --git` requires a Git worktree, materializes and stages those files,
and creates a commit. Neither mode clears the destination or removes paths that
are absent from the hypothesis. The operation neither merges nor rewrites
hypothesis history.

## 6. Interfaces and source of truth

The CLI defines the product contract. Standard output is JSON; `--format text`
produces sentences for people. State values remain stable across languages.

The MCP server and web page delegate commands to the current executable; editor
viewers run the configured binary. The TUI calls the same application core
directly. These interfaces do not keep another implementation of identity,
frontier, or storage.

When documentation and the executable disagree, the executable is the
behavioral authority. `acvc assistant` exposes the installed operational
contract.

## 7. Available evidence

The repository contains a TLA+/TLC model for the frontier rule. This evidence is
limited to the model properties over the explored states; it does not
demonstrate usability, adoption, or industrial performance.

The examples exercise campaigns with one and several metrics, incomparable
measurements, rejection, and promotion. They demonstrate the expected flow but
do not constitute a controlled study.

| Claim | Available evidence | Not established |
| --- | --- | --- |
| The frontier preserves non-dominated alternatives | TLA+/TLC model and tests | Comprehension by new users |
| Identity is stable for the same canonical content | Definition and implementation | Performance in extreme repositories |
| Interfaces share the executable | Declared architecture | Absence of future divergence |
| Hypotheses help the next session | Project use cases | Improvement in a longitudinal study |

## 8. Relationship to other systems

The systems below operate at neighboring stages and may appear to be
substitutes. The comparison separates responsibilities: version control,
execution isolation, and experiment tracking are not the same as preserving
competing hypotheses.

| Project | What it stores | Where it stops, relative to ACVC |
| --- | --- | --- |
| Git [1, 2] | Content-addressed blobs, trees, and commits | A commit records a decision; abandoned hypotheses do not form a frontier. |
| Gitless [4] | A conceptual layer over Git | Reorganizes Git concepts without premises or trials. |
| container-use [11] | A container, worktree, and branch per agent, with commands in Git notes | Isolates execution; it does not score hypotheses. |
| Scion [25] | Content-addressed changesets and blobs on a server for a monorepo | Makes history and checkout cheaper; it does not keep a frontier of measured hypotheses. |
| Variolite [5] | Lightweight variants inside the editor | Explores fragments without a Merkle campaign or frontier. |
| DVC [19] | Data, models, and pipelines over Git | Pipeline execution is central; there is no hypothesis edge or code frontier. |
| MLflow [9] | Run parameters, metrics, and artifacts | The run is central; the tree is not a Merkle hypothesis. |
| Sacred [10] | Configuration, source, and results | Records execution without premise revision or a frontier. |
| Weights & Biases [20] | Hosted experiments and charts | Tracks runs, not a local hypothesis hash. |
| Jupyter [21] | Narrative notebooks and outputs | Does not share subtrees by hash across alternatives. |
| Unison [22] | Content-addressed language definitions | The hash names a definition, not a measured hypothesis. |
| Nix [23] | Reproducible builds by derivation | The hash controls a build, not a premise and frontier. |
| Jujutsu [24] | Git-compatible version control | Maintains change history, not an experimental campaign. |
| Software product lines [18] | Feature variants | A feature is not a hypothesis under a metric schema. |

### 8.1 Scion and ACVC: from exploration to history

[Scion](https://www.anthropic.com/engineering/redesigning-source-control) is an
internal Anthropic system for a large monorepo and a high volume of changes
produced by agents [25]. Its client-server model handles changesets, queries,
and sparse checkouts. The published numbers—including 30,401 merges on
September 9, 2026, nearly 100 GB of history, and a 19 ms `status`—come from
Anthropic's report; this repository did not measure them.

Scion appears here because it also addresses source control for agents and
content by hash. Its central object is different: Scion manages monorepo
changesets; ACVC manages experimental hypotheses, measurements, and rejections.
That distinction enables composition rather than a choice between products.

ACVC can occupy the exploration plane. Agents record alternatives, add trials,
and keep hypotheses that have not yet been selected outside the main history.
The frontier reduces the set that requires a decision without erasing dominated
or rejected results. Scion can occupy the integration plane: after selection,
it receives the changeset and supports historical queries, blame, sparse
checkouts, and the monorepo's read volume.

The joint value lies in that transition. ACVC preserves the evidence that
explains why a change was selected; Scion makes operations over the history that
received the change inexpensive. When several attempts would otherwise reach
`main`, ACVC can prevent the whole exploration from becoming history. When an
attempt is already the decision, it can go directly to source control. In both
cases, Scion solves repository scale without deciding which hypothesis won, and
ACVC organizes the decision without solving checkout or blame at extreme scale.

This composition is still architectural, not an integration delivered by this
repository. Current ACVC can materialize files and create Git commits; it has no
Scion client. Joint use requires an external layer that converts a promoted
hypothesis into a Scion changeset. Section 8.4 shows where this handoff enters
the flow and which responsibilities remain separate.

### 8.2 container-use: execution isolation

[container-use](https://github.com/dagger/container-use) is an MCP server and a
Dagger-based CLI [11]. It creates an isolated environment for each agent, with
a container and Git branch, and records execution commands. Its purpose is to
prevent collisions between parallel work.

It answers where execution happened, which commands ran, and which files
changed. ACVC answers what the attempt claimed, how it was measured, and whether
it still belongs to the frontier. A workflow can use container-use for
execution and ACVC for comparable memory across executions.

### 8.3 Trackers, notebooks, and provenance

MLflow, Sacred, DVC, and Weights & Biases associate runs with parameters, data,
artifacts, or metrics [9, 10, 19, 20]. Notebooks preserve code, narrative, and
outputs [6, 7, 21]. W3C PROV provides a general model of entities, activities,
and agents [15]. ACVC's scope is narrow: it links a human premise to a code
tree, its lineage, and a multi-objective comparison.

### 8.4 Composition flow

ACVC sits before entry into history. The frontier informs the choice; `promote`
does not choose the hypothesis. Depending on the mode, it materializes files or
creates a Git commit. Authorization and the selection rule belong to the caller,
whether a person, script, or agent. In a Scion environment, an external layer
converts the promoted output into a changeset; current ACVC does not implement
that final step.

```mermaid
flowchart TB
  subgraph auto ["Machine to machine"]
    direction LR
    a1["agent records and measures"] --> f1["ACVC computes the frontier"]
    f1 --> p1["agent applies its rule"]
    p1 --> g1["promote --dest materializes files"]
    p1 --> r1["other attempts remain in .acvc"]
    g1 --> x1["external integration creates changeset"]
    x1 --> s1["Scion records the decision"]
  end
  subgraph human ["Person in the decision"]
    direction LR
    a2["agent records and measures"] --> f2["ACVC computes the frontier"]
    f2 --> h2["person reads the frontier"]
    h2 --> g2["promote --dest or reject"]
    h2 --> r2["other attempts remain in .acvc"]
    g2 --> x2["external integration creates changeset"]
    x2 --> s2["Scion records the decision"]
  end
```

| | Scion only | ACVC only | Both together |
| --- | --- | --- | --- |
| Question | Who changed a line, and what entered history? | Which complete measurements still compete? | Scion answers for history; ACVC answers for the preceding search. |
| Unpromoted attempt | The post does not define a separate experimental repository. | Remains in `.acvc`, with measurements. | Remains in `.acvc`; an external integration sends only the promoted decision to Scion. |
| Value of composition | Scalable history and checkout. | Explainable selection among measured hypotheses. | The decision retains its evidence without transferring the whole exploration into operational history. |
| Out of scope | Choosing among measured attempts. | Checkout and blame for an extreme monorepo. | Selection remains external; checkout cost remains Scion's responsibility. |

## 9. Limitations

### 9.1 Validity of records

The system does not verify the honesty of a premise or whether a metric
represents the real objective. A complete measurement may be irrelevant. The
frontier organizes the supplied evidence; it does not guarantee its quality.

### 9.2 Execution and isolation

ACVC does not execute code, automatically capture the environment, or isolate
processes. Shells, containers, and CI remain responsible for those tasks.

### 9.3 Portability and scale

The current identity model uses Unix metadata; Windows is outside the documented
scope. There is also no evaluation for checkouts of hundreds of gigabytes or
tens of millions of operations. A cache does not turn a campaign into a source
control system for monorepos.

### 9.4 Contract evolution

Software and description may diverge. The installed executable and
`acvc assistant` must be consulted before operating or explaining the product.

## 10. Conclusion

ACVC preserves a part of exploration that the selected commit does not contain:
each alternative's premise, tree, lineage, measurements, and reason for
rejection. The frontier retains more than one alternative when objectives
conflict, without turning dominance into a decision.

Git remains responsible for the preserved change. ACVC stays beside it as local
experimental memory. The central contribution is not another merge mechanism,
but an explicit object for the hypothesis that precedes the commit.

## References

[1] S. Chacon and B. Straub, *Pro Git*, 2nd ed. Apress, 2014. "Git Internals — Git Objects." https://git-scm.com/book/en/v2/Git-Internals-Git-Objects

[2] L. Torvalds and Git contributors, Git, 2005–. https://git-scm.com/

[3] J. O'Connor, J.-P. Aumasson, S. Neves, and Z. Wilcox-O'Hearn, "BLAKE3: one function, fast everywhere," 2020. https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf

[4] S. Perez De Rosso and D. Jackson, "Purposes, concepts, misfits, and a redesign of Git," in *Proc. OOPSLA 2016*, pp. 292–310. doi: 10.1145/2983990.2984018

[5] M. B. Kery, A. Horvath, and B. A. Myers, "Variolite: Supporting exploratory programming by data scientists," in *Proc. CHI 2017*, pp. 1265–1276. doi: 10.1145/3025453.3025626

[6] A. Rule, A. Tabard, and J. D. Hollan, "Exploration and explanation in computational notebooks," in *Proc. CHI 2018*, paper 32. doi: 10.1145/3173574.3173606

[7] A. Head, F. Hohman, T. Barik, S. M. Drucker, and R. DeLine, "Managing messes in computational notebooks," in *Proc. CHI 2019*, paper 270. doi: 10.1145/3290605.3300500

[8] H. T. Kung, F. Luccio, and F. P. Preparata, "On finding the maxima of a set of vectors," *Journal of the ACM*, vol. 22, no. 4, pp. 469–476, 1975. doi: 10.1145/321906.321910

[9] M. Zaharia et al., "Accelerating the machine learning lifecycle with MLflow," *IEEE Data Engineering Bulletin*, vol. 41, no. 4, pp. 39–45, 2018.

[10] K. Greff, A. Klein, M. Chovanec, F. Hutter, and J. Schmidhuber, "The Sacred infrastructure for computational research," in *Proc. SciPy 2017*, pp. 49–56. doi: 10.25080/shinma-7f4c6e7-008

[11] Dagger contributors, container-use, 2025–. https://github.com/dagger/container-use

[12] R. C. Merkle, "A digital signature based on a conventional encryption function," in *Advances in Cryptology — CRYPTO '87*, LNCS 293, Springer, 1988, pp. 369–378. doi: 10.1007/3-540-48184-2_32

[13] D. R. Hipp, "SQLite FTS5 extension." https://www.sqlite.org/fts5.html

[14] K. R. Popper, *The Logic of Scientific Discovery*. Hutchinson, 1959. English translation of *Logik der Forschung*, 1934.

[15] L. Moreau, P. Missier, et al., "PROV-DM: The PROV data model," W3C Recommendation, 30 April 2013. https://www.w3.org/TR/prov-dm/

[16] V. Pareto, *Manuale di economia politica*. Società Editrice Libraria, 1906.

[17] K. Deb, *Multi-Objective Optimization Using Evolutionary Algorithms*. Wiley, 2001.

[18] S. Apel, D. Batory, C. Kästner, and G. Saake, *Feature-Oriented Software Product Lines*. Springer, 2013.

[19] Iterative, DVC: Data Version Control. https://dvc.org

[20] Weights & Biases. https://wandb.ai

[21] T. Kluyver et al., "Jupyter notebooks — a publishing format for reproducible computational workflows," in *Positioning and Power in Academic Publishing: Players, Agents and Agendas*, IOS Press, 2016, pp. 87–90. doi: 10.3233/978-1-61499-649-1-87

[22] P. Chiusano and R. Bjarnason, Unison. https://www.unison-lang.org

[23] E. Dolstra, "The purely functional software deployment model," Ph.D. dissertation, Utrecht University, 2006.

[24] M. von Zweigbergk and contributors, Jujutsu. https://github.com/jj-vcs/jj

[25] Nick G. and Anthropic's source control team, "Redesigning source control at Anthropic," Anthropic Engineering, 13 October 2026, updated 15 October 2026. https://www.anthropic.com/engineering/redesigning-source-control
