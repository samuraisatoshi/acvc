import { FileSystemAdapter, ItemView, WorkspaceLeaf } from "obsidian";
import { Acvc } from "../../client/src/api.ts";
import {
  compareText,
  diffPaths,
  frontierLabel,
  recallText,
  whyText,
  type DiffPath,
} from "../../client/src/format.ts";
import type AcvcPlugin from "./main.ts";

export const VIEW_TYPE = "acvc-viewer";

export class AcvcView extends ItemView {
  private campaigns: string[] = [];
  private campaign = "";
  private frontier: { variant: string; label: string }[] = [];
  private left = "";
  private right = "";
  private paths: DiffPath[] = [];
  private body = "";
  private error = "";

  constructor(
    leaf: WorkspaceLeaf,
    private plugin: AcvcPlugin,
  ) {
    super(leaf);
  }

  getViewType(): string {
    return VIEW_TYPE;
  }

  getDisplayText(): string {
    return "ACVC";
  }

  async onOpen(): Promise<void> {
    await this.reload();
  }

  private client(): Acvc {
    const adapter = this.app.vault.adapter;
    if (!(adapter instanceof FileSystemAdapter)) throw new Error("ACVC needs the desktop app");
    const repo = this.plugin.settings.repo.trim();
    return new Acvc({
      binary: this.plugin.settings.binary.trim() || "acvc",
      cwd: adapter.getBasePath(),
      repo: repo || undefined,
    });
  }

  private async reload(): Promise<void> {
    try {
      this.campaigns = await this.client().campaigns();
      this.campaign = this.campaigns.includes(this.campaign) ? this.campaign : (this.campaigns[0] ?? "");
      this.error = "";
      if (this.campaign) await this.loadFrontier();
      else this.draw();
    } catch (err) {
      this.fail(err);
    }
  }

  private async choose(name: string): Promise<void> {
    this.campaign = name;
    this.paths = [];
    this.error = "";
    await this.loadFrontier();
  }

  private async loadFrontier(): Promise<void> {
    const report = await this.client().frontier(this.campaign);
    this.frontier = report.variants.map((row) => ({ variant: row.variant, label: frontierLabel(row) }));
    this.body = this.frontier.length === 0 ? "frontier is empty" : "";
    this.draw();
  }

  private async showWhy(variant: string): Promise<void> {
    this.body = whyText(await this.client().why(variant));
    this.draw();
  }

  private mark(side: "left" | "right", variant: string): void {
    if (side === "left") this.left = variant;
    else this.right = variant;
    this.draw();
  }

  private async compare(): Promise<void> {
    const report = await this.client().compare(this.left, this.right);
    this.paths = diffPaths(report);
    this.body = compareText(report);
    this.draw();
  }

  private async openFile(file: string): Promise<void> {
    this.body = (await this.client().fileDiff(this.left, this.right, file)).diff;
    this.draw();
  }

  private async search(query: string): Promise<void> {
    this.body = recallText(await this.client().recall(this.campaign, query));
    this.draw();
  }

  private fail(err: unknown): void {
    this.error = err instanceof Error ? err.message : String(err);
    this.draw();
  }

  private draw(): void {
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

  private button(parent: HTMLElement, label: string, run: () => Promise<void>): void {
    const button = parent.createEl("button", { text: label });
    button.onclick = () => {
      void this.run(run);
    };
  }

  private async run(action: () => Promise<void>): Promise<void> {
    try {
      this.error = "";
      await action();
    } catch (err) {
      this.fail(err);
    }
  }
}
