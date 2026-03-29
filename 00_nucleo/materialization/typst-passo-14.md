# Passo 14 — Integração comemo em testes e operações em eval()

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/eval.rs` — travessia parcial do Passo 13
- `lab/typst-original/crates/typst-eval/src/` — semântica de referência
- `lab/typst-original/crates/typst-library/src/foundations/ops.rs` (ou equivalente)

Pré-condição: `cargo test` — 225 testes (203 L1 + 22 L3, 1 ignorado), zero violations.

Duas tarefas independentes — ordem flexível conforme o diagnóstico:
1. Resolver integração `comemo::Tracked<>` nos testes de eval()
2. Implementar operações aritméticas e de comparação com semântica Typst correcta

**Go do Passo 15 é condicional**: só avançar se `1 + 1` resultar em
`Value::Int(2)` através do esqueleto de `eval()`. Operações sem paridade
confirmada não devem receber tipos complexos (Array, Dict).

---

## Tarefa 1 — Diagnóstico da integração comemo

**Parar aqui. Reportar output antes de continuar.**

```bash
# Como o original testa eval() — padrão de referência
find lab/typst-original/crates/typst-eval/tests -name "*.rs" 2>/dev/null | head -10
find lab/typst-original/crates/typst-eval/src -name "*.rs" \
  | xargs grep -l "#\[test\]" 2>/dev/null | head -5

# Mock de World nos testes do original
grep -rn "impl World\|TestWorld\|MockWorld" \
  lab/typst-original/crates/typst-eval/ \
  lab/typst-original/crates/typst/tests/ 2>/dev/null | head -15

# API real de comemo — track() como trait ou função livre?
find ~/.cargo/registry/src -path "*/comemo-*/src/lib.rs" 2>/dev/null \
  | head -1 | xargs grep -n "^pub fn track\|^pub trait Track\|fn track" 2>/dev/null | head -20

# O que #[comemo::track] gera para um struct stub como Sink(())
# (verificar se TrackedMut requer métodos específicos)
find ~/.cargo/registry/src -path "*/comemo-*/src/*.rs" 2>/dev/null \
  | xargs grep -n "TrackedMut\|track_mut" 2>/dev/null | head -20

# Versão exacta de comemo em uso
grep "^comemo" Cargo.lock | head -3

# Semântica de divisão no Typst — 5/2 = 2 ou 2.5?
grep -rn "Int.*Div\|Div.*Int\|division\|int.*div" \
  lab/typst-original/crates/typst-eval/src/ \
  lab/typst-original/crates/typst-library/src/foundations/ops.rs 2>/dev/null \
  | head -20

# Semântica de NaN e Inf — como o Typst os trata
grep -rn "NaN\|is_nan\|is_infinite\|Inf\b" \
  lab/typst-original/crates/typst-library/src/foundations/ops.rs 2>/dev/null \
  | head -20

# Confirmar: Int == Float é false?
grep -rn "Int.*Float\|Float.*Int\|type.*mismatch\|cannot.*compare" \
  lab/typst-original/crates/typst-library/src/foundations/ops.rs 2>/dev/null \
  | head -15
```

### Questões críticas a responder

1. **API de comemo**: `comemo::track(&w)` ou `w.track()` via trait?
2. **`TrackedMut<Sink(())>`**: o stub vazio satisfaz os requisitos do wrapper `TrackedMut`?
3. **`5 / 2` em Typst**: resulta em `2` (inteiro) ou `2.5` (float)?
4. **NaN e Infinito**: o Typst converte para erro, mantém IEEE 754, ou tem comportamento próprio?
5. **`Int == Float`**: confirmação de que `1 == 1.0` é `false` no Typst?

---

## Tarefa 2 — Resolver integração comemo

### Estratégia preferida: Cenário D (wrapper de teste)

Se `Sink(())` ou `Route(())` com `#[comemo::track]` vazio causarem
problemas de compilação com `TrackedMut`, o Cenário D é a saída
correcta — não um compromisso, mas a arquitectura certa:

- Código de produção permanece puro: `eval()` tem a assinatura correcta
- Suporte de teste absorve a fricção de comemo
- `eval_for_test` **não é visível fora de ficheiros de teste** —
  `#[cfg(test)]` garante que não contamina a build de produção

```rust
// Em 01_core/src/rules/eval.rs — APENAS em bloco #[cfg(test)]
#[cfg(test)]
pub(crate) fn eval_for_test(
    world: &dyn crate::contracts::world::World,
    source: &Source,
) -> SourceResult<Module> {
    let routines = Routines(());
    let traced   = Traced(());
    let mut sink = Sink(());
    let route    = Route(());

    // Confirmar API real de comemo com o diagnóstico antes de compilar
    // Opção A: trait Track
    // use comemo::Track;
    // eval(&routines, world.track(), ...)

    // Opção B: função livre
    // eval(&routines, comemo::track(world as &dyn TrackedWorld), ...)

    // Opção C: evict + track
    // comemo::evict(0);
    // eval(&routines, comemo::track(world as &dyn TrackedWorld), ...)

    todo!("preencher com API real após diagnóstico")
}
```

Após confirmação da API com o diagnóstico, substituir o `todo!()` pela
chamada correcta. Se a assinatura exigir `&dyn TrackedWorld` em vez de
`&dyn World`, adicionar um cast ou ajustar o parâmetro.

### Desbloquear o teste ignorado

```rust
#[test]
fn eval_let_int_via_world() {
    let world = MockWorld::new("#let x = 42");
    let source = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &source)
        .expect("eval não deve falhar em input válido");
    assert_eq!(module.scope().get("x"), Some(&Value::Int(42)));
}

#[test]
fn eval_multiplos_bindings_via_world() {
    let world = MockWorld::new("#let a = 1\n#let b = true\n#let c = \"x\"");
    let source = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &source).unwrap();
    assert_eq!(module.scope().get("a"), Some(&Value::Int(1)));
    assert_eq!(module.scope().get("b"), Some(&Value::Bool(true)));
    assert_eq!(module.scope().get("c"), Some(&Value::Str("x".into())));
}

#[test]
fn eval_texto_puro_scope_vazio() {
    let world = MockWorld::new("Apenas texto Typst.");
    let source = world.source(world.main()).unwrap();
    let module = eval_for_test(&world, &source).unwrap();
    assert!(module.scope().is_empty());
}
```

---

## Tarefa 3 — Operações com semântica Typst correcta

**Diagnóstico obrigatório antes de implementar**: confirmar com o
output da Tarefa 1 a semântica de divisão, NaN, e comparação de tipos.
Não assumir semântica de Rust — o Typst tem escolhas próprias.

### Semântica a confirmar

**Divisão inteira**: o Typst provavelmente faz `5 / 2 = 2.5` (float),
não `2` (truncamento). Se confirmado, implementar assim:

```rust
// Int / Int → Float no Typst (não truncamento como em Rust)
(Div, Value::Int(a), Value::Int(b)) if b != 0 => Ok(Value::Float(a as f64 / b as f64)),
(Div, Value::Int(_), Value::Int(0))            => Err("divisão por zero".into()),
```

Se o original truncar (menos provável): `Ok(Value::Int(a / b))`.
**Confirmar com o diagnóstico antes de escolher.**

**NaN e Infinito em Float**: o Typst provavelmente converte em erro
em vez de propagar `f64::NAN` ou `f64::INFINITY`. Se confirmado:

```rust
fn guard_float(f: f64, span_msg: &str) -> Result<Value, String> {
    if f.is_nan() {
        Err(format!("{span_msg}: resultado não é um número (NaN)"))
    } else if f.is_infinite() {
        Err(format!("{span_msg}: resultado é infinito"))
    } else {
        Ok(Value::Float(f))
    }
}

// Usar em operações Float:
(Add, Value::Float(a), Value::Float(b)) => guard_float(a + b, "adição"),
(Div, Value::Float(a), Value::Float(b)) => guard_float(a / b, "divisão"),
// etc.
```

