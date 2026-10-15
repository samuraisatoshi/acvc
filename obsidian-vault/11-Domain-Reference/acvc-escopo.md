---
tags:
    - acvc
    - escopo
title: Escopo do ACVC
---
# Escopo do ACVC

## Problema

O Git guarda o que um humano aceitou gravar. Um agente de código testa hipóteses em volume alto, volta atrás, revisa a premissa e esquece a sessão. O que se perde é a árvore do que foi tentado.

## Valor

O objeto principal é a variante: uma hipótese com premissa, árvore de código e medições. O grafo lembra o beco sem saída, o motivo da rejeição e o que ainda vale explorar.

## Objetos

- Campanha: nome e esquema de métricas. Cada métrica é maior-melhor ou menor-melhor.
- Variante: a hipótese. A identidade é o hash da codificação canônica da árvore, dos pais, da aresta, da premissa, da intenção e do agente. A data e as medições não entram na identidade. A mesma hipótese é o mesmo nó.
- Premissa: afirmação, suposições e, quando a aresta revisa, qual premissa ela substitui.
- Medição: registro append-only. Pontuar de novo cria outra medição, não outro nó.
- Arestas: seed sem pais; mutate e revert com um pai; revise_premise com um pai e a premissa revisada; synthesize com dois ou mais pais. Revert não apaga o beco sem saída.
- Fronteira: quem não é dominado no último ensaio completo. Rejeitados ficam de fora da fronteira e dentro do grafo. Empate permanece. Domina quem é melhor ou igual em toda métrica e estritamente melhor em uma.
- Pasta de trabalho: o diretório que o agente já está editando. O marcador guarda o repositório, a campanha e a variante corrente, para o próximo registro achar o pai sem o agente lembrar o hash.

## Fronteiras
- Git não é o núcleo. Promover só deposita a variante vencedora num diretório ou num checkout Git que já existe.
- Merge de texto implícito e reescrita de história continuam fora do produto.
- Hardlink é proibido: escrever na pasta de trabalho não pode corromper o armazenamento.
- A TUI é o visor nativo. A página web mostra o mesmo grafo. VS Code e Obsidian falam com o CLI em JSON, sem abrir o armazenamento por FFI. O servidor MCP reexecuta o mesmo binário.