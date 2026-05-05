# Relatório P190D — Categoria Document metadata (combinação 2 padrões)

**Data**: 2026-05-04
**Estado**: ✅ Completo (10 sub-passos A-J)
**Magnitude**: M (combinação 2 padrões + decisão pragmática de defer `lang`).
**Pattern arquitectural**: ADR-0070 (PROPOSTO P190A) — 3ª aplicação concreta; combinação de **"Layouter-runtime → struct dedicada"** (P190C) + **"eliminação directa via Introspector"** (P190B).

---

## §1 Sumário executivo

P190D fecha **Categoria 3 (Document metadata)** parcialmente:
- **`is_readonly`** movido para `LayouterRuntimeState` (Layouter-runtime). Guard movido de `CounterStateLegacy::step_*`/`update_*` para `counters::layout_counter_update` (Layouter level).
- **`has_outline`** eliminado directamente — caminho Introspector já activo desde P189B (`intr.kind_index.contains_key(&ElementKind::Outline)`).
- **`lang`** **deferido** — descobrir empiricamente que é lido por `compute_labelled` walk fn helper (introspect.rs:360); migração requer walk fn signature change ou helper migration. Defer para passo posterior.

**`CounterStateLegacy`**: 12 → **9 fields** (delta -3 mas só 2 foram eliminados; 1 campo `lang` permanece com nota P190D).

Wait — let me recheck. Empiricamente:
- `has_outline` ✅ eliminado.
- `is_readonly` ✅ movido para LayouterRuntimeState + eliminado de CounterStateLegacy.
- `lang` ❌ **mantido** em CounterStateLegacy (defer).

Então: 12 → **10 fields** (-2). Vou corrigir abaixo.

**Output observable em produção**: inalterado.

---

## §2 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `entities/layouter_runtime_state.rs` | Field `is_readonly: bool` adicionado (3 fields total). |
| 2 | `prompts/entities/layouter_runtime_state.md` | L0 actualizada: 3 fields documentados; `lang` defer notado. |
| 3 | `rules/layout/outline.rs:73-78` | `layouter.counter.is_readonly = true/false` → `layouter.runtime.is_readonly = true/false`. |
| 4 | `rules/layout/counters.rs` | Função `layout_counter_update` recebe `runtime: &LayouterRuntimeState` parameter; guard `if runtime.is_readonly { return; }` movido para aqui. Import `LayouterRuntimeState`. |
| 5 | `rules/layout/mod.rs:394` | Caller actualizado: `counters::layout_counter_update(&mut self.counter, &self.runtime, key, action)`. |
| 6 | `entities/counter_state_legacy.rs:107-138` | Guards `if self.is_readonly { return; }` removidos de `step_hierarchical`, `step_flat`, `update_flat`. Comentário P190D. |
| 7 | `entities/counter_state_legacy.rs:60, 63` | Fields `has_outline: bool` + `is_readonly: bool` eliminados. Comentário P190D substitui. |
| 8 | `entities/counter_state_legacy.rs:tests` | 5 tests sentinela `counter_state_readonly_*` removidos. Comentário P190D. |
| 9 | `rules/layout/tests.rs:1220` | Assertion `!state.has_outline` removida (field eliminado). |
| 10 | `rules/layout/tests.rs:4814+` | Test `outline_migrado_paridade_observable` adaptado (3 assertions sobre `state.has_outline` removidas; cobertura observable via Layouter integration preservada). |

**`lang` NÃO eliminado** — deferido (per achado empírico §6).

---

## §3 Estruturas

### `CounterStateLegacy` redução

| Estado | Fields | Δ |
|--------|--------|---|
| Pré-P190B | 16 | — |
| Pós-P190B | 14 | -2 |
| Pós-P190C | 12 | -2 |
| Pós-P190D | **10** | -2 (`has_outline`, `is_readonly`) |

**`lang` mantido** — deferido. Empiricamente apenas 1 consumer em produção (`compute_labelled`).

### `LayouterRuntimeState` evolução

| Estado | Fields |
|--------|--------|
| P190C (criação) | 2 (`label_pages`, `known_page_numbers`) |
| Pós-P190D | **3** (+`is_readonly`) |

`lang` deferido — não adicionado em P190D.

---

## §4 Tests workspace

| Estado | Total | Δ |
|--------|-------|---|
| Pré-P190D (P190C baseline) | 1.867 | — |
| Pós-P190D | **1.862** | -5 (5 tests sentinela `counter_state_readonly_*` removidos) |

