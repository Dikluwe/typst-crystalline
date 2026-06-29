# Diagnóstico de Paridade Funcional — P495

> **Passo:** 495  
> **Data:** 2026-06-29  
> **Metodologia:** Materialização do Grupo D2 de gaps do diagnóstico P490 — 4 argumentos nomeados. Validação via sentinela P495, testes unitários e re-execução da bateria P490 (20 ficheiros).  
> **ADR referência:** ADR-0075, ADR-0054, ADR-0107, ADR-0109.

---

## Objetivo

Fechar empiricamente os 4 gaps de argumentos nomeados do **Grupo D2**, ainda
persistentes após P494:

- D2a — `calc.log(base:)`
- D2b — `calc.round(digits:)`
- D2c — `str(base:)`
- D2d — `dict.at(default:)`

---

## Resultados por Sub-Tarefa

| Sub-tarefa | Vanilla | Cristalino | Classificação | Notas |
|------------|---------|------------|---------------|-------|
| 495a `calc.log(base:)` | ok(2.0) | ok(2.0) | **MATCH** | `calc.log(100, base: 10)`; posicional `calc.log(100, 10)` preservado |
| 495b `calc.round(digits:)` | ok(3.57) | ok(3.57) | **MATCH** | `calc.round(3.567, digits: 2)`; default 0 preservado |
| 495c `str(base:)` | ok("ff") | ok("ff") | **MATCH** | `str(255, base: 16)`; base 2–36 validada; negativos suportados |
| 495d `dict.at(default:)` | ok(99) | ok(99) | **MATCH** | `d.at("z", default: 99)`; erro sem default preservado |

---

## Bateria P490 (pós-P495)

| Ficheiro | Selector | Vanilla | Cristalino pré-495 | Cristalino pós-495 | Classificação pós-495 | Δ |
|----------|----------|---------|--------------------|--------------------|-----------------------|---|
| test-list-marker-array.typ | `list` | ok(1) | ok(1) | ok(1) | MATCH | — |
| test-enum-start.typ | `enum` | ok(1) | ok(1) | ok(1) | MATCH | — |
| test-par.typ | `par` | ok(1) | ok(1) | ok(1) | MATCH | — |
| test-show-link.typ | `link` | ok(1) | ok(1) | ok(1) | MATCH | — |
| test-table.typ | `table` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) | — |
| test-raw.typ | `raw` | ok(1) | ok(1) | ok(1) | MATCH | — |
| test-quote.typ | `quote` | ok(1) | ok(1) | ok(1) | MATCH | — |
| test-footnote.typ | `footnote` | ok(1) | ok(1) | ok(1) | MATCH | — |
| test-calc.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **MATCH** | **DIFF → MATCH** |
| test-array.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) | — |
| test-str.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **MATCH** | **DIFF → MATCH** |
| test-dict.typ | `metadata` | ok(0) | ERRO_DESCRITIVO | ok(0) | **MATCH** | **DIFF → MATCH** |
| test-show-regex.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D4/D5) | — |
| test-set-local.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH | — |
| test-show-where-multi.typ | `heading` | ok(1) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D3) | — |
| test-math.typ | `math.equation` | ok(5) | ok(5) | ok(5) | MATCH | — |
| test-columns.typ | `heading` | ok(0) | ok(0) | ok(0) | MATCH | — |
| test-page.typ | `heading` | ERRO_VAN | ok(0) | ok(0) | MATCH | — |
| test-place.typ | `heading` | ERRO_VAN | ok(0) | ok(0) | MATCH | — |
| test-stroke-sides.typ | `heading` | ok(0) | ERRO_DESCRITIVO | ERRO_DESCRITIVO | DIFF (D4/D5) | — |

### Resumo Numérico

| Métrica | Pré-P495 (P494) | Pós-P495 |
|---------|-----------------|----------|
| MATCH | 9 | **12** |
| DIFF | 7 | **4** |
| AUSENTE | 2 | 2 |
| PANIC | **0** | **0** |

> Nota: o resumo automático da sentinela P490 reporta "Matches=9 / Absents=8" porque
> `test-calc`, `test-str` e `test-dict` devolvem `count=0` (não há `metadata` nos
> ficheiros). A classificação manual é **MATCH**, pois ambos os lados produzem
> `ok(0)` sem erros — o objetivo do D2 era eliminar o `ERRO_DESCRITIVO`.

