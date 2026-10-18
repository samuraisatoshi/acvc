import assert from "node:assert/strict";
import { test } from "node:test";
import { compareText, pathLabel, recallText, whyText } from "../src/format.ts";
import type { Compare, Recall, Why } from "../src/types.ts";

const why: Why = {
  variant: "a".repeat(64),
  campaign: "evolucao",
  edge: "seed",
  claim: "frase humana",
  assumptions: ["os visores ainda dizem left"],
  intent: "a pessoa reconhece a hipótese",
  agent: "researcher",
  parents: [],
  children: [],
  trials: [{ id: "b".repeat(64), metrics: { uso_real: 3 } }],
  rejection: null,
  on_frontier: false,
  beaten_by: [
    {
      claim: "sessão gravada",
      gaps: [{ name: "uso_real", direction: "higher", winner: 5, loser: 3 }],
    },
  ],
  parent_claims: [],
  child_claims: ["leitura com direção"],
};

test("the premise replaces the hash and the side", () => {
  const text = whyText(why, "pt-BR");
  assert.match(text, /frase humana/);
  assert.match(text, /Superada por sessão gravada\. uso_real 3 contra 5 \(quanto maior, melhor\)/);
  assert.match(text, /Motivo não gravado\./);
  assert.match(whyText({ ...why, rejection: "a frase não servia" }, "pt-BR"), /Rejeitada: a frase não servia/);
  assert.doesNotMatch(whyText({ ...why, rejection: "a frase não servia" }, "pt-BR"), /Motivo não gravado/);
  assert.doesNotMatch(whyText({ ...why, on_frontier: true, beaten_by: [] }, "pt-BR"), /Motivo não gravado/);
  assert.match(text, /Esta hipótese abriu a campanha\./);
  assert.match(text, /O que veio depois: leitura com direção/);
  assert.doesNotMatch(text, /edge /);
  assert.doesNotMatch(text, /a{12}/);

  const compared = compareText({
    left: "a".repeat(64),
    right: "b".repeat(64),
    added: [],
    removed: [],
    modified: [],
    metrics: [{ name: "custo", direction: "lower", left: 5, right: 2 }],
    dominance: "right",
    left_claim: "frase humana",
    right_claim: "sessão gravada",
  } satisfies Compare, "pt-BR");
  assert.match(compared, /sessão gravada fica\./);
  assert.match(compared, /quanto menor, melhor/);
  assert.doesNotMatch(compared, /dominance/);

  const hits = recallText({
    hits: [{ variant: "a".repeat(64), claim: "frase humana", status: "superseded", parents: [] }],
  } satisfies Recall, "pt-BR");
  assert.match(hits, /superada/);
  assert.doesNotMatch(hits, /a{12}/);
  assert.equal(pathLabel("modified", "nota.txt", "pt-BR"), "mudou  nota.txt");
  assert.match(whyText(why, "en-US"), /Motive was not recorded\./);
  assert.doesNotMatch(whyText(why, "en-US"), /Motivo não gravado/);
});
