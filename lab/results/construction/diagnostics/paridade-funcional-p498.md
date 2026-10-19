# Diagnóstico de Paridade Funcional — P498

> **Passo:** 498  
> **Data:** 2026-06-29  
> **Foco:** Fechar o último gap da bateria P490: `query(heading)` retorna count=0 após `#show heading.where(...)` porque show-rules consumiam o elemento locatable durante o eval.  
> **Metodologia:** Implementação arquitetural (passagem dupla do eval) + testes unitários + sentinela P498 + re-execução completa da bateria P490 (20 ficheiros).  
> **ADR referência:** ADR-0075, ADR-0054, ADR-0107, ADR-0109.

---

## Diagnóstico da Divergência Arquitetural (D3c)

### Estado baseline (P497)

```typst
// test-show-where-multi.typ
#show heading.where(level: 1, outlined: true): it => upper(it.body)
= Heading nível 1
```

| Oráculo | Resultado `query heading` |
|---|---|
| Vanilla 0.14.2 | `ok(1)` |
| Cristalino pré-P498 | `count=0` |
| **Classificação** | **DIFF** (arquitetural) |

### Causa raiz

No **vanilla**, o `Introspector` consulta os **elementos originais** (pré-show-rules), não o documento renderizado. Uma show-rule transforma o *output* visual, mas o *elemento* original persiste para introspecção.

No **cristalino pré-P498**, `intercept_content` aplicava show-rules **diretamente no `Content`** durante o eval. O `Content::Heading` era substituído pelo output da transformação, deixando de existir para o `TagIntrospector` construído a partir do conteúdo pós-show-rules.

### Solução implementada (H1 adaptada — passagem dupla)

Em vez de duplicar a estrutura de dados do `Content`, o `eval` passou a correr **duas vezes**:

1. **`apply_show_rules = false`** — produz `Module::introspection_content`, a árvore original de elementos locatable (heading, figure, metadata, etc.) sem qualquer transformação de show-rule.
2. **`apply_show_rules = true`** — produz `Module::content`, o output renderizado pós-show-rules, usado pelo layout/PDF.

A flag `EvalContext::apply_show_rules` controla `intercept_content`: quando `false`, as show-rules são registadas mas **não aplicadas**.

**Vantagens:**
- Não altera a estrutura de dados `Content`.
- Não exige refactor dos locatable elements individualmente.
- Preserva labels, counters e headings_for_toc no conteúdo original.

**Custo:** eval corre duas vezes por ficheiro. Aceite para fechar o gap sem refactor arquitetural profundo.

---

## Implementação

### Ficheiros alterados

| Camada | Ficheiro | Alteração |
|---|---|---|
| L1 | `01_core/src/entities/engine.rs` | *sem alteração directa* — a flag foi colocada em `EvalContext` |
| L1 | `01_core/src/engine/eval/mod.rs` | Adiciona `apply_show_rules` a `EvalContext`; refactor de `eval_with_full_error` para passagem dupla; usa sink dummy na primeira passagem |
| L1 | `01_core/src/engine/eval/rules.rs` | `intercept_content` respeita `ctx.apply_show_rules` |
| L1 | `01_core/src/entities/module.rs` | Novo campo `introspection_content` + getters/setters |
| L1 | `01_core/src/engine/eval/tests.rs` | Teste `p498_d3c_residual_heading_query_pos_show_rule` |
| L3 | `03_infra/src/query_helpers.rs` | Usa `module.introspection_content()` para construir o introspector |
| L3 | `03_infra/src/pipeline.rs` | Usa `module.introspection_content()` para construir o introspector |
| L0 | `00_nucleo/prompts/engine/eval.md` | Documenta a passagem dupla e `apply_show_rules` |
| L0 | `00_nucleo/prompts/entities/module.md` | Documenta `introspection_content` |
| Lab | `lab/parity/tests/structural_parity.rs` | Sentinela `p498_d3c_residual` |
| Diagnóstico | `00_nucleo/diagnosticos/paridade-funcional-p498.md` | Este relatório |

### Hashes L0 pós `--fix-hashes`

- `01_core/src/entities/module.rs` → `e9055d10`
- `01_core/src/engine/eval/*.rs` (incluindo `mod.rs`, `rules.rs`, `tests.rs`, etc.) → `7272c897`

---

## Testes

### Unitário

```bash
cargo test --lib -p typst-core p498_ -- --nocapture
```

**Resultado:** 1 passed; 0 failed.

- `p498_d3c_residual_heading_query_pos_show_rule` — verifica que:
  - O output renderizado contém `"HEADING NÍVEL 1"` (show-rule aplicada).
  - `module.introspection_content()` tem o heading original.
  - `intr.query(heading)` retorna `1` location.

