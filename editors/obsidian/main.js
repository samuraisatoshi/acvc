"use strict";
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __hasOwnProp = Object.prototype.hasOwnProperty;
var __export = (target, all) => {
  for (var name in all)
    __defProp(target, name, { get: all[name], enumerable: true });
};
var __copyProps = (to, from, except, desc) => {
  if (from && typeof from === "object" || typeof from === "function") {
    for (let key of __getOwnPropNames(from))
      if (!__hasOwnProp.call(to, key) && key !== except)
        __defProp(to, key, { get: () => from[key], enumerable: !(desc = __getOwnPropDesc(from, key)) || desc.enumerable });
  }
  return to;
};
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/main.ts
var main_exports = {};
__export(main_exports, {
  default: () => AcvcPlugin
});
module.exports = __toCommonJS(main_exports);
var import_obsidian3 = require("obsidian");

// src/settings.ts
var import_obsidian = require("obsidian");
var DEFAULT_SETTINGS = {
  binary: "acvc",
  repo: ""
};
var AcvcSettingTab = class extends import_obsidian.PluginSettingTab {
  constructor(app, plugin) {
    super(app, plugin);
    this.plugin = plugin;
  }
  display() {
    const { containerEl } = this;
    containerEl.empty();
    new import_obsidian.Setting(containerEl).setName("acvc binary").setDesc("Path to the standalone executable. Cargo is not required.").addText(
      (text2) => text2.setValue(this.plugin.settings.binary).onChange(async (value) => {
        this.plugin.settings.binary = value.trim() || "acvc";
        await this.plugin.saveSettings();
      })
    );
    new import_obsidian.Setting(containerEl).setName("repository").setDesc("Folder that contains .acvc. Empty uses the vault folder.").addText(
      (text2) => text2.setValue(this.plugin.settings.repo).onChange(async (value) => {
        this.plugin.settings.repo = value.trim();
        await this.plugin.saveSettings();
      })
    );
  }
};

// src/view.ts
var import_obsidian2 = require("obsidian");

// ../client/src/run.ts
var import_child_process = require("child_process");
var AcvcError = class extends Error {
  context;
  constructor(message, context) {
    super(message);
    this.name = "AcvcError";
    this.context = context;
  }
};
function runAcvc(binary, args, cwd) {
  return new Promise((resolve, reject) => {
    const child = (0, import_child_process.spawn)(binary, args, { cwd });
    const out = [];
    const err = [];
    child.stdout.on("data", (chunk) => out.push(chunk));
    child.stderr.on("data", (chunk) => err.push(chunk));
    child.on("error", reject);
    child.on("close", (code) => {
      const stdout = Buffer.concat(out).toString("utf8");
      const stderr = Buffer.concat(err).toString("utf8");
      if (code !== 0) {
        reject(failure(stderr, code));
        return;
      }
      try {
        resolve(JSON.parse(stdout));
      } catch (cause) {
        reject(new AcvcError("acvc stdout is not json", { stdout, cause: String(cause) }));
      }
    });
  });
}
function failure(stderr, code) {
  try {
    const parsed = JSON.parse(stderr);
    if (parsed.error) return new AcvcError(parsed.error, parsed.context ?? {});
  } catch {
    return new AcvcError(stderr.trim() || `acvc exited ${code}`, { code });
  }
  return new AcvcError(stderr.trim() || `acvc exited ${code}`, { code });
}

// ../client/src/api.ts
var Acvc = class {
  opts;
  constructor(opts) {
    this.opts = opts;
  }
  init() {
    return this.call(["init"]);
  }
  campaigns() {
    return this.call(["campaign", "list"]).then((body) => body.campaigns);
  }
  createCampaign(name, metrics) {
    return this.call(["campaign", "create", name, ...flag("--metric", metrics)]);
  }
  seed(input) {
    const args = ["seed", "--campaign", input.campaign, "--path", input.path, "--premise", input.premise];
    if (input.intent) args.push("--intent", input.intent);
    if (input.agent) args.push("--agent", input.agent);
    args.push(...flag("--assumption", input.assumptions ?? []));
    return this.call(args);
  }
  materialize(variant, into) {
    return this.call(["materialize", "--variant", variant, "--into", into]);
  }
  record(input) {
    const args = ["record", "--workspace", input.workspace, "--premise", input.premise];
    if (input.edge) args.push("--edge", input.edge);
    return this.call(args);
  }
  score(variant, metrics) {
    return this.call(["score", "--variant", variant, ...flag("--metric", metrics)]);
  }
  frontier(campaign) {
    return this.call(["frontier", "--campaign", campaign]);
  }
  why(variant) {
    return this.call(["why", variant]);
  }
  compare(left, right) {
    return this.call(["compare", left, right]);
  }
  fileDiff(left, right, file) {
    return this.call(["file-diff", left, right, "--path", file]);
  }
  recall(campaign, query) {
    return this.call(["recall", "--campaign", campaign, "--query", query]);
  }
  call(args) {
    const prefix = ["--format", "json"];
    if (this.opts.repo) prefix.push("--repo", this.opts.repo);
    return runAcvc(this.opts.binary, [...prefix, ...args], this.opts.cwd);
  }
};
function flag(name, values) {
  return values.flatMap((value) => [name, value]);
}

