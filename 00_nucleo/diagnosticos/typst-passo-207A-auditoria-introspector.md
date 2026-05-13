# P207A — Auditoria empírica gap Introspector cristalino vs vanilla

**Data**: 2026-05-11.
**Spec**: `00_nucleo/materialization/typst-passo-207A.md`.
**Output 1 de 4** (auditoria empírica).
**Etiquetas**: CONFIRMADO / DIVERGÊNCIA / N/A.

---

## §1 Bloco 1 — Trait Introspector cristalino vs vanilla

### A1 — Trait cristalino actual

**CONFIRMADO**.

Caminho: `01_core/src/entities/introspector.rs:41-179`
(@prompt-hash `918d279b`; @prompt
`00_nucleo/prompts/entities/introspector.md`).

Bounds: `#[comemo::track] pub trait Introspector: Send + Sync` —
linhas 40-41. ADR-0073 aplicado em P204B.

Impls:
- `TagIntrospector` (`01_core/src/entities/introspector.rs:188-247`,
  `impl Introspector` 279-386) — produção.
- 3 sentinel tests P204B (`p204b_trait_e_send_sync`,
  `p204b_dyn_trait_implementa_track`,
  `p204b_tagintrospector_pode_ser_tracked_via_dyn`) confirmam
  `#[comemo::track]` aplicado.

**20 métodos literais** (assinaturas exactas extraídas de
`introspector.rs:41-179`):

| # | Método | Assinatura | Origem |
|---|--------|------------|--------|
|  1 | `query_by_kind` | `fn(&self, kind: ElementKind) -> Vec<Location>` | P165 (M3) |
|  2 | `query_by_label` | `fn(&self, label: &Label) -> Option<Location>` | P165 |
|  3 | `query_first` | `fn(&self, kind: ElementKind) -> Option<Location>` | P165 |
|  4 | `query_unique` | `fn(&self, kind: ElementKind) -> Option<Location>` | P165 |
|  5 | `position_of` | `fn(&self, location: Location) -> Option<Position>` | P204D + P205C |
|  6 | `figure_number_for_label` | `fn(&self, label: &Label) -> Option<usize>` | P168 (M5) |
|  7 | `query_metadata` | `fn(&self) -> &[Value]` | P169 (M9.1) |
|  8 | `formatted_counter` | `fn(&self, key: &str) -> Option<String>` | P170 (M9.2) |
|  9 | `state_value` | `fn(&self, key: &str, location: Location) -> Option<&Value>` | P171 (M9.3) |
| 10 | `state_final_value` | `fn(&self, key: &str) -> Option<&Value>` | P171 |
| 11 | `query` | `fn(&self, selector: &Selector) -> Vec<Location>` | P175 (M9.5) |
| 12 | `formatted_counter_at` | `fn(&self, key: &str, location: Location) -> Option<String>` | P177 (M9.7) |
| 13 | `bib_entry_for_key` | `fn(&self, key: &str) -> Option<&BibEntry>` | P181F |
| 14 | `bib_number_for_key` | `fn(&self, key: &str) -> Option<u32>` | P181F |
| 15 | `is_numbering_active` | `fn(&self, key: &str) -> bool` | P182B (M9) |
| 16 | `figure_number_at_index` | `fn(&self, kind: &str, idx: usize) -> Option<usize>` | P184C |
| 17 | `is_numbering_active_at` | `fn(&self, key: &str, location: Location) -> bool` | P185B |
| 18 | `flat_counter_at` | `fn(&self, key: &str, location: Location) -> Option<usize>` | P185B |
| 19 | `resolved_label_for` | `fn(&self, label: &Label) -> Option<&str>` | P193B |
| 20 | `headings_for_toc` | `fn(&self) -> &[(Label, Content, usize)]` | P200B (M5) |

Per spec hipótese: **20 métodos confirmados literalmente**.

### A2 — Trait vanilla actual

**CONFIRMADO**.

Caminho: `lab/typst-original/crates/typst-library/src/introspection/introspector.rs:28-89`.

Bounds: `#[comemo::track] pub trait Introspector: Send + Sync` —
linhas 28-29 (paridade exacta com cristalino bounds).

Impls vanilla — **4 distintas**:

| Impl | Localização | Semântica |
|------|-------------|-----------|
| `EmptyIntrospector` | `typst-library/src/introspection/introspector.rs:92-164` | Stub: todos métodos retornam `EcoVec::new()` / `None` / `bail!` |
| `PagedIntrospector` | `typst-layout/src/introspect.rs:77` | Wrap `ElementIntrospector<PagedPosition>` para PDF |
| `HtmlIntrospector` | `typst-html/src/introspect.rs:70` | Wrap `ElementIntrospector<HtmlPosition>` para HTML; `page*` retorna `None` |
| `BundleIntrospector` | `typst-bundle/src/introspect.rs:79` | Wrap múltiplos `ElementIntrospector`s para document bundles |

Underlying genérico: `ElementIntrospector<P>`
(`typst-library/.../introspector.rs:170-469`) com 5 acceleration
structures: `elems: Vec<(Content, P)>`, `keys: MultiMap<u128, Location>`,
`locations: FxHashMap<Location, Range<usize>>`, `labels:
MultiMap<Label, usize>`, `queries: QueryCache (RwLock)`.

**16 métodos literais** (assinaturas exactas):

| # | Método | Assinatura | Categoria semântica |
|---|--------|------------|----------------------|
|  1 | `query` | `fn(&self, selector: &Selector) -> EcoVec<Content>` | Query genérica via Selector |
|  2 | `query_first` | `fn(&self, selector: &Selector) -> Option<Content>` | Idem |
|  3 | `query_unique` | `fn(&self, selector: &Selector) -> StrResult<Content>` | Idem |
|  4 | `query_label` | `fn(&self, label: Label) -> StrResult<&Content>` | Lookup por label |
|  5 | `query_labelled` | `fn(&self) -> EcoVec<Content>` | Todos elementos com label |
|  6 | `query_count_before` | `fn(&self, selector: &Selector, end: Location) -> usize` | Optimização counters/state |
|  7 | `label_count` | `fn(&self, label: Label) -> usize` | Multi-label counting |
|  8 | `locator` | `fn(&self, key: u128, base: Location) -> Option<Location>` | Hash-keyed assignment (measurement) |
|  9 | `pages` | `fn(&self, location: Location) -> Option<NonZeroUsize>` | Total pages no documento |
| 10 | `page` | `fn(&self, location: Location) -> Option<NonZeroUsize>` | Page number da location |
| 11 | `position` | `fn(&self, location: Location) -> Option<DocumentPosition>` | Position (page, point) |
| 12 | `page_numbering` | `fn(&self, location: Location) -> Option<&Numbering>` | Numbering pattern da página |
| 13 | `page_supplement` | `fn(&self, location: Location) -> Option<&Content>` | Supplement (e.g. "p. ") |
| 14 | `anchor` | `fn(&self, location: Location) -> Option<&EcoString>` | HTML link anchor |
| 15 | `document` | `fn(&self, location: Location) -> Option<Location>` | Doc-bundle ancestor |
| 16 | `path` | `fn(&self, location: Location) -> Option<&VirtualPath>` | Doc-bundle file path |

