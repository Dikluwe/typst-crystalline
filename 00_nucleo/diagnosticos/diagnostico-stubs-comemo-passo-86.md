# Diagnóstico: stubs `Tipo(())` em L1 e dependências em falta

**Tipo vanilla**: `Routines`, `Engine`, `Sink`, `Traced`, `Route`, `Styles`
**Localização vanilla**: `lab/typst-original/crates/typst-library/src/{engine.rs, routines.rs, foundations/styles.rs}`
**Data do diagnóstico**: 2026-04-22
**Contexto**: Passo 86, Tarefa A (preparação para Passo 87+)

**Natureza**: registo factual do estado dos stubs L1 e da estrutura
vanilla correspondente. Decisões arquitecturais derivadas deste
diagnóstico ficam em ADR/passo separados. Este ficheiro não contém
decisões.

---

## 1. Inventário dos stubs em L1

Todos os stubs estão em `01_core/src/entities/world_types.rs`.

| Stub | Linha | Forma actual | `#[comemo::track]`? |
|------|-------|--------------|---------------------|
| `Routines` | 106 | `pub struct Routines(())` | Não |
| `Traced` | 121 | `pub struct Traced(())` + `#[comemo::track] impl Traced {}` | Sim (bloco vazio) |
| `Styles` | 140 | `pub struct Styles(())` | Não |
| `Route` | 156 | `pub struct Route(())` + `#[comemo::track] impl Route {}` | Sim (bloco vazio) |
| `Sink` | 175 | `pub struct Sink(())` + `#[comemo::track] impl Sink {}` | Sim (bloco vazio) |
| `Engine` | 193 | `pub struct Engine(())` | Não |

Total: **6 stubs `Tipo(())`**, dos quais **3 com `#[comemo::track]`**
(Traced, Route, Sink) — todos com bloco `impl` vazio.

Pontos de uso em L1 (apenas como tipos opacos, não construídos com
estado real):

```rust
// 01_core/src/rules/eval.rs:37, 251–255
use crate::entities::world_types::{Route, Routines, Sink, Traced};

pub fn eval(
    _routines: &Routines,
    world: &dyn World,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> { ... }

// 01_core/src/rules/eval.rs:1719–1722 (em testes)
let routines = Routines::new();
let traced   = Traced::new();
let mut sink = Sink::new();
let route    = Route::new();
```

`Engine` e `Styles` não são construídos em nenhum ponto activo de L1.
`Engine` é referenciado apenas em testes do stub. `Styles` é
referenciado em `entities/value.rs:80` num comentário (`Styles(Styles)`
está comentado em `Value`).

---

## 2. Estrutura vanilla de cada stub

### 2.1 `Routines` — vanilla

`lab/typst-original/crates/typst-library/src/routines.rs:31`:

```rust
pub struct Routines {
    pub rules: NativeRuleMap,
    pub eval_string: fn(...) -> SourceResult<Value>,
    pub eval_closure: fn(...) -> SourceResult<Value>,
    pub realize: for<'a> fn(...) -> SourceResult<Vec<Pair<'a>>>,
    pub layout_frame: fn(...) -> SourceResult<Frame>,
    pub html_module: fn() -> Module,
    pub html_span_filled: fn(Content, Color) -> Content,
}
```

Macro-gerado. `Hash` manual no-op. `Debug` manual `"Routines(..)"`.
Sem `#[comemo::track]` — é vtable, não estado mutável.

### 2.2 `Traced` — vanilla

`lab/typst-original/crates/typst-library/src/engine.rs:120`:

```rust
#[derive(Default)]
pub struct Traced(Option<Span>);

#[comemo::track]
impl Traced {
    pub fn get(&self, id: FileId) -> Option<Span> { ... }
}
```

Wrapper sobre `Option<Span>`. Um único método tracked (`get`).

### 2.3 `Styles` — vanilla

`lab/typst-original/crates/typst-library/src/foundations/styles.rs:24`:

```rust
#[ty(cast)]
#[derive(Default, Clone, PartialEq, Hash)]
pub struct Styles(EcoVec<LazyHash<Style>>);
```

Sem `#[comemo::track]` directo. `Style` é trait-object-like via
`LazyHash<Style>`. `EcoVec` vem de `ecow`.

### 2.4 `Route` — vanilla

