# Examples

## Open a campaign

```text
acvc init --repo .
acvc campaign create search --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign search --path ./hypothesis --premise "the cache fits on disk" --assumption "the disk is local" --intent "fewer reads" --agent researcher
```

The seed JSON brings the variant, the tree, how many files entered, and which weighed more.

## The next hypothesis

```text
acvc record --repo . --workspace ./hypothesis --parent PREFIX --premise "the index is a tree" --assumption "the keys are sorted" --intent "a shorter search" --agent researcher --edge mutate
acvc score --repo . --variant PREFIX --metric tests_passed=16 --metric latency_ms=4
```

## See what still competes

```text
acvc context --repo . --campaign search
acvc frontier --repo . --campaign search
acvc why --repo . PREFIX
acvc recall --repo . --campaign search --query "tree"
acvc compare --repo . PREFIX_A PREFIX_B
```

A `why` of a beaten hypothesis, without `reject`, names who won, the metric, and says the motive was not recorded.

## Two measurements of the same hypothesis

```text
acvc score --repo . --variant PREFIX --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PREFIX --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PREFIX --metric tests_passed=0 --metric latency_ms=20
```

The first two are incomparable and stay on the frontier. The third is worse on both metrics, leaves the frontier, and remains in `why`.

## Close on purpose

```text
acvc reject --repo . --variant PREFIX --reason "cache invalidation breaks under concurrent writes"
acvc spend --repo . --campaign search --tokens 15000 --note "hypothesis session" --agent researcher
```

## Keep junk out of the tree

In the seeded folder, `.acvcignore`:

```text
# report
lab/reports
```

Two clones with the same file leave the same paths out. Deleting the file lets `lab/reports` return on the next scan. Hypotheses already recorded are not rewritten.
