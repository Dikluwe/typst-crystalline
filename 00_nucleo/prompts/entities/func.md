# Prompt L0 — entities/func e entities/args
Hash do Código: 28ad56cb

**Camada**: L1
**Ficheiros alvo**: `01_core/src/entities/func.rs`, `01_core/src/entities/args.rs`
**ADRs relevantes**: ADR-0016 (adiamento Routines), ADR-0017 (adiamento eval completo), ADR-0107 (paridade linguagem), ADR-0109 (atomização)

## Contexto

`Func` representa uma função Typst — closures definidas no documento, funções nativas (built-ins) e construtores de elemento de utilizador. `Args` representa os argumentos de uma chamada de função.

## Tipos públicos

### Func

```rust
#[derive(Clone)]
pub struct Func(pub(crate) Arc<FuncRepr>);

pub(crate) enum FuncRepr {
    Closure(ClosureRepr),
    Native(NativeFunc),
    /// P394 — native function com acesso ao `Scopes` e `Engine` actuais.
    NativeWithEngine(NativeFuncWithEngine),
    /// Lote F-3 inc-2 — construtor de elemento de utilizador.
    Element(ElementFunc),
    /// **P699** — função de plugin WASM. O nome do export é dinâmico, logo não
    /// cabe num fn-ptr nativo (`Native`/`NativeWithEngine`); a chamada é
    /// delegada ao `PluginHost` capturado. Ver `entities/plugin_func.md`.
    Plugin(crate::entities::plugin_func::PluginFunc),
}

pub struct ClosureRepr {
    pub name: Option<String>,
    pub params: Vec<ClosureParam>,
    pub body: SyntaxNode,           // clone O(1) via Arc interno
    pub captured: Arc<Scope>,       // eager snapshot do scope no momento da definição
}

pub struct ClosureParam {
    pub name:    String,
    pub default: Option<Value>,
}

/// Função nativa implementada em Rust.
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(&mut crate::rules::eval::EvalContext, &Args, &dyn crate::contracts::world::World, FileId) -> SourceResult<Value>,
}

/// P394 — native function com acesso ao `Scopes` e `Engine` actuais.
pub struct NativeFuncWithEngine {
    pub name: &'static str,
    pub call: fn(&mut crate::rules::eval::EvalContext, &Args, &dyn crate::contracts::world::World, FileId, &mut crate::rules::scopes::Scopes<'_>, &mut crate::entities::engine::Engine<'_>) -> SourceResult<Value>,
}

/// Lote F-3 inc-2 — construtor de elemento de utilizador.
pub struct ElementFunc {
    pub name: String,
    pub ctor: crate::entities::element_registry::ElementCtor,
}
```

`Func(Arc<FuncRepr>)` — clone O(1), consistente com `Module`.

`ClosureRepr.captured` — eager snapshot do scope no momento da definição. Semântica: captura por valor, não por referência. Divergência do original (que usa `comemo` para lazy access) — registada em DEBT.md.

## Namespace anexado (P493)

**P493 — Field access em funções com namespace:** algumas funções nativas do Typst expõem sub-funções via field access (ex.: `table.header`, `table.footer`, `table.cell`). Para suportar isto, `Func` pode ter um namespace anexado.

### Representação

Adicionar a `FuncRepr::Native` e `FuncRepr::NativeWithEngine` um campo opcional `namespace: Option<Arc<Scope>>`:

```rust
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(...) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}

pub struct NativeFuncWithEngine {
    pub name: &'static str,
    pub call: fn(...) -> SourceResult<Value>,
    pub namespace: Option<Arc<Scope>>,
}
```

- `namespace: None` — função sem sub-funções (`heading`, `figure`, etc.);
- `namespace: Some(scope)` — função com sub-funções acessíveis via field access (`table`, `list`, `enum`, etc.).

### Construtores

