# Passo 15 — ADR-0025, control flow e variantes fáceis de Value

## Estado actual antes de começar

Ler antes de começar:
- `00_nucleo/adr/typst-adr-0025-int-eq-float.md` — **resolver antes de qualquer código**
- `01_core/src/rules/eval.rs` — estado actual com Binary/Unary
- `01_core/src/entities/value.rs` — subset actual (5 variantes)

Pré-condição: `cargo test` — todos os testes do Passo 14 passam, zero violations.

Três tarefas com dependência parcial:
1. **ADR-0025** — resolver desvio `Int == Float` (bloqueante para control flow)
2. **Control flow** — `if`/`else`, `while`, `for` (depende de ADR-0025)
3. **Variantes fáceis de Value** — Array, Dict, Module, Datetime (independente)

---

## Tarefa 1 — Resolver ADR-0025 (Int == Float)

### Diagnóstico obrigatório

```bash
# Confirmar se PartialEq no original é manual ou derived
grep -n "PartialEq\|fn eq\b\|impl.*Partial" \
  lab/typst-original/crates/typst-library/src/foundations/value.rs \
  | head -20

# Semântica exacta de Eq para Int/Float em ops.rs
grep -n -A 5 "Int.*Float\|Float.*Int" \
  lab/typst-original/crates/typst-library/src/foundations/ops.rs \
  | grep -i "eq\|==\|compare" | head -20

# A coerção estende-se a Lt/Leq/Gt/Geq ou apenas a Eq/Neq?
grep -n "Int.*Lt\|Float.*Lt\|Int.*Gt\|Float.*Gt\|ordering.*int.*float" \
  lab/typst-original/crates/typst-library/src/foundations/ops.rs \
  | head -15
```

### Implementação — Opção B

Manter `derive(PartialEq)` em `Value`. Adicionar coerção explícita
em `eval_binary_op` — **antes** dos wildcards `(Eq, a, b)`:

```rust
// Coerção numérica Int/Float em Eq/Neq — ADR-0025
// Razão: no Typst 1 == 1.0 é true; derive(PartialEq) retornaria false.
// derive(PartialEq) mantido para IndexMap, testes Rust, e estruturas de dados.
(Eq,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) == b)),
(Eq,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a == (b as f64))),
(Neq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) != b)),
(Neq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a != (b as f64))),
// Wildcards (tipos iguais ou incompatíveis):
(Eq,  a, b) => Ok(Value::Bool(a == b)),
(Neq, a, b) => Ok(Value::Bool(a != b)),
```

Se o diagnóstico confirmar que `<`/`<=`/`>`/`>=` também coercem
Int/Float, adicionar os casos equivalentes antes dos wildcards
de ordenação.

### Corrigir teste do Passo 14

```rust
// Passo 14 verificava comportamento Rust (false).
// Após ADR-0025, o comportamento correcto é Typst (true):
assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Float(1.0)),
           Ok(Value::Bool(true)));  // era false — corrigir
```

### Testes da dualidade de igualdade

**Estes dois testes devem passar simultaneamente — é a "Prova de
Invariância" da ADR-0025:**

```rust
#[test]
fn dualidade_eq_typst_coerce() {
    // No motor Typst: 1 == 1.0 → true (coerção)
    assert_eq!(
        eval_binary_op(BinOp::Eq, Value::Int(1), Value::Float(1.0)),
        Ok(Value::Bool(true))
    );
}

#[test]
fn dualidade_eq_rust_sem_coerce() {
    // Em Rust puro: Value::Int(1) != Value::Float(1.0) (derive(PartialEq))
    // Vital para depuração, IndexMap, e colecções — não deve mudar.
    assert_ne!(Value::Int(1), Value::Float(1.0));
}

#[test]
fn eq_tipos_radicalmente_distintos() {
    // Bool vs Int — sem coerção em nenhum sistema
    assert_eq!(
        eval_binary_op(BinOp::Eq, Value::Bool(true), Value::Int(1)),
        Ok(Value::Bool(false))
    );
}
```

---

## Tarefa 2 — Control flow

**Pré-condição**: ADR-0025 resolvida, teste `dualidade_eq_typst_coerce` passa.

### Diagnóstico obrigatório antes de qualquer código