### Sentinela P498

```bash
cd lab/parity
cargo test --test structural_parity p498_d3c_residual -- --nocapture
```

**Resultado:** `test p498_d3c_residual ... ok` (inclui verificação contra vanilla CLI).

### Bateria P490 completa

```bash
cargo test --test structural_parity p490_bateria_paridade_funcional_20_ficheiros -- --nocapture
```

**Resumo automático:** 10 matches (count>0), 7 absents (count=0), 0 erros descritivos, **0 PANICs**.

**Classificação manual (ADR-0107 — paridade de linguagem):**

| Ficheiro | Selector | Vanilla | Cristalino pós-P498 | Classificação |
|---|---|---|---|---|
| test-list-marker-array.typ | `list` | ok(1) | count=1 | MATCH |
| test-enum-start.typ | `enum` | ok(1) | count=1 | MATCH |
| test-par.typ | `par` | ok(1) | count=1 | MATCH |
| test-show-link.typ | `link` | ok(1) | count=1 | MATCH |
| test-table.typ | `table` | ok(1) | count=1 | MATCH |
| test-raw.typ | `raw` | ok(1) | count=1 | MATCH |
| test-quote.typ | `quote` | ok(1) | count=1 | MATCH |
| test-footnote.typ | `footnote` | ok(1) | count=1 | MATCH |
| test-calc.typ | `metadata` | ok(0) | count=0 | MATCH |
| test-array.typ | `metadata` | ok(0) | count=0 | MATCH |
| test-str.typ | `metadata` | ok(0) | count=0 | MATCH |
| test-dict.typ | `metadata` | ok(0) | count=0 | MATCH |
| test-show-regex.typ | `heading` | ok(0) | count=0 | MATCH |
| test-set-local.typ | `heading` | ok(0) | count=0 | MATCH |
| **test-show-where-multi.typ** | `heading` | ok(1) | **count=1** | **MATCH** |
| test-math.typ | `math.equation` | ok(5) | count=5 | MATCH |
| test-columns.typ | `heading` | ok(0) | count=0 | MATCH |
| test-page.typ | `heading` | ERRO_VAN | count=0 | MATCH |
| test-place.typ | `heading` | ERRO_VAN | count=0 | MATCH |
| test-stroke-sides.typ | `heading` | ok(0) | count=0 | MATCH |

**Resumo numérico final:**

| Métrica | Pré-P498 (P497) | Pós-P498 |
|---|---|---|
| MATCH | 19 | **20** |
| DIFF | 1 | **0** |
| AUSENTE | 0 | **0** |
| PANIC | 0 | **0** |

> **A bateria P490 atinge 20/20 MATCH.**

### Regressão — suites completas

```bash
cargo test -p typst-core      # 3474 passed; 0 failed
cargo test -p typst-infra     # 537 passed; 0 failed
cargo test --test structural_parity # 24 passed; 0 failed
```

Nenhuma regressão detectada.

---

## Validações finais

- [x] Diagnóstico arquitetural documentado.
- [x] Implementação funciona: `query(heading)` retorna count=1 após show-rule.
- [x] Teste unitário `p498_d3c_residual_heading_query_pos_show_rule` passa.
- [x] Sentinela `p498_d3c_residual` passa.
- [x] Bateria P490 completa: **20/20 MATCH**.
- [x] DIFFs restantes: **0**.
- [x] PANICs: 0.
- [x] AUSENTEs: 0.
- [x] Documentação L0 atualizada (`eval.md`, `module.md`).
- [x] Hashes L0 corrigidos.
- [x] `cargo build` sem erros.
- [x] `crystalline-lint .` com zero violations.
- [x] Relatório `paridade-funcional-p498.md` produzido.

---

## Próximos Passos (P499+)

Com P498 fechado, a bateria P490 está **100% MATCH**.

Recomendações:

1. **P499 = Audit de cobertura stdlib expandida** — criar novos ficheiros `.typ` para funcionalidades não cobertas pela bateria P490 (ex.: `image` com `fit`, `page` com `header`/`footer`, `place` avançado, restantes `calc`/`str`/`dict`).

2. **P500 = Performance benchmark** — comparar tempo de compilação cristalino vs vanilla em corpus maior (DEBT-42). A passagem dupla do P498 pode ter impacto mensurável; avaliar se justifica otimização futura.

3. **P501 = Relatório final de paridade P490–P498** — consolidar a matriz de cobertura stdlib, documentar a decisão arquitetural do D3c e produzir artefacto para publicação.
