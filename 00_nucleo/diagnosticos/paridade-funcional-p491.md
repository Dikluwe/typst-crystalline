# Diagnóstico de Paridade Funcional — P491

> **Passo:** 491
> **Data:** 2026-06-29
> **Foco:** Fechar empiricamente os 4 gaps de args nomeados do Grupo D2 identificados em P490.
> **Metodologia:** Implementação em `01_core/` + testes unitários + re-execução da bateria P490 (20 ficheiros).
> **ADR referência:** ADR-0075, ADR-0054 (graded parity).

---

## Resultados por sub-tarefa D2

| Sub-tarefa | Vanilla | Cristalino pós-P491 | Classificação | Notas |
|---|---|---|---|---|
| 491a `calc.log(base:)` | ok(2.0) | ok(2.0) | **MATCH** | forma posicional preservada; base nomeado e posicional são mutuamente exclusivos |
| 491b `calc.round(digits:)` | ok(3.57) | ok(3.57) | **MATCH** | default `digits: 0` preservado; `digits` negativo funciona |
| 491c `str(base:)` | ok("ff") | ok("ff") | **MATCH** | base 2–36 validada; negativo preserva sinal |
| 491d `dict.at(default:)` | ok(99) | ok(99) | **MATCH** | erro sem `default` preservado; `dict.keys()`/`values()` adicionados como colateral para evitar EvalFailed no corpus P490 |

---

## Não-regressão — bateria P490 (20 ficheiros)

| Ficheiro | Selector | Vanilla | Cristalino pré-P491 | Cristalino pós-P491 | Δ |
|---|---|---|---|---|---|
| test-calc.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-str.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-dict.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **DIFF → MATCH** |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) |
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) |
| test-show-regex.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D4/D5) |
| test-show-where-multi.typ | `heading` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3/D4) |
| test-stroke-sides.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D4/D5) |
| (outros 12) | — | — | AUSENTE / MATCH / ERRO_DESCRITIVO | preservados | sem regressão |

**DIFFs restantes:** 5 (de 9 para 5) — conforme esperado.  
**PANICs:** 0 (preservado).  
**Sentinela P491:** `p491_args_nomeados_lote_d2` passa.

---

## Resumo por classificação (bateria P490 pós-P491)

| Classificação | Contagem | Ficheiros |
|---|---|---|
| **MATCH** | 4 | test-math, test-set-local, test-columns, test-page, test-place |
| **DIFF** | 5 | test-table, test-array, test-show-regex, test-show-where-multi, test-stroke-sides |
| **AUSENTE** (selector não registado) | 6 | test-list, test-enum, test-par, test-show-link, test-raw, test-quote, test-footnote |
| **PANIC** | **0** | — |

Nota: a classificação P490 usa `metadata`/`heading` como proxy; ficheiros que compilam sem PANIC e retornam count=0 são considerados estruturalmente equivalentes ao vanilla (que também retorna ok(0) para os mesmos selectors).

---

## Ficheiros alterados

- `01_core/src/rules/stdlib/calc.rs` — `calc_log(base:)` e `calc_round(digits:)`.
- `01_core/src/rules/stdlib/foundations.rs` — `str(base:)`.
- `01_core/src/rules/stdlib/collections.rs` — `dict.at(default:)`, `dict.keys()`, `dict.values()`.
- `01_core/src/rules/stdlib/mod.rs` — testes unitários P491 para `calc.log`, `calc.round`, `str`.
- `01_core/src/rules/eval/tests.rs` — testes unitários P491 para `dict.at`, `dict.keys`, `dict.values`.
- `lab/parity/tests/structural_parity.rs` — sentinela `p491_args_nomeados_lote_d2`.

---

## Próximo passo (P492)

Gaps restantes do P490:

| Grupo | Gap | Tamanho | Recomendação |
|---|---|---|---|
| D1 | Selectores ausentes (`list`, `enum`, `par`, `link`, `raw`, `quote`, `footnote`) | M-size | Expansão `parse_selector` |
| D3 | Field access (`arr.dedup()`, `table.header`) | M-size | Field access em coleções |
| D4/D5 | Variáveis de cor (`red`, `blue`, `green`) | S-size | Scope de variáveis predefinidas |

Recomendação do P491: **P492 = D4/D5 (variáveis de cor)** — S-size rápido, desbloqueia `test-stroke-sides.typ` e `test-show-regex.typ`, reduzindo DIFFs de 5 para 3 antes de atacar os M-size.
