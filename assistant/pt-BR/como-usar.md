# Como usar

Os passos abaixo usam os nomes dos comandos. Uma tentativa é uma hipótese. Uma medição é um ensaio. As tentativas que ninguém superou em todos os números são a fronteira.

1. `acvc context --campaign NOME` antes de propor, medir ou descartar.
2. Se a campanha não existe: `acvc init --repo .` e `acvc campaign create NOME --metric metrica:higher`.
3. A primeira hipótese é `seed` da pasta que contém os arquivos da hipótese, com `--premise`, `--assumption`, `--intent` e `--agent`.
4. Mudou arquivo: `record` a partir do pai. Não faça outro `seed`.
5. Há número real da métrica declarada: `score`. Um ensaio incompleto não entra na fronteira.
6. `frontier` mostra o que ainda compete. `why` explica uma hipótese. `recall --query` acha pelo texto da premissa.
7. Duas medidas concretas: `compare`.
8. A hipótese morreu por um motivo conhecido: `reject --reason`. Sem motivo, não rejeite. A leitura já diz que o motivo não foi gravado.
9. A sessão gastou tokens e esse número é conhecido: `spend`. Não estime.
10. A variante que ficou vencedora entra no Git com `promote` num checkout que já existe.

Prefixo de identificador: 8 a 64 caracteres hexadecimais, sem ambiguidade.

Para outro agente na mesma máquina, o canal é o executável ou `acvc mcp`. A página e o TUI são para a pessoa. Não os substitua por uma leitura direta do banco.
