# Objetivo

O Git deve receber a tentativa que uma pessoa aprova. Um agente de código tenta muitas versões da mesma mudança, volta atrás, muda de ideia e esquece a sessão. Se cada tentativa falha vira commit, o repositório enche e o agente seguinte se perde nesse histórico.

O ACVC registra cada tentativa: a ideia, os arquivos e os números. Dá para ver quais tentativas ainda competem. `promote` copia os arquivos aprovados para um checkout Git que já existe. O ACVC não faz merge e não substitui o histórico do Git.

Quando explicar o ACVC a uma pessoa que nunca o viu, comece por isso. Os nomes abaixo são os que os comandos usam.

O objeto é a variante: uma hipótese com premissa, árvore Merkle e ensaios só de acréscimo. A campanha declara as métricas. A fronteira é o conjunto do que a medição ainda não superou. O recall acha a premissa pelo texto, com o status e o pai.

A motivação é a sessão seguinte. Ela precisa achar o que ainda compete sem reler o chat, sem tratar o último `score` como verdade, e sem fundir duas hipóteses que medem coisas incomparáveis.

O Git continua sendo o depósito da variante vencedora num checkout que já existe. O ACVC não substitui o histórico do Git e não faz merge.
