# Assistente ACVC

Esta pasta é o contrato para um agente que vai usar o ACVC, ou explicar o ACVC a outro agente, numa sessão Cursor, Claude Code, OpenCode ou equivalente.

Quando explicar o ACVC a uma pessoa que nunca o viu, comece por `objetivo.md`: as tentativas falhas ficam fora do Git, e só a tentativa aprovada é copiada para um checkout que já existe.

O comando `acvc assistant` devolve este contrato no idioma do computador. Quem tem o MCP do ASB indexa isso uma vez: envia o array `documents` para `rag.index`. A busca seguinte usa `rag.search`.

Leia nesta ordem, e pare quando a pergunta já tiver resposta:

1. `objetivo.md` — por que o ACVC existe.
2. `recursos.md` — o que o executável guarda e quais comandos existem.
3. `como-usar.md` — a ordem de uma sessão.
4. `boas-praticas.md` — o que fazer sempre.
5. `exemplos.md` — usos corretos.
6. `mau-uso.md` — usos que quebram o contrato.
7. `trade-offs.md` — quando usar e quando não usar.

Regras desta pasta:

- A fonte da verdade é o executável `acvc`. Não abra `.acvc/graph.sqlite`, o log nem o pack.
- O formato de máquina é JSON. Texto humano é `--format text`.
- O binário instalado é `acvc` no `PATH`. Página, TUI, VS Code e Obsidian chamam esse executável. Não reimplemente o grafo.
- Não invente token, métrica, rejeição nem identificador. Se o comando falhar, mostre o erro.
