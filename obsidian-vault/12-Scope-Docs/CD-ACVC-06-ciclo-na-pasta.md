---
card_id: CD-ACVC-06
epic: EP-ACVC
status: implemented
title: Ciclo na pasta de trabalho
---
# Ciclo na pasta de trabalho

## Problema
O agente trabalha no diretório que já está aberto e esquece o hash do pai. Pedir uma segunda cópia a cada passo quebra o ciclo.

## Valor
Semear a própria pasta grava o marcador. Os verbos seguintes acham a campanha e a variante sozinhos quando há um único contexto.

## Fronteiras
Materialize continua sendo só a segunda cópia. Init é uma vez. Campanha repetida é erro nomeado, não erro cru do banco.

## Status
status: implemented