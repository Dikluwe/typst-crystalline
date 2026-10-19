# Passo 108.A — Inventário do vanilla `Introspection`

**Data**: 2026-04-23
**Fonte**: `lab/typst-original/crates/typst-library/src/introspection/`
**Propósito**: caracterizar o que existe no vanilla para depois comparar
com o cristalino (108.B) e propor sub-escopos (108.D).

---

## Estrutura do módulo

Directório com 13 ficheiros (sem sub-pastas):

```
convergence.rs  counter.rs    here.rs     introspector.rs
locate.rs       location.rs   locator.rs  metadata.rs
mod.rs          position.rs   query.rs    state.rs       tag.rs
```

`mod.rs` re-exporta tudo. **Não existe um struct `Introspection`
god-struct**. `Introspection` (convergence.rs) é apenas um wrapper
de type-erasure (`Arc<dyn Bounds>`) para colectar implementações
heterogéneas do trait `Introspect`.

---

## Parte 1 — Tipos centrais

### `Introspector` (trait, introspector.rs:29)

- **Tipo**: trait, não struct. Impls concretas em
  `ElementIntrospector<P>` + wrappers `PagedIntrospector`,
  `HtmlIntrospector`.
- **Campos da impl concreta `ElementIntrospector<P>`**:
  - `elems: Vec<(Content, P)>` — elementos introspectáveis em ordem
    do documento.
  - `keys: MultiMap<u128, Location>` — hash → location (medição).
  - `locations: FxHashMap<Location, Range<usize>>` — location → range
    de elementos (incluindo descendentes).
  - `labels: MultiMap<Label, usize>` — label → índices de elementos.
  - `queries: QueryCache` — `RwLock<HashMap<u128, EcoVec<Content>>>`
    (cache por selector).
- **Métodos principais** (trait):
  - `query(selector) -> EcoVec<Content>`
  - `query_first(selector) -> Option<Content>`
  - `query_count_before(selector, end) -> usize`
  - `page(location) -> Option<NonZeroUsize>`
  - `position(location) -> Option<DocumentPosition>`
- **Deps externas**: `comemo` (tracked), `ecow`, `rustc_hash`,
  `smallvec`.
- **Deps internas**: `Location`, `Selector`, `Content`, `Label`,
  `DocumentPosition`.

### `Location` (location.rs:59)

- **Campos**: `u128` (único campo, hash opaco).
- **Métodos principais**:
  - `new(hash: u128) -> Location`
  - `hash() -> u128`
  - `variant(n: usize) -> Location` — resolver colisões.
  - `page(engine, span) -> NonZeroUsize` (contextual)
  - `position(engine, span) -> PagedPosition` (contextual)
- **Deps externas**: `comemo` (para métodos contextuais).
- **Deps internas**: nenhuma (opaco).

### `Counter` (counter.rs:211)

- **Campos**: `key: CounterKey` enum
  (`Page | Selector(Selector) | Str(Str)`).
- **Métodos principais**:
  - `new(key: CounterKey) -> Counter`
  - `of(func: Element) -> Counter`
  - `get(engine, context, span) -> SourceResult<CounterState>`
  - `at(engine, context, span, selector) -> SourceResult<CounterState>`
  - `final_(engine, context, span) -> SourceResult<CounterState>`
  - `display(engine, context, span, numbering, at, both) -> Value`
- **Deps externas**: `comemo`, `ecow`.
- **Deps internas**: `Location`, `Selector`, `Introspector`,
  `CounterState` (em `counter.rs`), `CounterUpdate`, `CounterUpdateElem`.
- **Element associado**: `CounterUpdateElem` (produzido por `#counter.step()`,
  etc.) — marca posições no documento que o Introspector colecta.

### `Query` (função, query.rs:149)

- **Tipo**: função contextual, não struct.
- **Assinatura**: `query(engine, context, span, target) ->
  HintedStrResult<Array>`.
- **Deps**: `Introspector::query(selector)` internamente.
- **Semântica**: devolve `EcoVec<Content>` dos elementos que casam
  com o selector.

### `Locator<'a>` (locator.rs:153)

- **Campos**: `local: u128`, `outer: Option<&'a LocatorLink<'a>>`.
- **Métodos principais**:
  - `root() -> Locator<'static>`
  - `split() -> SplitLocator`
  - `relayout(&self) -> Locator`
  - `resolve() -> Resolved` (tracked).
