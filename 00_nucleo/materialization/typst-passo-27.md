# Passo 27 — Conclusão de DEBT-4: funções de conversão e módulo calc

## Estado actual antes de começar

Ler antes de começar:
- `01_core/src/rules/stdlib.rs` — `native_type`, `native_len`, `native_range`,
  `native_rgb`, `native_luma`
- `01_core/src/entities/value.rs` — variantes actuais incluindo `Length`, `Color`, etc.
- `01_core/src/rules/eval.rs` — `make_stdlib()`, `eval_binary_op`
- `DEBT.md` — entrada DEBT-4

Pré-condição: `cargo test` — 353 testes, zero violations.

**Contexto**: DEBT-4 registava ~20 variantes de `Value` ausentes e funções nativas
que retornavam `Value::None` em vez do valor correcto. Os Passos 25 e 26 pagaram
o grupo tipográfico (`Length`, `Ratio`, `Angle`, `Color`, `Auto`) e as funções
de cor (`rgb`, `luma`). Este passo fecha DEBT-4 com dois grupos:

1. **Funções de conversão de tipo** — `str()`, `int()`, `float()`, `bool()`
2. **Módulo `calc`** — `calc.abs()`, `calc.pow()`, `calc.sqrt()`, `calc.floor()`,
   `calc.ceil()`, `calc.round()`, `calc.min()`, `calc.max()`, `calc.clamp()`

Após este passo, DEBT-4 está encerrado. As variantes restantes comentadas
(`Symbol`, `Version`, `Bytes`, `Decimal`, `Duration`, `Gradient`, `Tiling`,
`Relative`, `Fraction`) ficam para quando os passos que as necessitam chegarem.

---

## Tarefa 1 — Diagnóstico das funções no original

```bash
# str(), int(), float(), bool() — onde vivem no original?
grep -rn "fn native_str\|fn str_func\|\"str\"\|fn native_int\|fn native_float" \
  lab/typst-original/crates/typst-library/src/foundations/ 2>/dev/null | head -20

# Semântica de str() — aceita todos os tipos ou só alguns?
grep -rA 20 "fn str\b\|\"str\".*NativeFunc\|fn str_func" \
  lab/typst-original/crates/typst-library/src/foundations/ 2>/dev/null | head -30

# Semântica de int() — aceita Float com truncamento ou erro?
grep -rA 20 "fn int\b\|\"int\".*NativeFunc" \
  lab/typst-original/crates/typst-library/src/foundations/ 2>/dev/null | head -30

# Módulo calc — onde está definido?
grep -rn "\"calc\"\|calc\.abs\|CalcFunc\|mod calc" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -20

# Funções de calc — lista completa
grep -rn "\"abs\"\|\"pow\"\|\"sqrt\"\|\"floor\"\|\"ceil\"\|\"round\"\|\"min\"\|\"max\"\|\"clamp\"" \
  lab/typst-original/crates/typst-library/src/foundations/ \
  lab/typst-original/crates/typst-library/src/math/ 2>/dev/null | head -30

# Como calc é exposto — é um Value::Dict, um módulo separado, ou namespace?
grep -rA 15 "calc.*scope\|scope.*calc\|\"calc\".*Module\|\"calc\".*Dict" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -25
```

**Parar. Reportar output antes de qualquer código.**

Questões críticas:
1. `str()` — aceita `Int`, `Float`, `Bool`, `Content`, ou só alguns?
   Qual o formato de `str(3.14)` — `"3.14"` ou `"3.140000..."`?
2. `int()` — `int(3.7)` trunca para `3` ou dá erro?
   `int("42")` parseia string para int?
3. `calc` no original — é um `Value::Module` com funções, um `Value::Dict`,
   ou outro mecanismo?
4. `calc.min()`/`calc.max()` — aceitam número variável de argumentos?

---

## Tarefa 2 — Funções de conversão de tipo

### Semântica a implementar (ajustar após diagnóstico)

