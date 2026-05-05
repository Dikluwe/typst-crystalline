# Relatório P190H — Categoria Figures eliminada (2ª aplicação ADR-0071)

**Data**: 2026-05-05
**Magnitude**: M.
**Estado**: Completo. Categoria 7 eliminada — 3 fields removidos.
**Pattern**: 7ª aplicação "eliminação write paralelo M5"; 2ª aplicação directa do mecanismo ADR-0071 em P190 série.

---

## §1 Sumário executivo

P190H elimina categoria 7 (Figures) — 3 fields legacy + helper
`compute_figure` orphan. Walk arm Figure ficou puro (apenas desce
em body+caption). Layouter consumers C2/C3 migrados para Introspector
path puro (sem fallbacks substitution-with-fallback).

**Eliminados** (Opção α + Caso 2):
- `figure_numbers: HashMap<String, Vec<usize>>` — caminho
  Introspector via `intr.counters["figure:{kind}"]` (sub-store
  CounterRegistry; populated por populate_intr arm Figure P191C
  gated por is_counted).
- `figure_label_numbers: HashMap<Label, usize>` — caminho
  Introspector via `intr.figure_label_numbers` (populated por
  populate_intr arms Figure + Labelled).
- `local_figure_counters: HashMap<String, usize>` — walk-internal
  eliminado; helper `compute_figure` orphan removido (intr.counters
  já tem dado equivalente).
- Helper `compute_figure` ELIMINADO (orphan após eliminação dos
  consumers walk-side).

**3 mutações walk arm eliminadas**:
- Walk arm Figure: `state.figure_numbers.entry(...).push(...)`.
- Walk arm Figure: `state.local_figure_counters.entry(...) += 1`.
- Walk arm Labelled: `state.figure_label_numbers.insert(...)`.

**Layouter consumers migrados**:
- `references.rs:51`: fallback `counter.figure_label_numbers`
  ELIMINADO.
- `mod.rs:507-510`: fallback `counter.figure_numbers` ELIMINADO;
  `figure_number_at_index` é única fonte (rede de segurança final
  `unwrap_or(idx + 1)` preservada).

**`CounterStateLegacy`: 6 → 3 fields**.

**Defer `lang` (Caso 2)**: continua em CounterStateLegacy. Walk arm
Labelled passa-o ao helper `compute_labelled` via parameter (per
P191C Opção β). Eliminação completa em P190I.

---

## §2 Estado actual confirmado

| Item | Estado |
|------|--------|
| `cargo check --workspace` | passa (4 warnings pré-existentes) |
| `cargo test --workspace --lib` | 1812 verdes (1573 + 215 + 24) |
| `crystalline-lint .` | 0 violations |
| `CounterStateLegacy.figure_numbers` | **eliminado** |
| `CounterStateLegacy.figure_label_numbers` | **eliminado** |
| `CounterStateLegacy.local_figure_counters` | **eliminado** |
| Helper `compute_figure` | **eliminado** (orphan) |
| Walk arm Figure | puro (3 mutações eliminadas) |
| Walk arm Labelled | mutação `figure_label_numbers.insert` eliminada |
| Layouter `references.rs:51` fallback | eliminado |
| Layouter `mod.rs:507-510` fallback | eliminado |
| `CounterStateLegacy` fields | 3 (6 → 3, Δ -3) |
| Defer `lang` | preservado (Caso 2; defer P190I) |

---