// ../../locales/el.json
var el_default = {
  on_frontier: "\u03A3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF.",
  motive_unrecorded: "\u03A4\u03BF \u03BA\u03AF\u03BD\u03B7\u03C4\u03C1\u03BF \u03B4\u03B5\u03BD \u03BA\u03B1\u03C4\u03B1\u03B3\u03C1\u03AC\u03C6\u03B7\u03BA\u03B5.",
  weighed: "\u03B6\u03CD\u03B3\u03B9\u03B6\u03B5 {path} {bytes}",
  no_frontier: "\u039A\u03B1\u03BC\u03AF\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B4\u03B5\u03BD \u03B5\u03AF\u03BD\u03B1\u03B9 \u03C3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF.",
  no_recall: "\u039A\u03B1\u03BC\u03AF\u03B1 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B4\u03B5\u03BD \u03C0\u03B5\u03C1\u03B9\u03AD\u03C7\u03B5\u03B9 \u03B1\u03C5\u03C4\u03CC \u03C4\u03BF \u03BA\u03B5\u03AF\u03BC\u03B5\u03BD\u03BF.",
  opened_campaign: "\u0391\u03C5\u03C4\u03AE \u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03AC\u03BD\u03BF\u03B9\u03BE\u03B5 \u03C4\u03B7\u03BD \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1.",
  came_from: "\u0389\u03C1\u03B8\u03B5 \u03B1\u03C0\u03CC: {names}",
  stays: "\u039C\u03AD\u03BD\u03B5\u03B9: {claim}.",
  tie_both: "\u039F\u03B9 {left} \u03BA\u03B1\u03B9 {right} \u03BC\u03B5\u03C4\u03C1\u03BF\u03CD\u03BD \u03C4\u03BF \u03AF\u03B4\u03B9\u03BF.",
  incomparable: "\u039A\u03B1\u03BC\u03AF\u03B1 \u03B4\u03B5\u03BD \u03BC\u03AD\u03BD\u03B5\u03B9 \u03BC\u03CC\u03BD\u03B7. \u039A\u03B1\u03B8\u03B5\u03BC\u03AF\u03B1 \u03BA\u03B5\u03C1\u03B4\u03AF\u03B6\u03B5\u03B9 \u03C3\u03B5 \u03AC\u03BB\u03BB\u03BF \u03BC\u03AD\u03C4\u03C1\u03BF.",
  missing_measure: "\u039B\u03B5\u03AF\u03C0\u03B5\u03B9 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03B3\u03B9\u03B1 \u03BD\u03B1 \u03B1\u03C0\u03BF\u03C6\u03B1\u03C3\u03AF\u03C3\u03BF\u03C5\u03BC\u03B5.",
  missing_measure_which: "\u039B\u03B5\u03AF\u03C0\u03B5\u03B9 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03B3\u03B9\u03B1 \u03BD\u03B1 \u03B1\u03C0\u03BF\u03C6\u03B1\u03C3\u03AF\u03C3\u03BF\u03C5\u03BC\u03B5 \u03C0\u03BF\u03B9\u03B1 \u03BC\u03AD\u03BD\u03B5\u03B9.",
  no_measurement: "\u03C7\u03C9\u03C1\u03AF\u03C2 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7",
  cause: "\u0391\u03B9\u03C4\u03AF\u03B1: {items}",
  intent: "\u03A0\u03C1\u03CC\u03B8\u03B5\u03C3\u03B7: {intent}",
  measurement: "\u039C\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7: {text}",
  rejected: "\u0391\u03C0\u03BF\u03C1\u03C1\u03AF\u03C6\u03B8\u03B7\u03BA\u03B5: {reason}",
  rejected_gap: "\u0391\u03C0\u03BF\u03C1\u03C1\u03AF\u03C6\u03B8\u03B7\u03BA\u03B5  {reason}",
  beaten_by: "\u039E\u03B5\u03C0\u03B5\u03C1\u03AC\u03C3\u03C4\u03B7\u03BA\u03B5 \u03B1\u03C0\u03CC {claim}.",
  beaten_by_gap: "\u039E\u03B5\u03C0\u03B5\u03C1\u03AC\u03C3\u03C4\u03B7\u03BA\u03B5 \u03B1\u03C0\u03CC  {claim}.",
  higher_better: "\u03CC\u03C3\u03BF \u03BC\u03B5\u03B3\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03BF, \u03C4\u03CC\u03C3\u03BF \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03BF",
  lower_better: "\u03CC\u03C3\u03BF \u03BC\u03B9\u03BA\u03C1\u03CC\u03C4\u03B5\u03C1\u03BF, \u03C4\u03CC\u03C3\u03BF \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03BF",
  agent: "\u03A0\u03C1\u03AC\u03BA\u03C4\u03BF\u03C1\u03B1\u03C2: {agent}",
  agent_gap: "\u03A0\u03C1\u03AC\u03BA\u03C4\u03BF\u03C1\u03B1\u03C2  {agent}",
  no_scored: "\u039A\u03B1\u03BC\u03AF\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B4\u03B5\u03BD \u03AD\u03C7\u03B5\u03B9 \u03B2\u03B1\u03B8\u03BC\u03BF\u03BB\u03BF\u03B3\u03AF\u03B1.",
  appears_after_score: "\u0395\u03BC\u03C6\u03B1\u03BD\u03AF\u03B6\u03B5\u03C4\u03B1\u03B9 \u03B5\u03B4\u03CE \u03BC\u03B5\u03C4\u03AC \u03C4\u03BF acvc score.",
  open_hypothesis: "\u0391\u03BD\u03BF\u03AF\u03BE\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03C3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF.",
  score_heading: "\u0392\u03B1\u03B8\u03BC\u03BF\u03BB\u03BF\u03B3\u03AF\u03B1",
  still_unmeasured: "    \u03B1\u03BA\u03CC\u03BC\u03B7 \u03C7\u03C9\u03C1\u03AF\u03C2 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7",
  frontier_hint: "\u03A3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF, \u03C0\u03B1\u03C4\u03AE\u03C3\u03C4\u03B5 d \u03C3\u03B5 \u03B4\u03CD\u03BF \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2.",
  nothing_compared: "\u0394\u03B5\u03BD \u03AD\u03B3\u03B9\u03BD\u03B5 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03C3\u03CD\u03B3\u03BA\u03C1\u03B9\u03C3\u03B7.",
  search_label: "\u0391\u03BD\u03B1\u03B6\u03AE\u03C4\u03B7\u03C3\u03B7  {query}",
  none_chosen: "\u0394\u03B5\u03BD \u03B5\u03C0\u03B9\u03BB\u03AD\u03C7\u03B8\u03B7\u03BA\u03B5 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7.",
  came_from_short: "\u03AE\u03C1\u03B8\u03B5 \u03B1\u03C0\u03CC  {claim}",
  then_short: "\u03BC\u03B5\u03C4\u03AC  {claim}",
  origin_prefix: "\u03A0\u03C1\u03BF\u03AD\u03BB\u03B5\u03C5\u03C3\u03B7  {text}",
  origin_seed: "\u0386\u03BD\u03BF\u03B9\u03BE\u03B5 \u03C4\u03B7\u03BD \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1. \u0394\u03B5\u03BD \u03C5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03C0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7.",
  origin_mutate: "\u0391\u03BB\u03BB\u03B1\u03B3\u03AE \u03B1\u03C0\u03CC \u03AC\u03BB\u03BB\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7.",
  origin_revert: "\u0395\u03C0\u03B9\u03C3\u03C4\u03C1\u03BF\u03C6\u03AE \u03C3\u03B5 \u03C0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7.",
  origin_revise: "\u0397 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03BE\u03B1\u03BD\u03B1\u03B3\u03C1\u03AC\u03C6\u03C4\u03B7\u03BA\u03B5.",
  origin_synthesize: "\u03A3\u03CD\u03BD\u03B8\u03B5\u03C3\u03B7 \u03C0\u03BF\u03BB\u03BB\u03CE\u03BD \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03C9\u03BD.",
  agent_unknown: "\u03B4\u03B5\u03BD \u03B4\u03CC\u03B8\u03B7\u03BA\u03B5",
  status_frontier: "\u03C3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF",
  status_beaten: "\u03BE\u03B5\u03C0\u03B5\u03C1\u03AC\u03C3\u03C4\u03B7\u03BA\u03B5",
  status_rejected: "\u03B1\u03C0\u03BF\u03C1\u03C1\u03AF\u03C6\u03B8\u03B7\u03BA\u03B5",
  different_files: "\u03B4\u03B9\u03B1\u03C6\u03BF\u03C1\u03B5\u03C4\u03B9\u03BA\u03AC \u03B1\u03C1\u03C7\u03B5\u03AF\u03B1 {count}",
  against: "{name} {loser} \u03AD\u03BD\u03B1\u03BD\u03C4\u03B9 {winner} ({sense})",
  title: "\u038C,\u03C4\u03B9 \u03B1\u03BE\u03AF\u03B6\u03B5\u03B9 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03BD\u03B1 \u03B5\u03BE\u03B5\u03C1\u03B5\u03C5\u03BD\u03B7\u03B8\u03B5\u03AF",
  lede: "\u039C\u03B9\u03B1 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1 \u03C3\u03C5\u03B3\u03BA\u03C1\u03AF\u03BD\u03B5\u03B9 \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2 \u03BC\u03B5 \u03B2\u03AC\u03C3\u03B7 \u03C4\u03B7\u03BD \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03BA\u03B1\u03B9 \u03C4\u03B9\u03C2 \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2. \u03A4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF \u03B4\u03B5\u03AF\u03C7\u03BD\u03B5\u03B9 \u03CC\u03C3\u03B5\u03C2 \u03BA\u03B1\u03BC\u03AF\u03B1 \u03AC\u03BB\u03BB\u03B7 \u03B4\u03B5\u03BD \u03BE\u03B5\u03C0\u03AD\u03C1\u03B1\u03C3\u03B5.",
  campaign_label: "\u0395\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1",
  frontier_heading: "\u03A3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF",
  frontier_hint_page: "\u03A0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03BA\u03B1\u03B9 \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2 \u03CC\u03C3\u03C9\u03BD \u03B7 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03B4\u03B5\u03BD \u03AD\u03C7\u03B5\u03B9 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03B1\u03C0\u03BF\u03C1\u03C1\u03AF\u03C8\u03B5\u03B9.",
  hypothesis_heading: "\u0397 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7",
  why_hint: "\u0395\u03C0\u03B9\u03BB\u03AD\u03BE\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B3\u03B9\u03B1 \u03BD\u03B1 \u03B4\u03B5\u03AF\u03C4\u03B5 \u03C4\u03B7\u03BD \u03C0\u03C1\u03BF\u03AD\u03BB\u03B5\u03C5\u03C3\u03B7, \u03C4\u03B9\u03C2 \u03C0\u03B1\u03C1\u03B1\u03B4\u03BF\u03C7\u03AD\u03C2 \u03BA\u03B1\u03B9 \u03C4\u03B9\u03C2 \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2.",
  lineage_heading: "\u03A0\u03C1\u03BF\u03AD\u03BB\u03B5\u03C5\u03C3\u03B7 \u03BA\u03B1\u03B9 \u03B4\u03B9\u03B1\u03BA\u03BB\u03B1\u03B4\u03CE\u03C3\u03B5\u03B9\u03C2",
  lineage_hint: "\u03A4\u03BF \u03C0\u03C1\u03CC\u03B2\u03BB\u03B7\u03BC\u03B1, \u03C0\u03BF\u03B9\u03BF\u03C2 \u03B1\u03C0\u03B1\u03BD\u03C4\u03AC, \u03C0\u03CC\u03C3\u03BF \u03BA\u03B1\u03B9\u03C1\u03CC \u03B5\u03AF\u03BD\u03B1\u03B9 \u03B1\u03BD\u03BF\u03B9\u03C7\u03C4\u03AE \u03B7 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1, \u03BA\u03B1\u03B9 \u03BA\u03AC\u03B8\u03B5 \u03B4\u03B9\u03B1\u03B4\u03C1\u03BF\u03BC\u03AE \u03BC\u03B5 \u03C4\u03BF \u03B1\u03C0\u03BF\u03C4\u03AD\u03BB\u03B5\u03C3\u03BC\u03AC \u03C4\u03B7\u03C2.",
  search_heading: "\u0391\u03BD\u03B1\u03B6\u03AE\u03C4\u03B7\u03C3\u03B7",
  search_hint: "\u0392\u03C1\u03AF\u03C3\u03BA\u03B5\u03B9 \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2 \u03B1\u03C0\u03CC \u03C4\u03B7 \u03C6\u03C1\u03AC\u03C3\u03B7 \u03C4\u03B7\u03C2 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7\u03C2, \u03BA\u03B1\u03B9 \u03CC\u03C3\u03B5\u03C2 \u03AD\u03C6\u03C5\u03B3\u03B1\u03BD \u03B1\u03C0\u03CC \u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF.",
  premise_text: "\u039A\u03B5\u03AF\u03BC\u03B5\u03BD\u03BF \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7\u03C2",
  premise_placeholder: "\u03C4\u03B9 \u03B9\u03C3\u03C7\u03C5\u03C1\u03AF\u03B6\u03B5\u03C4\u03B1\u03B9 \u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7",
  search_button: "\u0391\u03BD\u03B1\u03B6\u03AE\u03C4\u03B7\u03C3\u03B7",
  which_stays: "\u03A0\u03BF\u03B9\u03B1 \u03BC\u03AD\u03BD\u03B5\u03B9",
  which_stays_hint: "\u03A3\u03C5\u03B3\u03BA\u03C1\u03AF\u03BD\u03B5\u03B9 \u03B4\u03CD\u03BF \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2: \u03C0\u03BF\u03B9\u03B1 \u03BC\u03B5\u03C4\u03C1\u03AC \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B1 \u03BA\u03B1\u03B9 \u03C0\u03BF\u03B9\u03B1 \u03B1\u03C1\u03C7\u03B5\u03AF\u03B1 \u03AC\u03BB\u03BB\u03B1\u03BE\u03B1\u03BD.",
  one_hypothesis: "\u039C\u03AF\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7",
  other_hypothesis: "\u0386\u03BB\u03BB\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7",
  compare_button: "\u03A3\u03CD\u03B3\u03BA\u03C1\u03B9\u03C3\u03B7",
  web_seed: "\u0391\u03C5\u03C4\u03AE \u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03AC\u03BD\u03BF\u03B9\u03BE\u03B5 \u03C4\u03B7\u03BD \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1. \u0394\u03B5\u03BD \u03C5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03C0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7.",
  web_mutate: "\u0391\u03C5\u03C4\u03AE \u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B1\u03BB\u03BB\u03AC\u03B6\u03B5\u03B9 \u03C4\u03BF\u03BD \u03BA\u03CE\u03B4\u03B9\u03BA\u03B1 \u03B1\u03C0\u03CC \u03AC\u03BB\u03BB\u03B7 \u03BA\u03B1\u03B9 \u03BA\u03C1\u03B1\u03C4\u03AC \u03C4\u03B7\u03BD \u03B5\u03C1\u03CE\u03C4\u03B7\u03C3\u03B7.",
  web_revert: "\u0391\u03C5\u03C4\u03AE \u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B3\u03C5\u03C1\u03AF\u03B6\u03B5\u03B9 \u03C3\u03B5 \u03C0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B7 \u03B4\u03B9\u03B1\u03B4\u03C1\u03BF\u03BC\u03AE. \u03A4\u03BF \u03B1\u03B4\u03B9\u03AD\u03BE\u03BF\u03B4\u03BF \u03BC\u03AD\u03BD\u03B5\u03B9 \u03BA\u03B1\u03C4\u03B1\u03B3\u03B5\u03B3\u03C1\u03B1\u03BC\u03BC\u03AD\u03BD\u03BF.",
  web_revise: "\u0397 \u03B5\u03C1\u03CE\u03C4\u03B7\u03C3\u03B7 \u03AC\u03BB\u03BB\u03B1\u03BE\u03B5. \u0397 \u03C0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B7 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B1\u03BD\u03C4\u03B9\u03BA\u03B1\u03C4\u03B1\u03C3\u03C4\u03AC\u03B8\u03B7\u03BA\u03B5.",
  web_synthesize: "\u0391\u03C5\u03C4\u03AE \u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B5\u03BD\u03CE\u03BD\u03B5\u03B9 \u03C0\u03B5\u03C1\u03B9\u03C3\u03C3\u03CC\u03C4\u03B5\u03C1\u03B5\u03C2 \u03B1\u03C0\u03CC \u03BC\u03AF\u03B1 \u03C0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B5\u03C2 \u03B4\u03B9\u03B1\u03B4\u03C1\u03BF\u03BC\u03AD\u03C2.",
  read_failed: "\u0394\u03B5\u03BD \u03AE\u03C4\u03B1\u03BD \u03B4\u03C5\u03BD\u03B1\u03C4\u03CC \u03BD\u03B1 \u03B4\u03B9\u03B1\u03B2\u03B1\u03C3\u03C4\u03B5\u03AF \u03B7 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1.",
  campaign_measure: "\u039C\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03B1\u03C5\u03C4\u03AE\u03C2 \u03C4\u03B7\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2.",
  higher_sentence: "\u038C\u03C3\u03BF \u03BC\u03B5\u03B3\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03BF, \u03C4\u03CC\u03C3\u03BF \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03BF.",
  lower_sentence: "\u038C\u03C3\u03BF \u03BC\u03B9\u03BA\u03C1\u03CC\u03C4\u03B5\u03C1\u03BF, \u03C4\u03CC\u03C3\u03BF \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03BF.",
  unmeasured_yet: "\u0391\u03BA\u03CC\u03BC\u03B7 \u03C7\u03C9\u03C1\u03AF\u03C2 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7.",
  origin_kicker: "\u03A0\u03C1\u03BF\u03AD\u03BB\u03B5\u03C5\u03C3\u03B7",
  assumption_kicker: "\u03A4\u03B9 \u03C5\u03C0\u03BF\u03B8\u03AD\u03C4\u03B5\u03B9 \u03B1\u03C5\u03C4\u03AE \u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7",
  intent_kicker: "\u03A4\u03B9 \u03AE\u03B8\u03B5\u03BB\u03B5 \u03BD\u03B1 \u03C0\u03B5\u03C4\u03CD\u03C7\u03B5\u03B9",
  metrics_kicker: "\u039C\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2",
  nobody_measured: "\u039A\u03B1\u03BD\u03B5\u03AF\u03C2 \u03B4\u03B5\u03BD \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B5 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03B1\u03C5\u03C4\u03AE \u03C4\u03B7\u03BD \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7.",
  left_frontier_measure: "\u039C\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03C0\u03BF\u03C5 \u03AD\u03C6\u03C5\u03B3\u03B5 \u03B1\u03C0\u03CC \u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF.",
  frontier_measure: "\u03A4\u03B5\u03BB\u03B5\u03C5\u03C4\u03B1\u03AF\u03B1 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03C0\u03BF\u03C5 \u03C7\u03C1\u03B7\u03C3\u03B9\u03BC\u03BF\u03C0\u03BF\u03B9\u03AE\u03B8\u03B7\u03BA\u03B5 \u03C3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF.",
  earlier_measure: "\u03A0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B7 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7.",
  off_frontier: "\u0395\u03BA\u03C4\u03CC\u03C2 \u03C3\u03C5\u03BD\u03CC\u03C1\u03BF\u03C5: {reason}",
  registered_by: "\u039A\u03B1\u03C4\u03B1\u03B3\u03C1\u03AC\u03C6\u03B7\u03BA\u03B5 \u03B1\u03C0\u03CC {agent}.",
  what_came_after: "\u03A4\u03B9 \u03AE\u03C1\u03B8\u03B5 \u03BC\u03B5\u03C4\u03AC",
  no_measure_title: "\u03A7\u03C9\u03C1\u03AF\u03C2 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7",
  branch_seed: "\u03C0\u03C1\u03BF\u03AD\u03BB\u03B5\u03C5\u03C3\u03B7",
  branch_mutate: "\u03B4\u03B9\u03B1\u03BA\u03BB\u03AC\u03B4\u03C9\u03C3\u03B7",
  branch_revert: "\u03B5\u03C0\u03B9\u03C3\u03C4\u03C1\u03BF\u03C6\u03AE",
  branch_revise: "\u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B1\u03BD\u03B1\u03B8\u03B5\u03C9\u03C1\u03B7\u03BC\u03AD\u03BD\u03B7",
  branch_synthesize: "\u03C3\u03CD\u03BD\u03B8\u03B5\u03C3\u03B7",
  beaten_inline: "\u03BE\u03B5\u03C0\u03B5\u03C1\u03AC\u03C3\u03C4\u03B7\u03BA\u03B5 \u03B1\u03C0\u03CC {claim}",
  need_fact: "\u0391\u03BD\u03AC\u03B3\u03BA\u03B7",
  owner_fact: "\u03A5\u03C0\u03B5\u03CD\u03B8\u03C5\u03BD\u03BF\u03C2",
  nobody: "\u03BA\u03B1\u03BD\u03B5\u03AF\u03C2",
  agent_word: "\u03C0\u03C1\u03AC\u03BA\u03C4\u03BF\u03C1\u03B1\u03C2",
  no_origin_yet: "\u0394\u03B5\u03BD \u03C5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03C0\u03C1\u03BF\u03AD\u03BB\u03B5\u03C5\u03C3\u03B7. \u0397 \u03C0\u03C1\u03CE\u03C4\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B1\u03BD\u03BF\u03AF\u03B3\u03B5\u03B9 \u03C4\u03BF\u03BD \u03C7\u03AC\u03C1\u03C4\u03B7.",
  none_survived: "\u039A\u03B1\u03BC\u03AF\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B4\u03B5\u03BD \u03B5\u03C0\u03AD\u03B6\u03B7\u03C3\u03B5 \u03C4\u03C9\u03BD \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03C9\u03BD.",
  search_or_score: "\u0391\u03BD\u03B1\u03B6\u03B7\u03C4\u03AE\u03C3\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03AE \u03BA\u03B1\u03C4\u03B1\u03B3\u03C1\u03AC\u03C8\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7.",
  opened_short: "\u03AC\u03BD\u03BF\u03B9\u03BE\u03B5 \u03C4\u03B7\u03BD \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1",
  came_from_inline: "\u03AE\u03C1\u03B8\u03B5 \u03B1\u03C0\u03CC {claim}",
  stays_detail: "\u039C\u03AD\u03BD\u03B5\u03B9 \u03B7 {claim}. \u0395\u03AF\u03BD\u03B1\u03B9 \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B7 \u03AE \u03AF\u03C3\u03B7 \u03C3\u03B5 \u03BA\u03AC\u03B8\u03B5 \u03BC\u03AD\u03C4\u03C1\u03BF \u03BA\u03B1\u03B9 \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B7 \u03C3\u03B5 \u03AD\u03BD\u03B1.",
  tie_metrics: "\u0399\u03C3\u03BF\u03C0\u03B1\u03BB\u03AF\u03B1. \u039F\u03B9 \u03B4\u03CD\u03BF \u03BC\u03B5\u03C4\u03C1\u03BF\u03CD\u03BD \u03C4\u03BF \u03AF\u03B4\u03B9\u03BF.",
  each_wins: "\u039A\u03B1\u03B8\u03B5\u03BC\u03AF\u03B1 \u03BA\u03B5\u03C1\u03B4\u03AF\u03B6\u03B5\u03B9 \u03C3\u03B5 \u03AC\u03BB\u03BB\u03BF \u03BC\u03AD\u03C4\u03C1\u03BF. \u0397 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1 \u03B4\u03B5\u03BD \u03AD\u03C7\u03B5\u03B9 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03BC\u03BF\u03BD\u03B1\u03B4\u03B9\u03BA\u03CC \u03BD\u03B9\u03BA\u03B7\u03C4\u03AE.",
  metric_tie: "\u0399\u03C3\u03BF\u03C0\u03B1\u03BB\u03AF\u03B1 \u03C3\u03B5 \u03B1\u03C5\u03C4\u03CC \u03C4\u03BF \u03BC\u03AD\u03C4\u03C1\u03BF.",
  text_diff: "\u0394\u03B9\u03B1\u03C6\u03BF\u03C1\u03AC \u03BA\u03B5\u03B9\u03BC\u03AD\u03BD\u03BF\u03C5.",
  only_in: "\u0391\u03C5\u03C4\u03CC \u03C4\u03BF \u03B1\u03C1\u03C7\u03B5\u03AF\u03BF \u03C5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03BC\u03CC\u03BD\u03BF \u03C3\u03C4\u03BF {name}.",
  no_text_diff: "\u03A4\u03BF \u03B1\u03C1\u03C7\u03B5\u03AF\u03BF \u03B4\u03B5\u03BD \u03AD\u03C7\u03B5\u03B9 \u03B4\u03B9\u03B1\u03C6\u03BF\u03C1\u03AC \u03BA\u03B5\u03B9\u03BC\u03AD\u03BD\u03BF\u03C5.",
  metric_col: "\u039C\u03AD\u03C4\u03C1\u03BF",
  reading_col: "\u0391\u03BD\u03AC\u03B3\u03BD\u03C9\u03C3\u03B7",
  first_one: "\u03B7 \u03C0\u03C1\u03CE\u03C4\u03B7",
  second_one: "\u03B7 \u03B4\u03B5\u03CD\u03C4\u03B5\u03C1\u03B7",
  the_first: "\u0397 \u03C0\u03C1\u03CE\u03C4\u03B7",
  the_second: "\u0397 \u03B4\u03B5\u03CD\u03C4\u03B5\u03C1\u03B7",
  different_text: "\u0394\u03B9\u03B1\u03C6\u03BF\u03C1\u03B5\u03C4\u03B9\u03BA\u03CC \u03BA\u03B5\u03AF\u03BC\u03B5\u03BD\u03BF",
  added_files: "\u039D\u03AD\u03B1 \u03B1\u03C1\u03C7\u03B5\u03AF\u03B1",
  removed_files: "\u0391\u03C1\u03C7\u03B5\u03AF\u03B1 \u03C0\u03BF\u03C5 \u03B1\u03C6\u03B1\u03B9\u03C1\u03AD\u03B8\u03B7\u03BA\u03B1\u03BD",
  campaign_line: "\u0395\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1 {name}",
  problem: "\u03A0\u03C1\u03CC\u03B2\u03BB\u03B7\u03BC\u03B1: {text}",
  need: "\u0391\u03BD\u03AC\u03B3\u03BA\u03B7: {text}",
  owner: "\u03A5\u03C0\u03B5\u03CD\u03B8\u03C5\u03BD\u03BF\u03C2: {name}",
  opened_summary: "\u0391\u03BD\u03BF\u03B9\u03C7\u03C4\u03AE {age}. {variants} \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2, {trials} \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2, {rejections} \u03B1\u03C0\u03BF\u03C1\u03C1\u03AF\u03C8\u03B5\u03B9\u03C2. {tokens}",
  map_label: "\u03A7\u03AC\u03C1\u03C4\u03B7\u03C2:",
  no_hypothesis_yet: "\u0397 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1 \u03B4\u03B5\u03BD \u03AD\u03C7\u03B5\u03B9 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7. \u0397 \u03C0\u03C1\u03CE\u03C4\u03B7 \u03B5\u03B3\u03B3\u03C1\u03B1\u03C6\u03AE \u03B5\u03AF\u03BD\u03B1\u03B9 \u03B7 \u03C0\u03C1\u03BF\u03AD\u03BB\u03B5\u03C5\u03C3\u03B7 \u03C4\u03BF\u03C5 \u03C0\u03C1\u03BF\u03B2\u03BB\u03AE\u03BC\u03B1\u03C4\u03BF\u03C2.",
  many_origins: "\u0397 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1 \u03AD\u03C7\u03B5\u03B9 {count} \u03C0\u03C1\u03BF\u03B5\u03BB\u03B5\u03CD\u03C3\u03B5\u03B9\u03C2: {claims}.",
  guide_intro: "\u03A4\u03BF acvc \u03C6\u03C5\u03BB\u03AC\u03B5\u03B9 \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2, \u03C4\u03B7\u03BD \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03BA\u03B1\u03B8\u03B5\u03BC\u03AF\u03B1\u03C2 \u03BA\u03B1\u03B9 \u03C4\u03B9\u03C2 \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2. \u03A4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF \u03B4\u03B5\u03AF\u03C7\u03BD\u03B5\u03B9 \u03CC\u03C3\u03B5\u03C2 \u03BA\u03B1\u03BC\u03AF\u03B1 \u03AC\u03BB\u03BB\u03B7 \u03B4\u03B5\u03BD \u03BE\u03B5\u03C0\u03AD\u03C1\u03B1\u03C3\u03B5. \u039C\u03B9\u03B1 \u03C7\u03B5\u03B9\u03C1\u03CC\u03C4\u03B5\u03C1\u03B7 \u03B4\u03BF\u03BA\u03B9\u03BC\u03AE \u03B4\u03B5\u03BD \u03B1\u03C6\u03B1\u03B9\u03C1\u03B5\u03AF \u03C4\u03B7\u03BD \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B7 \u03C4\u03B7\u03C2 \u03AF\u03B4\u03B9\u03B1\u03C2 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7\u03C2. \u0397 \u03B1\u03C0\u03CC\u03C1\u03C1\u03B9\u03C8\u03B7 \u03B5\u03AF\u03BD\u03B1\u03B9 \u03B1\u03BD\u03B8\u03C1\u03CE\u03C0\u03B9\u03BD\u03B7 \u03C0\u03C1\u03AC\u03BE\u03B7.",
  none_stays: "\u039A\u03B1\u03BC\u03AF\u03B1 \u03B4\u03B5\u03BD \u03BC\u03AD\u03BD\u03B5\u03B9.",
  metric_of_campaign: "\u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03C4\u03B7\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2",
  no_campaign: "\u03C7\u03C9\u03C1\u03AF\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1",
  campaigns: "\u0395\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B5\u03C2",
  frontier_screen: "\u03A3\u03CD\u03BD\u03BF\u03C1\u03BF",
  lineage_screen: "\u0393\u03B5\u03BD\u03B5\u03B1\u03BB\u03BF\u03B3\u03AF\u03B1",
  premise_screen: "\u03A0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7",
  compare_screen: "\u03A3\u03CD\u03B3\u03BA\u03C1\u03B9\u03C3\u03B7",
  recall_screen: "\u0391\u03BD\u03B1\u03B6\u03AE\u03C4\u03B7\u03C3\u03B7",
  help_search: "Enter \u03B1\u03BD\u03B1\u03B6\u03B7\u03C4\u03AC. Esc \u03B1\u03BA\u03C5\u03C1\u03CE\u03BD\u03B5\u03B9. Backspace \u03C3\u03B2\u03AE\u03BD\u03B5\u03B9.",
  help_campaigns: "Enter \u03B1\u03BD\u03BF\u03AF\u03B3\u03B5\u03B9 \u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF. \u2190 \u2192 \u03B1\u03BB\u03BB\u03AC\u03B6\u03B5\u03B9 \u03BF\u03B8\u03CC\u03BD\u03B7. q \u03B2\u03B3\u03B1\u03AF\u03BD\u03B5\u03B9.",
  help_frontier: "Enter \u03B1\u03BD\u03BF\u03AF\u03B3\u03B5\u03B9 \u03C4\u03B7\u03BD \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7. d \u03C3\u03C5\u03B3\u03BA\u03C1\u03AF\u03BD\u03B5\u03B9 \u03B4\u03CD\u03BF. \u2190 \u2192 \u03B1\u03BB\u03BB\u03AC\u03B6\u03B5\u03B9 \u03BF\u03B8\u03CC\u03BD\u03B7. q \u03B2\u03B3\u03B1\u03AF\u03BD\u03B5\u03B9.",
  help_lineage: "Enter \u03B1\u03BD\u03BF\u03AF\u03B3\u03B5\u03B9 \u03B1\u03C5\u03C4\u03AE \u03C4\u03B7\u03BD \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7. \u2190 \u2192 \u03B1\u03BB\u03BB\u03AC\u03B6\u03B5\u03B9 \u03BF\u03B8\u03CC\u03BD\u03B7. q \u03B2\u03B3\u03B1\u03AF\u03BD\u03B5\u03B9.",
  help_detail: "\u0397 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03BA\u03B1\u03B9 \u03BF\u03B9 \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2. \u2190 \u2192 \u03B1\u03BB\u03BB\u03AC\u03B6\u03B5\u03B9 \u03BF\u03B8\u03CC\u03BD\u03B7. q \u03B2\u03B3\u03B1\u03AF\u03BD\u03B5\u03B9.",
  help_diff: "Enter \u03B4\u03B5\u03AF\u03C7\u03BD\u03B5\u03B9 \u03C4\u03BF \u03B1\u03C1\u03C7\u03B5\u03AF\u03BF. \u2190 \u2192 \u03B1\u03BB\u03BB\u03AC\u03B6\u03B5\u03B9 \u03BF\u03B8\u03CC\u03BD\u03B7. q \u03B2\u03B3\u03B1\u03AF\u03BD\u03B5\u03B9.",
  no_campaigns: "\u039A\u03B1\u03BC\u03AF\u03B1 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1.",
  open_lineage: "\u0391\u03BD\u03BF\u03AF\u03BE\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03C3\u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF \u03B3\u03B9\u03B1 \u03BD\u03B1 \u03B4\u03B5\u03AF\u03C4\u03B5 \u03B1\u03C0\u03CC \u03C0\u03BF\u03CD \u03AE\u03C1\u03B8\u03B5.",
  no_before_after: "\u0394\u03B5\u03BD \u03C5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03C0\u03C1\u03BF\u03B7\u03B3\u03BF\u03CD\u03BC\u03B5\u03BD\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03BF\u03CD\u03C4\u03B5 \u03BA\u03AC\u03C4\u03B9 \u03C0\u03BF\u03C5 \u03AE\u03C1\u03B8\u03B5 \u03BC\u03B5\u03C4\u03AC.",
  intent_gap: "\u03A0\u03C1\u03CC\u03B8\u03B5\u03C3\u03B7  {intent}",
  assumptions_gap: "\u03A0\u03B1\u03C1\u03B1\u03B4\u03BF\u03C7\u03AD\u03C2  {items}",
  came_then: "\u0389\u03C1\u03B8\u03B5 \u03B1\u03C0\u03CC {from}    \u039C\u03B5\u03C4\u03AC {after}",
  recall_prompt: "\u0393\u03C1\u03AC\u03C8\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03BB\u03AD\u03BE\u03B7 \u03C4\u03B7\u03C2 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7\u03C2 \u03BA\u03B1\u03B9 \u03C0\u03B1\u03C4\u03AE\u03C3\u03C4\u03B5 Enter.",
  came_from_one: "\u03AE\u03C1\u03B8\u03B5 \u03B1\u03C0\u03CC {claim}",
  pick_campaign: "\u0395\u03C0\u03B9\u03BB\u03AD\u03BE\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1.",
  first_marked: "\u0397 \u03C0\u03C1\u03CE\u03C4\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03C3\u03B7\u03BC\u03B5\u03B9\u03CE\u03B8\u03B7\u03BA\u03B5. \u03A0\u03B1\u03C4\u03AE\u03C3\u03C4\u03B5 d \u03C3\u03C4\u03B7\u03BD \u03AC\u03BB\u03BB\u03B7.",
  first_named: "\u03A0\u03C1\u03CE\u03C4\u03B7: {claim}. \u03A0\u03B1\u03C4\u03AE\u03C3\u03C4\u03B5 d \u03C3\u03C4\u03B7\u03BD \u03AC\u03BB\u03BB\u03B7.",
  ago_under_minute: "\u03C0\u03C1\u03B9\u03BD \u03B1\u03C0\u03CC \u03BB\u03B9\u03B3\u03CC\u03C4\u03B5\u03C1\u03BF \u03B1\u03C0\u03CC \u03AD\u03BD\u03B1 \u03BB\u03B5\u03C0\u03C4\u03CC",
  ago_minutes: "\u03C0\u03C1\u03B9\u03BD \u03B1\u03C0\u03CC {count} \u03BB\u03B5\u03C0\u03C4\u03AC",
  ago_hours: "\u03C0\u03C1\u03B9\u03BD \u03B1\u03C0\u03CC {count} \u03CE\u03C1\u03B5\u03C2",
  ago_days: "\u03C0\u03C1\u03B9\u03BD \u03B1\u03C0\u03CC {count} \u03B7\u03BC\u03AD\u03C1\u03B5\u03C2",
  no_tokens: "\u039A\u03B1\u03BD\u03AD\u03BD\u03B1 token \u03B4\u03B5\u03BD \u03BA\u03B1\u03C4\u03B1\u03B3\u03C1\u03AC\u03C6\u03B7\u03BA\u03B5. \u039A\u03B1\u03C4\u03B1\u03B3\u03C1\u03AC\u03C8\u03C4\u03B5 \u03C4\u03B7\u03BD \u03BA\u03B1\u03C4\u03B1\u03BD\u03AC\u03BB\u03C9\u03C3\u03B7 \u03BC\u03B5 spend, \u03C7\u03C9\u03C1\u03AF\u03C2 \u03BD\u03B1 \u03B1\u03BB\u03BB\u03AC\u03BE\u03B5\u03C4\u03B5 \u03C4\u03B7\u03BD \u03C4\u03B1\u03C5\u03C4\u03CC\u03C4\u03B7\u03C4\u03B1 \u03C4\u03B7\u03C2 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7\u03C2.",
  tokens_recorded: "{tokens} tokens \u03AE\u03B4\u03B7 \u03BA\u03B1\u03C4\u03B1\u03B3\u03B5\u03B3\u03C1\u03B1\u03BC\u03BC\u03AD\u03BD\u03B1",
  rejection_label: "\u03B1\u03C0\u03CC\u03C1\u03C1\u03B9\u03C8\u03B7: {reason}",
  guide_example: "\u039F \u03C7\u03AC\u03C1\u03C4\u03B7\u03C2 \u03C4\u03B7\u03C2 \u03B1\u03BD\u03BF\u03B9\u03C7\u03C4\u03AE\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2. \u03A7\u03C9\u03C1\u03AF\u03C2 \u03B1\u03C0\u03BF\u03B8\u03B5\u03C4\u03AE\u03C1\u03B9\u03BF, \u03B4\u03B5\u03BD \u03C5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03C0\u03B1\u03C1\u03AC\u03B4\u03B5\u03B9\u03B3\u03BC\u03B1.",
  guide_text: "\u0394\u03B9\u03B1\u03B2\u03AC\u03C3\u03C4\u03B5 context \u03C4\u03B7\u03C2 \u03B1\u03BD\u03BF\u03B9\u03C7\u03C4\u03AE\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2. \u039F \u03C7\u03AC\u03C1\u03C4\u03B7\u03C2 \u03B1\u03C5\u03C4\u03BF\u03CD \u03C4\u03BF\u03C5 \u03B1\u03C0\u03BF\u03B8\u03B5\u03C4\u03B7\u03C1\u03AF\u03BF\u03C5 \u03B5\u03AF\u03BD\u03B1\u03B9 \u03C4\u03BF \u03C0\u03B1\u03C1\u03AC\u03B4\u03B5\u03B9\u03B3\u03BC\u03B1.\n\u0397 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B5\u03AF\u03BD\u03B1\u03B9 \u03C4\u03BF \u03C0\u03C1\u03CC\u03B2\u03BB\u03B7\u03BC\u03B1. \u0397 \u03B1\u03B9\u03C4\u03AF\u03B1 \u03BC\u03AD\u03BD\u03B5\u03B9 \u03C3\u03C4\u03B7\u03BD \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7, \u03CC\u03C7\u03B9 \u03C3\u03B5 \u03BC\u03AF\u03B1 \u03C6\u03C1\u03AC\u03C3\u03B7 \u03C4\u03B7\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2.\n\u03A4\u03BF score \u03BA\u03B1\u03C4\u03B1\u03B3\u03C1\u03AC\u03C6\u03B5\u03B9 \u03C4\u03B7 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7. \u039C\u03B9\u03B1 \u03C7\u03B5\u03B9\u03C1\u03CC\u03C4\u03B5\u03C1\u03B7 \u03B4\u03BF\u03BA\u03B9\u03BC\u03AE \u03B4\u03B5\u03BD \u03B1\u03C6\u03B1\u03B9\u03C1\u03B5\u03AF \u03C4\u03B7\u03BD \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B7 \u03B1\u03C0\u03CC \u03C4\u03BF \u03C3\u03CD\u03BD\u03BF\u03C1\u03BF.\n\u03A4\u03BF seed \u03B1\u03BD\u03BF\u03AF\u03B3\u03B5\u03B9 \u03C4\u03B7\u03BD \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1 \u03C3\u03C4\u03BF\u03BD \u03C6\u03AC\u03BA\u03B5\u03BB\u03BF \u03C4\u03B7\u03C2 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7\u03C2. \u0397 \u03C3\u03AC\u03C1\u03C9\u03C3\u03B7 \u03B4\u03B5\u03BD \u03B4\u03B9\u03B1\u03B2\u03AC\u03B6\u03B5\u03B9 \u03C4\u03BF .gitignore. \u0388\u03BD\u03B1 .acvcignore \u03C3\u03C4\u03BF\u03BD \u03C6\u03AC\u03BA\u03B5\u03BB\u03BF \u03B1\u03C6\u03B1\u03B9\u03C1\u03B5\u03AF \u03C4\u03B9\u03C2 \u03AF\u03B4\u03B9\u03B5\u03C2 \u03B4\u03B9\u03B1\u03B4\u03C1\u03BF\u03BC\u03AD\u03C2 \u03C3\u03B5 \u03BA\u03AC\u03B8\u03B5 \u03BA\u03BB\u03CE\u03BD\u03BF. \u03A4\u03BF seed \u03BB\u03AD\u03B5\u03B9 \u03C0\u03CC\u03C3\u03B1 \u03B1\u03C1\u03C7\u03B5\u03AF\u03B1 \u03BC\u03C0\u03AE\u03BA\u03B1\u03BD \u03BA\u03B1\u03B9 \u03C0\u03BF\u03B9\u03B1 \u03B6\u03CD\u03B3\u03B9\u03B6\u03B1\u03BD \u03C0\u03B5\u03C1\u03B9\u03C3\u03C3\u03CC\u03C4\u03B5\u03C1\u03BF. \u03A4\u03BF store \u03BC\u03AD\u03BD\u03B5\u03B9 \u03C3\u03C4\u03BF --repo\xB7 \u03BF \u03C6\u03AC\u03BA\u03B5\u03BB\u03BF\u03C2 \u03BA\u03C1\u03B1\u03C4\u03AC \u03C4\u03BF\u03C5\u03C2 \u03B4\u03CD\u03BF \u03B4\u03B5\u03AF\u03BA\u03C4\u03B5\u03C2, \u03C3\u03C4\u03BF .gitignore. \u0397 \u03B5\u03C0\u03CC\u03BC\u03B5\u03BD\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03C7\u03C1\u03B7\u03C3\u03B9\u03BC\u03BF\u03C0\u03BF\u03B9\u03B5\u03AF record \u03BA\u03B1\u03B9 \u03AD\u03C7\u03B5\u03B9 \u03B3\u03BF\u03BD\u03AD\u03B1.\n",
  status_frontier_why: "\u039C\u03B5\u03C4\u03C1\u03B7\u03BC\u03AD\u03BD\u03B7, \u03BA\u03B1\u03B9 \u03BA\u03B1\u03BC\u03AF\u03B1 \u03AC\u03BB\u03BB\u03B7 \u03B4\u03B5\u03BD \u03C4\u03B7\u03BD \u03BE\u03B5\u03C0\u03B5\u03C1\u03BD\u03AC.",
  status_beaten_why: "\u039C\u03B5\u03C4\u03C1\u03B7\u03BC\u03AD\u03BD\u03B7, \u03BA\u03B1\u03B9 \u03AC\u03BB\u03BB\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03C4\u03BF\u03C5 \u03C3\u03C5\u03BD\u03CC\u03C1\u03BF\u03C5 \u03B5\u03AF\u03BD\u03B1\u03B9 \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B7.",
  status_rejected_why: "\u03A4\u03BF \u03BA\u03AF\u03BD\u03B7\u03C4\u03C1\u03BF \u03B5\u03AF\u03BD\u03B1\u03B9 \u03C3\u03C4\u03BF \u03B3\u03C1\u03AC\u03C6\u03B7\u03BC\u03B1.",
  status_unmeasured_why: "\u0394\u03B5\u03BD \u03C5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03C0\u03BB\u03AE\u03C1\u03B5\u03C2 score \u03B3\u03B9\u03B1 \u03C4\u03BF \u03BC\u03AD\u03C4\u03C1\u03BF \u03C4\u03B7\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2.",
  when_context: "\u03A0\u03C1\u03B9\u03BD \u03B5\u03BD\u03B5\u03C1\u03B3\u03AE\u03C3\u03B5\u03C4\u03B5. \u0395\u03C0\u03B9\u03C3\u03C4\u03C1\u03AD\u03C6\u03B5\u03B9 \u03C4\u03BF\u03BD \u03C7\u03AC\u03C1\u03C4\u03B7 \u03B1\u03C5\u03C4\u03AE\u03C2 \u03C4\u03B7\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2.",
  when_guide: "\u039F\u03B9 \u03B5\u03BD\u03C4\u03BF\u03BB\u03AD\u03C2, \u03C3\u03C5\u03BD \u03C4\u03BF\u03BD \u03C7\u03AC\u03C1\u03C4\u03B7 \u03C4\u03BF\u03C5 \u03B1\u03BD\u03BF\u03B9\u03C7\u03C4\u03BF\u03CD \u03B1\u03C0\u03BF\u03B8\u03B5\u03C4\u03B7\u03C1\u03AF\u03BF\u03C5.",
  when_frontier: "\u039C\u03CC\u03BD\u03BF \u03CC,\u03C4\u03B9 \u03B7 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03B4\u03B5\u03BD \u03AD\u03C7\u03B5\u03B9 \u03B1\u03BA\u03CC\u03BC\u03B7 \u03BE\u03B5\u03C0\u03B5\u03C1\u03AC\u03C3\u03B5\u03B9.",
  when_why: "\u039C\u03AF\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7, \u03BC\u03B5 \u03C4\u03B7\u03BD \u03B1\u03B9\u03C4\u03AF\u03B1, \u03C4\u03B7 \u03BC\u03AD\u03C4\u03C1\u03B7\u03C3\u03B7 \u03BA\u03B1\u03B9 \u03C0\u03BF\u03B9\u03BF\u03C2 \u03C4\u03B7\u03BD \u03BE\u03B5\u03C0\u03AD\u03C1\u03B1\u03C3\u03B5.",
  when_recall: "\u0392\u03C1\u03B5\u03AF\u03C4\u03B5 \u03BC\u03B9\u03B1 \u03C0\u03C1\u03BF\u03CB\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B1\u03C0\u03CC \u03C4\u03BF \u03BA\u03B5\u03AF\u03BC\u03B5\u03BD\u03BF. \u0395\u03C0\u03B9\u03C3\u03C4\u03C1\u03AD\u03C6\u03B5\u03B9 \u03C4\u03B7\u03BD \u03BA\u03B1\u03C4\u03AC\u03C3\u03C4\u03B1\u03C3\u03B7 \u03BA\u03B1\u03B9 \u03C4\u03BF\u03BD \u03B3\u03BF\u03BD\u03AD\u03B1.",
  when_seed: "\u0397 \u03C0\u03C1\u03CE\u03C4\u03B7 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03BC\u03B9\u03B1\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2. \u03A3\u03C0\u03B5\u03AF\u03C1\u03B5\u03C4\u03B5 \u03C4\u03BF\u03BD \u03C6\u03AC\u03BA\u03B5\u03BB\u03BF \u03C4\u03B7\u03C2 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7\u03C2: \u03B7 \u03C3\u03AC\u03C1\u03C9\u03C3\u03B7 \u03B4\u03B5\u03BD \u03B4\u03B9\u03B1\u03B2\u03AC\u03B6\u03B5\u03B9 \u03C4\u03BF .gitignore \u03BA\u03B1\u03B9 \u03C0\u03B1\u03C1\u03B1\u03BB\u03B5\u03AF\u03C0\u03B5\u03B9 .git, .acvc, target, node_modules, .build \u03BA\u03B1\u03B9 DerivedData. \u0388\u03BD\u03B1 .acvcignore \u03C3\u03C4\u03BF\u03BD \u03C6\u03AC\u03BA\u03B5\u03BB\u03BF \u03B1\u03C6\u03B1\u03B9\u03C1\u03B5\u03AF \u03C4\u03B9\u03C2 \u03AF\u03B4\u03B9\u03B5\u03C2 \u03B4\u03B9\u03B1\u03B4\u03C1\u03BF\u03BC\u03AD\u03C2 \u03C3\u03B5 \u03BA\u03AC\u03B8\u03B5 \u03BA\u03BB\u03CE\u03BD\u03BF\xB7 \u03C7\u03C9\u03C1\u03AF\u03C2 \u03B1\u03C5\u03C4\u03CC \u03B9\u03C3\u03C7\u03CD\u03B5\u03B9 \u03B7 \u03C3\u03C4\u03B1\u03B8\u03B5\u03C1\u03AE \u03BB\u03AF\u03C3\u03C4\u03B1. \u03A4\u03BF seed \u03BB\u03AD\u03B5\u03B9 \u03C0\u03CC\u03C3\u03B1 \u03B1\u03C1\u03C7\u03B5\u03AF\u03B1 \u03BC\u03C0\u03AE\u03BA\u03B1\u03BD \u03BA\u03B1\u03B9 \u03C0\u03BF\u03B9\u03B1 \u03B6\u03CD\u03B3\u03B9\u03B6\u03B1\u03BD \u03C0\u03B5\u03C1\u03B9\u03C3\u03C3\u03CC\u03C4\u03B5\u03C1\u03BF. \u03A4\u03BF store \u03BC\u03AD\u03BD\u03B5\u03B9 \u03C3\u03C4\u03BF --repo. \u039F \u03C6\u03AC\u03BA\u03B5\u03BB\u03BF\u03C2 \u03BA\u03C1\u03B1\u03C4\u03AC .acvc-workspace \u03BA\u03B1\u03B9 .acvc-index, \u03BA\u03B1\u03B9 \u03C4\u03B1 \u03B4\u03CD\u03BF \u03C3\u03C4\u03BF .gitignore.",
  when_record: "\u039F \u03C6\u03AC\u03BA\u03B5\u03BB\u03BF\u03C2 \u03AC\u03BB\u03BB\u03B1\u03BE\u03B5. \u0397 \u03BD\u03AD\u03B1 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03B4\u03B5\u03AF\u03C7\u03BD\u03B5\u03B9 \u03C4\u03BF\u03BD \u03B3\u03BF\u03BD\u03AD\u03B1.",
  when_score: "\u03A5\u03C0\u03AC\u03C1\u03C7\u03B5\u03B9 \u03B1\u03C1\u03B9\u03B8\u03BC\u03CC\u03C2 \u03B3\u03B9\u03B1 \u03C4\u03BF \u03BC\u03AD\u03C4\u03C1\u03BF \u03C4\u03B7\u03C2 \u03B5\u03BA\u03C3\u03C4\u03C1\u03B1\u03C4\u03B5\u03AF\u03B1\u03C2. \u039C\u03B9\u03B1 \u03C7\u03B5\u03B9\u03C1\u03CC\u03C4\u03B5\u03C1\u03B7 \u03B4\u03BF\u03BA\u03B9\u03BC\u03AE \u03B4\u03B5\u03BD \u03B1\u03BD\u03C4\u03B9\u03BA\u03B1\u03B8\u03B9\u03C3\u03C4\u03AC \u03BC\u03B9\u03B1 \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B7.",
  when_reject: "\u0397 \u03C5\u03C0\u03CC\u03B8\u03B5\u03C3\u03B7 \u03C0\u03AD\u03B8\u03B1\u03BD\u03B5 \u03BA\u03B1\u03B9 \u03C4\u03BF \u03BA\u03AF\u03BD\u03B7\u03C4\u03C1\u03BF \u03BC\u03AD\u03BD\u03B5\u03B9 \u03C3\u03C4\u03BF \u03B3\u03C1\u03AC\u03C6\u03B7\u03BC\u03B1.",
  when_spend: "\u0397 \u03C3\u03C5\u03BD\u03B5\u03B4\u03C1\u03AF\u03B1 \u03BE\u03AD\u03C1\u03B5\u03B9 \u03C0\u03CC\u03C3\u03B1 tokens \u03BA\u03B1\u03C4\u03B1\u03BD\u03AC\u03BB\u03C9\u03C3\u03B5.",
  when_compare: "\u0394\u03CD\u03BF \u03BC\u03B5\u03C4\u03C1\u03B7\u03BC\u03AD\u03BD\u03B5\u03C2 \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2, \u03B3\u03B9\u03B1 \u03BD\u03B1 \u03B4\u03B5\u03AF\u03C4\u03B5 \u03C0\u03BF\u03B9\u03B1 \u03BC\u03AD\u03BD\u03B5\u03B9 \u03BA\u03B1\u03B9 \u03C0\u03BF\u03B9\u03B1 \u03B1\u03C1\u03C7\u03B5\u03AF\u03B1 \u03AC\u03BB\u03BB\u03B1\u03BE\u03B1\u03BD.",
  when_web: "\u039F \u03C7\u03AC\u03C1\u03C4\u03B7\u03C2 \u03C3\u03C4\u03B7 \u03C3\u03B5\u03BB\u03AF\u03B4\u03B1.",
  when_obsidian: "\u0395\u03B3\u03BA\u03B1\u03B8\u03B9\u03C3\u03C4\u03AC \u03C4\u03BF\u03BD \u03B8\u03B5\u03B1\u03C4\u03AE \u03C3\u03C4\u03BF vault \u03BA\u03B1\u03B9 \u03B4\u03B5\u03AF\u03C7\u03BD\u03B5\u03B9 \u03B1\u03C5\u03C4\u03CC \u03C4\u03BF \u03B1\u03C0\u03BF\u03B8\u03B5\u03C4\u03AE\u03C1\u03B9\u03BF. \u0397 \u03B5\u03BD\u03C4\u03BF\u03BB\u03AE \u03C3\u03C4\u03BF Obsidian \u03B5\u03AF\u03BD\u03B1\u03B9 Open ACVC.",
  none_count: "\u03BA\u03B1\u03BD\u03AD\u03BD\u03B1",
  help_recall: "/ \u03B1\u03BD\u03B1\u03B6\u03B7\u03C4\u03AC. Enter \u03B1\u03BD\u03B1\u03B6\u03B7\u03C4\u03AC. \u2190 \u2192 \u03B1\u03BB\u03BB\u03AC\u03B6\u03B5\u03B9 \u03BF\u03B8\u03CC\u03BD\u03B7. q \u03B2\u03B3\u03B1\u03AF\u03BD\u03B5\u03B9.",
  came_label: "\u0389\u03C1\u03B8\u03B5 \u03B1\u03C0\u03CC",
  problem_label: "\u03A0\u03C1\u03CC\u03B2\u03BB\u03B7\u03BC\u03B1",
  better_here: "\u0397 {name} \u03B5\u03AF\u03BD\u03B1\u03B9 \u03BA\u03B1\u03BB\u03CD\u03C4\u03B5\u03C1\u03B7 \u03B5\u03B4\u03CE.",
  only_side: "\u039C\u03CC\u03BD\u03BF \u03C3\u03C4\u03BF {name}",
  web_campaign_fact: "{age} \xB7 {variants} \u03C5\u03C0\u03BF\u03B8\u03AD\u03C3\u03B5\u03B9\u03C2 \xB7 {trials} \u03BC\u03B5\u03C4\u03C1\u03AE\u03C3\u03B5\u03B9\u03C2 \xB7 {tokens}",
  only_other: "\u03BC\u03CC\u03BD\u03BF \u03C3\u03C4\u03B7\u03BD \u03AC\u03BB\u03BB\u03B7  {path}",
  only_first: "\u03BC\u03CC\u03BD\u03BF \u03C3\u03C4\u03B7\u03BD \u03C0\u03C1\u03CE\u03C4\u03B7  {path}",
  changed_path: "\u03AC\u03BB\u03BB\u03B1\u03BE\u03B5  {path}",
  the_other: "\u0397 \u03AC\u03BB\u03BB\u03B7",
  after_named: "\u03A4\u03B9 \u03AE\u03C1\u03B8\u03B5 \u03BC\u03B5\u03C4\u03AC: {names}",
  assistant_pack: "\u03A4\u03BF \u03C3\u03C5\u03BC\u03B2\u03CC\u03BB\u03B1\u03B9\u03BF \u03B3\u03B9\u03B1 \u03AC\u03BB\u03BB\u03BF\u03BD \u03C0\u03C1\u03AC\u03BA\u03C4\u03BF\u03C1\u03B1 \u03B2\u03B3\u03B1\u03AF\u03BD\u03B5\u03B9 \u03B1\u03C0\u03CC \u03C4\u03BF `acvc assistant`, \u03C3\u03C4\u03B7 \u03B3\u03BB\u03CE\u03C3\u03C3\u03B1 \u03B1\u03C5\u03C4\u03BF\u03CD \u03C4\u03BF\u03C5 \u03C5\u03C0\u03BF\u03BB\u03BF\u03B3\u03B9\u03C3\u03C4\u03AE. \u038C\u03C0\u03BF\u03B9\u03BF\u03C2 \u03AD\u03C7\u03B5\u03B9 \u03C4\u03BF MCP \u03C4\u03BF\u03C5 ASB \u03C3\u03C4\u03AD\u03BB\u03BD\u03B5\u03B9 \u03C4\u03BF\u03BD \u03C0\u03AF\u03BD\u03B1\u03BA\u03B1 documents \u03C3\u03C4\u03BF rag.index \u03BA\u03B1\u03B9 \u03BC\u03B5\u03C4\u03AC \u03C8\u03AC\u03C7\u03BD\u03B5\u03B9 \u03BC\u03B5 rag.search.",
  assistant_index: "\u0391\u03BD \u03B1\u03C5\u03C4\u03AE \u03B7 \u03C3\u03C5\u03BD\u03B5\u03B4\u03C1\u03AF\u03B1 \u03AD\u03C7\u03B5\u03B9 \u03C4\u03BF MCP \u03C4\u03BF\u03C5 ASB, \u03C3\u03C4\u03B5\u03AF\u03BB\u03C4\u03B5 \u03C4\u03BF\u03BD \u03C0\u03AF\u03BD\u03B1\u03BA\u03B1 documents \u03C3\u03C4\u03BF rag.index \u03BC\u03AF\u03B1 \u03C6\u03BF\u03C1\u03AC. \u0397 \u03B5\u03C0\u03CC\u03BC\u03B5\u03BD\u03B7 \u03B1\u03BD\u03B1\u03B6\u03AE\u03C4\u03B7\u03C3\u03B7 \u03C7\u03C1\u03B7\u03C3\u03B9\u03BC\u03BF\u03C0\u03BF\u03B9\u03B5\u03AF rag.search.",
  when_assistant: "\u0395\u03C0\u03B9\u03C3\u03C4\u03C1\u03AD\u03C6\u03B5\u03B9 \u03C4\u03BF \u03BB\u03B5\u03B9\u03C4\u03BF\u03C5\u03C1\u03B3\u03B9\u03BA\u03CC \u03C3\u03C5\u03BC\u03B2\u03CC\u03BB\u03B1\u03B9\u03BF \u03C3\u03C4\u03B7 \u03B3\u03BB\u03CE\u03C3\u03C3\u03B1 \u03B1\u03C5\u03C4\u03BF\u03CD \u03C4\u03BF\u03C5 \u03C5\u03C0\u03BF\u03BB\u03BF\u03B3\u03B9\u03C3\u03C4\u03AE, \u03AD\u03C4\u03BF\u03B9\u03BC\u03BF \u03B3\u03B9\u03B1 rag.index."
};

