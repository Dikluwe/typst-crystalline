# Prompt L0 — `entities/sealed-positions`
Hash do Código: 94c68ba8

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/sealed_positions.rs`
**Criado em**: 2026-05-07 (P205B sub-passo .B —
materializa sealing infrastructure F3 per ADR-0074).
**ADRs relevantes**: ADR-0074 (F3 sealing), ADR-0073
(M8 ACEITE — paridade `#[comemo::track]`), ADR-0066
(SUPERSEDED-BY 0073).

---

## Contexto

ADR-0074 PROPOSTO (F3 — Layouter sub-stores trackable)
fixou per P205A C1+C2+C3+C4:

- **C1 = Mínimo** — 1 sub-store sealed: `positions`.
- **C2 = Híbrido sealing post-iteração** — populated
  single-pass durante layout; sealed no fim.
- **C3 = Padrão A literal** — `#[comemo::track]` em
  struct concreta (não trait dedicada — única impl).
- **C4 = Sealing após `Layouter::finish()`** por
  iteração fixpoint.

`SealedPositions` é o sub-store immutable que congela
`runtime.positions: HashMap<Location, Position>` no fim
de cada iteração de layout. Permite:

- Tracking via comemo de queries `position_of`
  pós-layout.
- Fecho da pendência ADR-0073 §C6a — `Introspector::position_of`
  ganha impl real (em P205C) via consumer que acede
  `SealedPositions`.

Vanilla equivalente: `PagedIntrospector::elements` +
`PagedIntrospector::position(loc) -> Option<PagedPosition>`
construído pós-layout via `PagedIntrospector::new(&pages)`
(`lab/typst-original/crates/typst-layout/src/introspect.rs:38`).
Cristalino diverge intencionalmente (sealing por
sub-store em vez de PagedIntrospector global; per
P205A.div-1).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O, sem estado
  global.
- Read-only após construção. Mutação só via construtor
  `from_runtime` (cf. pattern `BibStore`, `MetadataStore`).
- Derives: `Debug`, `Clone`, `Default`. Sem `PartialEq`
  / `Eq` (sem consumer identificado em P205B; pode ser
  adicionado se P205C precisar).
- Tipo interno: `std::collections::HashMap<Location,
  Position>` (não `FxHashMap` — coerência com
  `LayouterRuntimeState.positions` original; lookup
  não é hot path típico).
- `#[comemo::track]` aplicado a impl directo (não via
  trait). Único impl previsto.
- `Self: Clone` exigido por `#[comemo::track]` (CLAUDE.md
  convenção 1; lição P204B/P204C/P204G).

---

## Interface pública

```rust
use std::collections::HashMap;

use crate::entities::location::Location;
use crate::entities::position::Position;

#[derive(Debug, Clone, Default)]
pub struct SealedPositions {
    positions: HashMap<Location, Position>,
}

impl SealedPositions {
    /// Construtor vazio (Default::default).
    pub fn empty() -> Self;

    /// Constrói a partir de `LayouterRuntimeState.positions`
    /// (extraído por `Layouter::finish` ao fim da iteração).
    pub fn from_runtime(positions: HashMap<Location, Position>) -> Self;

    /// Número de positions registados.
    pub fn len(&self) -> usize;

    /// True se vazio.
    pub fn is_empty(&self) -> bool;
}

#[comemo::track]
impl SealedPositions {
    /// Position para a Location indicada, ou None se ausente.
    /// Tracked: queries repetidas reutilizam cache comemo.
    pub fn position_of(&self, location: Location) -> Option<Position>;
}
```

`from_runtime` é construtor por valor (consome o
HashMap; `Layouter::finish` move o conteúdo de
`runtime.positions`). Não há `Arc` interno —
clone é O(n) mas só acontece em re-tracking (raro).

`position_of` está no `#[comemo::track]` impl block
(separado do impl regular); demais métodos no impl
regular.

---

## Integração

- `PagedDocument` ganha campo
  `pub extracted_positions: SealedPositions`
  (`Default::default()` para call sites de
  `PagedDocument::new(vec![...])` em testes —
  retrocompatível).
- `Layouter::finish` constrói via `from_runtime` ao
  fim, antes de retornar `PagedDocument`.

Edição literal:

```text
// 01_core/src/rules/layout/mod.rs
pub fn finish(mut self) -> PagedDocument {
    // ... existing ...
    let mut doc = PagedDocument::new(self.pages);
    doc.extracted_label_pages = self.runtime.label_pages;
+   doc.extracted_positions = SealedPositions::from_runtime(
+       self.runtime.positions
+   );
    doc
}
```

---

## Sentinelas

Mínimo 2 (per P205B C7):

- `p205b_sealed_positions_struct_existe` — instancia
  `SealedPositions::empty()` + `from_runtime(HashMap::new())`.
- `p205b_sealed_positions_e_track` — verifica
  `comemo::Track` impl gerado pelo macro via
  `assert_track::<SealedPositions>()`.

Recomendado adicional:

- `p205b_layouter_finish_produz_sealed_positions` —
  invoca `Layouter::new(...).layout_content(...).finish()`;
  confirma `doc.extracted_positions` populated quando
  content tem locatable.

---

## Não-objectivos

- Não implementa `position_of` real em
  `TagIntrospector` (P205C).
- Não migra consumers do dual path (P205C).
- Não toca em `runtime.positions` populated (P204D
  preservado).
- Não materializa `SealedLabelPages` (P205D).
- Não transita ADR-0074 para ACEITE (P205E).

---

## Cross-references

- ADR-0074 PROPOSTO §P205B plano de materialização.
- P205A C1+C2+C3+C4 (caminhos fixados).
- P204D — Position concrete; pendência §C6a fechada
  estruturalmente por F3 minimal.
- P190C — `LayouterRuntimeState` pattern (struct
  dedicada para Layouter-runtime).
- BibStore (`entities/bib_store.md`) — pattern de
  sub-store cristalino.
- Vanilla `PagedIntrospector::position`:
  `lab/typst-original/crates/typst-layout/src/introspect.rs:60-63`
  — referência arquitectónica (não paridade literal).
