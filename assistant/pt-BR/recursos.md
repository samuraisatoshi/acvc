# Recursos

Uma pessoa que nunca viu o ACVC precisa de uma frase antes destes nomes. O ACVC guarda as tentativas que não devem virar commit. Cada tentativa é a ideia, os arquivos e os números. As tentativas que ninguém superou em todos os números ainda competem. `promote --dest` materializa os arquivos aprovados numa pasta; `--git` também cria um commit numa worktree Git existente. Os nomes abaixo são os que os comandos e o JSON usam.

## O que uma hipótese é

O identificador é o hash BLAKE3 dos bytes canônicos: árvore, pais, aresta, premissa, causas, revisão, intenção e agente. O nome da campanha, a data, as medições e o gasto de tokens não entram no hash. A mesma hipótese é o mesmo nó.

Arestas:

- `seed` — a única origem.
- `mutate` — os arquivos mudaram e a crença permanece. É a aresta padrão do `record`.
- `revert` — o passo volta a uma hipótese anterior.
- `revise_premise` — a própria crença mudou.
- `synthesize` — dois ou mais pais.

Status, nos valores JSON:

- `on_frontier` — medida, e nenhuma outra hipótese a supera.
- `superseded` — medida, e outra da fronteira é melhor.
- `rejected` — alguém gravou o motivo.
- `unmeasured` — ainda não há ensaio completo para o esquema da campanha.

Sair da fronteira não grava rejeição. Se não há motivo, a leitura diz quem ganhou, a métrica e que o motivo não foi gravado. Em pt-BR essa frase é `Motivo não gravado.`

O texto que a pessoa vê segue o idioma do computador (`LC_ALL`, depois `LC_MESSAGES`, depois `LANG`). Os quatro valores JSON acima não mudam. Agente lê o campo `status`, não a frase traduzida.

## Onde os arquivos ficam

O store é `.acvc/` no diretório de `--repo`: log append-only, pack e SQLite. Na pasta semeada ficam `.acvc-workspace` e `.acvc-index`.

A varredura não lê `.gitignore`. Pula `.git`, `.acvc`, `target`, `node_modules`, `.build` e `DerivedData`. Um `.acvcignore` na raiz da pasta semeada lista prefixos relativos; esse arquivo não entra no hash. Symlink na varredura é erro. Hardlink é proibido.

## Comandos

`--repo` aponta o store. `--format` é `json` (padrão) ou `text`.

Leitura, antes de agir:

- `context` — mapa da campanha: problema, causas, agentes, interações, tokens, idade e ramificações.
- `guide` — comandos mais o mapa do repositório aberto.
- `frontier` — o que ainda compete.
- `why` — uma hipótese: causa, medição, quem a superou, pais e filhos.
- `recall` — busca pela premissa. Devolve status e pai. Acento entra na busca.
- `compare` — duas hipóteses medidas: quem fica e quais arquivos mudaram.
- `file-diff` — o texto de um caminho entre duas hipóteses.
- `status` — a pasta de trabalho contra a hipótese materializada.
- `backtrack` — volta para um pai.

Escrita:

- `init` — cria o store.
- `campaign create NOME --metric nome:higher|lower` — o esquema. Pode haver mais de uma métrica.
- `seed` — a primeira hipótese da campanha. A segunda origem é recusada. O caminho seguinte é `record`.
- `record` — a pasta mudou. Sem `--parent`, o pai é a hipótese do workspace, e o workspace passa a apontar para a hipótese nova. Aresta padrão: `mutate`.
- `score --metric nome=número` — acrescenta um ensaio na mesma variante. O identificador não muda. Não apaga os anteriores.
- `reject --reason "..."` — ato explícito. O motivo fica no grafo.
- `spend --tokens N` — livro à parte. Não entra no hash, não é métrica e não trava o `record`. Zero tokens é recusado.
- `materialize` — copia a hipótese para uma pasta.
- `promote` — `--dest` materializa a variante vencedora numa pasta; `--git` exige uma worktree Git e cria um commit. Nenhum modo faz merge nem remove do destino caminhos ausentes da variante.
- `pack` — compacta blobs frios. Não muda a leitura de status nem de record.

Visores, todos chamam o executável:

- `web` — página em `127.0.0.1:9420`.
- `tui` — terminal.
- `mcp` — servidor stdio. Não abre o SQLite. Cada ferramenta reexecuta o binário.
- `obsidian` — instala o visor no vault. O comando no Obsidian é Open ACVC.

Remoto: `serve`, `push`, `fetch`. O remoto não faz checkout nem merge. Não reenvia objeto que o outro lado já tem.

## Fronteira

Entram todas as medições completas que ninguém supera. Duas medições incomparáveis da mesma hipótese ficam as duas. Uma medição estritamente pior sai da fronteira e permanece no histórico. Não se mistura métrica de um ensaio com métrica de outro. Hipótese rejeitada fica fora. Duas hipóteses incomparáveis continuam as duas. Não se funde a campanha para forçar um vencedor.