Já totalmente caracterizado em
`00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md`.
Resumo: `pub struct Route<'a>` com `outer: Option<Tracked<...>>`,
`id: Option<FileId>`, `len: usize`, `upper: AtomicUsize`.
`#[comemo::track] impl<'a> Route<'a>` com 2 métodos (`contains`,
`within`). Localização: `engine.rs:251`.

### 2.5 `Sink` — vanilla

`lab/typst-original/crates/typst-library/src/engine.rs:150`:

```rust
#[derive(Default, Clone)]
pub struct Sink {
    introspections: EcoVec<Introspection>,
    delayed: EcoVec<SourceDiagnostic>,
    warnings: EcoVec<SourceDiagnostic>,
    warnings_set: FxHashSet<u128>,
    values: EcoVec<(Value, Option<Styles>)>,
}

#[comemo::track]
impl Sink {
    pub fn introspection(&mut self, introspection: Introspection) { ... }
    pub fn delay(&mut self, errors: EcoVec<SourceDiagnostic>) { ... }
    pub fn warn(&mut self, warning: SourceDiagnostic) { ... }
    pub fn value(&mut self, value: Value, styles: Option<Styles>) { ... }
    fn extend(&mut self, ...) { ... }
}
```

5 métodos tracked, todos `&mut self -> ()` (push-only sink).

### 2.6 `Engine` — vanilla

`lab/typst-original/crates/typst-library/src/engine.rs:19`:

```rust
pub struct Engine<'a> {
    pub routines: &'a Routines,
    pub world: Tracked<'a, dyn World + 'a>,
    pub introspector: Protected<Tracked<'a, dyn Introspector + 'a>>,
    pub traced: Tracked<'a, Traced>,
    pub sink: TrackedMut<'a, Sink>,
    pub route: Route<'a>,
}
```

Sem `#[comemo::track]`. É um agregador de handles tracked, não tracked
ele próprio.

---

## 3. Classificação por campo

Legenda: ✓ = disponível em L1 ou externo autorizado; ✗ = pendente /
não autorizado.

**`Routines`** (vtable macro-gerada):
- `rules: NativeRuleMap` ✗ — pendente.
- `eval_string: fn(...)` — assinatura inclui `Tracked<dyn World>` ✓,
  `TrackedMut<Sink>` ✗ (Sink pendente), `Tracked<dyn Introspector>` ✗,
  `Tracked<Context>` ✗, `SyntaxMode` ✓, `Scope` ✓.
- `eval_closure: fn(...)` — inclui `LazyHash<Closure>` ✗, `Closure` ✓,
  `Func` ✓, `Args` ✓, restante idêntico a `eval_string`.
- `realize: for<'a> fn(...)` — `RealizationKind` ✗, `SplitLocator` ✗,
  `Arenas` ✗, `StyleChain` parcial (L1 já tem mas diverge do vanilla),
  `Pair` ✗.
- `layout_frame: fn(...)` — `Frame` ✗, `Locator` ✗, `Region` ✗.
- `html_module: fn() -> Module` — `Module` ✓.
- `html_span_filled: fn(Content, Color)` — `Content` ✓, `Color` ✗.

**`Traced`** (wrapper sobre `Option<Span>`):
- `Option<Span>` ✓ (`01_core/src/entities/span.rs`).
- Método `get(&self, id: FileId) -> Option<Span>` — `FileId` ✓, `Span` ✓.

**`Styles`** (`EcoVec<LazyHash<Style>>`):
- `EcoVec` ✗ (ecow — ADR-0024 só autorizou `EcoString`).
- `LazyHash<T>` ✗ (typst_utils não em L1).
- `Style` ✗ (pendente).

**`Route`** (linked list com atomic):
- `outer: Option<Tracked<'a, Self, ...>>` ✓ (ADR-0001).
- `id: Option<FileId>` ✓.
- `len: usize` ✓.
- `upper: AtomicUsize` ✓ (stdlib).

**`Sink`** (push-only):
- `introspections: EcoVec<Introspection>` ✗ (`EcoVec`, `Introspection`).
- `delayed: EcoVec<SourceDiagnostic>` ✗ (`EcoVec`); `SourceDiagnostic` ✓
  (`01_core/src/entities/source_result.rs`).
