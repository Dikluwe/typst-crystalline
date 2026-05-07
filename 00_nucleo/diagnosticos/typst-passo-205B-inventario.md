# P205B — Inventário empírico

**Data**: 2026-05-07.
**Cláusula**: C1 do passo P205B.
**Pré-condição confirmada**: P205A concluído; ADR-0074
PROPOSTO em vigor; 1852 tests verdes; 0 violations.

---

## §1 C1 — Inventário (7 sub-secções)

### C1.1 — `Layouter::finish`

**Status**: ✅ **CONFIRMADO**.

- **Caminho**: `01_core/src/rules/layout/mod.rs:1167`.
- **Assinatura actual**:
  `pub fn finish(mut self) -> PagedDocument`.
- **Comportamento**:
  - Drena `current_line` para `current_items`.
  - Se `current_items` não vazio, push como `Page` final.
  - Cria `PagedDocument::new(self.pages)`.
  - Atribui `doc.extracted_label_pages =
    self.runtime.label_pages`.
  - Retorna `doc`.
- **Call sites**: 2 totais.
  - `01_core/src/rules/layout/mod.rs:1532` (short-circuit
    sem TOC).
  - `01_core/src/rules/layout/mod.rs:1566` (loop fixpoint
    iteração).

Sem complicações. Sealing point natural existe — após
linha 1183 (atribuição `extracted_label_pages`).

### C1.2 — `PagedDocument`

**Status**: ✅ **CONFIRMADO**.

- **Caminho**: `01_core/src/entities/layout_types.rs:424`.
- **Definição**:
  ```text
  #[derive(Debug, Clone)]
  pub struct PagedDocument {
      pub pages: Vec<Page>,
      pub extracted_label_pages: HashMap<Label, usize>,
  }
  ```
- **Construtor**: `pub fn new(pages: Vec<Page>) -> Self`
  inicializa `extracted_label_pages: HashMap::new()`.
- **Consumers `pub` ou interno**:
  - `01_core/src/rules/layout/mod.rs` (cria via finish).
  - `03_infra/src/pipeline.rs` (consome via PDF export).
  - `03_infra/src/layout.rs` (helper test).
  - `03_infra/src/export.rs` (consome para PDF).
  - ~10 call sites em testes:
    `PagedDocument::new(vec![...])` directamente.
- **Pode ganhar campo novo sem quebrar?** SIM, desde que
  o construtor `new` inicialize com `Default::default()`.
  Os ~10 testes que usam `PagedDocument::new(vec![...])`
  continuam a compilar. Precedente: `extracted_label_pages`
  foi adicionado em Passo 63 sem breaking change.

### C1.3 — `runtime.positions`

**Status**: ✅ **CONFIRMADO**.

- **Estado**: populated single-pass por
  `Layouter::advance_locator_if_locatable`
  (`01_core/src/rules/layout/mod.rs:287`).
- **Tipo**: `HashMap<Location, Position>`.
- **Consumers actuais**: nenhum. P204D §C6a documenta
  que `TagIntrospector::position_of` retorna sempre
  `None`; consumers que precisem de Position deveriam
  aceder via `layouter.runtime.positions.get(&loc)`
  directamente — mas em produção actualmente não há
  consumer real (apenas tests E2E P204D que verificam
  populate).

### C1.4 — Convenção `Arc<HashMap<...>>` ou similar

**Status**: ✅ **CONFIRMADO** (decisão: **sem Arc**).

- **Padrão dos sub-stores cristalinos** (`bib_store.rs`,
  `state_registry.rs`, `metadata_store.rs`):
  ```text
  #[derive(Debug, Clone, Default)]
  pub struct X { ... fields raw ... }
  ```
  HashMap directo + Clone derived; sem Arc explícito.
- **Padrão Arc usado em**: `Source` (Arc<...> para
  partilha de bytes), `Vec<u8>` em Content::Image
  (`PtrEqArc<Vec<u8>>`).
- **Decisão**: seguir padrão dos sub-stores
  (`HashMap` directo). Sealing acontece 1× por
  iteração; clone O(n) só em re-tracking (raro).
  Mudar para Arc se P205C ou benchmark futuro
  identificar necessidade.

### C1.5 — Sealing point empírico

**Status**: ✅ **CONFIRMADO**.

- **Caminho**: dentro de `Layouter::finish` em
  `01_core/src/rules/layout/mod.rs:1167-1185`,
  imediatamente após a linha 1183
  (`doc.extracted_label_pages = self.runtime.label_pages;`).
- **Edição literal**:
  ```text
  doc.extracted_label_pages = self.runtime.label_pages;
+ doc.extracted_positions = SealedPositions::from_runtime(
+     self.runtime.positions
+ );
  doc
  ```
- **Loop fixpoint adapta-se?** Não — `Layouter::finish`
  já retorna `PagedDocument`; o loop fixpoint apenas
  observa `doc.extracted_label_pages`. Não precisa
  alteração — `extracted_positions` fica disponível em
  `doc` automaticamente.

### C1.6 — `Send + Sync` em `SealedPositions`

**Status**: ✅ **CONFIRMADO**.

- `Location` derives `Hash + Eq + Send + Sync` automáticos
  (`#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`
  em `01_core/src/entities/location.rs:18`).
- `Position` é `Copy + PartialEq` derived; `Send + Sync`
  automaticamente porque todos os fields são
  primitivos (`NonZeroUsize`, `Pt(f64)`).
- `HashMap<Location, Position>` é `Send + Sync` quando K
  e V são — confirmado.
- `SealedPositions { positions: HashMap<...> }` herda
  automaticamente.

