# Passo 17 — Recursão, stdlib nativa mínima e named args

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — closures do Passo 16
- `01_core/src/entities/func.rs` — `Func(Arc<FuncRepr>)`
- `lab/typst-original/crates/typst-library/src/foundations/func.rs`

Pré-condição: `cargo test` — 279 testes (257 L1 + 22 L3), zero violations.

Três tarefas com uma sequência preferida:
1. **Recursão** — auto-injecção no `call_scope` (evita ciclo Arc)
2. **Stdlib nativa mínima** — `type()`, `len()`, `range()` com interface `&[Value]`
3. **Named args** — completar `Args`

---

## Tarefa 1 — Diagnóstico de recursão

**Parar aqui. Reportar output antes de implementar.**

```bash
# Como o original resolve recursão em closures
grep -rn "recursive\|self.*ref\|name.*closure\|closure.*name\b" \
  lab/typst-original/crates/typst-eval/src/ | head -20

# Campo 'name' em ClosureRepr do original?
grep -n "name\b" \
  lab/typst-original/crates/typst-library/src/foundations/func.rs \
  | head -15

# Limite de profundidade no original
grep -rn "depth\|MAX.*CALL\|call.*limit\|stack.*limit" \
  lab/typst-original/crates/typst-eval/src/ | head -15
```

---

## Tarefa 2 — EvalContext e limite de profundidade

Antes de implementar recursão, introduzir `EvalContext` para carregar
o estado de execução que antes era passado como parâmetros avulsos.
Isto limpa a assinatura de `eval_expr` e é o lugar natural para o
contador de profundidade.

```rust
// 01_core/src/rules/eval.rs (ou rules/eval_context.rs)

/// Contexto de execução partilhado durante eval().
///
/// Introduzido no Passo 17 para suportar o limite de profundidade
/// de chamada. Substituirá parâmetros avulsos à medida que crescer.
pub(crate) struct EvalContext<'w> {
    pub world:  Tracked<'w, dyn TrackedWorld + 'w>,
    pub depth:  usize,
}

impl<'w> EvalContext<'w> {
    pub fn new(world: Tracked<'w, dyn TrackedWorld + 'w>) -> Self {
        Self { world, depth: 0 }
    }

    /// Entra numa chamada de função — retorna Err se profundidade excedida.
    pub fn enter_call(&mut self, span: Span) -> SourceResult<()> {
        self.depth += 1;
        if self.depth > MAX_CALL_DEPTH {
            Err(vec![SourceDiagnostic::error(
                span,
                format!("profundidade máxima de chamada ({MAX_CALL_DEPTH}) excedida"),
            )])
        } else {
            Ok(())
        }
    }

    pub fn leave_call(&mut self) {
        self.depth = self.depth.saturating_sub(1);
    }
}

const MAX_CALL_DEPTH: usize = 1_000;
```

Actualizar `eval_expr` para receber `&mut EvalContext` em vez de
`Tracked<dyn TrackedWorld>` directamente:

```rust
fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    // world acessível via ctx.world
}
```

E `eval()` público:

```rust
pub fn eval(
    _routines: &Routines,
    world: Tracked<dyn TrackedWorld + '_>,
    _traced: Tracked<Traced>,
    _sink: TrackedMut<Sink>,
    _route: Tracked<Route>,
    source: &Source,
) -> SourceResult<Module> {
    let mut ctx = EvalContext::new(world);
    let global = make_stdlib();
    let mut scopes = Scopes::new(&global);
    scopes.push();
    eval_markup(source.root(), &mut scopes, &mut ctx)?;
    let top = scopes.pop().unwrap_or_default();
    Ok(Module::new(source.id().into_raw().get().to_string(), top))
}
```

---

## Tarefa 3 — Recursão via auto-injecção

### Estratégia: campo `name` em ClosureRepr + injecção em call_scope

Esta estratégia é preferida à `Arc::make_mut` porque evita
completamente o ciclo Arc. A referência recursiva vive apenas
enquanto a chamada está activa — o `call_scope` é destruído
ao retornar, quebrando o ciclo naturalmente sem `Weak<T>` ou
`RefCell`.

```rust
// Em entities/func.rs — adicionar campo name:
pub struct ClosureRepr {
    pub name:     Option<String>,  // nome da binding para recursão
    pub params:   Vec<ClosureParam>,
    pub body:     SyntaxNode,
    pub captured: IndexMap<String, Value, FxBuildHasher>,
}
```

Em `eval_let`, após definir no scope, preencher o nome:

```rust
// Em eval_let — após criar a closure:
if let Value::Func(ref func) = value {
    if let FuncRepr::Closure(ref mut closure) = Arc::make_mut(&mut func.0.clone()) {
        // Não mutar o Arc existente — criar nova versão com nome
    }
}
// Alternativa mais simples: definir nome no momento da construção
// Se init() produz uma closure, o nome vem do LetBinding
```

**Abordagem mais simples**: no `eval_let`, se o valor resultante é
`Value::Func(Closure)`, criar uma nova `Func` com o nome preenchido
antes de definir no scope:

```rust
Expr::LetBinding(binding) => {
    let mut value = match binding.init() {
        Some(init) => eval_expr(init, scopes, ctx)?,
        None       => Value::None,
    };

    if let LetBindingKind::Normal(pattern) = binding.kind() {
        if let Some(ident) = pattern.ident() {
            let name = ident.as_str().to_string();

            // Se a closure ainda não tem nome, dar-lhe o nome da binding
            if let Value::Func(ref mut func) = value {
                func.set_name(name.clone());  // adicionar método set_name a Func
            }

            scopes.define(name, value);
        }
    }
    Ok(Value::None)
}
```

```rust
// Em entities/func.rs — adicionar set_name:
impl Func {
    pub fn set_name(&mut self, name: String) {
        if let Ok(inner) = Arc::get_mut(&mut self.0) {
            if let FuncRepr::Closure(ref mut c) = inner {
                if c.name.is_none() {
                    c.name = Some(name);
                }
            }
        }
        // Se Arc tem múltiplas referências (já foi clonado), não mutar
        // A closure já estará capturada em outro lugar — nome não necessário
    }
}
```

Em `apply_closure` — injectar o próprio Func no call_scope:

```rust
fn apply_closure(
    closure: &ClosureRepr,
    func: &Func,  // passar referência à própria Func
    args: Args,
    scopes_outer: &Scopes<'_>,
    ctx: &mut EvalContext<'_>,
) -> SourceResult<Value> {
    ctx.enter_call(closure.body.span())?;

    let mut call_scope = Scope::new();

    // Variáveis capturadas
    for (name, value) in &closure.captured {
        call_scope.define(name.clone(), value.clone());
    }

    // Auto-injecção para recursão — referência vive apenas durante esta chamada
    // Arc::clone é O(1); destruído quando call_scope sai do scope
    if let Some(ref name) = closure.name {
        call_scope.define(name.clone(), Value::Func(func.clone()));
    }

    // Parâmetros posicionais
    for (i, param) in closure.params.iter().enumerate() {
        let val = args.items.get(i).cloned()
            .or_else(|| args.named.get(param.name.as_str()).cloned())
            .or_else(|| param.default.clone())
            .unwrap_or(Value::None);
        call_scope.define(param.name.clone(), val);
    }

    let global = Scope::new();
    let mut call_scopes = Scopes::new(&global);
    call_scopes.push_scope(call_scope);

    let result = if let Some(expr) = Expr::from_untyped(&closure.body) {
        eval_expr(expr, &mut call_scopes, ctx)
    } else {
        Ok(Value::None)
    };

    ctx.leave_call();
    result
}
```

### Testes de recursão

```rust
#[test]
fn recursao_factorial() {
    let world = MockWorld::new(
        "#let fact = (n) => if n <= 1 { 1 } else { n * fact(n - 1) }\n\
         #let r = fact(5)"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"), Some(&Value::Int(120)));
}

#[test]
fn recursao_fibonacci() {
    let world = MockWorld::new(
        "#let fib = (n) => if n <= 1 { n } else { fib(n - 1) + fib(n - 2) }\n\
         #let r = fib(7)"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"), Some(&Value::Int(13)));
}

/// Teste de estabilidade — valida que recursão infinita não faz crash.
/// Um Stack Overflow é inaceitável em servidor; Err é a falha correcta.
#[test]
fn recursao_infinita_retorna_err_sem_crash() {
    let world = MockWorld::new(
        "#let inf = (n) => inf(n + 1)\n\
         #let r = inf(0)"
    );
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    assert!(result.is_err(), "recursão infinita deve Err, não crash");
    let msg = &result.unwrap_err()[0].message;
    assert!(msg.contains("profundidade") || msg.contains("depth") || msg.contains("1000"),
        "mensagem deve mencionar limite: {:?}", msg);
}
```

---

## Tarefa 4 — Stdlib nativa mínima