### A3 — Comparação trait-a-trait

**Tabela 3.1 — Cristalino → Vanilla** (20 → ?):

| Cristalino | Vanilla equivalente | Tipo gap |
|------------|---------------------|----------|
| `query_by_kind(kind)` | `query(&Selector::Elem(...))` | DIVERGÊNCIA arquitectónica (Selector vs ElementKind) |
| `query_by_label(&label)` | `query_label(label)` | DIVERGÊNCIA assinatura (`Option` vs `StrResult<&Content>`) |
| `query_first(kind)` | `query_first(&Selector::Elem(...))` | DIVERGÊNCIA arquitectónica |
| `query_unique(kind)` | `query_unique(&Selector::Elem(...))` | DIVERGÊNCIA arquitectónica |
| `position_of(loc)` | `position(loc)` | PARIDADE semântica (nome diferente; `Position` vs `DocumentPosition`) |
| `figure_number_for_label(&label)` | NÃO EQUIVALENTE no trait | Cristalino especializado; vanilla derivaria via Counter+labels |
| `query_metadata()` | NÃO EQUIVALENTE no trait | Cristalino especializado; vanilla via `query(&Selector::Elem(MetadataElem))` |
| `formatted_counter(key)` | NÃO EQUIVALENTE no trait | Vanilla tem `Counter` type domain (ver A4 / A5) |
| `state_value(key, loc)` | NÃO EQUIVALENTE no trait | Vanilla tem `State::at(...)` domain method |
| `state_final_value(key)` | NÃO EQUIVALENTE no trait | Vanilla `State::final_(...)` |
| `query(&Selector)` | `query(&Selector)` | PARIDADE nominal (mas Selector enum diverge — A12-A14) |
| `formatted_counter_at(key, loc)` | NÃO EQUIVALENTE no trait | Vanilla `Counter::at(...)` |
| `bib_entry_for_key(key)` | NÃO EQUIVALENTE | Vanilla `bibliography.rs` model separado |
| `bib_number_for_key(key)` | NÃO EQUIVALENTE | Idem |
| `is_numbering_active(key)` | NÃO EQUIVALENTE | Cristalino-only (encoding em `StateRegistry`) |
| `figure_number_at_index(kind, idx)` | NÃO EQUIVALENTE | Cristalino-only |
| `is_numbering_active_at(key, loc)` | NÃO EQUIVALENTE | Idem |
| `flat_counter_at(key, loc)` | NÃO EQUIVALENTE | Idem |
| `resolved_label_for(&label)` | NÃO EQUIVALENTE | Cristalino-only (P189B/P193B-P195) |
| `headings_for_toc()` | NÃO EQUIVALENTE | Cristalino-only (P200B; vanilla deriva via `query(heading)` + outline) |

**Tabela 3.2 — Vanilla → Cristalino** (16 → ?):

| Vanilla | Cristalino equivalente | Tipo gap |
|---------|------------------------|----------|
| `query(&Selector) -> EcoVec<Content>` | `query(&Selector) -> Vec<Location>` | DIVERGÊNCIA tipo retorno (`Content` vs `Location`); Selector reduzido |
| `query_first(&Selector)` | `query_first(kind)` (kind-only) | DIVERGÊNCIA arquitectónica |
| `query_unique(&Selector)` | `query_unique(kind)` (kind-only) | DIVERGÊNCIA arquitectónica |
| `query_label(label) -> StrResult<&Content>` | `query_by_label(&label) -> Option<Location>` | DIVERGÊNCIA tipo retorno + erro semântica |
| `query_labelled() -> EcoVec<Content>` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA |
| `query_count_before(sel, end)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (cristalino faz scan via `query(...).len()`) |
| `label_count(label)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (cristalino assume label única — `LabelRegistry` é HashMap) |
| `locator(key, base)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (locator hash-keyed; cristalino não tem measurement-driven assignment) |
| `pages(loc)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (page-aware) |
| `page(loc)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (page-aware) |
| `position(loc)` | `position_of(loc)` | PARIDADE semântica (rename) |
| `page_numbering(loc)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (page-aware) |
| `page_supplement(loc)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (page-aware) |
| `anchor(loc)` | NÃO EQUIVALENTE | DIVERGÊNCIA ARQUITECTÓNICA (HTML target apenas) |
| `document(loc)` | NÃO EQUIVALENTE | DIVERGÊNCIA ARQUITECTÓNICA (bundle target apenas) |
| `path(loc)` | NÃO EQUIVALENTE | DIVERGÊNCIA ARQUITECTÓNICA (bundle target apenas) |

**Tabela 3.3 — Divergências de assinatura (mesmo nome ou semântica próxima)**:

| Conceito | Cristalino | Vanilla | Implicação |
|----------|------------|---------|------------|
| Query genérica | `query(&Selector) -> Vec<Location>` | `query(&Selector) -> EcoVec<Content>` | Tipo retorno: cristalino devolve handles (Location), vanilla devolve elementos completos (Content). |
| Query por label | `query_by_label(&Label) -> Option<Location>` | `query_label(Label) -> StrResult<&Content>` | Ownership e erro: cristalino devolve owned `Option`; vanilla devolve `&Content` ou erro humano-legível. |
| Position | `position_of(Location) -> Option<Position>` | `position(Location) -> Option<DocumentPosition>` | Type alias: cristalino `Position { page: NonZeroUsize, point: Point }`; vanilla `DocumentPosition { Paged(PagedPosition) | Html(HtmlPosition) }` (target-polymorphic). |
| Bounds | `Send + Sync` + `#[comemo::track]` | Idem | **PARIDADE EXACTA** — ADR-0073. |

---

## §2 Bloco 2 — Sub-stores associados

### A4 — Sub-stores cristalinos

**CONFIRMADO**.

`TagIntrospector` (`introspector.rs:188-247`) tem **9 sub-stores
nomeados + 1 sub-store sealed** (P205C/F3):

