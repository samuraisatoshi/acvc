pub const PAGE: &str = r##"<!DOCTYPE html>
<html lang="pt">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">
<title>ACVC</title>
<style>
body { font: 16px/1.45 ui-sans-serif, system-ui, sans-serif; margin: 2rem auto; max-width: 960px; color: #1c1917; padding: 0 1rem; }
table { border-collapse: collapse; width: 100%; }
th, td { text-align: left; padding: 0.45rem 0.6rem; border-bottom: 1px solid #d6d3d1; vertical-align: top; }
tr { cursor: pointer; }
button, select, input { font: inherit; }
section { margin-top: 1.6rem; }
pre { white-space: pre-wrap; }
</style>
</head>
<body>
<h1>ACVC</h1>
<label>Campanha <select id="campaign"></select></label>
<section>
<h2>Fronteira</h2>
<table>
<thead><tr><th>Premissa</th><th>Métricas</th><th>Variante</th></tr></thead>
<tbody id="frontier"></tbody>
</table>
</section>
<section>
<h2>Linhagem</h2>
<pre id="why">Escolha uma linha da fronteira.</pre>
</section>
<section>
<h2>Busca</h2>
<input id="query" type="search">
<button id="search" type="button">Buscar</button>
<pre id="hits"></pre>
</section>
<section>
<h2>Comparar</h2>
<input id="left" placeholder="esquerda">
<input id="right" placeholder="direita">
<button id="compare" type="button">Comparar</button>
<pre id="diff"></pre>
</section>
<script>
const campaign = document.querySelector("#campaign");
async function api(path) {
  const response = await fetch(path);
  const body = await response.json();
  if (!response.ok) {
    const message = body.error || JSON.stringify(body);
    throw new Error(message);
  }
  return body;
}
function metrics(values) {
  return Object.entries(values).map(([name, value]) => name + "=" + value).join(" ");
}
function cell(text) {
  const item = document.createElement("td");
  item.textContent = text;
  return item;
}
async function refresh() {
  const data = await api("/api/frontier?campaign=" + encodeURIComponent(campaign.value));
  const body = document.querySelector("#frontier");
  body.replaceChildren();
  for (const row of data.variants) {
    const line = document.createElement("tr");
    line.append(cell(row.claim), cell(metrics(row.metrics)), cell(row.variant));
    line.addEventListener("click", async () => {
      const why = await api("/api/why?variant=" + encodeURIComponent(row.variant));
      const parents = (why.parents || []).join(" ") || "nenhum";
      document.querySelector("#why").textContent = why.edge + "\n" + why.claim + "\npais " + parents;
      document.querySelector("#left").value = row.variant;
    });
    body.append(line);
  }
}
document.querySelector("#search").addEventListener("click", async () => {
  const data = await api("/api/recall?campaign=" + encodeURIComponent(campaign.value) + "&query=" + encodeURIComponent(document.querySelector("#query").value));
  document.querySelector("#hits").textContent = data.hits.map((hit) => hit.variant + " " + hit.claim).join("\n");
});
document.querySelector("#compare").addEventListener("click", async () => {
  const left = document.querySelector("#left").value;
  const right = document.querySelector("#right").value;
  const data = await api("/api/compare?left=" + encodeURIComponent(left) + "&right=" + encodeURIComponent(right));
  document.querySelector("#diff").textContent = data.dominance + "\n" + (data.modified || []).join("\n");
});
campaign.addEventListener("change", () => { refresh(); });
async function boot() {
  const data = await api("/api/campaigns");
  for (const name of data.campaigns) {
    const option = document.createElement("option");
    option.value = name;
    option.textContent = name;
    campaign.append(option);
  }
  await refresh();
}
boot().catch((err) => { document.querySelector("#why").textContent = err.message; });
</script>
</body>
</html>
"##;
