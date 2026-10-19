# ACVC: keeping hypotheses by hash, and decisions in Git

Samuraisatoshi  
2026  
Correspondence: samuraisatoshi@cryptoworld.8shield.net

This is a system description of the software in this repository. It is not a peer-reviewed article. It reports no user study and no benchmark. The implementation is the source of the claims about behavior.

## If you have not seen ACVC

An agent tries many versions of the same change. Putting every failed try in Git fills the repository, and the next agent gets lost in that history. ACVC records each attempt: the idea, the files, and the measurement. You see which attempts still compete. Git receives one tree, the one you approve, through `acvc promote` into a checkout that already exists. ACVC does not merge and does not replace Git.

The rest of this note uses the precise names. A hypothesis is one attempt. A trial is one measurement. The frontier is the attempts nobody has beaten on every number. Two attempts can both stay.

## Abstract

ACVC (Agent Code Variant Control) is a local store for hypotheses about a body of code. A person or an agent may try many variants. Each variant has a premise, assumptions, an intent, a relation to its parents, and a tree of files. The tree is a Merkle tree: a node holds the hash of a child, not the child's bytes. A file that did not change is not stored again. Git remains the place where a person deposits the variant they chose to keep. The campaign names the metrics. A trial is one complete measurement. The frontier keeps every complete trial that no other hypothesis strictly beats. A loss on the frontier is not a rejection. A rejection is a separate act, and it carries a motive.

The design takes the content-addressed object store of Git [1, 2] and aims it at a different purpose. Git's hash controls a snapshot that someone decided to record. ACVC's hash controls a hypothesis that may never be that decision.

## 1. What this work is

A campaign is a named question with a fixed set of metrics. Each metric points lower or higher. The first tree in a campaign is a seed. Later trees are records. The edge says what kind of step it was: `mutate` changes the tree under the same premise family, `revert` returns to an earlier idea, `revise_premise` replaces an assumption that failed, and `synthesize` has two or more parents. The identity of a variant is a BLAKE3 hash [3] over a canonical encoding of the tree hash, the parents, the edge, the claim, the assumptions, the intent, and the agent. The campaign name, the clock, the measurements, and any token spend are outside that identity. Scoring a variant does not create a new variant.

The store is a directory `.acvc` beside the work: an append-only log of blobs and trees, a pack of blobs that already settled, and a SQLite index with full-text search over premises. Readers use the `acvc` executable. They do not open the database.

When a variant is the one that should enter history, `promote` copies that tree into an existing Git checkout. The other variants stay in `.acvc`.

## 2. Motivation and the problems that inspired the work

Three problems meet.

The first is what Git is for. Git is a content-addressable filesystem with a commit graph on top [1, 2]. A commit is a recorded decision: this tree, these parents, this message, this person. Perez De Rosso and Jackson showed that several of Git's concepts miss the purposes users actually bring to it [4]. Exploratory programming adds a purpose Git was not built for. Data scientists keep old analyses alive by copying files, commenting code out, and holding unused versions in the same buffer, because a commit is too heavy and too final for a thought that is still moving [5]. Notebooks accumulate that mess; the history of the exploration and the story one would publish are different objects [6, 7]. An agent that tries dozens of hypotheses in a session makes the misfit sharper. The session ends. The commits that were made are the few trees someone agreed to keep. The premises that were abandoned are gone, or they are buried in a branch nobody can score.

The second problem is measurement. Two stitching methods, two integration rules, or two stores behind a restaurant API are not ordered by a single number. One may be more accurate and slower. Multi-objective comparison keeps the undominated set: the maxima of a set of vectors [8]. Experiment trackers such as MLflow and Sacred record runs, parameters, and metrics for computational experiments [9, 10]. They assume the run is the object. Here the object is the hypothesis, and a run is a trial attached to it. A trial that omits a metric is incomplete and stays off the frontier. Metrics are not mixed across trials to invent a best score.

The third problem is motive. Dominance is not a decision. A hypothesis can leave the frontier because another complete trial strictly beats it, while nobody has said why the loser was abandoned. ACVC keeps that gap visible. The status says who won and that the motive was not recorded. A rejection is a human sentence, stored once for that variant. Agents forget sessions. A store that only keeps winners forgets the reason a loser lost.

container-use, from the Dagger project, answers a neighboring problem: an agent needs an isolated container and a Git branch so parallel work does not collide with the checkout [11]. That is an execution environment. It does not name a premise, it does not score a campaign, and it does not keep an undominated frontier. ACVC does not start a container. The two tools answer different questions. Section 6 lists the other neighbors.

## 3. Methods

### 3.1 Control by hash, after Git

