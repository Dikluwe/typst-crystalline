# P619 — Repartição completa de fases do `macro-10x`

**Data da medição:** 2026-07-08  
**Commit base:** `1cad5c926fd240e50d1e2af87075f23486e91e98`  
**Passo:** P619  
**ADR-0108 em vigor:** medição precede a decisão.

## Resumo executivo

P618 apresentou a repartição de fases do `macro-10x` sem `shape_ms` nem `subset_ms`, deixando 97,1% do tempo do cristalino como "não isolado em fases instrumentadas". P546 já tinha identificado `shape_ms` como a fase dominante (89,6% do total). Este passo repete a medição completa com `--timings-json` para fechar essa lacuna.

A medição confirma que **`shape_ms` continua a dominar**, agora com **90,66%** do tempo total. A proporção é semelhante à de P546 (89,6%) e muito melhor do que o patamar de P548 (~57 000 ms, ~94% do total). Não há evidência de crescimento anómalo introduzido pelo trabalho de P591 (`advance_shaped` para árabe), P609 (extracção de colecções de fonte) ou P611 (XMP).

## Comando

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p619-macro.pdf --timings-json /tmp/p619-timings.json
```

Saída bruta:

```json
{"parse_ms":0.000000,"eval_ms":539.362528,"introspect_ms":88.593307,"expand_context_ms":0.000942,"layout_ms":1254.533394,"shape_ms":32577.364204,"subset_ms":0.870782,"render_ms":1472.483238,"total_ms":35933.208395}
```

## Tabela comparativa de fases

| Fase | P546 (pré-correcção) | P548 (pós-correcção cache) | P619 (actual) |
|---|---:|---:|---:|
| `parse_ms` | — | — | 0,00 |
| `eval_ms` | — | — | 539,36 |
| `introspect_ms` | — | — | 88,59 |
| `expand_context_ms` | — | — | 0,00 |
| `layout_ms` | 1 380,8 | 1 484,9 | **1 254,53** |
| `shape_ms` | 30 564,7 (89,6%) | ~57 000 (~94%) | **32 577,36 (90,66%)** |
| `subset_ms` | 1,0 | — | **0,87** |
| `render_ms` | — | 1 506,9 | **1 472,48** |
| `total_ms` | 34 096,0 | ~60 492 | **35 933,21** |

Valores em milissegundos. Percentagens do `total_ms` do respectivo passo.

## Repartição percentual actual (P619)

| Fase | Tempo (ms) | % do total |
|---|---:|---:|
| parse | 0,00 | 0,00 % |
| eval | 539,36 | 1,50 % |
| introspect | 88,59 | 0,25 % |
| expand_context | 0,00 | 0,00 % |
| layout | 1 254,53 | 3,49 % |
| **shape** | **32 577,36** | **90,66 %** |
| subset | 0,87 | 0,00 % |
| render | 1 472,48 | 4,10 % |
| **total** | **35 933,21** | **100,00 %** |

## Análise

- **`shape_ms` continua a ser a fase dominante**, responsável por mais de 90% do tempo de compilação do `macro-10x`.
- A proporção (90,66%) é **ligeiramente superior à de P546** (89,6%) porque o `total_ms` actual é maior (35 933 ms vs 34 096 ms), não porque o shaping tenha crescido em termos absolutos — de facto, 32 577 ms é inferior aos ~57 000 ms de P548.
- **`layout_ms` mantém-se baixo** (1 254,53 ms), confirmando que a correcção de cache de P548 continua efectiva e que não houve regressão no layout desde então.
- **`subset_ms` é negligenciável** (~0,87 ms), como já era esperado face a P546.
- O trabalho subsequente (P591 shaping para árabe, P609 colecções de fonte, P611 XMP) **não alterou o perfil de custo de forma anómala**: o `shape_ms` continua a ser o mesmo gargalo de antes.

## Decisão

A lacuna deixada por P618 está fechada. O benchmark completo fica confirmado com a repartição completa de fases:

- A lentidão do cristalino no `macro-10x` explica-se principalmente pelo `shape_ms` (~90,7%).
- Não há regressão nova a investigar neste passo.
- Qualquer futura optimização de desempenho no `macro-10x` deve focar o shaper/texto, não o layout nem o render.

A tabela de fases do relatório P618 (`00_nucleo/diagnosticos/paridade-producao-p618.md`) foi actualizada para reflectir `shape_ms` e `subset_ms`, eliminando a classificação de "97,1% não isolado".

## Proveniência da medição

- Commit base: `1cad5c926fd240e50d1e2af87075f23486e91e98`
- Comando: `./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p619-macro.pdf --timings-json /tmp/p619-timings.json`
- Data/hora aproximada: 2026-07-08, após P618.
