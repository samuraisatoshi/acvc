# ACVC: hipóteses controladas por hash, e a decisão no Git

Samuraisatoshi  
2026  
Correspondência: samuraisatoshi@cryptoworld.8shield.net

Esta é uma descrição do sistema de software neste repositório. Não é um artigo com revisão por pares. Não relata estudo com usuários nem benchmark. A implementação é a fonte das afirmações sobre comportamento.

## Se você ainda não viu o ACVC

Um agente tenta muitas versões da mesma mudança. Colocar cada tentativa falha no Git enche o repositório, e o agente seguinte se perde nesse histórico. O ACVC registra cada tentativa: a ideia, os arquivos e a medição. Você vê quais tentativas ainda competem. O Git recebe uma árvore, a que você aprova, pelo `acvc promote`, num checkout que já existe. O ACVC não faz merge e não substitui o Git.

O resto desta nota usa os nomes precisos. Uma hipótese é uma tentativa. Um ensaio é uma medição. A fronteira são as tentativas que ninguém superou em todos os números. Duas tentativas podem continuar as duas.

## Resumo

O ACVC (Agent Code Variant Control) é um armazenamento local de hipóteses sobre um corpo de código. Uma pessoa ou um agente pode experimentar muitas variantes. Cada variante tem premissa, pressupostos, intenção, relação com os pais e uma árvore de arquivos. A árvore é uma árvore de Merkle: um nó guarda o hash do filho, não os bytes do filho. Um arquivo que não mudou não é gravado de novo. O Git continua sendo o lugar em que a pessoa deposita a variante que decidiu conservar. A campanha nomeia as métricas. Um ensaio é uma medição completa. A fronteira guarda todo ensaio completo que nenhuma outra hipótese supera em todas as métricas. Uma perda na fronteira não é uma rejeição. A rejeição é um ato separado, e carrega um motivo.

O desenho toma o armazenamento de objetos endereçados por conteúdo do Git [1, 2] e aponta para outro propósito. O hash do Git controla um snapshot que alguém decidiu registrar. O hash do ACVC controla uma hipótese que pode nunca ser essa decisão.

## 1. O que é este trabalho

Uma campanha é uma pergunta nomeada com um conjunto fixo de métricas. Cada métrica aponta para menor ou para maior. A primeira árvore de uma campanha é um seed. As árvores seguintes são records. A aresta diz que tipo de passo foi: `mutate` altera a árvore na mesma família de premissa, `revert` volta a uma ideia anterior, `revise_premise` substitui um pressuposto que falhou, e `synthesize` tem dois ou mais pais. A identidade de uma variante é um hash BLAKE3 [3] sobre uma codificação canônica do hash da árvore, dos pais, da aresta, da afirmação, dos pressupostos, da intenção e do agente. O nome da campanha, o relógio, as medições e qualquer gasto de tokens ficam fora dessa identidade. Pontuar uma variante não cria outra variante.

O armazenamento é um diretório `.acvc` ao lado do trabalho: um log append-only de blobs e árvores, um pack de blobs que já assentaram, e um índice SQLite com busca textual nas premissas. Quem lê usa o executável `acvc`. Não abre o banco.

Quando uma variante é a que deve entrar na história, `promote` copia essa árvore para um checkout Git que já existe. As outras variantes ficam em `.acvc`.

## 2. Motivação e os problemas que inspiraram o trabalho

Três problemas se encontram.

O primeiro é para que serve o Git. O Git é um sistema de arquivos endereçável por conteúdo, com um grafo de commits por cima [1, 2]. Um commit é uma decisão registrada: esta árvore, estes pais, esta mensagem, esta pessoa. Perez De Rosso e Jackson mostraram que vários conceitos do Git não atingem os propósitos que os usuários de fato levam até ele [4]. A programação exploratória acrescenta um propósito para o qual o Git não foi construído. Cientistas de dados mantêm análises antigas vivas copiando arquivos, comentando código e segurando versões sem uso no mesmo buffer, porque um commit é pesado e definitivo demais para um pensamento que ainda se move [5]. Notebooks acumulam essa desordem; a história da exploração e a história que se publicaria são objetos diferentes [6, 7]. Um agente que experimenta dezenas de hipóteses numa sessão torna o desajuste mais nítido. A sessão acaba. Os commits feitos são as poucas árvores que alguém concordou em conservar. As premissas abandonadas somem, ou ficam enterradas num branch que ninguém pontua.

