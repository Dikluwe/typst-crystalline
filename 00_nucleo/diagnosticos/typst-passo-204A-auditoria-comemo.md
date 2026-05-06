# Auditoria empírica P204A — Estado pré-M8 (comemo / paridade vanilla)

**Data**: 2026-05-06.
**Executor**: Claude Code (Opus 4.7).
**Spec**: `00_nucleo/materialization/typst-passo-204A.md`.
**Natureza**: factos empíricos — sem decisões. As decisões
ficam no diagnóstico (`typst-passo-204A-diagnostico.md`).

---

## §1 Estado de partida verificado (per spec §2)

- ✅ Tests workspace: **1824 verdes**.
- ✅ Crystalline-lint: **0 violations**.
- ✅ Trait `Introspector`: **20 métodos** (verificado A1).
- ✅ `TagIntrospector`: **9 sub-stores** (verificado A4).
- ✅ `Layouter`: **22 fields**, sem `counter` (verificado
  A14).
- ✅ Walk fn: 7 parâmetros.
- ✅ 2 loops fixpoint, MAX=5.
- ✅ `comemo` 0.4.0 declarado em `Cargo.toml` workspace
  (verificado A6).
- ✅ Position: stub `position_of() -> Option<()>`; 0
  consumers em produção (verificado A3).
- ✅ Lacunas residuais: zero formalmente catalogadas
  (pós-P203B).

---

## §2 Bloco 1 — Trait `Introspector` cristalino (A1-A4)

### A1 — Listagem completa dos 20 métodos — **CONFIRMADO 20**

Local: `01_core/src/entities/introspector.rs:38-164`.

| # | Método | Assinatura | Retorno |
|---|--------|------------|---------|
| 1 | `query_by_kind` | `(&self, kind: ElementKind)` | `Vec<Location>` |
| 2 | `query_by_label` | `(&self, label: &Label)` | `Option<Location>` |
| 3 | `query_first` | `(&self, kind: ElementKind)` | `Option<Location>` |
| 4 | `query_unique` | `(&self, kind: ElementKind)` | `Option<Location>` |
| 5 | `position_of` | `(&self, location: Location)` | `Option<()>` |
| 6 | `figure_number_for_label` | `(&self, label: &Label)` | `Option<usize>` |
| 7 | `query_metadata` | `(&self)` | `&[Value]` |
| 8 | `formatted_counter` | `(&self, key: &str)` | `Option<String>` |
| 9 | `state_value` | `(&self, key: &str, location: Location)` | `Option<&Value>` |
| 10 | `state_final_value` | `(&self, key: &str)` | `Option<&Value>` |
| 11 | `query` | `(&self, selector: &Selector)` | `Vec<Location>` |
| 12 | `formatted_counter_at` | `(&self, key: &str, location: Location)` | `Option<String>` |
| 13 | `bib_entry_for_key` | `(&self, key: &str)` | `Option<&BibEntry>` |
| 14 | `bib_number_for_key` | `(&self, key: &str)` | `Option<u32>` |
| 15 | `is_numbering_active` | `(&self, key: &str)` | `bool` |
| 16 | `figure_number_at_index` | `(&self, kind: &str, idx: usize)` | `Option<usize>` |
| 17 | `is_numbering_active_at` | `(&self, key: &str, location: Location)` | `bool` |
| 18 | `flat_counter_at` | `(&self, key: &str, location: Location)` | `Option<usize>` |
| 19 | `resolved_label_for` | `(&self, label: &Label)` | `Option<&str>` |
| 20 | `headings_for_toc` | `(&self)` | `&[(Label, Content, usize)]` |

**20 métodos confirmados**. Sem divergência face ao snapshot.

### A2 — Mutabilidade — **CONFIRMADO read-only**

Todos os 20 métodos têm assinatura `&self` (read-only).
Nenhum método requer `&mut self`. **Compatível com
`#[comemo::track]`**.

`position_of` é stub: retorna sempre `None`. Não muta state.

### A3 — Consumers em produção — **CONFIRMADO**

Análise por método (callers em `01_core/src/rules/layout/`
e similares):

