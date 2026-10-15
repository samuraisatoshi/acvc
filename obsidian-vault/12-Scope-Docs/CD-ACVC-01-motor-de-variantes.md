---
card_id: CD-ACVC-01
epic: EP-ACVC-DOM
status: implemented
title: Motor de variantes
---
# Motor de variantes

## Problema
O agente precisa de um objeto que seja a hipótese, não o commit. Sem isso, medir de novo fabrica outro nó e um arquivo sujo reescreve a árvore inteira.

## Valor
A mesma hipótese permanece o mesmo nó. Uma medição nova não muda a identidade. O diff não desce numa subárvore cujo hash já coincide.

## Fronteiras
Este corte não faz IO, não fala com Git e não comprime o log. Seed não tem pai. Synthesize tem dois ou mais. Revert não apaga o beco.

## Status
status: implemented