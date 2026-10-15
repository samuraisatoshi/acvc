---
tags:
    - acvc
    - uso
title: Exemplos de uso do ACVC
---
# Exemplos de uso do ACVC

O agente opera no diretório que já está aberto. A segunda cópia só existe quando alguém pede.

## Uma vez

```
acvc init
acvc campaign create engine --metric tests_passed:higher
```

Repetir init responde que o repositório já existe. Repetir a campanha responde que ela já existe.

## Ciclo na pasta

```
acvc seed --path . --premise "projeto acvc" --agent cursor
acvc status
acvc record --premise "cache mais curto"
acvc score --metric tests_passed=10
acvc why
acvc frontier
acvc recall "cache"
```

Se existe uma única campanha, seed, frontier e recall dispensam o nome. score, reject e why, sem variante, usam o marcador da pasta corrente. status não rehasheia arquivo limpo e devolve a premissa, a aresta, as métricas e se o nó está na fronteira.

## Segunda cópia e promoção

```
acvc materialize --variant <id> --into /tmp/acvc-copia
acvc promote --variant <id> --dest /tmp/acvc-destino
acvc promote --variant <id> --git
```

materialize exige diretório vazio. promote --git só age num checkout Git que já existe: sobrepõe os arquivos da variante, não apaga o resto e não altera a configuração do Git.

## Visores

```
acvc tui
acvc file-diff <esquerda> <direita> caminho/do/arquivo
```

A TUI abre na fronteira quando a campanha já tem nó pontuado. As abas são campanhas, fronteira, linhagem, premissa, comparar e busca.

## Pack
Empacota os blobs frios. Árvores ficam no log. Status e record não leem o pack.

```
acvc pack
```

Dois arquivos com o mesmo conteúdo continuam um único blob. A resposta traz objects, raw_bytes e packed_bytes.

## Remoto
Duas máquinas trocam a variante por hash. O destino declara o que já tem. Só o objeto que falta atravessa. A identidade da variante não muda. O grafo já gravado não é reescrito e não há checkout implícito.

No repositório que recebe:

```
acvc serve --bind 127.0.0.1:9419
```

Para empurrar, a partir da origem:

```
acvc push --to 127.0.0.1:9419 --variant <id> --campaign engine
```

Para puxar, o id é o hash completo de 64 hex:

```
acvc fetch --from 127.0.0.1:9419 --variant <64 hex> --campaign engine
```

A campanha nomeada já precisa existir em quem recebe. A resposta traz objects_sent, objects_skipped, payload_bytes e graph, com valor inserted ou unchanged.

## MCP
O servidor do produto fala MCP no stdio e reexecuta o mesmo binário. Não abre o log nem o SQLite por conta própria.

```
acvc mcp
```

Cada verbo do CLI é uma ferramenta: init, campaign_create, campaign_list, seed, materialize, status, record, score, reject, compare, file_diff, frontier, recall, why, backtrack, pack, serve, push, fetch, promote e tui. O argumento repo vira --repo. Se o CLI sai com erro, o texto da ferramenta é o JSON de erro, com error e context, e isError fica verdadeiro.

## Página
A página lê o mesmo CLI. Não abre o log nem o SQLite.

```
acvc web --bind 127.0.0.1:9420
```

A campanha escolhida mostra a fronteira com premissa e métricas. A linha abre a linhagem. A busca e a comparação usam os mesmos verbos recall e compare.
