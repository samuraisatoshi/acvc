# ACVC

[English](README.md) · [Português](README.pt-BR.md)

O ACVC (Agent Code Variant Control) guarda as hipóteses que você experimentou, as medições que fez e quais tentativas ainda não foram superadas. O Git guarda a variante que você decide registrar. Ele não guarda as tentativas abandonadas no caminho.

Isto é para quem compara abordagens: pesquisadores, desenvolvedores e engenheiros de software. Um agente pode executar os mesmos comandos. Uma pessoa lê o resultado com `--format text`, `acvc tui` ou `acvc web`.

O desenho, os problemas que o motivaram, os métodos e os sistemas vizinhos estão em [PAPER.md](PAPER.md) ([português](PAPER.pt-BR.md)). Essa nota é uma descrição do sistema. Não é um artigo com revisão por pares e não relata um estudo com usuários.

## Instalar

Gere o binário `acvc` a partir deste repositório:

```bash
cargo build --release -p acvc-cli
install -m 755 target/release/acvc ~/.local/bin/acvc
```

Os arquivos de release, quando publicados, são Linux x86_64 e macOS Apple Silicon. Cada arquivo traz o binário e o `LICENSE`.

A saída padrão é JSON. Use `--format text` quando quiser frases. As frases seguem o idioma do computador (`LC_ALL`, depois `LC_MESSAGES`, depois `LANG`). O inglês é o recurso quando o idioma não é identificado. Estes quatro valores de status no JSON não mudam: `na fronteira`, `superada`, `rejeitada`, `sem medição`.

## Uma campanha

```bash
acvc init
acvc --format text campaign create integral \
  --metric residual:lower \
  --metric evaluations:lower
```

Uma métrica é `nome:lower` ou `nome:higher`. `lower` significa que o número menor vence. `higher` significa que o número maior vence. O nome é um token: letras, dígitos e `_`.

`acvc seed` guarda a primeira árvore da campanha. Uma campanha tem uma origem. As tentativas seguintes são `acvc record` a partir desse workspace, depois de editar os arquivos.

Uma pontuação nomeia todas as métricas. `acvc score --metric residual=1e-4 --metric evaluations=80` é um ensaio. Um ensaio que omite uma métrica fica fora da fronteira.

A fronteira guarda todo ensaio completo que nenhuma outra hipótese supera em todas as métricas. Duas tentativas podem permanecer: uma mais precisa, a outra mais barata. Um ensaio estritamente pior sai da fronteira. Essa saída não é uma rejeição. Você rejeita só quando decide que a tentativa morreu:

```bash
acvc --format text reject --variant <id> --reason "a série diverge no extremo"
```

Se uma tentativa está fora da fronteira e você não a rejeitou, a leitura diz quem venceu e que o motivo não foi gravado.

`acvc promote --variant <id> --git <checkout>` copia a árvore escolhida para um checkout Git que já existe. Esse é o depósito. As outras tentativas continuam em `.acvc`.

Não abra `.acvc/graph.sqlite`, o log nem o pack. Pergunte com `acvc frontier`, `acvc why`, `acvc compare` e `acvc recall`.

## ACVC e container-use

