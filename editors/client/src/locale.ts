import el from "../../../locales/el.json" with { type: "json" };
import enUS from "../../../locales/en-US.json" with { type: "json" };
import es from "../../../locales/es.json" with { type: "json" };
import fr from "../../../locales/fr.json" with { type: "json" };
import nl from "../../../locales/nl.json" with { type: "json" };
import pl from "../../../locales/pl.json" with { type: "json" };
import ptBR from "../../../locales/pt-BR.json" with { type: "json" };
import ru from "../../../locales/ru.json" with { type: "json" };

export type Lang = "pt-BR" | "en-US" | "es" | "fr" | "nl" | "el" | "pl" | "ru";

const catalogs: Record<Lang, Record<string, string>> = {
  "pt-BR": ptBR,
  "en-US": enUS,
  es,
  fr,
  nl,
  el,
  pl,
  ru,
};

export function parseTag(raw: string | undefined): Lang | undefined {
  const bare = (raw ?? "").trim().split(/[.@]/)[0]?.replaceAll("_", "-").toLowerCase() ?? "";
  if (!bare || bare === "c" || bare === "posix") return undefined;
  const primary = bare.split("-")[0];
  if (primary === "pt") return "pt-BR";
  if (primary === "en") return "en-US";
  if (primary === "es" || primary === "fr" || primary === "nl" || primary === "el" || primary === "pl" || primary === "ru") {
    return primary;
  }
  return undefined;
}

export function detect(): Lang {
  const fromEnv = parseTag(process.env.LC_ALL) ?? parseTag(process.env.LC_MESSAGES) ?? parseTag(process.env.LANG);
  if (fromEnv) return fromEnv;
  const nav = typeof navigator === "undefined" ? "" : navigator.language;
  return parseTag(nav) ?? "en-US";
}

export function text(key: string, lang: Lang = detect()): string {
  return catalogs[lang][key] || catalogs["en-US"][key] || key;
}

export function fill(key: string, vars: Record<string, string>, lang?: Lang): string {
  let sentence = text(key, lang);
  for (const [name, value] of Object.entries(vars)) {
    sentence = sentence.replaceAll(`{${name}}`, value);
  }
  return sentence;
}

export function statusLabel(code: string, lang?: Lang): string {
  if (code === "on_frontier") return text("status_frontier", lang);
  if (code === "superseded") return text("status_beaten", lang);
  if (code === "rejected") return text("status_rejected", lang);
  if (code === "unmeasured") return text("no_measurement", lang);
  return code;
}