```
str(none)    → "none"
str(true)    → "true"  / "false"
str(42)      → "42"
str(3.14)    → "3.14"  (formato compacto, não científico)
str("hello") → "hello" (identity)
str(length)  → representação textual (ex: "12pt")

int(42)      → 42
int(3.7)     → Err  (Float não converte silenciosamente — confirmar com diagnóstico)
int("42")    → 42   (parseia string decimal)
int("0xFF")  → 255  (parseia hex — confirmar se suportado)
int(true)    → 1 / int(false) → 0  (confirmar)

float(3)     → 3.0
float(3.14)  → 3.14
float("3.14")→ 3.14
float(true)  → Err  (confirmar)

bool(0)      → false  (confirmar se suportado)
bool(1)      → true   (confirmar se suportado)
```

### Implementação

```rust
// Em rules/stdlib.rs

pub fn native_str(args: &[Value]) -> SourceResult<Value> {
    match args {
        [v] => {
            let s = match v {
                Value::None        => "none".into(),
                Value::Bool(b)     => if *b { "true" } else { "false" }.into(),
                Value::Int(i)      => i.to_string(),
                Value::Float(f)    => format_float(*f),
                Value::Str(s)      => return Ok(Value::Str(s.clone())),
                Value::Length(l)   => format_length(l),
                Value::Ratio(r)    => format!("{}%", r.to_percent()),
                Value::Angle(a)    => format!("{}deg", a.to_deg()),
                Value::Color(_)    => return Err(err_msg("str() não suporta color")),
                Value::Auto        => "auto".into(),
                other => return Err(err_msg(
                    format!("str() não suporta {}", other.type_name())
                )),
            };
            Ok(Value::Str(s.into()))
        }
        _ => Err(err_msg(format!("str() requer 1 argumento, recebeu {}", args.len()))),
    }
}

/// Formata um f64 de forma compacta — sem trailing zeros desnecessários.
fn format_float(f: f64) -> String {
    // "3.14" em vez de "3.140000000000000" ou "3.14e0"
    let s = format!("{}", f);
    // Se não tem ponto decimal, adicionar ".0" para distinguir de int
    if s.contains('.') || s.contains('e') { s } else { format!("{s}.0") }
}

/// Formata um Length como string (ex: "12pt", "1.5em", "6pt + 1em")
fn format_length(l: &Length) -> String {
    match (l.abs.to_pt(), l.em) {
        (abs, 0.0) => format!("{abs}pt"),
        (0.0, em)  => format!("{em}em"),
        (abs, em)  => format!("{abs}pt + {em}em"),
    }
}

pub fn native_int(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(i)]    => Ok(Value::Int(*i)),
        [Value::Float(f)]  => {
            // Confirmar com diagnóstico: Typst trunca ou dá Err?
            // Implementação conservadora: Err para Float
            Err(err_msg(format!(
                "int() não converte float {f} — usar int(round(x)) ou int(floor(x))"
            )))
        }
        [Value::Str(s)] => {
            s.parse::<i64>()
                .map(Value::Int)
                .map_err(|_| err_msg(format!("int() não consegue parsear {:?}", s.as_str())))
        }
        [Value::Bool(b)]   => Ok(Value::Int(if *b { 1 } else { 0 })),
        [other] => Err(err_msg(format!(
            "int() não suporta {}", other.type_name()
        ))),
        _ => Err(err_msg(format!("int() requer 1 argumento, recebeu {}", args.len()))),
    }
}

pub fn native_float(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Float(f)] => Ok(Value::Float(*f)),
        [Value::Int(i)]   => Ok(Value::Float(*i as f64)),
        [Value::Str(s)]   => s.parse::<f64>()
            .map(Value::Float)
            .map_err(|_| err_msg(format!("float() não consegue parsear {:?}", s.as_str()))),
        [other] => Err(err_msg(format!(
            "float() não suporta {}", other.type_name()
        ))),
        _ => Err(err_msg(format!("float() requer 1 argumento, recebeu {}", args.len()))),
    }
}
```

---

## Tarefa 3 — Módulo `calc`

No Typst, `calc` é acedido como `calc.abs(x)`, `calc.pow(b, e)`, etc.
Isso implica que `calc` é um valor no scope global com funções acessíveis
por field access.

### Representação de `calc` no cristalino

A forma mais directa: `calc` é um `Value::Dict` com entradas cujos
valores são `Value::Func`. Field access (`calc.abs`) resolve para
a função correspondente via `eval_field_access`.

