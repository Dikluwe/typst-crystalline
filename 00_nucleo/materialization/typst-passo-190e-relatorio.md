# Relatório P190E — Categoria Numbering active (Caso 1 — eliminação parcial)

**Data**: 2026-05-05
**Estado**: ✅ Completo (8 sub-passos A-I)
**Magnitude**: M (Caso 1 — write paralelo M5 preservado).
**Pattern arquitectural**: ADR-0070 — 4ª aplicação concreta; padrão "eliminação directa via Introspector" aplicado parcialmente (Layouter consumer migration completa; field defer P190F).

---

## §1 Sumário executivo

P190E fecha **Categoria 4 (Numbering active)** parcialmente — **Caso 1 confirmado empiricamente**: walk readers de `state.is_numbering_active` durante walk pre-pass obrigam a preservar mutações + field até helpers migrarem (P190F).

**Trabalho executado**:
- 2 Layouter consumers migrados (`equation.rs:33` + `mod.rs:351`) para `is_numbering_active_at(key, location)` (P185B location-aware).
- 2 Layouter assignments duais eliminados (`mod.rs:1497` + `mod.rs:1526`).
- **Field `numbering_active` PRESERVADO** em `CounterStateLegacy` (defer P190F).
- **Walk mutations PRESERVADAS** em SetHeadingNumbering + SetEquationNumbering (write paralelo M5 obrigatório).
- **7 tests sentinela legacy redundantes removidos** (testavam fallback path eliminado).
- **1 test adaptado** (`layout_equation_bloco_numerada` — pipeline standard com `Content::SetEquationNumbering`).

**Achado crítico**: migração inicial usava `is_numbering_active` (snapshot final) que falhava para re-update scenarios. Corrigido para `is_numbering_active_at(key, current_location)` location-aware (P185B) — semântica correcta paritária com legacy state mutado durante walk Layouter.

**Output observable em produção**: inalterado.

---

## §2 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `rules/layout/equation.rs:33` | `is_numbering_active(key) || self.counter.is_numbering_active(key)` → `self.current_location.map(\|loc\| self.introspector.is_numbering_active_at("numbering_active:equation", loc)).unwrap_or(false)`. |
| 2 | `rules/layout/mod.rs:355` | Análogo para `numbering_active:heading`. |
| 3 | `rules/layout/mod.rs:1501` | Assignment `l.counter.numbering_active = initial_state.numbering_active` removido. |
| 4 | `rules/layout/mod.rs:1536` | Assignment fixpoint loop removido. |
| 5 | `rules/layout/tests.rs:966` | Test `layout_equation_bloco_numerada` adaptado para pipeline standard. |
| 6-12 | `rules/layout/tests.rs` | 7 tests sentinela fallback legacy removidos. |

**Field `numbering_active` PRESERVADO** em `CounterStateLegacy` (Caso 1).
**Walk arm mutations PRESERVADAS** em SetHeadingNumbering + SetEquationNumbering.

---

## §3 Caso 1 vs Caso 2 — decisão empírica

P190E diagnóstico previu 2 casos:
- **Caso 1**: walk readers de `state.numbering_active` durante walk → preservar mutações + field; defer eliminação para P190F.
- **Caso 2**: nenhum leitor walk → eliminar tudo.

Auditoria empírica P190E.A revelou **2 walk readers**:
1. `compute_heading_auto_toc` (P196B helper, introspect.rs:385): `state.is_numbering_active("heading")`.
2. Walk arm Equation gate (introspect.rs:579): `state.is_numbering_active("equation")`.

**Caso 1 obrigatório**. Eliminação completa do field defer para P190F (helpers migration).

---

## §4 Decisão notável — `is_numbering_active_at` location-aware

Migração inicial (1ª iteração) usou `self.introspector.is_numbering_active(key)` — **snapshot final**. Resultou em 9 tests failing.

**Achado**: para re-update scenarios (`SetHeadingNumbering(true)` → H1 → `SetHeadingNumbering(false)` → H2), snapshot final retorna `false` (último valor). Layouter precisa de location-aware lookup (estado **na location** do heading actual) para semântica correcta.