| Método | Consumers em produção | Notas |
|--------|----------------------|-------|
| `query_by_kind` | populate_intr_from_tag_start; tests | Indirectos via from_tags |
| `query_by_label` | references.rs (figure-ref label resolve) | Activo |
| `query_first` | (sem callers directos) | Reservado API |
| `query_unique` | (sem callers directos) | Reservado API |
| **`position_of`** | **0 (apenas 2 tests stub)** | **Stub** |
| `figure_number_for_label` | references.rs C2 figure-ref | Activo |
| `query_metadata` | stdlib query() (M9 P175) | Activo |
| `formatted_counter` | stdlib counter_final (P176) | Activo |
| `state_value` | (StateRegistry consumers) | Indirecto |
| `state_final_value` | is_numbering_active impl | Indirecto |
| `query` | stdlib query() (P175); Selector kind | Activo |
| `formatted_counter_at` | Layouter heading prefix C1 (P187B) | Activo |
| `bib_entry_for_key` | Layouter cite-arm (P181G) | Activo |
| `bib_number_for_key` | Layouter cite-arm (P181G) | Activo |
| `is_numbering_active` | Layouter heading + equation (P182D) | Activo |
| `figure_number_at_index` | Layouter figure C3 (P184D) | Activo |
| `is_numbering_active_at` | (futuro location-aware) | Reservado |
| `flat_counter_at` | Layouter equation C2 (P188B) | Activo |
| `resolved_label_for` | Layouter references C4 (P194B) | Activo |
| `headings_for_toc` | Layouter outline.rs (P200B) | Activo |

**Consumers reais** (Layouter): ~10 dos 20 métodos
activamente consumidos. **`position_of`** sem consumers
(stub).

### A4 — Sub-stores `TagIntrospector` — granularidade — **CONFIRMADO 9**

Local: `01_core/src/entities/introspector.rs:173-225`.

| Sub-store | Tipo | População | Granularidade |
|-----------|------|-----------|---------------|
| `labels` | `LabelRegistry` (HashMap<Label, Location>) | populate_intr (Tag::Start) | per-key |
| `counters` | `CounterRegistry` (HashMap<String, Vec<usize>>) | populate_intr (Heading + Figure + StateUpdate) | per-key |
| `kind_index` | `HashMap<ElementKind, Vec<Location>>` | populate_intr (cada Tag::Start) | per-kind |
| `figure_label_numbers` | `HashMap<Label, usize>` | populate_intr (Figure is_counted + Labelled) | per-key |
| `metadata` | `MetadataStore` (Vec<Value>) | populate_intr (Metadata) | append-only |
| `state` | `StateRegistry` | populate_intr (State + StateUpdate) | per-key |
| `bib_store` | `BibStore` (entries + numbers) | populate_intr (Bibliography) | per-key |
| `resolved_labels` | `ResolvedLabelStore` (HashMap<Label, String>) | populate_intr (Labelled) | per-key |
| `headings_for_toc` | `Vec<(Label, Content, usize)>` | populate_intr (HeadingForToc) | append-only |

**Granularidade dominante**: per-key (HashMap-based).
2 sub-stores append-only (metadata, headings_for_toc).

**Implicação para tracking**: `#[comemo::track]` aplicado
ao trait `Introspector` rastreia leituras per-method. Como
métodos consultam sub-stores per-key, granularidade efectiva
é per-key (boa para invalidação selectiva).

---

## §3 Bloco 2 — Uso actual de `comemo` (A5-A6)

### A5 — Uso existente de `comemo` no projecto — **CONFIRMADO**

Local: `grep -rn "comemo::\|#\[comemo::\|use comemo"`.

#### A5.1 `#[comemo::track]` aplicado:

| Local | Tipo | Estado |
|-------|------|--------|
| `01_core/src/entities/world_types.rs:140` | impl block (`World`) | activo |
| `01_core/src/entities/world_types.rs:331` | impl block | activo |
| `01_core/src/entities/world_types.rs:399` | impl block | activo |

#### A5.2 `Tracked` / `TrackedMut` consumers:

- `01_core/src/entities/engine.rs:26`:
  `use comemo::{Tracked, TrackedMut}`.
- `01_core/src/rules/eval/mod.rs:19`:
  `use comemo::{Tracked, TrackedMut}`.
- `01_core/src/rules/eval/markup.rs:12`,
  `eval/modules.rs:12`, `eval/closures.rs:24`: usam
  `TrackedMut`.

#### A5.3 `comemo::Track`:

- `01_core/src/entities/world_types.rs:9`:
  `use comemo::{Track, Tracked, Validate}`.
- `01_core/src/rules/introspect.rs:2482, 2480`,
  `01_core/src/rules/introspect/fixpoint.rs:181`,
  `01_core/src/rules/eval/tests.rs:24, 52, 1236, 1261, 1288, 1316`:
  `use comemo::Track` em testes.

#### A5.4 `Introspector` ainda **NÃO** trackable:

- `01_core/src/entities/introspector.rs:10`: comentário
  literal: "Plain trait sem `#[comemo::track]` —
  tracking deferido para M7+".
- `Sink` similar (em `entities/sink.rs:14`):
  "Integração comemo adiada".

**Conclusão A5**: comemo já está estabelecido como
infraestrutura no projecto (World, Engine, eval), mas
**`Introspector` está explicitamente deferido** com
comentário-link para M7+. M8 fecha esta pendência.

### A6 — Versão exacta e API disponível — **CONFIRMADO**

#### A6.1 Versão

`Cargo.lock` (linha 1352-1353):
```
[[package]]
name = "comemo"
version = "0.4.0"
```

#### A6.2 API pública (`comemo-0.4.0/src/lib.rs:92-95`):

```rust
pub use crate::cache::evict;
pub use crate::prehashed::Prehashed;
pub use crate::track::{Track, Tracked, TrackedMut, Validate};
pub use comemo_macros::{memoize, track};
```

#### A6.3 `#[track]` em **traits** — **CONFIRMADO suportado**

`comemo-macros-0.4.0/src/track.rs:30-43`:
```rust
syn::Item::Trait(item) => {
    if let Some(first) = item.generics.params.first() {
        bail!(first, "tracked traits cannot be generic")
    }
    ...
    let ty = parse_quote! { dyn #name + '__comemo_dynamic };
    (ty, &item.generics, Some(item.ident.clone()))
}
```

`#[comemo::track]` aplicado a `pub trait Foo { ... }` gera
implementação para `dyn Foo + '__comemo_dynamic`. Restrição:
**traits trackable não podem ser genéricos**.

#### A6.4 Restrições nos métodos tracked

Per `comemo-macros-0.4.0/src/lib.rs:108-133`:

- ❌ Generics (não permitidos).
- ❌ `unsafe`, `async`, `const`.
- ✅ `&self` ou `&mut self` (obrigatório).
- ✅ Args devem implementar `ToOwned`.
- ✅ Return values devem implementar `Hash`.
- ❌ Destructuring patterns em args.
- ❌ Mutable references no return.
- "Side effects gerais — responsabilidade do dev".

**Conclusão A6**: comemo 0.4.0 suporta `#[track]` em
traits não-genéricos. `Introspector` cristalino é
não-genérico (trait sem parameters) — directamente
aplicável.

---

## §4 Bloco 3 — Vanilla typst — pipeline `Introspector` (A7-A9)

### A7 — Trait vanilla `Introspector` — **CONFIRMADO 16 métodos**

Local: `lab/typst-original/crates/typst-library/src/introspection/introspector.rs:28-89`.

