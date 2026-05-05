# Relatório P190G — Categoria Labels & TOC eliminada (1ª aplicação ADR-0071)

**Data**: 2026-05-05
**Magnitude**: M (real M+).
**Estado**: Completo. Categoria 6 eliminada — 4 fields removidos.
**Pattern**: 6ª aplicação "eliminação write paralelo M5"; 1ª aplicação directa do mecanismo ADR-0071 em P190 série.

---

## §1 Sumário executivo

P190G retoma série P190 após pausa P190F + ramo paralelo P191A-C
(ADR-0071 ACEITE). 1ª aplicação directa do mecanismo — walk arms
purificadas para 4 fields da categoria Labels & TOC; consumers
walk-side migrados para Introspector path puro durante P191B/C;
fields legacy eliminados sem deixar walk readers.

**Eliminados** (Opção α + Caso 1):
- `resolved_labels: HashMap<Label, String>` — caminho Introspector
  via Tag::Labelled pós-recursão (P195D + P196B).
- `headings_for_toc: Vec<(Label, Content, usize)>` — caminho
  Introspector via Tag::HeadingForToc pós-recursão (P200B).
- `auto_label_counter: usize` — substituído por local var threaded
  via walk fn parameter `auto_label_counter: &mut usize`.
- `numbering_active: HashMap<String, bool>` (Caso 1 `.H`) — caminho
  Introspector via populate_intr arm StateUpdate (P191B).

**4 mutações walk arm eliminadas**:
- Walk arm Heading: `state.resolved_labels.insert(auto_label, ...)`.
- Walk arm Heading: `state.headings_for_toc.push(...)`.
- Walk arm Heading: `state.auto_label_counter += 1` → local var.
- Walk arm Labelled: `state.resolved_labels.insert(label, text)`.
- Walk arm SetHeadingNumbering: `state.numbering_active.insert("heading", _)`.
- Walk arm SetEquationNumbering: `state.numbering_active.insert("equation", _)`.

**Layouter consumers migrados**:
- `references.rs:64`: fallback `counter.resolved_labels` eliminado;
  Introspector path puro.
- `outline.rs:38`: fallback `counter.headings_for_toc` eliminado;
  Introspector path puro.
- `mod.rs:1486-1487, 1524-1525`: assignments duais eliminados.
- `counters.rs`: helpers `layout_set_heading_numbering` e
  `layout_set_equation_numbering` eliminados; Layouter arms no-op.

**`CounterStateLegacy`: 10 → 6 fields**.

---

## §2 Estado actual confirmado

| Item | Estado |
|------|--------|
| `cargo check --workspace` | passa |
| `cargo test --workspace --lib` | 1812 verdes (1573 + 215 + 24) |
| `crystalline-lint .` | 0 violations |
| `CounterStateLegacy.resolved_labels` | **eliminado** |
| `CounterStateLegacy.headings_for_toc` | **eliminado** |
| `CounterStateLegacy.auto_label_counter` | **eliminado** |
| `CounterStateLegacy.numbering_active` | **eliminado** (Caso 1) |
| `CounterStateLegacy.is_numbering_active(...)` method | **eliminado** |
| Walk fn signature | 7 params (+`auto_label_counter: &mut usize`) |
| Walk-internal helpers (compute_figure, compute_heading_for_toc) | preservados (compute_heading_for_toc adaptado para receber parameter) |
| Layouter assignments duais | eliminados |
| Layouter fallbacks references.rs/outline.rs | eliminados |
| `CounterStateLegacy` fields | 6 (10 → 6, Δ -4) |

---

## §3 20 verificações `.J`

| # | Verificação | Estado |
|---|-------------|--------|
| 1 | `cargo check --workspace` passa | ✅ |
| 2 | `cargo test --workspace --lib` passa Δ marginal | ✅ 1812 verdes (vs 1812 P191C; +0 net) |
| 3 | `crystalline-lint .` zero violations | ✅ |
| 4 | `CounterStateLegacy.resolved_labels` NÃO existe | ✅ |
| 5 | `CounterStateLegacy.headings_for_toc` NÃO existe | ✅ |
| 6a | Opção α: `auto_label_counter` NÃO existe | ✅ |
| 6b | Walk fn tem `auto_label_counter: &mut usize` parameter | ✅ |
| 6c | Helpers `compute_heading_*` recebem auto_label_n parameter | ✅ |
| 8 | Caso 1: `numbering_active` NÃO existe | ✅ |
| 8b | Mutações walk arm SetHeadingNumbering/SetEquationNumbering eliminadas | ✅ |
| 8c | Walk arms canonical SetXNumbering ficam puros | ✅ |
| 10 | `CounterStateLegacy`: 6 fields | ✅ |
| 11 | Walk arm Labelled mutação `resolved_labels` eliminada | ✅ |
| 12 | Walk arm Heading mutação `headings_for_toc` eliminada | ✅ |
| 13 | Layouter assignments duais eliminados | ✅ |
| 14 | Comentários inline P190G presentes | ✅ |
| 15 | Trait `Introspector` NÃO modificado | ✅ |
| 16 | `TagIntrospector` fields NÃO modificados | ✅ |
| 17 | 2 helpers walk-internal: compute_heading_for_toc adaptado (parameter); compute_figure inalterado | ✅ |
| 18 | ADR-0070 PROPOSTO NÃO transitada | ✅ ACEITE em P190I |
| 19 | Snapshot tests verdes | ✅ |
| 20 | Linter passa final | ✅ |

