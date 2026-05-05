# Relatório P190F — Categoria Counters core (escopo reduzido por barreira arquitectural)

**Data**: 2026-05-05
**Estado**: ✅ Completo (escopo reduzido per achado empírico)
**Magnitude real**: M (Layouter migration); plano original era M+ (com helper migration).
**Pattern arquitectural**: ADR-0070 — 5ª aplicação parcial.

---

## §1 Sumário executivo — escopo reduzido por barreira arquitectural

P190F descobriu **barreira arquitectural empírica** que reduz escopo achievable. Plano original (P190F prompt) projectou:
- Migrar 2 helpers (`compute_labelled`, `compute_heading_auto_toc`) para Introspector path location-aware.
- Resolver 2 defers (`lang` P190D, `numbering_active` P190E).
- Eliminar 4 fields (`flat`, `hierarchical`, `numbering_active`, `lang`).
- Walk arm Equation gate migração.

**Auditoria empírica P190F.A revelou**: walk fn não tem acesso a `Introspector` (construído POST-walk via `from_tags::from_tags(&tags)`). Helpers chamados durante walk não podem queryar Introspector. Migração full helper requer redesign walk pipeline (out-of-scope para P190F).

**Escopo executado**:
1. **Layouter migration completa** — 2 mutações + 2 fallbacks eliminados.
2. **Documentação formal da barreira** com plano de redesign.

**Escopo deferido**:
- Helper migration (`compute_labelled`, `compute_heading_auto_toc`).
- Walk arm gate migration.
- Field elimination (4 fields).
- 2 defers (`lang`, `numbering_active`).

---

## §2 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `rules/layout/mod.rs:337` | Layouter mutação `self.counter.step_hierarchical("heading", level)` removida — counter populated via Introspector path (P185B). |
| 2 | `rules/layout/mod.rs:373` | Fallback `.or_else(\|\| self.counter.format_hierarchical("heading"))` removido — Introspector path único. |
| 3 | `rules/layout/equation.rs:39` | Layouter mutação `self.counter.step_flat("equation")` removida. |
| 4 | `rules/layout/equation.rs:113` | Fallback `.unwrap_or_else(\|\| self.counter.get_flat("equation"))` substituído por `.unwrap_or(0)` — Introspector path único. |
| 5 | Comentários inline P190F adicionados em todos os sítios. |

**Walk pre-pass UNCHANGED** — helpers + walk arm mutações preservadas (barreira arquitectural).
**Field `lang` PRESERVADO** (defer P190D mantido).
**Field `numbering_active` PRESERVADO** (defer P190E mantido).

---

## §3 Barreira arquitectural identificada

### Problema

Plano original P190F projectou migrar `compute_labelled` para signature:
```rust
fn compute_labelled<I: Introspector>(intr: &I, location: Location, ...) -> ...
```

E walk arm Equation gate para:
```rust
if !intr.is_numbering_active_at("numbering_active:equation", emitted_loc) { return; }
```

**Impossível com arquitectura actual**: Introspector é construído POST-walk:
```rust
pub fn introspect_with_introspector(...) -> (CounterStateLegacy, TagIntrospector) {
    walk(content, &mut state, &mut locator, &mut tags, None);
    let introspector = self::from_tags::from_tags(&tags, ...);
    (state, introspector)
}
```

Walk fn signature:
```rust
fn walk(
    content:           &Content,
    state:             &mut CounterStateLegacy,
    locator:           &mut Locator,
    tags:              &mut Vec<Tag>,
    label_from_parent: Option<&Label>,
);
```

Walk não tem `&Introspector` disponível durante execução.

### Consequências para fields walk-readable

- `state.flat` — lido por `compute_labelled` (Equation arm).
- `state.hierarchical` — lido por `compute_heading_auto_toc` (`format_hierarchical`).
- `state.figure_numbers` — lido por `compute_labelled` (Figure arm).
- `state.lang` — lido por `compute_labelled` (Figure arm para supplement).
- `state.numbering_active` — lido por `compute_heading_auto_toc` + walk arm Equation gate.

Estes 5 fields têm walk readers. Eliminação requer ou:
- **Opção A**: Walk fn signature change para aceitar `&mut TagIntrospector` (replicando from_tags logic durante walk).
- **Opção B**: Two-pass walk (1ª pass emit Tags; build Introspector; 2ª pass fill-in payloads).
- **Opção C**: Eliminate helpers; embed logic inline com computation manual.

Todas as opções são L cross-modular dentro de walk pre-pass — beyond P190F budget M+.

### Plano de redesign futuro

**Recomendação**: passo dedicado **P191A** (ou similar) **antes** de P190G/H/I:
- Audit empírico de architecture options (A/B/C).
- ADR-0071 para mecanismo escolhido.
- Magnitude L cross-modular esperada.
- Resolve simultaneamente 4 fields + 2 defers + helper migrations + walk arm gates.

**Alternativa pragmática**: P190G/H/I continuam com fields/categorias **sem walk readers** (Labels/TOC, Figures, Layouter assignment cleanup). Defer counters core categories até P191A (ou equivalente).

---

## §4 Estruturas

### `CounterStateLegacy`

| Estado | Fields | Δ |
|--------|--------|---|
| Pré-P190F | 10 | — |
| Pós-P190F | **10** | **0** (escopo reduzido por barreira) |

### `LayouterRuntimeState`

Inalterada — 3 fields.

### `Layouter<M, S>`

Mutações eliminadas (2 de 4 originais Layouter consumers — restantes 2 já migrados em P190E):
- `mod.rs:337` step_hierarchical → eliminada.
- `equation.rs:39` step_flat → eliminada.