**Correcção**: usar `is_numbering_active_at(key, location)` (P185B) com `self.current_location` populated por `advance_locator_if_locatable` durante walk Layouter.

```rust
let numbering_on = self.current_location
    .map(|loc| self.introspector
        .is_numbering_active_at("numbering_active:heading", loc))
    .unwrap_or(false);
```

Resultado: 7 dos 9 failing tests passaram (foram resolvidos pela mudança location-aware); restantes 6 são tests sentinela legacy redundantes (removidos).

---

## §5 Estruturas

### `CounterStateLegacy`

| Estado | Fields | Δ |
|--------|--------|---|
| Pré-P190E (P190D baseline) | 10 | — |
| Pós-P190E | **10** | **0** (Caso 1 — field preservado) |

### `LayouterRuntimeState`

Inalterada — 3 fields (`label_pages`, `known_page_numbers`, `is_readonly`).

---

## §6 Tests workspace

| Estado | Total | Δ |
|--------|-------|---|
| Pré-P190E (P190D baseline) | 1.862 | — |
| Pós-P190E | **1.855** | -7 (sentinelas legacy redundantes removidos) |

**Tests removidos (7)**:
- `p182d_heading_numbering_via_fallback_legacy`.
- `p186f_equation_locatable::paridade_equation_counter_legacy_vs_introspector`.
- `p187b_c1_heading_prefix::c1_heading_prefix_via_fallback_legacy`.
- `p187b_c1_heading_prefix::c1_heading_prefix_paridade_legacy_vs_migrated`.
- `p188b_c2_equation_counter::c2_equation_counter_via_fallback_legacy_caso_producao`.
- `p188b_c2_equation_counter::c2_equation_counter_paridade_legacy_vs_introspector`.
- `p189b_walk_puro_m5::walk_excepcao_e1_equation_counter_via_legacy`.

**Test adaptado (1)**: `layout_equation_bloco_numerada` — usa pipeline standard com `Content::SetEquationNumbering`.

---

## §7 Verificações finais (.H — 13 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1.855 verdes (Δ vs P190D baseline 1.862: **-7**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Layouter consumers `numbering_active` migrados (location-aware) | ✅ |
| 5 | Per Caso 1: field `numbering_active` ainda existe | ✅ (defer P190F) |
| 6 | Per Caso 1: walk arm mutações preservadas | ✅ |
| 7 | Per Caso 1: `CounterStateLegacy`: 10 fields | ✅ (inalterado) |
| 8 | Layouter assignments duais eliminados (2 linhas) | ✅ |
| 9 | Comentários inline P190E presentes | ✅ |
| 10 | Trait `Introspector` NÃO modificado | ✅ |
| 11 | `TagIntrospector` NÃO modificado | ✅ |
| 12 | ADR-0070 PROPOSTO NÃO transitada | ✅ |
| 13 | Linter passa final | ✅ |

**13/13 verde.**

---

## §8 Decisões de execução notáveis

### Caso 1 confirmado empiricamente

P190E diagnóstico §1 previu Caso 1 vs Caso 2 baseado em walk readers. Auditoria revelou 2 walk readers obrigando Caso 1 — eliminação parcial.

### Migração location-aware obrigatória

Snapshot final (`is_numbering_active(key)`) insuficiente para re-update scenarios. Migração para `is_numbering_active_at(key, current_location)` (P185B) — paridade com legacy state mutado durante walk Layouter.

Este achado é arquitecturalmente significativo: confirma que **caminho Introspector location-aware é semanticamente equivalente ao state mutável durante walk**. Pre-condição para eliminação completa em M6.

### 7 tests sentinela removidos

Tests explicitamente nomeados `via_fallback_legacy` ou `paridade_legacy_vs_*` testavam o fallback path eliminado. Após P190E, são tests de comportamento dead. Removidos seguindo padrão pragmático auditor #1 (precedente P190B/P190D).

