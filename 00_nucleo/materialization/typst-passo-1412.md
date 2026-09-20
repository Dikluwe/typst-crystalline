# Passo 1412 — regras de estilo de `outline` e `outline.entry`

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Fechar as duas rotas ainda `UNKNOWN` da matriz P1409 por paridade bilateral de
linguagem: `outline.entry.indented` deve receber o contexto privado do outline
que a produziu; `outline.entry.inner` deve observar `#set outline.entry(fill:)`.
O passo implementa `#set outline(indent:)` e `#set outline.entry(fill:)`, sem
ampliar a superfície para outros campos.

## Estado e proveniência inicial

- estado: commit `2ff1a23d4304e3b2b512d4d09f57e2e934db3398`, árvore limpa;
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- vanila ratificado: commit `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário `lab/typst-original/target/release/typst`, identificador administrativo
  [dado administrativo removido];
- preparação iniciada às `2026-09-15T18:16:57-03:00`;
- working tree anterior ao RED: este passo e as seis fixtures focais P1412;
  um sétimo controle mínimo U+2060 foi acrescentado após a matriz parcial.

## Medição antes da decisão

`eval_set_rule` reconhece alvos simples, além do caso pontuado
`math.equation`, mas não reconhece `outline` nem `outline.entry`. A realização
de headings em `rules.rs` fabrica sempre o fill pontilhado. Quando uma show
rule devolve `ContextBlock`, o valor produzido também não carrega os estilos
privados `outline.parent.*`; por isso a expansão P1411 encontra o bloco, mas
`it.indented` não encontra o parent causal.

Os oráculos focais e controles medem, antes de L1: saída morfológica de
`indented` e `inner`, default herdado de `fill`, precedência do argumento
explícito e presença efetiva do `indent` herdado/explicitamente substituído.

## Decisão

Esta é correção de paridade em tabela/regra interna: segue o fluxo contínuo da
ADR-0127. Antes do código, atualizar os prompts L0 proprietários e ressellar:

1. `compiler/eval/rules.md` legitima os dois targets, valida seus tipos, aplica
   estilos somente quando o argumento não foi explicitado e transporta os
   estilos privados do parent sobre resultados diferidos de show rules;
2. `entities/elements/outline.md` legitima um bit privado de presença de
   `indent`, preservado por clone, hash, igualdade e walkers, sem campo novo na
   linguagem nem mudança de assinatura pública.

`fill` aceita `content | none`. `indent` aceita `length | function | auto` e o
`bool` cristalino já compatível. Argumento explícito vence `set`; escopo léxico
continua sendo o `StyleChain`. Campo desconhecido é erro, nunca ignorado.

A primeira matriz GREEN parcial isolou um resíduo: `inner` passou a executar e
herdar `fill: none`, mas o PDF cristalino omitia U+2060 que o vanila conserva
na extração. A decisão P1404 de remover o controle dos sub-runs visuais evitava
o OOM, porém também removia esse observável. O owner `infra/shaper.md` é
atualizado antes de L3: controles conhecidos não fazem busca de cobertura;
entram no sub-run corrente ou no primeiro primário quando isolados. Isso
preserva o limite de memória e permite o `ToUnicode` sem busca global.

## Testes e ataques

Confirmar RED no binário inicial e o oráculo no vanila. Depois do GREEN:

- remover o carrier privado deve fazer `indented` voltar a falhar;
- remover a consulta do style deve restaurar os pontos em `inner`;
- `fill: [X]` explícito deve vencer `#set outline.entry(fill: none)`;
- `outline(indent: auto)` explícito deve vencer `#set outline(indent: 4em)`;
- o valor omitido deve herdar o set na morfologia posicionada do outline;
- repetir toda a matriz P1409, sem promover `par.line` ou `raw.line` por
  inferência.

## Protocolo de memória

- compilação Typst: `ulimit -Sv 524288`, `RAYON_NUM_THREADS=1`, timeout 15 s,
  saída isolada em `/tmp`;
- build release: limite 2 GiB, um job, incremental desligado, debug zero,
  `codegen-units=512` e linker `gold`;
- lint: limite 1 GiB, uma thread, timeout 60 s;
- proibido executar `cargo test -p typst-core --lib` ou `cargo check --tests`.

## Aceitação

Os seis oráculos focais e o controle U+2060 devem coincidir com o vanila no nível da linguagem, com exit
zero e stderr vazio. A matriz P1409 limitada deve terminar `PASS`, promovendo
`outline.entry.indented` e `outline.entry.inner` a `MATCH`. Build, formatação,
`git diff --check` e `crystalline-lint .` devem passar nos limites acima.

Execução em rehearsal completo, sem alegação de isolamento entre autoria e
veredito.

## RED, ataque de memória e GREEN executados

O RED foi executado no baseline às `2026-09-15T18:18:33-03:00`. O cristalino
avisava que os dois targets não eram suportados; `indented` terminava em erro
por falta do parent, e `inner` ainda mostrava o fill pontilhado. O vanila
produziu `Contents\nP I\n\n1. H\n\f` e
`Contents\nH          U+2060 1\n\n1. H\n\f`. O maior RSS observado nessa
rodada foi 43.076 KiB.

Dois controles iniciais foram rejeitados antes de ganhar crédito: leitura de
`entry.fill` fora do contexto é inválida no vanila; `indent: false` é extensão
cristalina, não domínio bilateral. Foram substituídos por leitura contextual e
override explícito `auto`. A precedência foi observada ainda pelas coordenadas
xMin: valor herdado `4em` produziu 44,0 pt; `auto` explícito produziu 13,2 pt
nos dois produtos.

A primeira matriz depois das regras retornou `{ABSENT: 2, MATCH: 3, UNKNOWN:
1}`: somente a extração de U+2060 impedia `inner` de fechar. O ataque mínimo
`a<U+2060>b` confirmou que conservar o controle num sub-run sem consulta de
cobertura não reabria o OOM. No GREEN final, os sete processos cristalinos e
os sete vanila terminaram com exit zero, stderr vazio e projeções extraídas
idênticas; RSS cristalino máximo 43.148 KiB. O binário cristalino final tem
identificador administrativo [dado administrativo removido].

## Reprobe P1409 final

Executado às `2026-09-15T18:38:24-03:00` sobre o baseline acima com working
tree não commitada descrita por `git diff HEAD --stat` no commit P1412.
Recibo relatório consolidado da limpeza:

- runner `PASS`, 84 processos, invariância nas três ordens;
- estados `{ABSENT: 2, MATCH: 4}`;
- RSS máximo 42.924 KiB e swap máximo 0 KiB;
- `outline.entry.indented` e `outline.entry.inner` promovidos a `MATCH`;
- `par.line` e `raw.line` permanecem `ABSENT`.

O build release limitado concluiu em 10 min 01 s na primeira passagem e 1 min
05 s após a correção isolada do shaper. Nenhum alvo guloso de testes foi
executado. Veredito do rehearsal: `Preserved` para os oráculos bilaterais,
precedência e proteção de memória; executado sem atestação de isolamento.
