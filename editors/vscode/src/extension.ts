import * as vscode from "vscode";
import { Acvc, AcvcError } from "../../client/src/api.ts";
import { compareText, diffPaths, frontierLabel, pathLabel, recallText, whyText, type DiffPath } from "../../client/src/format.ts";
import { pageHtml } from "./page.ts";

interface ViewState {
  campaigns: string[];
  campaign: string;
  frontier: { variant: string; claim: string; label: string }[];
  one: string;
  other: string;
  paths: (DiffPath & { label: string })[];
  body: string;
  error: string;
}

type Inbound =
  | { type: "campaign"; name: string }
  | { type: "why"; variant: string }
  | { type: "mark"; side: "one" | "other"; variant: string }
  | { type: "compare" }
  | { type: "file"; path: string }
  | { type: "recall"; query: string };

const empty: ViewState = {
  campaigns: [],
  campaign: "",
  frontier: [],
  one: "",
  other: "",
  paths: [],
  body: "",
  error: "",
};

export function activate(context: vscode.ExtensionContext): void {
  context.subscriptions.push(vscode.commands.registerCommand("acvc.open", () => open(context)));
}

export function deactivate(): void {}

function open(context: vscode.ExtensionContext): void {
  const panel = vscode.window.createWebviewPanel("acvc", "ACVC", vscode.ViewColumn.One, {
    enableScripts: true,
  });
  panel.webview.html = pageHtml();
  const viewer = new Viewer(panel);
  context.subscriptions.push(
    panel.webview.onDidReceiveMessage((message: Inbound) => {
      void viewer.onMessage(message);
    }),
  );
  void viewer.loadCampaigns();
}

class Viewer {
  private state: ViewState = { ...empty };
  private oneId = "";
  private otherId = "";

  constructor(private readonly panel: vscode.WebviewPanel) {}

  async loadCampaigns(): Promise<void> {
    try {
      const campaigns = await this.client().campaigns();
      this.oneId = "";
      this.otherId = "";
      this.state = { ...empty, campaigns, campaign: campaigns[0] ?? "" };
      if (this.state.campaign) await this.loadFrontier();
      else this.post();
    } catch (err) {
      this.fail(err);
    }
  }

  async onMessage(message: Inbound): Promise<void> {
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
        const claim = this.state.frontier.find((row) => row.variant === message.variant)?.claim ?? "";
        if (message.side === "one") {
          this.oneId = message.variant;
          this.state.one = claim;
        } else {
          this.otherId = message.variant;
          this.state.other = claim;
        }
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

  private async loadFrontier(): Promise<void> {
    const report = await this.client().frontier(this.state.campaign);
    this.state.frontier = report.variants.map((row) => ({
      variant: row.variant,
      claim: row.claim,
      label: frontierLabel(row),
    }));
    this.state.body = this.state.frontier.length === 0 ? "Nenhuma hipótese na fronteira." : "";
    this.post();
  }

  private async compare(): Promise<void> {
    if (!this.oneId || !this.otherId) {
      this.state.error = "Escolha duas hipóteses.";
      this.post();
      return;
    }
    const report = await this.client().compare(this.oneId, this.otherId);
    this.state.paths = diffPaths(report).map((entry) => ({
      ...entry,
      label: pathLabel(entry.kind, entry.path),
    }));
    this.state.body = compareText(report);
    this.post();
  }

  private async file(file: string): Promise<void> {
    const report = await this.client().fileDiff(this.oneId, this.otherId, file);
    this.state.body = report.diff;
    this.post();
  }

  private client(): Acvc {
    const folder = vscode.workspace.workspaceFolders?.[0]?.uri.fsPath;
    if (!folder) throw new AcvcError("Abra uma pasta antes de usar o ACVC.", {});
    const config = vscode.workspace.getConfiguration("acvc");
    const repo = config.get<string>("repo")?.trim();
    return new Acvc({
      binary: config.get<string>("binary")?.trim() || "acvc",
      cwd: folder,
      repo: repo || undefined,
    });
  }

  private fail(err: unknown): void {
    this.state.error = err instanceof Error ? err.message : String(err);
    this.post();
  }

  private post(): void {
    void this.panel.webview.postMessage(this.state);
  }
}
