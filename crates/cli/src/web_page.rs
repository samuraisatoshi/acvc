pub const PAGE: &str = r##"<!DOCTYPE html>
<html lang="__HTML_LANG__">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<meta name="color-scheme" content="light">
<title>ACVC</title>
<style>
:root {
  color-scheme: light;
  --bg: #f3f0ea;
  --card: #fffcf8;
  --ink: #1c1917;
  --muted: #57534e;
  --line: #e6e1d8;
  --accent: #9f1239;
  --chip: #fff7ed;
  --chip-line: #fed7aa;
  --good: #ecfdf3;
  --good-line: #86efac;
  --bad: #fef2f2;
  --bad-line: #fecaca;
}
html, body { background: var(--bg); color: var(--ink); }
body { margin: 0; font: 15px/1.5 "Segoe UI", system-ui, sans-serif; }
button, select, input { font: inherit; color: var(--ink); }
.shell { max-width: 1080px; margin: 0 auto; padding: 1.75rem 1.25rem 3rem; }
.top { display: flex; justify-content: space-between; gap: 1.5rem; align-items: flex-end; margin-bottom: 1.4rem; }
.eyebrow { margin: 0; font-size: 0.72rem; letter-spacing: 0.12em; text-transform: uppercase; color: var(--muted); }
h1 { margin: 0.15rem 0 0.35rem; font-size: 2rem; letter-spacing: -0.03em; line-height: 1.15; }
.lede { margin: 0; max-width: 40rem; color: var(--muted); }
.campaign { min-width: 12rem; }
.campaign span, .field span, .kicker { display: block; margin-bottom: 0.25rem; font-size: 0.78rem; color: var(--muted); }
select, input[type="search"] { width: 100%; box-sizing: border-box; padding: 0.55rem 0.7rem; border: 1px solid var(--line); border-radius: 10px; background: #fff; }
.workspace, .lower { display: grid; gap: 1rem; }
.workspace { grid-template-columns: minmax(250px, 320px) 1fr; align-items: start; }
.lower { grid-template-columns: 1fr 1fr; margin-top: 1rem; }
.panel { background: var(--card); border: 1px solid var(--line); border-radius: 16px; padding: 1.05rem 1.1rem 1.2rem; }
h2 { margin: 0 0 0.35rem; font-size: 1.05rem; }
.hint { margin: 0 0 0.8rem; color: var(--muted); }
.atlas { margin-top: 1rem; }
.fact { margin: 0.2rem 0; }
.fact b { font-weight: 650; }
.tree { margin: 0.15rem 0 0.15rem 0.2rem; padding-left: 0.85rem; border-left: 2px solid var(--line); }
.choice, .path { display: block; width: 100%; text-align: left; margin: 0.45rem 0; padding: 0.8rem 0.85rem; border: 1px solid var(--line); border-radius: 12px; background: #fff; cursor: pointer; }
.choice strong { display: block; font-size: 1.02rem; }
.choice span { color: var(--muted); font-size: 0.85rem; }
.choice.selected { border-color: var(--accent); background: #fff1f2; }
.metrics { display: flex; flex-wrap: wrap; gap: 0.45rem; margin: 0.7rem 0; }
.metric { min-width: 8.5rem; padding: 0.55rem 0.7rem; border-radius: 12px; background: var(--chip); border: 1px solid var(--chip-line); }
.metric b { display: block; font-size: 1.2rem; }
.metric em { display: block; font-style: normal; font-size: 0.78rem; color: var(--muted); }
.block { margin: 0.85rem 0; }
.verdict { margin: 0 0 0.8rem; padding: 0.85rem 1rem; border-radius: 12px; background: var(--good); border: 1px solid var(--good-line); font-weight: 650; }
.notice { margin: 0.7rem 0; padding: 0.75rem 0.9rem; border-radius: 12px; background: var(--bad); border: 1px solid var(--bad-line); }
table { width: 100%; border-collapse: collapse; margin: 0.4rem 0 0.8rem; }
th, td { text-align: left; padding: 0.45rem 0.35rem; border-bottom: 1px solid var(--line); vertical-align: top; }
th { font-size: 0.78rem; color: var(--muted); font-weight: 600; }
button.primary { margin-top: 0.7rem; padding: 0.55rem 0.95rem; border: 0; border-radius: 10px; background: var(--ink); color: var(--card); cursor: pointer; }
pre { white-space: pre-wrap; margin: 0.4rem 0 0; padding: 0.85rem; border-radius: 12px; background: #1c1917; color: #f5f5f4; font: 13px/1.45 ui-monospace, monospace; }
@media (max-width: 800px) {
  .top, .workspace, .lower { display: block; }
  .campaign { margin-top: 1rem; }
  .panel { margin-top: 1rem; }
}
</style>
</head>
<body>
<div class="shell">
<header class="top">
  <div>
    <p class="eyebrow">ACVC</p>
    <h1>{{title}}</h1>
    <p class="lede">{{lede}}</p>
  </div>
  <label class="campaign"><span>{{campaign_label}}</span><select id="campaign"></select></label>
</header>
<div class="workspace">
  <section class="panel">
    <h2>{{frontier_heading}}</h2>
    <p class="hint">{{frontier_hint_page}}</p>
    <div id="frontier"></div>
  </section>
  <section class="panel">
    <h2>{{hypothesis_heading}}</h2>
    <div id="why"><p class="hint">{{why_hint}}</p></div>
  </section>
</div>
<section class="panel atlas">
  <h2>{{lineage_heading}}</h2>
  <p class="hint">{{lineage_hint}}</p>
  <div id="context"></div>
  <div id="map"></div>
</section>
<div class="lower">
  <section class="panel">
    <h2>{{search_heading}}</h2>
    <p class="hint">{{search_hint}}</p>
    <label class="field"><span>{{premise_text}}</span><input id="query" type="search" placeholder="{{premise_placeholder}}"></label>
    <button id="search" class="primary" type="button">{{search_button}}</button>
    <div id="hits"></div>
  </section>
  <section class="panel">
    <h2>{{which_stays}}</h2>
    <p class="hint">{{which_stays_hint}}</p>
    <label class="field"><span>{{one_hypothesis}}</span><select id="left"></select></label>
    <label class="field"><span>{{other_hypothesis}}</span><select id="right"></select></label>
    <button id="compare" class="primary" type="button">{{compare_button}}</button>
    <div id="diff"></div>
  </section>
</div>
</div>
<script>
const L = __LANG_JSON__;
function fill(key, vars) {
  let text = L[key] || key;
  for (const name in vars) text = text.split("{" + name + "}").join(vars[name]);
  return text;
}
const campaign = document.querySelector("#campaign");
const known = new Map();
let schemas = [];
let selected = "";
const origins = {
  seed: L.web_seed,
  mutate: L.web_mutate,
  revert: L.web_revert,
  revise_premise: L.web_revise,
  synthesize: L.web_synthesize
};
async function api(path) {
  const response = await fetch(path);
  const body = await response.json();
  if (!response.ok) throw new Error(body.error || L.read_failed);
  return body;
}
function text(tag, className, value) {
  const node = document.createElement(tag);
  if (className) node.className = className;
  node.textContent = value;
  return node;
}
function schemaOf() {
  const found = schemas.find((item) => item.campaign === campaign.value);
  return found ? found.metrics : [];
}
function meaning(name) {
  const spec = schemaOf().find((item) => item.name === name);
  if (!spec) return L.campaign_measure;
  return spec.direction === "lower" ? L.lower_sentence : L.higher_sentence;
}
function chips(values) {
  const wrap = document.createElement("div");
  wrap.className = "metrics";
  const entries = Object.entries(values || {});
  if (!entries.length) {
    wrap.append(text("p", "hint", L.unmeasured_yet));
    return wrap;
  }
  for (const [name, value] of entries) {
    const chip = document.createElement("div");
    chip.className = "metric";
    chip.append(text("span", "", name));
    chip.append(text("b", "", String(value)));
    chip.append(text("em", "", meaning(name)));
    wrap.append(chip);
  }
  return wrap;
}
function remember(variant, claim) {
  if (!variant || !claim) return;
  known.set(variant, claim);
  for (const select of [document.querySelector("#left"), document.querySelector("#right")]) {
    const current = select.value;
    select.replaceChildren();
    for (const [id, name] of known) {
      const option = document.createElement("option");
      option.value = id;
      option.textContent = name;
      select.append(option);
    }
    if (known.has(current)) select.value = current;
  }
}
function markSelected() {
  for (const button of document.querySelectorAll(".choice")) {
    button.classList.toggle("selected", button.dataset.id === selected);
  }
}
async function show(variant) {
  selected = variant;
  markSelected();
  const why = await api("/api/why?variant=" + encodeURIComponent(variant));
  remember(why.variant, why.claim);
  const box = document.querySelector("#why");
  box.replaceChildren();
  box.append(text("h3", "", why.claim));
  const origin = document.createElement("div");
  origin.className = "block";
  origin.append(text("span", "kicker", L.origin_kicker));
  origin.append(text("p", "", origins[why.edge] || why.edge));
  box.append(origin);
  if (why.assumptions && why.assumptions.length) {
    const block = document.createElement("div");
    block.className = "block";
    block.append(text("span", "kicker", L.assumption_kicker));
    for (const item of why.assumptions) block.append(text("p", "", item));
    box.append(block);
  }
  if (why.intent) {
    const block = document.createElement("div");
    block.className = "block";
    block.append(text("span", "kicker", L.intent_kicker));
    block.append(text("p", "", why.intent));
    box.append(block);
  }
  const measured = document.createElement("div");
  measured.className = "block";
  measured.append(text("span", "kicker", L.metrics_kicker));
  const trials = why.trials || [];
  if (!trials.length) measured.append(text("p", "hint", L.nobody_measured));
  trials.forEach((trial, index) => {
    const label = index === 0
      ? (why.on_frontier === false ? L.left_frontier_measure : L.frontier_measure)
      : L.earlier_measure;
    measured.append(text("p", "hint", label));
    measured.append(chips(trial.metrics));
  });
  box.append(measured);
  for (const winner of why.beaten_by || []) {
    const gaps = (winner.gaps || []).map((gap) => fill("against", {
      name: gap.name,
      loser: String(gap.loser),
      winner: String(gap.winner),
      sense: gap.direction === "lower" ? L.lower_better : L.higher_better
    })).join("; ");
    box.append(text("p", "notice", fill("beaten_by", { claim: winner.claim }) + " " + gaps));
  }
  if (why.rejection) box.append(text("p", "notice", fill("off_frontier", { reason: why.rejection })));
  else if (why.on_frontier === false) box.append(text("p", "notice", L.motive_unrecorded));
  if (why.agent) box.append(text("p", "hint", fill("registered_by", { agent: why.agent })));
  await linked(box, L.came_label, why.parents || []);
  await linked(box, L.what_came_after, why.children || []);
  document.querySelector("#left").value = why.variant;
  markSelected();
}
async function linked(box, label, ids) {
  if (!ids.length) return;
  const block = document.createElement("div");
  block.className = "block";
  block.append(text("span", "kicker", label));
  for (const id of ids) {
    const item = await api("/api/why?variant=" + encodeURIComponent(id));
    remember(item.variant, item.claim);
    const button = document.createElement("button");
    button.type = "button";
    button.className = "choice";
    button.dataset.id = item.variant;
    button.append(text("strong", "", item.claim));
    button.addEventListener("click", () => show(item.variant));
    block.append(button);
  }
  box.append(block);
}
function choice(claim, detail, variant) {
  const button = document.createElement("button");
  button.type = "button";
  button.className = "choice";
  button.dataset.id = variant;
  button.append(text("strong", "", claim));
  button.append(text("span", "", detail));
  button.addEventListener("click", () => show(variant));
  return button;
}
function summary(values) {
  const entries = Object.entries(values || {});
  if (!entries.length) return L.no_measure_title;
  return entries.map(([name, value]) => name + " " + value).join(" · ");
}
function fact(label, value) {
  const row = document.createElement("p");
  row.className = "fact";
  row.append(text("b", "", label), document.createTextNode(" " + (value || "")));
  return row;
}
function mapNode(node) {
  remember(node.variant, node.claim);
  const box = document.createElement("div");
  box.className = "tree";
  const button = document.createElement("button");
  button.type = "button";
  button.className = "choice";
  const meta = document.createElement("span");
  const branches = { seed: L.branch_seed, mutate: L.branch_mutate, revert: L.branch_revert, revise_premise: L.branch_revise, synthesize: L.branch_synthesize };
  const statusLabel = { "on_frontier": L.status_frontier, "superseded": L.status_beaten, "rejected": L.status_rejected, "unmeasured": L.no_measurement };
  const cause = (node.assumptions || []).join(" · ");
  const beaten = (node.beaten_by || []).map((item) => fill("beaten_inline", { claim: item.claim })).join("; ");
  meta.textContent = (statusLabel[node.status] || node.status) + " · " + (node.agent || L.agent_word) + " · " + (branches[node.edge] || node.edge) + " · " + summary(node.metrics) + (cause ? " · " + cause : "") + (beaten ? " · " + beaten : "");
  button.append(text("strong", "", node.claim), meta);
  button.addEventListener("click", () => {
    show(node.variant).catch((err) => document.querySelector("#why").replaceChildren(text("p", "notice", err.message)));
  });
  box.append(button);
  for (const child of node.children || []) box.append(mapNode(child));
  return box;
}
async function showMap() {
  const data = await api("/api/context?campaign=" + encodeURIComponent(campaign.value));
  const interactions = data.interactions || {};
  document.querySelector("#context").replaceChildren(
    fact(L.problem_label, data.problem),
    ...(data.need ? [fact(L.need_fact, data.need)] : []),
    fact(L.owner_fact, (data.agents || []).join(", ") || L.nobody),
    fact(L.campaign_label, fill("web_campaign_fact", { age: data.open_for || "", variants: String(interactions.variants || 0), trials: String(interactions.trials || 0), tokens: data.tokens_label || "" }))
  );
  const map = document.querySelector("#map");
  map.replaceChildren();
  const roots = data.graph || [];
  if (!roots.length) {
    map.append(text("p", "hint", L.no_origin_yet));
    return;
  }
  for (const node of roots) map.append(mapNode(node));
}
async function refresh() {
  await showMap();
  const data = await api("/api/frontier?campaign=" + encodeURIComponent(campaign.value));
  const body = document.querySelector("#frontier");
  body.replaceChildren();
  const rows = data.variants || [];
  if (!rows.length) {
    body.append(text("p", "hint", L.none_survived));
    document.querySelector("#why").replaceChildren(text("p", "hint", L.search_or_score));
    return;
  }
  for (const row of rows) {
    remember(row.variant, row.claim);
    body.append(choice(row.claim, summary(row.metrics), row.variant));
  }
  await show(rows[0].variant);
}
document.querySelector("#search").addEventListener("click", async () => {
  const data = await api("/api/recall?campaign=" + encodeURIComponent(campaign.value) + "&query=" + encodeURIComponent(document.querySelector("#query").value));
  const body = document.querySelector("#hits");
  body.replaceChildren();
  const hits = data.hits || [];
  if (!hits.length) {
    body.append(text("p", "hint", L.no_recall));
    return;
  }
  for (const hit of hits) {
    remember(hit.variant, hit.claim);
    const parent = hit.parents && hit.parents[0] ? hit.parents[0].claim : "";
    const statusLabel = { "on_frontier": L.status_frontier, "superseded": L.status_beaten, "rejected": L.status_rejected, "unmeasured": L.no_measurement };
    const origin = parent ? fill("came_from_inline", { claim: parent }) : L.opened_short;
    body.append(choice(hit.claim, (statusLabel[hit.status] || hit.status || L.no_measurement) + " · " + origin, hit.variant));
  }
  markSelected();
});
function verdict(kind, leftName, rightName) {
  if (kind === "left") return fill("stays_detail", { claim: leftName });
  if (kind === "right") return fill("stays_detail", { claim: rightName });
  if (kind === "tie") return L.tie_metrics;
  if (kind === "incomparable") return L.each_wins;
  return L.missing_measure_which;
}
function edge(direction, left, right, leftName, rightName) {
  if (left == null || right == null || Number(left) === Number(right)) return L.metric_tie;
  const leftWins = direction === "lower" ? Number(left) < Number(right) : Number(left) > Number(right);
  return fill("better_here", { name: leftWins ? leftName : rightName });
}
function files(title, paths, left, right) {
  if (!paths || !paths.length) return null;
  const block = document.createElement("div");
  block.className = "block";
  block.append(text("span", "kicker", title));
  for (const path of paths) {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "path";
    button.textContent = path;
    button.addEventListener("click", async () => {
      const data = await api("/api/file-diff?left=" + encodeURIComponent(left) + "&right=" + encodeURIComponent(right) + "&path=" + encodeURIComponent(path));
      const view = document.querySelector("#file");
      const leftName = known.get(left) || L.first_one;
      const rightName = known.get(right) || L.second_one;
      let note = L.text_diff;
      let body = data.diff || "";
      if (body.startsWith("added\n")) {
        note = fill("only_in", { name: rightName });
        body = body.slice(6);
      } else if (body.startsWith("removed\n")) {
        note = fill("only_in", { name: leftName });
        body = body.slice(8);
      }
      view.replaceChildren(text("span", "kicker", path), text("p", "", note));
      const code = document.createElement("pre");
      code.textContent = body || L.no_text_diff;
      view.append(code);
    });
    block.append(button);
  }
  return block;
}
document.querySelector("#compare").addEventListener("click", async () => {
  const left = document.querySelector("#left");
  const right = document.querySelector("#right");
  const data = await api("/api/compare?left=" + encodeURIComponent(left.value) + "&right=" + encodeURIComponent(right.value));
  const leftName = known.get(left.value) || L.the_first;
  const rightName = known.get(right.value) || L.the_second;
  const box = document.querySelector("#diff");
  box.replaceChildren(text("p", "verdict", verdict(data.dominance, leftName, rightName)));
  const table = document.createElement("table");
  const head = document.createElement("tr");
  for (const label of [L.metric_col, leftName, rightName, L.reading_col]) {
    head.append(text("th", "", label));
  }
  table.append(head);
  for (const metric of data.metrics || []) {
    const spec = schemaOf().find((item) => item.name === metric.name);
    const line = document.createElement("tr");
    line.append(text("td", "", metric.name));
    line.append(text("td", "", metric.left == null ? L.no_measurement : String(metric.left)));
    line.append(text("td", "", metric.right == null ? L.no_measurement : String(metric.right)));
    line.append(text("td", "", edge(spec ? spec.direction : "higher", metric.left, metric.right, leftName, rightName)));
    table.append(line);
  }
  box.append(table);
  for (const block of [
    files(fill("only_side", { name: leftName }), data.removed, left.value, right.value),
    files(fill("only_side", { name: rightName }), data.added, left.value, right.value),
    files(L.different_text, data.modified, left.value, right.value)
  ]) {
    if (block) box.append(block);
  }
  const view = document.createElement("div");
  view.id = "file";
  box.append(view);
});
campaign.addEventListener("change", () => {
  refresh().catch((err) => document.querySelector("#why").replaceChildren(text("p", "notice", err.message)));
});
async function boot() {
  const data = await api("/api/campaigns");
  schemas = data.schemas || [];
  for (const name of data.campaigns) {
    const option = document.createElement("option");
    option.value = name;
    option.textContent = name;
    campaign.append(option);
  }
  await refresh();
}
boot().catch((err) => document.querySelector("#why").replaceChildren(text("p", "notice", err.message)));
</script>
</body>
</html>
"##;

pub fn page() -> String {
    let catalog = acvc_text::catalog_json();
    let mut html = PAGE.replace("__HTML_LANG__", acvc_text::active().tag());
    if let Ok(serde_json::Value::Object(map)) = serde_json::from_str::<serde_json::Value>(&catalog)
    {
        for (key, value) in map {
            if let Some(text) = value.as_str() {
                html = html.replace(&format!("{{{{{key}}}}}"), text);
            }
        }
    }
    html.replace("__LANG_JSON__", &catalog)
}