O [container-use](https://github.com/dagger/container-use) (Dagger) dá a um agente de código um container isolado e um branch Git. O ACVC dá a uma pessoa, ou a um agente, o registro das hipóteses e das medições. Um não substitui o outro. A escolha segue o objetivo.

O container-use cria um environment: um container Dagger, uma worktree e um branch. Os comandos rodam nesse container. As mudanças de arquivo caem nesse branch. O histórico de operações e o estado do container ficam em Git notes. Você inspeciona com `container-use log`, `diff` e `checkout`, e aceita o trabalho com `merge` (fica com os commits do environment) ou `apply` (deixa o diff preparado e escreve o seu commit). Um environment que falhou pode ser apagado. Vários agentes podem trabalhar ao mesmo tempo porque cada um tem o próprio container e o próprio branch.

O ACVC cria uma campanha. A primeira árvore é um seed: premissa, pressupostos, intenção e os arquivos. As árvores seguintes são records (`mutate`, `revert`, `revise_premise`, `synthesize`). Uma pontuação é um ensaio contra as métricas da campanha. A fronteira guarda todo ensaio completo que nenhuma outra hipótese supera em todas as métricas. O Git recebe uma árvore só quando você faz `promote` da que escolheu.

### Comparação

| Pergunta | ACVC | container-use |
| --- | --- | --- |
| O que é uma tentativa? | Uma variante: premissa, pressupostos, intenção, aresta e uma árvore endereçada pelo conteúdo | Um environment: um container mais um branch Git |
| Onde ficam as outras tentativas? | Em `.acvc`, até você promover uma árvore para um checkout Git | Em branches de environment, com commits e Git notes |
| O que o histórico responde? | Quais medições completas ainda não foram superadas, e o motivo de uma rejeição | Quais comandos rodaram, e o diff daquele branch |
| Como se descarta? | `reject` com um motivo, ou a tentativa fica fora da fronteira | Apaga o environment |
| Como o trabalho escolhido entra no seu branch? | `promote --git` copia essa árvore para um checkout que já existe | `merge` conserva os commits do environment, ou `apply` prepara o diff |
| Quem executa o código? | Você. O ACVC guarda a árvore e os números que você informa | O agente executa comandos dentro do container |
| O que isola a execução? | Nada. O workspace é um diretório que você já tem | O container e a worktree |
| Para quem serve | Um pesquisador ou um engenheiro comparando abordagens medidas, com ou sem agente | Agentes em paralelo que não podem mexer no seu checkout |

### Tradeoffs, pelo objetivo

Medir abordagens que competem. Use o ACVC. Um pipeline de stitching, uma regra de integração ou um armazenamento de comandas é uma campanha: as mesmas métricas, várias premissas, uma fronteira que pode guardar mais de um vencedor. O container-use guarda cada protótipo num branch e mostra os comandos. Ele não diz qual ensaio completo ainda não foi superado, e não guarda a premissa ao lado da pontuação.

Deixar agentes trabalharem em paralelo sem colidir. Use o container-use. Cada agente recebe um container e um branch, você acompanha o log, e faz merge ou apaga. O ACVC não sobe esse container, não esconde a execução da sua árvore de trabalho e não grava o histórico do shell.

Guardar o Git para a decisão. Use o ACVC quando as tentativas abandonadas devem ficar fora do histórico do branch. Use o container-use quando você quer essas tentativas como branches para fazer checkout, e aceita os commits do agente até o merge ou o apply.

Precisa dos dois. Um agente pode prototipar dentro de um environment do container-use. A hipótese que você quer lembrar — premissa, métricas e se ela ainda se sustenta — é uma variante ACVC da árvore que você conserva. O container explica como a execução foi isolada. A campanha explica o que a execução afirmava e como ela foi medida.

## Licença

Uso, estudo e modificação para pesquisa são permitidos quando o resultado não é um produto comercial. Uso comercial não é concedido. Isso inclui freeware, freemium e pesquisa cujo objetivo é gerar software comercial. Esses usos precisam de autorização por escrito de Samuraisatoshi. Veja `LICENSE`.

O terceiro caso abaixo é uma pergunta de SaaS comercial. Executá-lo como produto real exige essa autorização por escrito. Os comandos são os mesmos.

## Caso A. Um problema matemático

Você está integrando uma função suave num intervalo fechado. Importam o resíduo e quantas avaliações a regra gasta. Resíduo menor é melhor. Menos avaliações é melhor.

```bash
mkdir -p integral
printf 'trapezoid\n' > integral/rule.txt
acvc init
acvc --format text campaign create integral \
  --metric residual:lower \
  --metric evaluations:lower
acvc --format text seed \
  --campaign integral \
  --path integral \
  --premise "a regra do trapézio basta para um integrando suave" \
  --assumption "o integrando é suave num intervalo fechado" \
  --intent "um resíduo abaixo de 1e-6" \
  --agent researcher
```

O comando imprime o id da variante. Chame-o de `<trapezio>`. Edite `integral/rule.txt` para a implementação de Simpson e grave essa tentativa. O record usa a variante atual do workspace como pai.

```bash
printf 'adaptive-simpson\n' > integral/rule.txt
acvc --format text record \
  --workspace integral \
  --edge mutate \
  --premise "Simpson adaptativo gasta menos avaliações num integrando suave" \
  --assumption "o integrando é suave num intervalo fechado" \
  --intent "um resíduo abaixo de 1e-8" \
  --agent researcher
```

Chame esse id de `<simpson>`. Edite de novo e grave uma expansão em série. Chame-a de `<serie>`.

Pontue cada tentativa com as duas métricas. Estes números contam a história; não são uma execução real:

```bash
acvc --format text score --variant <trapezio> --metric residual=1.4e-3 --metric evaluations=400
acvc --format text score --variant <simpson> --metric residual=2.1e-8 --metric evaluations=80
acvc --format text score --variant <serie> --metric residual=4.0e-10 --metric evaluations=360
```

Simpson vence o trapézio nas duas métricas, então o ensaio do trapézio sai da fronteira. Simpson e a série permanecem: Simpson gasta menos avaliações, a série tem o resíduo menor. Nenhum dos dois é estritamente melhor.

```bash
acvc --format text frontier --campaign integral
acvc --format text why <trapezio>
acvc --format text compare <simpson> <serie>
acvc --format text recall --campaign integral --query "Simpson"
```

`why` na tentativa do trapézio nomeia quem venceu e a métrica, e diz que o motivo não foi gravado. Rejeite só se você decidiu que a regra em si acabou:

```bash
acvc --format text reject --variant <trapezio> \
  --reason "Simpson a domina num intervalo suave"
```

Quando o integrando é singular, a premissa muda. Isso não é um ajuste pequeno da mesma ideia:

```bash
printf 'substitution-then-simpson\n' > integral/rule.txt
acvc --format text record \
  --workspace integral \
  --edge revise_premise \
  --premise "uma singularidade no extremo pede uma substituição antes de Simpson" \
  --assumption "o integrando explode num extremo" \
  --intent "um resíduo abaixo de 1e-6 na integral singular" \
  --agent researcher
```

Pontue essa tentativa nova com os números dela. Não copie os números do intervalo suave. Promova para o Git a tentativa em que você confia:

```bash
acvc --format text promote --variant <serie> --git .
```

## Caso B. Costura de imagens

Você está melhorando um pipeline de stitching. Três números importam: o erro da emenda, o tempo de execução e quanto da cena ficou coberto. Emenda e tempo menores são melhores. Cobertura maior é melhor.

```bash
mkdir -p stitch
printf 'global-homography\n' > stitch/method.txt
acvc init
acvc --format text campaign create stitch \
  --metric seam:lower \
  --metric runtime_ms:lower \
  --metric coverage:higher
acvc --format text seed \
  --campaign stitch \
  --path stitch \
  --premise "uma homografia basta quando a cena é quase planar" \
  --assumption "o movimento da câmera é um único plano" \
  --intent "cobrir a sobreposição sem uma emenda visível" \
  --agent researcher
```

Troque o método, grave e pontue. O pai é o workspace que você acabou de semear.

```bash
printf 'local-mesh\n' > stitch/method.txt
acvc --format text record \
  --workspace stitch \
  --edge mutate \
  --premise "uma malha local absorve a paralaxe que uma homografia não absorve" \
  --assumption "a cena tem paralaxe leve" \
  --intent "aumentar a cobertura sem rasgar a sobreposição" \
  --agent researcher

printf 'seam-aware-blend\n' > stitch/method.txt
acvc --format text record \
  --workspace stitch \
  --edge mutate \
  --premise "uma mistura consciente da emenda esconde o erro residual da malha" \
  --assumption "a malha já alinha a sobreposição" \
  --intent "cortar o erro da emenda sem perder cobertura" \
  --agent researcher
```

Chame os três ids de `<homografia>`, `<malha>` e `<mistura>`.

```bash
acvc --format text score --variant <homografia> --metric seam=0.42 --metric runtime_ms=80 --metric coverage=0.71
acvc --format text score --variant <malha> --metric seam=0.18 --metric runtime_ms=640 --metric coverage=0.93
acvc --format text score --variant <mistura> --metric seam=0.07 --metric runtime_ms=710 --metric coverage=0.93
```

As três podem permanecer na fronteira. A homografia é a mais rápida e a menos precisa. A malha cobre mais e custa tempo. A mistura tem a emenda mais limpa, a mesma cobertura da malha e um tempo maior, então ela não apaga a malha: a malha continua mais rápida. Leia isso antes de escolher.

```bash
acvc --format text frontier --campaign stitch
acvc --format text compare <malha> <mistura>
acvc --format text why <homografia>
acvc --format text file-diff <homografia> <mistura> --path method.txt
```

Se uma experiência com malha rasgou a sobreposição, grave a volta com `--edge revert` e uma premissa que diga o que falhou. Promova a mistura quando essa for a árvore que você quer no Git:

```bash
acvc --format text promote --variant <mistura> --git .
```

## Caso C. SQL ou banco de documentos num SaaS de restaurante

Você está escolhendo o armazenamento por trás da venda de comandas. A superfície comum é a API HTTP e os testes. Os dois armazenamentos são tentativas posteriores, não duas origens: uma campanha tem um seed.

Esta comparação é sobre um produto comercial. Usar o ACVC para construir esse produto precisa de autorização por escrito do autor. Veja `LICENSE`.

As métricas são a latência de cauda, as escritas perdidas durante uma partição e a conta mensal. Menor é melhor nas três.

```bash
mkdir -p orders
printf 'api-and-tests\n' > orders/store.txt
acvc init
acvc --format text campaign create orders \
  --metric p95_ms:lower \
  --metric lost_writes:lower \
  --metric monthly_usd:lower
acvc --format text seed \
  --campaign orders \
  --path orders \
  --premise "a comanda é a unidade de venda e o armazenamento ainda não foi decidido" \
  --assumption "um salão, uma comanda aberta por mesa" \
  --intent "escolher um armazenamento que não perde uma comanda paga" \
  --agent researcher
```

```bash
printf 'postgresql\n' > orders/store.txt
acvc --format text record \
  --workspace orders \
  --edge mutate \
  --premise "uma tabela de comandas no PostgreSQL mantém a comanda durável" \
  --assumption "um primário basta para uma região" \
  --intent "zero escritas perdidas sob partição" \
  --agent researcher

printf 'document\n' > orders/store.txt
acvc --format text record \
  --workspace orders \
  --edge mutate \
  --premise "um documento por comanda evita um join no caminho quente" \
  --assumption "uma escrita perdida pode ser repetida pelo cliente" \
  --intent "uma conta menor e um p95 menor" \
  --agent researcher
```

O pai do record do documento é o workspace do PostgreSQL, porque o record segue o marcador do workspace. Se as duas tentativas precisam ramificar do seed da API, materialize o seed num segundo diretório antes do segundo record:

```bash
acvc --format text materialize --variant <api> --into orders-doc
# edite orders-doc/store.txt e então:
acvc --format text record \
  --workspace orders-doc \
  --parent <api> \
  --edge mutate \
  --premise "um documento por comanda evita um join no caminho quente" \
  --assumption "uma escrita perdida pode ser repetida pelo cliente" \
  --intent "uma conta menor e um p95 menor" \
  --agent researcher
```

Pontue os dois armazenamentos. Nenhum conjunto de números domina o outro: o PostgreSQL não perde escritas e custa mais; o banco de documentos é mais rápido, mais barato e perde escritas.

```bash
acvc --format text score --variant <postgres> --metric p95_ms=48 --metric lost_writes=0 --metric monthly_usd=220
acvc --format text score --variant <documento> --metric p95_ms=19 --metric lost_writes=3 --metric monthly_usd=90
acvc --format text frontier --campaign orders
acvc --format text compare <postgres> <documento>
```

A fronteira segurando os dois é o ponto. A escolha é sua. Se uma comanda perdida no caixa é inaceitável, diga isso. A rejeição é o motivo, não um efeito colateral da pontuação:

```bash
acvc --format text reject --variant <documento> \
  --reason "uma comanda perdida durante uma partição não é aceitável no caixa"
acvc --format text promote --variant <postgres> --git .
```

## Ler a campanha

```bash
acvc --format text frontier
acvc --format text context
acvc tui
acvc web
```

`acvc web` escuta em `127.0.0.1:9420`. `acvc guide` é o mapa curto da campanha aberta. `acvc assistant` devolve o contrato operacional para um agente, no idioma do computador.
