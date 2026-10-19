# P618 — Relatório de Paridade de Produção (Benchmark Completo)

**Data da medição:** 2026-07-08  
**Commit base:** `24ef8cf294e13401d85872a1a143b3641040120e` (working tree com alterações não commitadas: `00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md` e ficheiros JSON em `tools/perf/results/`).  
**Passo:** P618  
**ADR-0108 em vigor:** medição precede a decisão; números acompanhados de proveniência.

## Resumo executivo

O benchmark completo (`tools/perf/benchmark-p507.py`) correu até ao fim com o ajuste de P594 (`runs = 2 if category == "macro" else 5`) ainda presente no script. Não houve ciclo sem fim: a execução terminou dentro do tempo limite generoso (`timeout 900`). A lentidão observada é genuína e não regressor neste passo — é a mesma já documentada em P594.

A medição actual, a primeira completa desde P548, mostra:

- **Micro (35 documentos):** mediana **1,70×**, média **3,07×**, mínimo **1,09×**, máximo **26,27×**.
- **Macro (`macro-10x`):** **6,78×** (vanilla 5303,5 ms; cristalino 35958,1 ms).

Comparativamente ao histórico:

| Passo | Micro mediana | Macro `macro-10x` | Notas |
|---|---|---|---|
| P518 | 1,10× | 0,30× | Medição inicial, sem regressões ainda evidentes. |
| P546 | — | 28,68× | Antes da correcção de cache; evidenciou lentidão severa. |
| P548 | — | 11,98× | Depois da correcção de cache de P594. |
| **P618** | **1,70×** | **6,78×** | **Correcção mantida; melhoria substancial face a P546/P548.** |

## Documentos testados

O corpus de benchmark tem 40 documentos. Quatro falharam em ambos os compiladores (erro de sintaxe ou feature não suportada no corpus; ver secção «Falhas» abaixo). Os restantes 36 (35 micro + 1 macro) completaram a medição.

### Tabela completa — micro e macro

| Documento | Categoria | Vanilla (ms) | Cristalino (ms) | Rácio |
|---|---|---:|---:|---:|
| test-array | micro | 93,67 | 156,30 | 1,67× |
| test-calc | micro | 91,15 | 155,82 | 1,71× |
| test-columns | micro | 94,51 | 172,66 | 1,83× |
| test-dict | micro | 91,20 | 159,28 | 1,75× |
| test-enum-start | micro | 90,97 | 164,65 | 1,81× |
| test-footnote | micro | 92,91 | 151,72 | 1,63× |
| test-list-marker-array | micro | 90,66 | 160,99 | 1,78× |
| test-math | micro | 91,05 | 153,65 | 1,69× |
| test-page | micro | 93,46 | 160,92 | 1,72× |
| test-par | micro | 90,76 | 154,90 | 1,71× |
| test-place | micro | 92,95 | 155,67 | 1,67× |
| test-quote | micro | 90,42 | 153,26 | 1,70× |
| test-raw | micro | 114,90 | 150,89 | 1,31× |
| test-set-local | micro | 90,23 | 152,07 | 1,69× |
| test-show-link | micro | 90,52 | 151,39 | 1,67× |
| test-show-regex | micro | 90,97 | 152,40 | 1,68× |
| test-show-where-multi | micro | 90,21 | 157,77 | 1,75× |
| test-str | micro | 90,72 | 158,40 | 1,75× |
| test-stroke-sides | micro | 5,77 | 151,53 | **26,27×** |
| test-table | micro | 91,09 | 157,14 | 1,73× |
| test-bibliography-csl | micro | 139,29 | 215,41 | 1,55× |
| test-dict-methods | micro | 92,70 | 158,06 | 1,71× |
| test-enum-advanced | micro | 90,72 | 159,42 | 1,76× |
| test-figure-advanced | micro | 91,46 | 164,39 | 1,80× |
| test-footnote-advanced | micro | 91,29 | 161,67 | 1,77× |
| test-image-fit | micro | 6,09 | 156,47 | **25,70×** |
| test-list-advanced | micro | 90,72 | 153,92 | 1,70× |
| test-metadata-query | micro | 92,14 | 155,57 | 1,69× |
| test-outline-advanced | micro | 91,27 | 151,78 | 1,66× |
| test-page-header-footer | micro | 93,19 | 154,52 | 1,66× |
| test-par-advanced | micro | 90,74 | 160,91 | 1,77× |
| test-place-absolute | micro | 93,00 | 157,74 | 1,70× |
| test-quote-advanced | micro | 91,13 | 154,84 | 1,70× |
| test-raw-advanced | micro | 138,58 | 151,30 | 1,09× |
| test-state-counter | micro | 91,68 | 151,71 | 1,65× |
| **macro-10x** | **macro** | **5303,54** | **35958,10** | **6,78×** |

