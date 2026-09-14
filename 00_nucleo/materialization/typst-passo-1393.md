# Passo 1393 — auditoria global de paridade com o vanila ratificado

## Objetivo

Medir o estado atual da paridade do cristalino com a linguagem observável do
vanila ratificado em `upstream/main@586e1bd43`, reconstruir um inventário
reproduzível do que está coberto, separar lacuna do produto de lacuna do
instrumento de medição e entregar uma fila priorizada do que ainda falta.

Este passo é exclusivamente diagnóstico e read-only sobre o produto. Ele não
altera Prompt L0, código L1–L4, manifesto da matriz nem fixtures permanentes.
Qualquer correção encontrada será materializada por passo posterior, com o gate
L0 correspondente. Não se declara equivalência geral a partir de amostras.

## Medição anterior à decisão

Estado observado ao redigir este passo, em 2026-09-14, HEAD
`c1f6f3930fa2fce80cb6e6b7765ee69c34de3b83`, working tree limpo:

1. O inventário global vigente ainda se identifica como atualização ad-hoc e
   parte de um snapshot antigo em
   `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md:1-18`.
   Sua última consolidação quantitativa é P415: `75` implementadas, `27`
   parciais e `141` itens, ou aproximadamente `72,3%`, em
   `00_nucleo/diagnosticos/typst-cobertura-vanilla-vs-cristalino.md:5847-5866`.
   Esse número antecede centenas de passos e não pode representar o estado
   atual sem recontagem.
2. `lab/parity/reports/latest.md:1-20` ainda aponta para P150: 31 arquivos,
   30 compilações cristalinas e colunas bilaterais `N/A`. Portanto, o nome
   `latest` não prova a paridade atual.
3. O corpus estrutural P480 registra 28/46 arquivos incluídos e 73/73
   comparações ativas em `lab/parity/SKIPS.md:73-102`, mas a evidência histórica
   usou uma referência anterior. Ela prova o subconjunto e o momento medidos,
   não o vanila hoje ratificado nem a linguagem inteira.
4. A matriz moderna pina `586e1bd43` e declara os perfis `default`, `html` e
   `a11y-extras` em `lab/parity/matrix/manifest.yaml:1-10`. Ela contém somente
   20 casos: S=4, E=1, B=3, I=1, L=2, X=7 e C=2. Essa cobertura é insuficiente
   para usar seus matches como denominador global.
5. O runner executa cada processo com timeout interno de 30 s
   (`lab/parity/matrix/runner.py:207-220`), repete os casos em ordem direta e
   reversa (`lab/parity/matrix/runner.py:620-680`) e oferece validação sem
   execução (`lab/parity/matrix/runner.py:609-619`). Isso permite uma auditoria
   serial e limitada em memória.
6. Identidades locais no momento da redação:
   - vanila `lab/typst-original/target/release/typst`:
     `eb60986b522d9843172cdf318dd46c81f5922109f503ab1733cfe8baaeb1468f`;
   - cristalino `target/release/typst`:
     `1f2f392c64b77ff558d103327a94d9a9fa0ccc48606e773b2bf936e3d93f0c00`;
   - manifesto:
     `8f443c60e426d814f931ec4c9097406275e4c8b83ec70a2c8b793edd344f127d`.

As contagens acima são diagnóstico de entrada, não veredito de paridade.

## Pergunta que o passo deve responder

Para cada superfície pública conhecida da linguagem Typst:

1. há implementação cristalina declarada e owner L0 válido;
2. há testemunho bilateral atual contra o pin ratificado;
3. o observável pertence a semântica, sintaxe ou morfologia da linguagem;
4. o resultado é match, diferença, ausência, parcial, desconhecido ou
   scope-out explícito;
5. se não há resposta, falta produto ou falta instrumento de medição?

Somente após responder isso para todo o denominador o relatório pode dizer o
que falta e recomendar o próximo passo de implementação.

## Vocabulário obrigatório do veredito

Cada item do inventário recebe exatamente um estado:

| Estado | Significado |
|---|---|
| `MATCH` | testemunho bilateral atual coincide no observável da linguagem |
| `DIFFERENCE` | ambos respondem, mas divergem semanticamente, sintaticamente ou morfologicamente |
| `ABSENT` | o vanila possui o comportamento e o cristalino não o expõe |
| `PARTIAL` | o cristalino cobre somente subconjunto delimitado do comportamento |
| `UNKNOWN` | a evidência atual não permite decidir; a razão deve ser literal |
| `SCOPE_OUT` | exclusão arquitetural explícita, com ADR ou L0 citado |

`MATCH` exige evidência do pin atual. Sucesso de compilação isolado,
`artifact_presence`, igualdade de bytes ou igualdade interna de Rust não sobe
automaticamente a `MATCH` sem justificar por que aquela mecânica é o
observável público. Um sentinela `DIFFERENCE` corretamente detectado valida o
instrumento, mas continua sendo diferença e nunca conta como paridade.