| # | Sub-store | Tipo | Caminho | Origem | Pop. consumer |
|---|-----------|------|---------|--------|---------------|
| 1 | `labels`           | `LabelRegistry`                                   | `entities/label_registry.rs:23` (115L)  | P165 (M3) | `query_by_label` |
| 2 | `counters`         | `CounterRegistry`                                 | `entities/counter_registry.rs:23` (395L) | P170/P177/P184/P185 | `formatted_counter*` / `figure_number_at_index` / `flat_counter_at` |
| 3 | `kind_index`       | `HashMap<ElementKind, Vec<Location>>` (inline)    | `introspector.rs:191`                    | P165 + P175 | `query_by_kind` / `query` |
| 4 | `figure_label_numbers` | `HashMap<Label, usize>` (inline)              | `introspector.rs:197`                    | P168 (M5) | `figure_number_for_label` |
| 5 | `metadata`         | `MetadataStore`                                   | `entities/metadata_store.rs:24` (93L)   | P169 (M9.1) | `query_metadata` |
| 6 | `state`            | `StateRegistry`                                   | `entities/state_registry.rs:31` (172L)  | P171 (M9.3) | `state_value` / `state_final_value` / `is_numbering_active*` |
| 7 | `bib_store`        | `BibStore`                                        | `entities/bib_store.rs:30` (178L)       | P181B/E/F | `bib_entry_for_key` / `bib_number_for_key` |
| 8 | `resolved_labels`  | `ResolvedLabelStore`                              | `entities/resolved_label_store.rs:34` (122L) | P193B (P195 populate) | `resolved_label_for` |
| 9 | `headings_for_toc` | `Vec<(Label, Content, usize)>` (inline)           | `introspector.rs:232`                    | P200B (M5) | `headings_for_toc()` |
| 10 | `positions`       | `SealedPositions`                                 | `entities/sealed_positions.rs:31` (133L) | P205C (F3) per ADR-0074 | `position_of` (post-`inject_positions`) |

Public APIs por sub-store (extracto literal de cabeçalhos):

- `LabelRegistry`: `empty`, `lookup`, `len`, `is_empty` + interna `add` `pub(crate)`.
- `MetadataStore`: `empty`, `query`, `len`, `is_empty`.
- `BibStore`: `empty`, `entries`, `entry_for_key`, `number_for_key`,
  `len`, `numbers_len`, `is_empty` + `add_bibliography`,
  `assign_number` `pub(crate)`.
- `StateRegistry`: `empty`, `value_at`, `final_value`, `len`,
  `is_empty` + `init`, `update`, `apply_update` `pub(crate)`.
- `CounterRegistry`: `empty`, `value`, `len`, `is_empty`, `format`,
  `value_at`, `value_at_index` + `apply`, `apply_at`,
  `apply_hierarchical_at` `pub(crate)`.
- `ResolvedLabelStore`: `empty`, `get`, `len`, `is_empty` + `insert`
  `pub(crate)`.
- `SealedPositions`: `empty`, `from_runtime`, `len`, `is_empty`,
  `position_of`.

### A5 — Sub-stores vanilla

**CONFIRMADO + DIVERGÊNCIA ARQUITECTÓNICA**.

`lab/typst-original/crates/typst-library/src/introspection/`
contém **13 ficheiros** (3983L total):

```
convergence.rs    287L   QueryHistory + format_convergence_warning
counter.rs        991L   Counter type + CounterKey + CounterUpdate + CounterState + CounterDisplayElem
here.rs            49L   #[func(contextual)] pub fn here(...)
introspector.rs   695L   trait Introspector + EmptyIntrospector + ElementIntrospector<P> + Builder + MultiMap + QueryCache
locate.rs          41L   #[func(contextual)] pub fn locate(...)
location.rs       385L   Location u128 + helpers (page, position, page_numbering) + 6 *Introspection structs
locator.rs        395L   Locator<'a> + SplitLocator + LocatorLink (measurement-driven)
metadata.rs        30L   MetadataElem
mod.rs             45L   pub use + define()
position.rs       167L   DocumentPosition enum + PagedPosition + HtmlPosition
query.rs          285L   #[func(contextual)] pub fn query(...) + QueryIntrospection
state.rs          522L   State type + StateUpdate + StateUpdateElem
tag.rs             91L   Tag enum
```

**Divergência arquitectónica fundamental**:

Vanilla **não tem sub-stores nomeados análogos** ao cristalino.
A introspecção é centralizada em **`ElementIntrospector<P>`**
(introspector.rs:170-469) com 5 acceleration structures internas:

| # | Field | Tipo | Função |
|---|-------|------|--------|
| 1 | `elems`     | `Vec<(Content, P)>`                       | Lista linear de pairs (elemento, posição) |
| 2 | `keys`      | `MultiMap<u128, Location>`                | Hash-key index (measurement-driven assignment) |
| 3 | `locations` | `FxHashMap<Location, Range<usize>>`       | Range-acelerador (descendant queries) |
| 4 | `labels`    | `MultiMap<Label, usize>`                  | Label-index (multi-label support) |
| 5 | `queries`   | `QueryCache (RwLock<FxHashMap<u128, EcoVec<Content>>>)` | Cache de queries (subqueries dedup) |

Counter, State e Locator são **tipos de domínio separados**
(`counter.rs:215`, `state.rs:189`, `locator.rs:153`),
**não sub-stores do introspector**. São consumidos via
`Engine::introspect(...)` que usa o introspector tracked.

**Sumário paralelismo cristalino ↔ vanilla**:

| Cristalino sub-store | Vanilla equivalente | Tipo divergência |
|----------------------|---------------------|------------------|
| `LabelRegistry`      | `ElementIntrospector.labels` (single-multi diff) | DIVERGÊNCIA semântica (`Option` vs `MultiMap`) |
| `kind_index`         | (derivado via `query(&Selector::Elem(...))`)     | DIVERGÊNCIA arquitectónica (índice explícito vs query) |
| `CounterRegistry`    | `Counter` domain type (`counter.rs`)             | DIVERGÊNCIA arquitectónica (sub-store vs domain type) |
| `StateRegistry`      | `State` domain type (`state.rs`)                 | Idem |
| `MetadataStore`      | (derivado via `query(MetadataElem)`)             | DIVERGÊNCIA arquitectónica |
| `BibStore`           | `bibliography.rs` model (`typst-library/src/model/`) | DIVERGÊNCIA arquitectónica |
| `ResolvedLabelStore` | (derivado em layout)                              | DIVERGÊNCIA arquitectónica |
| `headings_for_toc`   | (derivado via `query(HeadingElem)` em outline)    | DIVERGÊNCIA arquitectónica |
| `figure_label_numbers` | (derivado via Counter+label)                    | DIVERGÊNCIA arquitectónica |
| `SealedPositions`    | `ElementIntrospector.elems[i].1: P` (per pair)   | DIVERGÊNCIA arquitectónica (sealed externo vs P inline) |

### A6 — Comparação sub-stores