### Interface `&[Value]` — padrão de bridge

A assinatura `fn(&[Value]) -> SourceResult<Value>` é o padrão de
bridge entre Rust e o motor Typst. Vantagens:
- Sem moves — `&[Value]` é uma view sobre os args
- Fácil de expandir: `max`, `min`, `abs`, `str.len()` seguem o mesmo padrão
- Testável sem `Args` completo

```rust
// Em entities/func.rs:
pub struct NativeFunc {
    pub name: &'static str,
    pub call: fn(&[Value]) -> SourceResult<Value>,
}

impl Func {
    pub fn native(name: &'static str, call: fn(&[Value]) -> SourceResult<Value>) -> Self {
        Self(Arc::new(FuncRepr::Native(NativeFunc { name, call })))
    }
}
```

Em `apply_closure_func` (renomear para `apply_func`):

```rust
fn apply_func(func: Func, args: Args, ctx: &mut EvalContext<'_>) -> SourceResult<Value> {
    match func.repr() {
        FuncRepr::Closure(closure) => apply_closure(closure, &func, args, ctx),
        FuncRepr::Native(native)   => (native.call)(&args.items),
    }
}
```

### Implementar as 3 nativas

```rust
// Em rules/stdlib.rs (ficheiro separado para isolamento)

use crate::entities::{
    source_result::{SourceDiagnostic, SourceResult},
    span::Span,
    value::Value,
};

fn err(msg: impl Into<String>) -> SourceResult<Value> {
    Err(vec![SourceDiagnostic::error(Span::detached(), msg.into())])
}

pub fn native_type(args: &[Value]) -> SourceResult<Value> {
    match args {
        [v] => Ok(Value::Str(v.type_name().into())),
        _   => err(format!("type() requer 1 argumento, recebeu {}", args.len())),
    }
}

pub fn native_len(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Str(s)]   => Ok(Value::Int(s.chars().count() as i64)),
        [Value::Array(a)] => Ok(Value::Int(a.len() as i64)),
        [Value::Dict(d)]  => Ok(Value::Int(d.len() as i64)),
        [other]           => err(format!("len() não suporta {}", other.type_name())),
        _                 => err(format!("len() requer 1 argumento, recebeu {}", args.len())),
    }
}

pub fn native_range(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(n)] => {
            if *n < 0 { return err("range() requer argumento não-negativo"); }
            Ok(Value::Array((0..*n).map(Value::Int).collect()))
        }
        [Value::Int(start), Value::Int(end)] => {
            let items = if start <= end {
                (*start..*end).map(Value::Int).collect()
            } else {
                vec![]
            };
            Ok(Value::Array(items))
        }
        _ => err(format!("range() requer 1 ou 2 Int, recebeu {} args", args.len())),
    }
}
```

**Nota sobre `len()` e strings**: `s.chars().count()` conta
grapheme-like (codepoints), não bytes. O Typst usa graphemes
via `unicode_segmentation` — se necessário para paridade, usar
`.graphemes(true).count()`. Confirmar com o original.

### make_stdlib()

```rust
// Em rules/eval.rs:
fn make_stdlib() -> Scope {
    use crate::rules::stdlib::*;
    let mut scope = Scope::new();
    scope.define("type",  Value::Func(Func::native("type",  native_type)));
    scope.define("len",   Value::Func(Func::native("len",   native_len)));
    scope.define("range", Value::Func(Func::native("range", native_range)));
    scope
}
```

### Testes da stdlib

