import { Plugin } from "obsidian";
import { AcvcSettingTab, DEFAULT_SETTINGS, type AcvcSettings } from "./settings.ts";
import { AcvcView, VIEW_TYPE } from "./view.ts";

export default class AcvcPlugin extends Plugin {
  settings: AcvcSettings = DEFAULT_SETTINGS;

  async onload(): Promise<void> {
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
      },
    });
  }

  onunload(): void {
    this.app.workspace.detachLeavesOfType(VIEW_TYPE);
  }

  async loadSettings(): Promise<void> {
    this.settings = Object.assign({}, DEFAULT_SETTINGS, await this.loadData());
  }

  async saveSettings(): Promise<void> {
    await this.saveData(this.settings);
  }

  private async activate(): Promise<void> {
    const leaf = this.app.workspace.getRightLeaf(false);
    if (!leaf) return;
    await leaf.setViewState({ type: VIEW_TYPE, active: true });
    await this.app.workspace.revealLeaf(leaf);
  }
}
