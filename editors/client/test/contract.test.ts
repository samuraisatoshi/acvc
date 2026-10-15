import assert from "node:assert/strict";
import { execFileSync } from "node:child_process";
import { mkdirSync, mkdtempSync, rmSync, writeFileSync } from "node:fs";
import { tmpdir } from "node:os";
import path from "node:path";
import { test } from "node:test";
import { Acvc } from "../src/api.ts";

const root = path.resolve(import.meta.dirname, "../../..");

function binary(): string {
  if (process.env.ACVC_BIN) return process.env.ACVC_BIN;
  execFileSync("cargo", ["build", "-q", "--bin", "acvc"], { cwd: root, stdio: "inherit" });
  const meta = JSON.parse(
    execFileSync("cargo", ["metadata", "--format-version", "1", "--no-deps"], {
      cwd: root,
      encoding: "utf8",
    }),
  ) as { target_directory: string };
  return path.join(meta.target_directory, "debug", "acvc");
}

test("a missing executable is reported", async () => {
  const acvc = new Acvc({ binary: path.join(tmpdir(), "missing-acvc"), cwd: tmpdir() });
  await assert.rejects(() => acvc.campaigns());
});

test("the json cli serves frontier, lineage, diff, and recall", async () => {
  const repo = mkdtempSync(path.join(tmpdir(), "acvc-plugin-"));
  try {
    const acvc = new Acvc({ binary: binary(), cwd: repo });
    await acvc.init();
    await acvc.createCampaign("speed", ["tests_passed:higher", "latency_ms:lower"]);
    const source = path.join(repo, "seed-src");
    mkdirSync(path.join(source, "a"), { recursive: true });
    mkdirSync(path.join(source, "b"), { recursive: true });
    writeFileSync(path.join(source, "a", "one.txt"), "one\n");
    writeFileSync(path.join(source, "b", "two.txt"), "two\n");
    const seeded = await acvc.seed({
      campaign: "speed",
      path: source,
      premise: "baseline",
      agent: "test",
    });
    const workspace = path.join(repo, "work");
    await acvc.materialize(seeded.variant, workspace);
    writeFileSync(path.join(workspace, "a", "one.txt"), "one-v1\n");
    const faster = await acvc.record({ workspace, premise: "faster cache" });
    await acvc.score(faster.variant, ["tests_passed=10", "latency_ms=40"]);
    writeFileSync(path.join(workspace, "a", "one.txt"), "one-v2\n");
    const slower = await acvc.record({ workspace, premise: "slower cache" });
    await acvc.score(slower.variant, ["tests_passed=10", "latency_ms=50"]);

    const frontier = await acvc.frontier("speed");
    assert.deepEqual(
      frontier.variants.map((row) => row.claim),
      ["faster cache"],
    );
    assert.equal(frontier.variants[0]?.variant, faster.variant);

    const hits = await acvc.recall("speed", "faster");
    assert.ok(hits.hits.some((hit) => hit.variant === faster.variant));

    const why = await acvc.why(faster.variant);
    assert.deepEqual(why.parents, [seeded.variant]);
    assert.equal(why.claim, "faster cache");

    const compared = await acvc.compare(faster.variant, slower.variant);
    assert.equal(compared.dominance, "left");
    assert.ok(compared.modified.includes("a/one.txt"));

    const diff = await acvc.fileDiff(faster.variant, slower.variant, "a/one.txt");
    assert.match(diff.diff, /- one-v1/);
    assert.match(diff.diff, /\+ one-v2/);
    assert.deepEqual(await acvc.campaigns(), ["speed"]);
  } finally {
    rmSync(repo, { recursive: true, force: true });
  }
});