```bash
# API de Conditional (if/else) na AST cristalina
grep -n "pub fn\|Conditional" \
  01_core/src/entities/ast/code.rs | grep -A5 -i "conditional\|if_body\|else" | head -20

# API de WhileLoop
grep -n "pub fn\|WhileLoop\|condition\|body" \
  01_core/src/entities/ast/code.rs | grep -A5 -i "while" | head -20

# API de ForLoop — CRÍTICO: confirmar nomes de pattern/iter/body
# e verificar se há trivia entre 'in' e o iterável
grep -n "pub fn\|ForLoop\|pattern\|iter\b\|body\|in\b" \
  01_core/src/entities/ast/code.rs | grep -A10 -i "forloop\|for_loop" | head -25

# Verificar também como o pattern de ForLoop é exposto
# (ident simples, destructuring, ou Pattern genérico?)
grep -n "ForPattern\|IterPattern\|for.*pattern" \
  01_core/src/entities/ast/code.rs \
  01_core/src/entities/ast/expr.rs | head -15
```

**Questão crítica sobre ForLoop**: a AST do Typst para `for x in items`
pode ter trivia (espaços, comentários) entre o `in` e o iterável.
Verificar se a AST cristalina expõe `iter()` directamente ou se
é necessário filtrar children para encontrar o nó do iterável.

### if/else

```rust
Expr::Conditional(cond) => {
    let condition = eval_expr(cond.condition(), scopes, world)?;
    match condition {
        Value::Bool(true) => eval_expr(cond.if_body(), scopes, world),
        Value::Bool(false) => match cond.else_body() {
            Some(else_body) => eval_expr(else_body, scopes, world),
            None            => Ok(Value::None),
        },
        other => Err(vec![SourceDiagnostic::error(
            cond.condition().span(),
            format!("condição if deve ser bool, encontrado {}", other.type_name()),
        )]),
    }
}
```

### while — com limite de segurança

```rust
Expr::WhileLoop(loop_expr) => {
    // Limite de segurança: impede loops infinitos em CI sem comemo+Route.
    // O original usa comemo para memoização incremental e Route para
    // detecção de ciclos — esses mecanismos serão ligados no Passo 16+.
    const MAX_ITER: usize = 10_000;
    let mut count = 0;

    loop {
        if count >= MAX_ITER {
            return Err(vec![SourceDiagnostic::error(
                loop_expr.span(),
                format!("loop excedeu {MAX_ITER} iterações (limite de segurança)"),
            )]);
        }

        let cond = eval_expr(loop_expr.condition(), scopes, world)?;
        match cond {
            Value::Bool(true) => {
                scopes.push();
                eval_expr(loop_expr.body(), scopes, world)?;
                scopes.pop();
                count += 1;
            }
            Value::Bool(false) => break,
            other => return Err(vec![SourceDiagnostic::error(
                loop_expr.condition().span(),
                format!("condição while deve ser bool, encontrado {}", other.type_name()),
            )]),
        }
    }
    Ok(Value::None)
}
```

### for — apenas Array neste passo

```rust
// Ajustar nomes de métodos conforme diagnóstico da AST
Expr::ForLoop(loop_expr) => {
    let iterable = eval_expr(loop_expr.iter(), scopes, world)?;
    match iterable {
        Value::Array(items) => {
            for item in items {
                scopes.push();
                // Confirmar API de pattern no diagnóstico antes de usar
                // Se pattern é Ident simples:
                let name = loop_expr.pattern().as_str();
                scopes.define(name, item);
                eval_expr(loop_expr.body(), scopes, world)?;
                scopes.pop();
            }
            Ok(Value::None)
        }
        other => Err(vec![SourceDiagnostic::error(
            loop_expr.iter().span(),
            format!("não é possível iterar sobre {}", other.type_name()),
        )]),
    }
}
```

### Testes de control flow

