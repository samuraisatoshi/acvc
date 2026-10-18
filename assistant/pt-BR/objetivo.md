# Objetivo

O Git guarda o que um humano escolheu registrar. Um agente de código testa hipóteses em volume, volta atrás, reescreve a premissa e esquece a sessão. O commit seguinte não diz qual medição ainda compete, nem por que a outra foi abandonada.

O ACVC existe para essa memória.

O objeto é a variante: uma hipótese com premissa, árvore Merkle e ensaios só de acréscimo. A campanha declara as métricas. A fronteira é o conjunto do que a medição ainda não superou. O recall acha a premissa pelo texto, com o status e o pai.

A motivação é a sessão seguinte. Ela precisa achar o que ainda compete sem reler o chat, sem tratar o último `score` como verdade, e sem fundir duas hipóteses que medem coisas incomparáveis.

O Git continua sendo o depósito da variante vencedora num checkout que já existe. O ACVC não substitui o histórico do Git e não faz merge.
