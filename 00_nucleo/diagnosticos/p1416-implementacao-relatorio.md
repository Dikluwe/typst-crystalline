# P1416 — relatório de implementação e verificação

## Veredito e proveniência

**PASS no fragmento uniforme congelado.** O resultado não declara paridade
geral de `par.line` em colunas.

- predecessor causal: `f4537577af3daba48cb880d7300e2e986f47702c`;
- medição final: `2026-09-16T08:09:43-03:00`;
- estado medido: working tree não commitada contendo somente P1416, conforme
  `git diff HEAD --stat` e arquivos novos listados no commit posterior;
- vanila ratificado: `586e1bd43fae6c9a973218163d3165c53ab8d16d`;
- binário cristalino final: SHA-256
  `6b72024dd05a92b8baa7730506fc99d0ab16de93ee97c808512a01dbc919e871`;
- regime: protocolo Tekt completo em rehearsal, executado sem atestação de
  isolamento; obrigação, contrato, implementação e verificação partilharam
  workspace/contexto, e não houve campanha de mutação independente formal.

## RED e decisão

Sob 512 MiB, uma thread e timeout de 15 s, o baseline cristalino somou as
linhas de regiões irmãs: `1 before / 2 left + 3 right / 4 after`. O vanila
contou o ordinal transversal uma vez:
`1 before / 2 left + right / 3 after`.

Ataques com três colunas, primeira coluna vazia, comprimentos 2×3, callback e
margem direita demonstraram que, para métricas uniformes, o progresso é o
máximo de ordinais das colunas e a ocorrência fica no bordo externo do
contentor. A fixture 18pt×8pt refutou a generalização por ordinal e foi
congelada como `Unknown`.

## Materialização

- `columns.rs` restaura o snapshot lógico de contador/página antes de cada
  região irmã de `#columns()` segmentado;
- um máximo escalar suprime somente o prefixo já emitido e conserva a cauda
  nova da coluna mais longa;
- Cursor suprimido não cria pattern, callback, item ou fixup;
- compensações escalares movem ocorrências novas ao bordo externo inicial ou
  final antes da translação normal da coluna;
- colunas de página P1415 e fluxos sem `colbreak()` não entram no mecanismo;
- o contexto privado participa de checkpoint/rollback e não guarda linhas,
  frames, páginas ou conteúdo.

## Gates executados

| Gate | Resultado |
|---|---|
| duas colunas | MATCH: `2 left + right`, depois `3 after` |
| três colunas | MATCH: uma ocorrência interna |
| primeira coluna vazia | MATCH: número no bordo externo inicial |
| comprimentos 2×3 | MATCH: ordinais internos `2,3,4` |
| callback | MATCH: `L2` único no contentor e `L3 after` |
| margem direita | MATCH: ocorrência única no bordo final |
| colunas de página | MATCH; P1415 preservado |
| alturas 18pt×8pt | `Unknown`, divergência preservada e não promovida |
| repetição em ordem inversa | PASS byte a byte no texto extraído |
| regressões P1414 | cinco válidos PASS |
| regressões P1415 | quatro válidos PASS; aridade exit 1 exato |
| `cargo check -p typst-core -j1` | PASS sob 2 GiB |
| build release definitivo | PASS em 3 min 12 s sob 2 GiB/um job |

O primeiro build release foi interrompido externamente com exit 143 antes do
link, sem diagnóstico de alocação. A repetição sob o mesmo teto, sem aumentar
memória, concluiu; o build definitivo posterior ao resselo também concluiu.
O maior RSS focal final foi 42.824 KiB, swap zero, sem OOM ou timeout.

## Limites

Métricas verticais diferentes entre segmentos, RTL, `#columns()` contínuo sem
`colbreak()`, columns aninhadas e estilos locais divergentes permanecem
`Unknown`. A ausência de isolamento de capacidades e de mutation score formal
impede alegar materialização segregada atestada. O veredito cobre somente as
oito fixtures congeladas e os controles P1414/P1415 executados.
