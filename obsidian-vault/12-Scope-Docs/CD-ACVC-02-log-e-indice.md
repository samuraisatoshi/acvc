---
card_id: CD-ACVC-02
epic: EP-ACVC-STORE
status: implemented
title: Log e índice
---
# Log e índice

## Problema
Milhares de arquivos não podem ser copiados a cada hipótese, e a consulta do grafo não pode varrer o disco.

## Valor
Cada blob fica uma vez no log. A pasta de trabalho nasce por clone do cache. Status decide o que está sujo pela impressão digital.

## Fronteiras
Hardlink é proibido. Symlink é erro. Packfile fica fora. O marcador e o índice não entram no hash da árvore.

## Status
status: implemented