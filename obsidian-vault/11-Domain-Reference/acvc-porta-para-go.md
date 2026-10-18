---
tags:
    - acvc
    - asb
    - mcp
title: Porta do ACVC para Go no MCP do ASB
---
## Decisão

O ACVC entra no MCP do ASB como biblioteca Go dentro do asb-core, não como um segundo processo stdio. O servidor TypeScript só encaminha. A regra de negócio, o hash e o store ficam em Go, no mesmo desenho hexagonal que o Rust tem hoje.

O binário Rust `acvc mcp` deixa de ser o servidor que o Cursor e o Claude Code registram. Os clientes passam a ver ferramentas `acvc.*` num perfil do ASB, ao lado de `asb-code` e `asb-kanban`.

## O que o ASB já é

O núcleo é Go, em `cmd/asb-core`. Cada ferramenta MCP é um `POST /tools/<nome.com.pontos>` nesse processo.

Os processos `asb-code`, `asb-kanban`, `asb-data` e os demais são TypeScript. Cada um escolhe um perfil em `asb-mcp/src/profiles.ts`, registra ferramentas com `registerTool` e chama `CoreClient.callTool`. Esse cliente faz `POST ${ASB_CORE_URL}/tools/${name}`. Não há regra de negócio nessa camada. O exemplo já existente é `formal-spec.analyze`: o arquivo `asb-mcp/src/tools/formalspec.ts` só descreve o argumento e encaminha; o trabalho está em `internal/infra/httpapi`.

O workspace da sessão já é resolvido pelo core a partir da pasta em que o MCP foi lançado. O ACVC usa essa pasta como repositório. Não se cria outro banco no Postgres do ASB para guardar hipóteses.

## O que o Rust faz hoje

O domínio não abre arquivo. A identidade da hipótese é o hash BLAKE3 dos bytes canônicos: árvore, pais, aresta, premissa, intenção e agente. O nome da campanha, a data e as medições ficam de fora. A mesma hipótese é o mesmo nó.

O store é `.acvc/` no repositório: log append-only, pack dos blobs frios, SQLite com FTS. Os marcadores `.acvc-workspace` e `.acvc-index` ficam na pasta semeada. A varredura não lê `.gitignore`. Ela pula `.git`, `.acvc`, `target`, `node_modules`, `.build` e `DerivedData`.

O `acvc mcp` atual fala JSON por linha na stdin e, para cada ferramenta, executa de novo o mesmo binário. Ele não abre o log nem o SQLite por conta própria. Os visores também não abrem: página, TUI, VS Code e Obsidian chamam o executável.

## Para onde cada peça vai

- `crates/domain` vira `internal/domain/acvc`. Só biblioteca padrão e BLAKE3. Sem `os`, sem SQL, sem HTTP.
- `crates/app` vira `internal/app/acvc`. Casos de uso: seed, record, score, reject, frontier, why, recall, compare, context, spend, pack, remoto.
- `crates/store` vira `internal/infra/acvc`. Log, pack, SQLite, varredura, marcadores.
- `crates/cli` vira `cmd/acvc`, um adaptador fino em cima dos mesmos pacotes. Os visores continuam chamando esse executável.
- `crates/cli` no modo `mcp` não é reescrito como servidor. O lugar dele passa a ser o perfil MCP.
- `crates/tui` e a página podem continuar chamando o executável. Não entram no processo do MCP.

O módulo Go pode viver dentro do repositório do asb-core. O repositório Rust permanece a referência dos vetores de identidade até a porta reproduzir os mesmos hashes.

## Contrato que a porta reproduz

Antes de apagar o Rust, os testes de Go têm de abrir um `.acvc` já gravado pelo Rust e devolver o mesmo identificador.

- A fronteira usa o melhor ensaio completo. Um ensaio posterior e pior permanece no histórico e não derruba o melhor.
- `recall` devolve o status já gravado e o pai, com a premissa.
- Uma campanha que já tem origem recusa um segundo seed. O caminho é `record` a partir do pai.
- Rejeição é uma linha explícita. Sair da fronteira não rejeita sozinho.
- `spend` é um livro à parte. Não entra no hash, não é métrica e não trava o `record`.
- Duas hipóteses incomparáveis continuam as duas na fronteira. A campanha `evolucao` não é fundida.
- O remoto não faz checkout nem merge. O promote só deposita a variante vencedora num checkout que já existe.
- Hardlink continua proibido. Symlink na varredura continua erro.

## Como entra no MCP

1. Handlers Go registram `POST /tools/acvc.context`, `acvc.frontier`, `acvc.why`, `acvc.recall`, `acvc.compare`, `acvc.seed`, `acvc.record`, `acvc.score`, `acvc.reject`, `acvc.spend`, `acvc.guide` e o restante dos verbos locais. O nome com pontos segue `formal-spec.analyze`.
2. O handler abre o `.acvc` do workspace já resolvido pela sessão. Não recebe um caminho de banco do Postgres.
3. Um arquivo novo `asb-mcp/src/tools/acvc.ts` só declara o esquema e encaminha com `client.callTool`. Zero regra.
4. `profiles.ts` ganha o perfil `acvc`, com entrada `asb-mcp/src/index-acvc.ts` no mesmo formato de `index-code.ts`.
5. O processo entra na lista de MCP do usuário como `asb-acvc`, comando `node .../index-acvc.js`, com `ASB_CORE_URL` e `ASB_API_TOKEN` iguais aos outros servidores. O registro separado `acvc` → `acvc mcp` sai.
6. O erro da ferramenta é o JSON `{error, context}` que o CLI já devolve, com `isError` verdadeiro. O MCP não reescreve a mensagem.

`acvc.web` e `acvc.tui` não entram no perfil. São processos de visor. `acvc.obsidian` também fica no executável: ele instala o plugin no vault e não é uma leitura do grafo.

## Ordem

1. Pacote de domínio e os testes de hash contra fixtures gravadas pelo Rust.
2. Store que abre o log e o SQLite atuais sem migrar o arquivo.
3. Casos de uso da leitura: frontier, why, recall, compare, context.
4. Casos de uso da escrita: seed, record, score, reject, spend.
5. Rotas `/tools/acvc.*` e o perfil `asb-acvc`.
6. O executável Go substitui `~/.local/bin/acvc` só quando os visores e o vault passam nos mesmos textos.

A página, o TUI e os editores não mudam de contrato: continuam lendo o JSON do executável. Quando o executável for Go, eles acompanham sem um segundo adaptador.
