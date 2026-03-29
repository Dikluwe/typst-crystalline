# Passo 16 — Closures simples e FuncCall em eval()

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — control flow funcional do Passo 15
- `01_core/src/entities/value.rs` — 9 variantes actuais
- `lab/typst-original/crates/typst-library/src/foundations/func.rs`
- `lab/typst-original/crates/typst-library/src/foundations/args.rs`

Pré-condição: `cargo test` — 243 testes (221 L1 + 22 L3), zero violations.

Este passo introduz `Value::Func` — closures definidas no documento.
Funções nativas da stdlib (`print`, `range`, `type`, etc.) ficam para
quando `Routines` real migrar. Não antecipar.

---

## Tarefa 1 — Diagnóstico de Func, Args e Closure

**Parar aqui. Reportar output antes de qualquer código.**

```bash
# Estrutura de Func — enum com vtable ou struct?
grep -A 30 "^pub struct Func\b\|^pub enum Func\b" \
  lab/typst-original/crates/typst-library/src/foundations/func.rs \
  | head -35

# Dependências externas de func.rs
grep "^use\|^extern" \
  lab/typst-original/crates/typst-library/src/foundations/func.rs \
  | grep -v "crate::\|super::\|std::" | head -20

# Closure — separada de Func ou interna?
grep -rn "^pub struct Closure\|struct Closure\b" \
  lab/typst-original/crates/typst-library/src/foundations/ | head -5
grep -rA 20 "^pub struct Closure\b" \
  lab/typst-original/crates/typst-library/src/foundations/ | head -25

# Eager vs lazy capture: o original guarda Scope por valor ou referência?
grep -n "captured\|capture\|env\|closure.*scope\|Arc.*Scope" \
  lab/typst-original/crates/typst-library/src/foundations/func.rs \
  lab/typst-original/crates/typst-eval/src/ 2>/dev/null | head -20

# Estrutura de Args — simples ou complexa?
grep -A 20 "^pub struct Args\b" \
  lab/typst-original/crates/typst-library/src/foundations/args.rs \
  | head -25

# API de ast::Args e ast::Param na AST cristalina
grep -n "pub fn\|Param\|Args\b" \
  01_core/src/entities/ast/expr.rs | head -30

# Como eval() original aplica uma closure
grep -rn "apply\|call.*closure\|Closure.*apply" \
  lab/typst-original/crates/typst-eval/src/ | head -20
```

### Questões críticas

1. **Eager vs lazy capture**: o original captura o scope por valor
   (snapshot) ou usa `comemo` para lazy capture por referência?
   O cristalino usa eager capture — se divergir em casos de shadowing,
   registar em DEBT.md, não tentar emular comemo na closure.
2. **`SyntaxNode` vs `Span` no body**: confirmar se guardar
   `SyntaxNode` (clone O(1) via Arc) é suficiente para re-avaliar
   o body de uma closure.
3. **Args**: é possível simplificar para `Vec<Value>` neste passo,
   ou a API de `ast::Args` expõe named args de forma que força
   tratamento?

---

## Tarefa 2 — Representação de Func e ClosureRepr

### Func — apenas Closure neste passo

