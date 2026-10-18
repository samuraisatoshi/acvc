# Exemplos

## Abrir uma campanha

```text
acvc init --repo .
acvc campaign create busca --metric tests_passed:higher --metric latency_ms:lower
acvc seed --repo . --campaign busca --path ./hipotese --premise "o cache cabe em disco" --assumption "o disco é local" --intent "menos leituras" --agent julio
```

O JSON do seed traz a variante, a árvore, quantos arquivos entraram e quais pesaram mais.

## A hipótese seguinte

```text
acvc record --repo . --workspace ./hipotese --parent PREFIXO --premise "o índice é uma árvore" --assumption "as chaves estão ordenadas" --intent "busca mais curta" --agent julio --edge mutate
acvc score --repo . --variant PREFIXO --metric tests_passed=16 --metric latency_ms=4
```

## Ver o que ainda compete

```text
acvc context --repo . --campaign busca
acvc frontier --repo . --campaign busca
acvc why --repo . PREFIXO
acvc recall --repo . --campaign busca --query "árvore"
acvc compare --repo . PREFIXO_A PREFIXO_B
```

Um `why` de hipótese superada, sem `reject`, nomeia quem ganhou, a métrica e diz que o motivo não foi gravado.

## Duas medições da mesma hipótese

```text
acvc score --repo . --variant PREFIXO --metric tests_passed=10 --metric latency_ms=10
acvc score --repo . --variant PREFIXO --metric tests_passed=1 --metric latency_ms=1
acvc score --repo . --variant PREFIXO --metric tests_passed=0 --metric latency_ms=20
```

As duas primeiras são incomparáveis e ficam na fronteira. A terceira é pior nas duas métricas, sai da fronteira e continua no `why`.

## Encerrar de propósito

```text
acvc reject --repo . --variant PREFIXO --reason "a invalidação do cache quebra com escrita concorrente"
acvc spend --repo . --campaign busca --tokens 15000 --note "sessão da hipótese" --agent julio
```

## Tirar lixo da árvore

Na pasta semeada, `.acvcignore`:

```text
# relatório
lab/reports
```

Dois clones com o mesmo arquivo deixam os mesmos caminhos de fora. Apagar o arquivo deixa `lab/reports` voltar na varredura seguinte. Hipóteses já gravadas não são reescritas.