- `Func::native(name, call)` — cria `Func` com `namespace: None`.
- `Func::native_with_namespace(name, call, namespace)` — cria `Func` com namespace anexado.
- `Func::native_with_engine(name, call)` — cria `Func` com `namespace: None`.
- `Func::native_with_engine_and_namespace(name, call, namespace)` — cria `Func` com namespace anexado.

### Acesso

```rust
impl Func {
    /// Retorna o namespace anexado, se existir.
    pub fn namespace(&self) -> Option<&Scope>;
}
```

## Interface pública de Func

```rust
impl Func {
    pub fn closure(repr: ClosureRepr) -> Self;
    pub fn native(name: &'static str, call: NativeFn) -> Self;
    pub fn native_with_namespace(name: &'static str, call: NativeFn, namespace: Arc<Scope>) -> Self;
    pub fn native_with_engine(name: &'static str, call: NativeFnWithEngine) -> Self;
    pub fn native_with_engine_and_namespace(name: &'static str, call: NativeFnWithEngine, namespace: Arc<Scope>) -> Self;
    pub fn element(name: impl Into<String>, ctor: ElementCtor) -> Self;
    /// **P699** — constrói `Func` a partir de um `PluginFunc` (export WASM).
    pub fn plugin(p: crate::entities::plugin_func::PluginFunc) -> Self;
    pub(crate) fn repr(&self) -> &FuncRepr;
    pub fn element_name(&self) -> Option<&str>;
    pub fn name(&self) -> Option<&str>;
    pub fn native_fn_addr(&self) -> Option<NativeFn>;
    pub fn namespace(&self) -> Option<&Scope>;
    pub fn set_name(&mut self, name: String);
}
```

## Interface pública de Args

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub items: Vec<Value>,
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}

impl Args {
    pub fn positional(items: Vec<Value>) -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

## Semântica confirmada

- **PartialEq por identidade**: duas `Func` são iguais se e só se partilham o mesmo `Arc<FuncRepr>` (mesmo ponteiro). Consistente com `Module`.
- **Clone O(1)**: `Arc::clone` — não clona o conteúdo da closure.
- **Debug**: `"<function>"` (string literal, nunca pânico).
- **Eager capture**: `ClosureRepr.captured` é um snapshot imutável; redefinições posteriores no scope pai não afectam a closure.
- **Namespace anexado**: clone O(1) via `Arc<Scope>`; field access em `Func` delega ao `Scope` se existir.

## Variante `Plugin` (P699)

`FuncRepr::Plugin(PluginFunc)` representa um **export de plugin WASM**
chamável como função da linguagem. O nome do export é dinâmico (vem do
módulo WASM), logo não cabe num fn-ptr nativo (`Native`/`NativeWithEngine`);
a chamada é delegada ao `PluginHost` capturado no `PluginFunc`
(ver `entities/plugin_func.md`).

- `Func::plugin(p)` constrói `Func(Arc::new(FuncRepr::Plugin(p)))`.
- `Func::name()` ⇒ `Some(&p.name)` (nome do export; usado em mensagens e
  na rejeição de named args).
- `Func::native_fn_addr()` ⇒ `None` (um plugin não tem endereço de fn
  nativa).
- `Func::namespace()` ⇒ `None` (um plugin não expõe sub-funções via field
  access neste passo).
- A aplicação (`apply_func` em `rules/eval/closures.rs`) delega a
  `call_plugin(p, args, span)`; ver `rules/stdlib/plugin.md`.

## Critérios de Verificação

```
Func::closure(...).clone() == Func::closure(...)  → false (Arc distintos)
let f1 = Func::closure(...); let f2 = f1.clone(); f1 == f2  → true (mesmo Arc)
format!("{:?}", func)  → "<function>"
Args::positional(vec![]).is_empty()  → true
Args::positional(vec![Value::Int(1)]).len()  → 1
```

## Scope-outs

- Namespace anexado não se aplica a closures (`FuncRepr::Closure`) nem a elementos de utilizador (`FuncRepr::Element`) neste passo.
- Não implementar field access mutável (set) no namespace.
