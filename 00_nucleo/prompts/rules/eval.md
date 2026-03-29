# Prompt L0 — rules/eval

**Camada**: L1
**Ficheiro alvo**: `01_core/src/rules/eval.rs`
**ADRs relevantes**: ADR-0017 (adiamento eval), ADR-0001 (comemo em L1), ADR-0024 (ecow/Value::Str)

## Contexto

`eval()` é o motor de avaliação do compilador Typst. Recebe uma `Source`
e retorna um `Module` com os bindings definidos nesse ficheiro.

**Estado actual (Passo 15)**: travessia AST com control flow e ADR-0025.
Avalia literais, Ident, Let, CodeBlock, Binary, Unary, Conditional (if/else),
WhileLoop, ForLoop (apenas Array). Fronteira deliberada: `_ => Ok(Value::None)`
para nós que requerem Content, Func, Styles.

## Assinatura pública

```rust
pub fn eval(
    _routines: &Routines,
    world: Tracked<dyn TrackedWorld + '_>,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module>
```

**Invariante**: `eval.rs` não importa nada de `03_infra`. Acesso ao
world sempre via `TrackedWorld` (L1).

## Variantes de Expr suportadas

- `Expr::Int` → `Value::Int(node.get())`
- `Expr::Float` → `Value::Float(node.get())`
- `Expr::Str` → `Value::Str(EcoString::from(node.get()))`
- `Expr::Bool` → `Value::Bool(node.get())`
- `Expr::None` → `Value::None`
- `Expr::Ident` → lookup em Scopes, erro se não encontrado
- `Expr::LetBinding` → eval_let: avalia init, define no scope activo
- `Expr::CodeBlock` → evalua exprs sequencialmente via `body().exprs()`
- `Expr::Binary(binary)` → eval_binary_op(binary.op(), lhs, rhs)
- `Expr::Unary(unary)` → eval_unary_op(unary.op(), operand)
- `Expr::Conditional(cond)` → eval_conditional: condition(), if_body(), else_body()
- `Expr::WhileLoop(loop)` → eval_while: MAX_ITER=10_000 limite de segurança
- `Expr::ForLoop(loop)` → eval_for: iterable() (não iter()), pattern().bindings(),
  body(); Value::None tratado como iterável vazio (sem parsing de array literal)

## Fronteira deliberada

`_ => Ok(Value::None)` — nós não implementados retornam None sem erro.
Permite encontrar `#let x = 1` dentro de markup sem avaliar texto puro.
Requer Content, Func, Styles para implementação completa (ADR-0017).

## Semântica Typst confirmada (ops.rs de referência)

- **Int/Int divisão → Float**: `5/2 = 2.5` (não truncamento)
- **Int overflow → Err**: `checked_add/sub/mul/neg`, mensagem "number too large"
- **Float → IEEE 754**: NaN e Inf propagados silenciosamente (sem guarda)
- **Divisão por zero → Err**: verificação `is_zero` antes do match
- **ADR-0025 — `Int == Float` → true em eval**: coerção explícita em eval_binary_op
  antes do wildcard `(Eq, a, b)`. `derive(PartialEq)` mantido para estruturas de dados.
  Coerção aplica-se também a ordenação (lt/leq/gt/geq com Int↔Float).
- **BinOp variants**: `Add, Sub, Mul, Div, And, Or, Eq, Neq, Lt, Leq, Gt, Geq,
  Assign, In, NotIn, AddAssign, SubAssign, MulAssign, DivAssign`
- **UnOp variants**: `Pos, Neg, Not`

## Integração com comemo — Cenário D (confirmado)

`eval_for_test<W: TrackedWorld>` em `#[cfg(test)]` coerce `&W` → `&dyn TrackedWorld`
e chama `dyn_world.track()` para obter `Tracked<dyn TrackedWorld>`. O `#[comemo::track]`
em `TrackedWorld` gera `impl Track for dyn TrackedWorld`.

`MockWorld` (local aos testes) implementa `World`; via blanket impl é `TrackedWorld`.

## Notas de implementação

- `Scopes::new` recebe `Option<&'a Library>` — usa `None` (stdlib vazia)
- `Scopes::enter()/exit()` em vez de `push()/pop()`
- `LetBindingKind::Normal(pattern)` → `pattern.bindings()` → primeiro Ident
- `Code::exprs()` já filtra trivia — não chamar `is_trivia()` adicionalmente
- `use BinOp::*` e `use UnOp::*` PROIBIDOS: o linter confunde com imports externos (V14)
  — usar sempre `BinOp::Add`, `UnOp::Neg`, etc. directamente nos braços do match

## Critérios de Verificação

```
eval_binary_op(Add, Int(1), Int(2))     → Ok(Int(3))
eval_binary_op(Add, Str("a"), Str("b")) → Ok(Str("ab"))
eval_binary_op(Div, Int(5), Int(2))     → Ok(Float(2.5))
eval_binary_op(Eq, Int(1), Float(1.0))  → Ok(Bool(false))
eval_binary_op(Div, Int(1), Int(0))     → Err(...)
eval_binary_op(Add, Int(MAX), Int(1))   → Err(...)
eval_unary_op(Not, Bool(true))          → Ok(Bool(false))
eval_for_test: Source("#let x = 1") → module.scope().get("x") = Some(&Value::Int(1))
```