```rust
#[comemo::track]
pub trait Introspector: Send + Sync {
    fn query(&self, selector: &Selector) -> EcoVec<Content>;
    fn query_first(&self, selector: &Selector) -> Option<Content>;
    fn query_unique(&self, selector: &Selector) -> StrResult<Content>;
    fn query_label(&self, label: Label) -> StrResult<&Content>;
    fn query_labelled(&self) -> EcoVec<Content>;
    fn query_count_before(&self, selector: &Selector, end: Location) -> usize;
    fn label_count(&self, label: Label) -> usize;
    fn locator(&self, key: u128, base: Location) -> Option<Location>;
    fn pages(&self, location: Location) -> Option<NonZeroUsize>;
    fn page(&self, location: Location) -> Option<NonZeroUsize>;
    fn position(&self, location: Location) -> Option<DocumentPosition>;
    fn page_numbering(&self, location: Location) -> Option<&Numbering>;
    fn page_supplement(&self, location: Location) -> Option<&Content>;
    fn anchor(&self, location: Location) -> Option<&EcoString>;
    fn document(&self, location: Location) -> Option<Location>;
    fn path(&self, location: Location) -> Option<&VirtualPath>;
}
```

**16 métodos vanilla**. Trait é `Send + Sync` (requisito
comemo).

#### A7.1 — Mapeamento método cristalino ↔ vanilla