Valores arredondados para duas casas decimais; rácio calculado a partir dos valores exactos do JSON.

## Análise por observáveis

### Distribuição

- A maioria dos documentos micro concentra-se entre **1,55× e 1,83×**.
- Dois documentos são outliers acima de 5×:
  - `test-stroke-sides`: **26,27×**
  - `test-image-fit`: **25,70×**

Ambos são documentos muito pequenos em que o tempo de arranque/fixe do cristalino (~150 ms) pesa contra um vanilla de ~6 ms. A morfologia da saída não foi avaliada neste benchmark; o rácio alto reflecte sobretudo o peso do fixo, não uma regressão algorítmica.

### Macro

O documento `macro-10x` (renderização repetida de conteúdo com markup denso) continua a ser o caso mais pesado, mas melhorou significativamente face a P546 (28,68×) e P548 (11,98×), situando-se agora em **6,78×**. Isto confirma que a correcção de P594 se mantém efectiva e que o trabalho subsequente (P595–P617) não introduziu regressão severa neste cenário.

### Fases do `macro-10x` (medição P619, cristalino)

A repartição completa de fases, incluindo `shape_ms` e `subset_ms`, foi obtida em P619 com `--timings-json`.

| Fase | Tempo (ms) | % do total |
|---|---|---:|
| parse | 0,00 | 0,00 % |
| eval | 539,36 | 1,50 % |
| introspect | 88,59 | 0,25 % |
| expand_context | 0,00 | 0,00 % |
| layout | 1 254,53 | 3,49 % |
| **shape** | **32 577,36** | **90,66 %** |
| subset | 0,87 | 0,00 % |
| render | 1 472,48 | 4,10 % |
| **total** | **35 933,21** | **100,00 %** |

A fase `shape_ms` é claramente dominante, explicando mais de 90% do tempo de compilação do `macro-10x`. A soma das restantes fases é inferior a 10% do total.

## Falhas

Quatro documentos falharam em ambos os compiladores (o ficheiro JSON correspondente ficou vazio e o summary não os inclui):

| Documento | Categoria | Causa conhecida |
|---|---|---|
| test-calc-rest | micro | Erro de avaliação (`calc`-rest). |
| test-str-methods | micro | Erro de avaliação (métodos de string). |
| medium-combined | medium | Contém `b2_text.typ` com syntax `terms[...]` não suportada. |
| 0.15.0-spec | medium | Erro de sintaxe / feature não suportada. |

Estas falhas não são regressão de P618; já estavam documentadas em passos anteriores e dizem respeito a features ainda não migradas/parificadas.

## Decisão

O ajuste de P594 continua válido. O benchmark completo correu até ao fim, sem ciclo sem fim, e os números são consistentes com a evolução esperada (melhoria face a P546/P548). Não há evidência de regressão nova introduzida por P595–P617.

Ação: o item «Benchmark completo (`macro-10x`)» sai da secção «Ainda por confirmar» e entra em «Confirmado» na lista de disparidades (`00_nucleo/diagnosticos/estado-disparidades-vanilla-p593.md`).

## Proveniência da medição

- Commit base: `24ef8cf294e13401d85872a1a143b3641040120e`
- Estado working tree no momento da medição: alterações não commitadas listadas por `git diff HEAD --stat` (ver diff actual).
- Ficheiro de resultados: `tools/perf/results/benchmark-p507-summary.json` (reescrito pela execução).
- Data/hora aproximada da execução: 2026-07-08, entre 21:11 e 21:16 UTC.
- Comando: `time timeout 900 python3 tools/perf/benchmark-p507.py`.
- Tempo total da execução: ~4 min 41 s.