```rust
// ── if/else ────────────────────────────────────────────────────────────────

#[test]
fn if_true_branch() {
    let world = MockWorld::new("#let x = if true { 1 } else { 2 }");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("x"), Some(&Value::Int(1)));
}

#[test]
fn if_false_branch() {
    let world = MockWorld::new("#let x = if false { 1 } else { 2 }");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("x"), Some(&Value::Int(2)));
}

#[test]
fn if_sem_else_retorna_none() {
    let world = MockWorld::new("#let x = if false { 1 }");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("x"), Some(&Value::None));
}

/// Teste de Fogo — End-to-End ADR-0025.
/// Valida simultaneamente: travessia AST, Scope/Binding, e coerção Int==Float.
#[test]
fn prova_de_vida_adr_0025() {
    let world = MockWorld::new("#let x = if 1 == 1.0 { 42 } else { 0 }");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    // Se ADR-0025 estiver correcta: 1 == 1.0 é true → ramo then → x = 42
    assert_eq!(m.scope().get("x"), Some(&Value::Int(42)));
}

// ── while ─────────────────────────────────────────────────────────────────

#[test]
fn while_condicao_falsa_nao_executa() {
    let world = MockWorld::new("#while false { }");
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_ok());
}

/// Teste de segurança do limite de iterações.
/// Um loop que seria infinito deve retornar Err antes de travar o CI.
#[test]
fn while_loop_infinito_retorna_err() {
    // while true {} — condição sempre verdadeira, deve atingir o limite
    let world = MockWorld::new("#while true { }");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    // Deve ser Err com mensagem sobre limite de iterações
    assert!(result.is_err(), "loop infinito deve retornar Err, não bloquear");
    let err = result.unwrap_err();
    assert!(!err.is_empty());
    assert!(err[0].message.contains("iterações") || err[0].message.contains("limite"),
            "mensagem de erro deve mencionar limite: {:?}", err[0].message);
}

// ── for ───────────────────────────────────────────────────────────────────

#[test]
fn for_sobre_array_vazio_nao_executa() {
    // Ajustar sintaxe conforme o que o parser Typst aceita para array literal vazio
    // Pode ser () ou precisar de #let arr = (); #for x in arr {}
    let world = MockWorld::new("#let arr = ()\n#for x in arr { }");
    let src = world.source(world.main()).unwrap();
    assert!(eval_for_test(&world, &src).is_ok());
}
```

---

## Tarefa 3 — Variantes fáceis de Value

Independente das outras tarefas. Pode ser feita antes ou depois de
control flow.

### Array — Vec<Value> simples neste passo

```rust
// Em value.rs — adicionar variante:
Array(Vec<Value>),

// type_name:
Self::Array(_) => "array",

// Acesso:
pub fn cast_array(&self) -> Option<&[Value]> {
    match self { Self::Array(a) => Some(a), _ => None }
}

// From:
impl From<Vec<Value>> for Value {
    fn from(v: Vec<Value>) -> Self { Self::Array(v) }
}
```

**Nota de performance**: `Vec<Value>` é suficiente para este passo.
Em documentos reais com arrays grandes e muitas passagens de argumento,
um `Arc<Vec<Value>>` ou `EcoVec<Value>` pode ser necessário para
clone O(1). Registar em DEBT.md como work item futuro — não antecipar
agora, mantém a tração do passo.

```
// DEBT.md — adicionar:
// Value::Array performance: Vec<Value> tem clone O(n).
// Candidatos: Arc<Vec<Value>> ou EcoVec<Value> para clone O(1).
// Activar quando arrays grandes aparecerem no hot path de eval().
```

### Dict

```rust
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;
use ecow::EcoString;

// Variante:
Dict(IndexMap<EcoString, Value, FxBuildHasher>),

// type_name:
Self::Dict(_) => "dictionary",

// From:
impl From<IndexMap<EcoString, Value, FxBuildHasher>> for Value {
    fn from(v: IndexMap<EcoString, Value, FxBuildHasher>) -> Self { Self::Dict(v) }
}

// Acesso:
pub fn cast_dict(&self) -> Option<&IndexMap<EcoString, Value, FxBuildHasher>> {
    match self { Self::Dict(d) => Some(d), _ => None }
}
```

`IndexMap` preserva ordem de inserção — paridade com o comportamento
de Dict em Typst (documentos produzem dicts na ordem de escrita).

### Module em Value

```rust
Module(crate::entities::module::Module),

Self::Module(_) => "module",

impl From<crate::entities::module::Module> for Value {
    fn from(m: crate::entities::module::Module) -> Self { Self::Module(m) }
}
```

