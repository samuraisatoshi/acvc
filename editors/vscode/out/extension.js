"use strict";
var __create = Object.create;
var __defProp = Object.defineProperty;
var __getOwnPropDesc = Object.getOwnPropertyDescriptor;
var __getOwnPropNames = Object.getOwnPropertyNames;
var __getProtoOf = Object.getPrototypeOf;
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
var __toESM = (mod, isNodeMode, target) => (target = mod != null ? __create(__getProtoOf(mod)) : {}, __copyProps(
  // If the importer is in node compatibility mode or this is not an ESM
  // file that has been converted to a CommonJS file using a Babel-
  // compatible transform (i.e. "__esModule" has not been set), then set
  // "default" to the CommonJS "module.exports" for node compatibility.
  isNodeMode || !mod || !mod.__esModule ? __defProp(target, "default", { value: mod, enumerable: true }) : target,
  mod
));
var __toCommonJS = (mod) => __copyProps(__defProp({}, "__esModule", { value: true }), mod);

// src/extension.ts
var extension_exports = {};
__export(extension_exports, {
  activate: () => activate,
  deactivate: () => deactivate
});
module.exports = __toCommonJS(extension_exports);
var vscode = __toESM(require("vscode"));

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

// src/page.ts
function pageHtml() {
  return `<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8" />
<meta http-equiv="Content-Security-Policy" content="default-src 'none'; style-src 'unsafe-inline'; script-src 'unsafe-inline';" />
<style>
  body { font: 13px/1.45 ui-sans-serif, sans-serif; margin: 0; color: var(--vscode-foreground); background: var(--vscode-editor-background); }
  header, main { padding: 12px; }
  header { display: flex; gap: 8px; align-items: center; flex-wrap: wrap; border-bottom: 1px solid var(--vscode-panel-border); }
  select, button, input { font: inherit; color: inherit; background: var(--vscode-input-background); border: 1px solid var(--vscode-input-border, transparent); padding: 4px 8px; }
  ul { list-style: none; padding: 0; margin: 8px 0; }
  li { display: flex; gap: 8px; align-items: center; padding: 4px 0; }
  .claim { flex: 1; }
  pre { white-space: pre-wrap; }
  .error { color: var(--vscode-errorForeground); }
</style>
</head>
<body>
<header>
  <label>Campaign <select id="campaign"></select></label>
  <form id="recall"><input id="query" placeholder="recall" /><button>Search</button></form>
</header>
<main>
  <p id="marks"></p>
  <p id="error" class="error"></p>
  <div id="actions"></div>
  <ul id="frontier"></ul>
  <ul id="paths"></ul>
  <pre id="body"></pre>
</main>
<script>
const vscode = acquireVsCodeApi();
const byId = (id) => document.getElementById(id);
function button(label, message) {
  const node = document.createElement("button");
  node.type = "button";
  node.textContent = label;
  node.addEventListener("click", () => vscode.postMessage(message));
  return node;
}
function render(state) {
  const campaign = byId("campaign");
  campaign.replaceChildren();
  for (const name of state.campaigns) {
    const option = document.createElement("option");
    option.value = name;
    option.textContent = name;
    option.selected = name === state.campaign;
    campaign.append(option);
  }
  byId("marks").textContent = "left " + (state.left || "none") + "    right " + (state.right || "none");
  byId("error").textContent = state.error || "";
  byId("body").textContent = state.body || "";
  const actions = byId("actions");
  actions.replaceChildren();
  if (state.left && state.right) actions.append(button("compare", { type: "compare" }));
  const frontier = byId("frontier");
  frontier.replaceChildren();
  for (const row of state.frontier) {
    const item = document.createElement("li");
    const claim = document.createElement("span");
    claim.className = "claim";
    claim.textContent = row.label;
    item.append(
      claim,
      button("premise", { type: "why", variant: row.variant }),
      button("left", { type: "mark", side: "left", variant: row.variant }),
      button("right", { type: "mark", side: "right", variant: row.variant })
    );
    frontier.append(item);
  }
  const paths = byId("paths");
  paths.replaceChildren();
  for (const entry of state.paths) {
    const item = document.createElement("li");
    item.append(button(entry.kind + " " + entry.path, { type: "file", path: entry.path }));
    paths.append(item);
  }
}
byId("campaign").addEventListener("change", (event) => {
  vscode.postMessage({ type: "campaign", name: event.target.value });
});
byId("recall").addEventListener("submit", (event) => {
  event.preventDefault();
  vscode.postMessage({ type: "recall", query: byId("query").value });
});
window.addEventListener("message", (event) => render(event.data));
</script>
</body>
</html>`;
}

