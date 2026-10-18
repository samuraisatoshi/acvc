# Trade-offs

## Use o ACVC quando

- A sessão testa mais de uma hipótese e a seguinte precisa saber qual ainda compete.
- A premissa muda, e o motivo do abandono importa tanto quanto o número.
- O último ensaio não é, por si, o melhor.
- Várias métricas puxam para lados opostos e as duas hipóteses devem continuar visíveis.
- Outro agente, horas depois, vai recall em vez de reler o chat.

## Não use quando

- O trabalho é um único caminho linear e o Git já conta a história que importa.
- Não há métrica declarada nem premissa. Sem isso o ACVC vira um segundo Git sem fronteira.
- A pergunta é só “o que está no working tree agora”. Isso é `git status` ou `acvc status`, não uma campanha nova.
- O custo de escrever premissa, causa e medição é maior do que o custo de esquecer a tentativa.

## O que se ganha

Memória de hipótese, fronteira de Pareto, recall da premissa, motivo separado do placar, e um depósito Git só da variante que venceu.

## O que se paga

Um store `.acvc` ao lado do Git. Disciplina de não abrir esse store à mão. Métrica declarada antes do ensaio. A varredura tem lista própria, então um arquivo grande entra no hash se não estiver no `.acvcignore` nem na lista fixa. O remoto copia objeto, não faz o merge que o Git faria.

## Agentes em Cursor, Claude Code e OpenCode

O recurso que esses agentes devem chamar é `acvc` no `PATH`, ou o servidor `acvc mcp` se a sessão já o registrou. Os dois falam JSON. Nenhum deles abre o SQLite. Esta pasta diz o contrato. O código do executável continua sendo a implementação.
