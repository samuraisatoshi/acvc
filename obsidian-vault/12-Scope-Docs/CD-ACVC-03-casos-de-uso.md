---
card_id: CD-ACVC-03
epic: EP-ACVC-APP
status: implemented
title: Casos de uso
---
# Casos de uso

## Problema
Os verbos do agente precisam de um único lugar, sem o CLI conhecer o arquivo do armazenamento.

## Valor
O agente semeia, mede, compara, lembra e promove pela mesma aplicação.

## Fronteiras
A aplicação não abre o log nem o SQLite. Promote não apaga arquivo extra do destino. Materialize exige diretório vazio.

## Status
status: implemented