O segundo problema é a medição. Dois métodos de stitching, duas regras de integração ou dois armazenamentos atrás de uma API de restaurante não se ordenam por um único número. Um pode ser mais preciso e mais lento. A comparação multiobjetivo guarda o conjunto não dominado: os máximos de um conjunto de vetores [8]. Rastreadores de experimento como MLflow e Sacred registram execuções, parâmetros e métricas de experimentos computacionais [9, 10]. Eles tomam a execução como objeto. Aqui o objeto é a hipótese, e a execução é um ensaio preso a ela. Um ensaio que omite uma métrica está incompleto e fica fora da fronteira. Métricas não são misturadas entre ensaios para inventar uma melhor pontuação.

O terceiro problema é o motivo. Dominação não é decisão. Uma hipótese pode sair da fronteira porque outro ensaio completo a supera em tudo, enquanto ninguém disse por que a perdedora foi abandonada. O ACVC mantém essa lacuna visível. O status diz quem venceu e que o motivo não foi gravado. Uma rejeição é uma frase humana, gravada uma vez para aquela variante. Agentes esquecem sessões. Um armazenamento que só guarda vencedores esquece a razão de uma derrota.

O container-use, do projeto Dagger, responde a um problema vizinho: um agente precisa de um container isolado e de um branch Git para que o trabalho em paralelo não colida com o checkout [11]. Isso é um ambiente de execução. Ele não nomeia uma premissa, não pontua uma campanha e não guarda uma fronteira não dominada. O ACVC não sobe um container. As duas ferramentas respondem a perguntas diferentes. A seção 6 lista os outros vizinhos.

## 3. Métodos

### 3.1 Controle por hash, a partir do Git

O Git não guarda uma cópia nova do projeto a cada commit. Guarda objetos endereçados pelo hash do conteúdo. Um blob são os bytes de um arquivo. Uma árvore é uma lista de nomes, modos e hashes de blobs ou de outras árvores. Um commit guarda o hash de uma árvore, não os arquivos. Se um arquivo não mudou, a árvore nova repete o hash antigo do blob. Prefixos iguais de um diretório são compartilhados. Comparação e armazenamento seguem o hash. Os bytes do arquivo são lidos quando o hash falta, não porque uma versão nova foi declarada [1, 2]. A construção de Merkle é a forma mais antiga da mesma ideia: um pai autentica os filhos pelo hash dos hashes, de modo que uma subárvore intacta é um único valor [12].

O ACVC usa essa estrutura de controle para hipóteses.

Um arquivo entra no armazenamento como blob, etiquetado e hasheado com BLAKE3 [3]. Um diretório é um objeto árvore: nomes em ordem, cada um com modo, tipo e hash. O objeto árvore não contém o arquivo. Contém o hash que já nomeia o arquivo. A variante não contém a árvore. Contém o hash da árvore, mais a premissa e os hashes dos pais. A identidade é o hash desse registro canônico.

Gravar uma mudança percorre o workspace e compara cada caminho com o índice da árvore pai. Um caminho limpo contribui o hash que já tem. Os bytes dele não são lidos e o blob não é escrito de novo. Só um caminho sujo ou novo é hasheado num blob novo. Uma subárvore igual não é carregada quando duas variantes são comparadas. O teste do desenho é o mesmo teste que o armazenamento de objetos do Git satisfaz: um irmão que não mudou não faz parte da escrita.

Isso é o contrário de copiar o projeto para uma pasta por ideia, e o contrário de colocar o arquivo inteiro dentro do registro da hipótese. O hash é o controle. Os bytes existem uma vez, sob esse hash.

Duas consequências seguem. Primeiro, uma campanha pode guardar muitas variantes de uma árvore grande sem copiar os arquivos intactos. Segundo, a premissa pode mudar com o hash da árvore estável, ou a árvore pode mudar com o texto da premissa estável. São hashes diferentes, porque a árvore e a afirmação entram no hash da variante. Uma medição não entra. A mesma variante pode carregar vários ensaios. O ensaio tem o próprio hash (o hash da variante, um nonce e o mapa de métricas). Pontuar não bifurca o código.

O hash histórico do Git foi SHA-1, depois acompanhado de SHA-256 no projeto Git. O ACVC usa BLAKE3 em todo objeto que cria. A ideia emprestada é o modelo de objetos, não a escolha da função.

### 3.2 A hipótese

