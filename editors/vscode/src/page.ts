export function pageHtml(): string {
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
