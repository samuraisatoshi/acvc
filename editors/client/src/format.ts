import type { Compare, FrontierVariant, Recall, Why } from "./types.ts";

export interface DiffPath {
  kind: "added" | "removed" | "modified";
  path: string;
}

export function metricsText(metrics: Record<string, number>): string {
  return Object.entries(metrics)
    .map(([name, value]) => `${name}=${value}`)
    .join(" ");
}

export function frontierLabel(row: FrontierVariant): string {
  const metrics = metricsText(row.metrics);
  return metrics ? `${row.claim}  ${metrics}` : row.claim;
}

export function whyText(why: Why): string {
  const lines = [
    why.claim,
    `edge ${why.edge}`,
    `agent ${why.agent}`,
    why.intent ? `intent ${why.intent}` : "",
    `assumptions ${why.assumptions.join("; ") || "none"}`,
    `parents ${why.parents.join(" ") || "none"}`,
    `children ${why.children.join(" ") || "none"}`,
  ];
  for (const trial of why.trials) lines.push(`trial ${trial.id} ${metricsText(trial.metrics)}`);
  if (why.rejection) lines.push(`rejected ${why.rejection}`);
  return lines.filter((line) => line.length > 0).join("\n");
}

export function compareText(report: Compare): string {
  const lines = [`dominance ${report.dominance}`];
  for (const delta of report.metrics) lines.push(`${delta.name} ${delta.left ?? "-"} -> ${delta.right ?? "-"}`);
  for (const path of report.added) lines.push(`added ${path}`);
  for (const path of report.removed) lines.push(`removed ${path}`);
  for (const path of report.modified) lines.push(`modified ${path}`);
  return lines.join("\n");
}

export function recallText(report: Recall): string {
  if (report.hits.length === 0) return "no hits";
  return report.hits.map((hit) => `${hit.claim}\n${hit.variant}`).join("\n\n");
}

export function diffPaths(report: Compare): DiffPath[] {
  return [
    ...report.added.map((path) => ({ kind: "added" as const, path })),
    ...report.removed.map((path) => ({ kind: "removed" as const, path })),
    ...report.modified.map((path) => ({ kind: "modified" as const, path })),
  ];
}
