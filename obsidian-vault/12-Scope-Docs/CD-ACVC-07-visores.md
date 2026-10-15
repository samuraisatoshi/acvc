---
card_id: CD-ACVC-07
epic: EP-ACVC-VIEW
status: implemented
title: Visores de editor
---
# Visores de editor

## Problema
Quem vive no editor não deve abrir outro modelo do grafo, nem ligar o armazenamento por FFI.

## Valor
Os dois editores mostram o mesmo contrato do CLI, inclusive a diferença de um arquivo entre duas variantes.

## Fronteiras
Sem FFI. Sem UI web. O binário é configuração; o repositório vazio significa a pasta do editor.

## Status
status: implemented