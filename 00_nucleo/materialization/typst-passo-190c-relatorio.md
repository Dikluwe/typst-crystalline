# Relatório P190C — Categoria Page tracking + LayouterRuntimeState

**Data**: 2026-05-04
**Estado**: ✅ Completo (10 sub-passos A-J)
**Magnitude**: M+ (cria struct nova + Layouter field; primeira aplicação padrão "Layouter-runtime → struct dedicada").
**Pattern arquitectural**: ADR-0070 (PROPOSTO P190A) — 2ª aplicação concreta do pattern stylesheet "eliminação write paralelo M5"; **1ª aplicação do padrão "Layouter-runtime → struct dedicada"**.

---

## §1 Sumário executivo

P190C fecha **Categoria 2 (Page tracking)** do plano M6:
- Struct nova `LayouterRuntimeState` criada em `entities/layouter_runtime_state.rs`.
- 2 fields movidos: `label_pages` + `known_page_numbers`.
- Layouter ganha field `runtime: LayouterRuntimeState`.
- 4 consumers Layouter migrados (`references.rs:30`, `outline.rs:51`, `mod.rs:1139`, `mod.rs:1535`).
- Fields eliminados de `CounterStateLegacy`.

**Pattern arquitectural novo**: "Layouter-runtime → struct dedicada" — para campos que não cabem em Introspector porque são populated durante layout (não derivados de Content pre-pass).

**Output observable em produção**: inalterado.

---

## §2 Trabalho concreto

| # | Ficheiro | Mudança |
|---|----------|---------|
| 1 | `entities/layouter_runtime_state.rs` | **Novo ficheiro** — struct `LayouterRuntimeState` com 2 fields. |
| 2 | `prompts/entities/layouter_runtime_state.md` | **Novo L0** — documentação do padrão. |
| 3 | `entities/mod.rs` | Adicionar `pub mod layouter_runtime_state;`. |
| 4 | `rules/layout/mod.rs` (Layouter struct) | Field novo `pub runtime: LayouterRuntimeState`; inicialização em `Layouter::new()`. |
| 5 | `rules/layout/references.rs:30` | `self.counter.label_pages.insert(...)` → `self.runtime.label_pages.insert(...)`. |
| 6 | `rules/layout/outline.rs:51` | `layouter.counter.known_page_numbers.get(...)` → `layouter.runtime.known_page_numbers.get(...)`. |
| 7 | `rules/layout/mod.rs:1139` | `doc.extracted_label_pages = self.counter.label_pages` → `self.runtime.label_pages`. |
| 8 | `rules/layout/mod.rs:1535` | `l.counter.known_page_numbers = ...` → `l.runtime.known_page_numbers = ...`. |
| 9 | `entities/counter_state_legacy.rs:46-53` | Fields `label_pages` + `known_page_numbers` eliminados. Comentário P190C documenta migração. |

**Walk arms NÃO modificados** — estes campos não eram tocados por walk (Layouter-runtime apenas).
**`from_tags`/Trait `Introspector`/`TagIntrospector` NÃO modificados** — estes campos não cabem em Introspector.

---

## §3 Estruturas

### `CounterStateLegacy` redução

| Estado | Fields | Δ |
|--------|--------|---|
| Pré-P190B | 16 | — |
| Pós-P190B | 14 | -2 (`bib_entries`, `bib_numbers`) |
| Pós-P190C | **12** | -2 (`label_pages`, `known_page_numbers`) |

### `Layouter<M, S>` evolução

| Estado | Fields | Δ |
|--------|--------|---|
| Pré-P190C | 19 | — |
| Pós-P190C | **20** | +1 (`runtime: LayouterRuntimeState`) |

**Nota**: F3 (Layouter 19 fields) ganhou +1 field temporariamente. Conceptualmente: trade-off — campo `counter` (em redução, 16→12 fields) será eliminado em P190I, ficando apenas `runtime` que é struct nova com 2-4 campos limitados (P190C 2 + P190D 2). Magnitude F3 melhora.

### `LayouterRuntimeState` (novo)

```rust
pub struct LayouterRuntimeState {
    pub label_pages: HashMap<Label, usize>,
    pub known_page_numbers: HashMap<Label, usize>,
}
```

2 fields. Replicado em P190D para incluir `is_readonly` + `lang` (4 fields total).

---

## §4 Tests workspace

| Estado | Total | Δ |
|--------|-------|---|
| Pré-P190C (P190B baseline) | 1.867 | — |
| Pós-P190C | **1.867** | **0** (sem regressão; sem remoção de tests sentinela) |

**Δ tests**: 0. Tests Layouter dependentes não regrediram porque mudança é apenas no path (`.counter.X` → `.runtime.X`); semântica preservada.

---

## §5 Verificações finais (.I — 18 checks)

| # | Verificação | Resultado |
|---|-------------|-----------|
| 1 | `cargo check --workspace` | ✅ ok |
| 2 | `cargo test --workspace` | ✅ 1.867 verdes (Δ vs P190B baseline 1.867: **0**) |
| 3 | `crystalline-lint .` | ✅ 0 violations |
| 4 | Struct nova `LayouterRuntimeState` existe | ✅ |
| 5 | Layouter tem field `runtime: LayouterRuntimeState` | ✅ |
| 6 | `CounterStateLegacy.label_pages` NÃO existe | ✅ |
| 7 | `CounterStateLegacy.known_page_numbers` NÃO existe | ✅ |
| 8 | `CounterStateLegacy`: 12 fields (era 14) | ✅ |
| 9 | Layouter consumer `label_pages` migrados (2: references.rs:30, mod.rs:1139) | ✅ |
| 10 | Layouter consumer `known_page_numbers` migrados (2: outline.rs:51, mod.rs:1535) | ✅ |
| 11 | Layouter assignments duais eliminados | ✅ (era 1 — `mod.rs:1535` agora aponta para `runtime`) |
| 12 | Comentários inline P190C presentes | ✅ |
| 13 | Trait `Introspector` NÃO modificado | ✅ |
| 14 | `TagIntrospector` NÃO modificado | ✅ |
| 15 | Walk arms NÃO modificados | ✅ |
| 16 | ADR-0070 PROPOSTO NÃO transitada | ✅ (ACEITE em P190I) |
| 17 | Snapshot tests verdes | ✅ |
| 18 | Linter passa final | ✅ |