// ../../locales/en-US.json
var en_US_default = {
  on_frontier: "On the frontier.",
  motive_unrecorded: "Motive was not recorded.",
  weighed: "weighed {path} {bytes}",
  no_frontier: "No hypothesis is on the frontier.",
  no_recall: "No premise contains that text.",
  opened_campaign: "This hypothesis opened the campaign.",
  came_from: "Came from: {names}",
  stays: "{claim} stays.",
  tie_both: "{left} and {right} measure the same.",
  incomparable: "Neither stays alone. Each wins on a different metric.",
  missing_measure: "A measurement is missing, so nothing is decided.",
  missing_measure_which: "A measurement is missing, so which one stays is undecided.",
  no_measurement: "no measurement",
  cause: "Cause: {items}",
  intent: "Intent: {intent}",
  measurement: "Measurement: {text}",
  rejected: "Rejected: {reason}",
  rejected_gap: "Rejected  {reason}",
  beaten_by: "Beaten by {claim}.",
  beaten_by_gap: "Beaten by  {claim}.",
  higher_better: "higher is better",
  lower_better: "lower is better",
  agent: "Agent: {agent}",
  agent_gap: "Agent  {agent}",
  no_scored: "No hypothesis has been scored.",
  appears_after_score: "It shows up here after acvc score.",
  open_hypothesis: "Open a hypothesis on the frontier.",
  score_heading: "Score",
  still_unmeasured: "    still no measurement",
  frontier_hint: "On the frontier, press d on two hypotheses.",
  nothing_compared: "Nothing compared yet.",
  search_label: "Search  {query}",
  none_chosen: "No hypothesis chosen.",
  came_from_short: "came from  {claim}",
  then_short: "then  {claim}",
  origin_prefix: "Origin  {text}",
  origin_seed: "It started the campaign. There is no earlier hypothesis.",
  origin_mutate: "A change from another hypothesis.",
  origin_revert: "A return to an earlier hypothesis.",
  origin_revise: "The premise was rewritten.",
  origin_synthesize: "A synthesis of several hypotheses.",
  agent_unknown: "not given",
  status_frontier: "on the frontier",
  status_beaten: "beaten",
  status_rejected: "rejected",
  different_files: "different files {count}",
  against: "{name} {loser} against {winner} ({sense})",
  title: "What is still worth exploring",
  lede: "A campaign compares hypotheses by premise and measurement. The frontier shows those that no other has beaten.",
  campaign_label: "Campaign",
  frontier_heading: "On the frontier",
  frontier_hint_page: "The premise and metrics of what measurement has not yet set aside.",
  hypothesis_heading: "The hypothesis",
  why_hint: "Choose a hypothesis to see its origin, assumptions, and measurements.",
  lineage_heading: "Origin and branches",
  lineage_hint: "The problem, who is responsible, how long the campaign has been open, and each path with its result.",
  search_heading: "Search",
  search_hint: "Finds hypotheses by the premise sentence, including those that left the frontier.",
  premise_text: "Premise text",
  premise_placeholder: "what the hypothesis claims",
  search_button: "Search",
  which_stays: "Which one stays",
  which_stays_hint: "Compares two hypotheses: which measures better and which files changed.",
  one_hypothesis: "One hypothesis",
  other_hypothesis: "Another hypothesis",
  compare_button: "Compare",
  web_seed: "This hypothesis opened the campaign. There is no earlier hypothesis.",
  web_mutate: "This hypothesis changes the code from another and keeps the question.",
  web_revert: "This hypothesis returns to an earlier path. The dead end stays recorded.",
  web_revise: "The question changed. The earlier premise was replaced.",
  web_synthesize: "This hypothesis joins more than one earlier path.",
  read_failed: "The campaign could not be read.",
  campaign_measure: "Measurement for this campaign.",
  higher_sentence: "Higher is better.",
  lower_sentence: "Lower is better.",
  unmeasured_yet: "Still no measurement.",
  origin_kicker: "Origin",
  assumption_kicker: "What this hypothesis assumes",
  intent_kicker: "What was wanted",
  metrics_kicker: "Metrics",
  nobody_measured: "Nobody has measured this hypothesis yet.",
  left_frontier_measure: "A measurement that left the frontier.",
  frontier_measure: "Latest measurement used on the frontier.",
  earlier_measure: "Earlier measurement.",
  off_frontier: "Off the frontier: {reason}",
  registered_by: "Recorded by {agent}.",
  what_came_after: "What came after",
  no_measure_title: "No measurement",
  branch_seed: "origin",
  branch_mutate: "branch",
  branch_revert: "return",
  branch_revise: "premise revised",
  branch_synthesize: "synthesis",
  beaten_inline: "beaten by {claim}",
  need_fact: "Need",
  owner_fact: "Owner",
  nobody: "nobody",
  agent_word: "agent",
  no_origin_yet: "There is no origin yet. The first hypothesis opens the map.",
  none_survived: "No hypothesis survived the measurements.",
  search_or_score: "Search for a premise or record a measurement.",
  opened_short: "opened the campaign",
  came_from_inline: "came from {claim}",
  stays_detail: "{claim} stays. It is better or equal on every metric and better on one.",
  tie_metrics: "Tie. The two measure the same.",
  each_wins: "Each wins on a different metric. The campaign does not have a single winner yet.",
  metric_tie: "Tie on this metric.",
  text_diff: "Text differs.",
  only_in: "This file exists only in {name}.",
  no_text_diff: "The file has no text difference.",
  metric_col: "Metric",
  reading_col: "Reading",
  first_one: "the first",
  second_one: "the second",
  the_first: "The first",
  the_second: "The second",
  different_text: "Different text",
  added_files: "New files",
  removed_files: "Removed files",
  campaign_line: "Campaign {name}",
  problem: "Problem: {text}",
  need: "Need: {text}",
  owner: "Owner: {name}",
  opened_summary: "Open {age}. {variants} hypotheses, {trials} measurements, {rejections} rejections. {tokens}",
  map_label: "Map:",
  no_hypothesis_yet: "The campaign has no hypothesis yet. The first recording is the origin of the problem.",
  many_origins: "The campaign has {count} origins: {claims}.",
  guide_intro: "acvc stores hypotheses, the premise of each one, and the measurements. The frontier shows those that no other has beaten. A worse trial does not remove the better trial of the same hypothesis. Rejection is a human act.",
  none_stays: "Neither stays.",
  metric_of_campaign: "campaign measurement",
  no_campaign: "no campaign",
  campaigns: "Campaigns",
  frontier_screen: "Frontier",
  lineage_screen: "Lineage",
  premise_screen: "Premise",
  compare_screen: "Compare",
  recall_screen: "Search",
  help_search: "Enter searches. Esc cancels. Backspace deletes.",
  help_campaigns: "Enter opens the frontier. \u2190 \u2192 changes the screen. q quits.",
  help_frontier: "Enter opens the premise. d compares two. \u2190 \u2192 changes the screen. q quits.",
  help_lineage: "Enter opens that hypothesis. \u2190 \u2192 changes the screen. q quits.",
  help_detail: "The hypothesis and the measurements. \u2190 \u2192 changes the screen. q quits.",
  help_diff: "Enter shows the file. \u2190 \u2192 changes the screen. q quits.",
  no_campaigns: "No campaign.",
  open_lineage: "Open a hypothesis on the frontier to see where it came from.",
  no_before_after: "There is no earlier hypothesis and nothing came after.",
  intent_gap: "Intent  {intent}",
  assumptions_gap: "Assumptions  {items}",
  came_then: "Came from {from}    Then {after}",
  recall_prompt: "Type a word from the premise and press Enter.",
  came_from_one: "came from {claim}",
  pick_campaign: "Choose a campaign.",
  first_marked: "First hypothesis marked. Press d on the other.",
  first_named: "First: {claim}. Press d on the other.",
  ago_under_minute: "less than a minute ago",
  ago_minutes: "{count} min ago",
  ago_hours: "{count} h ago",
  ago_days: "{count} days ago",
  no_tokens: "No tokens recorded. Record the spend with spend, without changing the hypothesis identity.",
  tokens_recorded: "{tokens} tokens already recorded",
  rejection_label: "rejection: {reason}",
  guide_example: "The map of the open campaign. Without a repository, there is no example.",
  guide_text: "Read context for the open campaign. The map of that repository is the example.\nThe premise is the problem. The cause stays on the hypothesis, not in one campaign sentence.\nscore records the measurement. A worse trial does not remove the better one from the frontier.\nseed opens the campaign in the hypothesis folder. The scan does not read .gitignore. A .acvcignore in the folder drops the same paths in every clone. The seed says how many files entered and which weighed more. The store stays in --repo; the folder keeps the two markers, in .gitignore. The next hypothesis uses record and has a parent.\n",
  status_frontier_why: "Measured, and no other beats it.",
  status_beaten_why: "Measured, and another hypothesis on the frontier is better.",
  status_rejected_why: "The motive is in the graph.",
  status_unmeasured_why: "There is no complete score yet for the campaign metric.",
  when_context: "Before acting. Returns the map of this campaign.",
  when_guide: "The commands, plus the map of the open repository.",
  when_frontier: "Only what measurement has not yet beaten.",
  when_why: "One hypothesis, with the cause, the measurement, and who beat it.",
  when_recall: "Find a premise by text. Returns the status and the parent.",
  when_seed: "The first hypothesis of a campaign. Seed the hypothesis folder: the scan does not read .gitignore and skips .git, .acvc, target, node_modules, .build, and DerivedData. A .acvcignore in the folder drops the same paths in every clone; without it, the fixed list applies. The seed says how many files entered and which weighed more. The store stays in --repo. The folder keeps .acvc-workspace and .acvc-index, both in .gitignore.",
  when_record: "The folder changed. The new hypothesis points at the parent.",
  when_score: "There is a number for the campaign metric. A worse trial does not replace a better one.",
  when_reject: "The hypothesis died and the motive stays in the graph.",
  when_spend: "The session knows how many tokens it consumed.",
  when_compare: "Two measured hypotheses, to see which stays and which files changed.",
  when_web: "The map on a page.",
  when_obsidian: "Installs the viewer in the vault and points at this repository. The Obsidian command is Open ACVC.",
  none_count: "none",
  help_recall: "/ searches. Enter looks. \u2190 \u2192 changes the screen. q quits.",
  came_label: "Came from",
  problem_label: "Problem",
  better_here: "{name} is better here.",
  only_side: "Only in {name}",
  web_campaign_fact: "{age} \xB7 {variants} hypotheses \xB7 {trials} measurements \xB7 {tokens}",
  only_other: "only in the other  {path}",
  only_first: "only in the first  {path}",
  changed_path: "changed  {path}",
  the_other: "The other",
  after_named: "What came after: {names}",
  assistant_pack: "The contract for another agent comes from `acvc assistant`, in this computer's language. An agent with the ASB MCP submits the documents array to rag.index and later searches with rag.search.",
  assistant_index: "If this session has the ASB MCP, submit the documents array to rag.index once. Later search uses rag.search.",
  when_assistant: "Returns the operational contract in this computer's language, ready for rag.index."
};