`Module` usa `Arc<ModuleInner>` internamente (Passo 11) — clone de
`Value::Module` é O(1) sem trabalho adicional.

### Datetime em Value

```rust
Datetime(crate::entities::world_types::Datetime),

Self::Datetime(_) => "datetime",

impl From<crate::entities::world_types::Datetime> for Value {
    fn from(d: crate::entities::world_types::Datetime) -> Self { Self::Datetime(d) }
}
```

### Actualizar comentários de variantes-fantasma

Remover dos comentários as variantes agora implementadas e actualizar
o contador: `// Variantes futuras (~21 restantes após Passo 15)`.

### Testes das novas variantes

```rust
#[test]
fn array_type_name_e_cast() {
    let v = Value::Array(vec![Value::Int(1), Value::Int(2)]);
    assert_eq!(v.type_name(), "array");
    assert_eq!(v.cast_array().unwrap().len(), 2);
}

#[test]
fn array_from_vec() {
    let v = Value::from(vec![Value::Bool(true)]);
    assert!(matches!(v, Value::Array(_)));
}

#[test]
fn array_clone_is_independent() {
    // Vec<Value> clone é O(n) — verificar que são independentes
    let v1 = Value::Array(vec![Value::Int(1)]);
    let mut v2 = v1.clone();
    if let Value::Array(ref mut a) = v2 {
        a.push(Value::Int(2));
    }
    // v1 não deve ter sido afectado
    assert_eq!(v1.cast_array().unwrap().len(), 1);
}

#[test]
fn dict_type_name() {
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    let d: IndexMap<_, _, FxBuildHasher> = IndexMap::default();
    assert_eq!(Value::Dict(d).type_name(), "dictionary");
}

#[test]
fn module_em_value_clone_barato() {
    use crate::entities::{module::Module, scope::Scope};
    let m = Module::new("test", Scope::new());
    let v1 = Value::from(m);
    let v2 = v1.clone();  // O(1) via Arc
    assert_eq!(v1.type_name(), v2.type_name());
}

#[test]
fn datetime_em_value() {
    let dt = crate::entities::world_types::Datetime::new_date(2026, 3, 27).unwrap();
    assert_eq!(Value::from(dt).type_name(), "datetime");
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
- `dualidade_eq_typst_coerce`: `eval_binary_op(Eq, Int(1), Float(1.0))` → `Ok(Bool(true))` ✓
- `dualidade_eq_rust_sem_coerce`: `Value::Int(1) != Value::Float(1.0)` em Rust ✓
- `prova_de_vida_adr_0025`: `if 1 == 1.0 { 42 } else { 0 }` → `Int(42)` ✓
- `while_loop_infinito_retorna_err`: `while true {}` → `Err(...)` sem bloquear ✓
- `Value::Array`, `Value::Dict`, `Value::Module`, `Value::Datetime` no enum ✓
- `array_clone_is_independent`: clone de Array é independente ✓
- Zero violations ✓
- Testes não regridem ✓

---

## Ao terminar, reportar

**Da ADR-0025:**
- Se coerção se aplica também a `<`/`<=`/`>`/`>=` (confirmar com diagnóstico)
- Se Opção B bastou ou foi necessário PartialEq manual

**Do control flow:**
- API real de `ForLoop` na AST — nomes de pattern/iter/body confirmados
- Se há trivia entre `in` e o iterável em ForLoop (e como foi tratada)
- Se `while true {}` retornou `Err` com mensagem correcta (confirmar no log)

**Das variantes Value:**
- Se `Array`, `Dict`, `Module`, `Datetime` foram adicionados com sucesso
- Se alguma variante revelou dep inesperada

**Número total de testes e zero violations confirmado.**

**Go/No-Go para o Passo 16:**
- **GO — FuncCall**: control flow funciona e `prova_de_vida_adr_0025` passa;
  Passo 16 começa a migrar closures simples (`(x) => x + 1`) para
  permitir chamadas de função básicas
- **GO — Content mínimo**: se avaliação de markup sem Content não é
  suficiente para verificação real; Passo 16 migra `Content` com
  apenas `Text` node para validar o pipeline completo
- **NO-GO — ForLoop API**: API de ForLoop difere e for não compila;
  Passo 16 começa com diagnóstico mais profundo da AST de iteração
