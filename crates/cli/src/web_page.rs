pub const PAGE: &str = r##"<!DOCTYPE html>
<html lang="pt">
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
    <h1>O que ainda vale explorar</h1>
    <p class="lede">Uma campanha compara hipóteses pela premissa e pelas medições. A fronteira mostra as que nenhuma outra superou.</p>
  </div>
  <label class="campaign"><span>Campanha</span><select id="campaign"></select></label>
</header>
<div class="workspace">
  <section class="panel">
    <h2>Na fronteira</h2>
    <p class="hint">Premissa e métricas do que a medição ainda não descartou.</p>
    <div id="frontier"></div>
  </section>
  <section class="panel">
    <h2>A hipótese</h2>
    <div id="why"><p class="hint">Escolha uma hipótese para ver a origem, as suposições e as medições.</p></div>
  </section>
</div>
<div class="lower">
  <section class="panel">
    <h2>Buscar</h2>
    <p class="hint">Encontra hipóteses pela frase da premissa, inclusive as que saíram da fronteira.</p>
    <label class="field"><span>Texto da premissa</span><input id="query" type="search" placeholder="o que a hipótese afirma"></label>
    <button id="search" class="primary" type="button">Buscar</button>
    <div id="hits"></div>
  </section>
  <section class="panel">
    <h2>Qual fica</h2>
    <p class="hint">Compara duas hipóteses: quem mede melhor e quais arquivos mudaram.</p>
    <label class="field"><span>Uma hipótese</span><select id="left"></select></label>
    <label class="field"><span>Outra hipótese</span><select id="right"></select></label>
    <button id="compare" class="primary" type="button">Comparar</button>
    <div id="diff"></div>
  </section>