// ../../locales/es.json
var es_default = {
  on_frontier: "En la frontera.",
  motive_unrecorded: "El motivo no qued\xF3 registrado.",
  weighed: "pes\xF3 {path} {bytes}",
  no_frontier: "Ninguna hip\xF3tesis est\xE1 en la frontera.",
  no_recall: "Ninguna premisa contiene ese texto.",
  opened_campaign: "Esta hip\xF3tesis abri\xF3 la campa\xF1a.",
  came_from: "Vino de: {names}",
  stays: "Se queda {claim}.",
  tie_both: "{left} y {right} miden igual.",
  incomparable: "Ninguna se queda sola. Cada una gana en una m\xE9trica distinta.",
  missing_measure: "Falta una medici\xF3n para decidir.",
  missing_measure_which: "Falta una medici\xF3n para decidir cu\xE1l se queda.",
  no_measurement: "sin medici\xF3n",
  cause: "Causa: {items}",
  intent: "Intenci\xF3n: {intent}",
  measurement: "Medici\xF3n: {text}",
  rejected: "Rechazada: {reason}",
  rejected_gap: "Rechazada  {reason}",
  beaten_by: "Superada por {claim}.",
  beaten_by_gap: "Superada por  {claim}.",
  higher_better: "cuanto mayor, mejor",
  lower_better: "cuanto menor, mejor",
  agent: "Agente: {agent}",
  agent_gap: "Agente  {agent}",
  no_scored: "Ninguna hip\xF3tesis tiene puntuaci\xF3n.",
  appears_after_score: "Aparece aqu\xED despu\xE9s de acvc score.",
  open_hypothesis: "Abra una hip\xF3tesis en la frontera.",
  score_heading: "Puntuaci\xF3n",
  still_unmeasured: "    a\xFAn sin medici\xF3n",
  frontier_hint: "En la frontera, pulse d en dos hip\xF3tesis.",
  nothing_compared: "A\xFAn no hay comparaci\xF3n.",
  search_label: "B\xFAsqueda  {query}",
  none_chosen: "Ninguna hip\xF3tesis elegida.",
  came_from_short: "vino de  {claim}",
  then_short: "despu\xE9s  {claim}",
  origin_prefix: "Origen  {text}",
  origin_seed: "Abri\xF3 la campa\xF1a. No hay una hip\xF3tesis anterior.",
  origin_mutate: "Un cambio a partir de otra hip\xF3tesis.",
  origin_revert: "Vuelve a una hip\xF3tesis anterior.",
  origin_revise: "La premisa fue reescrita.",
  origin_synthesize: "S\xEDntesis de varias hip\xF3tesis.",
  agent_unknown: "no informado",
  status_frontier: "en la frontera",
  status_beaten: "superada",
  status_rejected: "rechazada",
  different_files: "archivos diferentes {count}",
  against: "{name} {loser} contra {winner} ({sense})",
  title: "Lo que todav\xEDa vale explorar",
  lede: "Una campa\xF1a compara hip\xF3tesis por la premisa y las mediciones. La frontera muestra las que ninguna otra super\xF3.",
  campaign_label: "Campa\xF1a",
  frontier_heading: "En la frontera",
  frontier_hint_page: "Premisa y m\xE9tricas de lo que la medici\xF3n a\xFAn no descart\xF3.",
  hypothesis_heading: "La hip\xF3tesis",
  why_hint: "Elija una hip\xF3tesis para ver el origen, las suposiciones y las mediciones.",
  lineage_heading: "Origen y ramificaciones",
  lineage_hint: "El problema, qui\xE9n responde, cu\xE1nto tiempo lleva abierta la campa\xF1a y cada camino con su resultado.",
  search_heading: "Buscar",
  search_hint: "Encuentra hip\xF3tesis por la frase de la premisa, incluso las que salieron de la frontera.",
  premise_text: "Texto de la premisa",
  premise_placeholder: "lo que afirma la hip\xF3tesis",
  search_button: "Buscar",
  which_stays: "Cu\xE1l se queda",
  which_stays_hint: "Compara dos hip\xF3tesis: cu\xE1l mide mejor y qu\xE9 archivos cambiaron.",
  one_hypothesis: "Una hip\xF3tesis",
  other_hypothesis: "Otra hip\xF3tesis",
  compare_button: "Comparar",
  web_seed: "Esta hip\xF3tesis abri\xF3 la campa\xF1a. No existe una hip\xF3tesis anterior.",
  web_mutate: "Esta hip\xF3tesis cambia el c\xF3digo a partir de otra y mantiene la pregunta.",
  web_revert: "Esta hip\xF3tesis vuelve a un camino anterior. El callej\xF3n sin salida sigue registrado.",
  web_revise: "La pregunta cambi\xF3. La premisa anterior fue sustituida.",
  web_synthesize: "Esta hip\xF3tesis junta m\xE1s de un camino anterior.",
  read_failed: "No fue posible leer la campa\xF1a.",
  campaign_measure: "Medici\xF3n de esta campa\xF1a.",
  higher_sentence: "Cuanto mayor, mejor.",
  lower_sentence: "Cuanto menor, mejor.",
  unmeasured_yet: "A\xFAn sin medici\xF3n.",
  origin_kicker: "Origen",
  assumption_kicker: "Lo que esta hip\xF3tesis asume",
  intent_kicker: "Lo que se quer\xEDa obtener",
  metrics_kicker: "M\xE9tricas",
  nobody_measured: "Nadie ha medido esta hip\xF3tesis todav\xEDa.",
  left_frontier_measure: "Medici\xF3n que sali\xF3 de la frontera.",
  frontier_measure: "\xDAltima medici\xF3n usada en la frontera.",
  earlier_measure: "Medici\xF3n anterior.",
  off_frontier: "Fuera de la frontera: {reason}",
  registered_by: "Registrada por {agent}.",
  what_came_after: "Lo que vino despu\xE9s",
  no_measure_title: "Sin medici\xF3n",
  branch_seed: "origen",
  branch_mutate: "ramificaci\xF3n",
  branch_revert: "retorno",
  branch_revise: "premisa revisada",
  branch_synthesize: "s\xEDntesis",
  beaten_inline: "superada por {claim}",
  need_fact: "Necesidad",
  owner_fact: "Responsable",
  nobody: "nadie",
  agent_word: "agente",
  no_origin_yet: "A\xFAn no hay origen. La primera hip\xF3tesis abre el mapa.",
  none_survived: "Ninguna hip\xF3tesis sobrevivi\xF3 a las mediciones.",
  search_or_score: "Busque una premisa o registre una medici\xF3n.",
  opened_short: "abri\xF3 la campa\xF1a",
  came_from_inline: "vino de {claim}",
  stays_detail: "{claim} se queda. Es mejor o igual en todas las m\xE9tricas y mejor en una.",
  tie_metrics: "Empate. Las dos miden igual.",
  each_wins: "Cada una gana en una m\xE9trica distinta. La campa\xF1a a\xFAn no tiene un ganador \xFAnico.",
  metric_tie: "Empate en esta m\xE9trica.",
  text_diff: "Diferencia de texto.",
  only_in: "Este archivo solo existe en {name}.",
  no_text_diff: "El archivo no tiene diferencia de texto.",
  metric_col: "M\xE9trica",
  reading_col: "Lectura",
  first_one: "la primera",
  second_one: "la segunda",
  the_first: "La primera",
  the_second: "La segunda",
  different_text: "Texto diferente",
  added_files: "Archivos nuevos",
  removed_files: "Archivos eliminados",
  campaign_line: "Campa\xF1a {name}",
  problem: "Problema: {text}",
  need: "Necesidad: {text}",
  owner: "Responsable: {name}",
  opened_summary: "Abierta {age}. {variants} hip\xF3tesis, {trials} mediciones, {rejections} rechazos. {tokens}",
  map_label: "Mapa:",
  no_hypothesis_yet: "La campa\xF1a a\xFAn no tiene hip\xF3tesis. La primera grabaci\xF3n es el origen del problema.",
  many_origins: "La campa\xF1a tiene {count} or\xEDgenes: {claims}.",
  guide_intro: "acvc guarda hip\xF3tesis, la premisa de cada una y las mediciones. La frontera muestra las que ninguna otra super\xF3. Un ensayo peor no quita el mejor de la misma hip\xF3tesis. Rechazar es un acto humano.",
  none_stays: "Ninguna se queda.",
  metric_of_campaign: "medici\xF3n de la campa\xF1a",
  no_campaign: "sin campa\xF1a",
  campaigns: "Campa\xF1as",
  frontier_screen: "Frontera",
  lineage_screen: "Linaje",
  premise_screen: "Premisa",
  compare_screen: "Comparar",
  recall_screen: "B\xFAsqueda",
  help_search: "Enter busca. Esc cancela. Retroceso borra.",
  help_campaigns: "Enter abre la frontera. \u2190 \u2192 cambia la pantalla. q sale.",
  help_frontier: "Enter abre la premisa. d compara dos. \u2190 \u2192 cambia la pantalla. q sale.",
  help_lineage: "Enter abre esa hip\xF3tesis. \u2190 \u2192 cambia la pantalla. q sale.",
  help_detail: "La hip\xF3tesis y las mediciones. \u2190 \u2192 cambia la pantalla. q sale.",
  help_diff: "Enter muestra el archivo. \u2190 \u2192 cambia la pantalla. q sale.",
  no_campaigns: "Ninguna campa\xF1a.",
  open_lineage: "Abra una hip\xF3tesis en la frontera para ver de d\xF3nde vino.",
  no_before_after: "No hay hip\xF3tesis anterior ni lo que vino despu\xE9s.",
  intent_gap: "Intenci\xF3n  {intent}",
  assumptions_gap: "Suposiciones  {items}",
  came_then: "Vino de {from}    Despu\xE9s {after}",
  recall_prompt: "Escriba una palabra de la premisa y pulse Enter.",
  came_from_one: "vino de {claim}",
  pick_campaign: "Elija una campa\xF1a.",
  first_marked: "Primera hip\xF3tesis marcada. Pulse d en la otra.",
  first_named: "Primera: {claim}. Pulse d en la otra.",
  ago_under_minute: "hace menos de un minuto",
  ago_minutes: "hace {count} min",
  ago_hours: "hace {count} h",
  ago_days: "hace {count} d\xEDas",
  no_tokens: "Ning\xFAn token registrado. Grabe el consumo con spend, sin cambiar la identidad de la hip\xF3tesis.",
  tokens_recorded: "{tokens} tokens ya registrados",
  rejection_label: "rechazo: {reason}",
  guide_example: "El mapa de la campa\xF1a abierta. Sin repositorio, no hay ejemplo.",
  guide_text: "Lea context de la campa\xF1a abierta. El mapa de ese repositorio es el ejemplo.\nLa premisa es el problema. La causa queda en la hip\xF3tesis, no en una sola frase de la campa\xF1a.\nscore graba la medici\xF3n. Un ensayo peor no quita el mejor de la frontera.\nseed abre la campa\xF1a en la carpeta de la hip\xF3tesis. El recorrido no lee el .gitignore. Un .acvcignore en la carpeta quita los mismos caminos en todo clon. El seed dice cu\xE1ntos archivos entraron y cu\xE1les pesaron m\xE1s. El store queda en --repo; en la carpeta quedan los dos marcadores, en el .gitignore. La hip\xF3tesis siguiente usa record y tiene padre.\n",
  status_frontier_why: "Medida, y ninguna otra la supera.",
  status_beaten_why: "Medida, y otra hip\xF3tesis de la frontera es mejor.",
  status_rejected_why: "El motivo est\xE1 en el grafo.",
  status_unmeasured_why: "A\xFAn no hay un score completo para la m\xE9trica de la campa\xF1a.",
  when_context: "Antes de actuar. Devuelve el mapa de esta campa\xF1a.",
  when_guide: "Los comandos, m\xE1s el mapa del repositorio abierto.",
  when_frontier: "Solo lo que la medici\xF3n a\xFAn no super\xF3.",
  when_why: "Una hip\xF3tesis, con la causa, la medici\xF3n y qui\xE9n la super\xF3.",
  when_recall: "Encontrar una premisa por el texto. Devuelve el estado y el padre.",
  when_seed: "La primera hip\xF3tesis de una campa\xF1a. Siembre la carpeta de la hip\xF3tesis: el recorrido no lee el .gitignore y salta .git, .acvc, target, node_modules, .build y DerivedData. Un .acvcignore en la carpeta quita caminos iguales en todo clon; sin \xE9l, vale la lista fija. El seed dice cu\xE1ntos archivos entraron y cu\xE1les pesaron m\xE1s. El store queda en --repo. En la carpeta quedan .acvc-workspace y .acvc-index, ambos en el .gitignore.",
  when_record: "La carpeta cambi\xF3. La nueva hip\xF3tesis apunta al padre.",
  when_score: "Hay un n\xFAmero de la m\xE9trica de la campa\xF1a. Un ensayo peor no sustituye a uno mejor.",
  when_reject: "La hip\xF3tesis muri\xF3 y el motivo queda en el grafo.",
  when_spend: "La sesi\xF3n sabe cu\xE1ntos tokens consumi\xF3.",
  when_compare: "Dos hip\xF3tesis medidas, para ver cu\xE1l se queda y qu\xE9 archivos cambiaron.",
  when_web: "El mapa en la p\xE1gina.",
  when_obsidian: "Instala el visor en el vault y apunta a este repositorio. El comando en Obsidian es Open ACVC.",
  none_count: "ninguno",
  help_recall: "/ busca. Enter busca. \u2190 \u2192 cambia la pantalla. q sale.",
  came_label: "Vino de",
  problem_label: "Problema",
  better_here: "{name} est\xE1 mejor aqu\xED.",
  only_side: "Solo en {name}",
  web_campaign_fact: "{age} \xB7 {variants} hip\xF3tesis \xB7 {trials} mediciones \xB7 {tokens}",
  only_other: "solo en la otra  {path}",
  only_first: "solo en la primera  {path}",
  changed_path: "cambi\xF3  {path}",
  the_other: "La otra",
  after_named: "Lo que vino despu\xE9s: {names}",
  assistant_pack: "El contrato para otro agente sale de `acvc assistant`, en el idioma de este computador. Quien tiene el MCP de ASB env\xEDa el array documents a rag.index y despu\xE9s busca con rag.search.",
  assistant_index: "Si esta sesi\xF3n tiene el MCP de ASB, env\xEDe el array documents a rag.index una vez. La b\xFAsqueda siguiente usa rag.search.",
  when_assistant: "Devuelve el contrato operacional en el idioma de este computador, listo para rag.index."
};

// ../../locales/fr.json
var fr_default = {
  on_frontier: "Sur la fronti\xE8re.",
  motive_unrecorded: "Le motif n'a pas \xE9t\xE9 enregistr\xE9.",
  weighed: "pesait {path} {bytes}",
  no_frontier: "Aucune hypoth\xE8se n'est sur la fronti\xE8re.",
  no_recall: "Aucune pr\xE9misse ne contient ce texte.",
  opened_campaign: "Cette hypoth\xE8se a ouvert la campagne.",
  came_from: "Vient de : {names}",
  stays: "{claim} reste.",
  tie_both: "{left} et {right} mesurent pareil.",
  incomparable: "Aucune ne reste seule. Chacune gagne sur une m\xE9trique diff\xE9rente.",
  missing_measure: "Il manque une mesure pour d\xE9cider.",
  missing_measure_which: "Il manque une mesure pour d\xE9cider laquelle reste.",
  no_measurement: "sans mesure",
  cause: "Cause : {items}",
  intent: "Intention : {intent}",
  measurement: "Mesure : {text}",
  rejected: "Rejet\xE9e : {reason}",
  rejected_gap: "Rejet\xE9e  {reason}",
  beaten_by: "D\xE9pass\xE9e par {claim}.",
  beaten_by_gap: "D\xE9pass\xE9e par  {claim}.",
  higher_better: "plus c'est \xE9lev\xE9, mieux c'est",
  lower_better: "plus c'est bas, mieux c'est",
  agent: "Agent : {agent}",
  agent_gap: "Agent  {agent}",
  no_scored: "Aucune hypoth\xE8se n'a de score.",
  appears_after_score: "Elle appara\xEEt ici apr\xE8s acvc score.",
  open_hypothesis: "Ouvrez une hypoth\xE8se sur la fronti\xE8re.",
  score_heading: "Score",
  still_unmeasured: "    encore sans mesure",
  frontier_hint: "Sur la fronti\xE8re, appuyez sur d pour deux hypoth\xE8ses.",
  nothing_compared: "Rien n'a encore \xE9t\xE9 compar\xE9.",
  search_label: "Recherche  {query}",
  none_chosen: "Aucune hypoth\xE8se choisie.",
  came_from_short: "vient de  {claim}",
  then_short: "ensuite  {claim}",
  origin_prefix: "Origine  {text}",
  origin_seed: "Elle a commenc\xE9 la campagne. Il n'y a pas d'hypoth\xE8se pr\xE9c\xE9dente.",
  origin_mutate: "Un changement \xE0 partir d'une autre hypoth\xE8se.",
  origin_revert: "Un retour \xE0 une hypoth\xE8se pr\xE9c\xE9dente.",
  origin_revise: "La pr\xE9misse a \xE9t\xE9 r\xE9\xE9crite.",
  origin_synthesize: "Synth\xE8se de plusieurs hypoth\xE8ses.",
  agent_unknown: "non indiqu\xE9",
  status_frontier: "sur la fronti\xE8re",
  status_beaten: "d\xE9pass\xE9e",
  status_rejected: "rejet\xE9e",
  different_files: "fichiers diff\xE9rents {count}",
  against: "{name} {loser} contre {winner} ({sense})",
  title: "Ce qui vaut encore d'\xEAtre explor\xE9",
  lede: "Une campagne compare des hypoth\xE8ses par la pr\xE9misse et les mesures. La fronti\xE8re montre celles qu'aucune autre n'a d\xE9pass\xE9es.",
  campaign_label: "Campagne",
  frontier_heading: "Sur la fronti\xE8re",
  frontier_hint_page: "Pr\xE9misse et mesures de ce que la mesure n'a pas encore \xE9cart\xE9.",
  hypothesis_heading: "L'hypoth\xE8se",
  why_hint: "Choisissez une hypoth\xE8se pour voir l'origine, les hypoth\xE8ses et les mesures.",
  lineage_heading: "Origine et ramifications",
  lineage_hint: "Le probl\xE8me, qui r\xE9pond, depuis combien de temps la campagne est ouverte, et chaque chemin avec son r\xE9sultat.",
  search_heading: "Chercher",
  search_hint: "Trouve des hypoth\xE8ses par la phrase de la pr\xE9misse, y compris celles qui ont quitt\xE9 la fronti\xE8re.",
  premise_text: "Texte de la pr\xE9misse",
  premise_placeholder: "ce que l'hypoth\xE8se affirme",
  search_button: "Chercher",
  which_stays: "Laquelle reste",
  which_stays_hint: "Compare deux hypoth\xE8ses : laquelle mesure mieux et quels fichiers ont chang\xE9.",
  one_hypothesis: "Une hypoth\xE8se",
  other_hypothesis: "Une autre hypoth\xE8se",
  compare_button: "Comparer",
  web_seed: "Cette hypoth\xE8se a ouvert la campagne. Il n'existe pas d'hypoth\xE8se pr\xE9c\xE9dente.",
  web_mutate: "Cette hypoth\xE8se change le code \xE0 partir d'une autre et garde la question.",
  web_revert: "Cette hypoth\xE8se revient \xE0 un chemin pr\xE9c\xE9dent. L'impasse reste enregistr\xE9e.",
  web_revise: "La question a chang\xE9. La pr\xE9misse pr\xE9c\xE9dente a \xE9t\xE9 remplac\xE9e.",
  web_synthesize: "Cette hypoth\xE8se r\xE9unit plus d'un chemin pr\xE9c\xE9dent.",
  read_failed: "Impossible de lire la campagne.",
  campaign_measure: "Mesure de cette campagne.",
  higher_sentence: "Plus c'est \xE9lev\xE9, mieux c'est.",
  lower_sentence: "Plus c'est bas, mieux c'est.",
  unmeasured_yet: "Encore sans mesure.",
  origin_kicker: "Origine",
  assumption_kicker: "Ce que cette hypoth\xE8se suppose",
  intent_kicker: "Ce que l'on voulait obtenir",
  metrics_kicker: "M\xE9triques",
  nobody_measured: "Personne n'a encore mesur\xE9 cette hypoth\xE8se.",
  left_frontier_measure: "Mesure qui a quitt\xE9 la fronti\xE8re.",
  frontier_measure: "Derni\xE8re mesure utilis\xE9e sur la fronti\xE8re.",
  earlier_measure: "Mesure pr\xE9c\xE9dente.",
  off_frontier: "Hors de la fronti\xE8re : {reason}",
  registered_by: "Enregistr\xE9e par {agent}.",
  what_came_after: "Ce qui est venu ensuite",
  no_measure_title: "Sans mesure",
  branch_seed: "origine",
  branch_mutate: "ramification",
  branch_revert: "retour",
  branch_revise: "pr\xE9misse r\xE9vis\xE9e",
  branch_synthesize: "synth\xE8se",
  beaten_inline: "d\xE9pass\xE9e par {claim}",
  need_fact: "Besoin",
  owner_fact: "Responsable",
  nobody: "personne",
  agent_word: "agent",
  no_origin_yet: "Il n'y a pas encore d'origine. La premi\xE8re hypoth\xE8se ouvre la carte.",
  none_survived: "Aucune hypoth\xE8se n'a surv\xE9cu aux mesures.",
  search_or_score: "Cherchez une pr\xE9misse ou enregistrez une mesure.",
  opened_short: "a ouvert la campagne",
  came_from_inline: "vient de {claim}",
  stays_detail: "{claim} reste. Elle est meilleure ou \xE9gale sur toutes les m\xE9triques et meilleure sur une.",
  tie_metrics: "\xC9galit\xE9. Les deux mesurent pareil.",
  each_wins: "Chacune gagne sur une m\xE9trique diff\xE9rente. La campagne n'a pas encore un vainqueur unique.",
  metric_tie: "\xC9galit\xE9 sur cette m\xE9trique.",
  text_diff: "Diff\xE9rence de texte.",
  only_in: "Ce fichier n'existe que dans {name}.",
  no_text_diff: "Le fichier n'a pas de diff\xE9rence de texte.",
  metric_col: "M\xE9trique",
  reading_col: "Lecture",
  first_one: "la premi\xE8re",
  second_one: "la seconde",
  the_first: "La premi\xE8re",
  the_second: "La seconde",
  different_text: "Texte diff\xE9rent",
  added_files: "Fichiers nouveaux",
  removed_files: "Fichiers retir\xE9s",
  campaign_line: "Campagne {name}",
  problem: "Probl\xE8me : {text}",
  need: "Besoin : {text}",
  owner: "Responsable : {name}",
  opened_summary: "Ouverte {age}. {variants} hypoth\xE8ses, {trials} mesures, {rejections} rejets. {tokens}",
  map_label: "Carte :",
  no_hypothesis_yet: "La campagne n'a pas encore d'hypoth\xE8se. Le premier enregistrement est l'origine du probl\xE8me.",
  many_origins: "La campagne a {count} origines : {claims}.",
  guide_intro: "acvc garde les hypoth\xE8ses, la pr\xE9misse de chacune et les mesures. La fronti\xE8re montre celles qu'aucune autre n'a d\xE9pass\xE9es. Un essai pire ne retire pas le meilleur de la m\xEAme hypoth\xE8se. Rejeter est un acte humain.",
  none_stays: "Aucune ne reste.",
  metric_of_campaign: "mesure de la campagne",
  no_campaign: "sans campagne",
  campaigns: "Campagnes",
  frontier_screen: "Fronti\xE8re",
  lineage_screen: "Lign\xE9e",
  premise_screen: "Pr\xE9misse",
  compare_screen: "Comparer",
  recall_screen: "Recherche",
  help_search: "Entr\xE9e cherche. \xC9chap annule. Retour arri\xE8re efface.",
  help_campaigns: "Entr\xE9e ouvre la fronti\xE8re. \u2190 \u2192 change l'\xE9cran. q quitte.",
  help_frontier: "Entr\xE9e ouvre la pr\xE9misse. d compare deux. \u2190 \u2192 change l'\xE9cran. q quitte.",
  help_lineage: "Entr\xE9e ouvre cette hypoth\xE8se. \u2190 \u2192 change l'\xE9cran. q quitte.",
  help_detail: "L'hypoth\xE8se et les mesures. \u2190 \u2192 change l'\xE9cran. q quitte.",
  help_diff: "Entr\xE9e montre le fichier. \u2190 \u2192 change l'\xE9cran. q quitte.",
  no_campaigns: "Aucune campagne.",
  open_lineage: "Ouvrez une hypoth\xE8se sur la fronti\xE8re pour voir d'o\xF9 elle vient.",
  no_before_after: "Il n'y a pas d'hypoth\xE8se pr\xE9c\xE9dente ni de suite.",
  intent_gap: "Intention  {intent}",
  assumptions_gap: "Suppositions  {items}",
  came_then: "Vient de {from}    Ensuite {after}",
  recall_prompt: "\xC9crivez un mot de la pr\xE9misse et appuyez sur Entr\xE9e.",
  came_from_one: "vient de {claim}",
  pick_campaign: "Choisissez une campagne.",
  first_marked: "Premi\xE8re hypoth\xE8se marqu\xE9e. Appuyez sur d pour l'autre.",
  first_named: "Premi\xE8re : {claim}. Appuyez sur d pour l'autre.",
  ago_under_minute: "il y a moins d'une minute",
  ago_minutes: "il y a {count} min",
  ago_hours: "il y a {count} h",
  ago_days: "il y a {count} jours",
  no_tokens: "Aucun jeton enregistr\xE9. Enregistrez la consommation avec spend, sans changer l'identit\xE9 de l'hypoth\xE8se.",
  tokens_recorded: "{tokens} jetons d\xE9j\xE0 enregistr\xE9s",
  rejection_label: "rejet : {reason}",
  guide_example: "La carte de la campagne ouverte. Sans d\xE9p\xF4t, il n'y a pas d'exemple.",
  guide_text: "Lisez context de la campagne ouverte. La carte de ce d\xE9p\xF4t est l'exemple.\nLa pr\xE9misse est le probl\xE8me. La cause reste sur l'hypoth\xE8se, pas dans une seule phrase de campagne.\nscore enregistre la mesure. Un essai pire ne retire pas le meilleur de la fronti\xE8re.\nseed ouvre la campagne dans le dossier de l'hypoth\xE8se. Le parcours ne lit pas le .gitignore. Un .acvcignore dans le dossier retire les m\xEAmes chemins dans chaque clone. Le seed dit combien de fichiers sont entr\xE9s et lesquels pesaient le plus. Le store reste dans --repo ; le dossier garde les deux marqueurs, dans le .gitignore. L'hypoth\xE8se suivante utilise record et a un parent.\n",
  status_frontier_why: "Mesur\xE9e, et aucune autre ne la d\xE9passe.",
  status_beaten_why: "Mesur\xE9e, et une autre hypoth\xE8se de la fronti\xE8re est meilleure.",
  status_rejected_why: "Le motif est dans le graphe.",
  status_unmeasured_why: "Il n'y a pas encore de score complet pour la m\xE9trique de la campagne.",
  when_context: "Avant d'agir. Renvoie la carte de cette campagne.",
  when_guide: "Les commandes, plus la carte du d\xE9p\xF4t ouvert.",
  when_frontier: "Seulement ce que la mesure n'a pas encore d\xE9pass\xE9.",
  when_why: "Une hypoth\xE8se, avec la cause, la mesure et qui l'a d\xE9pass\xE9e.",
  when_recall: "Trouver une pr\xE9misse par le texte. Renvoie le statut et le parent.",
  when_seed: "La premi\xE8re hypoth\xE8se d'une campagne. Semez le dossier de l'hypoth\xE8se : le parcours ne lit pas le .gitignore et saute .git, .acvc, target, node_modules, .build et DerivedData. Un .acvcignore dans le dossier retire les m\xEAmes chemins dans chaque clone ; sans lui, la liste fixe s'applique. Le seed dit combien de fichiers sont entr\xE9s et lesquels pesaient le plus. Le store reste dans --repo. Le dossier garde .acvc-workspace et .acvc-index, tous deux dans le .gitignore.",
  when_record: "Le dossier a chang\xE9. La nouvelle hypoth\xE8se pointe vers le parent.",
  when_score: "Il y a un nombre pour la m\xE9trique de la campagne. Un essai pire ne remplace pas un meilleur.",
  when_reject: "L'hypoth\xE8se est morte et le motif reste dans le graphe.",
  when_spend: "La session sait combien de jetons elle a consomm\xE9s.",
  when_compare: "Deux hypoth\xE8ses mesur\xE9es, pour voir laquelle reste et quels fichiers ont chang\xE9.",
  when_web: "La carte sur la page.",
  when_obsidian: "Installe le visualiseur dans le vault et pointe ce d\xE9p\xF4t. La commande dans Obsidian est Open ACVC.",
  none_count: "aucun",
  help_recall: "/ cherche. Entr\xE9e cherche. \u2190 \u2192 change l'\xE9cran. q quitte.",
  came_label: "Vient de",
  problem_label: "Probl\xE8me",
  better_here: "{name} est meilleur ici.",
  only_side: "Seulement dans {name}",
  web_campaign_fact: "{age} \xB7 {variants} hypoth\xE8ses \xB7 {trials} mesures \xB7 {tokens}",
  only_other: "seulement dans l'autre  {path}",
  only_first: "seulement dans la premi\xE8re  {path}",
  changed_path: "a chang\xE9  {path}",
  the_other: "L'autre",
  after_named: "Ce qui est venu ensuite : {names}",
  assistant_pack: "Le contrat pour un autre agent sort de `acvc assistant`, dans la langue de cet ordinateur. Un agent qui a le MCP d'ASB envoie le tableau documents \xE0 rag.index, puis cherche avec rag.search.",
  assistant_index: "Si cette session a le MCP d'ASB, envoyez le tableau documents \xE0 rag.index une fois. La recherche suivante utilise rag.search.",
  when_assistant: "Renvoie le contrat op\xE9rationnel dans la langue de cet ordinateur, pr\xEAt pour rag.index."
};

