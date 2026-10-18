# ACVC

[English](README.md) · [Português](README.pt-BR.md)

ACVC (Agent Code Variant Control) keeps the hypotheses you tried, the measurements you took, and which attempts are still undominated. Git keeps the variant you decide to record. It does not keep the attempts you abandoned on the way there.

This is for people who compare approaches: researchers, developers, and software engineers. An agent can run the same commands. A person can read the result with `--format text`, `acvc tui`, or `acvc web`.

The design, the problems that motivated it, the methods, and the neighboring systems are written up in [PAPER.md](PAPER.md) ([português](PAPER.pt-BR.md)). That note is a system description. It is not a peer-reviewed article, and it does not report a user study.

## Install

Build the `acvc` binary from this repository:

```bash
cargo build --release -p acvc-cli
install -m 755 target/release/acvc ~/.local/bin/acvc
```

Release archives, when published, are Linux x86_64 and macOS Apple Silicon. Each archive contains the binary and `LICENSE`.

The default output is JSON. Add `--format text` when you want sentences. Human sentences follow the computer locale (`LC_ALL`, then `LC_MESSAGES`, then `LANG`). English is the fallback. These four JSON status values do not change: `na fronteira`, `superada`, `rejeitada`, `sem medição`.

## LLM agents

An agent learns ACVC from the executable, not from this README. At the start of a session, before explaining the tool or calling another verb, it runs `acvc assistant`. That command returns the operational contract in the computer's language. The same contract is the MCP tool `assistant`.

The contract has eight parts: `indice`, `objetivo`, `recursos`, `como-usar`, `boas-praticas`, `exemplos`, `mau-uso`, `trade-offs`. Read them in that order and stop when the question already has an answer.

### CLI

```bash
acvc assistant
acvc --format text assistant
acvc assistant --part como-usar
```

The default is JSON. `lang` is the locale (`en-US` when the language is missing or unknown). `documents` is the contract: each item has `id`, `source`, `text`, and `metadata`. `index` tells an agent that also has the ASB MCP to submit that `documents` array to `rag.index` once. Later search uses `rag.search`. `--part` returns one document. An unknown part is an error (`unknown assistant document`).

Work goes through the same binary. `seed`, `record`, `score`, `frontier`, `why`, `recall`, `compare`, `reject`, and `promote` are the verbs. Do not open `.acvc/graph.sqlite`, the log, or the pack. In JSON, read the field `status`. The four values stay `na fronteira`, `superada`, `rejeitada`, and `sem medição`. The translated sentence is for a person.

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

```bash
acvc init
acvc --format text campaign create integral \
  --metric residual:lower \
  --metric evaluations:lower
```

A metric is `name:lower` or `name:higher`. Lower means a smaller number wins. Higher means a larger number wins. Names are one token: letters, digits, and `_`.

`acvc seed` stores the first tree of that campaign. A campaign has one origin. Later attempts are `acvc record` from that workspace, after you edit the files.

A score names every metric. `acvc score --metric residual=1e-4 --metric evaluations=80` is one trial. A trial that omits a metric stays off the frontier.

The frontier keeps every complete trial that no other hypothesis strictly beats. Two attempts can both stay: one more accurate, the other cheaper. A strictly worse trial drops off the frontier. That drop is not a rejection. You reject only when you decide the attempt is dead:

```bash
acvc --format text reject --variant <id> --reason "the series diverges at the endpoint"
```

If an attempt is off the frontier and you did not reject it, the reading says who won and that the motive was not recorded.

`acvc promote --variant <id> --git <checkout>` copies the chosen tree into an existing Git checkout. That is the deposit. The other attempts remain in `.acvc`.

Do not open `.acvc/graph.sqlite`, the log, or the pack. Ask `acvc frontier`, `acvc why`, `acvc compare`, and `acvc recall`.

## ACVC and container-use