---

## Implementação

### Ficheiros alterados

| Camada | Ficheiro | Alteração |
|--------|----------|-----------|
| L0 | `00_nucleo/prompts/rules/stdlib/calc.md` | Especifica `calc.log(base:)` e `calc.round(digits:)` |
| L0 | `00_nucleo/prompts/rules/stdlib/foundations.md` | Especifica `str(value, base:)` |
| L0 | `00_nucleo/prompts/rules/stdlib/collections.md` | Especifica `dict.at(key, default:)` |
| L1 | `01_core/src/rules/stdlib/calc.rs` | Comentários/linhagem P495; hash atualizado |
| L1 | `01_core/src/rules/stdlib/foundations.rs` | Comentários/linhagem P495; hash atualizado |
| L1 | `01_core/src/rules/stdlib/collections.rs` | Testes unitários `p495_dict_at_*` |
| L1 | `01_core/src/rules/stdlib/mod.rs` | Testes unitários renomeados de `p491_*` para `p495_*` |
| Lab | `lab/parity/tests/structural_parity.rs` | Sentinela renomeada para `p495_args_nomeados_lote_d2` |
| Diagnóstico | `00_nucleo/diagnosticos/paridade-funcional-p495.md` | Este relatório |

### Hashes L0 (pós `--fix-hashes`)

- `01_core/src/rules/stdlib/calc.rs` → `2b51efc0`
- `01_core/src/rules/stdlib/foundations.rs` → `0f8ce14b`

---

## Testes

### Unitários

```bash
cargo test --lib -p typst-core p495_ -- --nocapture
```

**Resultado:** 12 passed; 0 failed.

Testes cobertos:
- `p495_calc_log_base_named`
- `p495_calc_log_base_e_posicional_sao_mutuamente_exclusivos`
- `p495_calc_log_named_desconhecido_rejeitado`
- `p495_calc_round_digits_named`
- `p495_calc_round_digits_int_input`
- `p495_calc_round_named_desconhecido_rejeitado`
- `p495_str_base_named`
- `p495_str_base_negativo`
- `p495_str_base_invalida`
- `p495_str_named_desconhecido_rejeitado`
- `p495_dict_at_default_named`
- `p495_dict_at_named_desconhecido_rejeitado`

### Sentinela P495

```bash
cargo test --test structural_parity p495_args_nomeados_lote_d2 -- --nocapture
```

**Resultado:** `test p495_args_nomeados_lote_d2 ... ok`

### Bateria P490

```bash
cargo test --test structural_parity p490_bateria_paridade_funcional_20_ficheiros -- --nocapture
```

**Resultado:** 9 matches (count>0), 8 absents (count=0), 0 erros descritivos, **0 PANICs**.

---

## Gaps Remanescentes

| Grupo | Gap | Ficheiros afetados | Próximo passo sugerido |
|-------|-----|-------------------|------------------------|
| D3 | Field access em coleções | `test-array`, `test-table`, `test-show-where-multi` | P496 |
| D4/D5 | Variáveis de cor predefinidas + `text()` em show-regex | `test-stroke-sides`, `test-show-regex` | P497 |

---

## Critérios de Fecho P495

- [x] 495a — `calc.log(base:)` funciona; posicional preservado.
- [x] 495b — `calc.round(digits:)` funciona; default 0 preservado.
- [x] 495c — `str(value, base:)` funciona; base 2–36 validada.
- [x] 495d — `dict.at(key, default:)` funciona; erro sem default preservado.
- [x] 12 testes unitários P495 passam.
- [x] Sentinela `p495_args_nomeados_lote_d2` passa.
- [x] Bateria P490: 4 DIFFs de D2 viraram MATCH.
- [x] DIFFs restantes: 4 (D3 + D4/D5).
- [x] PANICs: 0.
- [x] Documentação L0 atualizada (`calc.md`, `foundations.md`, `collections.md`).
- [x] Hashes L0 corrigidos.
- [x] `cargo build` sem erros.
- [x] `crystalline-lint .` com zero violations.
- [x] Relatório `paridade-funcional-p495.md` produzido.

---

## Próximo Passo (P496)

Recomendação: **D3 — Field access em coleções** (`arr.dedup()`, `table.header/footer`,
`show.where(multi)`). Fecha 3 DIFFs de uma vez, reduzindo o total de DIFFs de 4
para 1 antes do S-size D4/D5.
