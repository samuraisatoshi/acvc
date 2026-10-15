import { runAcvc } from "./run.ts";
import type {
  AcvcOptions,
  CampaignList,
  Compare,
  FileDiff,
  Frontier,
  Recall,
  Recorded,
  Scored,
  Seeded,
  Why,
} from "./types.ts";

export { AcvcError } from "./run.ts";

export class Acvc {
  private readonly opts: AcvcOptions;

  constructor(opts: AcvcOptions) {
    this.opts = opts;
  }

  init(): Promise<{ repo: string }> {
    return this.call(["init"]);
  }

  campaigns(): Promise<string[]> {
    return this.call<CampaignList>(["campaign", "list"]).then((body) => body.campaigns);
  }

  createCampaign(name: string, metrics: string[]): Promise<{ campaign: string; metrics: string[] }> {
    return this.call(["campaign", "create", name, ...flag("--metric", metrics)]);
  }

  seed(input: {
    campaign: string;
    path: string;
    premise: string;
    assumptions?: string[];
    intent?: string;
    agent?: string;
  }): Promise<Seeded> {
    const args = ["seed", "--campaign", input.campaign, "--path", input.path, "--premise", input.premise];
    if (input.intent) args.push("--intent", input.intent);
    if (input.agent) args.push("--agent", input.agent);
    args.push(...flag("--assumption", input.assumptions ?? []));
    return this.call(args);
  }

  materialize(variant: string, into: string): Promise<{ variant: string; dest: string; files: number }> {
    return this.call(["materialize", "--variant", variant, "--into", into]);
  }

  record(input: { workspace: string; premise: string; edge?: string }): Promise<Recorded> {
    const args = ["record", "--workspace", input.workspace, "--premise", input.premise];
    if (input.edge) args.push("--edge", input.edge);
    return this.call(args);
  }

  score(variant: string, metrics: string[]): Promise<Scored> {
    return this.call(["score", "--variant", variant, ...flag("--metric", metrics)]);
  }

  frontier(campaign: string): Promise<Frontier> {
    return this.call(["frontier", "--campaign", campaign]);
  }

  why(variant: string): Promise<Why> {
    return this.call(["why", variant]);
  }

  compare(left: string, right: string): Promise<Compare> {
    return this.call(["compare", left, right]);
  }

  fileDiff(left: string, right: string, file: string): Promise<FileDiff> {
    return this.call(["file-diff", left, right, "--path", file]);
  }

  recall(campaign: string, query: string): Promise<Recall> {
    return this.call(["recall", "--campaign", campaign, "--query", query]);
  }

  private call<T>(args: string[]): Promise<T> {
    const prefix = ["--format", "json"];
    if (this.opts.repo) prefix.push("--repo", this.opts.repo);
    return runAcvc(this.opts.binary, [...prefix, ...args], this.opts.cwd) as Promise<T>;
  }
}

function flag(name: string, values: string[]): string[] {
  return values.flatMap((value) => [name, value]);
}