// src/extension.ts
var empty = {
  campaigns: [],
  campaign: "",
  frontier: [],
  left: "",
  right: "",
  paths: [],
  body: "",
  error: ""
};
function activate(context) {
  context.subscriptions.push(vscode.commands.registerCommand("acvc.open", () => open(context)));
}
function deactivate() {
}
function open(context) {
  const panel = vscode.window.createWebviewPanel("acvc", "ACVC", vscode.ViewColumn.One, {
    enableScripts: true
  });
  panel.webview.html = pageHtml();
  const viewer = new Viewer(panel);
  context.subscriptions.push(
    panel.webview.onDidReceiveMessage((message) => {
      void viewer.onMessage(message);
    })
  );
  void viewer.loadCampaigns();
}
var Viewer = class {
  constructor(panel) {
    this.panel = panel;
  }
  state = { ...empty };
  async loadCampaigns() {
    try {
      const campaigns = await this.client().campaigns();
      this.state = { ...empty, campaigns, campaign: campaigns[0] ?? "" };
      if (this.state.campaign) await this.loadFrontier();
      else this.post();
    } catch (err) {
      this.fail(err);
    }
  }
  async onMessage(message) {
    try {
      this.state.error = "";
      if (message.type === "campaign") {
        this.state.campaign = message.name;
        this.state.paths = [];
        await this.loadFrontier();
      } else if (message.type === "why") {
        this.state.body = whyText(await this.client().why(message.variant));
        this.post();
      } else if (message.type === "mark") {
        if (message.side === "left") this.state.left = message.variant;
        else this.state.right = message.variant;
        this.post();
      } else if (message.type === "compare") {
        await this.compare();
      } else if (message.type === "file") {
        await this.file(message.path);
      } else {
        this.state.body = recallText(await this.client().recall(this.state.campaign, message.query));
        this.post();
      }
    } catch (err) {
      this.fail(err);
    }
  }
  async loadFrontier() {
    const report = await this.client().frontier(this.state.campaign);
    this.state.frontier = report.variants.map((row) => ({
      variant: row.variant,
      label: frontierLabel(row)
    }));
    this.state.body = this.state.frontier.length === 0 ? "frontier is empty" : "";
    this.post();
  }
  async compare() {
    if (!this.state.left || !this.state.right) {
      this.state.error = "mark two variants before compare";
      this.post();
      return;
    }
    const report = await this.client().compare(this.state.left, this.state.right);
    this.state.paths = diffPaths(report);
    this.state.body = compareText(report);
    this.post();
  }
  async file(file) {
    const report = await this.client().fileDiff(this.state.left, this.state.right, file);
    this.state.body = report.diff;
    this.post();
  }
  client() {
    const folder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!folder) throw new AcvcError("open a folder before using ACVC", {});
    const config = vscode.workspace.getConfiguration("acvc");
    const repo = config.get("repo")?.trim();
    return new Acvc({
      binary: config.get("binary")?.trim() || "acvc",
      cwd: folder,
      repo: repo || void 0
    });
  }
  fail(err) {
    this.state.error = err instanceof Error ? err.message : String(err);
    this.post();
  }
  post() {
    void this.panel.webview.postMessage(this.state);
  }
};
// Annotate the CommonJS export names for ESM import in node:
0 && (module.exports = {
  activate,
  deactivate
});