| Cristalino | Vanilla | Tipo de mapeamento |
|------------|---------|-------------------|
| `query_by_kind`, `query_by_label`, `query_first`, `query_unique` | `query` (subsume) | Cristalino fragmenta selectors |
| `query` (Selector::Kind) | `query` | 1:1 (parcial) |
| `query_metadata` | `query` (filtered by Metadata kind) | Many-to-one |
| `position_of` | `position` | 1:1 (cristalino é stub) |
| `figure_number_for_label` | (stdlib query + state) | Cristalino-só (M9 lacuna #6) |
| `formatted_counter`, `formatted_counter_at` | (stdlib counter via context) | Cristalino-só |
| `state_value`, `state_final_value` | (stdlib state via context) | Cristalino-só |
| `is_numbering_active(_at)` | (StyleChain numbering) | Cristalino-só (P182 lacuna #4) |
| `figure_number_at_index` | (stdlib counter("figure")) | Cristalino-só (P184 C3) |
| `flat_counter_at` | (stdlib counter("equation").get()) | Cristalino-só (P188 C2) |
| `bib_entry_for_key`, `bib_number_for_key` | (stdlib bibliography) | Cristalino-só (P181) |
| `resolved_label_for` | (resolved during walk) | Cristalino-só (P194) |
| `headings_for_toc` | (outline iter) | Cristalino-só (P200B) |
| (sem equiv) | `query_label`, `query_labelled` | Vanilla-só |
| (sem equiv) | `query_count_before` | Vanilla-só (otimização) |
| (sem equiv) | `label_count` | Vanilla-só |
| (sem equiv) | `locator` (key+base) | Vanilla-só |
| (sem equiv) | `pages`, `page` | Vanilla-só (paginação) |
| (sem equiv) | `page_numbering`, `page_supplement` | Vanilla-só |
| (sem equiv) | `anchor`, `document`, `path` | Vanilla-só (HTML/bundling) |

**Sumário**:
- 1:1 ou 1:few — 5 métodos.
- Cristalino-só (idiossincrasias M9 sem StyleChain) — ~12
  métodos.
- Vanilla-só (ainda não materializadas no cristalino) —
  ~10 métodos.

**Implicação para M8**: M8 não precisa replicar todas as
16 APIs vanilla. As idiossincrasias cristalinas são
funcionais e cobrem features (M9 11/11). Para paridade
**output observable**, basta que queries comuns
(`query`, `position`) tenham resultados equivalentes.

### A8 — Pipeline vanilla — fluxo completo — **CONFIRMADO**

#### A8.1 Construção

`lab/typst-original/crates/typst-layout/src/introspect.rs:35-58`:
```rust
impl PagedIntrospector {
    pub fn new(pages: &[Page]) -> PagedIntrospector {
        let mut builder = PagedIntrospectorBuilder::default();
        ...
        for (i, page) in pages.iter().enumerate() {
            let nr = NonZeroUsize::new(1 + i).unwrap();
            ...
            builder.discover_frame(&page.frame, Transform::identity(),
                &mut |point| { PagedPosition { page: nr, point } });
        }
        builder.finish(...)
    }
}
```

**Construído POST-LAYOUT** sobre `&[Page]` finalizadas.

#### A8.2 Tracking

`introspector.rs:95`:
```rust
pub fn track(&self) -> Tracked<'_, dyn Introspector + '_> {
    ...
}
```

Conversão `&self` → `Tracked<dyn Introspector>` via método
`.track()`.

#### A8.3 Distribuição via Engine

`engine.rs:26`:
```rust
pub introspector: Protected<Tracked<'a, dyn Introspector + 'a>>,
```

`Engine` carrega `Tracked` introspector como field. `Protected`
é wrapper para evitar mutação acidental.

#### A8.4 Consumers que aceitam `Tracked<dyn Introspector>`

Múltiplos call sites identificados:
- `model/bibliography.rs:179, 622`
- `routines.rs:58, 72`
- `introspection/location.rs:181, 220, 248, 277, 309, 343`
- `introspection/locator.rs:322`
- `introspection/query.rs:177, 204, 229, 254`
- `introspection/convergence.rs:136, 226`
- `typst-layout/src/pages/run.rs:80`

**Pattern canónico**: função recebe
`introspector: Tracked<dyn Introspector + '_>`.

#### A8.5 Convergence (fixpoint vanilla)

`introspection/convergence.rs:17`:
```rust
pub const MAX_ITERS: usize = 5;
const INSTANCES: usize = MAX_ITERS + 1;
```

`INSTANCES = 6 introspectors`. Convergence detection:
`hash128(introspectors[MAX_ITERS-1]) == hash128(introspectors[MAX_ITERS])`
(linha 254-255).

**Vanilla também usa hash-based convergence**, igual ao
cristalino. Diferença: vanilla executa até MAX+1 iter
para detectar; cristalino executa até MAX (5).

### A9 — Cache invalidation no vanilla — **CONFIRMADO**

#### A9.1 `comemo::evict` calls

`lab/typst-original/crates/typst-cli/src/watch.rs:81`:
```rust
comemo::evict(10);
```

**Único call site** em todo o vanilla. Aplicado em watch
mode para limpar cache.

`evict(N)` remove entries que não foram usadas em N ciclos
(definição da API comemo).

#### A9.2 Tracking-based invalidation

Comemo invalida automaticamente quando:
- Inputs hashed mudam (cache miss).
- Tracked methods retornam valor diferente.

Não há `evict` adicional intra-compilation. Cache vive ao
longo da compilação; eviction é do lado do user (CLI watch).

#### A9.3 Conclusão A9

Política vanilla:
- **Per-document via watch**: `evict(10)` no CLI.
- **Tracking-based** dentro de compilação: comemo gere
  automaticamente.
- Não há TTL nem per-query eviction explícita.

---

## §5 Bloco 4 — Invariantes do crate `comemo` (A10-A12)

### A10 — Tracking constraints — **CONFIRMADO**

#### A10.1 Tipos de argumentos para função tracked

Per `comemo-macros-0.4.0/src/lib.rs:30-46`:

- **Hashed** (default): hash 128-bit sobre args; cache key.
- **Immutably tracked**: `Tracked<T>` — fine-grained access
  tracking.
- **Mutably tracked**: `TrackedMut<T>` — mutações replayed
  em cache hit.

#### A10.2 Restrições para `#[track]`

Per `comemo-macros-0.4.0/src/lib.rs:107-134`:

**Aplicável a**:
- impl blocks (sem generics).
- traits (sem generics).

**Métodos tracked**:
- ❌ Genéricos.
- ❌ `unsafe`, `async`, `const`.
- ✅ `&self` ou `&mut self`.
- ✅ Args: `ToOwned`.
- ✅ Return: `Hash`.
- ❌ Destructuring patterns.
- ❌ Mutable references no return.

**Pureza**:
- "Apenas mutações via `&mut self` são observáveis".
- Side effects responsabilidade do dev.

#### A10.3 Composição

`Tracked` of `Tracked` é permitido (via parameters de
função tracked). Vanilla usa pesadamente.

#### A10.4 Conclusão A10

Trait `Introspector` cristalino satisfaz **todos** os
requisitos:
- Não-genérico ✅.
- Métodos `&self` ✅.
- Args com `ToOwned` (str, Location, ElementKind, Label,
  Selector — todos OK) ✅.
- Returns com `Hash` (Vec, Option<usize>, &str, &[T], etc.
  — todos OK) ✅.

### A11 — Lifetimes em `Tracked` — **CONFIRMADO**

Per `comemo-0.4.0/src/track.rs:147-153, 208-214`:

```rust
pub struct Tracked<'a, T, C = <T as Validate>::Constraint>
where
    T: Track + ?Sized,
{
    pub(crate) value: &'a T,
    ...
}

pub struct TrackedMut<'a, T, C = <T as Validate>::Constraint>
where
    T: Track + ?Sized,
{
    pub(crate) value: &'a mut T,
    ...
}
```

#### A11.1 Lifetime tied

`Tracked<'a, T>` mantém `&'a T`. Lifetime atado à
referência original.

#### A11.2 Storage em struct

Vanilla armazena em `Engine<'a>`:
```rust
pub introspector: Protected<Tracked<'a, dyn Introspector + 'a>>,
```

**Funciona**: struct ganha lifetime parameter.

#### A11.3 Atravessar fronteira de função

Sim — vanilla passa `Tracked<dyn Introspector + '_>` para
muitas funções. Padrão canónico.

#### A11.4 Mutable references

`TrackedMut` é exclusivo (`&mut`) — não coexiste com
`Tracked` na mesma value. Padrão typst: cada compilação
constrói nova instance + tracks.

### A12 — Performance characteristics — **CONFIRMADO**

#### A12.1 Cache lookup

Hash 128-bit sobre args + constraint validation. **O(1)**
amortizado por hit.

#### A12.2 Constraint validation (per `Tracked`)

Para cache hit, comemo valida **todas** as constraints
gravadas. Custo: O(N) sobre N tracked accesses durante
chamada original.

#### A12.3 Memory overhead

Per entry: hashed inputs + serialized output. `Send + Sync`
required para output.

#### A12.4 Threading

Internal `RwLock<Cache>` (per `comemo-0.4.0/src/cache.rs`).
Concorrência segura. `Send + Sync` on outputs obrigatório.

#### A12.5 Eviction

`comemo::evict(N: usize)` remove entries não tocadas em N
ciclos. Single global eviction call. Vanilla usa-o no CLI
watch.

---

## §6 Bloco 5 — Estado pré-M8 (A13-A16)

### A13 — Loops fixpoint cristalinos — **ORTOGONAIS a comemo**

#### A13.1 Loops cristalinos

| Loop | Local | MAX | Convergência |
|------|-------|-----|--------------|
| TOC fixpoint | `01_core/src/rules/layout/mod.rs:1506` | 5 | hash sobre `extracted_label_pages` |
| `run_fixpoint` | `01_core/src/rules/introspect/fixpoint.rs:33` | 5 | hash sobre tags via `compute_tags_hash` |

#### A13.2 Vanilla equivalente

`lab/typst-original/crates/typst-library/src/introspection/convergence.rs:17`:
```rust
pub const MAX_ITERS: usize = 5;
const INSTANCES: usize = MAX_ITERS + 1;
```

`hash128(introspectors[MAX_ITERS-1]) == hash128(introspectors[MAX_ITERS])`.

#### A13.3 Análise de compatibilidade

**Mecanismos paralelos não-conflituosos**:

- Loops fixpoint = controle de iteração até convergência.
  Cristalino e vanilla usam ambos hash-based check.
- comemo = invalidação granular dentro de cada iteração
  (memoíza queries; re-emite só quando inputs mudam).

**Conclusão A13**: **ortogonais**. M8 mantém loops
fixpoint cristalinos (compatíveis com vanilla); adiciona
comemo em cima para granularidade de invalidação.

### A14 — F3 parcial — Layouter fields — **CONFIRMADO**

22 fields actuais. Classificação:

#### A14.1 Categoria A (trackable / sub-store eligible) — 2 fields

| Field | Tipo | Justificação |
|-------|------|--------------|
| `introspector` | `TagIntrospector` | É a fonte canónica de tracking. Substituir por `Tracked<dyn Introspector + '_>` em M8. |
| `runtime` | `LayouterRuntimeState` | Já é struct dedicada (P190C/D). Pode ganhar `#[comemo::track]` se M8 estender. |

#### A14.2 Categoria B (runtime puro) — 17 fields

`metrics`, `sizer`, `font_size_pt`, `style`, `chain`,
`page_config`, `pages`, `current_items`, `cursor_x`,
`cursor_y`, `line_start_x`, `current_line`, `figure_progress`,
`is_height_unconstrained`, `cell_available_h`,
`cell_origin_x`, `cell_origin_y`, `cell_origin_w`.

Justificação: state de layout puro — posições, métricas,
flags de contexto. Não é trackable porque não é consumido
por queries de introspection.

#### A14.3 Categoria C (ambíguos) — 3 fields

| Field | Caso ambíguo |
|-------|--------------|
| `locator` | `Locator` sincronizado com walk. Pode beneficiar de tracking se queries `position_of` materializarem. |
| `current_location` | `Option<Location>` reads location-aware. Reads via Tracked podem cache `position_of` se materializado. |
| `pages` (parte de B mas relevante para Position) | `Vec<Page>` — fonte de truth para Position concrete. Não é trackable mas é input para Position computation. |

**Decisão C fica para sub-passos M8**.

### A15 — Corpus de paridade actual — **CRÍTICO**

#### A15.1 Tamanho

```
lab/parity/corpus/  →  30 ficheiros .typ
```

Distribuição:
- code/ — 2 ficheiros (let, set).
- markup/ — 7 ficheiros (heading, strong, parbreak, etc.).
- math/ — 2 ficheiros.
- semantic/ — 10 ficheiros (types, conditionals, closures).
- visual/ — 9 ficheiros (text styling).

#### A15.2 Cobertura introspection

Pesquisa exhaustiva:

```bash
find lab/parity/corpus/ -name "*.typ" | xargs grep -lE "outline|here\(\)|locate|counter|cite|figure"
```

Resultado: **0 ficheiros**.

**Conclusão A15**: corpus actual **NÃO exercita** features
de introspection (outline, here(), locate, counter, cite,
figure). Validação cristalino == vanilla para introspection
em M8 **requer expansão do corpus**.

### A16 — Position concrete — escopo M8 — **CONFIRMADO sub-passo M8**

#### A16.1 Stub actual

`Introspector::position_of(&self, location: Location) -> Option<()>`.

#### A16.2 0 consumers + 0 corpus pressure

Per A3 + A15.

#### A16.3 Vanilla cobre Position no trait

Trait vanilla declara:
```rust
fn position(&self, location: Location) -> Option<DocumentPosition>;
```

Sob `#[comemo::track]`. `DocumentPosition::Paged { page,
point }`.

#### A16.4 Recomendação

Position concrete é parte natural da paridade vanilla
trazida por M8:

1. Substituir stub `Option<()>` por `Option<Position>`
   (struct paralela a `PagedPosition`).
2. Layouter feedback single-pass popula `runtime.positions`
   (per A6 + A14 categoria A).
3. `position_of` consulta `runtime.positions`.

**Sub-passo M8** — não justifica M8.5 nem adiamento
pós-M8. Magnitude S-M dentro de M8 (struct + populate
arm + 1 ou 2 tests E2E).

---

## §7 Resumo dos achados empíricos

| Item | Etiqueta | Sumário |
|------|:--:|---------|
| A1 | CONFIRMADO 20 | Trait Introspector com 20 métodos read-only |
| A2 | CONFIRMADO read-only | Todos &self; compatível com `#[comemo::track]` |
| A3 | CONFIRMADO | ~10 métodos com consumers Layouter; `position_of` 0 consumers |
| A4 | CONFIRMADO 9 sub-stores | Granularidade dominante per-key (HashMap) |
| A5 | CONFIRMADO | comemo já usado em World/Engine/eval; Introspector explicitamente deferido |
| A6 | CONFIRMADO | comemo 0.4.0; `#[track]` suporta traits não-genéricos |
| A7 | CONFIRMADO 16 | Vanilla Introspector com `#[comemo::track]`; ~5 métodos overlap; ~12 cristalino-só |
| A8 | CONFIRMADO | Vanilla pipeline: PagedIntrospector::new(pages) post-layout; Engine<'a> carries Tracked |
| A9 | CONFIRMADO | comemo::evict(10) único call site (CLI watch); tracking-based intra-compilation |
| A10 | CONFIRMADO | Restrições compatíveis com Introspector cristalino |
| A11 | CONFIRMADO | Tracked<'a, T> tied to ref; storage em struct OK; vanilla usa pesadamente |
| A12 | CONFIRMADO | O(1) lookup amortizado; `Send + Sync` required em outputs |
| A13 | ORTOGONAIS | Fixpoint loops cristalinos coexistem com comemo (granularidade de invalidação) |
| A14 | CONFIRMADO | 22 fields: 2 categoria A (trackable), 17 categoria B (runtime), 3 categoria C (ambíguo) |
| A15 | **CRÍTICO** | Corpus paridade actual: 0 ficheiros exercitam introspection (M8 requer expansão) |
| A16 | CONFIRMADO sub-passo M8 | Position concrete é parte natural da paridade vanilla; sub-passo M8 |

---

## §8 Divergências relevantes

**Nenhuma**.

Todos os 16 itens de auditoria CONFIRMADO. O snapshot
2026-05-05 (pós-P203B) reflecte estado correcto. Sem
`P204A.div-N` registadas.

---

## §9 Referências

### Cristalino

- `01_core/src/entities/introspector.rs` (trait + impl;
  20 métodos; 9 sub-stores).
- `01_core/src/entities/world_types.rs` (3 `#[comemo::track]`
  pré-existentes).
- `01_core/src/entities/engine.rs` (Engine usa `Tracked`).
- `01_core/src/rules/layout/mod.rs:69` (Layouter struct
  22 fields).
- `01_core/src/rules/introspect/fixpoint.rs:33`
  (`MAX_FIXPOINT_ITERATIONS = 5`).
- `01_core/src/rules/layout/mod.rs:1506` (TOC
  `MAX_ITERATIONS = 5`).

### Vanilla

- `lab/typst-original/crates/typst-library/src/introspection/introspector.rs:28-89`
  (trait com `#[comemo::track]`).
- `lab/typst-original/crates/typst-layout/src/introspect.rs:35-63`
  (PagedIntrospector::new pipeline).
- `lab/typst-original/crates/typst-library/src/engine.rs:26`
  (Engine.introspector field).
- `lab/typst-original/crates/typst-library/src/introspection/convergence.rs:17`
  (MAX_ITERS = 5).
- `lab/typst-original/crates/typst-cli/src/watch.rs:81`
  (`comemo::evict(10)`).

### comemo crate

- `~/.cargo/registry/src/index.crates.io-*/comemo-0.4.0/`.
- `comemo-0.4.0/src/lib.rs` (API pública).
- `comemo-0.4.0/src/track.rs:12, 147, 208` (Track,
  Tracked, TrackedMut).
- `comemo-macros-0.4.0/src/track.rs:30-43` (suporte a
  traits não-genéricos).
- `comemo-macros-0.4.0/src/lib.rs:107-134` (restrições).
- `comemo-0.4.0/examples/calc.rs` (exemplo canónico).

### Snapshot

- `00_nucleo/diagnosticos/snapshot-2026-05-05.md` (estado
  pós-P203B reconciliado).

### Lacunas / corpus

- `lab/parity/corpus/` (30 .typ files; 0 introspection).
- `00_nucleo/diagnosticos/m1-lacunas-captura.md`.

### ADRs relevantes

- ADR-0066 (Introspection runtime adiada — ACEITE com nota
  intermediário até M8).
- ADR-0067 (Attribute grammar scoping — PROPOSTO).
- ADR-0072 (M7 fixpoint runtime estruturalmente fechado).