**Tabela 6.1 — Cristalino → Vanilla**: ver A5 sumário.

**Tabela 6.2 — Vanilla → Cristalino**:

| Vanilla field | Cristalino equivalente | Gap |
|---------------|-----------------------|-----|
| `elems: Vec<(Content, P)>` | NÃO EQUIVALENTE — cristalino mantém só `Location`s; `Content` vive no Layouter pipeline | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA (single-pass; cristalino não retém `Content` post-walk) |
| `keys: MultiMap<u128, Location>` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (locator hash-key — vanilla precisa para measurement-driven assignment; cristalino não tem measurement) |
| `locations: FxHashMap<Loc, Range>` | (substituído por sub-stores diferenciados) | DIVERGÊNCIA ARQUITECTÓNICA |
| `labels: MultiMap<Label, usize>` | `LabelRegistry` (single-mapping) | DIVERGÊNCIA semântica (multi-label vs single-label) |
| `queries: QueryCache` | NÃO EQUIVALENTE | DIVERGÊNCIA — cristalino usa `#[comemo::track]` no trait (P204B); vanilla cache interna no `ElementIntrospector` |

**Conclusão**: comparação literal é misleading. Cristalino e vanilla
modelam introspecção com **arquitecturas diferentes**:

- **Cristalino**: sub-stores especializados por feature (counter,
  state, bib, etc.); trait expõe ~20 métodos finos; comemo
  trackability no trait (P204B).
- **Vanilla**: introspector único genérico baseado em
  `Vec<(Content, P)>`; trait expõe 16 métodos de query genérica via
  `Selector`; Counter/State como tipos de domínio separados;
  cache interna via `RwLock`.

Decisões de design não-trivial-isomorphic:
- ADR-0029 (Pureza física `Arc` permitido): cristalino pode ter
  sub-stores como `Arc<X>` em struct sem violar L1.
- ADR-0073 / ADR-0074: cristalino segue paridade vanilla **no
  trait + bounds**, mas mantém sub-stores especializados per
  arquitectura cristalina (ADR-0074 §C2 explicita).
- `P205A.div-1`: cristalino reusa `TagIntrospector` enriquecido
  por simplicidade (vanilla tem `PagedIntrospector`/`HtmlIntrospector`
  separados — assimetria por target).

---

## §3 Bloco 3 — Consumers (stdlib + rules)

### A7 — `here()` / `locate()` em stdlib cristalino

**CONFIRMADO ausência**.

Empírico:
```
$ grep -rn "fn here\|fn locate\|\"here\"\|\"locate\"" 01_core/src/ | grep -v scanner
01_core/src/rules/eval/closures.rs:212:        if ident.as_str() == "outline" {
[ apenas false-positive `locate` em scanner.rs:147 (lexer offset) ]
```

`here()` e `locate()` **não existem** em cristalino — nem como
funções stdlib registadas, nem como tipos de domínio, nem como
métodos no trait `Introspector`.

Localização esperada se materializadas:
`01_core/src/rules/stdlib/foundations.rs` (ao lado de `native_query`,
`native_state`, `native_counter_at`).

Vanilla:
- `here()`: `lab/typst-original/.../introspection/here.rs:46-49`,
  ```rust
  #[func(contextual)]
  pub fn here(context: Tracked<Context>) -> HintedStrResult<Location> {
      context.location()
  }
  ```
- `locate()`: `lab/typst-original/.../introspection/locate.rs:26-41`,
  ```rust
  #[func(contextual)]
  pub fn locate(
      engine: &mut Engine,
      context: Tracked<Context>,
      span: Span,
      selector: LocatableSelector,
  ) -> SourceResult<Location> {
      selector.resolve_unique(engine, context, span)
  }
  ```

Ambas dependem de `Tracked<Context>` que não tem análogo
cristalino directo. Cristalino faz introspecção via `ctx.introspector`
em `EvalContext` (single-threaded, pipeline directa).

### A8 — Counter / State

**CONFIRMADO + DIVERGÊNCIA ARQUITECTÓNICA**.

**Cristalino — counter** (`01_core/src/rules/stdlib/foundations.rs`):
- `native_counter_at(key, label_str)` linha 335-368 — **forma minimal
  P177**: retorna `Value::Str` formatado (`"1.2.3"`); reusa
  `Introspector::query_by_label` + `formatted_counter_at`.
- `native_counter_final(key)` linha 383-409 — retorna `Value::Str`
  formato hierárquico final.
- **Sem `counter(key)` constructor** — cristalino expõe directamente
  os helpers; não há `Counter` type.
- **Sem `counter.step()`, `counter.update(...)`, `counter.display(...)`,
  `counter.get()`, `counter.final()`, `counter.at(...)`** — vanilla expõe 7+
  métodos no `Counter` type; cristalino expõe 2 funções stdlib.

**Vanilla — counter** (`lab/typst-original/.../introspection/counter.rs`):
- `Counter::new(key)`, `Counter::of(func)`, `Counter::select_any()`,
  `Counter::select(...)`, `Counter::is_page()`, `Counter::display_at(...)`,
  + `#[scope]` API: `construct`, `get`, `display`, `at`, `final_`, `step`,
  `update` — total ~15 métodos públicos.

**Cristalino — state** (`foundations.rs`):
- `native_state(key, init)` linha 221-245 — produz `Content::State { key, init }`.
- `native_state_update(key, value)` linha 252-278 — produz
  `Content::StateUpdate { key, update: Set(...) }`.
- `native_state_update_with(key, fn)` linha 293-321 — **stub P172**:
  produz `StateUpdate::Func` mas é **silenciosamente ignorado** por
  `StateRegistry::apply_update`.
- **Sem `state.get()`, `state.at(loc)`, `state.final()`** — vanilla
  expõe 4+ métodos no `State` type.

**Vanilla — state** (`lab/typst-original/.../introspection/state.rs`):
- `State::new(key, init)`, `State::select(...)`, `State::select_any()`,
  + `#[scope]` API: `construct`, `get`, `at`, `final_`, `update` — total ~7 métodos.

**Tabela 8 — Resumo gap counter+state**:

| Feature | Cristalino | Vanilla | Categoria |
|---------|------------|---------|-----------|
| `counter(key)` constructor | NÃO EQUIVALENTE | `#[func(constructor)] Counter::construct` | EXTENSÃO NECESSÁRIA (consumer real ainda inexistente — usa native_counter_*) |
| `counter.step()` | NÃO EQUIVALENTE | `Counter::step` | EXTENSÃO NECESSÁRIA |
| `counter.update(by_or_fn)` | NÃO EQUIVALENTE | `Counter::update` | EXTENSÃO NECESSÁRIA |
| `counter.get()` (here-aware) | NÃO EQUIVALENTE (precisa `here()`) | `Counter::get` | EXTENSÃO NECESSÁRIA (bloqueada por `here()`) |
| `counter.at(label)` | `native_counter_at(key, label)` (forma plana) | `Counter::at` (label/loc/sel) | DIVERGÊNCIA semântica (forma minimal vs polymorphic) |
| `counter.final()` | `native_counter_final(key)` | `Counter::final_` | PARIDADE semântica (forma minimal) |
| `counter.display(numbering)` | NÃO EQUIVALENTE | `Counter::display` | EXTENSÃO NECESSÁRIA |
| `state.get()` (here-aware) | NÃO EQUIVALENTE | `State::get` | EXTENSÃO NECESSÁRIA |
| `state.at(loc)` | NÃO EQUIVALENTE | `State::at` | EXTENSÃO NECESSÁRIA |
| `state.final()` | NÃO EQUIVALENTE | `State::final_` | EXTENSÃO NECESSÁRIA |
| `state.update(value_or_fn)` | `native_state_update(key, value)` + `native_state_update_with(key, fn)` (stub) | `State::update` | DIVERGÊNCIA semântica + stub `_with` |

### A9 — Outline

**CONFIRMADO + DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA documentada**.

**Cristalino** (`01_core/src/rules/layout/outline.rs:23-78`):
- `pub(super) fn layout_outline(layouter)` — lê
  `intr.headings_for_toc()` + `runtime.known_page_numbers`;
  produz Sequence de `Content::Ref { target } + body_content +
  page_num`.
- Activa `is_readonly = true` durante layout das linhas TOC para
  bloquear CounterUpdate (DEBT-13).
- `Content::Outline` é `ElementKind::Outline` (P178 — locatable).
- Caminho Introspector activo via `headings_for_toc` sub-store
  (P200B); fallback legacy `counter.headings_for_toc` ELIMINADO em
  P190G.

**Vanilla** (`lab/typst-original/crates/typst-library/src/model/outline.rs` — 853L):
- `OutlineElem` com `target: Selector`, `depth: Smart<NonZeroUsize>`,
  `indent: Smart<Length>`, `fill: Option<Content>`.
- Resolve TOC via `Engine::query(...)` na phase de layout, não via
  sub-store dedicado.
- Suporta `target` configurável (qualquer Selector — heading com
  level específico, figura, equation, etc.).

**Divergências P206C registadas** (cf. ADR-0075 §"Achados empíricos"):
- `outline-toc.typ` heading count diff cristalino vs vanilla — TOC
  entries contadas distintamente.

**Tabela 9 — Outline gap**:

| Feature | Cristalino | Vanilla | Categoria |
|---------|------------|---------|-----------|
| `#outline()` básico | Sim (P200B) | Sim | PARIDADE LITERAL |
| TOC com page numbers | Sim (`known_page_numbers`) | Sim (via Counter "page") | PARIDADE LITERAL |
| `outline(target: ...)` configurável | NÃO — hardcoded para headings | Sim (qualquer Selector) | EXTENSÃO NECESSÁRIA (bloqueada por Selector::Elem) |
| `outline(depth: ...)` | NÃO empírico | Sim | EXTENSÃO NECESSÁRIA |
| `outline(indent: ..., fill: ...)` | NÃO empírico | Sim | EXTENSÃO NECESSÁRIA (cosmético) |
| TOC count exact match | DIVERGÊNCIA (P206C) | — | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA (cristalino emite count diff devido a fixpoint single-pass) |

### A10 — Bibliography

**CONFIRMADO + DIVERGÊNCIA ARQUITECTÓNICA + EXTENSÃO documentada**.

**Cristalino**:
- `native_bibliography(...)` `01_core/src/rules/stdlib/structural.rs:660`.
- `native_cite(...)` `01_core/src/rules/stdlib/structural.rs:713`.
- `BibStore` sub-store (P181B), `bib_entry_for_key` /
  `bib_number_for_key` trait methods (P181F).
- Layout consumer: `01_core/src/rules/layout/mod.rs:713` —
  `let entry = self.introspector.bib_entry_for_key(key);`.

**Vanilla**:
- `lab/typst-original/crates/typst-library/src/model/bibliography.rs`
  — implementação completa via `hayagriva` crate (BibLaTeX/Hayagriva
  format; CSL styles).

**DEBT-55** (em aberto): "Bibliography + Cite (XL; pré-condição
ADR-0062 hayagriva)". P181 série materializou subset cristalino
sem hayagriva.

**Divergência P206C** registada: `cite-bibliography.typ` falha em
cristalino eval — bibliography support cristalino parcial; gap
conhecido.

### A11 — Outros consumers

**CONFIRMADO ausências page-relevantes + label_count** (per spec
hipótese P205D D3).

**Page-relevantes — todos NÃO EQUIVALENTES em cristalino**:

| Vanilla | Cristalino | Categoria |
|---------|------------|-----------|
| `Introspector::pages(loc)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA |
| `Introspector::page(loc)` | NÃO EQUIVALENTE (parcial via `runtime.known_page_numbers`) | EXTENSÃO NECESSÁRIA |
| `Introspector::page_numbering(loc)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA |
| `Introspector::page_supplement(loc)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA |
| `location.page(...)` (stdlib accessor) | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA (bloqueada por trait) |
| `location.position(...)` | parcialmente — `position_of` post-`inject_positions` (P205C) | DIVERGÊNCIA — sealed runtime, sem stdlib accessor |
| `location.page_numbering(...)` | NÃO EQUIVALENTE | EXTENSÃO NECESSÁRIA |

**`label_count`**:
- Vanilla: `Introspector::label_count(label) -> usize`.
- Cristalino: `LabelRegistry` é `HashMap<Label, Location>` —
  **assume label única**. Multi-label não suportado.
- Categoria: EXTENSÃO NECESSÁRIA + refactor de `LabelRegistry`
  para `MultiMap` ou equivalente (impacto cross-modular).

**`locator(key, base)`**:
- Vanilla: usado para **measurement-driven assignment** (ver
  `Locator::synthesize`, `Locator::next_location`).
- Cristalino: não tem measurement-driven layout phase. Locations
  são atribuídas single-pass durante walk (`from_tags`).
- Categoria: DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA (single-pass
  cristalino vs measurement vanilla).

**`anchor`/`document`/`path`**:
- Vanilla: HTML target (`anchor`) e bundle target (`document`,
  `path`).
- Cristalino: PDF-only output target via `PagedDocument`. HTML e
  bundle não suportados.
- Categoria: DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA (single target).

**`query_labelled`**:
- Vanilla: `query_labelled() -> EcoVec<Content>` — todos
  elementos com label.
- Cristalino: NÃO EQUIVALENTE. `LabelRegistry::lookup` retorna
  por label específica; iterar sobre todas labels exigiria expor
  iterator no `LabelRegistry`.
- Categoria: EXTENSÃO NECESSÁRIA (trivial — adicionar `iter()` ao
  `LabelRegistry`).

**`query_count_before(selector, end)`**:
- Vanilla: optimização para counters/state que precisam contar
  antes de `end`.
- Cristalino: caller faria `query(sel).into_iter().filter(...)`
  manual.
- Categoria: EXTENSÃO NECESSÁRIA (optimização) ou DIVERGÊNCIA
  ARQUITECTÓNICA LEGÍTIMA (não há consumer real ainda).

---

## §4 Bloco 4 — Selector enum + parsing

### A12 — `Selector` cristalino

**CONFIRMADO**.

`01_core/src/entities/selector.rs:18-22`
(@prompt-hash `92ddd3cd`):

```rust
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Selector {
    /// Selector de kind — matches todos os elementos de um tipo.
    Kind(ElementKind),
}
```

**1 variant apenas** (P175 minimal):
- `Kind(ElementKind)` — onde `ElementKind` tem 10 variants
  (`Heading`, `Figure`, `Citation`, `Metadata`, `State`,
  `StateUpdate`, `Outline`, `Bibliography`, `Equation`,
  `CounterUpdate`).

**Sem** `Label`, `Where`, `And`, `Or`, `Before`, `After`, `Within`,
`Regex`, `Can`, `Elem(_, fields)`, `Location`.

Parsing standalone (text → Selector): **inexistente como tipo
formal**. Helper L3 `03_infra/src/query_helpers.rs:107`
(`parse_selector`) faz parsing simples (kind name OU `<label>`
syntax) mas devolve `ParsedSelector` (enum local L3), não
`Selector`. Per L0 §"Restrições": "Rejeita formas vanilla complexas
(`heading.where(...)`, etc.) com `InvalidSelector`."

### A13 — `Selector` vanilla

**CONFIRMADO**.

`lab/typst-original/crates/typst-library/src/foundations/selector.rs:73-103`:

```rust
#[ty(scope, cast)]
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Selector {
    Elem(Element, Option<SmallVec<[(u8, Value); 1]>>),  // type + field filter
    Location(Location),
    Label(Label),
    Regex(Regex),
    Can(TypeId),                           // capability — non-exposed
    Or(EcoVec<Self>),
    And(EcoVec<Self>),
    Before { selector: Arc<Self>, end: Arc<Self>, inclusive: bool },
    After { selector: Arc<Self>, start: Arc<Self>, inclusive: bool },
    Within { selector: Arc<Self>, ancestor: Arc<Self> },        // non-exposed
}
```

**10 variants**.

`#[scope]` API (selector.rs:158-...): `construct`, `or`, `and`,
`before`, `after` — encadeáveis. Construtor a partir de
`Element`, `String`, `Regex`, `Label`, `Location`.

Parsing standalone: **integrado no syntax** via `LocatableSelector`
type (`foundations/selector.rs` + `introspection/locate.rs:38`) que
aceita `<label>` syntax + element function references.

### A14 — Gap concreto

**Tabela 14 — Selector gap literal**:

| Variant | Cristalino | Vanilla | Categoria |
|---------|------------|---------|-----------|
| `Kind`/`Elem` | `Kind(ElementKind)` (10 kinds) | `Elem(Element, Option<fields>)` (~20 elements + field filter) | DIVERGÊNCIA semântica (kind enumerado vs reflection) |
| `Label` | NÃO | `Label(Label)` | EXTENSÃO NECESSÁRIA |
| `Where` | NÃO | (via `Elem` field filter) | EXTENSÃO NECESSÁRIA (bloqueada por `Elem` reflection) |
| `Or` | NÃO | `Or(EcoVec<Self>)` | EXTENSÃO NECESSÁRIA |
| `And` | NÃO | `And(EcoVec<Self>)` | EXTENSÃO NECESSÁRIA |
| `Before`/`After` | NÃO | Sim | EXTENSÃO NECESSÁRIA (bloqueada por `here()`) |
| `Within` | NÃO | Sim (non-exposed) | EXTENSÃO NECESSÁRIA |
| `Regex` | NÃO | Sim | EXTENSÃO NECESSÁRIA |
| `Location` | NÃO | Sim | EXTENSÃO NECESSÁRIA |
| `Can` (capability) | NÃO | Sim (non-exposed) | DIVERGÊNCIA ARQUITECTÓNICA (cristalino não tem capability system) |

Implicações para queries complexas:
- `query(heading.where(level: 1))` — **impossível em cristalino**
  (sem `Where`, sem `Elem` field reflection).
- `query(<intro>)` — **impossível** (sem `Label` variant; helper L3
  parseia `<...>` mas resolve via `query_by_label`, não `query`).
- `query(heading.before(here()))` — **impossível**
  (sem `Before` + sem `here()`).

---

## §5 Bloco 5 — Classificação do gap

### A15 — Categorias

Aplicação literal das 4 categorias (per spec §3 Bloco 5):

**Tabela A15 — Classificação por item** (62 itens auditados):

#### Trait methods (Vanilla → Cristalino) — 16 itens

| # | Item vanilla | Categoria | Razão |
|---|--------------|-----------|-------|
|  1 | `query(&Selector)` retorna `EcoVec<Content>` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino retorna `Vec<Location>` per design (post-walk não retém Content; Engine refaz lookup) |
|  2 | `query_first(&Selector)` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Idem (cristalino assume kind-only signature) |
|  3 | `query_unique(&Selector)` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Idem |
|  4 | `query_label(label)` retorna `&Content` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino devolve `Option<Location>` (single-pass) |
|  5 | `query_labelled()` | EXTENSÃO NECESSÁRIA | Trivial (1 método; itera `LabelRegistry`) |
|  6 | `query_count_before(sel, end)` | DECISÃO PENDENTE | Optimização sem consumer cristalino actual |
|  7 | `label_count(label)` | EXTENSÃO NECESSÁRIA + refactor `LabelRegistry` | Cristalino assume label única; multi-label exige refactor |
|  8 | `locator(key, base)` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino single-pass (sem measurement) |
|  9 | `pages(loc)` | EXTENSÃO NECESSÁRIA | Page-aware introspection |
| 10 | `page(loc)` | EXTENSÃO NECESSÁRIA | Page-aware (parcial via `runtime.known_page_numbers`) |
| 11 | `position(loc)` | PARIDADE LITERAL | `position_of` (rename) |
| 12 | `page_numbering(loc)` | EXTENSÃO NECESSÁRIA | Page-aware |
| 13 | `page_supplement(loc)` | EXTENSÃO NECESSÁRIA | Page-aware |
| 14 | `anchor(loc)` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | HTML-only |
| 15 | `document(loc)` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Bundle-only |
| 16 | `path(loc)` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Bundle-only |