Δ tests **negativo marginal**: tests sentinela legacy redundantes removidos. Cobertura preservada via tests Layouter integration de outline render (DEBT-13).

---

## §5 Verificações finais (.I — 19 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1.862 verdes (Δ vs P190C baseline 1.867: **-5**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | `LayouterRuntimeState` ganha `is_readonly` | ✅ (3 fields) |
| 5 | `CounterStateLegacy.is_readonly` NÃO existe | ✅ |
| 6 | `CounterStateLegacy.has_outline` NÃO existe | ✅ |
| 7 | `CounterStateLegacy.lang` ainda existe | ⚠️ Deferido (vide §6) |
| 8 | `CounterStateLegacy`: 10 fields (era 12) | ✅ |
| 9 | Layouter consumer `is_readonly` migrado (outline.rs:73-78) | ✅ |
| 10 | Consumer `has_outline` migrado (já estava em P189B) | ✅ |
| 11 | Guard movido de `CounterStateLegacy::step_*` para `layout_counter_update` | ✅ |
| 12 | Comentários inline P190D presentes | ✅ |
| 13 | Trait `Introspector` NÃO modificado | ✅ |
| 14 | `TagIntrospector` NÃO modificado | ✅ |
| 15 | Walk arms NÃO modificados | ✅ |
| 16 | Walk arm Outline já era puro desde P189B | ✅ |
| 17 | ADR-0070 PROPOSTO NÃO transitada | ✅ |
| 18 | Snapshot tests verdes | ✅ |
| 19 | Linter passa final | ✅ |

**18/19 verde + 1 deferido** (cláusula 7 — `lang` mantido por decisão empírica documentada §6).

---

## §6 Decisão notável — `lang` deferido

P190A diagnóstico cláusula 3 prescreveu mover `is_readonly` + `lang` para `LayouterRuntimeState`. Empiricamente em P190D auditoria descoberto:

- **`state.lang` é lido durante walk pre-pass** por `compute_labelled` em `introspect.rs:360`:
  ```rust
  Content::Figure { kind, ... } => {
      ...
      let supplement = figure_supplement_for_lang(kind_key, state.lang.as_ref());
      ...
  }
  ```

- **Walk fn não tem acesso a `LayouterRuntimeState`** — `walk` recebe `&mut state: CounterStateLegacy`; mover `lang` para LayouterRuntimeState exige:
  - Mudança de signature de `walk` para passar `&LayouterRuntimeState` ou
  - Migração do helper `compute_labelled` para receber `lang` como parâmetro ou
  - Refactor mais profundo.

- **Em produção `state.lang` é sempre `None`** — não há caller setting it antes de walk. Tests usam helper `introspect_with_lang(content, lang_code)` (introspect.rs:1281) que set manualmente.

**Decisão pragmática P190D**: defer `lang` para passo posterior. Motivos:
- Trabalho excede budget P190D (M+).
- Tests `figure_label_lang_*` dependem do helper `introspect_with_lang` que set `state.lang` directamente; refactor exige migração de tests também.
- Em produção `lang` é sempre None — eliminação não tem impacto observable imediato; defer não é dívida bloqueante.

**Documentação**: comentário P190D em CounterStateLegacy regista decisão.

---

## §7 Decisões de execução notáveis

### Guard movido para Layouter level

P190D fez refactor não-trivial: guard `is_readonly` em `CounterStateLegacy::step_*`/`update_*` movido para `counters::layout_counter_update` (Layouter level). Motivação:
- `is_readonly` é genuinamente Layouter-runtime (set por `outline.rs` durante render).
- Walk arms calling `state.step_*` durante walk pre-pass **nunca têm `is_readonly = true`** em produção (walk pre-pass não toca o flag; só Layouter render toca).
- Manter guard em CounterStateLegacy seria duplicação semântica desnecessária.

Resultado: `layout_counter_update` agora recebe `runtime: &LayouterRuntimeState` como param e faz guard ANTES de chamar counter methods. `CounterStateLegacy::step_*`/`update_*` agora **sem guard** (semântica simplificada).

### Tests sentinela legacy

5 tests `counter_state_readonly_*` em counter_state_legacy.rs removidos:
- Tests testavam guard logic em CounterStateLegacy::step_*/update_*.
- Após P190D, guard é em layout_counter_update.
- Cobertura preservada via tests Layouter integration de outline render (DEBT-13 preservada).