20/20 ✅.

---

## §4 Δ tests vs baseline P191C

| Categoria | Pré-P190G | Pós-P190G | Δ |
|-----------|-----------|-----------|---|
| Tests workspace lib | 1812 | 1812 | 0 |
| Tests adaptados (state→intr migrações) | — | ~25 | adaptados |
| Tests removidos sentinela legacy | — | ~3 | redundantes (substituídos por equivalent intr) |
| **Net Δ** | — | — | **0 marginal** |

---

## §5 Decisões de execução notáveis

### §5.1 Opção α vs β em `.D` (`auto_label_counter`)

**Decisão**: Opção α — eliminar field; threading via parameter.

**Razão**: walk recursion exige que o counter seja accessível ao
longo da pilha de chamadas. Local var sozinha não funciona.
Solução: parameter `auto_label_counter: &mut usize` adicionado a
walk fn (7º parameter). Caller (entry points) declaram local var
e passam `&mut`.

**Trade-off**: walk fn signature ganha +1 parameter (5 → 6 → 7
desde P191B + P190G). Aceitável — refactor mecânico via sed para
todos os call sites.

### §5.2 Caso 1 vs Caso 2 em `.H` (`numbering_active`)

**Decisão**: Caso 1 — `numbering_active` ELIMINADO.

**Empírico**: nenhum walk reader de `state.numbering_active` após
P191B (walk arm Equation gate migrado) e P191B/C (helpers migrados
para `intr.is_numbering_active_at`). Fields legacy redundantes;
elimináveis sem dependência adicional.

### §5.3 Padrão "1ª aplicação directa ADR-0071 em P190" estabelecido

P190G é primeira aplicação directa do mecanismo P191/ADR-0071 em
P190 série. Trabalho concreto:
- Fields walk-readable previamente bloqueados por barreira P190F.
- Após P191B/C migrarem helpers e gates para Introspector
  location-aware, fields ficam dead writes.
- P190G elimina dead writes + fields + Layouter consumers
  associados.

Padrão aplicável a P190H (Figures) e P190I (struct elim) — 2 e 3ª
aplicações futuras.

### §5.4 Layouter consumers — modificação intencional

Per restrição: "**Não** modificar Layouter — P190I excepto se
consumers desta categoria existirem". Layouter tinha consumers
desta categoria (`references.rs:64` fallback resolved_labels;
`outline.rs:38` fallback headings_for_toc), pelo que modificação é
**autorizada e necessária**:

- `references.rs:64`: `or_else(|| layouter.counter.resolved_labels.get(...))`
  fallback ELIMINADO. Substitution-with-fallback (P194B) colapsa
  em Introspector path puro.
- `outline.rs:38`: `else { layouter.counter.headings_for_toc.clone() }`
  fallback ELIMINADO. Introspector path puro.
- `mod.rs:1486-1487, 1524-1525`: 4 assignments
  `l.counter.X = initial_state.X` eliminados (fields já não
  existem).
- `counters.rs`: 2 helpers
  (`layout_set_heading_numbering`/`layout_set_equation_numbering`)
  eliminados; Layouter arms `Content::SetXNumbering` no-op.

---

## §6 Estado activo vs preservado

### Activado em P190G

- ✅ Walk fn aceita 7 params (`+auto_label_counter: &mut usize`).
- ✅ Helper `compute_heading_for_toc` adaptado para receber
  `auto_label_n: usize` parameter (walk-internal).
- ✅ Layouter `references.rs:64` Introspector path puro.
- ✅ Layouter `outline.rs:38` Introspector path puro.
- ✅ Walk arms canonical SetHeadingNumbering/SetEquationNumbering
  PUROS (sem mutação legacy).

### Preservado

- 2 helpers walk-internal (compute_figure inalterado;
  compute_heading_for_toc adaptado para parameter).
- Pattern ADR-0069 stylesheet (5 variantes operacionais).
- Pattern ADR-0070 stylesheet ("eliminação write paralelo M5"; 6ª
  aplicação P190G).