// ../../locales/nl.json
var nl_default = {
  on_frontier: "Op de grens.",
  motive_unrecorded: "Het motief is niet vastgelegd.",
  weighed: "woog {path} {bytes}",
  no_frontier: "Geen hypothese staat op de grens.",
  no_recall: "Geen premisse bevat die tekst.",
  opened_campaign: "Deze hypothese opende de campagne.",
  came_from: "Kwam van: {names}",
  stays: "{claim} blijft.",
  tie_both: "{left} en {right} meten hetzelfde.",
  incomparable: "Geen van beide blijft alleen. Elk wint op een andere maat.",
  missing_measure: "Er ontbreekt een meting om te beslissen.",
  missing_measure_which: "Er ontbreekt een meting om te bepalen welke blijft.",
  no_measurement: "geen meting",
  cause: "Oorzaak: {items}",
  intent: "Bedoeling: {intent}",
  measurement: "Meting: {text}",
  rejected: "Afgewezen: {reason}",
  rejected_gap: "Afgewezen  {reason}",
  beaten_by: "Voorbijgestreefd door {claim}.",
  beaten_by_gap: "Voorbijgestreefd door  {claim}.",
  higher_better: "hoger is beter",
  lower_better: "lager is beter",
  agent: "Agent: {agent}",
  agent_gap: "Agent  {agent}",
  no_scored: "Geen hypothese is gescoord.",
  appears_after_score: "Ze verschijnt hier na acvc score.",
  open_hypothesis: "Open een hypothese op de grens.",
  score_heading: "Score",
  still_unmeasured: "    nog geen meting",
  frontier_hint: "Op de grens, druk op d bij twee hypothesen.",
  nothing_compared: "Nog niets vergeleken.",
  search_label: "Zoeken  {query}",
  none_chosen: "Geen hypothese gekozen.",
  came_from_short: "kwam van  {claim}",
  then_short: "daarna  {claim}",
  origin_prefix: "Oorsprong  {text}",
  origin_seed: "Ze begon de campagne. Er is geen eerdere hypothese.",
  origin_mutate: "Een wijziging vanuit een andere hypothese.",
  origin_revert: "Een terugkeer naar een eerdere hypothese.",
  origin_revise: "De premisse is herschreven.",
  origin_synthesize: "Een synthese van meerdere hypothesen.",
  agent_unknown: "niet opgegeven",
  status_frontier: "op de grens",
  status_beaten: "voorbijgestreefd",
  status_rejected: "afgewezen",
  different_files: "verschillende bestanden {count}",
  against: "{name} {loser} tegen {winner} ({sense})",
  title: "Wat nog de moeite is om te verkennen",
  lede: "Een campagne vergelijkt hypothesen op premisse en meting. De grens toont die welke geen andere heeft overtroffen.",
  campaign_label: "Campagne",
  frontier_heading: "Op de grens",
  frontier_hint_page: "Premisse en maten van wat de meting nog niet terzijde heeft gelegd.",
  hypothesis_heading: "De hypothese",
  why_hint: "Kies een hypothese om oorsprong, aannames en metingen te zien.",
  lineage_heading: "Oorsprong en vertakkingen",
  lineage_hint: "Het probleem, wie verantwoordelijk is, hoe lang de campagne open is, en elk pad met het resultaat.",
  search_heading: "Zoeken",
  search_hint: "Vindt hypothesen via de premissezin, ook die de grens hebben verlaten.",
  premise_text: "Premissetekst",
  premise_placeholder: "wat de hypothese beweert",
  search_button: "Zoeken",
  which_stays: "Welke blijft",
  which_stays_hint: "Vergelijkt twee hypothesen: welke beter meet en welke bestanden veranderden.",
  one_hypothesis: "E\xE9n hypothese",
  other_hypothesis: "Een andere hypothese",
  compare_button: "Vergelijken",
  web_seed: "Deze hypothese opende de campagne. Er is geen eerdere hypothese.",
  web_mutate: "Deze hypothese wijzigt de code vanuit een andere en houdt de vraag.",
  web_revert: "Deze hypothese keert terug naar een eerder pad. De doodlopende weg blijft vastgelegd.",
  web_revise: "De vraag veranderde. De eerdere premisse is vervangen.",
  web_synthesize: "Deze hypothese voegt meer dan \xE9\xE9n eerder pad samen.",
  read_failed: "De campagne kon niet worden gelezen.",
  campaign_measure: "Meting van deze campagne.",
  higher_sentence: "Hoger is beter.",
  lower_sentence: "Lager is beter.",
  unmeasured_yet: "Nog geen meting.",
  origin_kicker: "Oorsprong",
  assumption_kicker: "Wat deze hypothese aanneemt",
  intent_kicker: "Wat men wilde bereiken",
  metrics_kicker: "Metrieken",
  nobody_measured: "Niemand heeft deze hypothese nog gemeten.",
  left_frontier_measure: "Meting die de grens verliet.",
  frontier_measure: "Laatste meting gebruikt op de grens.",
  earlier_measure: "Eerdere meting.",
  off_frontier: "Buiten de grens: {reason}",
  registered_by: "Vastgelegd door {agent}.",
  what_came_after: "Wat daarna kwam",
  no_measure_title: "Geen meting",
  branch_seed: "oorsprong",
  branch_mutate: "vertakking",
  branch_revert: "terugkeer",
  branch_revise: "premisse herzien",
  branch_synthesize: "synthese",
  beaten_inline: "voorbijgestreefd door {claim}",
  need_fact: "Behoefte",
  owner_fact: "Verantwoordelijke",
  nobody: "niemand",
  agent_word: "agent",
  no_origin_yet: "Er is nog geen oorsprong. De eerste hypothese opent de kaart.",
  none_survived: "Geen hypothese heeft de metingen overleefd.",
  search_or_score: "Zoek een premisse of leg een meting vast.",
  opened_short: "opende de campagne",
  came_from_inline: "kwam van {claim}",
  stays_detail: "{claim} blijft. Ze is beter of gelijk op elke maat en beter op \xE9\xE9n.",
  tie_metrics: "Gelijkspel. De twee meten hetzelfde.",
  each_wins: "Elk wint op een andere maat. De campagne heeft nog geen enkele winnaar.",
  metric_tie: "Gelijkspel op deze maat.",
  text_diff: "Tekstverschil.",
  only_in: "Dit bestand bestaat alleen in {name}.",
  no_text_diff: "Het bestand heeft geen tekstverschil.",
  metric_col: "Maat",
  reading_col: "Lezing",
  first_one: "de eerste",
  second_one: "de tweede",
  the_first: "De eerste",
  the_second: "De tweede",
  different_text: "Afwijkende tekst",
  added_files: "Nieuwe bestanden",
  removed_files: "Verwijderde bestanden",
  campaign_line: "Campagne {name}",
  problem: "Probleem: {text}",
  need: "Behoefte: {text}",
  owner: "Verantwoordelijke: {name}",
  opened_summary: "Open {age}. {variants} hypothesen, {trials} metingen, {rejections} afwijzingen. {tokens}",
  map_label: "Kaart:",
  no_hypothesis_yet: "De campagne heeft nog geen hypothese. De eerste opname is de oorsprong van het probleem.",
  many_origins: "De campagne heeft {count} oorsprongen: {claims}.",
  guide_intro: "acvc bewaart hypothesen, de premisse van elk en de metingen. De grens toont die welke geen andere heeft overtroffen. Een slechtere proef haalt de betere van dezelfde hypothese niet weg. Afwijzen is een menselijke daad.",
  none_stays: "Geen van beide blijft.",
  metric_of_campaign: "meting van de campagne",
  no_campaign: "geen campagne",
  campaigns: "Campagnes",
  frontier_screen: "Grens",
  lineage_screen: "Afstamming",
  premise_screen: "Premisse",
  compare_screen: "Vergelijken",
  recall_screen: "Zoeken",
  help_search: "Enter zoekt. Esc annuleert. Backspace wist.",
  help_campaigns: "Enter opent de grens. \u2190 \u2192 wisselt het scherm. q stopt.",
  help_frontier: "Enter opent de premisse. d vergelijkt twee. \u2190 \u2192 wisselt het scherm. q stopt.",
  help_lineage: "Enter opent die hypothese. \u2190 \u2192 wisselt het scherm. q stopt.",
  help_detail: "De hypothese en de metingen. \u2190 \u2192 wisselt het scherm. q stopt.",
  help_diff: "Enter toont het bestand. \u2190 \u2192 wisselt het scherm. q stopt.",
  no_campaigns: "Geen campagne.",
  open_lineage: "Open een hypothese op de grens om te zien waar ze vandaan komt.",
  no_before_after: "Er is geen eerdere hypothese en er kwam niets daarna.",
  intent_gap: "Bedoeling  {intent}",
  assumptions_gap: "Aannames  {items}",
  came_then: "Kwam van {from}    Daarna {after}",
  recall_prompt: "Typ een woord uit de premisse en druk op Enter.",
  came_from_one: "kwam van {claim}",
  pick_campaign: "Kies een campagne.",
  first_marked: "Eerste hypothese gemarkeerd. Druk op d bij de andere.",
  first_named: "Eerste: {claim}. Druk op d bij de andere.",
  ago_under_minute: "minder dan een minuut geleden",
  ago_minutes: "{count} min geleden",
  ago_hours: "{count} u geleden",
  ago_days: "{count} dagen geleden",
  no_tokens: "Geen tokens vastgelegd. Leg het verbruik vast met spend, zonder de identiteit van de hypothese te wijzigen.",
  tokens_recorded: "{tokens} tokens al vastgelegd",
  rejection_label: "afwijzing: {reason}",
  guide_example: "De kaart van de open campagne. Zonder repository is er geen voorbeeld.",
  guide_text: "Lees context van de open campagne. De kaart van die repository is het voorbeeld.\nDe premisse is het probleem. De oorzaak blijft bij de hypothese, niet in \xE9\xE9n campagnezin.\nscore legt de meting vast. Een slechtere proef haalt de betere niet van de grens.\nseed opent de campagne in de map van de hypothese. De scan leest .gitignore niet. Een .acvcignore in de map laat dezelfde paden weg in elke kloon. De seed zegt hoeveel bestanden binnenkwamen en welke zwaarder wogen. De store blijft in --repo; de map houdt de twee markeringen, in .gitignore. De volgende hypothese gebruikt record en heeft een ouder.\n",
  status_frontier_why: "Gemeten, en geen andere overtreft haar.",
  status_beaten_why: "Gemeten, en een andere hypothese op de grens is beter.",
  status_rejected_why: "Het motief staat in de graaf.",
  status_unmeasured_why: "Er is nog geen volledige score voor de maat van de campagne.",
  when_context: "Voor het handelen. Geeft de kaart van deze campagne terug.",
  when_guide: "De commando's, plus de kaart van de open repository.",
  when_frontier: "Alleen wat de meting nog niet heeft overtroffen.",
  when_why: "E\xE9n hypothese, met de oorzaak, de meting en wie haar overtrof.",
  when_recall: "Een premisse vinden via tekst. Geeft de status en de ouder terug.",
  when_seed: "De eerste hypothese van een campagne. Zaai de map van de hypothese: de scan leest .gitignore niet en slaat .git, .acvc, target, node_modules, .build en DerivedData over. Een .acvcignore in de map laat dezelfde paden weg in elke kloon; zonder geldt de vaste lijst. De seed zegt hoeveel bestanden binnenkwamen en welke zwaarder wogen. De store blijft in --repo. De map houdt .acvc-workspace en .acvc-index, beide in .gitignore.",
  when_record: "De map veranderde. De nieuwe hypothese wijst naar de ouder.",
  when_score: "Er is een getal voor de maat van de campagne. Een slechtere proef vervangt geen betere.",
  when_reject: "De hypothese is gestorven en het motief blijft in de graaf.",
  when_spend: "De sessie weet hoeveel tokens ze verbruikte.",
  when_compare: "Twee gemeten hypothesen, om te zien welke blijft en welke bestanden veranderden.",
  when_web: "De kaart op de pagina.",
  when_obsidian: "Installeert de viewer in de vault en wijst naar deze repository. Het commando in Obsidian is Open ACVC.",
  none_count: "geen",
  help_recall: "/ zoekt. Enter zoekt. \u2190 \u2192 wisselt het scherm. q stopt.",
  came_label: "Kwam van",
  problem_label: "Probleem",
  better_here: "{name} is hier beter.",
  only_side: "Alleen in {name}",
  web_campaign_fact: "{age} \xB7 {variants} hypothesen \xB7 {trials} metingen \xB7 {tokens}",
  only_other: "alleen in de andere  {path}",
  only_first: "alleen in de eerste  {path}",
  changed_path: "gewijzigd  {path}",
  the_other: "De andere",
  after_named: "Wat daarna kwam: {names}",
  assistant_pack: "Het contract voor een andere agent komt uit `acvc assistant`, in de taal van deze computer. Een agent met de ASB-MCP stuurt de array documents naar rag.index en zoekt daarna met rag.search.",
  assistant_index: "Als deze sessie de ASB-MCP heeft, stuur de array documents \xE9\xE9n keer naar rag.index. Zoeken daarna gaat met rag.search.",
  when_assistant: "Geeft het operationele contract terug in de taal van deze computer, klaar voor rag.index."
};

// ../../locales/pl.json
var pl_default = {
  on_frontier: "Na granicy.",
  motive_unrecorded: "Motyw nie zosta\u0142 zapisany.",
  weighed: "wa\u017Cy\u0142o {path} {bytes}",
  no_frontier: "\u017Badna hipoteza nie jest na granicy.",
  no_recall: "\u017Badna przes\u0142anka nie zawiera tego tekstu.",
  opened_campaign: "Ta hipoteza otworzy\u0142a kampani\u0119.",
  came_from: "Pochodzi od: {names}",
  stays: "Zostaje: {claim}.",
  tie_both: "{left} i {right} mierz\u0105 tak samo.",
  incomparable: "\u017Badna nie zostaje sama. Ka\u017Cda wygrywa w innej mierze.",
  missing_measure: "Brakuje pomiaru, \u017Ceby rozstrzygn\u0105\u0107.",
  missing_measure_which: "Brakuje pomiaru, \u017Ceby rozstrzygn\u0105\u0107, kt\xF3ra zostaje.",
  no_measurement: "bez pomiaru",
  cause: "Przyczyna: {items}",
  intent: "Intencja: {intent}",
  measurement: "Pomiar: {text}",
  rejected: "Odrzucona: {reason}",
  rejected_gap: "Odrzucona  {reason}",
  beaten_by: "Pokonana przez {claim}.",
  beaten_by_gap: "Pokonana przez  {claim}.",
  higher_better: "im wi\u0119cej, tym lepiej",
  lower_better: "im mniej, tym lepiej",
  agent: "Agent: {agent}",
  agent_gap: "Agent  {agent}",
  no_scored: "\u017Badna hipoteza nie ma wyniku.",
  appears_after_score: "Pojawia si\u0119 tutaj po acvc score.",
  open_hypothesis: "Otw\xF3rz hipotez\u0119 na granicy.",
  score_heading: "Wynik",
  still_unmeasured: "    nadal bez pomiaru",
  frontier_hint: "Na granicy naci\u015Bnij d na dw\xF3ch hipotezach.",
  nothing_compared: "Jeszcze nic nie por\xF3wnano.",
  search_label: "Szukaj  {query}",
  none_chosen: "Nie wybrano hipotezy.",
  came_from_short: "pochodzi od  {claim}",
  then_short: "potem  {claim}",
  origin_prefix: "Pochodzenie  {text}",
  origin_seed: "Zacz\u0119\u0142a kampani\u0119. Nie ma wcze\u015Bniejszej hipotezy.",
  origin_mutate: "Zmiana wzgl\u0119dem innej hipotezy.",
  origin_revert: "Powr\xF3t do wcze\u015Bniejszej hipotezy.",
  origin_revise: "Przes\u0142anka zosta\u0142a napisana od nowa.",
  origin_synthesize: "Synteza kilku hipotez.",
  agent_unknown: "nie podano",
  status_frontier: "na granicy",
  status_beaten: "pokonana",
  status_rejected: "odrzucona",
  different_files: "r\xF3\u017Cne pliki {count}",
  against: "{name} {loser} przeciw {winner} ({sense})",
  title: "Co jeszcze warto zbada\u0107",
  lede: "Kampania por\xF3wnuje hipotezy wed\u0142ug przes\u0142anki i pomiar\xF3w. Granica pokazuje te, kt\xF3rych \u017Cadna inna nie pokona\u0142a.",
  campaign_label: "Kampania",
  frontier_heading: "Na granicy",
  frontier_hint_page: "Przes\u0142anka i miary tego, czego pomiar jeszcze nie odrzuci\u0142.",
  hypothesis_heading: "Hipoteza",
  why_hint: "Wybierz hipotez\u0119, aby zobaczy\u0107 pochodzenie, za\u0142o\u017Cenia i pomiary.",
  lineage_heading: "Pochodzenie i rozga\u0142\u0119zienia",
  lineage_hint: "Problem, kto odpowiada, jak d\u0142ugo kampania jest otwarta i ka\u017Cda \u015Bcie\u017Cka z wynikiem.",
  search_heading: "Szukaj",
  search_hint: "Znajduje hipotezy po zdaniu przes\u0142anki, tak\u017Ce te, kt\xF3re zesz\u0142y z granicy.",
  premise_text: "Tekst przes\u0142anki",
  premise_placeholder: "co twierdzi hipoteza",
  search_button: "Szukaj",
  which_stays: "Kt\xF3ra zostaje",
  which_stays_hint: "Por\xF3wnuje dwie hipotezy: kt\xF3ra mierzy lepiej i kt\xF3re pliki si\u0119 zmieni\u0142y.",
  one_hypothesis: "Jedna hipoteza",
  other_hypothesis: "Inna hipoteza",
  compare_button: "Por\xF3wnaj",
  web_seed: "Ta hipoteza otworzy\u0142a kampani\u0119. Nie ma wcze\u015Bniejszej hipotezy.",
  web_mutate: "Ta hipoteza zmienia kod wzgl\u0119dem innej i zachowuje pytanie.",
  web_revert: "Ta hipoteza wraca do wcze\u015Bniejszej \u015Bcie\u017Cki. \u015Alepy zau\u0142ek zostaje zapisany.",
  web_revise: "Pytanie si\u0119 zmieni\u0142o. Wcze\u015Bniejsza przes\u0142anka zosta\u0142a zast\u0105piona.",
  web_synthesize: "Ta hipoteza \u0142\u0105czy wi\u0119cej ni\u017C jedn\u0105 wcze\u015Bniejsz\u0105 \u015Bcie\u017Ck\u0119.",
  read_failed: "Nie uda\u0142o si\u0119 odczyta\u0107 kampanii.",
  campaign_measure: "Pomiar tej kampanii.",
  higher_sentence: "Im wi\u0119cej, tym lepiej.",
  lower_sentence: "Im mniej, tym lepiej.",
  unmeasured_yet: "Nadal bez pomiaru.",
  origin_kicker: "Pochodzenie",
  assumption_kicker: "Co zak\u0142ada ta hipoteza",
  intent_kicker: "Czego chciano uzyska\u0107",
  metrics_kicker: "Metryki",
  nobody_measured: "Nikt jeszcze nie zmierzy\u0142 tej hipotezy.",
  left_frontier_measure: "Pomiar, kt\xF3ry zszed\u0142 z granicy.",
  frontier_measure: "Ostatni pomiar u\u017Cyty na granicy.",
  earlier_measure: "Wcze\u015Bniejszy pomiar.",
  off_frontier: "Poza granic\u0105: {reason}",
  registered_by: "Zapisana przez {agent}.",
  what_came_after: "Co przysz\u0142o potem",
  no_measure_title: "Bez pomiaru",
  branch_seed: "pochodzenie",
  branch_mutate: "rozga\u0142\u0119zienie",
  branch_revert: "powr\xF3t",
  branch_revise: "przes\u0142anka zmieniona",
  branch_synthesize: "synteza",
  beaten_inline: "pokonana przez {claim}",
  need_fact: "Potrzeba",
  owner_fact: "Odpowiedzialny",
  nobody: "nikt",
  agent_word: "agent",
  no_origin_yet: "Nie ma jeszcze pochodzenia. Pierwsza hipoteza otwiera map\u0119.",
  none_survived: "\u017Badna hipoteza nie przetrwa\u0142a pomiar\xF3w.",
  search_or_score: "Szukaj przes\u0142anki albo zapisz pomiar.",
  opened_short: "otworzy\u0142a kampani\u0119",
  came_from_inline: "pochodzi od {claim}",
  stays_detail: "Zostaje {claim}. Jest lepsza lub r\xF3wna w ka\u017Cdej mierze i lepsza w jednej.",
  tie_metrics: "Remis. Obie mierz\u0105 tak samo.",
  each_wins: "Ka\u017Cda wygrywa w innej mierze. Kampania nie ma jeszcze jednego zwyci\u0119zcy.",
  metric_tie: "Remis w tej mierze.",
  text_diff: "R\xF3\u017Cnica tekstu.",
  only_in: "Ten plik istnieje tylko w {name}.",
  no_text_diff: "Plik nie ma r\xF3\u017Cnicy tekstu.",
  metric_col: "Miara",
  reading_col: "Odczyt",
  first_one: "pierwsza",
  second_one: "druga",
  the_first: "Pierwsza",
  the_second: "Druga",
  different_text: "Inny tekst",
  added_files: "Nowe pliki",
  removed_files: "Usuni\u0119te pliki",
  campaign_line: "Kampania {name}",
  problem: "Problem: {text}",
  need: "Potrzeba: {text}",
  owner: "Odpowiedzialny: {name}",
  opened_summary: "Otwarta {age}. {variants} hipotez, {trials} pomiar\xF3w, {rejections} odrzuce\u0144. {tokens}",
  map_label: "Mapa:",
  no_hypothesis_yet: "Kampania nie ma jeszcze hipotezy. Pierwszy zapis jest pochodzeniem problemu.",
  many_origins: "Kampania ma {count} pochodze\u0144: {claims}.",
  guide_intro: "acvc przechowuje hipotezy, przes\u0142ank\u0119 ka\u017Cdej i pomiary. Granica pokazuje te, kt\xF3rych \u017Cadna inna nie pokona\u0142a. Gorsza pr\xF3ba nie usuwa lepszej tej samej hipotezy. Odrzucenie jest czynem cz\u0142owieka.",
  none_stays: "\u017Badna nie zostaje.",
  metric_of_campaign: "pomiar kampanii",
  no_campaign: "bez kampanii",
  campaigns: "Kampanie",
  frontier_screen: "Granica",
  lineage_screen: "Rodow\xF3d",
  premise_screen: "Przes\u0142anka",
  compare_screen: "Por\xF3wnaj",
  recall_screen: "Szukaj",
  help_search: "Enter szuka. Esc anuluje. Backspace kasuje.",
  help_campaigns: "Enter otwiera granic\u0119. \u2190 \u2192 zmienia ekran. q wychodzi.",
  help_frontier: "Enter otwiera przes\u0142ank\u0119. d por\xF3wnuje dwie. \u2190 \u2192 zmienia ekran. q wychodzi.",
  help_lineage: "Enter otwiera t\u0119 hipotez\u0119. \u2190 \u2192 zmienia ekran. q wychodzi.",
  help_detail: "Hipoteza i pomiary. \u2190 \u2192 zmienia ekran. q wychodzi.",
  help_diff: "Enter pokazuje plik. \u2190 \u2192 zmienia ekran. q wychodzi.",
  no_campaigns: "\u017Badna kampania.",
  open_lineage: "Otw\xF3rz hipotez\u0119 na granicy, aby zobaczy\u0107, sk\u0105d pochodzi.",
  no_before_after: "Nie ma wcze\u015Bniejszej hipotezy ani tego, co przysz\u0142o potem.",
  intent_gap: "Intencja  {intent}",
  assumptions_gap: "Za\u0142o\u017Cenia  {items}",
  came_then: "Pochodzi od {from}    Potem {after}",
  recall_prompt: "Wpisz s\u0142owo z przes\u0142anki i naci\u015Bnij Enter.",
  came_from_one: "pochodzi od {claim}",
  pick_campaign: "Wybierz kampani\u0119.",
  first_marked: "Pierwsza hipoteza zaznaczona. Naci\u015Bnij d na drugiej.",
  first_named: "Pierwsza: {claim}. Naci\u015Bnij d na drugiej.",
  ago_under_minute: "mniej ni\u017C minut\u0119 temu",
  ago_minutes: "{count} min temu",
  ago_hours: "{count} godz. temu",
  ago_days: "{count} dni temu",
  no_tokens: "Nie zapisano token\xF3w. Zapisz zu\u017Cycie przez spend, bez zmiany to\u017Csamo\u015Bci hipotezy.",
  tokens_recorded: "{tokens} token\xF3w ju\u017C zapisanych",
  rejection_label: "odrzucenie: {reason}",
  guide_example: "Mapa otwartej kampanii. Bez repozytorium nie ma przyk\u0142adu.",
  guide_text: "Czytaj context otwartej kampanii. Mapa tego repozytorium jest przyk\u0142adem.\nPrzes\u0142anka jest problemem. Przyczyna zostaje przy hipotezie, nie w jednym zdaniu kampanii.\nscore zapisuje pomiar. Gorsza pr\xF3ba nie usuwa lepszej z granicy.\nseed otwiera kampani\u0119 w folderze hipotezy. Skan nie czyta .gitignore. .acvcignore w folderze usuwa te same \u015Bcie\u017Cki w ka\u017Cdym klonie. Seed m\xF3wi, ile plik\xF3w wesz\u0142o i kt\xF3re wa\u017Cy\u0142y wi\u0119cej. Magazyn zostaje w --repo; folder trzyma dwa znaczniki, w .gitignore. Nast\u0119pna hipoteza u\u017Cywa record i ma rodzica.\n",
  status_frontier_why: "Zmierzona, i \u017Cadna inna jej nie przewy\u017Csza.",
  status_beaten_why: "Zmierzona, i inna hipoteza na granicy jest lepsza.",
  status_rejected_why: "Motyw jest w grafie.",
  status_unmeasured_why: "Nie ma jeszcze pe\u0142nego wyniku dla miary kampanii.",
  when_context: "Zanim zaczniesz. Zwraca map\u0119 tej kampanii.",
  when_guide: "Polecenia oraz mapa otwartego repozytorium.",
  when_frontier: "Tylko to, czego pomiar jeszcze nie pokona\u0142.",
  when_why: "Jedna hipoteza, z przyczyn\u0105, pomiarem i tym, kto j\u0105 pokona\u0142.",
  when_recall: "Znale\u017A\u0107 przes\u0142ank\u0119 po tek\u015Bcie. Zwraca status i rodzica.",
  when_seed: "Pierwsza hipoteza kampanii. Zasiej folder hipotezy: skan nie czyta .gitignore i pomija .git, .acvc, target, node_modules, .build i DerivedData. .acvcignore w folderze usuwa te same \u015Bcie\u017Cki w ka\u017Cdym klonie; bez niego obowi\u0105zuje sta\u0142a lista. Seed m\xF3wi, ile plik\xF3w wesz\u0142o i kt\xF3re wa\u017Cy\u0142y wi\u0119cej. Magazyn zostaje w --repo. Folder trzyma .acvc-workspace i .acvc-index, oba w .gitignore.",
  when_record: "Folder si\u0119 zmieni\u0142. Nowa hipoteza wskazuje rodzica.",
  when_score: "Jest liczba miary kampanii. Gorsza pr\xF3ba nie zast\u0119puje lepszej.",
  when_reject: "Hipoteza umar\u0142a, a motyw zostaje w grafie.",
  when_spend: "Sesja wie, ile token\xF3w zu\u017Cy\u0142a.",
  when_compare: "Dwie zmierzone hipotezy, aby zobaczy\u0107, kt\xF3ra zostaje i kt\xF3re pliki si\u0119 zmieni\u0142y.",
  when_web: "Mapa na stronie.",
  when_obsidian: "Instaluje podgl\u0105d w vault i wskazuje to repozytorium. Polecenie w Obsidian to Open ACVC.",
  none_count: "\u017Caden",
  help_recall: "/ szuka. Enter szuka. \u2190 \u2192 zmienia ekran. q wychodzi.",
  came_label: "Pochodzi od",
  problem_label: "Problem",
  better_here: "{name} jest tu lepsza.",
  only_side: "Tylko w {name}",
  web_campaign_fact: "{age} \xB7 {variants} hipotez \xB7 {trials} pomiar\xF3w \xB7 {tokens}",
  only_other: "tylko w drugiej  {path}",
  only_first: "tylko w pierwszej  {path}",
  changed_path: "zmieni\u0142o si\u0119  {path}",
  the_other: "Druga",
  after_named: "Co przysz\u0142o potem: {names}",
  assistant_pack: "Umowa dla innego agenta wychodzi z `acvc assistant`, w j\u0119zyku tego komputera. Agent z MCP ASB wysy\u0142a tablic\u0119 documents do rag.index, a potem szuka przez rag.search.",
  assistant_index: "Je\u015Bli ta sesja ma MCP ASB, wy\u015Blij tablic\u0119 documents do rag.index raz. P\xF3\u017Aniejsze szukanie u\u017Cywa rag.search.",
  when_assistant: "Zwraca umow\u0119 operacyjn\u0105 w j\u0119zyku tego komputera, gotow\u0105 dla rag.index."
};

