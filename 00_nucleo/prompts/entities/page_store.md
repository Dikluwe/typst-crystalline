# Prompt L0 — `entities/page_store`
Hash do Código: b5409f41

**Camada**: L1
**Ficheiro alvo**: `01_core/src/entities/page_store.rs`
**Criado em**: 2026-05-12 (P207D sub-passo — materializa
sub-store sealed para metadata page-level per ADR-0076 Bloco
II + Bloco VIII).
**ADRs relevantes**: ADR-0076 (M9c — Introspector completion),
ADR-0074 (F3 sealing — pattern paralelo aplicado), ADR-0073
(M8 ACEITE — paridade `#[comemo::track]`).

---

## Contexto

ADR-0076 PROPOSTO (M9c — Introspector completion) fixa em
Bloco II 4 trait methods page-aware (`pages`, `page`,
`page_numbering`, `page_supplement`) + em Bloco VIII a
infraestrutura subjacente. P207D fixa em C2 a opção
**Opção 2 — `PageStore` sub-store dedicado** paralelo a
`SealedPositions` (P205B).

Pattern arquitectural reuso literal de P205B/C:
- Sub-store immutable construído pós-`Layouter::finish`.
- Pre-injecção: todos os queries retornam `None`/`0`.
- `inject_pages` em `TagIntrospector` análogo a
  `inject_positions` (P205C).

`PageStore` é o sub-store que congela metadata page-level
no fim de cada iteração de layout:

- Total de páginas (`NonZeroUsize`).
- Numbering pattern por página (`Option<EcoString>`).
- Supplement por página (`Content`).

Vanilla equivalente: `PagedIntrospector::new(pages: &[Page])`
pre-computa `page_numberings: Vec<Option<Numbering>>` e
`page_supplements: Vec<Content>`
(`lab/typst-original/crates/typst-layout/src/introspect.rs:38-58`).
Cristalino diverge em 2 pontos:

1. **Sealing por sub-store** (não por PagedIntrospector global,
   per P205A.div-1 + P205B).
2. **Numbering como `EcoString` pattern** (não enum
   `Numbering` vanilla), por coerência com ADR-0024
   (`EcoString` é o tipo cristalino para string-like values).
   `Content` mantém-se per ADR-0026 (cristalino tem `Content`
   próprio).

---

## Restrições Estruturais

- Camada **L1**: struct puro, sem I/O, sem estado global.
- Read-only após construção. Mutação só via construtores
  `empty`, `from_total_pages`, `from_runtime`.
- Derives: `Debug`, `Clone`, `Default`. Sem `PartialEq` /
  `Eq` (sem consumer identificado em P207D; pode ser
  adicionado se P207E ou futuros precisarem).
- Sem `#[comemo::track]` aplicado ao impl: queries retornam
  references (`Option<&EcoString>`, `Option<&Content>`).
  Lifetime elision em métodos tracked falha per CLAUDE.md
  convenção 2 (lição P107). Tracking acontece a nível do
  trait `Introspector` (P204B).

---

## Interface pública

```rust
use std::num::NonZeroUsize;

use ecow::EcoString;

use crate::entities::content::Content;

#[derive(Debug, Clone, Default)]
pub struct PageStore {
    total_pages: Option<NonZeroUsize>,
    numberings:  Vec<Option<EcoString>>,  // index = page - 1
    supplements: Vec<Content>,            // index = page - 1
}

impl PageStore {
    /// Construtor vazio (Default::default).
    pub fn empty() -> Self;

    /// Construtor minimal (P207D): só com `total_pages`
    /// populado; `numberings` e `supplements` vazios.
    /// Usado por `Layouter::finish` enquanto a captura
    /// de numbering+supplement no walk de layout não está
    /// materializada (deferred a passo futuro per Bloco
    /// VIII).
    pub fn from_total_pages(total: NonZeroUsize) -> Self;

    /// Construtor completo (P207E+ ou futuro): com
    /// numberings + supplements populados. `numberings.len()`
    /// e `supplements.len()` devem ser `total.get()`
    /// (1 entrada por página, 1-based indexada como
    /// `page.get() - 1`); construtor não valida (caller
    /// responsável).
    pub fn from_runtime(
        total:       NonZeroUsize,
        numberings:  Vec<Option<EcoString>>,
        supplements: Vec<Content>,
    ) -> Self;

    /// Total de páginas, ou `None` pre-injecção.
    pub fn total_pages(&self) -> Option<NonZeroUsize>;

    /// Numbering pattern para `page` (1-based). `None` se
    /// pre-injecção, página fora de range, ou página tem
    /// `None` numbering. Equivalente vanilla:
    /// `PagedIntrospector::page_numbering`.
    pub fn numbering_for_page(&self, page: NonZeroUsize)
        -> Option<&EcoString>;

    /// Supplement para `page` (1-based). `None` se
    /// pre-injecção, página fora de range, ou sem
    /// supplement capturado.
    pub fn supplement_for_page(&self, page: NonZeroUsize)
        -> Option<&Content>;

    /// True se pre-injecção (`total_pages.is_none()`).
    pub fn is_empty(&self) -> bool;
}
```