[container-use](https://github.com/dagger/container-use) (Dagger) gives a coding agent an isolated container and a Git branch. ACVC gives a person, or an agent, a record of hypotheses and measurements. One does not replace the other. The choice follows the objective.

container-use creates an environment: a Dagger container, a worktree, and a branch. Commands run in that container. File changes land on that branch. Operation history and container state are stored as Git notes. You inspect with `container-use log`, `diff`, and `checkout`, then accept the work with `merge` (keep the environment commits) or `apply` (stage the diff and write your own commit). A failed environment can be deleted. Several agents can run at once because each one has its own container and branch.

ACVC creates a campaign. The first tree is a seed: a premise, assumptions, an intent, and the files. Later trees are records (`mutate`, `revert`, `revise_premise`, `synthesize`). A score is a trial against the campaign metrics. The frontier keeps every complete trial that no other hypothesis strictly beats. Git receives a tree only when you `promote` the one you chose.

### Comparison

| Question | ACVC | container-use |
| --- | --- | --- |
| What is one attempt? | A variant: premise, assumptions, intent, edge, and a content-addressed tree | An environment: a container plus a Git branch |
| Where do the other attempts live? | In `.acvc`, until you promote one tree into a Git checkout | On environment branches, with commits and Git notes |
| What do you learn from history? | Which complete measurements are still undominated, and the motive of a rejection | Which commands ran, and the diff of that branch |
| How do you discard? | `reject` with a reason, or leave the attempt off the frontier | Delete the environment |
| How does the chosen work enter your branch? | `promote --git` copies that tree into an existing checkout | `merge` keeps the environment commits, or `apply` stages the diff |
| What runs the code? | You do. ACVC stores the tree and the numbers you report | The agent runs commands inside the container |
| What isolates the run? | Nothing. The workspace is a directory you already have | The container and the worktree |
| Who it fits | A researcher or an engineer comparing measured approaches, with or without an agent | Agents working in parallel who must not touch your checkout |

### Tradeoffs, by objective

Measure competing approaches. Use ACVC. A stitching pipeline, an integral rule, or a store for restaurant tickets is a campaign: same metrics, several premises, a frontier that can hold more than one winner. container-use will keep each prototype on a branch and show the commands. It will not say which complete trial is still undominated, and it will not store the premise next to the score.

Let agents work in parallel without colliding. Use container-use. Each agent gets a container and a branch, you watch the log, and you merge or delete. ACVC will not start that container, will not hide the run from your working tree, and will not record the shell history.

Keep Git for the decision. Use ACVC when the abandoned attempts should stay out of the branch history. Use container-use when you want those attempts as branches you can check out, and you are willing to have the agent's commits until you merge or apply.

Need both. An agent can prototype inside a container-use environment. The hypothesis you want to remember — premise, metrics, and whether it still stands — is an ACVC variant of the tree you keep. The container explains how the run was isolated. The campaign explains what the run was claiming and how it measured.

## License

Use, study, and modification for research are allowed when the result is not a commercial product. Commercial use is not granted. That includes freeware, freemium, and research whose purpose is to produce commercial software. Those uses need written authorization from Samuraisatoshi. See `LICENSE`.

The third case below is a commercial SaaS question. Running it as a real product requires that written authorization. The commands themselves are the same.

## Case A. A mathematical problem

You are integrating a smooth function on a closed interval. You care about the residual and about how many evaluations the rule spends. A smaller residual is better. Fewer evaluations are better.

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

Simpson beats the trapezoid rule on both metrics, so the trapezoid trial leaves the frontier. Simpson and the series both stay: Simpson spends fewer evaluations, the series has the smaller residual. Neither one is strictly better.

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

You are improving a stitching pipeline. Three numbers matter: the seam error, the runtime, and how much of the scene is covered. Lower seam and runtime are better. Higher coverage is better.

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

All three can stay on the frontier. Homography is the fastest and the least accurate. The mesh covers more and costs time. The blend has the cleanest seam, the same coverage as the mesh, and a higher runtime, so it does not erase the mesh: the mesh is still faster. Read that before you pick.

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

You are choosing the store behind ticket sales. The shared surface is the HTTP API and the tests. The two stores are later attempts, not two origins: a campaign has one seed.

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

The frontier holding both is the point. The choice is yours. If a lost ticket at the pass is unacceptable, say so. The rejection is the motive, not a side effect of the score:

```bash
acvc --format text reject --variant <document> \
  --reason "a lost ticket during a partition is not acceptable at the pass"
acvc --format text promote --variant <postgres> --git .
```

## Read the campaign

```bash
acvc --format text frontier
acvc --format text context
acvc tui
acvc web
```

`acvc web` listens on `127.0.0.1:9420`. `acvc guide` is the short map of the open campaign. `acvc assistant` returns the operational contract for an agent, in the computer's language.