// ../../locales/pt-BR.json
var pt_BR_default = {
  on_frontier: "Na fronteira.",
  motive_unrecorded: "Motivo n\xE3o gravado.",
  weighed: "pesou {path} {bytes}",
  no_frontier: "Nenhuma hip\xF3tese na fronteira.",
  no_recall: "Nenhuma premissa cont\xE9m esse texto.",
  opened_campaign: "Esta hip\xF3tese abriu a campanha.",
  came_from: "Veio de: {names}",
  stays: "{claim} fica.",
  tie_both: "{left} e {right} medem igual.",
  incomparable: "Nenhuma fica. Cada uma ganha numa m\xE9trica diferente.",
  missing_measure: "Falta medi\xE7\xE3o para decidir.",
  missing_measure_which: "Falta medi\xE7\xE3o para decidir qual fica.",
  no_measurement: "sem medi\xE7\xE3o",
  cause: "Causa: {items}",
  intent: "Inten\xE7\xE3o: {intent}",
  measurement: "Medi\xE7\xE3o: {text}",
  rejected: "Rejeitada: {reason}",
  rejected_gap: "Rejeitada  {reason}",
  beaten_by: "Superada por {claim}.",
  beaten_by_gap: "Superada por  {claim}.",
  higher_better: "quanto maior, melhor",
  lower_better: "quanto menor, melhor",
  agent: "Agente: {agent}",
  agent_gap: "Agente  {agent}",
  no_scored: "Nenhuma hip\xF3tese pontuada.",
  appears_after_score: "Ela aparece aqui depois de acvc score.",
  open_hypothesis: "Abra uma hip\xF3tese na fronteira.",
  score_heading: "Pontua\xE7\xE3o",
  still_unmeasured: "    ainda sem medi\xE7\xE3o",
  frontier_hint: "Na fronteira, aperte d em duas hip\xF3teses.",
  nothing_compared: "Nada comparado ainda.",
  search_label: "Busca  {query}",
  none_chosen: "Nenhuma hip\xF3tese escolhida.",
  came_from_short: "veio de  {claim}",
  then_short: "depois  {claim}",
  origin_prefix: "Origem  {text}",
  origin_seed: "Come\xE7ou a campanha. N\xE3o h\xE1 hip\xF3tese anterior.",
  origin_mutate: "Mudan\xE7a a partir de outra hip\xF3tese.",
  origin_revert: "Volta para uma hip\xF3tese anterior.",
  origin_revise: "A premissa foi reescrita.",
  origin_synthesize: "S\xEDntese de v\xE1rias hip\xF3teses.",
  agent_unknown: "n\xE3o informado",
  status_frontier: "na fronteira",
  status_beaten: "superada",
  status_rejected: "rejeitada",
  different_files: "arquivos diferentes {count}",
  against: "{name} {loser} contra {winner} ({sense})",
  title: "O que ainda vale explorar",
  lede: "Uma campanha compara hip\xF3teses pela premissa e pelas medi\xE7\xF5es. A fronteira mostra as que nenhuma outra superou.",
  campaign_label: "Campanha",
  frontier_heading: "Na fronteira",
  frontier_hint_page: "Premissa e m\xE9tricas do que a medi\xE7\xE3o ainda n\xE3o descartou.",
  hypothesis_heading: "A hip\xF3tese",
  why_hint: "Escolha uma hip\xF3tese para ver a origem, as suposi\xE7\xF5es e as medi\xE7\xF5es.",
  lineage_heading: "Origem e ramifica\xE7\xF5es",
  lineage_hint: "O problema, quem responde, h\xE1 quanto tempo a campanha est\xE1 aberta, e cada caminho com o resultado.",
  search_heading: "Buscar",
  search_hint: "Encontra hip\xF3teses pela frase da premissa, inclusive as que sa\xEDram da fronteira.",
  premise_text: "Texto da premissa",
  premise_placeholder: "o que a hip\xF3tese afirma",
  search_button: "Buscar",
  which_stays: "Qual fica",
  which_stays_hint: "Compara duas hip\xF3teses: quem mede melhor e quais arquivos mudaram.",
  one_hypothesis: "Uma hip\xF3tese",
  other_hypothesis: "Outra hip\xF3tese",
  compare_button: "Comparar",
  web_seed: "Esta hip\xF3tese abriu a campanha. N\xE3o existe uma hip\xF3tese anterior.",
  web_mutate: "Esta hip\xF3tese muda o c\xF3digo a partir de outra, e mant\xE9m a pergunta.",
  web_revert: "Esta hip\xF3tese volta para um caminho anterior. O beco sem sa\xEDda continua registrado.",
  web_revise: "A pergunta mudou. A premissa anterior foi substitu\xEDda.",
  web_synthesize: "Esta hip\xF3tese junta mais de um caminho anterior.",
  read_failed: "N\xE3o foi poss\xEDvel ler a campanha.",
  campaign_measure: "Medi\xE7\xE3o desta campanha.",
  higher_sentence: "Quanto maior, melhor.",
  lower_sentence: "Quanto menor, melhor.",
  unmeasured_yet: "Ainda sem medi\xE7\xE3o.",
  origin_kicker: "Origem",
  assumption_kicker: "O que esta hip\xF3tese assume",
  intent_kicker: "O que se queria obter",
  metrics_kicker: "M\xE9tricas",
  nobody_measured: "Ningu\xE9m mediu esta hip\xF3tese ainda.",
  left_frontier_measure: "Medi\xE7\xE3o que saiu da fronteira.",
  frontier_measure: "\xDAltima medi\xE7\xE3o usada na fronteira.",
  earlier_measure: "Medi\xE7\xE3o anterior.",
  off_frontier: "Fora da fronteira: {reason}",
  registered_by: "Registrada por {agent}.",
  what_came_after: "O que veio depois",
  no_measure_title: "Sem medi\xE7\xE3o",
  branch_seed: "origem",
  branch_mutate: "ramifica\xE7\xE3o",
  branch_revert: "retorno",
  branch_revise: "premissa revista",
  branch_synthesize: "s\xEDntese",
  beaten_inline: "superada por {claim}",
  need_fact: "Necessidade",
  owner_fact: "Respons\xE1vel",
  nobody: "ningu\xE9m",
  agent_word: "agente",
  no_origin_yet: "Ainda n\xE3o h\xE1 origem. A primeira hip\xF3tese abre o mapa.",
  none_survived: "Nenhuma hip\xF3tese sobreviveu \xE0s medi\xE7\xF5es.",
  search_or_score: "Busque uma premissa ou registre uma medi\xE7\xE3o.",
  opened_short: "abriu a campanha",
  came_from_inline: "veio de {claim}",
  stays_detail: "{claim} fica. \xC9 melhor ou igual em todas as m\xE9tricas e melhor em uma.",
  tie_metrics: "Empate. As duas medem igual.",
  each_wins: "Cada uma ganha numa m\xE9trica diferente. A campanha ainda n\xE3o tem um vencedor \xFAnico.",
  metric_tie: "Empate nesta m\xE9trica.",
  text_diff: "Diferen\xE7a de texto.",
  only_in: "Este arquivo s\xF3 existe em {name}.",
  no_text_diff: "O arquivo n\xE3o tem diferen\xE7a de texto.",
  metric_col: "M\xE9trica",
  reading_col: "Leitura",
  first_one: "a primeira",
  second_one: "a segunda",
  the_first: "A primeira",
  the_second: "A segunda",
  different_text: "Texto diferente",
  added_files: "Arquivos novos",
  removed_files: "Arquivos removidos",
  campaign_line: "Campanha {name}",
  problem: "Problema: {text}",
  need: "Necessidade: {text}",
  owner: "Respons\xE1vel: {name}",
  opened_summary: "Aberta {age}. {variants} hip\xF3teses, {trials} medi\xE7\xF5es, {rejections} rejei\xE7\xF5es. {tokens}",
  map_label: "Mapa:",
  no_hypothesis_yet: "A campanha ainda n\xE3o tem hip\xF3tese. A primeira grava\xE7\xE3o \xE9 a origem do problema.",
  many_origins: "A campanha tem {count} origens: {claims}.",
  guide_intro: "O acvc guarda hip\xF3teses, a premissa de cada uma e as medi\xE7\xF5es. A fronteira mostra as que nenhuma outra superou. Um ensaio pior n\xE3o tira o melhor da mesma hip\xF3tese. Rejeitar \xE9 um ato humano.",
  none_stays: "Nenhuma fica.",
  metric_of_campaign: "medi\xE7\xE3o da campanha",
  no_campaign: "sem campanha",
  campaigns: "Campanhas",
  frontier_screen: "Fronteira",
  lineage_screen: "Linhagem",
  premise_screen: "Premissa",
  compare_screen: "Comparar",
  recall_screen: "Busca",
  help_search: "Enter busca. Esc cancela. Backspace apaga.",
  help_campaigns: "Enter abre a fronteira. \u2190 \u2192 troca a tela. q sai.",
  help_frontier: "Enter abre a premissa. d compara duas. \u2190 \u2192 troca a tela. q sai.",
  help_lineage: "Enter abre essa hip\xF3tese. \u2190 \u2192 troca a tela. q sai.",
  help_detail: "A hip\xF3tese e as medi\xE7\xF5es. \u2190 \u2192 troca a tela. q sai.",
  help_diff: "Enter mostra o arquivo. \u2190 \u2192 troca a tela. q sai.",
  no_campaigns: "Nenhuma campanha.",
  open_lineage: "Abra uma hip\xF3tese na fronteira para ver de onde ela veio.",
  no_before_after: "N\xE3o h\xE1 hip\xF3tese anterior nem o que veio depois.",
  intent_gap: "Inten\xE7\xE3o  {intent}",
  assumptions_gap: "Suposi\xE7\xF5es  {items}",
  came_then: "Veio de {from}    Depois {after}",
  recall_prompt: "Escreva uma palavra da premissa e aperte Enter.",
  came_from_one: "veio de {claim}",
  pick_campaign: "Escolha uma campanha.",
  first_marked: "Primeira hip\xF3tese marcada. Aperte d na outra.",
  first_named: "Primeira: {claim}. Aperte d na outra.",
  ago_under_minute: "h\xE1 menos de um minuto",
  ago_minutes: "h\xE1 {count} min",
  ago_hours: "h\xE1 {count} h",
  ago_days: "h\xE1 {count} dias",
  no_tokens: "Nenhum token registrado. Grave o consumo com spend, sem mudar a identidade da hip\xF3tese.",
  tokens_recorded: "{tokens} tokens j\xE1 registrados",
  rejection_label: "rejei\xE7\xE3o: {reason}",
  guide_example: "O mapa da campanha aberta. Sem reposit\xF3rio, n\xE3o h\xE1 exemplo.",
  guide_text: "Leia context da campanha aberta. O mapa desse reposit\xF3rio \xE9 o exemplo.\nA premissa \xE9 o problema. A causa fica na hip\xF3tese, n\xE3o numa frase s\xF3 da campanha.\nscore grava a medi\xE7\xE3o. Um ensaio pior n\xE3o tira o melhor da fronteira.\nseed abre a campanha na pasta da hip\xF3tese. A varredura n\xE3o l\xEA o .gitignore. Um .acvcignore na pasta tira os mesmos caminhos em todo clone. O seed diz quantos arquivos entraram e quais pesaram mais. O store fica em --repo; na pasta ficam os dois marcadores, no .gitignore. A hip\xF3tese seguinte usa record e tem pai.\n",
  status_frontier_why: "Medida, e nenhuma outra a supera.",
  status_beaten_why: "Medida, e outra hip\xF3tese da fronteira \xE9 melhor.",
  status_rejected_why: "O motivo est\xE1 no grafo.",
  status_unmeasured_why: "Ainda n\xE3o h\xE1 score completo para a m\xE9trica da campanha.",
  when_context: "Antes de agir. Devolve o mapa desta campanha.",
  when_guide: "Os comandos, mais o mapa do reposit\xF3rio aberto.",
  when_frontier: "S\xF3 o que a medi\xE7\xE3o ainda n\xE3o superou.",
  when_why: "Uma hip\xF3tese, com a causa, a medi\xE7\xE3o e quem a superou.",
  when_recall: "Achar uma premissa pelo texto. Devolve o status e o pai.",
  when_seed: "A primeira hip\xF3tese de uma campanha. Semeie a pasta da hip\xF3tese: a varredura n\xE3o l\xEA o .gitignore e pula .git, .acvc, target, node_modules, .build e DerivedData. Um .acvcignore na pasta tira caminhos iguais em todo clone; sem ele, vale a lista fixa. O seed diz quantos arquivos entraram e quais pesaram mais. O store fica em --repo. Na pasta ficam .acvc-workspace e .acvc-index, os dois no .gitignore.",
  when_record: "A pasta mudou. A nova hip\xF3tese aponta para o pai.",
  when_score: "H\xE1 um n\xFAmero da m\xE9trica da campanha. Um ensaio pior n\xE3o substitui um melhor.",
  when_reject: "A hip\xF3tese morreu e o motivo fica no grafo.",
  when_spend: "A sess\xE3o sabe quantos tokens consumiu.",
  when_compare: "Duas hip\xF3teses medidas, para ver quem fica e quais arquivos mudaram.",
  when_web: "O mapa na p\xE1gina.",
  when_obsidian: "Instala o visor no vault e aponta este reposit\xF3rio. O comando no Obsidian \xE9 Open ACVC.",
  none_count: "nenhum",
  help_recall: "/ busca. Enter procura. \u2190 \u2192 troca a tela. q sai.",
  came_label: "Veio de",
  problem_label: "Problema",
  better_here: "{name} est\xE1 melhor aqui.",
  only_side: "S\xF3 em {name}",
  web_campaign_fact: "{age} \xB7 {variants} hip\xF3teses \xB7 {trials} medi\xE7\xF5es \xB7 {tokens}",
  only_other: "s\xF3 na outra  {path}",
  only_first: "s\xF3 na primeira  {path}",
  changed_path: "mudou  {path}",
  the_other: "A outra",
  after_named: "O que veio depois: {names}",
  assistant_pack: "O contrato para outro agente sai de `acvc assistant`, no idioma deste computador. Quem tem o MCP do ASB envia o array documents para rag.index e depois busca com rag.search.",
  assistant_index: "Se esta sess\xE3o tem o MCP do ASB, envie o array documents para rag.index uma vez. A busca seguinte usa rag.search.",
  when_assistant: "Devolve o contrato operacional no idioma deste computador, pronto para rag.index."
};

