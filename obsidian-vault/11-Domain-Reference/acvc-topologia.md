---
tags:
    - acvc
    - topologia
title: Topologia do ACVC
---
# Topologia do ACVC

## Processo

Um executável local. Cargo só compila. Quem roda o produto não precisa de Rust. O binário de uso é target/release/acvc.

## Dentro de .acvc/

- Log append-only de blobs e árvores, com índice de hash para offset.
- SQLite em WAL para campanhas, variantes, medições, rejeições e busca textual.
- Cache somente-leitura dos blobs já extraídos, para o clone da pasta de trabalho.

## Fora do hash da árvore

- Marcador .acvc-workspace, na pasta que o agente edita.
- Índice .acvc-index, impressão digital por caminho.

A varredura ignora as pastas .git, .acvc, target e node_modules, e os arquivos .acvc-workspace e .acvc-index. Symlink é erro.

## Editores

editors/client, editors/vscode e editors/obsidian são processos à parte. O contrato estável é o JSON do CLI, incluindo a diferença de um arquivo. Não há FFI para o armazenamento.

## Workspace ASB

O workspace acvc é standalone, linguagem Rust, diretório deste repositório. O vault é obsidian-vault/, relativo a esse diretório. Não há remoto do produto nesta topologia.