Git does not store a fresh copy of the project at every commit. It stores objects addressed by the hash of their contents. A blob is the bytes of a file. A tree is a list of names, modes, and the hashes of blobs or of other trees. A commit stores the hash of a tree, not the files. If a file is unchanged, the new tree repeats the old blob hash. Equal prefixes of a directory are shared. Comparison and storage follow the hash. The file bytes are read when the hash is missing, not because a new version was declared [1, 2]. Merkle's construction is the older form of the same idea: a parent authenticates children by hashing their hashes, so an unchanged subtree is a single value [12].

ACVC uses that control structure for hypotheses.

A file enters the store as a blob, tagged and hashed with BLAKE3 [3]. A directory is a tree object: names in order, each with a mode, a kind, and a hash. The tree object does not contain the file. It contains the hash that already names the file. The variant does not contain the tree. It contains the tree hash, plus the premise and the parent hashes. Identity is the hash of that canonical record.

Recording a change walks the workspace and compares each path with the index of the parent tree. A clean path contributes the hash it already has. Its bytes are not read and its blob is not written again. A dirty or new path is the only one hashed into a new blob. An equal subtree is not loaded when two variants are diffed. The test of the design is the same test Git's object store satisfies: a sibling that did not change is not part of the write.

This is the opposite of copying the project into a folder per idea, and the opposite of putting the whole file inside the hypothesis record. The hash is the control. The bytes exist once, under that hash.

Two consequences follow. First, a campaign can hold many variants of a large tree without copying the unchanged files. Second, the premise can change while the tree hash stays, or the tree can change while the wording of the premise stays. Those are different hashes, because both the tree and the claim are inputs to the variant hash. A measurement is not an input. The same variant can carry several trials. The trial has its own hash (the variant hash, a nonce, and the metric map). Scoring does not fork the code.

Git's historical hash was SHA-1, later joined by SHA-256 in the Git project. ACVC uses BLAKE3 for every object it creates. The borrowed idea is the object model, not the choice of function.

### 3.2 The hypothesis

A seed has no parents. It is the origin of the campaign, and there is one origin. A mutation has the workspace's current variant as parent unless the caller names another. Revising a premise records which assumption was replaced; if the caller does not name it, the parent is the revised claim. Synthesis names at least two parents. The edge is part of the hash, so a mutation and a premise revision of the same tree are different variants.

The claim, the assumptions, and the intent are strings in the identity. They are not comments beside a commit. Recall searches those strings. The full-text index is SQLite FTS5 [13].

### 3.3 Campaign, trials, and the frontier

The campaign is a schema: metric names and a direction, lower or higher. A trial is complete when it carries every name. The frontier is the set of complete trials that are not strictly dominated. Domination uses the classical maxima of a set of vectors [8]: one trial dominates another when it is no worse on every metric and better on at least one. Two trials that trade accuracy for cost both stay. Within one variant, a strictly worse complete trial is dropped. Across the campaign, every remaining undominated complete trial is reported. The frontier does not pick a single winner and does not fill a missing metric from a different trial.

Spend, a side ledger of tokens, is append-only and is not a metric. It does not enter the variant hash and it does not move the frontier.

### 3.4 Rejection

Dominance writes no rejection. `reject` stores one motive for a variant. Until that motive exists, a variant that fell off the frontier is described as beaten, with the winner and the metric, and the text says the motive was not recorded. The JSON status values are fixed codes (`on_frontier`, `superseded`, `rejected`, `unmeasured`). Prose follows the locale of the computer.

The explicit premise revision is inspired by the habit, in the philosophy of science, of treating a failed assumption as a change of hypothesis rather than as a silent edit [14]. The software does not test falsifiability. It only refuses to let that change hide inside an ordinary mutation: the edge is different, and the hash follows.

### 3.5 What is not hashed

The clock, the campaign name, the pack layout, and the path of the workspace are not part of variant identity. Moving a campaign from one directory to another does not rename the variant. Two campaigns can hold the same variant hash only by holding the same hypothesis; the campaign is a label in the index, not a field in the hash.

### 3.6 The seam with Git

`promote` materializes the chosen tree into a Git checkout the caller already has. ACVC does not commit, does not rewrite history, and does not replace the remote. Git still means what it meant: the record of what a person chose to keep [1]. The search stays in the hash store until that choice.

## 4. Literature that supports the design

The object model is Git's, as documented by Chacon and Straub [1] and as built by Torvalds in 2005 [2]: blobs, trees, and hashes as the names of objects. Merkle gives the reason a tree of hashes detects a changed leaf without storing the leaf in the parent [12]. BLAKE3 is the concrete hash [3].

The critique of using a commit as the unit of exploration is empirical and conceptual. Variolite studied data scientists versioning snippets by copy and comment because conventional version control fights a short-lived alternative [5]. Rule, Tabard, and Hollan separated exploration from explanation in notebooks [6]. Head, Hohman, Barik, Drucker, and DeLine showed how notebook history becomes a mess the author must manage by hand [7]. Perez De Rosso and Jackson argued that Git's difficulty is partly a misfit between concepts and purposes, and they redesigned the concepts in Gitless [4]. ACVC does not redesign Git. It refuses to put the search into Git's concepts.