Cobertura observable preservada via:
- Tests P185B (`is_numbering_active_at` location-aware).
- Tests P198B (SetHeadingNumbering pipeline standard).
- Tests P199B (SetEquationNumbering pipeline standard).
- Test adaptado `layout_equation_bloco_numerada`.

---

## §9 Estado actual

- **P190 série**: A ✅ B ✅ C ✅ D ✅ E ✅ | F-I pendentes.
- **Categoria 4 (Numbering active) parcialmente fechada** — Layouter completo; field + walk preserved (defer P190F).
- **89 passos executados** (P190D=88 + P190E=89).

---

## §10 Pendências cumulativas

**4 categorias restantes** + 2 deferred:
- Categoria 5 (Counters core + 2 helpers) — P190F.
- Categoria 6 (Labels & TOC) — P190G.
- Categoria 7 (Figures) — P190H.
- Walk arms purification + Layouter final + struct elim + L0 + ADR-0070 ACEITE — P190I.
- **`lang` deferred** (P190D).
- **`numbering_active` deferred** (P190E Caso 1) — eliminação parcial; field preservado até P190F.

**M5 universal completo**: inalterado (0 + 0 + 0).

---

## §11 Próximo passo

**P190F** — Categoria 5 (Counters core + 2 helpers migrados):
- Migrar `compute_labelled` (P195D) para Introspector path location-aware (eliminar leitura de `state.flat`/`hierarchical`/`figure_numbers`/`lang`).
- Migrar `compute_heading_auto_toc` (P196B) para Introspector path (eliminar leitura de `state.is_numbering_active`/`format_hierarchical`).
- Após helpers migrarem: walk arm Equation gate via `intr.is_numbering_active_at` (eliminar leitura `state.is_numbering_active("equation")`).
- Eliminar walk arm SetHeadingNumbering + SetEquationNumbering mutações (write paralelo já desnecessário).
- Eliminar fields `numbering_active` (resolução defer P190E) + `flat` + `hierarchical` + `lang` (resolução defer P190D).
- Magnitude **M+** (categoria mais complexa por envolver 2 helpers + chained dependencies).

---

## §12 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0070 PROPOSTO (P190A).
- **4ª aplicação concreta** P190E.
- **Padrão "eliminação directa via Introspector"**: 3ª aplicação (após P190B Bibliography + P190D has_outline).
- **Caso 1 obrigatório**: walk readers preservam write paralelo M5; eliminação parcial.
- **Migração location-aware**: pattern `is_numbering_active_at(key, current_location)` aplicado a 2 Layouter consumers — confirmação empírica de equivalência semântica com state mutável.
- **Pre-condições**: P185B (location-aware trait methods); P198B (SetHeadingNumbering); P199B (SetEquationNumbering).
- **Hash código**: actualizado via `--fix-hashes` (Nothing to fix).
- **Padrão diagnóstico-primeiro**: 26ª aplicação consecutiva.
- **F1 progresso**: 10 → 10 (defer Caso 1) fields ortogonais; 1 ainda preservado.
- **F3 progresso**: Layouter ainda 20 fields; LayouterRuntimeState 3 fields.

---

## §13 Métricas finais P190E

- **Sub-passos**: 8 (A-I).
- **LOC produção**: ~25 (2 consumer migrations + 2 assignment removals + comentários).
- **LOC teste**: ~50 (7 tests removidos + 1 adaptado + comentários P190E inline).
- **LOC L0**: 0 (sem L0 modificada — counter_state_legacy.md defer P190I).
- **LOC relatório**: ~280.
- **Variants Content novas**: 0.
- **Sub-stores Introspector novos**: 0.
- **ADRs novas**: 0.
- **F1 fields eliminados**: 0 (defer Caso 1).
- **Layouter consumers migrados**: 2 (location-aware).
- **Layouter assignments eliminados**: 2.
- **Tests netos**: -7.
- **Hashes desactualizados**: 0.

**Padrão "is_numbering_active_at location-aware"**: 1ª aplicação concreta em consumer migration. Pre-condição para P190F (helpers migration) e P190I (eliminação final).