#### Trait methods (Cristalino → Vanilla) — 20 itens

| # | Item cristalino | Categoria | Razão |
|---|-----------------|-----------|-------|
| 17 | `query_by_kind` | PARIDADE LITERAL (semântica via `query(&Selector)`) | Cristalino especializado |
| 18 | `query_by_label` | PARIDADE LITERAL (semântica via `query_label`) | Idem |
| 19 | `query_first(kind)` | PARIDADE LITERAL (semântica via `query_first(&Selector)`) | Idem |
| 20 | `query_unique(kind)` | PARIDADE LITERAL (idem) | Idem |
| 21 | `position_of(loc)` | PARIDADE LITERAL (rename de `position`) | F3 (P205C) |
| 22 | `figure_number_for_label` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino specialised; vanilla deriva |
| 23 | `query_metadata` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino sub-store dedicado |
| 24 | `formatted_counter` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino sub-store; vanilla domain type |
| 25 | `state_value` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Idem |
| 26 | `state_final_value` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Idem |
| 27 | `query(&Selector)` | PARIDADE NOMINAL + DIVERGÊNCIA Selector | Selector reduzido (A14) |
| 28 | `formatted_counter_at` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino specialised |
| 29 | `bib_entry_for_key` | DIVERGÊNCIA ARQUITECTÓNICA + DEBT-55 | Bib stdlib parcial (DEBT separado) |
| 30 | `bib_number_for_key` | DIVERGÊNCIA ARQUITECTÓNICA + DEBT-55 | Idem |
| 31 | `is_numbering_active` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Encoding cristalino-specific |
| 32 | `figure_number_at_index` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino specialised |
| 33 | `is_numbering_active_at` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Idem |
| 34 | `flat_counter_at` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Idem |
| 35 | `resolved_label_for` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino specialised (P189B/P195) |
| 36 | `headings_for_toc` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino sub-store (P200B); vanilla deriva via outline+query |

#### Sub-stores cristalino (10) — comparação

| # | Sub-store | Categoria | Razão |
|---|-----------|-----------|-------|
| 37 | `LabelRegistry` (single) vs `labels: MultiMap` vanilla | DIVERGÊNCIA SEMÂNTICA | Multi-label exige refactor (item 7) |
| 38 | `kind_index` vs derivação via Selector vanilla | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino indexa explicitamente |
| 39 | `figure_label_numbers` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino specialised |
| 40 | `MetadataStore` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Idem |
| 41 | `StateRegistry` (sub-store) vs `State` (domain type) vanilla | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Decisão F3 / P171 |
| 42 | `BibStore` vs `bibliography.rs` model vanilla | DIVERGÊNCIA ARQUITECTÓNICA + DEBT-55 | Subset funcional |
| 43 | `ResolvedLabelStore` | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino specialised |
| 44 | `headings_for_toc` (Vec inline) | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Cristalino specialised (P200B) |
| 45 | `CounterRegistry` (sub-store) vs `Counter` (domain type) vanilla | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | Decisão F3 / P170-P185 |
| 46 | `SealedPositions` (post-layout inject) vs `(Content, P)` inline vanilla | DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA (P205C / ADR-0074) | F3 minimal |

#### Stdlib consumers — 10 itens

| # | Item | Categoria | Razão |
|---|------|-----------|-------|
| 47 | `here()` | EXTENSÃO NECESSÁRIA | Stdlib consumer central; bloqueia `counter.get`, `state.get`, `query.before(here())` |
| 48 | `locate(selector)` | EXTENSÃO NECESSÁRIA | Bloqueado por Selector::Label + `here()` |
| 49 | `Counter` type (constructor + métodos) | DECISÃO PENDENTE | Cristalino tem `native_counter_*` planos; rich `Counter` type é XL |
| 50 | `State` type (constructor + métodos) | DECISÃO PENDENTE | Cristalino tem `native_state_*` planos; rich `State` type é XL |
| 51 | `counter.step()` | EXTENSÃO NECESSÁRIA | Bloqueada por (49) |
| 52 | `counter.update(by_or_fn)` | EXTENSÃO NECESSÁRIA | Idem |
| 53 | `counter.display(numbering)` | EXTENSÃO NECESSÁRIA | Idem |
| 54 | `state.update(value_or_fn)` | DIVERGÊNCIA + STUB | `_with` é stub silencioso; `Set` arm funcional |
| 55 | `outline(target, depth, indent, fill)` | EXTENSÃO NECESSÁRIA | Bloqueada por Selector + outline params |
| 56 | `bibliography()`/`cite()` full | EXTENSÃO NECESSÁRIA + DEBT-55 (XL) | DEBT separado, fora P207 |

#### Selector enum + parsing — 6 itens

| # | Item | Categoria | Razão |
|---|------|-----------|-------|
| 57 | `Selector::Label` | EXTENSÃO NECESSÁRIA | Trivial; desbloqueia consumers |
| 58 | `Selector::Where` (Elem field filter) | DECISÃO PENDENTE | Exige Element reflection (cristalino tem ElementKind enum, não reflection) |
| 59 | `Selector::And`/`Or` | EXTENSÃO NECESSÁRIA | Trivial estrutural; semântica simples |
| 60 | `Selector::Before`/`After` | EXTENSÃO NECESSÁRIA | Bloqueada por `here()` para uso real |
| 61 | `Selector::Regex` | DECISÃO PENDENTE | Cristalino não tem regex domain |
| 62 | Parsing standalone (`<>`, `heading.where(...)`) | DECISÃO PENDENTE | Exige integration com syntax — refactor cross-modular |

**Sumário contagens A15**:

| Categoria | Count | % |
|-----------|-------|---|
| PARIDADE LITERAL | 7 | 11% |
| DIVERGÊNCIA ARQUITECTÓNICA LEGÍTIMA | 27 | 44% |
| EXTENSÃO NECESSÁRIA | 19 | 31% |
| DECISÃO PENDENTE | 7 | 11% |
| Subtotal classificáveis | 60 | — |
| (DEBT-55 separados) | 2 | — |
| **TOTAL** | **62** | 100% |