## Denominador e eixos

O inventário renovado parte das tabelas antigas, mas é reconciliado com Prompts
L0, ADRs, código produtivo e diagnósticos P416–P1392. Nenhum item é removido só
porque não há fixture. Duplicatas devem ser fundidas por comportamento público,
não por função, tipo ou arquivo Rust.

Cada item recebe um eixo primário:

- `S` — sintaxe, parsing e diagnósticos;
- `E` — avaliação e valores da linguagem;
- `B` — built-ins e stdlib;
- `I` — introspecção, estados, counters e queries;
- `L` — layout e morfologia paginada;
- `X` — exportação e observáveis dos formatos;
- `C` — CLI e integração pública necessária à linguagem.

O relatório mantém separadas três razões:

1. **cobertura declarada**: itens implementados ou parciais sobre o denominador;
2. **cobertura medida**: itens com testemunho bilateral atual sobre o
   denominador;
3. **paridade medida**: `MATCH` sobre itens efetivamente decididos por evidência.

Toda razão publica numerador, denominador e contagem de `UNKNOWN`. Não haverá
uma única “percentagem de paridade” que esconda itens ainda não medidos.

## Execução

### Fase A — congelar proveniência

Registrar em `p1393-paridade-relatorio.md` antes de executar casos:

- timestamp ISO-8601 com timezone;
- `git rev-parse HEAD`;
- `git status --short` e `git diff HEAD --stat`;
- pin completo do vanila;
- caminho, tamanho e SHA-256 dos dois binários;
- SHA-256 do manifesto e das listas de casos auxiliares;
- comandos exatos e limites de memória/tempo.

Se o cristalino não corresponder ao HEAD ou ao working tree declarado, marcar
`UNKNOWN` por identidade obsoleta. Só então um rebuild release separado pode
ser autorizado dentro do limite definido abaixo. A string `--version` é
evidência auxiliar, nunca identidade suficiente.

### Fase B — reconstruir o inventário

1. Extrair a tabela antiga sem confiar nos estados históricos.
2. Confrontar cada linha com L0 proprietário, consumer produtivo e ADR vigente.
3. Incorporar comportamentos materializados entre P416 e P1392.
4. Identificar entradas duplicadas, agregados ambíguos e itens sem critério
   observável.
5. Atribuir a cada item um ID estável, eixo, estado declarado, owner L0,
   consumer, evidência existente e caso bilateral correspondente.
6. Marcar `UNKNOWN` onde a revisão documental não bastar; não inferir suporte
   pela mera presença de símbolos no código.

Saída: `00_nucleo/diagnosticos/p1393-paridade-inventario-atualizado.md`.

### Fase C — auditar a capacidade da matriz

Mapear os 20 casos atuais para os IDs do inventário e registrar:

- qual comportamento cada caso realmente decide;
- se o comparador observa linguagem ou somente mecânica;
- quais itens não possuem caso bilateral;
- quais fixtures históricas ainda apontam para referência obsoleta;
- quais comparadores ou normalizadores faltam.

Casos candidatos não entram no manifesto permanente neste passo. Eles são
descritos em uma lista temporária com fonte mínima, comando bilateral,
observável, resultado esperado e risco de falso positivo.

### Fase D — executar a linha de base atual

Executar primeiro a validação, depois os três perfis, sempre serialmente e com
artefatos em `/tmp`. O runner já executa ordem direta e reversa; discordância de
ordem permanece `UNKNOWN`.

```bash
bash -lc 'ulimit -v 2097152; exec timeout 60s /usr/bin/time -v python3 lab/parity/matrix/runner.py --validate-only'
bash -lc 'ulimit -v 2097152; exec timeout 600s /usr/bin/time -v python3 lab/parity/matrix/runner.py --output /tmp/p1393-default.json'
bash -lc 'ulimit -v 2097152; exec timeout 600s /usr/bin/time -v python3 lab/parity/matrix/runner.py --profile html --output /tmp/p1393-html.json'
bash -lc 'ulimit -v 2097152; exec timeout 600s /usr/bin/time -v python3 lab/parity/matrix/runner.py --profile a11y-extras --output /tmp/p1393-a11y.json'
```

Falha de expectativa não interrompe o inventário: é dado a classificar.
Timeout, OOM, dependência externa ausente ou erro do harness produz `UNKNOWN`,
nunca `ABSENT` nem `DIFFERENCE` automaticamente.

### Fase E — preencher lacunas de evidência por amostragem estratificada

Para cada eixo, escolher primeiro itens com maior impacto e sem testemunho
atual. Executar casos bilaterais temporários em lotes de no máximo 25, com uma
única característica discriminante por fixture. Cada lote deve conter:

- ao menos um controle positivo conhecido;
- ao menos um sentinela de diferença conhecida, quando houver;
- execução direta e reversa;
- resultado bruto dos dois binários;
- observável normalizado e justificativa da classificação.