- `warnings: EcoVec<SourceDiagnostic>` ✗ idem.
- `warnings_set: FxHashSet<u128>` ✓ (ADR-0018).
- `values: EcoVec<(Value, Option<Styles>)>` ✗ (`EcoVec`, `Styles`);
  `Value` ✓.

**`Engine`** (agregador):
- `routines: &'a Routines` ✗.
- `world: Tracked<'a, dyn World + 'a>` ✓ (`World` trait já em L1;
  `Tracked` autorizado).
- `introspector: Protected<Tracked<'a, dyn Introspector + 'a>>` ✗
  (`Introspector`, `Protected`).
- `traced: Tracked<'a, Traced>` ✗ (struct material).
- `sink: TrackedMut<'a, Sink>` ✗.
- `route: Route<'a>` ✗.

---

## 4. Grafo de dependências

```
Traced       -> depende de: Span (✓ L1), FileId (✓ L1)
             -> PRONTO A MATERIALIZAR.

Route        -> depende de: FileId (✓), AtomicUsize (✓ std),
                            Tracked (✓ ADR-0001 autorizado),
                            comemo::Track (✓)
             -> PRONTO A MATERIALIZAR.

Sink         -> depende de: SourceDiagnostic (✓ L1),
                            FxHashSet (✓ ADR-0018),
                            Value (✓ L1),
                            EcoVec (✗ não autorizado em L1),
                            Introspection (✗ pendente),
                            Styles (✗ pendente — bloqueado por EcoVec/Style/LazyHash)
             -> BLOQUEADO por 3 dependências.

Styles       -> depende de: EcoVec (✗), LazyHash (✗), Style (✗)
             -> BLOQUEADO por 3 dependências externas/pendentes.

Routines     -> depende de: NativeRuleMap (✗), Color (✗),
                            Tracked<dyn World> (✓), TrackedMut<Sink> (precisa Sink),
                            Tracked<dyn Introspector> (precisa Introspector),
                            Context (✗), SyntaxMode (✓ L1),
                            Scope (✓ L1), Closure (✓ L1), Func (✓ L1),
                            LazyHash (✗), Args (✓ L1), Module (✓ L1),
                            RealizationKind (✗), SplitLocator (✗),
                            Arenas (✗), StyleChain (parcial — existe em L1
                              mas a vanilla é diferente), Pair (✗),
                            Frame (✗), Locator (✗), Region (✗)
             -> BLOQUEADO por 12+ dependências.

Engine       -> depende de: Routines, Introspector, Traced, Sink, Route,
                            Protected, World (✓ L1), Tracked* (✓)
             -> BLOQUEADO por 6 tipos (5 stubs L1 + Protected externo +
                Introspector pendente).
```

Ordem topológica derivável:

1. **Folhas prontas**: `Traced`, `Route`.
2. **Próximo nível**: `Introspector` (trait pendente), `Style`/`LazyHash`/decisão sobre `EcoVec`.
3. **Após (2)**: `Styles`, `Sink`.
4. **Após (3)**: `Engine` (precisa Sink+Traced+Route+Introspector).
5. **Independente mas profundo**: `Routines` — depende de muitos tipos
   de layout/realize que são separados da cadeia Engine→Sink.

---

## 5. Tipos pendentes identificados

Lista consolidada de tipos vanilla referenciados pelos campos dos
stubs e ainda não materializados em L1:

| Tipo pendente | Localização vanilla |
|---------------|---------------------|
| `Introspection` | `crates/typst-library/src/introspection/convergence.rs:148` (`Arc<dyn Bounds>`) |
| `Introspector` (trait) | `crates/typst-library/src/introspection/introspector.rs:29` |
| `Style` | `crates/typst-library/src/foundations/styles.rs` (mesmo ficheiro do `Styles`) |
| `LazyHash<T>` | `crates/typst-utils/src/` (não inspeccionado neste passo) |
| `Context<'a>` | `crates/typst-library/src/foundations/context.rs:35` (`#[comemo::track]`) |
| `Locator<'a>` | `crates/typst-library/src/introspection/locator.rs:208` (`#[comemo::track]`) |
| `SplitLocator` | mesma família que `Locator` |
| `Closure` | `crates/typst-library/src/foundations/` |
| `NativeRuleMap` | `crates/typst-library/src/foundations/` |
| `RealizationKind<'a>` | `crates/typst-library/src/routines.rs:110` |
| `Arenas` | `crates/typst-library/src/foundations/` |
| `StyleChain<'a>` | parcial em L1 (`01_core/src/entities/style_chain.rs`); vanilla é diferente |
| `Pair<'a>` | usado em `realize()` — `crates/typst-library/src/foundations/` |
| `Frame` | `crates/typst-library/src/layout/` |
| `Region` | `crates/typst-library/src/layout/` |
| `Color` | `crates/typst-library/src/visualize/` |
| `DocumentInfo` | `crates/typst-library/src/model/` |
| `FragmentKind` | `crates/typst-library/src/foundations/` |
| `Protected<T>` | `crates/typst-utils/src/` (wrapper externo) |