```rust
// Em rules/stdlib.rs

pub fn make_calc_module() -> Value {
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    use ecow::EcoString;

    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();

    dict.insert("abs".into(),   Value::Func(Func::native("calc.abs",   calc_abs)));
    dict.insert("pow".into(),   Value::Func(Func::native("calc.pow",   calc_pow)));
    dict.insert("sqrt".into(),  Value::Func(Func::native("calc.sqrt",  calc_sqrt)));
    dict.insert("floor".into(), Value::Func(Func::native("calc.floor", calc_floor)));
    dict.insert("ceil".into(),  Value::Func(Func::native("calc.ceil",  calc_ceil)));
    dict.insert("round".into(), Value::Func(Func::native("calc.round", calc_round)));
    dict.insert("min".into(),   Value::Func(Func::native("calc.min",   calc_min)));
    dict.insert("max".into(),   Value::Func(Func::native("calc.max",   calc_max)));
    dict.insert("clamp".into(), Value::Func(Func::native("calc.clamp", calc_clamp)));

    Value::Dict(dict)
}
```

### Funções de calc

```rust
fn calc_abs(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(i)]   => Ok(Value::Int(i.saturating_abs())),
        [Value::Float(f)] => Ok(Value::Float(f.abs())),
        [other] => Err(err_msg(format!(
            "calc.abs() requer Int ou Float, recebeu {}", other.type_name()
        ))),
        _ => Err(err_msg(format!("calc.abs() requer 1 argumento, recebeu {}", args.len()))),
    }
}

fn calc_pow(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(base), Value::Int(exp)] => {
            if *exp < 0 {
                return Err(err_msg("calc.pow() expoente negativo requer Float"));
            }
            Ok(Value::Int(base.saturating_pow(*exp as u32)))
        }
        [base, exp] => {
            let b = coerce_to_f64(base, "calc.pow() base")?;
            let e = coerce_to_f64(exp,  "calc.pow() expoente")?;
            guard_float(b.powf(e))
        }
        _ => Err(err_msg(format!("calc.pow() requer 2 argumentos, recebeu {}", args.len()))),
    }
}

fn calc_sqrt(args: &[Value]) -> SourceResult<Value> {
    match args {
        [v] => {
            let f = coerce_to_f64(v, "calc.sqrt()")?;
            if f < 0.0 {
                return Err(err_msg("calc.sqrt() argumento negativo"));
            }
            guard_float(f.sqrt())
        }
        _ => Err(err_msg(format!("calc.sqrt() requer 1 argumento, recebeu {}", args.len()))),
    }
}

fn calc_floor(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.floor() as i64)),
        [other] => Err(err_msg(format!(
            "calc.floor() requer Int ou Float, recebeu {}", other.type_name()
        ))),
        _ => Err(err_msg(format!("calc.floor() requer 1 argumento, recebeu {}", args.len()))),
    }
}

fn calc_ceil(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.ceil() as i64)),
        [other] => Err(err_msg(format!(
            "calc.ceil() requer Int ou Float, recebeu {}", other.type_name()
        ))),
        _ => Err(err_msg(format!("calc.ceil() requer 1 argumento, recebeu {}", args.len()))),
    }
}

fn calc_round(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(i)]   => Ok(Value::Int(*i)),
        [Value::Float(f)] => Ok(Value::Int(f.round() as i64)),
        [other] => Err(err_msg(format!(
            "calc.round() requer Int ou Float, recebeu {}", other.type_name()
        ))),
        _ => Err(err_msg(format!("calc.round() requer 1 argumento, recebeu {}", args.len()))),
    }
}

fn calc_min(args: &[Value]) -> SourceResult<Value> {
    if args.is_empty() {
        return Err(err_msg("calc.min() requer pelo menos 1 argumento"));
    }
    // Redução: mínimo dos argumentos, todos do mesmo tipo numérico
    let mut result = args[0].clone();
    for v in &args[1..] {
        result = match (&result, v) {
            (Value::Int(a),   Value::Int(b))   => Value::Int(*a.min(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.min(*b)),
            (Value::Int(a),   Value::Float(b)) => Value::Float((*a as f64).min(*b)),
            (Value::Float(a), Value::Int(b))   => Value::Float(a.min(*b as f64)),
            (_, other) => return Err(err_msg(format!(
                "calc.min() tipos incompatíveis: {} e {}", result.type_name(), other.type_name()
            ))),
        };
    }
    Ok(result)
}

fn calc_max(args: &[Value]) -> SourceResult<Value> {
    if args.is_empty() {
        return Err(err_msg("calc.max() requer pelo menos 1 argumento"));
    }
    let mut result = args[0].clone();
    for v in &args[1..] {
        result = match (&result, v) {
            (Value::Int(a),   Value::Int(b))   => Value::Int(*a.max(b)),
            (Value::Float(a), Value::Float(b)) => Value::Float(a.max(*b)),
            (Value::Int(a),   Value::Float(b)) => Value::Float((*a as f64).max(*b)),
            (Value::Float(a), Value::Int(b))   => Value::Float(a.max(*b as f64)),
            (_, other) => return Err(err_msg(format!(
                "calc.max() tipos incompatíveis: {} e {}", result.type_name(), other.type_name()
            ))),
        };
    }
    Ok(result)
}

fn calc_clamp(args: &[Value]) -> SourceResult<Value> {
    // clamp(value, min, max)
    match args {
        [Value::Int(v), Value::Int(lo), Value::Int(hi)] =>
            Ok(Value::Int((*v).clamp(*lo, *hi))),
        [v, lo, hi] => {
            let vf  = coerce_to_f64(v,  "calc.clamp() value")?;
            let lof = coerce_to_f64(lo, "calc.clamp() min")?;
            let hif = coerce_to_f64(hi, "calc.clamp() max")?;
            if lof > hif {
                return Err(err_msg(format!("calc.clamp() min ({lof}) > max ({hif})")));
            }
            Ok(Value::Float(vf.clamp(lof, hif)))
        }
        _ => Err(err_msg(format!("calc.clamp() requer 3 argumentos, recebeu {}", args.len()))),
    }
}

// Helper — coerce Int ou Float para f64
fn coerce_to_f64(v: &Value, ctx: &str) -> SourceResult<f64> {
    match v {
        Value::Int(i)   => Ok(*i as f64),
        Value::Float(f) => Ok(*f),
        other => Err(err_msg(format!(
            "{ctx}: esperava Int ou Float, recebeu {}", other.type_name()
        ))),
    }
}

// Helper — guarda NaN/Inf em resultados Float
fn guard_float(f: f64) -> SourceResult<Value> {
    if f.is_nan() {
        Err(err_msg("resultado não é um número (NaN)"))
    } else if f.is_infinite() {
        Err(err_msg("resultado é infinito"))
    } else {
        Ok(Value::Float(f))
    }
}
```

