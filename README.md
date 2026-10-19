# ACVC

[English](README.md) · [Português](README.pt-BR.md)

ACVC (Agent Code Variant Control) is where a coding agent keeps the attempts that should not become commits.

An agent tries many versions of the same change. Putting every failed try in Git fills the repository, and the next agent gets lost in that history. ACVC records each attempt as a hypothesis: the idea, the files, and the measurement. You see which attempts still compete. Git receives one tree, the one you approve.

`acvc promote` copies that approved tree into a Git checkout that already exists. The other attempts stay in `.acvc`. ACVC does not merge branches and does not replace Git history.

A person uses the same record. Researchers, developers, and software engineers compare approaches with the same commands. Read them with `--format text`, `acvc tui`, or `acvc web`. An agent calls the CLI or the MCP server `acvc mcp`.

The longer description of the design is in [PAPER.md](PAPER.md) ([português](PAPER.pt-BR.md)). That note is a system description. It is not a peer-reviewed article, and it does not report a user study.

## The words, once

- An **attempt** is one version of the files, plus the sentence that says what you believe. The commands call that a hypothesis.
- A **campaign** is one question, and the numbers you will use to compare attempts.
- A **measurement** is those numbers written down for one attempt. The commands call that a trial, or a score.
- The **frontier** is the attempts nobody has beaten on every number. Two attempts can both stay.
- **Promote** copies the attempt you approve into a Git checkout that already exists. The other attempts stay out of Git.

## Install

Build the `acvc` binary from this repository:

```bash
cargo build --release -p acvc-cli
install -m 755 target/release/acvc ~/.local/bin/acvc
```

Release archives, when published, are Linux x86_64 and macOS Apple Silicon. Each archive contains the binary and `LICENSE`.

The default output is JSON. Add `--format text` when you want sentences. Human sentences follow the computer locale (`LC_ALL`, then `LC_MESSAGES`, then `LANG`). English is the fallback. These four JSON status values do not change with language: `on_frontier`, `superseded`, `rejected`, `unmeasured`.

## LLM agents

A person reads this page. An agent that will use the tool runs `acvc assistant` before it explains ACVC or calls another command. The answer comes back in the language of the computer. If you register `acvc mcp`, that same answer is the tool `assistant`.

The answer has eight parts: `indice`, `objetivo`, `recursos`, `como-usar`, `boas-praticas`, `exemplos`, `mau-uso`, `trade-offs`. Read them in that order and stop when the question already has an answer.

### CLI

```bash
acvc assistant
acvc --format text assistant
acvc assistant --part como-usar
```

The default is JSON. `lang` is the locale (`en-US` when the language is missing or unknown). `documents` is the contract: each item has `id`, `source`, `text`, and `metadata`. `index` tells an agent that also has the ASB MCP to submit that `documents` array to `rag.index` once. Later search uses `rag.search`. `--part` returns one document. An unknown part is an error (`unknown assistant document`).

Work goes through the same binary. `seed`, `record`, `score`, `frontier`, `why`, `recall`, `compare`, `reject`, and `promote` are the verbs. Do not open `.acvc/graph.sqlite`, the log, or the pack. In JSON, read the field `status`. The four values stay `on_frontier`, `superseded`, `rejected`, and `unmeasured`. The translated sentence is for a person.

### MCP

`acvc mcp` speaks MCP on stdio. It does not open the store. Each tool starts the `acvc` binary again and returns that process's output.

```json
{
  "mcpServers": {
    "acvc": {
      "command": "acvc",
      "args": ["mcp"]
    }
  }
}
```

Call the tool `assistant` first, with an optional argument `part`. The other tools are the CLI verbs. Pass `repo` when the campaign is not in the server's current directory. A page, a TUI, or an editor that wants the data calls this executable. It does not reimplement the graph.

## One campaign

A campaign is one question. You name the numbers before the first attempt. `lower` means a smaller number is better. `higher` means a larger number is better.

```bash
acvc init
acvc --format text campaign create integral \
  --metric residual:lower \
  --metric evaluations:lower
```

A name is one token: letters, digits, and `_`.

`seed` saves the first version of the files and the sentence that says what you believe. A campaign has one first version. After you edit the files, `record` saves the next version and points at the one you edited.

`score` is you writing the numbers. You fill in every number you named. If one is missing, that measurement does not compete.

