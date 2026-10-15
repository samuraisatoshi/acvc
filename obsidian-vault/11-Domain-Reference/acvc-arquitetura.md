---
tags:
    - acvc
    - arquitetura
title: Arquitetura do ACVC
---
# Arquitetura do ACVC

## Forma

Hexagonal. O domínio não faz IO. A aplicação só depende de portas. O armazenamento é o adaptador dirigido. O CLI é a raiz de composição. A TUI e os plugins de editor são adaptadores de entrada.

## Módulos
- crates/domain — variante, premissa, medição, campanha, arestas, hash canônico, Pareto, diff de árvore, formato do índice e a seleção do que falta numa oferta. Sem IO. A única dependência externa é a função de hash.
- crates/app — seed, record incremental, score, reject, compare, frontier, recall, why, backtrack, materialize, promote, status, pack e a aceitação de uma variante remota. Não importa o armazenamento. Aceitar não faz checkout e não reescreve uma variante já gravada.
- crates/store — log append-only, cache somente-leitura e SQLite. Implementa as portas.
- crates/cli — verbos, JSON na saída padrão, erro JSON na saída de erro. serve, push e fetch falam o quadro ACVCNET1. mcp expõe os mesmos verbos e reexecuta o binário. web serve a página e também reexecuta o binário.
- crates/tui — visor nativo. Chama os mesmos casos de uso. Não raspa a saída do CLI.
- editors — cliente compartilhado, VS Code e Obsidian. Cada um executa o binário.
## Árvore grande

A identidade de conteúdo é BLAKE3 com tag de blob, árvore, variante e medição. Fork é o hash da raiz, 32 bytes. Editar um arquivo reescreve esse blob e as árvores do caminho até a raiz. Pasta irmã limpa não é relida. Comparar pula a subárvore cujo hash coincide. Status compara a impressão digital e não rehasheia arquivo limpo. Materializar clona o blob a partir do cache somente-leitura. Sem clone no sistema de arquivos, a queda é cópia. Hardlink é proibido.

## Direção

cli depende de tui, app e store. tui depende de app. app depende de domain. store depende de domain. app não depende de store.