- **`SplitLocator`**: `local: u128 + disambiguators: FxHashMap<u128, usize>`.
- **`LocatorLink`**: lista ligada imutável com `OnceLock<Resolved>`.
- **Propósito**: gerar `Location` únicas durante layout, numa
  **árvore posicional** que sobrevive a re-layouts (entre iterações
  de convergência).
- **Deps externas**: `comemo`, `rustc_hash`.
- **Deps internas**: `Location`.

### `Tag` (tag.rs:12)

- **Tipo**: enum.
- **Variantes**:
  - `Start(Content, TagFlags)` — entrada de um elemento
    introspectável com `Content` que tem `Location`.
  - `End(Location, u128, TagFlags)` — saída, regista location + hash.
- **`TagFlags`**: `{ introspectable: bool, tagged: bool }`.
- **Propósito**: eventos emitidos pelo layout para construir o
  `Introspector`. Cada par Start/End delimita um elemento.

### `State` (state.rs:189)

- **Campos**: `key: Str`, `init: Value`.
- **Métodos principais**:
  - `new(key: Str, init: Value) -> State`
  - `select() -> Selector`
  - `get(engine, context, span) -> SourceResult<Value>`
  - `at(engine, context, span, selector) -> SourceResult<Value>`
  - `final_(engine, context, span) -> SourceResult<Value>`
  - `update(span, update: StateUpdate) -> Content`
- **`StateUpdate`**: enum `Set(Value) | Func(Func)`.
- **Element associado**: `StateUpdateElem` (marcado `Locatable`).
- **Deps**: `Location`, `Selector`, `Value`, `Func`.

### Apoio: `Introspection` (convergence.rs)

- **Não é** o god-struct. É `pub struct Introspection(Arc<dyn Bounds>)`.
- Type-erasure para colecionar implementações do trait
  `Introspect` no `Sink`.
- **Trait `Introspect`**:
  - `introspect(engine, introspector) -> Output`
  - `diagnose(history) -> SourceDiagnostic`

### Apoio: `History<T>` (convergence.rs:218)

- Guarda saídas das 6 iterações (5 + 1 final).
- `converged() -> bool`: `hash(iter[4]) == hash(iter[5])`.

---

## Parte 2 — Funcionalidades

### `counter(heading).get()`

**Fluxo**:

```
counter.get(engine, ctx, span)
    ↓ (usa ctx.location())
engine.introspect::<CounterAtIntrospection>(CounterAtIntrospection(cnt, loc, span))
    ↓
Engine::introspect<I>(introspection: I) -> I::Output
    ├─ let introspector = engine.introspector.access(...)
    ├─ output = introspection.introspect(&mut engine, introspector)
    ├─ engine.sink.introspection(Introspection::new(introspection))
    └─ return output
    ↓
CounterAtIntrospection::introspect()
    ├─ counter.select(introspector, loc) -> Selector
    ├─ sequence(counter, selector, engine, introspector)  [memoizado]
    │    └─ for each elem in introspector.query(selector):
    │         - page counter: ajusta delta de página
    │         - aplica CounterUpdate::Step ou ::Func
    ├─ offset = introspector.query_count_before(selector, loc)
    └─ return sequence[offset]  // (CounterState, page_num)
```

### `query(heading)`

- Função contextual em `query.rs`.
- Chama `introspector.query(selector)` → `EcoVec<Content>`.
- Protegida pela mesma máquina de convergência
  (`QueryIntrospection`).

### `locate(x)`

- `locate` em `locate.rs` — função contextual.
- Aceita `Selector` ou callback; devolve posição(ões).

### Numeração hierárquica (`1.1`, `1.2.3`)

- **Implementada via `Counter`** com `CounterUpdate::Step`.
- `#set heading(numbering: "1.1")` injecta um `CounterUpdate` no
  documento; `Counter::display(numbering)` formata as componentes.
- Profundidade do `CounterState` é `Vec<usize>`; cada nível de
  heading corresponde a um elemento.

### Referências cruzadas (`@label`)

- `Label` gerado pelo parser (`<label>` no texto).
- No vanilla, associado a um `Content` via `Labelled`.
- `Introspector::labels: MultiMap<Label, usize>` faz o look-up.
- `Ref` element usa `engine.introspect` para resolver a label →
  location.