---

## Tarefa 4 — Registar em `make_stdlib()`

```rust
fn make_stdlib() -> Scope {
    // ... existentes (type, len, range, rgb, luma) ...

    // Conversões de tipo
    scope.define("str",   Value::Func(Func::native("str",   native_str)));
    scope.define("int",   Value::Func(Func::native("int",   native_int)));
    scope.define("float", Value::Func(Func::native("float", native_float)));

    // Módulo calc — Dict com funções
    scope.define("calc", make_calc_module());

    scope
}
```

---

## Tarefa 5 — Suporte a field access em eval() para `calc.abs(x)`

`calc.abs(x)` é avaliado como `(calc.abs)(x)` — field access seguido de
chamada de função. Verificar se `eval_expr` já suporta `Expr::FieldAccess`
sobre `Value::Dict`.

```bash
# Ver se FieldAccess já está implementado em eval_expr
grep -n "FieldAccess\|field_access\|Field\b" \
  01_core/src/rules/eval.rs | head -20

# Ver a API de ast::FieldAccess
grep -n "pub fn field\|FieldAccess\|fn target\|fn field" \
  01_core/src/entities/ast/expr.rs | head -20
```

Se `FieldAccess` não estiver implementado:

```rust
// Em eval_expr — adicionar arm:
Expr::FieldAccess(access) => {
    let target = eval_expr(access.target(), scopes, ctx)?;
    let field  = access.field().as_str().to_string();
    match &target {
        Value::Dict(d) => d.get(field.as_str())
            .cloned()
            .ok_or_else(|| vec![SourceDiagnostic::error(
                access.span(),
                format!("campo '{field}' não existe"),
            )]),
        other => Err(vec![SourceDiagnostic::error(
            access.span(),
            format!("field access não suportado em {}", other.type_name()),
        )]),
    }
}
```