The frontier is who still competes. An attempt stays when no other attempt is better on every number. Two can both stay: one more accurate, the other cheaper. An attempt that loses on every number drops off that list. Dropping off is not a decision. You `reject` only when you have decided the attempt is finished, and you write the reason:

```bash
acvc --format text reject --variant <id> --reason "the series diverges at the endpoint"
```

If an attempt dropped off and you did not reject it, the reading names who won and says the reason was not written down.

`promote` copies the attempt you approve into a Git checkout that already exists. That is the commit you were keeping the experiments out of. The other attempts stay in `.acvc`.

```bash
acvc --format text promote --variant <id> --git <checkout>
```

Ask `frontier` (who still competes), `why` (one attempt), `compare` (two attempts), and `recall` (find an attempt by the sentence you wrote). Do not open `.acvc/graph.sqlite`, the log, or the pack.

## ACVC and container-use

[container-use](https://github.com/dagger/container-use) (Dagger) gives a coding agent a sandbox and a Git branch, so the experiment does not touch your checkout. ACVC remembers the attempts, the numbers, and which one you approved for Git. One does not replace the other. Pick by what you need.

container-use creates an environment: a Dagger container, a worktree, and a branch. Commands run in that container. File changes land on that branch. Operation history and container state are stored as Git notes. You inspect with `container-use log`, `diff`, and `checkout`, then accept the work with `merge` (keep the environment commits) or `apply` (stage the diff and write your own commit). A failed environment can be deleted. Several agents can run at once because each one has its own container and branch.

ACVC starts with one question and one first version of the files (`seed`): what you believe, what you are assuming, what you hope to get, and the files. Later versions are `record`. The edge says what kind of step it was: `mutate` changes the files, `revert` goes back to an earlier idea, `revise_premise` changes the belief itself, `synthesize` combines two or more attempts. You then write the numbers. Git receives files only when you `promote` the attempt you approve.

### Comparison

| Question | ACVC | container-use |
| --- | --- | --- |
| What is one attempt? | One version of the files, the sentence that says what you believe, and how it relates to the previous version | An environment: a container plus a Git branch |
| Where do the other attempts live? | In `.acvc`, until you copy the one you approve into a Git checkout | On environment branches, with commits and Git notes |
| What do you learn from history? | Which measurements still compete, and the reason you wrote when you decided an attempt was finished | Which commands ran, and the diff of that branch |
| How do you discard? | `reject` with a reason, or leave the attempt off the list of what still competes | Delete the environment |
| How does the chosen work enter your branch? | `promote --git` copies those files into a checkout that already exists | `merge` keeps the environment commits, or `apply` stages the diff |
| What runs the code? | You do. ACVC stores the tree and the numbers you report | The agent runs commands inside the container |
| What isolates the run? | Nothing. The workspace is a directory you already have | The container and the worktree |
| Who it fits | A researcher or an engineer comparing measured approaches, with or without an agent | Agents working in parallel who must not touch your checkout |

### Tradeoffs, by objective

You want to compare approaches by numbers. Use ACVC. A stitching pipeline, an integral rule, or a store for restaurant tickets is one question: the same numbers, several beliefs, and a list that can keep more than one attempt because each wins on a different number. container-use will keep each prototype on a branch and show the commands. It will not say which attempts still compete, and it will not store the belief next to the numbers.

Let agents work in parallel without colliding. Use container-use. Each agent gets a container and a branch, you watch the log, and you merge or delete. ACVC will not start that container, will not hide the run from your working tree, and will not record the shell history.

Keep Git for the decision. Use ACVC when the abandoned attempts should stay out of the branch history. Use container-use when you want those attempts as branches you can check out, and you are willing to have the agent's commits until you merge or apply.

Need both. An agent can try things inside a container-use environment. The attempt you want to remember — the belief, the numbers, and whether it still competes — is an ACVC record of the files you keep. The container explains how the run was isolated. ACVC explains what the run was claiming and how it measured.

## License

Use, study, and modification for research are allowed when the result is not a commercial product. Commercial use is not granted. That includes freeware, freemium, and research whose purpose is to produce commercial software. Those uses need written authorization from Samuraisatoshi. See `LICENSE`.

The third case below is a commercial SaaS question. Running it as a real product requires that written authorization. The commands themselves are the same.

## Case A. A mathematical problem

You are integrating a smooth function on a closed interval. Two numbers matter: how far the answer is from the true value, and how many times the rule evaluates the function. Smaller is better for both. Each rule is one attempt. Git receives only the rule you approve.

```bash
mkdir -p integral
printf 'trapezoid\n' > integral/rule.txt
acvc init
acvc --format text campaign create integral \
  --metric residual:lower \
  --metric evaluations:lower
acvc --format text seed \
  --campaign integral \
  --path integral \
  --premise "the trapezoid rule is enough for a smooth integrand" \
  --assumption "the integrand is smooth on a closed interval" \
  --intent "a residual under 1e-6" \
  --agent researcher
```

The command prints a variant id. Call it `<trapezoid>`. Edit `integral/rule.txt` to the Simpson implementation, then record that attempt. Record uses the workspace's current variant as the parent.

```bash
printf 'adaptive-simpson\n' > integral/rule.txt
acvc --format text record \
  --workspace integral \
  --edge mutate \
  --premise "adaptive Simpson spends fewer evaluations on a smooth integrand" \
  --assumption "the integrand is smooth on a closed interval" \
  --intent "a residual under 1e-8" \
  --agent researcher
```

Call that id `<simpson>`. Edit again and record a series expansion. Call it `<series>`.

Score each attempt with both metrics. These numbers are the story, not a real run:

```bash
acvc --format text score --variant <trapezoid> --metric residual=1.4e-3 --metric evaluations=400
acvc --format text score --variant <simpson> --metric residual=2.1e-8 --metric evaluations=80
acvc --format text score --variant <series> --metric residual=4.0e-10 --metric evaluations=360
```

Simpson beats the trapezoid rule on both numbers, so the trapezoid attempt drops off the list of what still competes. Simpson and the series both stay: Simpson uses fewer evaluations, the series has the smaller error. Neither one wins on both numbers.

```bash
acvc --format text frontier --campaign integral
acvc --format text why <trapezoid>
acvc --format text compare <simpson> <series>
acvc --format text recall --campaign integral --query "Simpson"
```

`why` on the trapezoid attempt names the winner and the metric, and says the motive was not recorded. Reject it only if you have decided the rule itself is finished:

```bash
acvc --format text reject --variant <trapezoid> \
  --reason "Simpson dominates it on a smooth interval"
```

When the integrand is singular, the premise changes. That is not a small edit of the same idea:

```bash
printf 'substitution-then-simpson\n' > integral/rule.txt
acvc --format text record \
  --workspace integral \
  --edge revise_premise \
  --premise "a singularity at the endpoint needs a substitution before Simpson" \
  --assumption "the integrand blows up at one endpoint" \
  --intent "a residual under 1e-6 on the singular integral" \
  --agent researcher
```

Score that new attempt on its own. Do not copy the smooth-interval numbers onto it. Promote the attempt you trust into Git:

```bash
acvc --format text promote --variant <series> --git .
```

## Case B. Image stitching

You are joining photos into one picture. Three numbers matter: the seam error, the time, and how much of the scene is covered. Lower seam and time are better. Higher coverage is better. Each method is one attempt. You can keep every method that is best at something, and copy only the one you choose into Git.

```bash
mkdir -p stitch
printf 'global-homography\n' > stitch/method.txt
acvc init
acvc --format text campaign create stitch \
  --metric seam:lower \
  --metric runtime_ms:lower \
  --metric coverage:higher
acvc --format text seed \
  --campaign stitch \
  --path stitch \
  --premise "one homography is enough when the scene is nearly planar" \
  --assumption "the camera motion is a single plane" \
  --intent "cover the overlap without a visible seam" \
  --agent researcher
```

Replace the method, record, and score. The parent is the workspace you just seeded.

```bash
printf 'local-mesh\n' > stitch/method.txt
acvc --format text record \
  --workspace stitch \
  --edge mutate \
  --premise "a local mesh absorbs parallax that one homography cannot" \
  --assumption "the scene has mild parallax" \
  --intent "raise coverage without tearing the overlap" \
  --agent researcher

printf 'seam-aware-blend\n' > stitch/method.txt
acvc --format text record \
  --workspace stitch \
  --edge mutate \
  --premise "a seam-aware blend hides the residual mesh error" \
  --assumption "the mesh already aligns the overlap" \
  --intent "cut the seam error without giving up coverage" \
  --agent researcher
```

Call the three ids `<homography>`, `<mesh>`, and `<blend>`.

```bash
acvc --format text score --variant <homography> --metric seam=0.42 --metric runtime_ms=80 --metric coverage=0.71
acvc --format text score --variant <mesh> --metric seam=0.18 --metric runtime_ms=640 --metric coverage=0.93
acvc --format text score --variant <blend> --metric seam=0.07 --metric runtime_ms=710 --metric coverage=0.93
```

All three can stay on the list of what still competes. Homography is the fastest and the least accurate. The mesh covers more and costs time. The blend has the cleanest seam, the same coverage as the mesh, and a higher runtime, so it does not erase the mesh: the mesh is still faster. Read that before you pick which files go to Git.

```bash
acvc --format text frontier --campaign stitch
acvc --format text compare <mesh> <blend>
acvc --format text why <homography>
acvc --format text file-diff <homography> <blend> --path method.txt
```

If a mesh experiment tore the overlap, record the return with `--edge revert` and a premise that says what failed. Promote the blend when that is the tree you want in Git:

```bash
acvc --format text promote --variant <blend> --git .
```

## Case C. SQL or a document store for a restaurant SaaS

You are choosing the database behind ticket sales. The HTTP API and the tests stay the same. SQL and a document store are two later attempts at that one question. A campaign has one first version, not two.

This comparison is about a commercial product. Using ACVC to build that product needs written authorization from the author. See `LICENSE`.

The metrics are the tail latency, lost writes during a partition, and the monthly bill. Lower is better on all three.

```bash
mkdir -p orders
printf 'api-and-tests\n' > orders/store.txt
acvc init
acvc --format text campaign create orders \
  --metric p95_ms:lower \
  --metric lost_writes:lower \
  --metric monthly_usd:lower
acvc --format text seed \
  --campaign orders \
  --path orders \
  --premise "the ticket is the unit of sale and the store is still undecided" \
  --assumption "one venue, one open ticket per table" \
  --intent "choose a store that does not lose a paid ticket" \
  --agent researcher
```

```bash
printf 'postgresql\n' > orders/store.txt
acvc --format text record \
  --workspace orders \
  --edge mutate \
  --premise "one PostgreSQL orders table keeps the ticket durable" \
  --assumption "a single primary is enough for one region" \
  --intent "zero lost writes under a partition" \
  --agent researcher

printf 'document\n' > orders/store.txt
acvc --format text record \
  --workspace orders \
  --edge mutate \
  --premise "one document per ticket avoids a join on the hot path" \
  --assumption "a lost write can be retried by the client" \
  --intent "a cheaper bill and a lower p95" \
  --agent researcher
```

The document record's parent is the PostgreSQL workspace, because record follows the workspace marker. If you need both attempts to branch from the API seed, materialize the seed into a second directory before the second record:

```bash
acvc --format text materialize --variant <api> --into orders-doc
# edit orders-doc/store.txt, then:
acvc --format text record \
  --workspace orders-doc \
  --parent <api> \
  --edge mutate \
  --premise "one document per ticket avoids a join on the hot path" \
  --assumption "a lost write can be retried by the client" \
  --intent "a cheaper bill and a lower p95" \
  --agent researcher
```

Score both stores. Neither number set dominates the other: PostgreSQL loses no writes and costs more; the document store is faster and cheaper and loses writes.

```bash
acvc --format text score --variant <postgres> --metric p95_ms=48 --metric lost_writes=0 --metric monthly_usd=220
acvc --format text score --variant <document> --metric p95_ms=19 --metric lost_writes=3 --metric monthly_usd=90
acvc --format text frontier --campaign orders
acvc --format text compare <postgres> <document>
```

Both stores can stay worth considering, because each wins on a different number. The choice is yours. If a lost ticket at the pass is unacceptable, write that down. That sentence is the decision. The numbers alone do not discard the attempt:

```bash
acvc --format text reject --variant <document> \
  --reason "a lost ticket during a partition is not acceptable at the pass"
acvc --format text promote --variant <postgres> --git .
```

## Read the campaign

These commands answer a person who was not in the session. `frontier` lists the attempts that still compete. `context` is the map of the question: who tried what, and how the attempts relate. `tui` is the same map in the terminal. `web` is the same map in a browser.

```bash
acvc --format text frontier
acvc --format text context
acvc tui
acvc web
```

`acvc web` listens on `127.0.0.1:9420`. `acvc guide` is a short map of the open question and the commands. `acvc assistant` is the explanation an agent should read, in the language of the computer.