- ADR-0071 mecanismo (walk fn aceita TagIntrospector).
- Trait `Introspector` 20 métodos (inalterado).
- `TagIntrospector` 9 sub-stores (inalterado).
- `LayouterRuntimeState` 3 fields (inalterado).

---

## §7 Métricas finais P190G

- **LOC produção líquido**: -614 (1248 insertions − 1862 deletions
  per `git diff --stat`; cumulativo desde P191A).
- **LOC produção P190G específico**: ~-200 (eliminação fields + 4
  mutações walk arm + 2 helpers Layouter + 4 assignments + 2
  fallbacks).
- **LOC tests**: ~25 tests adaptados; ~3 tests sentinela removidos.
- **LOC L0**: 0 (defer P190I).
- **Variants Content novas**: 0.
- **Sub-stores novos**: 0.
- **ADRs novas**: 0 (ADR-0070 PROPOSTO; ACEITE em P190I).
- **Helpers privados novos**: 0; 1 adaptado (`compute_heading_for_toc`).
- **Helpers Layouter eliminados**: 2
  (`layout_set_heading_numbering`, `layout_set_equation_numbering`).
- **Fields eliminados**: 4 (resolved_labels, headings_for_toc,
  auto_label_counter, numbering_active).
- **Métodos eliminados**: 1 (`is_numbering_active`).
- **F1 progresso**: 10 → 6 fields. Δ -4. Faltam 4 (figure_numbers,
  figure_label_numbers, local_figure_counters, lang).
- **F3 progresso**: Layouter ainda 20 fields (assignments
  eliminados mas struct inalterado; defer P190I).

---

## §8 Estado actual

- **P190 série**: A ✅ B ✅ C ✅ D ✅ E ✅ F ⚠️ G ✅ | H-I pendentes.
- **P191 série** (ramo paralelo): A ✅ B ✅ C ✅ — fechada.
- **Categoria 6 (Labels & TOC) fechada**.
- **95 passos executados** (P191C=94 + P190G=95).
- **Pattern "eliminação write paralelo M5"**: 6ª aplicação concreta.
- **1ª aplicação directa ADR-0071 mecanismo em P190 série**.

---

## §9 Pendências cumulativas

### Restante M6

- **P190H** — Categoria 7 (Figures). Magnitude M.
  Trabalho:
  - Eliminar fields `figure_numbers`, `figure_label_numbers`,
    `local_figure_counters`.
  - Possivelmente resolver defer `lang` se walk arm Labelled
    deixar de o ler (após Figure migration).
  - 2ª aplicação directa ADR-0071 mecanismo em P190.

- **P190I** — Walk arms purification + Layouter final + struct
  elim + ADR-0070 ACEITE. Magnitude M+.
  Trabalho:
  - Eliminar `CounterStateLegacy` struct completamente.
  - Layouter ganha `counter` field eliminado (F3 parcialmente
    fecha).
  - L0 update.
  - ADR-0070 ACEITE.

### Defers remanescentes

- `lang` (P190D) — ainda passado via parameter; eliminação em P190H/I.
- `flat`, `hierarchical` (P190F) — fields privados; eliminação em P190I.
- ~~`numbering_active`~~ — **resolvido em P190G Caso 1**.

### F1, F3

- F1 fecha após P190I (struct eliminado).
- F3 parcialmente fecha após P190I (Layouter `counter` field
  eliminado).

---

## §10 Próximo passo

**P190H** — Categoria 7 (Figures). Magnitude M.

Trabalho:
- Eliminar 3 fields da categoria Figures.
- 2ª aplicação directa ADR-0071 mecanismo em P190.
- Possível resolução defer `lang`.

---

## §11 Linhagem

- **Pattern arquitectural eliminação**: ADR-0070 PROPOSTO P190A;
  ACEITE projectada P190I.
- **Pattern arquitectural pre-condition**: ADR-0071 ACEITE P191C —
  mecanismo walk pipeline com Introspector accessible.
- **5 variantes operacionais ADR-0069**: preservadas.
- **6ª aplicação eliminação write paralelo M5** (ADR-0070
  stylesheet): 1ª P190B (Bibliography); 2ª P190C (Page tracking);
  3ª P190D (Document metadata); 4ª P190E (Numbering active —
  Layouter side); 5ª P190F (Counters core — Layouter side); **6ª
  P190G (Labels & TOC — Walk side)**.
- **Pattern stylesheet "diagnóstico-primeiro"**: 30ª aplicação
  consecutiva.
- **F1**: 10 → 6 fields. Faltam 4. F1 fecha após P190I.
- **F3**: Layouter inalterado em P190G (assignments eliminados;
  struct fica pendente até P190I).