---

## Integração

- `TagIntrospector` ganha campo `pub page_store: PageStore`
  (default `PageStore::empty()` — retrocompatível).
- `TagIntrospector` ganha método
  `pub fn inject_pages(&mut self, page_store: PageStore)`
  paralelo a `inject_positions` (P205C).
- Caller pós-layout faz:

  ```rust,ignore
  let mut intr = introspect_with_introspector(content);
  let doc = layout_with_introspector(content, intr.clone());
  intr.inject_positions(doc.extracted_positions.clone());
  // P207D:
  intr.inject_pages(PageStore::from_total_pages(
      NonZeroUsize::new(doc.pages.len()).unwrap_or(NonZeroUsize::MIN),
  ));
  ```

- 4 trait methods em `Introspector` delegam:
  - `pages(loc)`: retorna `page_store.total_pages()` (ignora
    `loc` por paridade com vanilla `PagedIntrospector::pages`).
  - `page(loc)`: delega a `positions.position_of(loc)?.page`
    via `SealedPositions` (sub-store consolidado P205B).
  - `page_numbering(loc)`: `let p = page(loc)?;
    page_store.numbering_for_page(p)`.
  - `page_supplement(loc)`: `let p = page(loc)?;
    page_store.supplement_for_page(p)`.

---

## Semântica detalhada

- `total_pages()`: `None` pre-injecção; `Some(N)` pós-injecção
  onde N = total de páginas no documento.
- `numbering_for_page(page)`:
  - `None` se `is_empty()` (pre-injecção).
  - `None` se `page.get() > numberings.len()` (fora de range).
  - `None` se `numberings[page.get() - 1]` é `None` (página
    sem numbering — corresponde a vanilla `page.numbering.is_none()`).
  - `Some(&pattern)` caso contrário.
- `supplement_for_page(page)`: análogo. `None` se vazio, fora
  de range, ou sem entry. `Some(&content)` caso contrário.
  Vanilla devolve sempre `Some(&Content::empty())` ou
  similar; cristalino devolve `None` para distinguir
  "sem supplement" de "supplement vazio".
- `is_empty()`: equivalente a `total_pages.is_none()`. Não
  reflecte se `numberings`/`supplements` foram populados
  (que seriam vazios em P207D mas ganham conteúdo em
  passos futuros).

---

## Tests obrigatórios

- `PageStore::empty()` — sentinel construtor + `total_pages
  == None` + `is_empty == true`.
- `from_total_pages(NonZeroUsize::new(5))` produz
  `total_pages == Some(5)` + `is_empty == false`.
- `numbering_for_page` e `supplement_for_page` retornam
  `None` para `PageStore::empty()` (pre-injecção).
- `numbering_for_page` retorna `None` para `from_total_pages`
  (numberings vazios mesmo com total injectado).
- `from_runtime` com numberings/supplements populados
  resolve queries por página correctamente.
- Fora de range (`page > total_pages`) retorna `None`
  sem panic.

---

## Não-objectivos

- Não captura numbering/supplement durante walk de layout
  (deferred para Bloco VIII futuro — sub-passo dedicado
  pós-P207E).
- Não toca em `Page` struct (`width`, `height`, `items`
  permanecem inalterados; numbering/supplement só
  residem no `PageStore`, não nas `Page` individuais —
  divergência intencional vs vanilla onde `Page` carrega
  `numbering: Option<Numbering>` e `supplement: Content`).
- Não materializa `Numbering` enum vanilla (cristalino usa
  `EcoString` pattern directo per ADR-0024).
- Não migra `LayouterRuntimeState` (campos page-level
  ficam no `PageStore` sealed, não no runtime state mutable).
- Não transita ADR-0076 PROPOSTO → ACEITE (P211B).

---

## Cross-references

- ADR-0076 PROPOSTO §P207D plano de materialização.
- P207A C1 (item 9, 10, 12, 13 da auditoria — trait methods
  page-aware identificados).
- P207A.div-1 (escopo reduzido aprovado).
- P205B (`SealedPositions`) — pattern de sealing reusado
  literal.
- P205C (`inject_positions`) — pattern de injecção reusado.
- ADR-0074 (F3 sealing) — fundamento arquitectónico.
- ADR-0024 (`EcoString` em L1) — fundamento para
  `Option<EcoString>` em vez de `Numbering` enum.
- ADR-0026 (`Content` cristalino) — fundamento para
  `Content` field directo.
- Vanilla `PagedIntrospector`:
  `lab/typst-original/crates/typst-layout/src/introspect.rs:22-145`
  (referência arquitectónica — divergência sealing-por-sub-store
  preservada).