Um seed não tem pais. É a origem da campanha, e há uma origem. Uma mutação tem a variante atual do workspace como pai, a menos que quem chama nomeie outro. Revisar uma premissa registra qual pressuposto foi substituído; se quem chama não o nomeia, o pai é a afirmação revisada. Uma síntese nomeia pelo menos dois pais. A aresta faz parte do hash, então uma mutação e uma revisão de premissa da mesma árvore são variantes diferentes.

A afirmação, os pressupostos e a intenção são strings na identidade. Não são comentários ao lado de um commit. O recall busca essas strings. O índice textual é o SQLite FTS5 [13].

### 3.3 Campanha, ensaios e fronteira

A campanha é um esquema: nomes de métrica e uma direção, menor ou maior. Um ensaio está completo quando carrega todos os nomes. A fronteira é o conjunto de ensaios completos que não são estritamente dominados. A dominação usa os máximos clássicos de um conjunto de vetores [8]: um ensaio domina outro quando não é pior em nenhuma métrica e é melhor em pelo menos uma. Dois ensaios que trocam precisão por custo permanecem. Dentro de uma variante, um ensaio completo estritamente pior é descartado. Na campanha, todo ensaio completo não dominado que restar é informado. A fronteira não escolhe um único vencedor e não preenche uma métrica faltante com outro ensaio.

O spend, um livro-caixa lateral de tokens, é append-only e não é métrica. Não entra no hash da variante e não move a fronteira.

### 3.4 Rejeição

A dominação não escreve rejeição. `reject` guarda um motivo para uma variante. Até esse motivo existir, uma variante que saiu da fronteira é descrita como superada, com o vencedor e a métrica, e o texto diz que o motivo não foi gravado. Os valores de status no JSON são códigos fixos (`on_frontier`, `superseded`, `rejected`, `unmeasured`). A prosa segue o idioma do computador.

A revisão explícita de premissa se inspira no hábito, na filosofia da ciência, de tratar um pressuposto que falhou como mudança de hipótese, e não como edição silenciosa [14]. O software não testa falseabilidade. Só recusa deixar essa mudança escondida numa mutação comum: a aresta é outra, e o hash segue.

### 3.5 O que não entra no hash

O relógio, o nome da campanha, o arranjo do pack e o caminho do workspace não fazem parte da identidade da variante. Mover uma campanha de um diretório para outro não renomeia a variante. Duas campanhas só podem guardar o mesmo hash de variante se guardarem a mesma hipótese; a campanha é um rótulo no índice, não um campo no hash.

### 3.6 A costura com o Git

`promote` materializa a árvore escolhida num checkout Git que quem chama já tem. O ACVC não faz commit, não reescreve história e não substitui o remoto. O Git continua significando o que significava: o registro do que uma pessoa escolheu conservar [1]. A busca fica no armazenamento por hash até essa escolha.

## 4. Literatura que sustenta o desenho

O modelo de objetos é o do Git, como documentado por Chacon e Straub [1] e como construído por Torvalds em 2005 [2]: blobs, árvores e hashes como nomes dos objetos. Merkle dá a razão de uma árvore de hashes detectar uma folha alterada sem guardar a folha no pai [12]. BLAKE3 é o hash concreto [3].

A crítica de usar o commit como unidade de exploração é empírica e conceitual. O Variolite estudou cientistas de dados versionando trechos por cópia e comentário porque o controle de versão convencional briga com uma alternativa de vida curta [5]. Rule, Tabard e Hollan separaram exploração e explicação em notebooks [6]. Head, Hohman, Barik, Drucker e DeLine mostraram como a história do notebook vira uma desordem que o autor administra à mão [7]. Perez De Rosso e Jackson argumentaram que a dificuldade do Git é em parte um desajuste entre conceitos e propósitos, e redesenharam os conceitos no Gitless [4]. O ACVC não redesenha o Git. Recusa colocar a busca dentro dos conceitos do Git.

Modelos de proveniência, em particular o W3C PROV [15], descrevem entidades, atividades e agentes em geral. O ACVC é uma instância estreita: uma árvore de código, uma hipótese, um ensaio e uma string de agente. Não se apresenta como formato de intercâmbio de proveniência.

O manual de Pareto é a fonte econômica da fronteira não dominada [16]. O algoritmo de que o ACVC depende é o dos máximos de um conjunto de vetores [8]. A síntese de Deb sobre otimização multiobjetivo é o enquadramento moderno de manual [17]. Os sistemas de rastreamento de experimento mostraram que execuções devem ser consultáveis [9, 10]. O ACVC prende a execução consultável a uma hipótese endereçada por conteúdo, em vez de a um arquivo de parâmetros numa linha de banco.