Se o original propagar NaN/Inf silenciosamente (menos provável):
remover `guard_float` e usar `Ok(Value::Float(a op b))` directamente.

**`Int == Float` é `false`**: o match de `Eq` deve retornar `false`
para tipos diferentes, não tentar coerção:

```rust
// Eq compara value e tipo — tipos diferentes são sempre diferentes
(Eq,  a, b) => Ok(Value::Bool(a == b)),
// Como Value::Int(1) != Value::Float(1.0) por definição de PartialEq,
// isto é correcto automaticamente se PartialEq não fizer coerção.
// Verificar que o derive(PartialEq) em Value não faz coerção — não faz.
```

### Implementação em eval.rs

```rust
// Adicionar ao match em eval_expr:

Expr::Binary(binary) => {
    let lhs = eval_expr(binary.lhs(), scopes, world)?;
    let rhs = eval_expr(binary.rhs(), scopes, world)?;
    eval_binary_op(binary.op(), lhs, rhs)
        .map_err(|msg| vec![SourceDiagnostic::error(binary.span(), msg)])
}

Expr::Unary(unary) => {
    let operand = eval_expr(unary.expr(), scopes, world)?;
    eval_unary_op(unary.op(), operand)
        .map_err(|msg| vec![SourceDiagnostic::error(unary.span(), msg)])
}
```

```rust
// Funções puras separadas — testáveis sem world, sem mocks

/// Avalia uma operação binária com semântica Typst.
/// Função pura: sem I/O, sem estado, sem efeitos.
pub(crate) fn eval_binary_op(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    use BinOp::*;
    match (op, lhs, rhs) {
        // ── Adição ──────────────────────────────────────────────────────
        (Add, Value::Int(a),   Value::Int(b))   => Ok(Value::Int(a.saturating_add(b))),
        (Add, Value::Float(a), Value::Float(b)) => guard_float(a + b, "add"),
        (Add, Value::Float(a), Value::Int(b))   => guard_float(a + b as f64, "add"),
        (Add, Value::Int(a),   Value::Float(b)) => guard_float(a as f64 + b, "add"),
        (Add, Value::Str(a),   Value::Str(b))   => Ok(Value::Str(a + b.as_str())),

        // ── Subtracção ──────────────────────────────────────────────────
        (Sub, Value::Int(a),   Value::Int(b))   => Ok(Value::Int(a.saturating_sub(b))),
        (Sub, Value::Float(a), Value::Float(b)) => guard_float(a - b, "sub"),
        (Sub, Value::Float(a), Value::Int(b))   => guard_float(a - b as f64, "sub"),
        (Sub, Value::Int(a),   Value::Float(b)) => guard_float(a as f64 - b, "sub"),

        // ── Multiplicação ───────────────────────────────────────────────
        (Mul, Value::Int(a),   Value::Int(b))   => Ok(Value::Int(a.saturating_mul(b))),
        (Mul, Value::Float(a), Value::Float(b)) => guard_float(a * b, "mul"),
        (Mul, Value::Float(a), Value::Int(b))   => guard_float(a * b as f64, "mul"),
        (Mul, Value::Int(a),   Value::Float(b)) => guard_float(a as f64 * b, "mul"),

        // ── Divisão — confirmar semântica Int/Int com diagnóstico ───────
        // Expectativa: Int/Int → Float (não truncamento)
        (Div, Value::Int(_),   Value::Int(0))    => Err("divisão por zero".into()),
        (Div, Value::Int(a),   Value::Int(b))    => guard_float(a as f64 / b as f64, "div"),
        (Div, Value::Float(_), Value::Float(b)) if b == 0.0 => Err("divisão por zero".into()),
        (Div, Value::Float(a), Value::Float(b)) => guard_float(a / b, "div"),
        (Div, Value::Float(a), Value::Int(b))   => guard_float(a / b as f64, "div"),
        (Div, Value::Int(a),   Value::Float(b)) => guard_float(a as f64 / b, "div"),

        // ── Comparações ─────────────────────────────────────────────────
        // Eq/Neq: tipos diferentes → sempre false/true (sem coerção)
        (Eq,  a, b) => Ok(Value::Bool(a == b)),
        (Neq, a, b) => Ok(Value::Bool(a != b)),
        // Ordenação só para tipos compatíveis
        (Lt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a < b)),
        (Lt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
        (Leq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a <= b)),
        (Leq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
        (Gt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a > b)),
        (Gt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
        (Geq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a >= b)),
        (Geq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),

        // ── Lógica booleana ─────────────────────────────────────────────
        (And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
        (Or,  Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),

        // ── Fronteira — tipos não migrados ou combinações inválidas ─────
        (op, lhs, rhs) => Err(format!(
            "cannot apply {:?} to {} and {}",
            op, lhs.type_name(), rhs.type_name()
        )),
    }
}

/// Avalia uma operação unária com semântica Typst.
pub(crate) fn eval_unary_op(op: UnOp, operand: Value) -> Result<Value, String> {
    use UnOp::*;
    match (op, operand) {
        (Neg, Value::Int(i))   => Ok(Value::Int(i.saturating_neg())),
        (Neg, Value::Float(f)) => guard_float(-f, "neg"),
        (Not, Value::Bool(b))  => Ok(Value::Bool(!b)),
        (Pos, Value::Int(i))   => Ok(Value::Int(i)),
        (Pos, Value::Float(f)) => Ok(Value::Float(f)),
        (op, operand) => Err(format!(
            "cannot apply {:?} to {}",
            op, operand.type_name()
        )),
    }
}

/// Guarda contra NaN e Inf em resultados Float.
/// Se o Typst propagar NaN/Inf silenciosamente (confirmar com diagnóstico),
/// substituir por `Ok(Value::Float(f))` directamente.
fn guard_float(f: f64, op: &str) -> Result<Value, String> {
    if f.is_nan() {
        Err(format!("{op}: resultado não é um número"))
    } else if f.is_infinite() {
        Err(format!("{op}: resultado é infinito"))
    } else {
        Ok(Value::Float(f))
    }
}
```

