---
card_id: CD-ACVC-08
epic: EP-ACVC-PACK
status: implemented
title: Packfile
---
# Packfile

## Problema
O hash já compartilha arquivo idêntico, mas o log append-only cresce sem compressão das variantes antigas.

## Valor
Histórico antigo ocupa menos disco sem mudar a identidade da variante nem o clone da pasta de trabalho.

## Fronteiras
Não entra no caminho de status nem de record. Não reescreve história. Não autoriza hardlink.

## Status
status: implemented