### Single-pass vs. multi-pass (convergence.rs)

**Multi-pass com fixpoint** — **crítico** para o futuro da
migração:

| Fase | Acção |
|------|-------|
| Iter 0 | Começa com Introspector prévio/vazio; eval + layout; novo Introspector dos Tags. |
| Iter 1–4 | Re-eval com novo Introspector; re-layout; novo Introspector. |
| Iter 5 (final) | Re-eval uma vez mais para verificação. |
| Pós-loop | `convergence::analyze(introspectors[0..6], introspections)`. |

Detecção de convergência compara **hashes** dos outputs, não
igualdade. `MAX_ITERS = 5`. Se não convergir: warnings por
introspection via `diagnose()`.

**Porquê multi-pass**: leituras de introspecção **afectam** layout
(ex: `query(heading)` pode mudar número de páginas). Layout produz
novas locations → novo introspector. Fixpoint necessário.

---

## Parte 3 — Grafo de dependências

```
                         ┌───────────────┐
                         │  Tag (layout) │
                         └───────┬───────┘
                                 │ produz
                                 ▼
                   ┌──────────────────────────┐
                   │ ElementIntrospector      │
                   │  elems / keys /          │
                   │  locations / labels /    │
                   │  queries (cache)         │
                   └────────────┬─────────────┘
                                │ acedido via Protected<Tracked<dyn Introspector>>
                                ▼
   ┌───────┐  ┌─────────┐  ┌─────┐  ┌──────┐  ┌───────┐
   │Counter│  │Locator  │  │Query│  │State │  │Location│
   └───┬───┘  └────┬────┘  └──┬──┘  └──┬───┘  └───┬────┘
       │           │          │        │          │
       └─────┬─────┴──────────┴────────┴──────────┘
             │ todos consomem
             ▼
   ┌─────────────────────────────────────────┐
   │ Engine { introspector, sink, ... }      │
   │  engine.introspect(I) -> I::Output      │
   │     ├─ chama I::introspect(engine, ...) │
   │     └─ sink.introspection(...)          │
   └─────────────────────────────────────────┘
             │ colecta
             ▼
   ┌────────────────────────────────────┐
   │ Sink.introspections: Vec<Introspec>│
   │ (convergence::analyze pós-layout)  │
   └────────────────────────────────────┘
```

**Deps de alto nível**:

```
Location              ← sem dep (u128 opaco)
Tag                   ← Content, Location
Locator               ← Location (gera-as)
ElementIntrospector   ← Content, Location, Selector, Label,
                        DocumentPosition
Counter               ← Location, Selector, Introspector,
                        CounterUpdateElem (layout)
State                 ← Location, Selector, Value, Func,
                        StateUpdateElem (layout)
Query                 ← Introspector, Selector
Introspection (wrap)  ← Arc<dyn Bounds> (type-erasure)
History               ← Introspection (múltiplas)
Engine                ← Introspector (Tracked), Sink
```

---

## Observações relevantes para o cristalino

1. **Não há god-struct `Introspection`**. O conceito é distribuído:
   `Introspector` trait + implementações + `Engine::introspect<I>`.
2. **Múltiplas iterações de eval são obrigatórias** para semântica
   completa. Um cristalino single-pass pode fazer *versão
   simplificada* mas não paridade total.
3. **`Location` é leaf** (sem deps estruturais). Candidato óbvio a
   materializar primeiro.
4. **`Counter` em vanilla é maior do que parece** — depende de
   `CounterUpdateElem` (elemento de layout), de `Selector`,
   e de contorno eval→layout→introspect→eval.
5. **Dependência de `comemo`**: extensa (tracked access em todo o
   lado). No cristalino, `comemo` já está disponível (ADR-0001).
6. **Dependência de `Element`/`Content`**: o sistema assume que
   elementos introspectáveis têm identidade via `Location`. No
   cristalino, `Content` é enum fechado (ADR-0026); não há
   `Locatable` trait. Materializar Location no cristalino implica
   decidir **onde** vive (campo extra em cada variante de Content? em
   wrapper?).
7. **Tag está fortemente acoplado a layout**. O cristalino actual
   não tem layout com Tags — tem Frame/FrameItem directo (ADR-0026).

Estas observações são exploradas em 108.C (dependências cruzadas).