```rust
// 01_core/src/entities/func.rs (ou rules/func.rs — decidir após diagnóstico)

use std::sync::Arc;
use crate::entities::syntax_node::SyntaxNode;
use crate::entities::value::Value;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

/// Função Typst — subset: apenas closures definidas no documento.
///
/// Funções nativas (built-ins) adicionadas quando Routines migrar.
/// Ver ADR-0016.
#[derive(Clone)]
pub struct Func(Arc<FuncRepr>);

enum FuncRepr {
    Closure(ClosureRepr),
    // Native(NativeRepr),  // variante futura — não implementar agora
}

pub struct ClosureRepr {
    /// Parâmetros com nomes e defaults opcionais.
    pub params: Vec<ClosureParam>,
    /// Corpo da closure — SyntaxNode clone O(1) via Arc interno.
    /// Guardado directamente em vez de Span para evitar re-parse.
    pub body: SyntaxNode,
    /// Variáveis capturadas — eager snapshot do scope no momento da definição.
    ///
    /// SEMÂNTICA: captura por valor, não por referência.
    /// Se o scope pai for mutado após a definição da closure, a closure
    /// continua a ver os valores do momento da captura. Isto é comportamento
    /// deliberado — divergência do original (que usa comemo) registada em DEBT.md.
    pub captured: IndexMap<String, Value, FxBuildHasher>,
}

pub struct ClosureParam {
    pub name:    String,
    pub default: Option<Value>,
}

impl Func {
    pub fn closure(repr: ClosureRepr) -> Self {
        Self(Arc::new(FuncRepr::Closure(repr)))
    }

    pub(crate) fn repr(&self) -> &FuncRepr {
        &self.0
    }
}

// Debug manual — Arc não implementa Debug automaticamente
impl std::fmt::Debug for Func {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<function>")
    }
}

// PartialEq por identidade de ponteiro — duas Func são iguais se
// são o mesmo objecto Arc. Consistente com Module (ADR do Passo 15).
impl PartialEq for Func {
    fn eq(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.0, &other.0)
    }
}
```

### Args simplificado

```rust
// 01_core/src/entities/args.rs (ou rules/args.rs)

/// Argumentos de chamada de função.
///
/// Subset: apenas args posicionais neste passo.
/// Named args e spread adicionados quando Routines migrar.
#[derive(Debug, Clone, PartialEq)]
pub struct Args {
    pub items: Vec<Value>,
    // named: IndexMap<EcoString, Value>,  // adiado — ADR futura
}

impl Args {
    pub fn positional(items: Vec<Value>) -> Self {
        Self { items }
    }

    pub fn len(&self) -> usize { self.items.len() }
    pub fn is_empty(&self) -> bool { self.items.is_empty() }
}
```

---

## Tarefa 3 — Value::Func e Value::Args

```rust
// Em value.rs — adicionar:
Func(crate::entities::func::Func),
// Args não precisa de ser Value neste passo — apenas Func é necessário
// para eval(). Args como Value é futuro (quando Args for passado como value).

// type_name:
Self::Func(_) => "function",
```

---

## Tarefa 4 — Criar closure e chamar função em eval()

### Assinatura de eval_expr — passar &Source

Para que `apply_closure` possa re-avaliar o body, `eval_expr` precisa
de acesso à `Source`. Solução para este passo: adicionar `source: &Source`
como parâmetro. Se a assinatura ficar longa demais, refactorizar para
`EvalContext` no Passo 17 — não antecipar agora.

```rust
fn eval_expr(
    expr: Expr<'_>,
    scopes: &mut Scopes<'_>,
    world: Tracked<dyn TrackedWorld + '_>,
    source: &Source,  // novo parâmetro
) -> SourceResult<Value> {
    // ... actualizar todas as chamadas recursivas para passar source
}
```

### Criar closure

```rust
Expr::Closure(closure_expr) => {
    // Captura eager — snapshot do scope actual
    let mut captured = IndexMap::with_hasher(FxBuildHasher::default());
    for (name, value) in scopes.iter_all() {
        captured.insert(name.to_string(), value.clone());
    }

    // Extrair parâmetros — confirmar API de ast::Param no diagnóstico
    let params = closure_expr.params()
        .children()
        .filter_map(|param| {
            let name = param.name()?.as_str().to_string();
            let default = param.default()
                .map(|d| eval_expr(d, scopes, world, source))
                .transpose()?;
            Some(ClosureParam { name, default })
        })
        .collect();

    // Body: SyntaxNode clone O(1) via Arc
    let body = closure_expr.body().to_untyped().clone();

    Ok(Value::Func(Func::closure(ClosureRepr { params, body, captured })))
}
```

### Chamar função