Externos não autorizados em L1 a registar:
- `ecow::EcoVec` — usado por `Sink`, `Styles`, `Routines`. ADR-0024
  só autorizou `EcoString`, não `EcoVec`. Decisão pendente.
- `typst_utils::LazyHash` — usado por `Styles`, `Routines`,
  `World::library()`/`book()`. Pendente.
- `typst_utils::Protected` — usado por `Engine`. Pendente.

---

## 6. Candidatos "prontos a materializar"

Stubs cujas dependências estão **todas disponíveis** ou são externas
**autorizadas**:

### 6.1 `Traced` ✓ pronto

Materialização requer:
- Mudar `pub struct Traced(())` para `pub struct Traced(Option<Span>)`.
- Mudar `#[comemo::track] impl Traced {}` para incluir
  `pub fn get(&self, id: FileId) -> Option<Span>`.
- Mudar `Traced::new()` para aceitar `Span` (ou manter
  `Default::default()` para "não rastrear nada").
- Adicionar `#[derive(Default)]`.

Custo estimado: **trivial**. Toca em `world_types.rs` e nada mais.
Diagnóstico vanilla já incluído neste documento (secção 2.2). Sem
ADR adicional necessária — `Span`, `FileId` e `Option` já estão em L1.

### 6.2 `Route` ✓ pronto (com nota)

Materialização requer:
- Mudar `pub struct Route(())` para a forma vanilla com `outer`, `id`,
  `len`, `upper`.
- Importar `std::sync::atomic::AtomicUsize`.
- Adicionar bloco `#[comemo::track] impl<'a> Route<'a>` com `contains`
  e `within`.
- Adicionar bloco fora-de-track com `root`, `extend`, `with_id`,
  `unnested`, `track`, `increase`, `decrease`.
- Adicionar limites `MAX_*_DEPTH` e métodos `check_*_depth`.

Custo estimado: **médio**. Diagnóstico vanilla já completo no Passo 85.
Decisão pendente: preservar todos os 4 níveis de profundidade do vanilla
(`SHOW_RULE`, `LAYOUT`, `HTML`, `CALL`) ou só o `CALL` que o cristalino
já tem. Nota: a materialização de `Route` torna o `EvalContext.import_stack`
+ `ImportGuard` actuais redundantes (DEBT-40).

---

## Referências

- `01_core/src/entities/world_types.rs:106,121,140,156,175,193`
  — declarações dos 6 stubs.
- `01_core/src/rules/eval.rs:37,251–255,1719–1722` — pontos de uso.
- `lab/typst-original/crates/typst-library/src/engine.rs:19,120,150,251`
  — `Engine`, `Traced`, `Sink`, `Route`.
- `lab/typst-original/crates/typst-library/src/routines.rs:31`
  — `Routines` macro-gerado.
- `lab/typst-original/crates/typst-library/src/foundations/styles.rs:24`
  — `Styles`.
- `00_nucleo/diagnosticos/diagnostico-route-vanilla-passo-85.md`
  — diagnóstico detalhado do `Route`.
- `00_nucleo/adr/typst-adr-0001-*.md` — `comemo` autorizado em L1.
- `00_nucleo/adr/typst-adr-0017-*.md` — origem dos stubs `Tipo(())`.
- `00_nucleo/adr/typst-adr-0018-*.md` — `rustc_hash` autorizado.
- `00_nucleo/adr/typst-adr-0024-*.md` — `EcoString` autorizado (mas não `EcoVec`).
