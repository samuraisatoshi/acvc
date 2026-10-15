import { PluginSettingTab, Setting, type App } from "obsidian";
import type AcvcPlugin from "./main.ts";

export interface AcvcSettings {
  binary: string;
  repo: string;
}

export const DEFAULT_SETTINGS: AcvcSettings = {
  binary: "acvc",
  repo: "",
};

export class AcvcSettingTab extends PluginSettingTab {
  constructor(
    app: App,
    private plugin: AcvcPlugin,
  ) {
    super(app, plugin);
  }

  display(): void {
    const { containerEl } = this;
    containerEl.empty();
    new Setting(containerEl)
      .setName("acvc binary")
      .setDesc("Path to the standalone executable. Cargo is not required.")
      .addText((text) =>
        text.setValue(this.plugin.settings.binary).onChange(async (value) => {
          this.plugin.settings.binary = value.trim() || "acvc";
          await this.plugin.saveSettings();
        }),
      );
    new Setting(containerEl)
      .setName("repository")
      .setDesc("Folder that contains .acvc. Empty uses the vault folder.")
      .addText((text) =>
        text.setValue(this.plugin.settings.repo).onChange(async (value) => {
          this.plugin.settings.repo = value.trim();
          await this.plugin.saveSettings();
        }),
      );
  }
}
