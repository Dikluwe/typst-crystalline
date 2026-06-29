# Diagnóstico de Paridade Funcional — P490

> **Passo:** 490
> **Data:** 2026-06-29
> **Metodologia:** Bateria de 20 ficheiros `.typ` testados contra vanilla 0.14.2 (`typst query`) e cristalino (`query_to_summary` + `typst-wiring` compilação).
> **ADR referência:** ADR-0075 (comparação via `typst query --format json`), ADR-0054 (graded parity).

---

## Tabela de Resultados

| Ficheiro | Selector | Vanilla | Cristalino | Classificação | Notas |
|----------|----------|---------|------------|---------------|-------|
| test-list-marker-array.typ | `list` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** | `list` não é selector no cristalino; `marker:Array` scope-out P470 |
| test-enum-start.typ | `enum` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** | `enum` não é selector no cristalino; `start:` + `(a)` gap real |
| test-par.typ | `par` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** | `par` não é selector no cristalino; `leading/spacing/justify` implementados |
| test-show-link.typ | `link` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** | `link` não é selector no cristalino; `#show link:` funciona (P452/P463) |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | **DIFF** | `table.header/footer` — field access em função não suportado |
| test-raw.typ | `raw` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** | `raw` não é selector; compilação: raw com `lang` funciona |
| test-quote.typ | `quote` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** | `quote` não é selector; `attribution` funciona (P155) |
| test-footnote.typ | `footnote` | ok(1) | ERRO_DESCRITIVO | **AUSENTE** | `footnote` não é selector; body no rodapé funciona (P304/P305) |
| test-page.typ | `page` | ERRO_VAN (not locatable) | ok(0) | **MATCH** | Ambos não retornam itens (não locatable / sem headings) |
| test-place.typ | `place` | ERRO_VAN (not locatable) | ok(0) | **MATCH** | `place(top+right)` não causa erro no cristalino |
| test-calc.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `calc.log(base:)` e `calc.round(digits:)` — args nomeados não suportados |
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `arr.dedup()` — field access em array não suportado no cristalino |
| test-str.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `str(255, base: 16)` — arg `base:` não suportado em `str()` |
| test-dict.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `d.at("z", default: 99)` — campo `at` não existe (arg `default:` ausente) |
| test-show-regex.typ | `heading` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `text` não reconhecido em contexto de show-regex (P393/P473 gap residual) |
| test-set-local.typ | `heading` | ok(0) | ok(0) | **MATCH** | `#set` local em `#[...]` funciona (sem heading no ficheiro) |
| test-show-where-multi.typ | `heading` | ok(1) | ERRO_DESCRITIVO | **DIFF** | `heading.where(multi-field)` scope-out P474; erro descritivo (não PANIC) ✓ |
| test-math.typ | `math.equation` | ok(5) | ok(5) | **MATCH** | `vec`, `mat`, `cases`, `integral`, `sum` — 5/5 equações ✓ |
| test-stroke-sides.typ | `heading` | ok(0) | ERRO_DESCRITIVO | **DIFF** | `red`, `blue`, `green` — variáveis de cor não reconhecidas em `stroke:dict` |
| test-columns.typ | `heading` | ok(0) | ok(0) | **MATCH** | `columns:2` + `colbreak()` sem heading — sem PANIC ✓ |

---

## Resumo por Classificação

| Classificação | Contagem | Ficheiros |
|--------------|----------|-----------|
| **MATCH** | 5 | test-page, test-place, test-set-local, test-math, test-columns |
| **DIFF** | 9 | test-table, test-calc, test-array, test-str, test-dict, test-show-regex, test-show-where-multi, test-stroke-sides |
| **AUSENTE** (selector não registado) | 6 | test-list, test-enum, test-par, test-show-link, test-raw, test-quote, test-footnote |
| **PANIC** | **0** | — |

---

## PANICs — Critério Estrito

**0 PANICs** detectados em todos os 20 ficheiros. Critério P490 cumprido.

---

## Análise dos DIFFs

### Grupo D1 — Selectors não registados no cristalino (AUSENTE)

O cristalino expõe apenas: `heading`, `figure`, `citation`, `metadata`, `state`, `state_update`, `outline`, `bibliography`, `equation`, `counter_update`.

Seletores presentes no vanilla mas ausentes no cristalino: `list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`.