---

## Tarefa 6 — Testes

```rust
// ── str() ────────────────────────────────────────────────────────────────
#[test]
fn native_str_de_int() {
    assert_eq!(native_str(&[Value::Int(42)]), Ok(Value::Str("42".into())));
}

#[test]
fn native_str_de_float() {
    assert_eq!(native_str(&[Value::Float(3.14)]), Ok(Value::Str("3.14".into())));
}

#[test]
fn native_str_de_bool() {
    assert_eq!(native_str(&[Value::Bool(true)]),  Ok(Value::Str("true".into())));
    assert_eq!(native_str(&[Value::Bool(false)]), Ok(Value::Str("false".into())));
}

#[test]
fn native_str_identity() {
    assert_eq!(native_str(&[Value::Str("hello".into())]), Ok(Value::Str("hello".into())));
}

#[test]
fn native_str_de_none() {
    assert_eq!(native_str(&[Value::None]), Ok(Value::Str("none".into())));
}

// ── int() ─────────────────────────────────────────────────────────────────
#[test]
fn native_int_de_int() {
    assert_eq!(native_int(&[Value::Int(42)]), Ok(Value::Int(42)));
}

#[test]
fn native_int_de_str() {
    assert_eq!(native_int(&[Value::Str("42".into())]), Ok(Value::Int(42)));
    assert!(native_int(&[Value::Str("abc".into())]).is_err());
}

#[test]
fn native_int_de_bool() {
    assert_eq!(native_int(&[Value::Bool(true)]),  Ok(Value::Int(1)));
    assert_eq!(native_int(&[Value::Bool(false)]), Ok(Value::Int(0)));
}

// ── float() ───────────────────────────────────────────────────────────────
#[test]
fn native_float_de_int() {
    assert_eq!(native_float(&[Value::Int(3)]), Ok(Value::Float(3.0)));
}

#[test]
fn native_float_de_str() {
    assert_eq!(native_float(&[Value::Str("3.14".into())]), Ok(Value::Float(3.14)));
    assert!(native_float(&[Value::Str("abc".into())]).is_err());
}

// ── calc.abs() ────────────────────────────────────────────────────────────
#[test]
fn calc_abs_int() {
    assert_eq!(calc_abs(&[Value::Int(-5)]),  Ok(Value::Int(5)));
    assert_eq!(calc_abs(&[Value::Int(5)]),   Ok(Value::Int(5)));
    assert_eq!(calc_abs(&[Value::Int(0)]),   Ok(Value::Int(0)));
}

#[test]
fn calc_abs_float() {
    assert_eq!(calc_abs(&[Value::Float(-3.14)]), Ok(Value::Float(3.14)));
}

// ── calc.pow() ────────────────────────────────────────────────────────────
#[test]
fn calc_pow_int() {
    assert_eq!(calc_pow(&[Value::Int(2), Value::Int(10)]), Ok(Value::Int(1024)));
    assert_eq!(calc_pow(&[Value::Int(2), Value::Int(0)]),  Ok(Value::Int(1)));
}

#[test]
fn calc_pow_float() {
    let r = calc_pow(&[Value::Float(2.0), Value::Float(0.5)]);
    assert!(matches!(r, Ok(Value::Float(f)) if (f - std::f64::consts::SQRT_2).abs() < 1e-10));
}

#[test]
fn calc_pow_negativo_retorna_err() {
    assert!(calc_pow(&[Value::Int(2), Value::Int(-1)]).is_err());
}

// ── calc.sqrt() ───────────────────────────────────────────────────────────
#[test]
fn calc_sqrt_positivo() {
    assert_eq!(calc_sqrt(&[Value::Float(4.0)]), Ok(Value::Float(2.0)));
    assert_eq!(calc_sqrt(&[Value::Int(4)]),     Ok(Value::Float(2.0)));
}

#[test]
fn calc_sqrt_negativo_retorna_err() {
    assert!(calc_sqrt(&[Value::Float(-1.0)]).is_err());
}

// ── calc.floor / ceil / round ─────────────────────────────────────────────
#[test]
fn calc_floor_ceil_round() {
    assert_eq!(calc_floor(&[Value::Float(3.7)]), Ok(Value::Int(3)));
    assert_eq!(calc_ceil(&[Value::Float(3.2)]),  Ok(Value::Int(4)));
    assert_eq!(calc_round(&[Value::Float(3.5)]), Ok(Value::Int(4)));
    assert_eq!(calc_round(&[Value::Float(3.4)]), Ok(Value::Int(3)));
}

// ── calc.min / max ────────────────────────────────────────────────────────
#[test]
fn calc_min_max_int() {
    assert_eq!(calc_min(&[Value::Int(3), Value::Int(1), Value::Int(2)]), Ok(Value::Int(1)));
    assert_eq!(calc_max(&[Value::Int(3), Value::Int(1), Value::Int(2)]), Ok(Value::Int(3)));
}

#[test]
fn calc_min_vazio_retorna_err() {
    assert!(calc_min(&[]).is_err());
    assert!(calc_max(&[]).is_err());
}

// ── calc.clamp ────────────────────────────────────────────────────────────
#[test]
fn calc_clamp_int() {
    assert_eq!(calc_clamp(&[Value::Int(5), Value::Int(0), Value::Int(10)]),  Ok(Value::Int(5)));
    assert_eq!(calc_clamp(&[Value::Int(-5), Value::Int(0), Value::Int(10)]), Ok(Value::Int(0)));
    assert_eq!(calc_clamp(&[Value::Int(15), Value::Int(0), Value::Int(10)]), Ok(Value::Int(10)));
}

#[test]
fn calc_clamp_min_maior_max_retorna_err() {
    assert!(calc_clamp(&[Value::Float(5.0), Value::Float(10.0), Value::Float(0.0)]).is_err());
}

// ── Pipeline end-to-end ───────────────────────────────────────────────────
#[test]
fn pipeline_str_conversao() {
    let world = MockWorld::new("#let s = str(42)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("s"), Some(&Value::Str("42".into())));
}

#[test]
fn pipeline_calc_abs() {
    let world = MockWorld::new("#let x = calc.abs(-5)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("x"), Some(&Value::Int(5)));
}

#[test]
fn pipeline_calc_pow() {
    let world = MockWorld::new("#let x = calc.pow(2, 8)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("x"), Some(&Value::Int(256)));
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

# Confirmar que DEBT-4 está encerrado
grep "DEBT-4" 00_nucleo/DEBT.md && echo "VERIFICAR" || echo "OK — DEBT-4 removido"
```