// ../../locales/ru.json
var ru_default = {
  on_frontier: "\u041D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435.",
  motive_unrecorded: "\u041C\u043E\u0442\u0438\u0432 \u043D\u0435 \u0437\u0430\u043F\u0438\u0441\u0430\u043D.",
  weighed: "\u0432\u0435\u0441\u0438\u043B\u043E {path} {bytes}",
  no_frontier: "\u041D\u0438 \u043E\u0434\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043D\u0435 \u043D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435.",
  no_recall: "\u041D\u0438 \u043E\u0434\u043D\u0430 \u043F\u043E\u0441\u044B\u043B\u043A\u0430 \u043D\u0435 \u0441\u043E\u0434\u0435\u0440\u0436\u0438\u0442 \u044D\u0442\u043E\u0442 \u0442\u0435\u043A\u0441\u0442.",
  opened_campaign: "\u042D\u0442\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043E\u0442\u043A\u0440\u044B\u043B\u0430 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044E.",
  came_from: "\u041F\u0440\u0438\u0448\u043B\u0430 \u043E\u0442: {names}",
  stays: "\u041E\u0441\u0442\u0430\u0451\u0442\u0441\u044F: {claim}.",
  tie_both: "{left} \u0438 {right} \u0438\u0437\u043C\u0435\u0440\u044F\u044E\u0442\u0441\u044F \u043E\u0434\u0438\u043D\u0430\u043A\u043E\u0432\u043E.",
  incomparable: "\u041D\u0438 \u043E\u0434\u043D\u0430 \u043D\u0435 \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F \u043E\u0434\u043D\u0430. \u041A\u0430\u0436\u0434\u0430\u044F \u0432\u044B\u0438\u0433\u0440\u044B\u0432\u0430\u0435\u0442 \u043F\u043E \u0441\u0432\u043E\u0435\u0439 \u043C\u0435\u0440\u0435.",
  missing_measure: "\u041D\u0435 \u0445\u0432\u0430\u0442\u0430\u0435\u0442 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F, \u0447\u0442\u043E\u0431\u044B \u0440\u0435\u0448\u0438\u0442\u044C.",
  missing_measure_which: "\u041D\u0435 \u0445\u0432\u0430\u0442\u0430\u0435\u0442 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F, \u0447\u0442\u043E\u0431\u044B \u0440\u0435\u0448\u0438\u0442\u044C, \u043A\u0430\u043A\u0430\u044F \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F.",
  no_measurement: "\u0431\u0435\u0437 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F",
  cause: "\u041F\u0440\u0438\u0447\u0438\u043D\u0430: {items}",
  intent: "\u041D\u0430\u043C\u0435\u0440\u0435\u043D\u0438\u0435: {intent}",
  measurement: "\u0418\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435: {text}",
  rejected: "\u041E\u0442\u043A\u043B\u043E\u043D\u0435\u043D\u0430: {reason}",
  rejected_gap: "\u041E\u0442\u043A\u043B\u043E\u043D\u0435\u043D\u0430  {reason}",
  beaten_by: "\u041F\u0440\u0435\u0432\u0437\u043E\u0439\u0434\u0435\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u043E\u0439 {claim}.",
  beaten_by_gap: "\u041F\u0440\u0435\u0432\u0437\u043E\u0439\u0434\u0435\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u043E\u0439  {claim}.",
  higher_better: "\u0447\u0435\u043C \u0431\u043E\u043B\u044C\u0448\u0435, \u0442\u0435\u043C \u043B\u0443\u0447\u0448\u0435",
  lower_better: "\u0447\u0435\u043C \u043C\u0435\u043D\u044C\u0448\u0435, \u0442\u0435\u043C \u043B\u0443\u0447\u0448\u0435",
  agent: "\u0410\u0433\u0435\u043D\u0442: {agent}",
  agent_gap: "\u0410\u0433\u0435\u043D\u0442  {agent}",
  no_scored: "\u041D\u0438 \u043E\u0434\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043D\u0435 \u043E\u0446\u0435\u043D\u0435\u043D\u0430.",
  appears_after_score: "\u041E\u043D\u0430 \u043F\u043E\u044F\u0432\u0438\u0442\u0441\u044F \u0437\u0434\u0435\u0441\u044C \u043F\u043E\u0441\u043B\u0435 acvc score.",
  open_hypothesis: "\u041E\u0442\u043A\u0440\u043E\u0439\u0442\u0435 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0443 \u043D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435.",
  score_heading: "\u041E\u0446\u0435\u043D\u043A\u0430",
  still_unmeasured: "    \u0432\u0441\u0451 \u0435\u0449\u0451 \u0431\u0435\u0437 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F",
  frontier_hint: "\u041D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435 \u043D\u0430\u0436\u043C\u0438\u0442\u0435 d \u043D\u0430 \u0434\u0432\u0443\u0445 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430\u0445.",
  nothing_compared: "\u041F\u043E\u043A\u0430 \u043D\u0438\u0447\u0435\u0433\u043E \u043D\u0435 \u0441\u0440\u0430\u0432\u043D\u0435\u043D\u043E.",
  search_label: "\u041F\u043E\u0438\u0441\u043A  {query}",
  none_chosen: "\u0413\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043D\u0435 \u0432\u044B\u0431\u0440\u0430\u043D\u0430.",
  came_from_short: "\u043F\u0440\u0438\u0448\u043B\u0430 \u043E\u0442  {claim}",
  then_short: "\u043F\u043E\u0442\u043E\u043C  {claim}",
  origin_prefix: "\u041F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u0435  {text}",
  origin_seed: "\u041E\u043D\u0430 \u043D\u0430\u0447\u0430\u043B\u0430 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044E. \u0411\u043E\u043B\u0435\u0435 \u0440\u0430\u043D\u043D\u0435\u0439 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B \u043D\u0435\u0442.",
  origin_mutate: "\u0418\u0437\u043C\u0435\u043D\u0435\u043D\u0438\u0435 \u043E\u0442 \u0434\u0440\u0443\u0433\u043E\u0439 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B.",
  origin_revert: "\u0412\u043E\u0437\u0432\u0440\u0430\u0442 \u043A \u0431\u043E\u043B\u0435\u0435 \u0440\u0430\u043D\u043D\u0435\u0439 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0435.",
  origin_revise: "\u041F\u043E\u0441\u044B\u043B\u043A\u0430 \u0431\u044B\u043B\u0430 \u043F\u0435\u0440\u0435\u043F\u0438\u0441\u0430\u043D\u0430.",
  origin_synthesize: "\u0421\u0438\u043D\u0442\u0435\u0437 \u043D\u0435\u0441\u043A\u043E\u043B\u044C\u043A\u0438\u0445 \u0433\u0438\u043F\u043E\u0442\u0435\u0437.",
  agent_unknown: "\u043D\u0435 \u0443\u043A\u0430\u0437\u0430\u043D",
  status_frontier: "\u043D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435",
  status_beaten: "\u043F\u0440\u0435\u0432\u0437\u043E\u0439\u0434\u0435\u043D\u0430",
  status_rejected: "\u043E\u0442\u043A\u043B\u043E\u043D\u0435\u043D\u0430",
  different_files: "\u0440\u0430\u0437\u043B\u0438\u0447\u0430\u044E\u0449\u0438\u0445\u0441\u044F \u0444\u0430\u0439\u043B\u043E\u0432 {count}",
  against: "{name} {loser} \u043F\u0440\u043E\u0442\u0438\u0432 {winner} ({sense})",
  title: "\u0427\u0442\u043E \u0435\u0449\u0451 \u0441\u0442\u043E\u0438\u0442 \u0438\u0441\u0441\u043B\u0435\u0434\u043E\u0432\u0430\u0442\u044C",
  lede: "\u041A\u0430\u043C\u043F\u0430\u043D\u0438\u044F \u0441\u0440\u0430\u0432\u043D\u0438\u0432\u0430\u0435\u0442 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B \u043F\u043E \u043F\u043E\u0441\u044B\u043B\u043A\u0435 \u0438 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F\u043C. \u0413\u0440\u0430\u043D\u0438\u0446\u0430 \u043F\u043E\u043A\u0430\u0437\u044B\u0432\u0430\u0435\u0442 \u0442\u0435, \u043A\u043E\u0442\u043E\u0440\u044B\u0435 \u043D\u0438 \u043E\u0434\u043D\u0430 \u0434\u0440\u0443\u0433\u0430\u044F \u043D\u0435 \u043F\u0440\u0435\u0432\u0437\u043E\u0448\u043B\u0430.",
  campaign_label: "\u041A\u0430\u043C\u043F\u0430\u043D\u0438\u044F",
  frontier_heading: "\u041D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435",
  frontier_hint_page: "\u041F\u043E\u0441\u044B\u043B\u043A\u0430 \u0438 \u043C\u0435\u0442\u0440\u0438\u043A\u0438 \u0442\u043E\u0433\u043E, \u0447\u0442\u043E \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435 \u0435\u0449\u0451 \u043D\u0435 \u043E\u0442\u0431\u0440\u043E\u0441\u0438\u043B\u043E.",
  hypothesis_heading: "\u0413\u0438\u043F\u043E\u0442\u0435\u0437\u0430",
  why_hint: "\u0412\u044B\u0431\u0435\u0440\u0438\u0442\u0435 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0443, \u0447\u0442\u043E\u0431\u044B \u0443\u0432\u0438\u0434\u0435\u0442\u044C \u043F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u0435, \u0434\u043E\u043F\u0443\u0449\u0435\u043D\u0438\u044F \u0438 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F.",
  lineage_heading: "\u041F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u0435 \u0438 \u0432\u0435\u0442\u0432\u0438",
  lineage_hint: "\u041F\u0440\u043E\u0431\u043B\u0435\u043C\u0430, \u043A\u0442\u043E \u043E\u0442\u0432\u0435\u0447\u0430\u0435\u0442, \u043A\u0430\u043A \u0434\u0430\u0432\u043D\u043E \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044F \u043E\u0442\u043A\u0440\u044B\u0442\u0430, \u0438 \u043A\u0430\u0436\u0434\u044B\u0439 \u043F\u0443\u0442\u044C \u0441 \u0440\u0435\u0437\u0443\u043B\u044C\u0442\u0430\u0442\u043E\u043C.",
  search_heading: "\u0418\u0441\u043A\u0430\u0442\u044C",
  search_hint: "\u041D\u0430\u0445\u043E\u0434\u0438\u0442 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B \u043F\u043E \u0444\u0440\u0430\u0437\u0435 \u043F\u043E\u0441\u044B\u043B\u043A\u0438, \u0432\u043A\u043B\u044E\u0447\u0430\u044F \u0443\u0448\u0435\u0434\u0448\u0438\u0435 \u0441 \u0433\u0440\u0430\u043D\u0438\u0446\u044B.",
  premise_text: "\u0422\u0435\u043A\u0441\u0442 \u043F\u043E\u0441\u044B\u043B\u043A\u0438",
  premise_placeholder: "\u0447\u0442\u043E \u0443\u0442\u0432\u0435\u0440\u0436\u0434\u0430\u0435\u0442 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430",
  search_button: "\u0418\u0441\u043A\u0430\u0442\u044C",
  which_stays: "\u041A\u0430\u043A\u0430\u044F \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F",
  which_stays_hint: "\u0421\u0440\u0430\u0432\u043D\u0438\u0432\u0430\u0435\u0442 \u0434\u0432\u0435 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B: \u043A\u0430\u043A\u0430\u044F \u0438\u0437\u043C\u0435\u0440\u044F\u0435\u0442\u0441\u044F \u043B\u0443\u0447\u0448\u0435 \u0438 \u043A\u0430\u043A\u0438\u0435 \u0444\u0430\u0439\u043B\u044B \u0438\u0437\u043C\u0435\u043D\u0438\u043B\u0438\u0441\u044C.",
  one_hypothesis: "\u041E\u0434\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430",
  other_hypothesis: "\u0414\u0440\u0443\u0433\u0430\u044F \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430",
  compare_button: "\u0421\u0440\u0430\u0432\u043D\u0438\u0442\u044C",
  web_seed: "\u042D\u0442\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043E\u0442\u043A\u0440\u044B\u043B\u0430 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044E. \u0411\u043E\u043B\u0435\u0435 \u0440\u0430\u043D\u043D\u0435\u0439 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B \u043D\u0435\u0442.",
  web_mutate: "\u042D\u0442\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043C\u0435\u043D\u044F\u0435\u0442 \u043A\u043E\u0434 \u043E\u0442 \u0434\u0440\u0443\u0433\u043E\u0439 \u0438 \u0441\u043E\u0445\u0440\u0430\u043D\u044F\u0435\u0442 \u0432\u043E\u043F\u0440\u043E\u0441.",
  web_revert: "\u042D\u0442\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u0432\u043E\u0437\u0432\u0440\u0430\u0449\u0430\u0435\u0442\u0441\u044F \u043A \u043F\u0440\u0435\u0436\u043D\u0435\u043C\u0443 \u043F\u0443\u0442\u0438. \u0422\u0443\u043F\u0438\u043A \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F \u0437\u0430\u043F\u0438\u0441\u0430\u043D\u043D\u044B\u043C.",
  web_revise: "\u0412\u043E\u043F\u0440\u043E\u0441 \u0438\u0437\u043C\u0435\u043D\u0438\u043B\u0441\u044F. \u041F\u0440\u0435\u0436\u043D\u044F\u044F \u043F\u043E\u0441\u044B\u043B\u043A\u0430 \u0437\u0430\u043C\u0435\u043D\u0435\u043D\u0430.",
  web_synthesize: "\u042D\u0442\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u0441\u043E\u0435\u0434\u0438\u043D\u044F\u0435\u0442 \u0431\u043E\u043B\u044C\u0448\u0435 \u043E\u0434\u043D\u043E\u0433\u043E \u043F\u0440\u0435\u0436\u043D\u0435\u0433\u043E \u043F\u0443\u0442\u0438.",
  read_failed: "\u041D\u0435 \u0443\u0434\u0430\u043B\u043E\u0441\u044C \u043F\u0440\u043E\u0447\u0438\u0442\u0430\u0442\u044C \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044E.",
  campaign_measure: "\u0418\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435 \u044D\u0442\u043E\u0439 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438.",
  higher_sentence: "\u0427\u0435\u043C \u0431\u043E\u043B\u044C\u0448\u0435, \u0442\u0435\u043C \u043B\u0443\u0447\u0448\u0435.",
  lower_sentence: "\u0427\u0435\u043C \u043C\u0435\u043D\u044C\u0448\u0435, \u0442\u0435\u043C \u043B\u0443\u0447\u0448\u0435.",
  unmeasured_yet: "\u0412\u0441\u0451 \u0435\u0449\u0451 \u0431\u0435\u0437 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F.",
  origin_kicker: "\u041F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u0435",
  assumption_kicker: "\u0427\u0442\u043E \u043F\u0440\u0435\u0434\u043F\u043E\u043B\u0430\u0433\u0430\u0435\u0442 \u044D\u0442\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430",
  intent_kicker: "\u0427\u0435\u0433\u043E \u0445\u043E\u0442\u0435\u043B\u0438 \u0434\u043E\u0431\u0438\u0442\u044C\u0441\u044F",
  metrics_kicker: "\u041C\u0435\u0442\u0440\u0438\u043A\u0438",
  nobody_measured: "\u042D\u0442\u0443 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0443 \u0435\u0449\u0451 \u043D\u0438\u043A\u0442\u043E \u043D\u0435 \u0438\u0437\u043C\u0435\u0440\u0438\u043B.",
  left_frontier_measure: "\u0418\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435, \u043A\u043E\u0442\u043E\u0440\u043E\u0435 \u0443\u0448\u043B\u043E \u0441 \u0433\u0440\u0430\u043D\u0438\u0446\u044B.",
  frontier_measure: "\u041F\u043E\u0441\u043B\u0435\u0434\u043D\u0435\u0435 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435, \u0438\u0441\u043F\u043E\u043B\u044C\u0437\u043E\u0432\u0430\u043D\u043D\u043E\u0435 \u043D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435.",
  earlier_measure: "\u0411\u043E\u043B\u0435\u0435 \u0440\u0430\u043D\u043D\u0435\u0435 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435.",
  off_frontier: "\u0412\u043D\u0435 \u0433\u0440\u0430\u043D\u0438\u0446\u044B: {reason}",
  registered_by: "\u0417\u0430\u043F\u0438\u0441\u0430\u043D\u0430 \u0430\u0433\u0435\u043D\u0442\u043E\u043C {agent}.",
  what_came_after: "\u0427\u0442\u043E \u0431\u044B\u043B\u043E \u043F\u043E\u0442\u043E\u043C",
  no_measure_title: "\u0411\u0435\u0437 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F",
  branch_seed: "\u043F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u0435",
  branch_mutate: "\u0432\u0435\u0442\u0432\u044C",
  branch_revert: "\u0432\u043E\u0437\u0432\u0440\u0430\u0442",
  branch_revise: "\u043F\u043E\u0441\u044B\u043B\u043A\u0430 \u043F\u0435\u0440\u0435\u0441\u043C\u043E\u0442\u0440\u0435\u043D\u0430",
  branch_synthesize: "\u0441\u0438\u043D\u0442\u0435\u0437",
  beaten_inline: "\u043F\u0440\u0435\u0432\u0437\u043E\u0439\u0434\u0435\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u043E\u0439 {claim}",
  need_fact: "\u041F\u043E\u0442\u0440\u0435\u0431\u043D\u043E\u0441\u0442\u044C",
  owner_fact: "\u041E\u0442\u0432\u0435\u0442\u0441\u0442\u0432\u0435\u043D\u043D\u044B\u0439",
  nobody: "\u043D\u0438\u043A\u0442\u043E",
  agent_word: "\u0430\u0433\u0435\u043D\u0442",
  no_origin_yet: "\u041F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u044F \u0435\u0449\u0451 \u043D\u0435\u0442. \u041F\u0435\u0440\u0432\u0430\u044F \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043E\u0442\u043A\u0440\u044B\u0432\u0430\u0435\u0442 \u043A\u0430\u0440\u0442\u0443.",
  none_survived: "\u041D\u0438 \u043E\u0434\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043D\u0435 \u043F\u0435\u0440\u0435\u0436\u0438\u043B\u0430 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F.",
  search_or_score: "\u041D\u0430\u0439\u0434\u0438\u0442\u0435 \u043F\u043E\u0441\u044B\u043B\u043A\u0443 \u0438\u043B\u0438 \u0437\u0430\u043F\u0438\u0448\u0438\u0442\u0435 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435.",
  opened_short: "\u043E\u0442\u043A\u0440\u044B\u043B\u0430 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044E",
  came_from_inline: "\u043F\u0440\u0438\u0448\u043B\u0430 \u043E\u0442 {claim}",
  stays_detail: "\u041E\u0441\u0442\u0430\u0451\u0442\u0441\u044F {claim}. \u041E\u043D\u0430 \u043B\u0443\u0447\u0448\u0435 \u0438\u043B\u0438 \u0440\u0430\u0432\u043D\u0430 \u043F\u043E \u043A\u0430\u0436\u0434\u043E\u0439 \u043C\u0435\u0440\u0435 \u0438 \u043B\u0443\u0447\u0448\u0435 \u043F\u043E \u043E\u0434\u043D\u043E\u0439.",
  tie_metrics: "\u041D\u0438\u0447\u044C\u044F. \u041E\u0431\u0435 \u0438\u0437\u043C\u0435\u0440\u044F\u044E\u0442\u0441\u044F \u043E\u0434\u0438\u043D\u0430\u043A\u043E\u0432\u043E.",
  each_wins: "\u041A\u0430\u0436\u0434\u0430\u044F \u0432\u044B\u0438\u0433\u0440\u044B\u0432\u0430\u0435\u0442 \u043F\u043E \u0441\u0432\u043E\u0435\u0439 \u043C\u0435\u0440\u0435. \u0423 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438 \u0435\u0449\u0451 \u043D\u0435\u0442 \u0435\u0434\u0438\u043D\u0441\u0442\u0432\u0435\u043D\u043D\u043E\u0433\u043E \u043F\u043E\u0431\u0435\u0434\u0438\u0442\u0435\u043B\u044F.",
  metric_tie: "\u041D\u0438\u0447\u044C\u044F \u043F\u043E \u044D\u0442\u043E\u0439 \u043C\u0435\u0440\u0435.",
  text_diff: "\u0420\u0430\u0437\u043B\u0438\u0447\u0438\u0435 \u0442\u0435\u043A\u0441\u0442\u0430.",
  only_in: "\u042D\u0442\u043E\u0442 \u0444\u0430\u0439\u043B \u0435\u0441\u0442\u044C \u0442\u043E\u043B\u044C\u043A\u043E \u0432 {name}.",
  no_text_diff: "\u0412 \u0444\u0430\u0439\u043B\u0435 \u043D\u0435\u0442 \u0440\u0430\u0437\u043B\u0438\u0447\u0438\u044F \u0442\u0435\u043A\u0441\u0442\u0430.",
  metric_col: "\u041C\u0435\u0440\u0430",
  reading_col: "\u0427\u0442\u0435\u043D\u0438\u0435",
  first_one: "\u043F\u0435\u0440\u0432\u0430\u044F",
  second_one: "\u0432\u0442\u043E\u0440\u0430\u044F",
  the_first: "\u041F\u0435\u0440\u0432\u0430\u044F",
  the_second: "\u0412\u0442\u043E\u0440\u0430\u044F",
  different_text: "\u0414\u0440\u0443\u0433\u043E\u0439 \u0442\u0435\u043A\u0441\u0442",
  added_files: "\u041D\u043E\u0432\u044B\u0435 \u0444\u0430\u0439\u043B\u044B",
  removed_files: "\u0423\u0434\u0430\u043B\u0451\u043D\u043D\u044B\u0435 \u0444\u0430\u0439\u043B\u044B",
  campaign_line: "\u041A\u0430\u043C\u043F\u0430\u043D\u0438\u044F {name}",
  problem: "\u041F\u0440\u043E\u0431\u043B\u0435\u043C\u0430: {text}",
  need: "\u041F\u043E\u0442\u0440\u0435\u0431\u043D\u043E\u0441\u0442\u044C: {text}",
  owner: "\u041E\u0442\u0432\u0435\u0442\u0441\u0442\u0432\u0435\u043D\u043D\u044B\u0439: {name}",
  opened_summary: "\u041E\u0442\u043A\u0440\u044B\u0442\u0430 {age}. {variants} \u0433\u0438\u043F\u043E\u0442\u0435\u0437, {trials} \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0439, {rejections} \u043E\u0442\u043A\u043B\u043E\u043D\u0435\u043D\u0438\u0439. {tokens}",
  map_label: "\u041A\u0430\u0440\u0442\u0430:",
  no_hypothesis_yet: "\u0423 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438 \u0435\u0449\u0451 \u043D\u0435\u0442 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B. \u041F\u0435\u0440\u0432\u0430\u044F \u0437\u0430\u043F\u0438\u0441\u044C \u2014 \u043F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u0435 \u043F\u0440\u043E\u0431\u043B\u0435\u043C\u044B.",
  many_origins: "\u0423 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438 {count} \u043F\u0440\u043E\u0438\u0441\u0445\u043E\u0436\u0434\u0435\u043D\u0438\u0439: {claims}.",
  guide_intro: "acvc \u0445\u0440\u0430\u043D\u0438\u0442 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B, \u043F\u043E\u0441\u044B\u043B\u043A\u0443 \u043A\u0430\u0436\u0434\u043E\u0439 \u0438 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F. \u0413\u0440\u0430\u043D\u0438\u0446\u0430 \u043F\u043E\u043A\u0430\u0437\u044B\u0432\u0430\u0435\u0442 \u0442\u0435, \u043A\u043E\u0442\u043E\u0440\u044B\u0435 \u043D\u0438 \u043E\u0434\u043D\u0430 \u0434\u0440\u0443\u0433\u0430\u044F \u043D\u0435 \u043F\u0440\u0435\u0432\u0437\u043E\u0448\u043B\u0430. \u0425\u0443\u0434\u0448\u0430\u044F \u043F\u0440\u043E\u0431\u0430 \u043D\u0435 \u0443\u0431\u0438\u0440\u0430\u0435\u0442 \u043B\u0443\u0447\u0448\u0443\u044E \u0442\u043E\u0439 \u0436\u0435 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B. \u041E\u0442\u043A\u043B\u043E\u043D\u0435\u043D\u0438\u0435 \u2014 \u0434\u0435\u0439\u0441\u0442\u0432\u0438\u0435 \u0447\u0435\u043B\u043E\u0432\u0435\u043A\u0430.",
  none_stays: "\u041D\u0438 \u043E\u0434\u043D\u0430 \u043D\u0435 \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F.",
  metric_of_campaign: "\u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438",
  no_campaign: "\u0431\u0435\u0437 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438",
  campaigns: "\u041A\u0430\u043C\u043F\u0430\u043D\u0438\u0438",
  frontier_screen: "\u0413\u0440\u0430\u043D\u0438\u0446\u0430",
  lineage_screen: "\u0420\u043E\u0434\u043E\u0441\u043B\u043E\u0432\u043D\u0430\u044F",
  premise_screen: "\u041F\u043E\u0441\u044B\u043B\u043A\u0430",
  compare_screen: "\u0421\u0440\u0430\u0432\u043D\u0438\u0442\u044C",
  recall_screen: "\u041F\u043E\u0438\u0441\u043A",
  help_search: "Enter \u0438\u0449\u0435\u0442. Esc \u043E\u0442\u043C\u0435\u043D\u044F\u0435\u0442. Backspace \u0441\u0442\u0438\u0440\u0430\u0435\u0442.",
  help_campaigns: "Enter \u043E\u0442\u043A\u0440\u044B\u0432\u0430\u0435\u0442 \u0433\u0440\u0430\u043D\u0438\u0446\u0443. \u2190 \u2192 \u043C\u0435\u043D\u044F\u0435\u0442 \u044D\u043A\u0440\u0430\u043D. q \u0432\u044B\u0445\u043E\u0434\u0438\u0442.",
  help_frontier: "Enter \u043E\u0442\u043A\u0440\u044B\u0432\u0430\u0435\u0442 \u043F\u043E\u0441\u044B\u043B\u043A\u0443. d \u0441\u0440\u0430\u0432\u043D\u0438\u0432\u0430\u0435\u0442 \u0434\u0432\u0435. \u2190 \u2192 \u043C\u0435\u043D\u044F\u0435\u0442 \u044D\u043A\u0440\u0430\u043D. q \u0432\u044B\u0445\u043E\u0434\u0438\u0442.",
  help_lineage: "Enter \u043E\u0442\u043A\u0440\u044B\u0432\u0430\u0435\u0442 \u044D\u0442\u0443 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0443. \u2190 \u2192 \u043C\u0435\u043D\u044F\u0435\u0442 \u044D\u043A\u0440\u0430\u043D. q \u0432\u044B\u0445\u043E\u0434\u0438\u0442.",
  help_detail: "\u0413\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u0438 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u044F. \u2190 \u2192 \u043C\u0435\u043D\u044F\u0435\u0442 \u044D\u043A\u0440\u0430\u043D. q \u0432\u044B\u0445\u043E\u0434\u0438\u0442.",
  help_diff: "Enter \u043F\u043E\u043A\u0430\u0437\u044B\u0432\u0430\u0435\u0442 \u0444\u0430\u0439\u043B. \u2190 \u2192 \u043C\u0435\u043D\u044F\u0435\u0442 \u044D\u043A\u0440\u0430\u043D. q \u0432\u044B\u0445\u043E\u0434\u0438\u0442.",
  no_campaigns: "\u041D\u0435\u0442 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438.",
  open_lineage: "\u041E\u0442\u043A\u0440\u043E\u0439\u0442\u0435 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0443 \u043D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435, \u0447\u0442\u043E\u0431\u044B \u0443\u0432\u0438\u0434\u0435\u0442\u044C, \u043E\u0442\u043A\u0443\u0434\u0430 \u043E\u043D\u0430.",
  no_before_after: "\u041D\u0435\u0442 \u0431\u043E\u043B\u0435\u0435 \u0440\u0430\u043D\u043D\u0435\u0439 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B \u0438 \u043D\u0438\u0447\u0435\u0433\u043E \u043D\u0435 \u0431\u044B\u043B\u043E \u043F\u043E\u0442\u043E\u043C.",
  intent_gap: "\u041D\u0430\u043C\u0435\u0440\u0435\u043D\u0438\u0435  {intent}",
  assumptions_gap: "\u0414\u043E\u043F\u0443\u0449\u0435\u043D\u0438\u044F  {items}",
  came_then: "\u041F\u0440\u0438\u0448\u043B\u0430 \u043E\u0442 {from}    \u041F\u043E\u0442\u043E\u043C {after}",
  recall_prompt: "\u0412\u0432\u0435\u0434\u0438\u0442\u0435 \u0441\u043B\u043E\u0432\u043E \u0438\u0437 \u043F\u043E\u0441\u044B\u043B\u043A\u0438 \u0438 \u043D\u0430\u0436\u043C\u0438\u0442\u0435 Enter.",
  came_from_one: "\u043F\u0440\u0438\u0448\u043B\u0430 \u043E\u0442 {claim}",
  pick_campaign: "\u0412\u044B\u0431\u0435\u0440\u0438\u0442\u0435 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044E.",
  first_marked: "\u041F\u0435\u0440\u0432\u0430\u044F \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043E\u0442\u043C\u0435\u0447\u0435\u043D\u0430. \u041D\u0430\u0436\u043C\u0438\u0442\u0435 d \u043D\u0430 \u0434\u0440\u0443\u0433\u043E\u0439.",
  first_named: "\u041F\u0435\u0440\u0432\u0430\u044F: {claim}. \u041D\u0430\u0436\u043C\u0438\u0442\u0435 d \u043D\u0430 \u0434\u0440\u0443\u0433\u043E\u0439.",
  ago_under_minute: "\u043C\u0435\u043D\u044C\u0448\u0435 \u043C\u0438\u043D\u0443\u0442\u044B \u043D\u0430\u0437\u0430\u0434",
  ago_minutes: "{count} \u043C\u0438\u043D \u043D\u0430\u0437\u0430\u0434",
  ago_hours: "{count} \u0447 \u043D\u0430\u0437\u0430\u0434",
  ago_days: "{count} \u0434\u043D. \u043D\u0430\u0437\u0430\u0434",
  no_tokens: "\u0422\u043E\u043A\u0435\u043D\u044B \u043D\u0435 \u0437\u0430\u043F\u0438\u0441\u0430\u043D\u044B. \u0417\u0430\u043F\u0438\u0448\u0438\u0442\u0435 \u0440\u0430\u0441\u0445\u043E\u0434 \u0447\u0435\u0440\u0435\u0437 spend, \u043D\u0435 \u043C\u0435\u043D\u044F\u044F \u043B\u0438\u0447\u043D\u043E\u0441\u0442\u044C \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B.",
  tokens_recorded: "{tokens} \u0442\u043E\u043A\u0435\u043D\u043E\u0432 \u0443\u0436\u0435 \u0437\u0430\u043F\u0438\u0441\u0430\u043D\u043E",
  rejection_label: "\u043E\u0442\u043A\u043B\u043E\u043D\u0435\u043D\u0438\u0435: {reason}",
  guide_example: "\u041A\u0430\u0440\u0442\u0430 \u043E\u0442\u043A\u0440\u044B\u0442\u043E\u0439 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438. \u0411\u0435\u0437 \u0440\u0435\u043F\u043E\u0437\u0438\u0442\u043E\u0440\u0438\u044F \u043F\u0440\u0438\u043C\u0435\u0440\u0430 \u043D\u0435\u0442.",
  guide_text: "\u0427\u0438\u0442\u0430\u0439\u0442\u0435 context \u043E\u0442\u043A\u0440\u044B\u0442\u043E\u0439 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438. \u041A\u0430\u0440\u0442\u0430 \u044D\u0442\u043E\u0433\u043E \u0440\u0435\u043F\u043E\u0437\u0438\u0442\u043E\u0440\u0438\u044F \u2014 \u043F\u0440\u0438\u043C\u0435\u0440.\n\u041F\u043E\u0441\u044B\u043B\u043A\u0430 \u2014 \u044D\u0442\u043E \u043F\u0440\u043E\u0431\u043B\u0435\u043C\u0430. \u041F\u0440\u0438\u0447\u0438\u043D\u0430 \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F \u0443 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B, \u043D\u0435 \u0432 \u043E\u0434\u043D\u043E\u0439 \u0444\u0440\u0430\u0437\u0435 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438.\nscore \u0437\u0430\u043F\u0438\u0441\u044B\u0432\u0430\u0435\u0442 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435. \u0425\u0443\u0434\u0448\u0430\u044F \u043F\u0440\u043E\u0431\u0430 \u043D\u0435 \u0443\u0431\u0438\u0440\u0430\u0435\u0442 \u043B\u0443\u0447\u0448\u0443\u044E \u0441 \u0433\u0440\u0430\u043D\u0438\u0446\u044B.\nseed \u043E\u0442\u043A\u0440\u044B\u0432\u0430\u0435\u0442 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u044E \u0432 \u043F\u0430\u043F\u043A\u0435 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B. \u041E\u0431\u0445\u043E\u0434 \u043D\u0435 \u0447\u0438\u0442\u0430\u0435\u0442 .gitignore. .acvcignore \u0432 \u043F\u0430\u043F\u043A\u0435 \u0443\u0431\u0438\u0440\u0430\u0435\u0442 \u0442\u0435 \u0436\u0435 \u043F\u0443\u0442\u0438 \u0432 \u043A\u0430\u0436\u0434\u043E\u043C \u043A\u043B\u043E\u043D\u0435. seed \u0433\u043E\u0432\u043E\u0440\u0438\u0442, \u0441\u043A\u043E\u043B\u044C\u043A\u043E \u0444\u0430\u0439\u043B\u043E\u0432 \u0432\u043E\u0448\u043B\u043E \u0438 \u043A\u0430\u043A\u0438\u0435 \u0432\u0435\u0441\u0438\u043B\u0438 \u0431\u043E\u043B\u044C\u0448\u0435. \u0425\u0440\u0430\u043D\u0438\u043B\u0438\u0449\u0435 \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F \u0432 --repo; \u0432 \u043F\u0430\u043F\u043A\u0435 \u043E\u0441\u0442\u0430\u044E\u0442\u0441\u044F \u0434\u0432\u0430 \u043C\u0430\u0440\u043A\u0435\u0440\u0430, \u0432 .gitignore. \u0421\u043B\u0435\u0434\u0443\u044E\u0449\u0430\u044F \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u0438\u0441\u043F\u043E\u043B\u044C\u0437\u0443\u0435\u0442 record \u0438 \u0438\u043C\u0435\u0435\u0442 \u0440\u043E\u0434\u0438\u0442\u0435\u043B\u044F.\n",
  status_frontier_why: "\u0418\u0437\u043C\u0435\u0440\u0435\u043D\u0430, \u0438 \u043D\u0438 \u043E\u0434\u043D\u0430 \u0434\u0440\u0443\u0433\u0430\u044F \u0435\u0451 \u043D\u0435 \u043F\u0440\u0435\u0432\u043E\u0441\u0445\u043E\u0434\u0438\u0442.",
  status_beaten_why: "\u0418\u0437\u043C\u0435\u0440\u0435\u043D\u0430, \u0438 \u0434\u0440\u0443\u0433\u0430\u044F \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043D\u0430 \u0433\u0440\u0430\u043D\u0438\u0446\u0435 \u043B\u0443\u0447\u0448\u0435.",
  status_rejected_why: "\u041C\u043E\u0442\u0438\u0432 \u0435\u0441\u0442\u044C \u0432 \u0433\u0440\u0430\u0444\u0435.",
  status_unmeasured_why: "\u0415\u0449\u0451 \u043D\u0435\u0442 \u043F\u043E\u043B\u043D\u043E\u0433\u043E score \u0434\u043B\u044F \u043C\u0435\u0440\u044B \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438.",
  when_context: "\u041F\u0435\u0440\u0435\u0434 \u0434\u0435\u0439\u0441\u0442\u0432\u0438\u0435\u043C. \u0412\u043E\u0437\u0432\u0440\u0430\u0449\u0430\u0435\u0442 \u043A\u0430\u0440\u0442\u0443 \u044D\u0442\u043E\u0439 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438.",
  when_guide: "\u041A\u043E\u043C\u0430\u043D\u0434\u044B \u0438 \u043A\u0430\u0440\u0442\u0430 \u043E\u0442\u043A\u0440\u044B\u0442\u043E\u0433\u043E \u0440\u0435\u043F\u043E\u0437\u0438\u0442\u043E\u0440\u0438\u044F.",
  when_frontier: "\u0422\u043E\u043B\u044C\u043A\u043E \u0442\u043E, \u0447\u0442\u043E \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435 \u0435\u0449\u0451 \u043D\u0435 \u043F\u0440\u0435\u0432\u0437\u043E\u0448\u043B\u043E.",
  when_why: "\u041E\u0434\u043D\u0430 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430, \u0441 \u043F\u0440\u0438\u0447\u0438\u043D\u043E\u0439, \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0435\u043C \u0438 \u0442\u0435\u043C, \u043A\u0442\u043E \u0435\u0451 \u043F\u0440\u0435\u0432\u0437\u043E\u0448\u0451\u043B.",
  when_recall: "\u041D\u0430\u0439\u0442\u0438 \u043F\u043E\u0441\u044B\u043B\u043A\u0443 \u043F\u043E \u0442\u0435\u043A\u0441\u0442\u0443. \u0412\u043E\u0437\u0432\u0440\u0430\u0449\u0430\u0435\u0442 \u0441\u0442\u0430\u0442\u0443\u0441 \u0438 \u0440\u043E\u0434\u0438\u0442\u0435\u043B\u044F.",
  when_seed: "\u041F\u0435\u0440\u0432\u0430\u044F \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438. \u041F\u043E\u0441\u0435\u0439\u0442\u0435 \u043F\u0430\u043F\u043A\u0443 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B: \u043E\u0431\u0445\u043E\u0434 \u043D\u0435 \u0447\u0438\u0442\u0430\u0435\u0442 .gitignore \u0438 \u043F\u0440\u043E\u043F\u0443\u0441\u043A\u0430\u0435\u0442 .git, .acvc, target, node_modules, .build \u0438 DerivedData. .acvcignore \u0432 \u043F\u0430\u043F\u043A\u0435 \u0443\u0431\u0438\u0440\u0430\u0435\u0442 \u0442\u0435 \u0436\u0435 \u043F\u0443\u0442\u0438 \u0432 \u043A\u0430\u0436\u0434\u043E\u043C \u043A\u043B\u043E\u043D\u0435; \u0431\u0435\u0437 \u043D\u0435\u0433\u043E \u0434\u0435\u0439\u0441\u0442\u0432\u0443\u0435\u0442 \u0444\u0438\u043A\u0441\u0438\u0440\u043E\u0432\u0430\u043D\u043D\u044B\u0439 \u0441\u043F\u0438\u0441\u043E\u043A. seed \u0433\u043E\u0432\u043E\u0440\u0438\u0442, \u0441\u043A\u043E\u043B\u044C\u043A\u043E \u0444\u0430\u0439\u043B\u043E\u0432 \u0432\u043E\u0448\u043B\u043E \u0438 \u043A\u0430\u043A\u0438\u0435 \u0432\u0435\u0441\u0438\u043B\u0438 \u0431\u043E\u043B\u044C\u0448\u0435. \u0425\u0440\u0430\u043D\u0438\u043B\u0438\u0449\u0435 \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F \u0432 --repo. \u0412 \u043F\u0430\u043F\u043A\u0435 \u043E\u0441\u0442\u0430\u044E\u0442\u0441\u044F .acvc-workspace \u0438 .acvc-index, \u043E\u0431\u0430 \u0432 .gitignore.",
  when_record: "\u041F\u0430\u043F\u043A\u0430 \u0438\u0437\u043C\u0435\u043D\u0438\u043B\u0430\u0441\u044C. \u041D\u043E\u0432\u0430\u044F \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u0443\u043A\u0430\u0437\u044B\u0432\u0430\u0435\u0442 \u043D\u0430 \u0440\u043E\u0434\u0438\u0442\u0435\u043B\u044F.",
  when_score: "\u0415\u0441\u0442\u044C \u0447\u0438\u0441\u043B\u043E \u043C\u0435\u0440\u044B \u043A\u0430\u043C\u043F\u0430\u043D\u0438\u0438. \u0425\u0443\u0434\u0448\u0430\u044F \u043F\u0440\u043E\u0431\u0430 \u043D\u0435 \u0437\u0430\u043C\u0435\u043D\u044F\u0435\u0442 \u043B\u0443\u0447\u0448\u0443\u044E.",
  when_reject: "\u0413\u0438\u043F\u043E\u0442\u0435\u0437\u0430 \u0443\u043C\u0435\u0440\u043B\u0430, \u0438 \u043C\u043E\u0442\u0438\u0432 \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F \u0432 \u0433\u0440\u0430\u0444\u0435.",
  when_spend: "\u0421\u0435\u0441\u0441\u0438\u044F \u0437\u043D\u0430\u0435\u0442, \u0441\u043A\u043E\u043B\u044C\u043A\u043E \u0442\u043E\u043A\u0435\u043D\u043E\u0432 \u043F\u043E\u0442\u0440\u0430\u0442\u0438\u043B\u0430.",
  when_compare: "\u0414\u0432\u0435 \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u043D\u044B\u0435 \u0433\u0438\u043F\u043E\u0442\u0435\u0437\u044B, \u0447\u0442\u043E\u0431\u044B \u0443\u0432\u0438\u0434\u0435\u0442\u044C, \u043A\u0430\u043A\u0430\u044F \u043E\u0441\u0442\u0430\u0451\u0442\u0441\u044F \u0438 \u043A\u0430\u043A\u0438\u0435 \u0444\u0430\u0439\u043B\u044B \u0438\u0437\u043C\u0435\u043D\u0438\u043B\u0438\u0441\u044C.",
  when_web: "\u041A\u0430\u0440\u0442\u0430 \u043D\u0430 \u0441\u0442\u0440\u0430\u043D\u0438\u0446\u0435.",
  when_obsidian: "\u0421\u0442\u0430\u0432\u0438\u0442 \u043F\u0440\u043E\u0441\u043C\u043E\u0442\u0440 \u0432 vault \u0438 \u0443\u043A\u0430\u0437\u044B\u0432\u0430\u0435\u0442 \u043D\u0430 \u044D\u0442\u043E\u0442 \u0440\u0435\u043F\u043E\u0437\u0438\u0442\u043E\u0440\u0438\u0439. \u041A\u043E\u043C\u0430\u043D\u0434\u0430 \u0432 Obsidian \u2014 Open ACVC.",
  none_count: "\u043D\u0438\u0447\u0435\u0433\u043E",
  help_recall: "/ \u0438\u0449\u0435\u0442. Enter \u0438\u0449\u0435\u0442. \u2190 \u2192 \u043C\u0435\u043D\u044F\u0435\u0442 \u044D\u043A\u0440\u0430\u043D. q \u0432\u044B\u0445\u043E\u0434\u0438\u0442.",
  came_label: "\u041F\u0440\u0438\u0448\u043B\u0430 \u043E\u0442",
  problem_label: "\u041F\u0440\u043E\u0431\u043B\u0435\u043C\u0430",
  better_here: "{name} \u0437\u0434\u0435\u0441\u044C \u043B\u0443\u0447\u0448\u0435.",
  only_side: "\u0422\u043E\u043B\u044C\u043A\u043E \u0432 {name}",
  web_campaign_fact: "{age} \xB7 {variants} \u0433\u0438\u043F\u043E\u0442\u0435\u0437 \xB7 {trials} \u0438\u0437\u043C\u0435\u0440\u0435\u043D\u0438\u0439 \xB7 {tokens}",
  only_other: "\u0442\u043E\u043B\u044C\u043A\u043E \u0432 \u0434\u0440\u0443\u0433\u043E\u0439  {path}",
  only_first: "\u0442\u043E\u043B\u044C\u043A\u043E \u0432 \u043F\u0435\u0440\u0432\u043E\u0439  {path}",
  changed_path: "\u0438\u0437\u043C\u0435\u043D\u0438\u043B\u043E\u0441\u044C  {path}",
  the_other: "\u0414\u0440\u0443\u0433\u0430\u044F",
  after_named: "\u0427\u0442\u043E \u0431\u044B\u043B\u043E \u043F\u043E\u0442\u043E\u043C: {names}",
  assistant_pack: "\u041A\u043E\u043D\u0442\u0440\u0430\u043A\u0442 \u0434\u043B\u044F \u0434\u0440\u0443\u0433\u043E\u0433\u043E \u0430\u0433\u0435\u043D\u0442\u0430 \u0432\u044B\u0434\u0430\u0451\u0442 `acvc assistant`, \u043D\u0430 \u044F\u0437\u044B\u043A\u0435 \u044D\u0442\u043E\u0433\u043E \u043A\u043E\u043C\u043F\u044C\u044E\u0442\u0435\u0440\u0430. \u0410\u0433\u0435\u043D\u0442 \u0441 MCP ASB \u043E\u0442\u043F\u0440\u0430\u0432\u043B\u044F\u0435\u0442 \u043C\u0430\u0441\u0441\u0438\u0432 documents \u0432 rag.index, \u0430 \u043F\u043E\u0442\u043E\u043C \u0438\u0449\u0435\u0442 \u0447\u0435\u0440\u0435\u0437 rag.search.",
  assistant_index: "\u0415\u0441\u043B\u0438 \u0432 \u044D\u0442\u043E\u0439 \u0441\u0435\u0441\u0441\u0438\u0438 \u0435\u0441\u0442\u044C MCP ASB, \u043E\u0442\u043F\u0440\u0430\u0432\u044C\u0442\u0435 \u043C\u0430\u0441\u0441\u0438\u0432 documents \u0432 rag.index \u043E\u0434\u0438\u043D \u0440\u0430\u0437. \u0414\u0430\u043B\u044C\u043D\u0435\u0439\u0448\u0438\u0439 \u043F\u043E\u0438\u0441\u043A \u0438\u0434\u0451\u0442 \u0447\u0435\u0440\u0435\u0437 rag.search.",
  when_assistant: "\u0412\u043E\u0437\u0432\u0440\u0430\u0449\u0430\u0435\u0442 \u043E\u043F\u0435\u0440\u0430\u0446\u0438\u043E\u043D\u043D\u044B\u0439 \u043A\u043E\u043D\u0442\u0440\u0430\u043A\u0442 \u043D\u0430 \u044F\u0437\u044B\u043A\u0435 \u044D\u0442\u043E\u0433\u043E \u043A\u043E\u043C\u043F\u044C\u044E\u0442\u0435\u0440\u0430, \u0433\u043E\u0442\u043E\u0432\u044B\u0439 \u0434\u043B\u044F rag.index."
};

