# Prompt L0 — entities/func e entities/args
Hash do Código: a7ea5b1b

**Camada**: L1
**Ficheiros alvo**: `01_core/src/entities/func.rs`, `01_core/src/entities/args.rs`
**ADRs relevantes**: ADR-0016 (adiamento Routines), ADR-0017 (adiamento eval completo)

## Contexto

`Func` representa uma função Typst — neste passo apenas closures definidas
no documento. Funções nativas (built-ins) ficam para quando `Routines` real
migrar (ADR-0016). `Args` representa os argumentos de uma chamada de função.

## Tipos públicos

### Func

```rust
#[derive(Clone)]
pub struct Func(Arc<FuncRepr>);

pub(crate) enum FuncRepr {
    Closure(ClosureRepr),
    // Native — variante futura, ADR-0016
}

pub struct ClosureRepr {
    pub params: Vec<ClosureParam>,
    pub body: SyntaxNode,           // clone O(1) via Arc interno
    pub captured: IndexMap<String, Value, FxBuildHasher>,
}

pub struct ClosureParam {
    pub name:    String,
    pub default: Option<Value>,
}
```

`Func(Arc<FuncRepr>)` — clone O(1), consistente com `Module`.

`ClosureRepr.captured` — eager snapshot do scope no momento da definição.
Semântica: captura por valor, não por referência. Divergência do original
(que usa `comemo` para lazy access) — registada em DEBT.md.

### Args

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub items: Vec<Value>,
}
```

Apenas args posicionais neste passo. Named args e spread adiados (ADR-0016).

## Interface pública de Func

```rust
impl Func {
    pub fn closure(repr: ClosureRepr) -> Self;
    pub(crate) fn repr(&self) -> &FuncRepr;
}

impl std::fmt::Debug for Func { /* "<function>" */ }
impl PartialEq for Func { /* Arc::ptr_eq */ }
impl Clone for Func { /* derive */ }
```

## Interface pública de Args

```rust
impl Args {
    pub fn positional(items: Vec<Value>) -> Self;
    pub fn len(&self) -> usize;
    pub fn is_empty(&self) -> bool;
}
```

## Semântica confirmada

- **PartialEq por identidade**: duas `Func` são iguais se e só se partilham
  o mesmo `Arc<FuncRepr>` (mesmo ponteiro). Consistente com `Module`.
- **Clone O(1)**: `Arc::clone` — não clona o conteúdo da closure.
- **Debug**: `"<function>"` (string literal, nunca pânico).
- **Eager capture**: `ClosureRepr.captured` é um snapshot imutável;
  redefinições posteriores no scope pai não afectam a closure.

## Critérios de Verificação

```
Func::closure(...).clone() == Func::closure(...)  → false (Arc distintos)
let f1 = Func::closure(...); let f2 = f1.clone(); f1 == f2  → true (mesmo Arc)
format!("{:?}", func)  → "<function>"
Args::positional(vec![]).is_empty()  → true
Args::positional(vec![Value::Int(1)]).len()  → 1
```