### `has_outline` foi quase trivial

Field `has_outline` já era dead code parcial:
- Mutação `state.has_outline = true` removida em P189B.
- Layouter `mod.rs:1488` já usava `introspector.kind_index.contains_key(&ElementKind::Outline)`.
- Field só permanecia para compatibilidade tests sentinela P189B.

P190D apenas eliminou o field e adaptou as 3 tests assertions.

### Sem cláusula gate substancial disparada

Apesar do refactor de guard logic ter potencial para regressões, tests Layouter integration passaram — guard semantics preservada por construção.

---

## §8 Estado actual

- **P190 série**: A ✅ B ✅ C ✅ D ✅ | E-I pendentes.
- **Categoria 3 (Document metadata) parcialmente fechada** (2/3 fields; `lang` deferido).
- **88 passos executados** (P190C=87 + P190D=88).

---

## §9 Pendências cumulativas

**5 categorias restantes** + 1 deferred:
- Categoria 4 (Numbering active) — P190E.
- Categoria 5 (Counters core + 2 helpers) — P190F.
- Categoria 6 (Labels & TOC) — P190G.
- Categoria 7 (Figures) — P190H.
- Walk arms purification + Layouter final + struct elim + L0 + ADR-0070 ACEITE — P190I.
- **`lang` deferred** — pode ser endereçado em P190F (helper migration) ou P190I (eliminação final). Documentar entry no plano.

**M5 universal completo**: inalterado (0 + 0 + 0).

---

## §10 Próximo passo

**P190E** — Categoria 4 (Numbering active):
- Eliminar `numbering_active: HashMap<String, bool>` de `CounterStateLegacy`.
- Caminho Introspector activo desde P198B + P199B (StateRegistry com chaves `numbering_active:heading` + `numbering_active:equation`).
- Layouter consumers (`equation.rs:33`, `mod.rs:343`, ...) migram para `intr.is_numbering_active_at(...)` location-aware.
- Walk arm Equation gate (`introspect.rs:579`) — read of `state.is_numbering_active("equation")` deferido para P190F (helper migration).
- Magnitude **M**.

---

## §11 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0070 PROPOSTO (P190A).
- **3ª aplicação concreta** P190D.
- **Combinação 2 padrões**: "Layouter-runtime → struct dedicada" (P190C-style para `is_readonly`) + "eliminação directa via Introspector" (P190B-style para `has_outline`).
- **Categoria 3 parcialmente fechada**: Document metadata (2/3 fields).
- **`lang` deferred**: documentado como achado empírico — walk fn signature change requerida para migração; baixa prioridade (None em produção).
- **Pre-condições**: P189B (has_outline mutation removed; Introspector path activo); P190C (LayouterRuntimeState criada).
- **Layouter migration**: 1 consumer (outline.rs) + guard logic refactor (counters.rs).
- **Hash código**: actualizado via `--fix-hashes` (Nothing to fix).
- **Hash L0** introspect.md: `7a3ba2b7` (inalterado em P190D — walk não tocado).
- **Padrão diagnóstico-primeiro**: 25ª aplicação consecutiva.
- **F1 progresso**: 12 → 10 fields ortogonais.
- **F3 progresso**: Layouter ainda 20 fields; LayouterRuntimeState 3 fields (era 2).

---

## §12 Métricas finais P190D

- **Sub-passos**: 10 (A-J).
- **LOC produção**: ~30 (LayouterRuntimeState +1 field; layout_counter_update guard refactor; 1 consumer migration; 2 field eliminations + 3 guard removals; comentários).
- **LOC teste**: ~40 (5 tests sentinela removidos + 3 test adaptations).
- **LOC L0**: ~15 (L0 LayouterRuntimeState actualizada).
- **LOC relatório**: ~280.
- **Variants Content novas**: 0.
- **Sub-stores Introspector novos**: 0.
- **ADRs novas**: 0.
- **Helpers privados**: 0.
- **F1 fields eliminados**: -2 (has_outline, is_readonly).
- **`lang` deferido**: 1 field.
- **LayouterRuntimeState fields**: 2 → 3.
- **Tests netos**: -5.
- **Hashes desactualizados**: 0.

**Pattern combinado**: 1ª aplicação simultânea de 2 padrões em mesmo passo. Padrão arquitectural complementar a ADR-0070 stylesheet "eliminação write paralelo M5".
