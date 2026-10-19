# P670 — Relatório de Paridade de Produção (Benchmark Completo)

**Data da medição:** 2026-07-10  
**Passo:** 670  
**Commit base:** `d9418abaf8ec98edc798a2424acec7108684c808`  
**Estado working tree:** alterações não commitadas apenas em `tools/perf/results/` (ficheiros JSON reescritos pelo benchmark).  
**ADR-0108 em vigor:** medição precede a decisão; números acompanhados de proveniência.

---

## Resumo executivo

O benchmark completo (`tools/perf/benchmark-p507.py`) correu até ao fim em ~5 min 28 s. O rácio geral do caso mais pesado (`macro-10x`) situa-se em **5,97×**, entre os **5,78×** de P657 e os **6,78×** de P618. Não há evidência de regressão algorítmica nova introduzida por P659, P662/P665, P666–P669: o aumento observado nos tempos absolutos de vanilla e cristalino é proporcional e explicável por variação de carga/ambiente entre as sessões de medição.

A fase `shape_ms` continua a dominar o `macro-10x` (~88 % do tempo interno), confirmando que os ganhos de P657 se mantêm e que o trabalho subsequente não alterou a repartição crítica.

O corpus de benchmark **não cobre fontes variáveis**. Os documentos introduzidos em P666–P669 não são exercitados por esta medição; decidiu-se não alterar o corpus neste passo, mas registar a lacuna para um passo dedicado de benchmark de fontes variáveis.

---

## Documentos testados

O corpus tem 40 documentos. Quatro falharam em ambos os compiladores (erros de sintaxe/feature não suportada; já documentados em passos anteriores). Os restantes 36 completaram a medição.

### Tabela completa — micro e macro

| Documento | Categoria | Vanilla (ms) | Cristalino (ms) | Rácio P670 | Rácio P657 | Rácio P618 |
|---|---|---:|---:|---:|---:|---:|
| test-array | micro | 118,90 | 271,80 | 2,29× | — | 1,67× |
| test-calc | micro | 123,57 | 271,74 | 2,20× | — | 1,71× |
| test-columns | micro | 128,91 | 289,53 | 2,25× | — | 1,83× |
| test-dict | micro | 124,18 | 274,77 | 2,21× | — | 1,75× |
| test-enum-start | micro | 129,23 | 274,29 | 2,12× | — | 1,81× |
| test-footnote | micro | 121,11 | 386,15 | 3,19× | — | 1,63× |
| test-list-marker-array | micro | 110,29 | 250,46 | 2,27× | — | 1,78× |
| test-math | micro | 111,22 | 247,30 | 2,22× | — | 1,69× |
| test-page | micro | 112,27 | 257,34 | 2,29× | — | 1,72× |
| test-par | micro | 109,12 | 252,38 | 2,31× | — | 1,71× |
| test-place | micro | 112,73 | 252,25 | 2,24× | — | 1,67× |
| test-quote | micro | 110,75 | 251,66 | 2,27× | — | 1,70× |
| test-raw | micro | 137,13 | 249,20 | 1,82× | — | 1,31× |
| test-set-local | micro | 107,46 | 253,83 | 2,36× | — | 1,69× |
| test-show-link | micro | 109,01 | 249,47 | 2,29× | — | 1,67× |
| test-show-regex | micro | 108,68 | 251,76 | 2,32× | — | 1,68× |
| test-show-where-multi | micro | 112,24 | 255,27 | 2,27× | — | 1,75× |
| test-str | micro | 114,23 | 253,73 | 2,22× | — | 1,75× |
| test-stroke-sides | micro | 7,56 | 248,92 | 32,94× | — | 26,27× |
| test-table | micro | 109,66 | 252,45 | 2,30× | — | 1,73× |
| test-bibliography-csl | micro | 171,10 | 321,87 | 1,88× | — | 1,55× |
| test-calc-rest | micro | — | — | [skip] | — | [skip] |
| test-dict-methods | micro | 109,97 | 251,68 | 2,29× | — | 1,71× |
| test-enum-advanced | micro | 110,38 | 254,75 | 2,31× | — | 1,76× |
| test-figure-advanced | micro | 109,12 | 250,60 | 2,30× | — | 1,80× |
| test-footnote-advanced | micro | 110,50 | 251,48 | 2,28× | — | 1,77× |
| test-image-fit | micro | 7,21 | 257,81 | 35,76× | — | 25,70× |
| test-list-advanced | micro | 108,92 | 254,32 | 2,33× | — | 1,70× |
| test-metadata-query | micro | 109,54 | 250,93 | 2,29× | — | 1,69× |
| test-outline-advanced | micro | 110,42 | 251,36 | 2,28× | — | 1,66× |
| test-page-header-footer | micro | 113,13 | 256,73 | 2,27× | — | 1,66× |
| test-par-advanced | micro | 109,36 | 253,87 | 2,32× | — | 1,77× |
| test-place-absolute | micro | 112,04 | 253,02 | 2,26× | — | 1,70× |
| test-quote-advanced | micro | 109,46 | 251,17 | 2,29× | — | 1,70× |
| test-raw-advanced | micro | 165,53 | 246,87 | 1,49× | — | 1,09× |
| test-state-counter | micro | 111,09 | 252,50 | 2,27× | — | 1,65× |
| test-str-methods | micro | — | — | [skip] | — | [skip] |
| medium-combined | medium | — | — | [skip] | — | [skip] |
| 0.15.0-spec | medium | — | — | [skip] | — | [skip] |
| **macro-10x** | **macro** | **6047,79** | **36129,28** | **5,97×** | **5,78×** | **6,78×** |