O relato de Popper sobre conjectura e refutação é inspiração só para `revise_premise` [14]. Linhas de produto orientadas a features também falam em variantes [18]. Essas variantes são features de uma família de produto. Uma variante ACVC é uma hipótese sob teste. As palavras colidem; os objetos não.

## 5. O que torna o projeto interessante

O movimento interessante é a divisão de trabalho com o Git, usando o próprio truque do Git. O Git mostrou que uma versão pode ser o hash de uma árvore, e que uma árvore pode ser hashes de arquivos, de modo que um arquivo intacto não é escrito de novo. O ACVC conserva essa economia e muda o substantivo. O hash nomeia uma hipótese. Os bytes do arquivo ficam no armazenamento de blobs, compartilhados entre hipóteses. O Git recebe uma árvore só como depósito.

Quatro consequências são incomuns juntas.

A fronteira pode conter mais de uma hipótese viva. Um stitching mais rápido e um stitching mais limpo permanecem se nenhum domina estritamente o outro.

Uma perda não é uma morte. O sistema pode dizer quem venceu sem inventar um motivo. O motivo aparece só quando uma pessoa o escreve.

Um agente pode emitir muitas variantes numa sessão sem um commit por palpite. A sessão pode ser reconstruída a partir de premissas e ensaios depois que a conversa acabou.

O mesmo armazenamento serve a um pesquisador humano comparando regras de integração e a um engenheiro comparando um armazenamento SQL com um banco de documentos. Os comandos não mudam. As métricas mudam.

## 6. Projetos equivalentes e vizinhos

Nenhum destes é um substituto. Cada um possui um propósito. O ACVC possui a hipótese pontuada e os blobs compartilhados. Onde um projeto já guarda conteúdo por hash, a diferença é o que o hash pode significar.

| Projeto | O que guarda | Onde para, em relação ao ACVC |
| --- | --- | --- |
| Git [1, 2] | Blobs, árvores e commits endereçados por conteúdo | O commit é uma decisão. Hipóteses abandonadas não são uma fronteira. |
| Gitless [4] | Uma camada conceitual mais simples sobre o Git | Repara os conceitos do Git. Não acrescenta premissas nem ensaios. |
| container-use [11] | Um container, uma worktree e um branch por agente, mais o histórico de comandos em Git notes | Isola a execução. Não pontua hipóteses nem compartilha blobs intactos numa campanha. |
| Variolite [5] | Variantes locais leves dentro do editor | Variantes são trechos para exploração. Não são uma campanha Merkle com fronteira. |
| DVC [19] | Dados, modelos e pipelines, com hashes, sobre o Git | A execução do pipeline é o objeto. A aresta da hipótese e o conjunto não dominado de variantes de código não são. |
| MLflow [9] | Parâmetros, métricas e artefatos de execuções | A execução é uma linha. A árvore de código não é uma hipótese Merkle, e a dominação não é o status. |
| Sacred [10] | Configuração, fonte e resultados de experimentos computacionais | Escrituração de uma execução. Sem aresta de revisão de premissa, sem árvore de blobs compartilhada entre hipóteses. |
| Weights & Biases [20] | Rastreamento de experimentos hospedado | A mesma família do MLflow: execuções e gráficos, não um hash local de uma hipótese. |
| Jupyter [21] | Um notebook narrativo e suas saídas | A história é o notebook. Subárvores iguais não são compartilhadas por hash entre análises alternativas. |
| Unison [22] | Uma linguagem de programação endereçada por conteúdo | O código é guardado por hash. O hash nomeia uma definição, não uma hipótese medida numa campanha. |
| Nix [23] | Builds reproduzíveis endereçados pelo hash de uma derivação | O hash controla um build. Não registra qual premissa ainda não foi superada. |
| Jujutsu [24] | Um sistema de controle de versão compatível com Git, com outro modelo de trabalho | Continua sendo uma história de decisões. |
| Linhas de produto de software [18] | Variantes de feature de um produto | Uma feature não é uma hipótese sob um esquema de métricas. |

## 7. Limites

Não há estudo sobre se pesquisadores ou agentes registram premissas com honestidade. A fronteira só é tão boa quanto os ensaios. Um ensaio completo ainda pode ser a métrica errada. O ACVC não executa o código e não o isola; o container-use ou um shell comum faz isso. Windows está fora do modelo atual de identidade de arquivo, que usa metadados Unix. O paper e o software podem divergir; o executável é a autoridade sobre o comportamento.

## 8. Conclusão