Hipótese spec §9 ("~30% PARIDADE + ~25% DIVERGÊNCIA + ~35%
EXTENSÃO + ~10% DECISÃO PENDENTE"): **PARCIALMENTE CONFIRMADA**.
Empírico: PARIDADE 11% (menor que esperado 30% — porque vanilla
e cristalino têm arquitecturas mais divergentes do que paridade
trait sugeriria); DIVERGÊNCIA 44% (maior que esperado 25%);
EXTENSÃO 31% (próximo a 35%); DECISÃO PENDENTE 11% (próximo a 10%).

### A16 — Magnitude estimada de "completar"

Para cada item EXTENSÃO NECESSÁRIA (19 itens), estimativa S/M/L/XL
+ dependências:

**Bloco I — Trait extensions baixo custo** (S, sem dependências):
- (5) `query_labelled` — S (~30 min; expor iterator em
  `LabelRegistry`).
- (11→PARIDADE LITERAL) `position` rename — N/A (já feito como
  `position_of`).

**Bloco II — Trait extensions médio custo** (M, refactor sub-store):
- (7) `label_count` — M (~2h; refactor `LabelRegistry` para
  `MultiMap` ou expor count).
- (9) `pages(loc)` — M (~2-3h; depende de page-aware infrastructure
  que cristalino não tem completa).
- (10) `page(loc)` — M (~2-3h; idem; parcial via
  `runtime.known_page_numbers` mas não no trait).
- (12) `page_numbering(loc)` — M (~2h; depende de PageNumbering
  store).
- (13) `page_supplement(loc)` — M (~2h; idem).

**Bloco III — Selector extensions** (S-M):
- (57) `Selector::Label` — S (~1h; arm + dispatch em `query`).
- (59) `Selector::And`/`Or` — M (~3-4h; semântica de intersecção/união +
  consumers).
- (60) `Selector::Before`/`After` — M (~3h estrutural; **bloqueada por
  `here()` para uso real**).

**Bloco IV — Stdlib expansion** (M-L):
- (47) `here()` stdlib — M (~3-4h; depende de `Tracked<Context>`
  análogo cristalino + `EvalContext.location`).
- (48) `locate(selector)` — M (~2h; depende de Selector::Label
  + `here()`).
- (51-53) `counter.step/update/display` — L (~6-8h agregado;
  depende de `Counter` rich type — DECISÃO PENDENTE 49).
- (55) `outline(target, depth, ...)` — M-L (~4-6h; depende de
  Selector::Elem + outline params).

**Bloco V — Bibliography (DEBT-55 separado)** — XL fora de P207.

**Tabela 16 — Magnitude por bloco**:

| Bloco | Itens | Magnitude agregada | Tempo |
|-------|-------|---------------------|-------|
| I — Trait fácil | 1 | S | ~30 min |
| II — Trait page-aware | 5 | M-L | ~10-12h |
| III — Selector | 3 | S-M | ~7-8h |
| IV — Stdlib (sem rich types) | 4 | M | ~10-15h |
| IV+ — Stdlib (rich Counter/State) | 2-3 (DECISÃO PENDENTE) | L-XL | ~15-25h |
| V — Bibliography | 1 (DEBT-55) | XL | fora P207 |
| **Total Bloco I-IV (sem rich)** | **13** | **L** (~30h) | série dedicada |
| **Total Bloco I-IV+ (com rich)** | **15-16** | **XL** (~50h) | múltiplas séries |

**Bloqueios principais identificados**:
1. `here()` bloqueia: `counter.get`, `state.get`,
   `Selector::Before/After` consumers.
2. `Selector::Label` bloqueia: `locate(<...>)` stdlib.
3. `Counter`/`State` rich types bloqueiam: maioria dos métodos
   `.step/.update/.display`.
4. Page-aware infrastructure bloqueia: 5 trait methods page-*.
5. Element reflection (não-existente) bloqueia: `Selector::Where`
   + `outline(target: ...)` configurável.

**Ordenação por dependência** (sem ramos):
1. `Selector::Label` (S; sem dependências) →
2. `here()` stdlib (M; sem dependências) →
3. `locate(selector)` (S; depende 1+2) →
4. `Selector::And`/`Or`/`Before`/`After` (M-L; depende 2 para uso) →
5. `query_labelled` + `label_count` (M; sem dependências) →
6. Page-aware infrastructure (L; cross-cutting) →
7. `Counter`/`State` rich types (L-XL; DECISÃO PENDENTE 49+50) →
8. Outline configurável (M-L; depende 7+Selector).

---

## §6 Resumo final da auditoria

| Bloco | Etiqueta | Sub-itens |
|-------|----------|-----------|
| 1 — Trait Introspector (A1-A3) | **CONFIRMADO** | 20 cristalino + 16 vanilla; 4 impls vanilla, 1 cristalino |
| 2 — Sub-stores (A4-A6) | **CONFIRMADO + DIVERGÊNCIA ARQUITECTÓNICA fundamental** | 10 sub-stores cristalino vs 5 acceleration structures vanilla; arquitecturas não-isomorphic |
| 3 — Consumers (A7-A11) | **CONFIRMADO ausências** | `here()`/`locate()` ausentes; counter/state minimal cristalino; outline/bib parciais |
| 4 — Selector (A12-A14) | **CONFIRMADO** | Cristalino 1 variant (`Kind`); vanilla 10 variants |
| 5 — Classificação (A15-A16) | **CONFIRMADO + magnitude L-XL** | 62 itens classificados; magnitude agregada L (sem rich types) ou XL (completo) |

**Decisões habilitadas pela auditoria**:

- C1 (trait gaps prioritários): page-aware (5 itens M) +
  `query_labelled` (S) + `label_count` (M) — fundamentado em A11+A16.
- C2 (sub-stores gaps): `LabelRegistry` multi-label (item 37) é
  único refactor estrutural; restantes sub-stores são divergências
  legítimas.
- C3 (consumers prioritários): `here()` desbloqueia 4+ outros
  itens; é dependência crítica.
- C4 (Selector enum): `Label` (S) tem alta razão
  custo/desbloqueio; `Where`/`Regex` ficam DECISÃO PENDENTE.
- C5 (stdlib expansion): `here()`/`locate()` materializar como
  série dedicada; rich Counter/State é DECISÃO PENDENTE.
- C6 (estrutura trajectória): magnitude L-XL exige sub-séries
  ou marco arquitectónico — **não** série única.
- C9 (ADR proposta): cada marco prévio (M7, M8, F3, vanilla
  integration) ganhou ADR; "completar Introspector" é decisão
  arquitectural com alternativas reais (escopo amplo vs reduzido vs
  divergência aceite). **Sim — ADR-0076 PROPOSTO**.
- C12 (escopo amplo vs reduzido): A15 mostra 44% DIVERGÊNCIA
  ARQUITECTÓNICA LEGÍTIMA; tentar "completar tudo" inflaria
  trabalho sem benefício. **`P207A.div-1` recomendado**:
  reduzir escopo a EXTENSÃO NECESSÁRIA (19 itens) excluindo
  divergências legítimas.