</div>
</div>
<script>
const campaign = document.querySelector("#campaign");
const known = new Map();
let schemas = [];
let selected = "";
const origins = {
  seed: "Esta hipótese abriu a campanha. Não existe uma hipótese anterior.",
  mutate: "Esta hipótese muda o código a partir de outra, e mantém a pergunta.",
  revert: "Esta hipótese volta para um caminho anterior. O beco sem saída continua registrado.",
  revise_premise: "A pergunta mudou. A premissa anterior foi substituída.",
  synthesize: "Esta hipótese junta mais de um caminho anterior."
};
async function api(path) {
  const response = await fetch(path);
  const body = await response.json();
  if (!response.ok) throw new Error(body.error || "Não foi possível ler a campanha.");
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
  if (!spec) return "Medição desta campanha.";
  return spec.direction === "lower" ? "Quanto menor, melhor." : "Quanto maior, melhor.";
}
function chips(values) {
  const wrap = document.createElement("div");
  wrap.className = "metrics";
  const entries = Object.entries(values || {});
  if (!entries.length) {
    wrap.append(text("p", "hint", "Ainda sem medição."));
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
  origin.append(text("span", "kicker", "Origem"));
  origin.append(text("p", "", origins[why.edge] || why.edge));
  box.append(origin);
  if (why.assumptions && why.assumptions.length) {
    const block = document.createElement("div");
    block.className = "block";
    block.append(text("span", "kicker", "O que esta hipótese assume"));
    for (const item of why.assumptions) block.append(text("p", "", item));
    box.append(block);
  }
  if (why.intent) {
    const block = document.createElement("div");
    block.className = "block";
    block.append(text("span", "kicker", "O que se queria obter"));
    block.append(text("p", "", why.intent));
    box.append(block);
  }
  const measured = document.createElement("div");
  measured.className = "block";
  measured.append(text("span", "kicker", "Métricas"));
  const trials = why.trials || [];
  if (!trials.length) measured.append(text("p", "hint", "Ninguém mediu esta hipótese ainda."));
  trials.forEach((trial, index) => {
    measured.append(text("p", "hint", trials.length > 1 ? "Medição " + (index + 1) : "Última medição usada na fronteira."));
    measured.append(chips(trial.metrics));
  });
  box.append(measured);
  if (why.rejection) box.append(text("p", "notice", "Fora da fronteira: " + why.rejection));
  if (why.agent) box.append(text("p", "hint", "Registrada por " + why.agent + "."));
  await linked(box, "Veio de", why.parents || []);
  await linked(box, "O que veio depois", why.children || []);
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
  if (!entries.length) return "Sem medição";
  return entries.map(([name, value]) => name + " " + value).join(" · ");
}
async function refresh() {
  const data = await api("/api/frontier?campaign=" + encodeURIComponent(campaign.value));
  const body = document.querySelector("#frontier");
  body.replaceChildren();
  const rows = data.variants || [];
  if (!rows.length) {
    body.append(text("p", "hint", "Nenhuma hipótese sobreviveu às medições."));
    document.querySelector("#why").replaceChildren(text("p", "hint", "Busque uma premissa ou registre uma medição."));
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
    body.append(text("p", "hint", "Nenhuma premissa contém esse texto."));
    return;
  }
  for (const hit of hits) {
    remember(hit.variant, hit.claim);
    body.append(choice(hit.claim, "Abrir a origem e as medições", hit.variant));
  }
  markSelected();
});
function verdict(kind, leftName, rightName) {
  if (kind === "left") return leftName + " fica. É melhor ou igual em todas as métricas e melhor em uma.";
  if (kind === "right") return rightName + " fica. É melhor ou igual em todas as métricas e melhor em uma.";
  if (kind === "tie") return "Empate. As duas medem igual.";
  if (kind === "incomparable") return "Cada uma ganha numa métrica diferente. A campanha ainda não tem um vencedor único.";
  return "Falta medição para decidir qual fica.";
}
function edge(direction, left, right, leftName, rightName) {
  if (left == null || right == null || Number(left) === Number(right)) return "Empate nesta métrica.";
  const leftWins = direction === "lower" ? Number(left) < Number(right) : Number(left) > Number(right);
  return (leftWins ? leftName : rightName) + " está melhor aqui.";
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
      const leftName = known.get(left) || "a primeira";
      const rightName = known.get(right) || "a segunda";
      let note = "Diferença de texto.";
      let body = data.diff || "";
      if (body.startsWith("added\n")) {
        note = "Este arquivo só existe em " + rightName + ".";
        body = body.slice(6);
      } else if (body.startsWith("removed\n")) {
        note = "Este arquivo só existe em " + leftName + ".";
        body = body.slice(8);
      }
      view.replaceChildren(text("span", "kicker", path), text("p", "", note));
      const code = document.createElement("pre");
      code.textContent = body || "O arquivo não tem diferença de texto.";
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
  const leftName = known.get(left.value) || "A primeira";
  const rightName = known.get(right.value) || "A segunda";
  const box = document.querySelector("#diff");
  box.replaceChildren(text("p", "verdict", verdict(data.dominance, leftName, rightName)));
  const table = document.createElement("table");
  const head = document.createElement("tr");
  for (const label of ["Métrica", leftName, rightName, "Leitura"]) {
    head.append(text("th", "", label));
  }
  table.append(head);
  for (const metric of data.metrics || []) {
    const spec = schemaOf().find((item) => item.name === metric.name);
    const line = document.createElement("tr");
    line.append(text("td", "", metric.name));
    line.append(text("td", "", metric.left == null ? "sem medição" : String(metric.left)));
    line.append(text("td", "", metric.right == null ? "sem medição" : String(metric.right)));
    line.append(text("td", "", edge(spec ? spec.direction : "higher", metric.left, metric.right, leftName, rightName)));
    table.append(line);
  }
  box.append(table);
  for (const block of [
    files("Só em " + leftName, data.removed, left.value, right.value),
    files("Só em " + rightName, data.added, left.value, right.value),
    files("Texto diferente", data.modified, left.value, right.value)
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
