# Diagnóstico de Paridade Funcional — P493

> **Passo:** 493
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 3 gaps de field access em coleções identificados no diagnóstico P490 (Grupo D3).
> **Metodologia:** Implementação em `01_core/` + testes unitários + re-execução da bateria P490 (20 ficheiros).
> **ADR referência:** ADR-0075, ADR-0054 (graded parity), ADR-0109 (atomização).

---

## Resultados por sub-tarefa D3

| Sub-tarefa | Vanilla | Cristalino pós-P493 | Classificação | Notas |
|---|---|---|---|---|
| 493a `arr.dedup()` | ok((3, 1, 4, 1, 5)) | ok((3, 1, 4, 1, 5)) | **MATCH** | Remove duplicados adjacentes; paridade vanilla confirmada. |
| 493a `arr.chunks(n)` | ok(((1,2), (3,4), (5))) | ok(((1,2), (3,4), (5))) | **MATCH** | Último chunk menor quando `len % n != 0`. |
| 493a `arr.windows(n)` | ok(((1,2), (2,3))) | ok(((1,2), (2,3))) | **MATCH** | Janelas deslizantes. |
| 493a colateral `arr.flatten()` | ok((1, 2, 3, 4)) | ok((1, 2, 3, 4)) | **MATCH** | Necessário para `test-array.typ` passar sem erro. |
| 493a colateral `arr.fold(start, reducer)` | ok(6) | ok(6) | **MATCH** | Necessário para `test-array.typ` passar sem erro. |
| 493b `table.header` / `table.footer` / `table.cell` | ok(1) | ok(1) | **MATCH** | Namespace anexado em `Func`; bindings flat mantidos como fallback. |
| 493c `heading.where(level: 1, outlined: true)` | ok(0) | ok(0) | **MATCH** | Cadeia de `Selector::Where`; campo `outlined` adicionado a `HeadingElem`. |

---

## Não-regressão — bateria P490 (20 ficheiros)

| Ficheiro | Selector | Vanilla | Cristalino pré-P493 | Cristalino pós-P493 | Δ |
|---|---|---|---|---|---|
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | ok(1) | **DIFF → MATCH** |
| test-show-where-multi.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-stroke-sides.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH (P492) |
| test-show-regex.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH (P492) |
| test-calc.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-str.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-dict.typ | `metadata` | ok(0) | ok(0) | ok(0) | MATCH (P491) |
| test-math.typ | `math.equation` | ok(5) | ok(5) | ok(5) | MATCH |
| (outros 10) | — | — | AUSENTE / ERRO_DESCRITIVO | preservados | sem regressão |

**DIFFs restantes:** 0 (todos os DIFFs do P490 resolvidos).  
**PANICs:** 0 (preservado).  
**Sentinela P493:** `p493_field_access_colecoes` passa.

---

## Resumo por classificação (bateria P490 pós-P493)

| Classificação | Contagem | Ficheiros |
|---|---|---|
| **MATCH** | 3 | test-table, test-math, test-array |
| **DIFF** | **0** | — |
| **AUSENTE** (selector não registado) | 8 | test-list, test-enum, test-par, test-show-link, test-raw, test-quote, test-footnote, test-columns, test-set-local, test-page, test-place, test-stroke-sides, test-show-regex (count=0 sem PANIC) |
| **ERRO_DESCRITIVO** | 7 | test-list, test-enum, test-par, test-show-link, test-raw, test-quote, test-footnote |
| **PANIC** | **0** | — |

Nota: a classificação P490 usa `metadata`/`heading`/`table`/`math.equation` como proxy; ficheiros que compilam sem PANIC e retornam count=0 são considerados estruturalmente equivalentes ao vanilla (que também retorna ok(0) para os mesmos selectors). A distinção entre AUSENTE e ERRO_DESCRITIVO depende de onde a bateria deteta a falha (selector não registado vs erro descritivo no eval).

---

## Ficheiros alterados

- `01_core/src/rules/stdlib/collections.rs` — `array.dedup()`, `array.chunks(n)`, `array.windows(n)`, `array.flatten()`, `array.fold(start, reducer)`.
- `01_core/src/rules/eval/bindings.rs` — field access em `Value::Array` (`len`, `first`, `last`); field access em `Value::Func` com namespace; `eval_element_where` multi-field.
- `01_core/src/entities/func.rs` — namespace anexado em `NativeFunc`/`NativeFuncWithEngine`; construtores `native_with_namespace`/`native_with_engine_and_namespace`; método `Func::namespace()`.
- `01_core/src/rules/eval/mod.rs` — registo de `table` com namespace (`table.header`, `table.footer`, `table.cell`).
- `01_core/src/rules/eval/rules.rs` — `query_selector_to_show_selector` recursivo para `Where` encadeado.
- `01_core/src/entities/elements/heading.rs` — campo `outlined: bool` em `HeadingElem`; `get_field` expõe `outlined`.
- `01_core/src/rules/eval/repr.rs` — teste `repr_content_heading` atualizado para `outlined`.
- `01_core/src/rules/eval/tests.rs` — testes unitários P493.
- `lab/parity/tests/structural_parity.rs` — sentinela `p493_field_access_colecoes`.
- `lab/parity/src/value_dto.rs` — mapeamento de variants `Value` adicionais (P492).
- Prompts L0 atualizados:
  - `00_nucleo/prompts/entities/func.md`
  - `00_nucleo/prompts/rules/eval/field-access.md`
  - `00_nucleo/prompts/rules/stdlib/collections.md`
  - `00_nucleo/prompts/rules/eval/table.md`
  - `00_nucleo/prompts/entities/selector.md`
  - `00_nucleo/prompts/entities/elements/heading.md`

---

## Próximo passo (P494)

Com P493 fechado, todos os DIFFs do P490 estão resolvidos. Os gaps restantes são apenas AUSENTEs/ERRO_DESCRITIVOs (selectors não registrados):

| Grupo | Gap | Tamanho | Recomendação |
|---|---|---|---|
| D1 | Selectores ausentes (`list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`) | M-size | P494 = expansão `parse_selector` |

**Recomendação:** P494 = **D1 (selectors ausentes)** — pode ser split em sub-tarefas:
- P494a: `list`, `enum`, `par`
- P494b: `link`, `raw`, `quote`, `footnote`
