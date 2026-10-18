import { fill, statusLabel, text, type Lang } from "./locale.ts";
import type { Compare, FrontierVariant, Recall, Why } from "./types.ts";

export interface DiffPath {
  kind: "added" | "removed" | "modified";
  path: string;
}

export function metricsText(metrics: Record<string, number>): string {
  return Object.entries(metrics)
    .map(([name, value]) => `${name} ${value}`)
    .join(" ");
}

export function frontierLabel(row: FrontierVariant): string {
  const metrics = metricsText(row.metrics);
  return metrics ? `${row.claim}  ${metrics}` : row.claim;
}

export function whyText(why: Why, lang?: Lang): string {
  const lines = [why.claim];
  if (why.assumptions.length > 0) lines.push(fill("cause", { items: why.assumptions.join(" · ") }, lang));
  if (why.intent) lines.push(fill("intent", { intent: why.intent }, lang));
  const trial = why.trials[0];
  const measured = trial ? metricsText(trial.metrics) || text("no_measurement", lang) : text("no_measurement", lang);
  lines.push(fill("measurement", { text: measured }, lang));
  if (why.rejection) lines.push(fill("rejected", { reason: why.rejection }, lang));
  else if (why.on_frontier) lines.push(text("on_frontier", lang));
  else lines.push(text("motive_unrecorded", lang));
  for (const winner of why.beaten_by ?? []) {
    const gaps = winner.gaps
      .map((gap) => fill("against", { name: gap.name, loser: String(gap.loser), winner: String(gap.winner), sense: sense(gap.direction, lang) }, lang))
      .join("; ");
    const beaten = fill("beaten_by", { claim: winner.claim }, lang);
    lines.push(gaps ? `${beaten} ${gaps}` : beaten);
  }
  const parents = why.parent_claims ?? [];
  lines.push(parents.length === 0 ? text("opened_campaign", lang) : fill("came_from", { names: parents.join("; ") }, lang));
  const children = why.child_claims ?? [];
  if (children.length > 0) lines.push(fill("after_named", { names: children.join("; ") }, lang));
  lines.push(fill("agent", { agent: why.agent || text("agent_unknown", lang) }, lang));
  return lines.join("\n");
}

export function compareText(report: Compare, lang?: Lang): string {
  const left = report.left_claim || text("the_first", lang);
  const right = report.right_claim || text("the_other", lang);
  const lines = [verdict(report.dominance, left, right, lang)];
  for (const delta of report.metrics) {
    lines.push(fill("against", {
      name: delta.name,
      loser: number(delta.left, lang),
      winner: number(delta.right, lang),
      sense: sense(delta.direction, lang),
    }, lang));
  }
  return lines.join("\n");
}

export function recallText(report: Recall, lang?: Lang): string {
  if (report.hits.length === 0) return text("no_recall", lang);
  return report.hits
    .map((hit) => {
      const parents = hit.parents ?? [];
      const from = parents.length === 0 ? text("opened_campaign", lang) : fill("came_from", { names: parents.map((parent) => parent.claim).join("; ") }, lang);
      return `${hit.claim}\n${statusLabel(hit.status || "sem medição", lang)}\n${from}`;
    })
    .join("\n\n");
}

export function pathLabel(kind: DiffPath["kind"], path: string, lang?: Lang): string {
  if (kind === "added") return fill("only_other", { path }, lang);
  if (kind === "removed") return fill("only_first", { path }, lang);
  return fill("changed_path", { path }, lang);
}

function verdict(kind: string, left: string, right: string, lang?: Lang): string {
  if (kind === "left") return fill("stays", { claim: left }, lang);
  if (kind === "right") return fill("stays", { claim: right }, lang);
  if (kind === "tie") return fill("tie_both", { left, right }, lang);
  if (kind === "incomparable") return text("incomparable", lang);
  return text("missing_measure_which", lang);
}

function sense(direction: string | undefined, lang?: Lang): string {
  return direction === "lower" ? text("lower_better", lang) : text("higher_better", lang);
}

function number(value: number | null, lang?: Lang): string {
  if (value == null) return text("no_measurement", lang);
  return Number.isInteger(value) ? String(value) : String(value);
}

export function diffPaths(report: Compare): DiffPath[] {
  return [
    ...report.added.map((path) => ({ kind: "added" as const, path })),
    ...report.removed.map((path) => ({ kind: "removed" as const, path })),
    ...report.modified.map((path) => ({ kind: "modified" as const, path })),
  ];
}