Critérios de conclusão:
- `native_str`, `native_int`, `native_float` em stdlib com semântica correcta ✓
- Semântica de `int(float)` alinhada com diagnóstico do original ✓
- `make_calc_module()` retorna `Value::Dict` com 9 entradas ✓
- `calc.abs`, `calc.pow`, `calc.sqrt`, `calc.floor`, `calc.ceil`,
  `calc.round`, `calc.min`, `calc.max`, `calc.clamp` implementadas ✓
- `str`, `int`, `float`, `calc` registados em `make_stdlib()` ✓
- `Expr::FieldAccess` sobre `Value::Dict` funciona em `eval_expr` ✓
- `pipeline_calc_abs` e `pipeline_calc_pow` passam ✓
- DEBT-4 removido de `DEBT.md` (ou marcado como encerrado) ✓
- Zero violations ✓
- Testes não regridem (353 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**
- Semântica real de `int(float)` no original — truncamento ou Err?
- Se `int("0xFF")` é suportado no original
- Como `calc` é exposto no original — `Value::Dict`, `Value::Module`, ou outro?
- Se `calc.min`/`calc.max` aceitam número variável de args no original

**Da implementação:**
- Se `Expr::FieldAccess` já estava implementado ou foi adicionado neste passo
- Se `format_float` produziu o formato correcto para `3.14`, `0.1`, `1e-10`
- Se `calc_pow` com expoente negativo retornou Err como esperado

**Número total de testes e zero violations.**

**DEBT-4 encerrado. Go para Passo 28 — DEBT-3: safety rails
(`while` limit, `MAX_CALL_DEPTH`, detecção semântica de ciclos).**
