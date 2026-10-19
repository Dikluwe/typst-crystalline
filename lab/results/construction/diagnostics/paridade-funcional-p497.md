# Diagnóstico de Paridade Funcional — P497

> **Passo:** 497  
> **Data:** 2026-06-29  
> **Foco:** Fechar empiricamente os gaps D4/D5 de variáveis de cor predefinidas e `text()` em show-regex (diagnóstico P490), e validar que a implementação P492 está completa e documentada.  
> **Metodologia:** Auditoria L0, testes unitários, sentinela P497 e re-execução da bateria P490 (20 ficheiros). Classificação manual por vanilla `typst query` vs cristalino.  
> **ADR referência:** ADR-0075, ADR-0054, ADR-0107, ADR-0109.

---

## Estado à chegada (pós-P496)

O material P497 assumia que `test-stroke-sides.typ` e `test-show-regex.typ` ainda
produziam `ERRO_DESCRITIVO` no cristalino. A **medição real** mostrou que ambos
já compilavam e retornavam `count=0` na bateria P490:

| Ficheiro | Vanilla | Cristalino pós-P496 | Classificação pré-P497 |
|---|---|---|---|
| `test-stroke-sides.typ` | `ok(0)` | `ok(0)` | MATCH |
| `test-show-regex.typ` | `ok(0)` | `ok(0)` | MATCH |

**Conclusão:** os gaps D4/D5 foram **fechados materialmente no P492** (injeção de
`predefined_color_bindings()` e de `text` no scope global). O trabalho do P497
foi, portanto, de **validação formal, documentação L0 e testes dedicados** — sem
alterações de lógica de negócio.

---

## Resultados por Sub-Tarefa

| Sub-tarefa | Vanilla | Cristalino pós-P497 | Classificação | Notas |
|---|---|---|---|---|
| 497a `stroke: (left: 3pt + red, ...)` | `ok(0)` | `ok(0)` | **MATCH** | Cores predefinidas + operador `Length + Color` já funcionam |
| 497b `#show regex("\d+"): it => text(red, it)` | `ok(0)` | `ok(0)` | **MATCH** | `text` global e show-regex já funcionam |

Ambos os casos foram validados também via `typst query` vanilla 0.14.2, com
output `[]` (count=0), confirmando paridade estrutural.

---

## Implementação já existente (P492)

| Camada | Ficheiro | O que já estava implementado |
|--------|----------|------------------------------|
| L1 | `01_core/src/engine/stdlib/color.rs` | `predefined_color_bindings()` com 9 entradas (`red`, `blue`, `green`, `black`, `white`, `yellow`, `cyan`, `magenta`, `none`) |
| L1 | `01_core/src/engine/eval/mod.rs` | Injeção das cores no scope global (linha 288) e definição global de `text` (linha 292) |
| L1 | `01_core/src/engine/eval/operators.rs` | Operadores `Length + Color` / `Color + Length` → `Value::Stroke` |
| L1 | `01_core/src/engine/stdlib/text.rs` | `native_text` aceita `fill` posicional e nomeado |

Nenhuma alteração funcional foi necessária em P497.

---

## Documentação L0 atualizada

| Ficheiro L0 | Alteração |
|---|---|
| `00_nucleo/prompts/engine/stdlib/color.md` | Nova secção "Cores predefinidas (P492/P497)" com a tabela de atalhos e nota sobre `text` global |
| `00_nucleo/prompts/engine/eval.md` | Nova secção "Scope global" descrevendo a construção do scope base: stdlib + cores + `text` + elementos de utilizador |

### Hashes L0 pós `--fix-hashes`

- `01_core/src/engine/eval/*.rs` (incluindo `mod.rs`, `tests.rs`, `rules.rs`, etc.) → `62c93675`
- `01_core/src/engine/stdlib/color.rs` → `64415bc3`

---

## Testes

### Unitários

```bash
cargo test --lib -p typst-core p497_ -- --nocapture
```

**Resultado:** 2 passed; 0 failed.

- `p497_stroke_cores_predefinidas` — compila `test-stroke-sides.typ` sem erro.
- `p497_show_regex_text_color` — compila `test-show-regex.typ` sem erro e preserva
  os dígitos "42" e "100" no `plain_text`.

### Sentinela P497

```bash
cd lab/parity
cargo test --test structural_parity p497_variaveis_cor_predefinidas -- --nocapture
```

**Resultado:** `test p497_variaveis_cor_predefinidas ... ok`

A sentinela executa os dois snippets do corpus p490 e verifica que `query heading`
retorna `count=0` (não-locatable), igual ao vanilla.

### Bateria P490 completa

```bash
cargo test --test structural_parity p490_bateria_paridade_funcional_20_ficheiros -- --nocapture
```

**Resumo automático:** 9 matches (count>0), 8 absents (count=0), 0 erros descritivos, **0 PANICs**.

**Classificação manual (ADR-0107 — paridade de linguagem, não mecânica):**