**Avaliação:** Gap real de `parse_selector`. Funcionalidades subjacentes podem estar implementadas (links, footnotes, quotes funcionam em compilação), mas o mecanismo de query não as expõe. Gap de M-size para P491+.

### Grupo D2 — Args nomeados ausentes

| Caso | Gap específico |
|------|---------------|
| `calc.log(100, base: 10)` | Arg `base:` não suportado em `calc.log` |
| `calc.round(3.567, digits: 2)` | Arg `digits:` não suportado em `calc.round` |
| `str(255, base: 16)` | Arg `base:` não suportado em `str()` constructor |
| `d.at("z", default: 99)` | Arg `default:` não suportado em `dict.at()` |

**Avaliação:** Gaps de S-size, isolados, materializáveis em P491+ como grupo coerente.

### Grupo D3 — Field access em coleções

| Caso | Gap específico |
|------|---------------|
| `arr.dedup()` | Field access `dedup` não suportado em array |
| `table.header[...]` | Field access em `table` função não suportado |

**Avaliação:** `arr.dedup()` (e provavelmente `arr.chunks()`, `arr.windows()`) — gap residual pós-P466. `table.header` é sintaxe especial da stdlib Typst. Gap M-size para P491+.

### Grupo D4 — Variáveis de cor em contexto inline

`red`, `blue`, `green`, `none` como variáveis predefinidas não reconhecidas em `stroke: (left: 3pt + red, ...)`.

**Avaliação:** Provável scope de disponibilidade de variáveis de cor. Gap S-size.

### Grupo D5 — show-regex com `text()` 

`text` não reconhecido em `#show regex(...): it => text(red, it)`. Gap provavelmente relacionado com D4 (variáveis de cor) ou com o pipe de show-rule.

**Avaliação:** Gap S-size adjacente ao D4.

---

## Scope-outs Confirmados

| Funcionalidade | Status |
|----------------|--------|
| `show heading.where(level:1, outlined: true)` multi-field | scope-out P474 — ERRO_DESCRITIVO (não PANIC) ✓ |
| `list.marker: Array` | scope-out P470 — gap documentado |

---

## Lista Priorizada de Gaps para P491+

### Prioridade Alta (M-size, impacto de usabilidade alto)
1. **Selectors ausentes** (`list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`) — gap do `parse_selector` que impede instrospection de elementos comuns.
2. **`table.header` / `table.footer`** — sintaxe de tabela avançada não suportada.

### Prioridade Média (S-size, args nomeados)
3. **Args nomeados em `calc`** (`base:`, `digits:`) — `calc.log`, `calc.round`.
4. **Args nomeados em `str()`** (`base:`) — conversão de base.
5. **`dict.at(default:)`** — fallback em dicionários.

### Prioridade Média (S-size, métodos de array)
6. **`arr.dedup()`, `arr.chunks(n)`, `arr.windows(n)`** — métodos pós-P466 não materializados.

### Prioridade Baixa (S-size, variáveis de cor / show-rule)
7. **Variáveis de cor predefinidas** (`red`, `blue`, `green`) em `stroke: dict` inline.
8. **`text()` em show-rule com regex** — ligação entre show-regex e `text()` variável.

---

## Critérios de Fecho P490

- [x] Todos os 20 ficheiros executados contra vanilla 0.14.2.
- [x] Todos os 20 ficheiros executados contra cristalino (query + compilação).
- [x] Tabela de resultados com classificação para cada teste.
- [x] **PANICs: 0** (critério estrito cumprido).
- [x] DIFFs avaliados: 9 gaps reais identificados e categorizados.
- [x] AUSENTEs avaliados: 6 selectors não registados (gap de `parse_selector`).
- [x] `00_nucleo/diagnosticos/paridade-funcional-p490.md` produzido.
- [x] Lista priorizada de gaps para P491+ produzida.

---

## Sentinela P490

Adicionado em `lab/parity/tests/structural_parity.rs`:
- `fn p490_bateria_paridade_funcional_20_ficheiros()` — verifica 0 PANICs.
- Resultado: **2 tests passed; 0 failed** (inclui sentinela P488 remanescente).

---

## Próximo Passo (P491)

**Sem PANICs → não é P491 bug-fix.**

Recomendação: P491 = **materialização do grupo D2 (args nomeados)** como conjunto coerente S-size:
- `calc.log(base:)`, `calc.round(digits:)`, `str(base:)`, `dict.at(default:)`.

Alternativa: P491 = **expansão do `parse_selector`** para incluir `list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote` (M-size).