// ../client/src/locale.ts
var catalogs = {
  "pt-BR": pt_BR_default,
  "en-US": en_US_default,
  es: es_default,
  fr: fr_default,
  nl: nl_default,
  el: el_default,
  pl: pl_default,
  ru: ru_default
};
function parseTag(raw) {
  const bare = (raw ?? "").trim().split(/[.@]/)[0]?.replaceAll("_", "-").toLowerCase() ?? "";
  if (!bare || bare === "c" || bare === "posix") return void 0;
  const primary = bare.split("-")[0];
  if (primary === "pt") return "pt-BR";
  if (primary === "en") return "en-US";
  if (primary === "es" || primary === "fr" || primary === "nl" || primary === "el" || primary === "pl" || primary === "ru") {
    return primary;
  }
  return void 0;
}
function detect() {
  const fromEnv = parseTag(process.env.LC_ALL) ?? parseTag(process.env.LC_MESSAGES) ?? parseTag(process.env.LANG);
  if (fromEnv) return fromEnv;
  const nav = typeof navigator === "undefined" ? "" : navigator.language;
  return parseTag(nav) ?? "en-US";
}
function text(key, lang = detect()) {
  return catalogs[lang][key] || catalogs["en-US"][key] || key;
}
function fill(key, vars, lang) {
  let sentence = text(key, lang);
  for (const [name, value] of Object.entries(vars)) {
    sentence = sentence.replaceAll(`{${name}}`, value);
  }
  return sentence;
}
function statusLabel(code, lang) {
  if (code === "on_frontier") return text("status_frontier", lang);
  if (code === "superseded") return text("status_beaten", lang);
  if (code === "rejected") return text("status_rejected", lang);
  if (code === "unmeasured") return text("no_measurement", lang);
  return code;
}

// ../client/src/format.ts
function metricsText(metrics) {
  return Object.entries(metrics).map(([name, value]) => `${name} ${value}`).join(" ");
}
function frontierLabel(row) {
  const metrics = metricsText(row.metrics);
  return metrics ? `${row.claim}  ${metrics}` : row.claim;
}
function whyText(why, lang) {
  const lines = [why.claim];
  if (why.assumptions.length > 0) lines.push(fill("cause", { items: why.assumptions.join(" \xB7 ") }, lang));
  if (why.intent) lines.push(fill("intent", { intent: why.intent }, lang));
  const trial = why.trials[0];
  const measured = trial ? metricsText(trial.metrics) || text("no_measurement", lang) : text("no_measurement", lang);
  lines.push(fill("measurement", { text: measured }, lang));
  if (why.rejection) lines.push(fill("rejected", { reason: why.rejection }, lang));
  else if (why.on_frontier) lines.push(text("on_frontier", lang));
  else lines.push(text("motive_unrecorded", lang));
  for (const winner of why.beaten_by ?? []) {
    const gaps = winner.gaps.map((gap) => fill("against", { name: gap.name, loser: String(gap.loser), winner: String(gap.winner), sense: sense(gap.direction, lang) }, lang)).join("; ");
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
function compareText(report, lang) {
  const left = report.left_claim || text("the_first", lang);
  const right = report.right_claim || text("the_other", lang);
  const lines = [verdict(report.dominance, left, right, lang)];
  for (const delta of report.metrics) {
    lines.push(fill("against", {
      name: delta.name,
      loser: number(delta.left, lang),
      winner: number(delta.right, lang),
      sense: sense(delta.direction, lang)
    }, lang));
  }
  return lines.join("\n");
}
function recallText(report, lang) {
  if (report.hits.length === 0) return text("no_recall", lang);
  return report.hits.map((hit) => {
    const parents = hit.parents ?? [];
    const from = parents.length === 0 ? text("opened_campaign", lang) : fill("came_from", { names: parents.map((parent) => parent.claim).join("; ") }, lang);
    return `${hit.claim}
${statusLabel(hit.status || "unmeasured", lang)}
${from}`;
  }).join("\n\n");
}
function pathLabel(kind, path, lang) {
  if (kind === "added") return fill("only_other", { path }, lang);
  if (kind === "removed") return fill("only_first", { path }, lang);
  return fill("changed_path", { path }, lang);
}
function verdict(kind, left, right, lang) {
  if (kind === "left") return fill("stays", { claim: left }, lang);
  if (kind === "right") return fill("stays", { claim: right }, lang);
  if (kind === "tie") return fill("tie_both", { left, right }, lang);
  if (kind === "incomparable") return text("incomparable", lang);
  return text("missing_measure_which", lang);
}
function sense(direction, lang) {
  return direction === "lower" ? text("lower_better", lang) : text("higher_better", lang);
}
function number(value, lang) {
  if (value == null) return text("no_measurement", lang);
  return Number.isInteger(value) ? String(value) : String(value);
}
function diffPaths(report) {
  return [
    ...report.added.map((path) => ({ kind: "added", path })),
    ...report.removed.map((path) => ({ kind: "removed", path })),
    ...report.modified.map((path) => ({ kind: "modified", path }))
  ];
}

// src/view.ts
var VIEW_TYPE = "acvc-viewer";
var AcvcView = class extends import_obsidian2.ItemView {
  constructor(leaf, plugin) {
    super(leaf);
    this.plugin = plugin;
  }
  campaigns = [];
  campaign = "";
  frontier = [];
  one = "";
  other = "";
  oneId = "";
  otherId = "";
  paths = [];
  body = "";
  error = "";
  getViewType() {
    return VIEW_TYPE;
  }
  getDisplayText() {
    return "ACVC";
  }
  async onOpen() {
    await this.reload();
  }
  client() {
    const adapter = this.app.vault.adapter;
    if (!(adapter instanceof import_obsidian2.FileSystemAdapter)) throw new Error("O ACVC precisa do Obsidian no computador.");
    const repo = this.plugin.settings.repo.trim();
    return new Acvc({
      binary: this.plugin.settings.binary.trim() || "acvc",
      cwd: adapter.getBasePath(),
      repo: repo || void 0
    });
  }
  async reload() {
    try {
      this.campaigns = await this.client().campaigns();
      this.campaign = this.campaigns.includes(this.campaign) ? this.campaign : this.campaigns[0] ?? "";
      this.error = "";
      if (this.campaign) await this.loadFrontier();
      else this.draw();
    } catch (err) {
      this.fail(err);
    }
  }
  async choose(name) {
    this.campaign = name;
    this.paths = [];
    this.error = "";
    await this.loadFrontier();
  }
  async loadFrontier() {
    const report = await this.client().frontier(this.campaign);
    this.frontier = report.variants.map((row) => ({
      variant: row.variant,
      claim: row.claim,
      label: frontierLabel(row)
    }));
    this.body = this.frontier.length === 0 ? "Nenhuma hip\xF3tese na fronteira." : "";
    this.draw();
  }
  async showWhy(variant) {
    this.body = whyText(await this.client().why(variant));
    this.draw();
  }
  mark(side, variant) {
    const claim = this.frontier.find((row) => row.variant === variant)?.claim ?? "";
    if (side === "one") {
      this.oneId = variant;
      this.one = claim;
    } else {
      this.otherId = variant;
      this.other = claim;
    }
    this.draw();
  }
  async compare() {
    if (!this.oneId || !this.otherId) {
      this.error = "Escolha duas hip\xF3teses.";
      this.draw();
      return;
    }
    const report = await this.client().compare(this.oneId, this.otherId);
    this.paths = diffPaths(report).map((entry) => ({
      ...entry,
      label: pathLabel(entry.kind, entry.path)
    }));
    this.body = compareText(report);
    this.draw();
  }
  async openFile(file) {
    this.body = (await this.client().fileDiff(this.oneId, this.otherId, file)).diff;
    this.draw();
  }
  async search(query) {
    this.body = recallText(await this.client().recall(this.campaign, query));
    this.draw();
  }
  fail(err) {
    this.error = err instanceof Error ? err.message : String(err);
    this.draw();
  }
  draw() {
    const root = this.contentEl;
    root.empty();
    root.addClass("acvc-view");
    const bar = root.createDiv({ cls: "acvc-bar" });
    const select = bar.createEl("select");
    for (const name of this.campaigns) {
      const option = select.createEl("option", { text: name });
      option.value = name;
      option.selected = name === this.campaign;
    }
    select.onchange = () => {
      void this.run(() => this.choose(select.value));
    };
    const query = bar.createEl("input", { type: "text", placeholder: "o que a hip\xF3tese afirma" });
    const search = bar.createEl("button", { text: "Buscar" });
    search.onclick = () => {
      void this.run(() => this.search(query.value));
    };
    if (this.error) root.createEl("p", { cls: "acvc-error", text: this.error });
    root.createEl("p", { text: `Uma: ${this.one || "nenhuma"}    Outra: ${this.other || "nenhuma"}` });
    if (this.one && this.other) {
      const compare = root.createEl("button", { text: "Qual fica" });
      compare.onclick = () => {
        void this.run(() => this.compare());
      };
    }
    for (const row of this.frontier) {
      const line = root.createDiv({ cls: "acvc-row" });
      line.createSpan({ text: row.label });
      this.button(line, "Premissa", () => this.showWhy(row.variant));
      this.button(line, "Uma", async () => this.mark("one", row.variant));
      this.button(line, "Outra", async () => this.mark("other", row.variant));
    }
    for (const entry of this.paths) {
      const button = root.createEl("button", { text: entry.label });
      button.onclick = () => {
        void this.run(() => this.openFile(entry.path));
      };
    }
    root.createEl("pre", { text: this.body });
  }
  button(parent, label, run) {
    const button = parent.createEl("button", { text: label });
    button.onclick = () => {
      void this.run(run);
    };
  }
  async run(action) {
    try {
      this.error = "";
      await action();
    } catch (err) {
      this.fail(err);
    }
  }
};

// src/main.ts
var AcvcPlugin = class extends import_obsidian3.Plugin {
  settings = DEFAULT_SETTINGS;
  async onload() {
    await this.loadSettings();
    this.registerView(VIEW_TYPE, (leaf) => new AcvcView(leaf, this));
    this.addSettingTab(new AcvcSettingTab(this.app, this));
    this.addRibbonIcon("git-branch", "Open ACVC", () => {
      void this.activate();
    });
    this.addCommand({
      id: "open",
      name: "Open ACVC",
      callback: () => {
        void this.activate();
      }
    });
  }
  onunload() {
    this.app.workspace.detachLeavesOfType(VIEW_TYPE);
  }
  async loadSettings() {
    this.settings = Object.assign({}, DEFAULT_SETTINGS, await this.loadData());
  }
  async saveSettings() {
    await this.saveData(this.settings);
  }
  async activate() {
    const leaf = this.app.workspace.getRightLeaf(false);
    if (!leaf) return;
    await leaf.setViewState({ type: VIEW_TYPE, active: true });
    await this.app.workspace.revealLeaf(leaf);
  }
};
