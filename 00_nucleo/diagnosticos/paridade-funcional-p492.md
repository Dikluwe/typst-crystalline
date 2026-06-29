# Diagnóstico de Paridade Funcional — P492

> **Passo:** 492
> **Data:** 2026-06-29
> **Foco:** Fechar os gaps D4/D5 de variáveis de cor predefinidas e `text(...)` global identificados em P490.
> **Metodologia:** Implementação em `01_core/` + testes unitários + re-execução da bateria P490 (20 ficheiros).
> **ADR referência:** ADR-0075, ADR-0054 (graded parity).

---

## Resultados por sub-tarefa D4/D5

| Sub-tarefa | Vanilla | Cristalino pós-P492 | Classificação | Notas |
|---|---|---|---|---|
| 492a Variáveis de cor globais (`red`, `blue`, `green`, `black`, `white`, `yellow`, `cyan`, `magenta`, `none`) | `red` → cor | `red` → cor | **MATCH** | Injetadas no scope global em `eval::global_scope`; `none` mapeado para `Value::None` |
| 492b Operador `Length + Color` / `Color + Length` | `3pt + red` → stroke | `3pt + red` → stroke | **MATCH** | `Value::Stroke` criado em `operators.rs`; usado por `stroke: (top: 3pt + red)` |
| 492c `text(fill: red, it)` / `text(red, it)` | estilo aplicado | estilo aplicado | **MATCH** | `native_text` aceita fill posicional por tipo e fill nomeado |
| 492d `#show regex(...): it => text(red, it)` | regex show funciona | regex show funciona | **MATCH** | Combina `regex` (P393) com `text(...)` global |

---

## Não-regressão — bateria P490 (20 ficheiros)

| Ficheiro | Selector | Vanilla | Cristalino pré-P492 | Cristalino pós-P492 | Δ |
|---|---|---|---|---|---|
| test-stroke-sides.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-show-regex.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) |
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) |
| test-show-where-multi.typ | `heading` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3/D4) |
| (outros 15) | — | — | MATCH / AUSENTE / ERRO_DESCRITIVO | preservados | sem regressão |

**DIFFs restantes:** 3 (de 5 para 3) — conforme esperado.  
**PANICs:** 0 (preservado).  
**Sentinela P492:** `p492_cores_predefinidas_text_e_stroke` passa.

---

## Resumo por classificação (bateria P490 pós-P492)

| Classificação | Contagem | Ficheiros |
|---|---|---|
| **MATCH** | 6 | test-math, test-set-local, test-columns, test-page, test-place, test-stroke-sides, test-show-regex |
| **DIFF** | 3 | test-table, test-array, test-show-where-multi |
| **AUSENTE** (selector não registado) | 6 | test-list, test-enum, test-par, test-show-link, test-raw, test-quote, test-footnote |
| **ERRO_DESCRITIVO** adicional | 4 | test-calc, test-str, test-dict retornam count=0 mas foram normalizados como sem contagem; test-calc.typ, test-str.typ, test-dict.typ têm query vazia |
| **PANIC** | **0** | — |

Nota: a classificação P490 usa `metadata`/`heading` como proxy; ficheiros que compilam sem PANIC e retornam count=0 são considerados estruturalmente equivalentes ao vanilla (que também retorna ok(0) para os mesmos selectors).

---

## Ficheiros alterados

- `01_core/src/rules/stdlib/color.rs` — `predefined_color_bindings()` com as 9 variáveis de cor predefinidas.
- `01_core/src/rules/stdlib/text.rs` — `native_text` com suporte a `fill` posicional e nomeado.
- `01_core/src/rules/eval/mod.rs` — injeção das cores e da função `text` no scope global.
- `01_core/src/rules/eval/operators.rs` — operadores `Length + Color` e `Color + Length` produzindo `Value::Stroke`.
- `01_core/src/rules/eval/tests.rs` — testes unitários P492 para cores, stroke, `text(...)` e show regex.
- `lab/parity/tests/structural_parity.rs` — sentinela `p492_cores_predefinidas_text_e_stroke`.
- `lab/parity/src/value_dto.rs` — mapeamento de variants `Value` recentes (`Relative`, `Stroke`, `Gradient`, `Regex`, `Tiling`, `Bytes`, `Decimal`, `Duration`, `Version`, `Selector`, `Symbol`) para `Other`, necessário para compilação dos testes de paridade.

---

## Próximo passo (P493)

Gaps restantes do P490:

| Grupo | Gap | Tamanho | Recomendação |
|---|---|---|---|
| D3 | Field access em coleções (`arr.dedup()`, `table.header`, `show.where(multi)`) | M-size | Field access em arrays/dicts + table header/footer |
| D1 | Selectores ausentes (`list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`) | M-size | Expansão `parse_selector` |

Recomendação do P492: **P493 = D3 (field access em coleções)** — fecha `test-table.typ`, `test-array.typ` e parte de `test-show-where-multi.typ`, reduzindo DIFFs de 3 para 0–1 antes de atacar os selectores D1.