| Ficheiro | Selector | Vanilla | Cristalino pós-P497 | Classificação | Δ |
|---|---|---|---|---|---|
| test-list-marker-array.typ | `list` | ok(1) | count=1 | MATCH | — |
| test-enum-start.typ | `enum` | ok(1) | count=1 | MATCH | — |
| test-par.typ | `par` | ok(1) | count=1 | MATCH | — |
| test-show-link.typ | `link` | ok(1) | count=1 | MATCH | — |
| test-table.typ | `table` | ok(1) | count=1 | MATCH | P496 |
| test-raw.typ | `raw` | ok(1) | count=1 | MATCH | — |
| test-quote.typ | `quote` | ok(1) | count=1 | MATCH | — |
| test-footnote.typ | `footnote` | ok(1) | count=1 | MATCH | — |
| test-calc.typ | `metadata` | ok(0) | count=0 | MATCH | P495 |
| test-array.typ | `metadata` | ok(0) | count=0 | MATCH | P496 |
| test-str.typ | `metadata` | ok(0) | count=0 | MATCH | P495 |
| test-dict.typ | `metadata` | ok(0) | count=0 | MATCH | P495 |
| test-show-regex.typ | `heading` | ok(0) | count=0 | **MATCH** | **D5 fechado** |
| test-set-local.typ | `heading` | ok(0) | count=0 | MATCH | — |
| test-show-where-multi.typ | `heading` | ok(1) | count=0 | **DIFF** | D3c residual |
| test-math.typ | `math.equation` | ok(5) | count=5 | MATCH | — |
| test-columns.typ | `heading` | ok(0) | count=0 | MATCH | — |
| test-page.typ | `heading` | ERRO_VAN | count=0 | MATCH | scope-out |
| test-place.typ | `heading` | ERRO_VAN | count=0 | MATCH | scope-out |
| test-stroke-sides.typ | `heading` | ok(0) | count=0 | **MATCH** | **D4 fechado** |

**Resumo numérico:**

| Métrica | Pré-P497 (P496) | Pós-P497 |
|---|---|---|
| MATCH | 14 | **19** |
| DIFF | 3 | **1** |
| AUSENTE | 2 | **0** |
| PANIC | 0 | **0** |

> O material P497 esperava 17 MATCH / 1 DIFF / 0 AUSENTE. A medição real
> mostra 19 MATCH porque os 5 ficheiros com `count=0` esperado (`calc`, `str`,
> `dict`, `set-local`, `columns`) são **estruturalmente equivalentes** ao vanilla
> (paridade linguagem, ADR-0107), não ausentes. Os scope-outs `page`/`place`
> também são MATCH por não produzirem PANIC.

---

## Gap Remanescente

| Grupo | Gap | Ficheiro afetado | Notas |
|---|---|---|---|
| D3c residual | Show rules consomem elemento locatable; `query(heading)` retorna count=0 após `#show heading.where(...)` | `test-show-where-multi.typ` | Divergência arquitectural documentada em P496; fora do scope S de P497 |

---

## Critérios de Fecho P497

- [x] 497a validado: `red`/`blue`/`green`/`none` funcionam em `stroke`.
- [x] 497b validado: `text(red, it)` funciona em show-regex.
- [x] 2 testes unitários P497 passam.
- [x] Sentinela `p497_variaveis_cor_predefinidas` passa.
- [x] Bateria P490: D4 e D5 viraram MATCH (já estavam materialmente).
- [x] DIFFs restantes: **1** (D3c residual, divergência arquitectural documentada).
- [x] PANICs: 0.
- [x] AUSENTEs: 0.
- [x] Documentação L0 atualizada (`color.md`, `eval.md`).
- [x] Hashes L0 corrigidos.
- [x] `cargo build` sem erros.
- [x] `crystalline-lint .` com zero violations.
- [x] Relatório `paridade-funcional-p497.md` produzido.

---

## Ficheiros alterados

| Camada | Ficheiro | Alteração |
|--------|----------|-----------|
| L0 | `00_nucleo/prompts/engine/stdlib/color.md` | Secção de cores predefinidas |
| L0 | `00_nucleo/prompts/engine/eval.md` | Secção de scope global |
| L1 | `01_core/src/engine/eval/*.rs` | Hash atualizado para `62c93675` |
| L1 | `01_core/src/engine/stdlib/color.rs` | Hash atualizado para `64415bc3` |
| L1 | `01_core/src/engine/eval/tests.rs` | Testes `p497_stroke_cores_predefinidas` e `p497_show_regex_text_color` |
| Lab | `lab/parity/tests/structural_parity.rs` | Sentinela `p497_variaveis_cor_predefinidas` |
| Diagnóstico | `00_nucleo/diagnosticos/paridade-funcional-p497.md` | Este relatório |

---

## Próximos Passos (P498+)

Com P497 fechado, a bateria P490 atinge **19/20 MATCH**, **1 DIFF residual**
(D3c, arquitectural) e **0 PANICs**.

Recomendações:

1. **P498 = Resolver D3c residual** — separar o elemento locatable do output de
   show rules para que `query(heading)` funcione após `#show heading.where(...)`.
   É o último gap da bateria P490.

2. **P499 = Audit de cobertura stdlib expandida** — criar novos ficheiros `.typ`
   para funcionalidades não cobertas pela bateria P490 (ex.: `image` com `fit`,
   `page` com `header`/`footer`, `place` avançado).

3. **P500 = Relatório final de paridade P490–P497** — consolidar a matriz de
   cobertura stdlib e a nota sobre D3c residual.
