import { FileSystemAdapter, ItemView, WorkspaceLeaf } from "obsidian";
import { Acvc } from "../../client/src/api.ts";
import {
  compareText,
  diffPaths,
  frontierLabel,
  pathLabel,
  recallText,
  whyText,
  type DiffPath,
} from "../../client/src/format.ts";
import type AcvcPlugin from "./main.ts";

export const VIEW_TYPE = "acvc-viewer";

export class AcvcView extends ItemView {
  private campaigns: string[] = [];
  private campaign = "";
  private frontier: { variant: string; claim: string; label: string }[] = [];
  private one = "";
  private other = "";
  private oneId = "";
  private otherId = "";
  private paths: (DiffPath & { label: string })[] = [];
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
    if (!(adapter instanceof FileSystemAdapter)) throw new Error("O ACVC precisa do Obsidian no computador.");
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
    this.frontier = report.variants.map((row) => ({
      variant: row.variant,
      claim: row.claim,
      label: frontierLabel(row),
    }));
    this.body = this.frontier.length === 0 ? "Nenhuma hipótese na fronteira." : "";
    this.draw();
  }

  private async showWhy(variant: string): Promise<void> {
    this.body = whyText(await this.client().why(variant));
    this.draw();
  }

  private mark(side: "one" | "other", variant: string): void {
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

  private async compare(): Promise<void> {
    if (!this.oneId || !this.otherId) {
      this.error = "Escolha duas hipóteses.";
      this.draw();
      return;
    }
    const report = await this.client().compare(this.oneId, this.otherId);
    this.paths = diffPaths(report).map((entry) => ({
      ...entry,
      label: pathLabel(entry.kind, entry.path),
    }));
    this.body = compareText(report);
    this.draw();
  }

  private async openFile(file: string): Promise<void> {
    this.body = (await this.client().fileDiff(this.oneId, this.otherId, file)).diff;
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
    const query = bar.createEl("input", { type: "text", placeholder: "o que a hipótese afirma" });
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