**18/18 verde.**

---

## §6 Decisões de execução notáveis

### Nome da struct

**Opção α confirmada**: `LayouterRuntimeState`. Clareza explícita sobre origem (Layouter-runtime, não derivada de Content pre-pass). Distingue de `TagIntrospector` (Introspector-derived).

### Type signatures empíricos

- `label_pages: HashMap<Label, usize>` — confirmado em P190C audit.
- `known_page_numbers: HashMap<Label, usize>` — confirmado.

Ambos `HashMap<Label, usize>` (símétrico) — sem complexidade de lifetimes.

### Padrão arquitectural estabelecido

**"Layouter-runtime → struct dedicada"** introduzido por P190C como padrão para campos que não cabem em Introspector. Replicado em P190D (`is_readonly` + `lang`). Critério de aplicação:
- Field é populated durante layout (não derivado de Content pre-pass).
- Field não tem cobertura natural em sub-stores Introspector.
- Field tem semântica Layouter-only.

### Sem remoção de tests sentinela

Diferente de P190B (que removeu 2 tests sentinela bib_numbers redundantes), P190C não removeu tests. Razão: fields `label_pages`/`known_page_numbers` são Layouter-runtime — tests dependentes via `layouter.X` continuam válidos com path migrado.

### Sem cláusula gate disparada

P190B previu cláusulas gate possíveis (signature complexo, consumers em sítios não previstos). **Nenhuma disparada** — auditoria empírica P190C identificou todos os 5 sítios upfront.

---

## §7 Estado actual

- **P190 série**: A ✅ B ✅ C ✅ | D-I pendentes.
- **Categoria 2 (Page tracking) fechada estruturalmente**.
- **Pattern "Layouter-runtime → struct dedicada" estabelecido**.
- **87 passos executados** (P190B=86 + P190C=87).

---

## §8 Pendências cumulativas

**6 categorias restantes** do plano M6:
- Categoria 3 (Document metadata: `is_readonly` + `lang` + `has_outline`) — P190D.
- Categoria 4 (Numbering active) — P190E.
- Categoria 5 (Counters core + 2 helpers) — P190F.
- Categoria 6 (Labels & TOC) — P190G.
- Categoria 7 (Figures) — P190H.
- Walk arms purification + Layouter final + struct elim + L0 + ADR-0070 ACEITE — P190I.

**M5 universal completo**: inalterado (0 + 0 + 0).

---

## §9 Próximo passo

**P190D** — Categoria 3 (Document metadata):
- `is_readonly` + `lang` movem para `LayouterRuntimeState` (+2 fields → struct passa a 4 fields).
- `has_outline` é caso diferente — eliminado directamente via `intr.kind_index[Outline]` (P178 substituição).
- 2ª aplicação do padrão "Layouter-runtime → struct dedicada".
- Magnitude **M**.

---

## §10 Linhagem

- **Pattern arquitectural stylesheet**: ADR-0070 PROPOSTO (P190A).
- **2ª aplicação concreta** P190C.
- **Padrão novo introduzido**: "Layouter-runtime → struct dedicada" — replicado em P190D.
- **Categoria 2 fechada**: Page tracking.
- **Pre-condição**: P190B (Bibliography eliminada).
- **Layouter migration**: 4 consumers + 1 assignment block.
- **Hash código** mod.rs Layouter: actualizado via `--fix-hashes` (Nothing to fix — ainda não havia hash drift).
- **Hash L0** introspect.md: `7a3ba2b7` (inalterado em P190C — walk não tocado).
- **Padrão diagnóstico-primeiro**: 24ª aplicação consecutiva.
- **F1 progresso**: 14 → 12 fields ortogonais.
- **F3 progresso**: Layouter 19 → 20 fields (delta +1; conceptual: 2 categorias claras agora — Introspector-derived via `counter` em redução; Layouter-runtime via `runtime`).

---

## §11 Métricas finais P190C

- **Sub-passos**: 10 (A-J).
- **LOC produção**: ~50 (struct nova + Layouter field + 4 consumer migrations + 2 field eliminations + comentários).
- **LOC teste**: 0 (sem adaptação necessária — semântica preservada).
- **LOC L0**: ~50 (L0 novo `entities/layouter_runtime_state.md`).
- **LOC relatório**: ~250.
- **Variants Content novas**: 0.
- **Sub-stores Introspector novos**: 0.
- **ADRs novas**: 0 (ADR-0070 PROPOSTO já criada).
- **Helpers privados**: 0 mudanças.
- **F1 fields eliminados**: -2 (label_pages, known_page_numbers).
- **Layouter fields**: 19 → 20 (+1 runtime).
- **Tests netos**: 0.
- **Hashes desactualizados**: 0.

**Pattern "Layouter-runtime → struct dedicada"**: 1ª aplicação. Padrão arquitectural complementar a ADR-0070 stylesheet "eliminação write paralelo M5".