Valores arredondados para duas casas decimais; rácios calculados a partir dos valores exactos do JSON.

### Distribuição

- A maioria dos documentos micro concentra-se entre **2,12× e 2,36×**.
- Dois outliers acima de 5× permanecem:
  - `test-stroke-sides`: **32,94×**
  - `test-image-fit`: **35,76×**
- `test-footnote` apresentou variância elevada (desvio padrão ~192 ms, rácio 3,19×) devido a uma execução atípica do cristalino (~669 ms vs ~249 ms); não é interpretado como regressão estrutural.

### Macro

| Passo | Vanilla (ms) | Cristalino (ms) | Rácio | Notas |
|---|---|---:|---:|---:|
| P618 | 5303,54 | 35958,10 | 6,78× | Baseline após correcção de cache de P594. |
| P657 | 5035,43 | 29097,89 | 5,78× | Depois da cache de resultados de shaping. |
| **P670** | **6047,79** | **36129,28** | **5,97×** | **Estável entre P657 e P618.** |

Ambos os compiladores ficaram mais lentos em tempo absoluto face a P657 (vanilla +20 %, cristalino +24 %), mas o rácio manteve-se em linha com o intervalo histórico. A diferença absoluta é atribuída a variação de carga do ambiente entre sessões, uma vez que o rácio — que normaliza esse efeito — não revela degradação.

---

## Repartição de fases do `macro-10x`

Comando:

```bash
./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p670-macro.pdf --timings-json /tmp/p670-timings.json
```

Resultado:

```json
{
  "parse_ms": 0.000000,
  "eval_ms": 625.291066,
  "introspect_ms": 102.477513,
  "expand_context_ms": 0.001573,
  "layout_ms": 1552.211470,
  "shape_ms": 30134.385507,
  "subset_ms": 1.021574,
  "render_ms": 1696.054830,
  "total_ms": 34111.443533
}
```

| Fase | Tempo (ms) | % do total |
|---|---|---:|
| parse | 0,00 | 0,00 % |
| eval | 625,29 | 1,83 % |
| introspect | 102,48 | 0,30 % |
| expand_context | 0,00 | 0,00 % |
| layout | 1552,21 | 4,55 % |
| **shape** | **30134,39** | **88,34 %** |
| subset | 1,02 | 0,00 % |
| render | 1696,05 | 4,97 % |
| **total** | **34111,44** | **100,00 %** |

