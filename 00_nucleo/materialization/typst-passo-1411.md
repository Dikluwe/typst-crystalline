# Passo 1411 — estabilizar contexto no `Outline` realizado

> **Nota de preservação (limpeza de 2026-09-19):** este passo é mantido como
> registro histórico. Menções a selos, recibos, certificados, manifests, hashes
> administrativos, papéis de agentes e gates do protocolo revogado não criam
> requisitos vigentes. Os artefatos removidos foram consolidados em
> relatório consolidado da limpeza; o comportamento normativo
> permanece nos Prompts L0 e nos testes funcionais atuais.


## Objetivo

Integrar à fase paginada o conteúdo contextual produzido por
`show outline.entry` durante a realização pós-introspecção. O critério focal é
paridade bilateral de linguagem para `outline.entry.page()` e
`outline.entry.prefix()`.

As construções `#set outline(indent: ...)` e
`#set outline.entry(fill: ...)` permanecem fora deste passo: P1409 mediu nelas
avisos e semântica de regras de estilo independentes desta travessia.

## Estado e proveniência inicial

- estado: commit `82e210bd1826ccbf0cd2378cddc605fb2546cf34`, árvore limpa;
- cristalino: `target/release/typst`, identificador administrativo
  [dado administrativo removido];
- vanila ratificado: commit `586e1bd43fae6c9a973218163d3165c53ab8d16d`,
  binário `lab/typst-original/target/release/typst`, identificador administrativo
  [dado administrativo removido];
- medição inicial preparada às `2026-09-15T17:31:45-03:00`;
- alterações antes do RED: este passo, a hipótese L0 inicial e os dois oráculos
  P1411; a hipótese foi depois falsificada e revertida.

## Auditoria e falsificação antes da decisão

A primeira hipótese apontava somente para `Content::map_content`: o hub tratava
`Outline` como terminal, embora `OutlineElem::map_content` já percorresse
`realized_body`. A delegação foi implementada provisoriamente, ressellada e
compilada sob o protocolo de memória. O segundo probe, às
`2026-09-15T17:38:13-03:00`, permaneceu RED: cristalino continuou emitindo
`Contents\n1. H\n\f` nos dois casos, com RSS de 42.252 KiB (`page`) e
42.116 KiB (`prefix`). A suficiência da hipótese foi refutada e a alteração L1
foi revertida antes do gate; a travessia continua necessária depois que o corpo
realizado passar a existir antes da nova expansão.

A ordem efetiva em `03_infra/src/pipeline.rs` é expansão contextual original,
introspecção, realização P1406 e layout. Logo, o `ContextBlock` só nasce depois
da expansão que poderia avaliá-lo. Os owners são
`00_nucleo/prompts/infra/pipeline.md`, consumer 1:1
`03_infra/src/pipeline.rs`, para a ordem das fases, e
`00_nucleo/prompts/entities/content.md`, consumer 1:1
`01_core/src/entities/content.rs`, para a travessia exaustiva.

## Protocolo de memória

- cada compilação Typst: `ulimit -Sv 524288`, `RAYON_NUM_THREADS=1`, timeout
  de 15 s e saída em diretório temporário isolado;
- build: limite de 2 GiB, um job, incremental desligado, debug zero e linker
  `gold`;
- lint: limite de 1 GiB, uma thread e timeout de 60 s;
- proibido executar `cargo test -p typst-core --lib` ou `cargo check --tests`.

## RED obrigatório

Executar os dois oráculos contra os binários acima antes de mudar L1. O vanila
deve expor respectivamente `1` e `1.` no corpo do outline; o cristalino antigo
deve omiti-los. Registrar stdout extraído, stderr, exit code e pico de RSS.

Executado às `2026-09-15T17:32:50-03:00`, ainda no commit inicial, com
working tree contendo apenas L0, passo e fixtures P1411:

| Oráculo | Implementação | Texto extraído (`pdftotext -layout`) | Exit | RSS pico | stderr |
|---|---|---|---:|---:|---|
| `page` | cristalino | `Contents\n1. H\n\f` | 0 | 42.404 KiB | vazio |
| `page` | vanila | `Contents\n1\n\n1. H\n\f` | 0 | 30.988 KiB | vazio |
| `prefix` | cristalino | `Contents\n1. H\n\f` | 0 | 42.528 KiB | vazio |
| `prefix` | vanila | `Contents\n1.\n\n1. H\n\f` | 0 | 31.164 KiB | vazio |