```rust
Expr::FuncCall(call) => {
    let callee = eval_expr(call.callee(), scopes, world, source)?;

    // Avaliar args antes de verificar o callee (order of evaluation)
    let items: Vec<Value> = call.args()
        .items()
        .filter_map(|arg| match arg {
            ast::Arg::Pos(expr) => Some(eval_expr(expr, scopes, world, source)),
            _ => None,  // named args → fronteira deliberada (adiado)
        })
        .collect::<Result<_, _>>()?;
    let args = Args::positional(items);

    match callee {
        Value::Func(func) => apply_closure_func(func, args, world, source),
        other => Err(vec![SourceDiagnostic::error(
            call.callee().span(),
            format!("não é possível chamar {}", other.type_name()),
        )]),
    }
}
```

### Aplicar closure

```rust
fn apply_closure_func(
    func: Func,
    args: Args,
    world: Tracked<dyn TrackedWorld + '_>,
    source: &Source,
) -> SourceResult<Value> {
    let FuncRepr::Closure(closure) = func.repr();
    apply_closure(closure, args, world, source)
}

fn apply_closure(
    closure: &ClosureRepr,
    args: Args,
    world: Tracked<dyn TrackedWorld + '_>,
    source: &Source,
) -> SourceResult<Value> {
    // Criar scope da chamada com variáveis capturadas
    let mut call_scope = Scope::new();
    for (name, value) in &closure.captured {
        call_scope.define(name.clone(), value.clone());
    }

    // Bind parâmetros posicionais
    for (param, value) in closure.params.iter().zip(args.items.iter()) {
        call_scope.define(param.name.clone(), value.clone());
    }
    // Parâmetros sem argumento → default ou None
    for param in closure.params.iter().skip(args.items.len()) {
        let val = param.default.clone().unwrap_or(Value::None);
        call_scope.define(param.name.clone(), val);
    }

    // Scopes da chamada: scope capturado como base, call_scope no topo
    // O scope do chamador NÃO é visível — captura por valor garante isolamento
    let global = Scope::new();
    let mut call_scopes = Scopes::new(&global);
    call_scopes.push_scope(call_scope);  // adicionar push_scope a Scopes se necessário

    // Avaliar o body com o scope da chamada
    if let Some(body_expr) = Expr::from_untyped(&closure.body) {
        eval_expr(body_expr, &mut call_scopes, world, source)
    } else {
        Ok(Value::None)
    }
}
```

### Adicionar `Scopes::push_scope` se não existir

```rust
// Em 01_core/src/rules/scopes.rs
impl<'a> Scopes<'a> {
    /// Empurra um scope pre-populado para a pilha.
    /// Usado por apply_closure para criar o ambiente de chamada.
    pub fn push_scope(&mut self, scope: Scope) {
        self.scopes.push(scope);
    }

    /// Itera sobre todos os bindings visíveis (para eager capture).
    pub fn iter_all(&self) -> impl Iterator<Item = (&str, &Value)> {
        self.scopes.iter().rev()
            .flat_map(|s| s.iter())
            .chain(self.top.iter())
    }
}
```

---

## Tarefa 5 — Testes (incluindo o Teste de Ouro)

### Testes básicos

```rust
#[test]
fn closure_cria_value_func() {
    let world = MockWorld::new("#let f = (x) => x + 1");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert!(matches!(m.scope().get("f"), Some(Value::Func(_))));
}

#[test]
fn funcall_soma_dois_args() {
    let world = MockWorld::new(
        "#let add = (x, y) => x + y\n#let r = add(1, 2)"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"), Some(&Value::Int(3)));
}

#[test]
fn funcall_arg_errado_retorna_err() {
    let world = MockWorld::new("#let r = 42(1)");
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_err());
}

#[test]
fn closure_default_param() {
    let world = MockWorld::new(
        "#let greet = (prefix: \"Hi\") => prefix\n#let r = greet()"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"), Some(&Value::Str("Hi".into())));
}
```