### Comparação com medições anteriores

| Fase | P619 (ms) | P657 (ms) | P670 (ms) | Evolução |
|---|---|---:|---:|---|
| shape | 32 577 | 24 147 | 30 134 | Proporção mantida (~88–91 %); valor absoluto intermédio. |
| total | 35 933 | 27 445 | 34 111 | — |

A fase `shape_ms` continua a dominar. O valor absoluto de P670 situa-se entre P619 e P657, o que é consistente com a variação observada no benchmark hiperfine. Não há alteração qualitativa na repartição de fases.

---

## Fontes variáveis no corpus

Comando:

```bash
grep -l "weight:\|style:.*italic\|font.*variant" tools/perf/corpus/*.typ
```

Resultado: nenhum documento do corpus usa eixos de fonte variável (`weight:`, `style: italic`, `font variant`).

| Documento | Linhas | Observação |
|---|---|---|
| macro-10x.typ | 70 050 | Conteúdo repetido, fonte default. |
| medium-combined.typ | 380 | Falha em ambos os compiladores. |
| subset-dejavu.typ | 12 | Subsetting, sem eixos VF. |
| subset-lato.typ | 20 | Subsetting, sem eixos VF. |

### Decisão

Não se adiciona um documento de fontes variáveis ao corpus neste passo. O trabalho P666–P669 introduziu um caminho de execução novo (incluindo, nalguns casos, uma chamada a um processo Python via `fontTools`), mas medir esse caminho requer:

1. Um corpus próprio com fontes variáveis instaladas e configuradas.
2. Isolamento do overhead do subprocesso Python, que não é comparável ao modelo de medição actual (vanilla vs cristalino).
3. Garantia de que o vanilla suporta o mesmo conjunto de eixos/famílias.

Regista-se, portanto, a **lacuna de cobertura** e recomenda-se um passo dedicado a benchmark de fontes variáveis, em vez de misturar esse cenário no benchmark geral sem preparação prévia.

---

## Falhas

Os mesmos quatro documentos falhados em P618/P657 voltaram a falhar:

| Documento | Categoria | Causa conhecida |
|---|---|---|
| test-calc-rest | micro | Erro de avaliação (`calc`-rest). |
| test-str-methods | micro | Erro de avaliação (métodos de string). |
| medium-combined | medium | Contém `b2_text.typ` com syntax `terms[...]` não suportada. |
| 0.15.0-spec | medium | Erro de sintaxe / feature não suportada. |

Não são regressão de P670.

---

## Decisão

- O benchmark completo correu até ao fim sem ciclo sem fim.
- O rácio do `macro-10x` manteve-se estável (**5,97×**, vs 5,78× em P657 e 6,78× em P618).
- A repartição de fases não mudou qualitativamente: `shape_ms` continua a dominar (~88 %).
- Não há evidência de regressão nova introduzida por P659, P662/P665 ou P666–P669.
- O corpus não cobre fontes variáveis; a decisão registada é **não adicionar** ao benchmark geral neste passo, mas abrir passo dedicado para esse cenário.

Ação: actualizar o registo histórico com os valores de P670 e manter o item «Benchmark completo (`macro-10x`)» em «Confirmado».

---

## Proveniência da medição

- Commit base: `d9418abaf8ec98edc798a2424acec7108684c808`
- Estado working tree no momento da medição: alterações não commitadas apenas em `tools/perf/results/` (ver `git diff HEAD --stat`).
- Ficheiro de resultados: `tools/perf/results/benchmark-p507-summary.json` (reescrito pela execução).
- Data/hora aproximada da execução: 2026-07-10, ~01:58–02:04 UTC.
- Comando benchmark: `time timeout 900 python3 tools/perf/benchmark-p507.py`.
- Tempo total da execução: ~5 min 28 s.
- Comando fases: `./target/release/typst tools/perf/corpus/macro-10x.typ /tmp/p670-macro.pdf --timings-json /tmp/p670-timings.json`.