O ACVC é uma campanha de hipóteses guardada do jeito que o Git guarda árvores: por hash, com os bytes do arquivo escritos só quando o conteúdo é novo. O hash controla a hipótese. O Git controla a decisão de conservar uma árvore. A fronteira mostra quais medições completas ainda não foram superadas, e recusa tratar uma perda como um motivo que ninguém escreveu.

## Referências

[1] S. Chacon and B. Straub, *Pro Git*, 2nd ed. Apress, 2014. "Git Internals — Git Objects." https://git-scm.com/book/en/v2/Git-Internals-Git-Objects

[2] L. Torvalds e os contribuidores do Git, Git, 2005–. https://git-scm.com/ O desenho de 2005 é um armazenamento de objetos endereçável por conteúdo: blobs e árvores nomeados por hash, commits nomeando o hash de uma árvore.

[3] J. O'Connor, J.-P. Aumasson, S. Neves, and Z. Wilcox-O'Hearn, "BLAKE3: one function, fast everywhere," 2020. https://github.com/BLAKE3-team/BLAKE3-specs/blob/master/blake3.pdf

[4] S. Perez De Rosso and D. Jackson, "Purposes, concepts, misfits, and a redesign of Git," in *Proc. OOPSLA 2016*, pp. 292–310. doi: 10.1145/2983990.2984018

[5] M. B. Kery, A. Horvath, and B. A. Myers, "Variolite: Supporting exploratory programming by data scientists," in *Proc. CHI 2017*, pp. 1265–1276. doi: 10.1145/3025453.3025626

[6] A. Rule, A. Tabard, and J. D. Hollan, "Exploration and explanation in computational notebooks," in *Proc. CHI 2018*, paper 32. doi: 10.1145/3173574.3173606

[7] A. Head, F. Hohman, T. Barik, S. M. Drucker, and R. DeLine, "Managing messes in computational notebooks," in *Proc. CHI 2019*, paper 270. doi: 10.1145/3290605.3300500

[8] H. T. Kung, F. Luccio, and F. P. Preparata, "On finding the maxima of a set of vectors," *Journal of the ACM*, vol. 22, no. 4, pp. 469–476, 1975. doi: 10.1145/321906.321910

[9] M. Zaharia et al., "Accelerating the machine learning lifecycle with MLflow," *IEEE Data Engineering Bulletin*, vol. 41, no. 4, pp. 39–45, 2018.

[10] K. Greff, A. Klein, M. Chovanec, F. Hutter, and J. Schmidhuber, "The Sacred infrastructure for computational research," in *Proc. SciPy 2017*, pp. 49–56. doi: 10.25080/shinma-7f4c6e7-008

[11] Contribuidores do Dagger, container-use, 2025–. https://github.com/dagger/container-use

[12] R. C. Merkle, "A digital signature based on a conventional encryption function," in *Advances in Cryptology — CRYPTO '87*, LNCS 293, Springer, 1988, pp. 369–378. doi: 10.1007/3-540-48184-2_32

[13] D. R. Hipp, "SQLite FTS5 extension." https://www.sqlite.org/fts5.html

[14] K. R. Popper, *The Logic of Scientific Discovery*. Hutchinson, 1959. Tradução inglesa de *Logik der Forschung*, 1934.

[15] L. Moreau, P. Missier, et al., "PROV-DM: The PROV data model," W3C Recommendation, 30 April 2013. https://www.w3.org/TR/prov-dm/

[16] V. Pareto, *Manuale di economia politica*. Società Editrice Libraria, 1906.

[17] K. Deb, *Multi-Objective Optimization Using Evolutionary Algorithms*. Wiley, 2001.

[18] S. Apel, D. Batory, C. Kästner, and G. Saake, *Feature-Oriented Software Product Lines*. Springer, 2013.

[19] Iterative, DVC: Data Version Control. https://dvc.org

[20] Weights & Biases. https://wandb.ai

[21] T. Kluyver et al., "Jupyter notebooks — a publishing format for reproducible computational workflows," in *Positioning and Power in Academic Publishing: Players, Agents and Agendas*, IOS Press, 2016, pp. 87–90. doi: 10.3233/978-1-61499-649-1-87

[22] P. Chiusano and R. Bjarnason, Unison. https://www.unison-lang.org

[23] E. Dolstra, "The purely functional software deployment model," Ph.D. dissertation, Utrecht University, 2006.

[24] M. von Zweigbergk e contribuidores, Jujutsu. https://github.com/jj-vcs/jj