A amostragem serve para decidir itens específicos e descobrir classes de gap.
Ela não autoriza extrapolar o resultado para irmãos não executados. Quando um
agregado do inventário contiver comportamentos diferentes, ele será dividido.

### Fase F — classificar o que falta

Gerar uma fila ordenada em cinco grupos:

1. **bloqueadores da linguagem** — vanila aceita/produz e cristalino falha ou
   responde com semântica incompatível;
2. **comportamentos parciais** — o caminho principal existe, mas casos públicos
   delimitados faltam;
3. **lacunas de medição** — produto possivelmente presente, porém sem oráculo,
   comparador ou fixture suficiente;
4. **scope-outs explícitos** — decisões mantidas, sem tratá-las como match;
5. **diferenças mecânicas irrelevantes** — registradas e excluídas da fila de
   paridade segundo ADR-0107.

Dentro dos grupos 1 e 2, ordenar por: superfície pública afetada, frequência
provável, dependências desbloqueadas, tamanho do menor witness e risco de
regressão. O relatório recomenda exatamente um próximo passo, mas não o
implementa.

## Artefatos obrigatórios

O passo entrega:

1. `00_nucleo/diagnosticos/p1393-paridade-inventario-atualizado.md` —
   denominador reconciliado e rastreável;
2. `00_nucleo/diagnosticos/p1393-paridade-matriz.json` — resultados atuais e
   referências aos lotes temporários;
3. `00_nucleo/diagnosticos/p1393-paridade-gaps.md` — fila priorizada do que
   falta, separando produto de harness;
4. `00_nucleo/diagnosticos/p1393-paridade-relatorio.md` — proveniência,
   contagens, limites, falhas e veredito;
5. `00_nucleo/diagnosticos/p1393-paridade-gaps.json` — forma legível por máquina
   com ID, eixo, estado, evidência e próximo witness.

`lab/parity/reports/latest.md` só poderá ser atualizado depois de o relatório
P1393 estar selado. O conteúdo P150 deve ser preservado como histórico, não
sobrescrito sem cópia identificável.

## Disciplina de memória e parada

- Não iniciar por `cargo test --workspace` nem por rebuild global.
- Runner e lotes bilaterais: 2 GiB, serial, subprocesso com 30 s por lado e
  limite exterior explícito.
- Teste Rust focal, somente se indispensável para validar o harness: 6 GiB e
  180 s, um binário de teste por vez.
- Rebuild release, somente por identidade obsoleta: 4 GiB e 180 s.
- `cargo check` ou lint, se algum artefato executável precisar mudar em passo
  posterior: 3 GiB e execução serial.
- Registrar RSS máximo, swap, duração e status de cada comando medido.
- Manter artefatos volumosos em `/tmp`; somente sumários e evidência mínima
  reproduzível entram no repositório.
- Após duas tentativas com a mesma falha e nenhuma informação nova, parar esse
  caso, classificá-lo `UNKNOWN` com causa literal e continuar pelos demais.
- Se o RSS superar 85% do limite ou crescer sem estabilizar, interromper o lote,
  reduzir sua cardinalidade e preservar o último caso concluído.

## Critérios de aceitação

O passo fecha somente quando:

1. todo número decisório inclui HEAD, estado exato da working tree, timestamp e
   identidades dos binários;
2. o pin completo `586e1bd43fae6c9a973218163d3165c53ab8d16d` é a única autoridade do
   vanila; versões textuais antigas não fundamentam veredito;
3. todo item do denominador possui exatamente um estado e uma evidência atual,
   ou razão literal para `UNKNOWN`;
4. os três perfis da matriz foram executados, ou a impossibilidade foi
   registrada sem transformar falha de harness em falha do produto;
5. os resultados distinguem semântica, sintaxe e morfologia de igualdade de
   bytes, estrutura interna e algoritmo;
6. cobertura declarada, cobertura medida e paridade medida aparecem separadas,
   cada uma com numerador e denominador;
7. todos os `UNKNOWN`, `PARTIAL`, `ABSENT`, `DIFFERENCE` e `SCOPE_OUT` aparecem
   nominalmente na fila ou na justificativa de exclusão;
8. a fila identifica o menor witness reproduzível do item prioritário e propõe
   um único próximo passo;
9. o relatório declara explicitamente que a auditoria não prova equivalência
   geral além dos observáveis executados;
10. nenhuma alteração de produto, L0 ou manifesto permanente foi feita durante
    esta auditoria.

## Regime Tekt deste passo

Por ser diagnóstico read-only, este passo não reivindica independência de
implementação nem exige materialização segregada de código. Ainda assim,
inventário, execução, classificação e veredito permanecem seções distintas: a
decisão deve citar a medição que a precede, e nenhuma ausência de evidência pode
ser convertida em evidência de ausência.