```rust
#[test]
fn stdlib_type_int() {
    let world = MockWorld::new("#let t = type(42)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("t"), Some(&Value::Str("int".into())));
}

#[test]
fn stdlib_type_func() {
    let world = MockWorld::new("#let f = () => 1\n#let t = type(f)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("t"), Some(&Value::Str("function".into())));
}

#[test]
fn stdlib_range_simples() {
    let world = MockWorld::new("#let r = range(3)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"),
               Some(&Value::Array(vec![Value::Int(0), Value::Int(1), Value::Int(2)])));
}

#[test]
fn stdlib_range_vazio_se_start_eq_end() {
    let world = MockWorld::new("#let r = range(3, 3)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"), Some(&Value::Array(vec![])));
}

#[test]
fn for_com_range_integrado() {
    // Verificar que for + range funciona end-to-end
    // Typst não tem mutação — este teste verifica apenas que não falha
    let world = MockWorld::new(
        "#for i in range(3) { }"
    );
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_ok());
}

/// Testes directos de nativas — sem world, sem eval_for_test.
/// Usa a interface &[Value] directamente.
#[test]
fn native_type_directo() {
    assert_eq!(native_type(&[Value::Int(1)]),    Ok(Value::Str("int".into())));
    assert_eq!(native_type(&[Value::Bool(true)]),Ok(Value::Str("bool".into())));
    assert!(native_type(&[]).is_err());
    assert!(native_type(&[Value::Int(1), Value::Int(2)]).is_err());
}

#[test]
fn native_len_directo() {
    assert_eq!(native_len(&[Value::Str("abc".into())]), Ok(Value::Int(3)));
    assert_eq!(native_len(&[Value::Array(vec![Value::Int(1), Value::Int(2)])]),
               Ok(Value::Int(2)));
    assert!(native_len(&[Value::Int(1)]).is_err());
}

#[test]
fn native_range_directo() {
    assert_eq!(native_range(&[Value::Int(3)]),
               Ok(Value::Array(vec![Value::Int(0),Value::Int(1),Value::Int(2)])));
    assert_eq!(native_range(&[Value::Int(2), Value::Int(5)]),
               Ok(Value::Array(vec![Value::Int(2),Value::Int(3),Value::Int(4)])));
    assert!(native_range(&[Value::Int(-1)]).is_err());
}
```

---

## Tarefa 5 — Named args

### Expandir Args

```rust
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub items: Vec<Value>,
    pub named: IndexMap<EcoString, Value, FxBuildHasher>,
}

impl Args {
    pub fn positional(items: Vec<Value>) -> Self {
        Self { items, named: IndexMap::default() }
    }
}
```

### eval_args actualizado

```rust
fn eval_args(args_node: ast::Args<'_>, scopes: &mut Scopes<'_>, ctx: &mut EvalContext<'_>)
    -> SourceResult<Args>
{
    let mut items = Vec::new();
    let mut named = IndexMap::default();
    for arg in args_node.items() {
        match arg {
            ast::Arg::Pos(expr) =>
                items.push(eval_expr(expr, scopes, ctx)?),
            ast::Arg::Named(name, expr) =>
                { named.insert(name.as_str().into(), eval_expr(expr, scopes, ctx)?); }
            ast::Arg::Spread(_) => {}  // fronteira deliberada
        }
    }
    Ok(Args { items, named })
}
```

### Testes de named args

```rust
#[test]
fn named_arg_simples() {
    let world = MockWorld::new(
        "#let greet = (prefix: \"Hi\", name) => prefix\n\
         #let r = greet(\"world\", prefix: \"Hello\")"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"), Some(&Value::Str("Hello".into())));
}
```

---

## Verificação final

```bash
cargo test -p typst-core
cargo test -p typst-infra
cargo build
crystalline-lint .
crystalline-lint --fix-hashes .
crystalline-lint .
# ✓ No violations found
```

Critérios de conclusão obrigatórios:
- `fact(5)` → `Value::Int(120)` ✓
- `recursao_infinita_retorna_err_sem_crash` — `Err(...)`, não crash ✓
- `native_type_directo` — interface `&[Value]` funcional ✓
- `native_range_directo` — range(3) → [0,1,2] ✓
- `EvalContext` com `depth` rastreado ✓
- Zero violations ✓
- Testes não regridem (279 base + novos) ✓

---

## Ao terminar, reportar

**Da recursão:**
- Abordagem usada para `set_name` — `Arc::get_mut` funcionou?
- Se `EvalContext` simplificou a assinatura de `eval_expr`
- Se `fibonacci(7) = 13` passou (valida profundidade até ~30 chamadas)

**Da stdlib:**
- Se `&[Value]` compilou directamente com os function pointers
- Se `len()` para strings usa `.chars().count()` ou `.graphemes()`
- Se `for + range` funciona end-to-end

**Dos named args:**
- API real de `ast::Arg::Named` — tem campo de nome e expr separados?

**Número total de testes e zero violations confirmado.**

**Go/No-Go para o Passo 18:**
- **GO — Content mínimo**: motor de eval() completo com recursão e
  stdlib; Passo 18 migra `Content` com `Text` para validar o pipeline
  completo parse→eval→layout
- **GO — mais stdlib**: se Content pode esperar, Passo 18 expande
  com `str.len()`, `array.map()`, `int.abs()` via method calls em FieldAccess
- **NO-GO — EvalContext bloqueado**: se a refactorização de `eval_expr`
  para usar `EvalContext` criou problemas de lifetime; Passo 18 resolve
  antes de avançar