### Teste de Ouro — Eager Capture e Shadowing

```rust
/// Teste de Ouro: valida que eager capture é determinista e isolada.
///
/// Uma closure captura x = 1 no momento da definição.
/// Após a closure ser definida, x é redefinido como 2 no scope pai.
/// A closure deve continuar a retornar 1 (valor capturado),
/// não 2 (valor actual do scope pai).
///
/// Se este teste passar: eager capture está correcta e isolada.
/// Se falhar: a captura está a referenciar o scope em vez de copiar.
#[test]
fn eager_capture_isolada_do_scope_pai() {
    let world = MockWorld::new(
        "#let x = 1\n\
         #let get_x = () => x\n\
         #let x = 2\n\
         #let r = get_x()"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // Eager capture: get_x capturou x = 1 na definição
    // x = 2 é uma nova binding no scope, não muta a closure
    assert_eq!(m.scope().get("r"), Some(&Value::Int(1)),
        "eager capture deve isolar a closure do shadowing posterior");
}

/// Variante: shadowing dentro do corpo da closure não afecta o scope pai.
#[test]
fn closure_scope_nao_vaza_para_chamador() {
    let world = MockWorld::new(
        "#let f = () => { let local = 99; local }\n\
         #let r = f()"
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // r = 99 (valor retornado pela closure)
    assert_eq!(m.scope().get("r"), Some(&Value::Int(99)));
    // 'local' não deve estar visível no scope exterior
    assert!(m.scope().get("local").is_none(),
        "variáveis locais da closure não devem vazar para o chamador");
}
```

### Teste de ciclo Arc (segurança de memória)

```rust
/// Verifica que uma closure que se refere a si própria via nome
/// não impede a libertação de memória.
/// Uma closure recursiva captura a sua própria referência Func —
/// o Arc deve ser libertado quando o module sai do scope.
#[test]
fn closure_recursiva_nao_vaza_memoria() {
    // Esta closure chama-se a si própria — cria ciclo Arc potencial
    // O teste verifica que o programa não entra em loop infinito
    // (recursão sem base case → limite de profundidade ou erro)
    let world = MockWorld::new(
        "#let fact = (n) => if n <= 0 { 1 } else { n }\n\
         #let r = fact(5)"
        // Nota: sem chamada recursiva real neste teste —
        // apenas verifica que fact(5) funciona sem ciclo
    );
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("r"), Some(&Value::Int(5)));
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
- `Value::Func(...)` existe no enum ✓
- `add(1, 2)` retorna `Value::Int(3)` ✓
- **Teste de Ouro** `eager_capture_isolada_do_scope_pai` passa ✓
- `closure_scope_nao_vaza_para_chamador` passa ✓
- Chamar não-função → `Err(...)` ✓
- `Func::PartialEq` via `Arc::ptr_eq` ✓
- Zero violations ✓
- Testes não regridem (243 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- Eager vs lazy capture no original — confirmado?
- Estrutura real de `Func` (enum com vtable, ou struct com Arc?)
- Se `ast::Args` forçou tratamento de named args ou foi simples

**Da implementação:**
- Se `eval_expr` recebeu `&Source` como parâmetro ou foi necessário outro approach
- Se `Scopes::push_scope` e `iter_all` foram adicionados
- Se ciclo Arc em closures recursivas foi um problema observável

**Número total de testes e zero violations confirmado.**

**Go/No-Go para o Passo 17:**
- **GO — recursão**: Teste de Ouro passa; Passo 17 adiciona recursão
  real (closure chamando-se a si própria com base case) e funções
  nativas mínimas (`type()`, `len()`, `range()`)
- **GO — Content mínimo**: se ausência de Content impede validação
  real; Passo 17 migra `Content` com apenas `Text` para validar
  o pipeline completo parse→eval→layout
- **NO-GO — FuncCall API**: `ast::Args`/`ast::Param` diferem e
  closures não compilam; Passo 17 começa com diagnóstico AST mais
  profundo