RED confirmado: os dois observáveis contextuais desapareciam somente no
cristalino. Todos os quatro processos ficaram abaixo de 512 MiB e terminaram
em menos de 0,4 s.

## Implementação autorizada após confirmação ADR-0127

1. após cada realização P1406, introspectar a árvore realizada para localizar
   os novos blocos contextuais e preservar o snapshot paginado corrente;
2. expandir esses blocos antes do layout do candidato, sem acumular realizações
   entre iterações;
3. fazer `Content::map_content` atravessar `Outline.realized_body` pelo walker
   já pertencente à unidade;
4. exigir que o candidato entregue ao layout não contenha o bloco pendente;
5. ressellar hashes e repetir os oráculos focais e a matriz P1409 limitada.

## GREEN e veredito futuro

Após confirmação e implementação, repetir os dois oráculos nos dois binários e
exigir igualdade de texto extraído e ausência de erro. Em seguida repetir a
matriz P1409 limitada para medir quais rotas mudaram de estado, sem promover
`indented`/`inner` por inferência.

## Ataques e GREEN executados

Dois ataques impediram um falso positivo antes do resultado final:

1. mutação equivalente sem a nova fase — somente a travessia de
   `Outline.realized_body` — manteve ambos os oráculos RED;
2. nova fase sem rearmar a fronteira de bloco tornou o texto visível, mas
   colocou o heading seguinte acima dele; a comparação de ordem e coordenadas
   rejeitou esse candidato;
3. a fixture mista com um `#context [X]` anterior ao outline revelou que uma
   expansão indiscriminada reexecutava produtores antigos. A passagem foi
   restringida aos objetos causais nascidos durante a realização; o controle
   misto deve preservar `X` uma única vez.

Depois de fechar as linhas de entrada/saída e rearmar a cadeia normal de blocos,
o GREEN bilateral final foi executado às `2026-09-15T18:10:55-03:00` no baseline
`82e210bd1826ccbf0cd2378cddc605fb2546cf34` com working tree não commitada
composta pelos três prompts, três consumers, este passo e as três fixtures:

| Oráculo | Implementação | Texto extraído (`pdftotext -layout`) | Exit | RSS pico | stderr |
|---|---|---|---:|---:|---|
| `page` | cristalino | `Contents\n1\n\n1. H\n\f` | 0 | 42.456 KiB | vazio |
| `page` | vanila | `Contents\n1\n\n1. H\n\f` | 0 | 31.164 KiB | vazio |
| `prefix` | cristalino | `Contents\n1.\n\n1. H\n\f` | 0 | 41.960 KiB | vazio |
| `prefix` | vanila | `Contents\n1.\n\n1. H\n\f` | 0 | 30.108 KiB | vazio |
| misto | cristalino | `X\n\nContents\n1.\n\n1. H\n\f` | 0 | 44.028 KiB | vazio |
| misto | vanila | `X\n\nContents\n1.\n\n1. H\n\f` | 0 | 30.532 KiB | vazio |

Binário cristalino final: identificador administrativo
[dado administrativo removido].
As fixtures `page`, `prefix` e mista têm respectivamente identificador administrativo
[dado administrativo removido]
[dado administrativo removido] e
[dado administrativo removido].

## Reprobe P1409 final

Executado às `2026-09-15T18:11:29-03:00`, no mesmo estado não commitado, sob
512 MiB e uma thread. Recibo:
relatório consolidado da limpeza.

- resultado do runner: `PASS`;
- 84 processos; RSS máximo 42.928 KiB; swap máximo 0 KiB;
- `outline.entry.page` e `outline.entry.prefix`: `MATCH` por observáveis
  públicos bilaterais;
- `outline.entry.indented` e `outline.entry.inner`: continuam `UNKNOWN`, sem
  promoção inferida;
- `par.line` e `raw.line`: continuam `ABSENT`.

Não houve alegação de isolamento entre autoria da implementação e veredito; o
passo foi executado em rehearsal completo sem atestação de independência.

## Estado do gate

P1411 altera a sequência de fases ao executar contexto surgido depois da
realização de outline. Confirmação humana explícita recebida em 2026-09-15; a
implementação está autorizada. O veredito final continua condicionado ao GREEN
bilateral e aos limites de memória.

Condições satisfeitas: GREEN bilateral focal, matriz limitada `PASS`, zero swap,
build release limitado concluído e hashes L0 ressellados. P1411 está pronto
para commit; as lacunas de regras de estilo de `indented`/`inner` permanecem
fora do escopo.