**Nota sobre `saturating_add/sub/mul/neg`**: o Typst provavelmente
não usa saturação silenciosa — o original pode retornar erro em
overflow ou simplesmente wrapping. Confirmar com o diagnóstico e
ajustar se necessário.

### Testes de paridade obrigatórios

Estes testes são o critério de Go/No-Go para o Passo 15.
**Se falharem, não avançar.**

```rust
// Testes de eval_binary_op — puros, sem world, sem comemo

#[test]
fn paridade_add_int() {
    assert_eq!(eval_binary_op(BinOp::Add, Value::Int(1), Value::Int(2)),
               Ok(Value::Int(3)));
}

#[test]
fn paridade_add_float() {
    let r = eval_binary_op(BinOp::Add, Value::Float(1.5), Value::Float(2.5));
    assert_eq!(r, Ok(Value::Float(4.0)));
}

#[test]
fn paridade_add_str() {
    assert_eq!(
        eval_binary_op(BinOp::Add, Value::Str("hello ".into()), Value::Str("world".into())),
        Ok(Value::Str("hello world".into()))
    );
}

#[test]
fn paridade_div_int_int() {
    // Semântica Typst: 5 / 2 = 2.5 (float), não 2 (truncamento)
    // SE diagnóstico confirmar truncamento, mudar para Ok(Value::Int(2))
    assert_eq!(eval_binary_op(BinOp::Div, Value::Int(5), Value::Int(2)),
               Ok(Value::Float(2.5)));
}

#[test]
fn paridade_div_por_zero() {
    assert!(eval_binary_op(BinOp::Div, Value::Int(1), Value::Int(0)).is_err());
    assert!(eval_binary_op(BinOp::Div, Value::Float(1.0), Value::Float(0.0)).is_err());
}

#[test]
fn paridade_eq_int_float_sao_distintos() {
    // Int(1) != Float(1.0) — tipos diferentes, sem coerção
    assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Float(1.0)),
               Ok(Value::Bool(false)));
}

#[test]
fn paridade_eq_int_int() {
    assert_eq!(eval_binary_op(BinOp::Eq, Value::Int(1), Value::Int(1)),
               Ok(Value::Bool(true)));
}

#[test]
fn paridade_not() {
    assert_eq!(eval_unary_op(UnOp::Not, Value::Bool(true)),  Ok(Value::Bool(false)));
    assert_eq!(eval_unary_op(UnOp::Not, Value::Bool(false)), Ok(Value::Bool(true)));
}

#[test]
fn paridade_neg_int() {
    assert_eq!(eval_unary_op(UnOp::Neg, Value::Int(5)),  Ok(Value::Int(-5)));
    assert_eq!(eval_unary_op(UnOp::Neg, Value::Int(-3)), Ok(Value::Int(3)));
}

#[test]
fn paridade_nan_e_erro() {
    // 0.0 / 0.0 em f64 dá NaN — deve ser Err, não Ok(Float(NaN))
    // SE diagnóstico confirmar que Typst propaga NaN, remover este teste
    let r = eval_binary_op(BinOp::Div, Value::Float(0.0), Value::Float(0.0));
    // Ou Err (NaN) ou Err (divisão por zero) — ambos aceitáveis, não Ok(NaN)
    assert!(r.is_err() || matches!(r, Ok(Value::Float(f)) if !f.is_nan()));
}

#[test]
fn paridade_tipo_invalido_retorna_err() {
    // None + Int → Err com mensagem, não panic
    let r = eval_binary_op(BinOp::Add, Value::None, Value::Int(1));
    assert!(r.is_err());
}
```

