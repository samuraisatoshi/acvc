---
card: CD-ACVC-17
title: Porta para o núcleo do ASB
---
## Problema

As ferramentas do ACVC ainda são um processo separado. No ASB, o servidor só encaminha a chamada para o núcleo em Go.

## Valor

O ACVC passa a responder dentro do núcleo, no perfil da sessão, com o mesmo contrato.

## Fronteiras

O armazenamento continua no repositório da sessão, não no banco do ASB. O perfil só encaminha; a regra não mora na camada de encaminhamento. Página, terminal e visores não entram nesse perfil. O executável atual só é substituído quando um arquivo já gravado por ele abre com o mesmo identificador. A fase de fixtures já feita no workspace do ASB não é refeita. Esta porta espera as quatro correções anteriores, para não copiar o buraco.
