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
      (text) => text.setValue(this.plugin.settings.binary).onChange(async (value) => {
        this.plugin.settings.binary = value.trim() || "acvc";
        await this.plugin.saveSettings();
      })
    );
    new import_obsidian.Setting(containerEl).setName("repository").setDesc("Folder that contains .acvc. Empty uses the vault folder.").addText(
      (text) => text.setValue(this.plugin.settings.repo).onChange(async (value) => {
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

// ../client/src/format.ts
function metricsText(metrics) {
  return Object.entries(metrics).map(([name, value]) => `${name}=${value}`).join(" ");
}
function frontierLabel(row) {
  const metrics = metricsText(row.metrics);
  return metrics ? `${row.claim}  ${metrics}` : row.claim;
}
function whyText(why) {
  const lines = [
    why.claim,
    `edge ${why.edge}`,
    `agent ${why.agent}`,
    why.intent ? `intent ${why.intent}` : "",
    `assumptions ${why.assumptions.join("; ") || "none"}`,
    `parents ${why.parents.join(" ") || "none"}`,
    `children ${why.children.join(" ") || "none"}`
  ];
  for (const trial of why.trials) lines.push(`trial ${trial.id} ${metricsText(trial.metrics)}`);
  if (why.rejection) lines.push(`rejected ${why.rejection}`);
  return lines.filter((line) => line.length > 0).join("\n");
}
function compareText(report) {
  const lines = [`dominance ${report.dominance}`];
  for (const delta of report.metrics) lines.push(`${delta.name} ${delta.left ?? "-"} -> ${delta.right ?? "-"}`);
  for (const path of report.added) lines.push(`added ${path}`);
  for (const path of report.removed) lines.push(`removed ${path}`);
  for (const path of report.modified) lines.push(`modified ${path}`);
  return lines.join("\n");
}
function recallText(report) {
  if (report.hits.length === 0) return "no hits";
  return report.hits.map((hit) => `${hit.claim}
${hit.variant}`).join("\n\n");
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
  left = "";
  right = "";
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
    if (!(adapter instanceof import_obsidian2.FileSystemAdapter)) throw new Error("ACVC needs the desktop app");
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
    this.frontier = report.variants.map((row) => ({ variant: row.variant, label: frontierLabel(row) }));
    this.body = this.frontier.length === 0 ? "frontier is empty" : "";
    this.draw();
  }
  async showWhy(variant) {
    this.body = whyText(await this.client().why(variant));
    this.draw();
  }
  mark(side, variant) {
    if (side === "left") this.left = variant;
    else this.right = variant;
    this.draw();
  }
  async compare() {
    const report = await this.client().compare(this.left, this.right);
    this.paths = diffPaths(report);
    this.body = compareText(report);
    this.draw();
  }
  async openFile(file) {
    this.body = (await this.client().fileDiff(this.left, this.right, file)).diff;
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
    const query = bar.createEl("input", { type: "text", placeholder: "recall" });
    const search = bar.createEl("button", { text: "Search" });
    search.onclick = () => {
      void this.run(() => this.search(query.value));
    };
    if (this.error) root.createEl("p", { cls: "acvc-error", text: this.error });
    root.createEl("p", { text: `left ${this.left || "none"}    right ${this.right || "none"}` });
    if (this.left && this.right) {
      const compare = root.createEl("button", { text: "compare" });
      compare.onclick = () => {
        void this.run(() => this.compare());
      };
    }
    for (const row of this.frontier) {
      const line = root.createDiv({ cls: "acvc-row" });
      line.createSpan({ text: row.label });
      this.button(line, "premise", () => this.showWhy(row.variant));
      this.button(line, "left", async () => this.mark("left", row.variant));
      this.button(line, "right", async () => this.mark("right", row.variant));
    }
    for (const entry of this.paths) {
      const button = root.createEl("button", { text: `${entry.kind} ${entry.path}` });
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