## §3 20 verificações `.J`

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace --lib` passa Δ marginal | ✅ 1812 verdes |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | `CounterStateLegacy.figure_numbers` NÃO existe | ✅ |
| 5 | `CounterStateLegacy.figure_label_numbers` NÃO existe | ✅ |
| 6 | Opção α: `local_figure_counters` NÃO existe | ✅ |
| 6b | Helper `compute_figure` ELIMINADO | ✅ |
| 8 | Caso 2: `lang` preservado | ✅ |
| 10 | `CounterStateLegacy`: 3 fields | ✅ |
| 11 | Walk arm Figure 3 mutações eliminadas | ✅ |
| 12 | Layouter consumers Figure migrados | ✅ |
| 13 | Layouter fallbacks Figure eliminados | ✅ |
| 14 | Layouter assignments duais | ✅ (none — fields já não existem) |
| 15 | Comentários inline P190H presentes | ✅ |
| 16 | Trait `Introspector` NÃO modificado | ✅ |
| 17 | `TagIntrospector` fields NÃO modificados | ✅ |
| 18 | ADR-0070 PROPOSTO NÃO transitada | ✅ ACEITE em P190I |
| 19 | Snapshot tests verdes | ✅ |
| 20 | Linter passa final | ✅ |

20/20 ✅.

---

## §4 Δ tests vs baseline P190G

| Categoria | Pré-P190H | Pós-P190H | Δ |
|-----------|-----------|-----------|---|
| Tests workspace lib | 1812 | 1812 | 0 |
| Tests adaptados (state→intr) | — | ~10 | adaptados |
| Tests sentinela legacy removidos | — | ~3 | redundantes (substituídos por intr) |
| **Net Δ** | — | — | **0 marginal** |

---

## §5 Decisões de execução notáveis

### §5.1 Opção α em `.D` (`local_figure_counters`)

**Decisão**: Opção α — eliminar field + helper `compute_figure`.

**Razão**: populate_intr arm Figure (P191C) já popula
`intr.counters["figure:{kind}"]` no momento da Tag emission (gated
por is_counted). Walk arm Figure não precisa de calcular figure
number durante walk — o número está disponível via `intr.flat_counter_at`
para consumers (`compute_labelled`) ou `figure_number_at_index`
para Layouter. compute_figure helper torna-se orphan.

**Trade-off**: walk fn signature inalterada (não precisa de 8º
parameter). Walk arm Figure body simplifica drasticamente:
```rust
Content::Figure { body, caption, .. } => {
    walk(body, state, locator, tags, intr, auto_label_counter, None);
    if let Some(cap) = caption {
        walk(cap, state, locator, tags, intr, auto_label_counter, None);
    }
}
```

### §5.2 Caso 2 em `.H` (defer `lang`)

**Decisão**: Caso 2 — `lang` preservado em CounterStateLegacy.

**Razão**: walk arm Labelled lê `state.lang` para passar ao helper
`compute_labelled` (per P191C Opção β). Eliminar `lang` exigiria
walk fn signature change (8º parameter `lang: Option<&Lang>`).
Magnitude marginal isolado, mas combinado com P190I struct elim
fica mais limpo. Defer P190I.

**Trade-off**: 1 field permanece em CounterStateLegacy (`lang`)
após P190H. P190I elimina struct inteiro.

### §5.3 Padrão "2ª aplicação directa ADR-0071" estabelecido

P190G foi 1ª aplicação directa do mecanismo ADR-0071 em P190 série
(categoria Labels & TOC). P190H é 2ª aplicação (categoria Figures).
P190I será 3ª e final (struct elim).

Padrão consolidado:
- Walk arm puro após eliminação de fields legacy.
- populate_intr arms cobrem todos os sub-stores antes lidos via state.
- Layouter consumers migrados para Introspector path puro
  (substitution-with-fallback colapsa).
- Helpers walk-side adaptam signature ou ficam orphan (eliminados).

---

## §6 Estado activo vs preservado

### Activado em P190H

- ✅ Walk arm Figure puro (apenas desce em body+caption).
- ✅ Walk arm Labelled mutação `figure_label_numbers.insert`
  eliminada.
- ✅ Layouter `references.rs:51` Introspector path puro.
- ✅ Layouter `mod.rs:507-510` Introspector path puro com rede de
  segurança final preservada.

### Preservado

- 1 helper walk-internal: `compute_heading_for_toc` (P200B/P190G
  adaptado).
- Pattern ADR-0069 stylesheet (5 variantes operacionais).
- Pattern ADR-0070 stylesheet (7ª aplicação concreta P190H).
- ADR-0071 mecanismo (walk fn aceita TagIntrospector).
- Trait `Introspector` 20 métodos (inalterado).
- `TagIntrospector` 9 sub-stores (inalterado).
- `LayouterRuntimeState` 3 fields (inalterado).
- `CounterStateLegacy.lang` (Caso 2 defer P190I).

### Eliminado

- Helper `compute_figure` (orphan após walk arm Figure puro).
- 3 fields CounterStateLegacy (figure_numbers, figure_label_numbers,
  local_figure_counters).
- 4 helpers Layouter já eliminados em P190G/P190H combinados.

---

## §7 Métricas finais P190H

- **LOC produção líquido**: estimado -100 (eliminação fields +
  walk arm body + helper compute_figure + Layouter fallbacks).
- **LOC tests**: ~10 tests adaptados; ~3 sentinela removidos.
- **LOC L0**: 0 (defer P190I).
- **Variants Content novas**: 0.
- **Sub-stores novos**: 0.
- **ADRs novas**: 0.
- **Helpers privados eliminados**: 1 (`compute_figure`).
- **Layouter fallbacks eliminados**: 2 (references.rs:51,
  mod.rs:507-510).
- **Fields eliminados**: 3 (figure_numbers, figure_label_numbers,
  local_figure_counters).
- **F1 progresso**: 6 → 3 fields. Δ -3. Faltam 3 fields
  (hierarchical, flat, lang) para eliminação total em P190I.
- **F3 progresso**: Layouter ainda 20 fields; struct elim em P190I.

---

## §8 Estado actual

- **P190 série**: A ✅ B ✅ C ✅ D ✅ E ✅ F ⚠️ G ✅ H ✅ | I pendente.
- **P191 série** (ramo paralelo): A ✅ B ✅ C ✅ — fechada.
- **Categoria 7 (Figures) fechada**.
- **96 passos executados** (P190G=95 + P190H=96).
- **Pattern "eliminação write paralelo M5"**: 7ª aplicação concreta.
- **2ª aplicação directa ADR-0071 mecanismo em P190 série**.

---

## §9 Pendências cumulativas

### Restante M6

- **P190I** — **PASSO FINAL M6**. Magnitude M+.
  Trabalho:
  - Eliminar `CounterStateLegacy` struct completamente.
  - Eliminar fields restantes (hierarchical, flat, lang).
  - Layouter `counter` field eliminado (F3 parcialmente fecha).
  - L0 update final.
  - ADR-0070 PROPOSTO → ACEITE.
  - **F1 fecha**.
  - **M6 fechado**.

### Defers remanescentes

- `lang` (P190D) — preserved Caso 2; eliminado em P190I.
- `flat`, `hierarchical` (P190F) — fields privados; eliminação em P190I.
- ~~`numbering_active`~~ — resolvido em P190G.
- ~~`figure_numbers`/`figure_label_numbers`/`local_figure_counters`~~
  — resolvidos em P190H.

### F1, F3

- F1 (`CounterStateLegacy` 16 fields heterogéneos): fecha após P190I.
- F3 (Layouter 19 fields): parcialmente fecha após P190I.

---

## §10 Próximo passo

**P190I** — **PASSO FINAL M6**. Magnitude M+.

Trabalho concreto:
- Eliminar 3 fields restantes + struct.
- Layouter struct refactor (eliminar `counter` field).
- Walk fn signature simplifica (state pode ser eliminado ou
  reduzido).
- L0 update final.
- ADR-0070 ACEITE.
- F1 fecha. M6 completo.

---

## §11 Linhagem

- **Pattern arquitectural eliminação**: ADR-0070 PROPOSTO P190A;
  ACEITE projectada P190I.
- **Pattern arquitectural pre-condition**: ADR-0071 ACEITE P191C —
  mecanismo walk pipeline com Introspector accessible.
- **5 variantes operacionais ADR-0069**: preservadas.
- **7ª aplicação eliminação write paralelo M5** (ADR-0070
  stylesheet): 1ª P190B (Bibliography); 2ª P190C (Page tracking);
  3ª P190D (Document metadata); 4ª P190E (Numbering active —
  Layouter side); 5ª P190F (Counters core — Layouter side); 6ª
  P190G (Labels & TOC — Walk side); **7ª P190H (Figures — Walk
  side + Helper elim)**.
- **Pattern stylesheet "diagnóstico-primeiro"**: 31ª aplicação
  consecutiva.
- **F1**: 6 → 3 fields. Faltam 3. F1 fecha após P190I.
- **F3**: Layouter inalterado em P190H (struct fica pendente até
  P190I).