### Prompt L0

**Actualizar**: `00_nucleo/prompts/rules/eval.md`

Documentar:
- `eval_binary_op` e `eval_unary_op` como funções puras
- Semântica Typst de divisão (confirmada com diagnóstico)
- Tratamento de NaN/Inf (confirmado com diagnóstico)
- `Int == Float` é `false`
- Fronteira `_ => Err(...)` para tipos não migrados

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
- `eval_binary_op(Add, Int(1), Int(2))` → `Ok(Int(3))` ✓
- `eval_binary_op(Add, Str("a"), Str("b"))` → `Ok(Str("ab"))` ✓
- `eval_binary_op(Div, Int(5), Int(2))` → semântica confirmada com diagnóstico ✓
- `eval_binary_op(Eq, Int(1), Float(1.0))` → `Ok(Bool(false))` ✓
- Div por zero → `Err(...)` ✓
- NaN/Inf → `Err(...)` (ou comportamento confirmado com diagnóstico) ✓
- Tipo inválido → `Err(...)`, não panic ✓
- `eval_for_test` confinado a `#[cfg(test)]`, não visível em build de produção ✓
- Zero violations ✓
- Testes não regridem (225 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico comemo:**
- Cenário usado (A/B/C/D) e se `eval_for_test` foi necessário
- Se `TrackedMut<Sink(())>` compilou directamente ou exigiu ajustes
- Se o teste ignorado foi desbloqueado

**Da semântica Typst (confirmar com diagnóstico):**
- `5 / 2` → `2` (int, truncamento) ou `2.5` (float)?
- `0.0 / 0.0` → `Err` ou `Float(NaN)` propagado?
- `Int == Float` → `false` confirmado?
- Overflow em Int → saturação, wrapping, ou erro?

**Das operações:**
- Se os nomes de BinOp/UnOp diferiram do esperado — lista de ajustes
- Número total de testes e zero violations confirmado

**Go/No-Go para o Passo 15:**
- **GO — testes de paridade passam**: `1 + 1 = 2`, `"a" + "b" = "ab"`,
  `Int != Float`; Passo 15 adiciona control flow (`if`/`while`/`for`)
  e variantes fáceis de Value (Array, Dict, Module)
- **NO-GO — semântica errada**: algum teste de paridade falha;
  corrigir semântica antes de avançar para tipos complexos
- **NO-GO — operadores**: nomes de BinOp/UnOp diferem e a compilação
  falha; Passo 15 começa com diagnóstico AST das expressões binárias