`#[comemo::track]` exige `Self: Clone` (per CLAUDE.md
convenção 1; lição P204B/P204C/P204G). `derive(Clone)`
em `SealedPositions` satisfaz.

### C1.7 — `Hash` satisfeito por `SealedPositions`

**Status**: ✅ **CONFIRMADO**.

- `Position` tem `impl std::hash::Hash` manual via
  `to_bits()` (`01_core/src/entities/position.rs:49`,
  per P204D).
- `Option<Position>` retorno de `position_of` é Hash
  automaticamente (Option de tipo Hash é Hash).
- `Location` parameter é Hash (derived).
- `usize` retorno de `len`/`is_empty` é Hash automático.
- `bool` retorno é Hash automático.

Bounds satisfeitos. Compile-time confirmation em C8.

---

## §2 C2 — Forma de sealing fixada

**Decisão**: **Caminho B — Field anexado** em
`PagedDocument`.

Justificação:

- Precedente claro: `extracted_label_pages` foi
  adicionado da mesma forma em Passo 63.
- `PagedDocument::new(pages)` continua a funcionar para
  os ~10 call sites em testes (Default no campo novo).
- Caminho A (tuple) exigiria adaptar 2 call sites de
  `Layouter::finish` para destructurar; mais invasivo
  sem benefício.
- Caminho C (sub-tipo `IntrospectionData`) é
  over-engineering sem necessidade actual; antecipa
  P205D mas custo de coordenação não compensa.

C2 fixa **uma**: **Caminho B**.

---

## §3 C3+C4 — Forma e localização

### C3 — Definição literal

```text
#[derive(Debug, Clone, Default)]
pub struct SealedPositions {
    positions: HashMap<Location, Position>,
}

impl SealedPositions {
    pub fn empty() -> Self;
    pub fn from_runtime(positions: HashMap<Location, Position>) -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}

#[comemo::track]
impl SealedPositions {
    pub fn position_of(&self, location: Location) -> Option<Position>;
}
```

- **Sem `Arc`**: padrão dos sub-stores.
- **Newtype com field privado** (`positions`): encapsula
  HashMap; expõe API explícita.
- **`#[comemo::track] impl` separado** do impl regular:
  apenas o método tracked vive lá (per pattern
  `entities/introspector.rs:40`).

### C4 — Localização

`01_core/src/entities/sealed_positions.rs` (módulo
dedicado).

Coerência com pattern `bib_store.rs`,
`state_registry.rs`, `metadata_store.rs`,
`resolved_label_store.rs`, `counter_registry.rs`,
`label_registry.rs`. Cada sub-store tem o seu próprio
ficheiro.

L0 prompt em `00_nucleo/prompts/entities/sealed-positions.md`
per Protocolo de Nucleação (CLAUDE.md).

---

## §4 C5 — Alterações literais

### C5.1 — `01_core/src/entities/mod.rs`

```text
+ pub mod sealed_positions;
```

Inserido após `pub mod position;` linha 48 (ordenação
alfabética relativa).

### C5.2 — `01_core/src/entities/sealed_positions.rs` (novo)

`SealedPositions` struct + impl + `#[comemo::track]
impl` + `#[cfg(test)] mod tests` com 4 tests
(2 sentinelas + 2 unit tests).

### C5.3 — `01_core/src/entities/layout_types.rs`

```text
  pub struct PagedDocument {
      pub pages: Vec<Page>,
      pub extracted_label_pages: HashMap<Label, usize>,
+     pub extracted_positions: crate::entities::sealed_positions::SealedPositions,
  }

  impl PagedDocument {
      pub fn new(pages: Vec<Page>) -> Self {
          Self {
              pages,
              extracted_label_pages: HashMap::new(),
+             extracted_positions:   crate::entities::sealed_positions::SealedPositions::empty(),
          }
      }
  }
```

### C5.4 — `01_core/src/rules/layout/mod.rs`

```text
  pub fn finish(mut self) -> PagedDocument {
      // ... existing ...
      let mut doc = PagedDocument::new(self.pages);
      doc.extracted_label_pages = self.runtime.label_pages;
+     doc.extracted_positions = crate::entities::sealed_positions::SealedPositions::from_runtime(
+         self.runtime.positions,
+     );
      doc
  }
```

---

## §5 Decisões durante a leitura

### D1 — Sem `Arc<HashMap<...>>` interno

C1.4 mostrou que sub-stores cristalinos não usam Arc.
Mantive o mesmo padrão. Clone O(n) só acontece em
re-tracking (improvável fora de fixpoint convergido);
overhead aceitável vs. complexidade de Arc/Rc.

### D2 — Caminho B sem hesitação

C1.2 confirmou precedente literal em
`extracted_label_pages` (Passo 63). Caminho A seria
desproporcional para apenas 1 sub-store novo. Caminho
C antecipa P205D que pode nem prosseguir (ADR-0074
declara P205D condicional).

### D3 — Newtype com field privado vs. tuple struct

`SealedPositions { positions: HashMap<...> }` em vez de
`SealedPositions(HashMap<...>)`. Razão: nome do field
ajuda em construções literais e em pattern matching
futuro; tuple struct daria `.0` opaco.

### D4 — `from_runtime` consume em vez de clone

Move `runtime.positions` por valor — `Layouter::finish`
consume `self`, então o HashMap fica disponível para
move sem clone overhead. Sealing é zero-cost no path
hot.

### D5 — 4 tests adicionados (não 2 mínimo)

Spec C7 exigia mínimo 2 sentinelas. Adicionei 2 sentinelas
+ 2 unit tests substantivos (empty + from_runtime
preserva mappings) — exercem o macro `#[comemo::track]`
via `.track().position_of(...)`. Cobertura adicional
trivial; valor acima do mínimo.