Provenance models, in particular W3C PROV [15], describe entities, activities, and agents in general. ACVC is a narrow instance: a code tree, a hypothesis, a trial, and an agent string. It does not claim to be a provenance interchange format.

Pareto's manual is the economic source of the undominated frontier [16]. The algorithm ACVC relies on is the maxima of a set of vectors [8]. Deb's synthesis of multi-objective optimization is the modern textbook frame [17]. Experiment-tracking systems showed that runs should be queryable [9, 10]. ACVC attaches the queryable run to a content-addressed hypothesis instead of to a parameter file in a database row.

Popper's account of conjecture and refutation is an inspiration for `revise_premise` only [14]. Feature-oriented product lines also speak of variants [18]. Those variants are features of a product family. An ACVC variant is a hypothesis under test. The words collide; the objects do not.

## 5. What makes the project interesting

The interesting move is the split of labor with Git, using Git's own trick. Git showed that a version can be the hash of a tree, and that a tree can be hashes of files, so an unchanged file is not written again. ACVC keeps that economy and changes the noun. The hash names a hypothesis. The file bytes stay in the blob store, shared across hypotheses. Git receives a tree only as a deposit.

Four consequences are unusual together.

The frontier may contain more than one live hypothesis. A faster stitch and a cleaner stitch both remain if neither strictly dominates the other.

A loss is not a death. The system can say who won without inventing a motive. The motive appears only when a person writes it.

An agent can emit many variants in one session without a commit per guess. The session can be reconstructed from premises and trials after the conversation is gone.

The same store serves a human researcher comparing integration rules and an engineer comparing a SQL store with a document store. The commands do not change. The metrics do.

## 6. Equivalent and neighboring projects

None of these is a substitute. Each one owns a purpose. ACVC owns the scored hypothesis and the shared blobs. Where a project already stores content by hash, the difference is what the hash is allowed to mean.

| Project | What it keeps | Where it stops, relative to ACVC |
| --- | --- | --- |
| Git [1, 2] | Content-addressed blobs, trees, and commits | The commit is a decision. Abandoned hypotheses are not a frontier. |
| Gitless [4] | A simpler conceptual layer over Git | It repairs Git's concepts. It does not add premises or trials. |
| container-use [11] | A container, a worktree, and a branch per agent, plus command history in Git notes | It isolates execution. It does not score hypotheses or share unchanged blobs across a campaign. |
| Variolite [5] | Lightweight local variants inside the editor | Variants are snippets for exploration. They are not a Merkle campaign with a frontier. |
| DVC [19] | Data, models, and pipelines, with hashes, on top of Git | The pipeline run is the object. The hypothesis edge and the undominated set of code variants are not. |
| MLflow [9] | Parameters, metrics, and artifacts of runs | The run is a row. The code tree is not a Merkle hypothesis, and dominance is not the status. |
| Sacred [10] | Configuration, source, and results of computational experiments | Bookkeeping for a run. No premise revision edge, no shared blob tree across hypotheses. |
| Weights & Biases [20] | Hosted experiment tracking | Same family as MLflow: runs and charts, not a local hash of a hypothesis. |
| Jupyter [21] | A narrative notebook and its outputs | History is the notebook. Equal subtrees are not hash-shared across alternative analyses. |
| Unison [22] | A content-addressed programming language | Code is stored by hash. The hash names a definition, not a measured hypothesis in a campaign. |
| Nix [23] | Reproducible builds addressed by the hash of a derivation | The hash controls a build. It does not record which premise is still undominated. |
| Jujutsu [24] | A Git-compatible version control system with a different working model | Still a history of decisions. |
| Software product lines [18] | Feature variants of a product | A feature is not a hypothesis under a metric schema. |

## 7. Limits

There is no study of whether researchers or agents record premises honestly. The frontier is only as good as the trials. A complete trial can still be the wrong metric. ACVC does not run the code and does not isolate it; container-use or an ordinary shell does that. Windows is outside the current file-identity model, which uses Unix metadata. The paper and the software can drift; the executable is the authority for behavior.

## 8. Conclusion

ACVC is a campaign of hypotheses stored the way Git stores trees: by hash, with file bytes written only when the content is new. The hash controls the hypothesis. Git controls the decision to keep one tree. The frontier shows which complete measurements are still undominated, and it refuses to treat a loss as a motive that nobody wrote.

## References

[1] S. Chacon and B. Straub, *Pro Git*, 2nd ed. Apress, 2014. "Git Internals — Git Objects." https://git-scm.com/book/en/v2/Git-Internals-Git-Objects

[2] L. Torvalds and the Git contributors, Git, 2005–. https://git-scm.com/ The 2005 design is a content-addressable object store: blobs and trees named by hash, commits naming a tree hash.

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
