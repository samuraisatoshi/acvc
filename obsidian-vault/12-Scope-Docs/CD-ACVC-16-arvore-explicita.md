---
card: CD-ACVC-16
title: Árvore explícita do seed
---
## Problema

Semear a raiz de um repositório ainda coloca relatório e imagem na identidade da hipótese. A varredura pula uma lista fixa de nomes e não tem um ignore igual em todo clone.

## Valor

A árvore semeada é a mesma em qualquer máquina, e a pessoa vê o que entrou antes de isso virar a hipótese.

## Fronteiras

Não consulta o gitignore. Não muda o lugar dos marcadores nem do armazenamento. A lista fixa que já pula pastas de build continua. Não cria uma exclusão mágica para uma pasta de relatório. Não altera hipóteses já gravadas.