Fallbacks eliminados:
- `mod.rs:373` `format_hierarchical` → eliminado.
- `equation.rs:113` `get_flat` → eliminado.

Field `counter: CounterStateLegacy` ainda preserved (até P190I).

---

## §5 Tests workspace

| Estado | Total | Δ |
|--------|-------|---|
| Pré-P190F (P190E baseline) | 1.855 | — |
| Pós-P190F | **1.855** | **0** |

Sem regressões — Layouter migration completou sem afectar tests existentes.

---

## §6 Verificações finais

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1.855 verdes (Δ=0) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Layouter mod.rs:337 mutação eliminada | ✅ |
| 5 | Layouter mod.rs:373 fallback eliminado | ✅ |
| 6 | Layouter equation.rs:39 mutação eliminada | ✅ |
| 7 | Layouter equation.rs:113 fallback eliminado | ✅ |
| 8 | Walk pre-pass NÃO modificado | ✅ |
| 9 | Helpers `compute_*` NÃO modificados | ✅ |
| 10 | `CounterStateLegacy`: 10 fields (inalterado) | ✅ |
| 11 | Defers `lang`, `numbering_active` mantidos | ✅ |
| 12 | Comentários inline P190F presentes | ✅ |
| 13 | Trait `Introspector` NÃO modificado | ✅ |
| 14 | `TagIntrospector` NÃO modificado | ✅ |
| 15 | Barreira arquitectural documentada | ✅ |

---

## §7 Decisão de execução notável — escopo reduzido honesto

P190F prompt projectou plano agressivo (4 fields + 2 helpers + walk gate). Auditoria empírica P190F.A revelou que isso requer redesign walk pipeline — beyond M+ budget.

**Decisão**: executar escopo reduzido (Layouter side apenas) e documentar formalmente a barreira. Honestidade > ilusão de progresso.

Esta decisão é arquiteturalmente significativa: identifica que **eliminação completa de `CounterStateLegacy` requer mais do que cleanup incremental** — precisa redesign do walk pipeline. M6 não é viável apenas com aplicações da pattern stylesheet ADR-0070; precisa de mecanismo arquitectural novo.

---

## §8 Estado actual

- **P190 série**: A ✅ B ✅ C ✅ D ✅ E ✅ F ⚠️ (escopo reduzido) | G-I pendentes.
- **Categoria 5 (Counters core) Layouter side fechada**.
- **Categoria 5 walk side defer** — barreira arquitectural.
- **90 passos executados** (P190E=89 + P190F=90).

---

## §9 Pendências cumulativas

**3 categorias restantes** + 3 deferred:
- Categoria 6 (Labels & TOC) — P190G.
- Categoria 7 (Figures) — P190H.
- Walk arms purification + Layouter final + struct elim + L0 + ADR-0070 ACEITE — P190I.
- **`lang` deferred** (P190D + P190F).
- **`numbering_active` deferred** (P190E).
- **`flat`, `hierarchical` deferred** (P190F barreira).

**Recomendação nova**: passo dedicado **P191A** (ou similar) para redesign walk pipeline antes de P190I final. ADR-0071 esperado.

**M5 universal completo**: inalterado (0 + 0 + 0).

---

## §10 Próximo passo

**P190G** — Categoria 6 (Labels & TOC):
- Field `auto_label_counter: usize` — walk-internal (incrementado por walk arm Heading; lido por compute_heading_for_toc P200B).
- Field `resolved_labels: HashMap<Label, String>` — write paralelo M5; caminho Introspector activo desde P193B (`intr.resolved_labels` ResolvedLabelStore).
- Field `headings_for_toc: Vec<(Label, Content, usize)>` — write paralelo M5; caminho Introspector activo desde P200B.

**Eligibility**: `resolved_labels` + `headings_for_toc` têm caminho Introspector estável; `auto_label_counter` é walk-internal — pode ser local var em walk fn.

Achievable: eliminate 2 fields (resolved_labels, headings_for_toc) com migration consumer + Layouter assignment cleanup; 1 field `auto_label_counter` requer walk-internal refactor.

Magnitude **M**.

---

## §11 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0070 PROPOSTO.
- **5ª aplicação parcial** P190F.
- **Achado arquitectural**: barreira walk pipeline para helper migration. Documentação formal.
- **Layouter side cleanup**: 2 mutações + 2 fallbacks eliminados.
- **Walk side preservado**: helpers + walk arm mutações intactas (defer architectural).
- **Pre-condições**: P185B (location-aware methods); P190E (location-aware Layouter consumers).
- **Hash código**: actualizado (Nothing to fix).
- **Padrão diagnóstico-primeiro**: 27ª aplicação consecutiva; **1ª vez** que diagnóstico empírico revelou plano original infeasível e levou a escopo reduzido honesto.
- **F1 progresso**: 10 → 10 fields (escopo reduzido).
- **F3 progresso**: Layouter ainda 20 fields; Layouter consumer-side 100% migrado para Introspector path location-aware (counters core).

---

## §12 Métricas finais P190F

- **LOC produção**: ~30 (Layouter 4 mudanças + comentários).
- **LOC teste**: 0 (sem regressões).
- **LOC L0**: 0.
- **LOC relatório**: ~370 (mais extenso por causa de documentação da barreira).
- **F1 fields eliminados**: 0 (escopo reduzido).
- **Layouter mutations eliminadas**: 2.
- **Layouter fallbacks eliminados**: 2.
- **Tests netos**: 0.
- **Hashes desactualizados**: 0.

**Achado arquitectural significativo**: P190F é o **primeiro passo M6** que descobre **barreira impondo redesign maior** antes de eliminação completa. ADR-0071 (a propor) cobrirá mecanismo de walk pipeline redesign.
