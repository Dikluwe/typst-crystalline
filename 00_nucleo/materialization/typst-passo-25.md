# Passo 25 — DEBT-4: Value incompleto — Length, Color, Ratio, Angle

## Estado actual antes de começar

Ler antes de começar:

- `01_core/src/entities/value.rs` — enum com 11 variantes actuais e ~20 comentadas
- `01_core/src/rules/stdlib.rs` — `native_type`, `native_len`, `native_range`
- `01_core/src/rules/eval.rs` — `eval_expr`, `eval_binary_op`
- `DEBT.md` — entrada DEBT-4

Pré-condição: `cargo test` — 368 testes (326 L1 + 42 L3), 5 ignored, zero violations.

**Contexto de DEBT-4**: o `Value` actual tem 11 variantes cobrindo primitivos,
colecções e tipos de infra. As ~20 restantes bloqueiam funções nativas que retornam
`Length`, `Color`, `Ratio`, `Angle` — quando invocadas, devolvem `Value::None`
em vez do valor correcto. Este passo adiciona o grupo de tipos tipográficos
fundamentais e desbloqueia as funções nativas correspondentes.

**Fronteira deste passo**: `Length`, `Color`, `Ratio`, `Angle`, `Auto`.
As variantes `Relative`, `Fraction`, `Symbol`, `Version`, `Bytes`, `Decimal`,
`Duration`, `Gradient`, `Tiling` ficam comentadas — sem ADR e sem tipo migrado,
não se tocam.

---

## Decisões obrigatórias pré-implementação

### PartialEq e precisão numérica — regra de ouro

`PartialEq` em L1 de produção representa valores matemáticos **exactos**.
Não sobrescrever o `derive(PartialEq)` para embutir tolerância — isso esconderia
ruído numérico que deve ser explícito e observável.

A conversão `deg → rad → deg` pode introduzir ruído IEEE 754. Esse ruído é
**correcto e esperado** — o código de produção deve representá-lo fielmente.
A tolerância existe apenas nos testes, via macro `assert_approx_eq!`.

```rust
// CORRECTO — PartialEq exacto em produção
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

// ERRADO — tolerância embutida em produção
impl PartialEq for Angle {
    fn eq(&self, other: &Self) -> bool {
        (self.0 - other.0).abs() < 1e-10  // NUNCA FAZER ISTO EM L1
    }
}
```

### `assert_approx_eq!` — apenas em testes

Antes de escrever qualquer teste com floats, verificar se `approx` está disponível
como dev-dependency:

```bash
grep "approx" 01_core/Cargo.toml
```

Se não estiver, adicionar:

```toml
# 01_core/Cargo.toml
[dev-dependencies]
approx = "0.5"
```

Se `approx` não for viável (política de deps), definir a macro localmente no módulo
de testes — **nunca fora de `#[cfg(test)]`**:

```rust
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => {
        assert_approx_eq!($a, $b, 1e-10)
    };
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
        assert!(
            (a - b).abs() < eps,
            "assert_approx_eq falhou: |{a} - {b}| = {} >= {eps}",
            (a - b).abs()
        );
    }};
}
```

### Propagação de `Err` em somas mistas — regra de fail-fast

Soma mista `Pt + Em` retorna `Err` via o mecanismo normal de propagação.
Três padrões proibidos:

```rust
// PROIBIDO 1 — panic
(BinOp::Add, Value::Length(Length::Em(_)), Value::Length(Length::Pt(_))) =>
    panic!("soma mista não suportada"),

// PROIBIDO 2 — absorver silenciosamente
(BinOp::Add, Value::Length(_), Value::Length(_)) =>
    Ok(Value::None),

// PROIBIDO 3 — resolver com font-size arbitrário
(BinOp::Add, Value::Length(Length::Em(e)), Value::Length(Length::Pt(p))) =>
    Ok(Value::Length(Length::Pt(e * 12.0 + p))),  // 12.0 hardcoded destrói determinismo
```

Padrão correcto — `Err` explícito e propagável:

```rust
(BinOp::Add, Value::Length(a), Value::Length(b)) => match (a, b) {
    (Length::Pt(x), Length::Pt(y)) => Ok(Value::Length(Length::Pt(x + y))),
    (Length::Em(x), Length::Em(y)) => Ok(Value::Length(Length::Em(x + y))),
    (a, b) => Err(vec![SourceDiagnostic::error(
        Span::detached(),
        format!(
            "não é possível somar {} com {} — requer Relative (DEBT-4/ADR-0028)",
            length_unit_name(&a),
            length_unit_name(&b),
        ),
    )]),
},

// helper local — não é API pública
fn length_unit_name(l: &Length) -> &'static str {
    match l { Length::Pt(_) => "pt", Length::Em(_) => "em" }
}
```

---

## Tarefa 1 — Diagnóstico de tipos no original

```bash
# Length — estrutura interna (newtype? enum de unidades?)
grep -rA 20 "^pub struct Length\b\|^pub enum Length\b" \
  lab/typst-original/crates/typst-library/src/layout/ | head -30

# Unidades de Length — em, pt, cm, mm, in, fr, %?
grep -rn "Em\b\|Pt\b\|Cm\b\|Mm\b\|In\b\|Unit\b" \
  lab/typst-original/crates/typst-library/src/layout/length.rs 2>/dev/null | head -20

# Color — enum de espaços de cor ou struct?
grep -rA 20 "^pub enum Color\b\|^pub struct Color\b" \
  lab/typst-original/crates/typst-library/src/visualize/ 2>/dev/null | head -30

# Ratio — newtype f64 (percentagem)?
grep -rA 10 "^pub struct Ratio\b" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -15

# Angle — newtype f64 (radianos ou graus)?
grep -rA 10 "^pub struct Angle\b" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -15

# Auto — unit struct ou enum?
grep -rA 5 "^pub struct AutoValue\b\|^pub enum Auto\b\|Auto,\b" \
  lab/typst-original/crates/typst-library/src/ 2>/dev/null | head -15

# Funções nativas que retornam estes tipos
grep -rn "-> Length\|-> Color\|-> Ratio\|-> Angle" \
  lab/typst-original/crates/typst-library/src/foundations/ 2>/dev/null | head -20
```

**Parar após diagnóstico. Reportar antes de escrever qualquer código.**

---

## Tarefa 2 — Decisão de representação (ADR-0028)

```text
ADR-0028 — Representação simplificada dos tipos tipográficos em Value
Data: 2026-03-29
Status: Accepted

Decisão: tipos tipográficos usam representações simplificadas em L1
para este passo. A fidelidade ao original é adiada até que StyleChain
(DEBT-1) e o sistema de unidades completo sejam necessários.

Length: enum { Pt(f64), Em(f64) } — sem somas mistas (requer Relative).
Ratio:  newtype f64 (0.0–1.0 representa 0%–100%).
Angle:  newtype f64 (radianos internamente); construtores deg/rad.
Color:  enum { Rgb{r,g,b: u8}, Rgba{r,g,b,a: u8} } — sem CMYK/Oklab.
Auto:   unit variant sem dados.

PartialEq: derive exacto em produção — sem tolerância embutida.
           Tolerância apenas em testes via assert_approx_eq! (cfg(test)).

Somas mistas Pt+Em: Err explícito e propagável — nunca panic, nunca None,
                     nunca resolução com font-size hardcoded.

Consequência: rgb(r,g,b) e luma(l) implementadas; espaços avançados
(Oklab, CMYK) retornam Value::None com comentário DEBT-4.
Próxima revisão: quando StyleChain (DEBT-1) for implementado no Passo 30.
```

---

## Tarefa 3 — Tipos em `layout_types.rs`

Adicionar a `01_core/src/entities/layout_types.rs` (ou `typographic_types.rs`
se o ficheiro ficar grande demais):

```rust
// Ratio — percentagem normalizada
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Ratio(pub f64);  // 0.5 = 50%

impl Ratio {
    pub fn from_percent(pct: f64) -> Self { Self(pct / 100.0) }
    pub fn to_percent(self) -> f64 { self.0 * 100.0 }
    pub fn get(self) -> f64 { self.0 }
}

// Angle — ângulo (interno: radianos)
// PartialEq é exacto — derive sem tolerância (ADR-0028).
// A conversão deg→rad pode introduzir ruído IEEE 754 — é correcto e observável.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Angle(f64);

impl Angle {
    pub fn rad(r: f64) -> Self { Self(r) }
    pub fn deg(d: f64) -> Self { Self(d.to_radians()) }
    pub fn to_rad(self) -> f64 { self.0 }
    pub fn to_deg(self) -> f64 { self.0.to_degrees() }
}

// Length — comprimento tipográfico (subset: Pt e Em)
// Somas mistas Pt+Em retornam Err em eval_binary_op (ADR-0028).
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    Pt(f64),   // pontos absolutos
    Em(f64),   // múltiplos do font-size actual
    // Mm, Cm, In, Fr, Relative — adiados (ADR-0028)
}

impl Length {
    pub fn pt(v: f64) -> Self { Self::Pt(v) }
    pub fn em(v: f64) -> Self { Self::Em(v) }

    /// Resolve para Pt dado um font-size em pt.
    /// Apenas para uso em L3 (Layouter) onde o font-size é conhecido.
    /// Em(x) → Pt(x * font_size_pt)
    pub fn resolve_pt(self, font_size_pt: f64) -> f64 {
        match self {
            Self::Pt(v) => v,
            Self::Em(v) => v * font_size_pt,
        }
    }
}

// Color — espaço RGB/RGBA (subset)
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    Rgb  { r: u8, g: u8, b: u8 },
    Rgba { r: u8, g: u8, b: u8, a: u8 },
    // Luma, Cmyk, Oklab, Oklch — adiados (ADR-0028)
}

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self { Self::Rgb { r, g, b } }
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self { Self::Rgba { r, g, b, a } }

    /// Converte para componentes RGBA normalizadas (0.0–1.0).
    pub fn to_rgba_f32(self) -> (f32, f32, f32, f32) {
        let norm = |v: u8| v as f32 / 255.0;
        match self {
            Self::Rgb  { r, g, b }    => (norm(r), norm(g), norm(b), 1.0),
            Self::Rgba { r, g, b, a } => (norm(r), norm(g), norm(b), norm(a)),
        }
    }
}
```

---

## Tarefa 4 — Novas variantes em `value.rs`

```rust
pub enum Value {
    // ... existentes ...

    // ── Passo 25 — tipos tipográficos (ADR-0028) ─────────────────────────
    Length(crate::entities::layout_types::Length),
    Ratio(crate::entities::layout_types::Ratio),
    Angle(crate::entities::layout_types::Angle),
    Color(crate::entities::layout_types::Color),
    Auto,

    // ── Variantes futuras — NÃO implementar sem ADR e tipo migrado ───────
    // Relative(Relative),   // abs + ratio — requer StyleChain (DEBT-1)
    // Fraction(f64),        // fr — fracção de espaço disponível
    // Symbol(Symbol),       // símbolo Unicode
    // ...
}
```

Actualizar `type_name()`:

```rust
Self::Length(_) => "length",
Self::Ratio(_)  => "ratio",
Self::Angle(_)  => "angle",
Self::Color(_)  => "color",
Self::Auto      => "auto",
```

Conversões `From`:

```rust
impl From<Length> for Value { fn from(v: Length) -> Self { Self::Length(v) } }
impl From<Ratio>  for Value { fn from(v: Ratio)  -> Self { Self::Ratio(v) } }
impl From<Angle>  for Value { fn from(v: Angle)  -> Self { Self::Angle(v) } }
impl From<Color>  for Value { fn from(v: Color)  -> Self { Self::Color(v) } }
```

---

## Tarefa 5 — Funções nativas

### `rgb(r, g, b)` e `rgb(r, g, b, a)`

```rust
pub fn native_rgb(args: &[Value]) -> SourceResult<Value> {
    let to_u8 = |v: &Value, name: &str| -> SourceResult<u8> {
        match v {
            Value::Int(i) if (0..=255).contains(i) => Ok(*i as u8),
            Value::Int(i) => Err(err_msg(
                format!("{name} deve estar entre 0 e 255, recebeu {i}")
            )),
            other => Err(err_msg(
                format!("{name} deve ser Int, recebeu {}", other.type_name())
            )),
        }
    };
    match args {
        [r, g, b] => Ok(Value::Color(Color::rgb(
            to_u8(r, "r")?, to_u8(g, "g")?, to_u8(b, "b")?
        ))),
        [r, g, b, a] => Ok(Value::Color(Color::rgba(
            to_u8(r, "r")?, to_u8(g, "g")?, to_u8(b, "b")?, to_u8(a, "a")?
        ))),
        _ => Err(err_msg(format!(
            "rgb() requer 3 ou 4 argumentos, recebeu {}", args.len()
        ))),
    }
}
```

### `luma(l)` — escala de cinzentos

```rust
pub fn native_luma(args: &[Value]) -> SourceResult<Value> {
    match args {
        [Value::Int(l)] if (0..=255).contains(l) => {
            let v = *l as u8;
            Ok(Value::Color(Color::rgb(v, v, v)))
        }
        [Value::Int(l)] => Err(err_msg(
            format!("luma() requer valor entre 0 e 255, recebeu {l}")
        )),
        [other] => Err(err_msg(
            format!("luma() requer Int, recebeu {}", other.type_name())
        )),
        _ => Err(err_msg(format!(
            "luma() requer 1 argumento, recebeu {}", args.len()
        ))),
    }
}
```

Registar em `make_stdlib()`:

```rust
scope.define("rgb",  Value::Func(Func::native("rgb",  native_rgb)));
scope.define("luma", Value::Func(Func::native("luma", native_luma)));
```

---

## Tarefa 6 — Operações aritméticas

```rust
// Ratio aritmética
(BinOp::Mul, Value::Ratio(a), Value::Int(b)) =>
    Ok(Value::Ratio(Ratio(a.get() * *b as f64))),
(BinOp::Mul, Value::Int(a), Value::Ratio(b)) =>
    Ok(Value::Ratio(Ratio(*a as f64 * b.get()))),
(BinOp::Add, Value::Ratio(a), Value::Ratio(b)) =>
    Ok(Value::Ratio(Ratio(a.get() + b.get()))),
(BinOp::Sub, Value::Ratio(a), Value::Ratio(b)) =>
    Ok(Value::Ratio(Ratio(a.get() - b.get()))),

// Ratio × Length (50% * 12pt = 6pt)
(BinOp::Mul, Value::Ratio(r), Value::Length(Length::Pt(v))) =>
    Ok(Value::Length(Length::Pt(r.get() * v))),
(BinOp::Mul, Value::Length(Length::Pt(v)), Value::Ratio(r)) =>
    Ok(Value::Length(Length::Pt(v * r.get()))),

// Length aritmética — mesmo tipo
(BinOp::Add, Value::Length(a), Value::Length(b)) => match (a, b) {
    (Length::Pt(x), Length::Pt(y)) => Ok(Value::Length(Length::Pt(x + y))),
    (Length::Em(x), Length::Em(y)) => Ok(Value::Length(Length::Em(x + y))),
    (a, b) => Err(vec![SourceDiagnostic::error(
        Span::detached(),
        format!(
            "não é possível somar {} com {} — requer Relative (DEBT-4/ADR-0028)",
            length_unit_name(&a),
            length_unit_name(&b),
        ),
    )]),
},
(BinOp::Sub, Value::Length(a), Value::Length(b)) => match (a, b) {
    (Length::Pt(x), Length::Pt(y)) => Ok(Value::Length(Length::Pt(x - y))),
    (Length::Em(x), Length::Em(y)) => Ok(Value::Length(Length::Em(x - y))),
    (a, b) => Err(vec![SourceDiagnostic::error(
        Span::detached(),
        format!(
            "não é possível subtrair {} de {} — requer Relative (DEBT-4/ADR-0028)",
            length_unit_name(&b),
            length_unit_name(&a),
        ),
    )]),
},
```

Helper local em `eval.rs` (não é API pública):

```rust
fn length_unit_name(l: &Length) -> &'static str {
    match l { Length::Pt(_) => "pt", Length::Em(_) => "em" }
}
```

---

## Tarefa 7 — Testes

```rust
// ── setup: assert_approx_eq! (apenas cfg(test)) ───────────────────────────
#[cfg(test)]
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr) => { assert_approx_eq!($a, $b, 1e-10) };
    ($a:expr, $b:expr, $eps:expr) => {{
        let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
        assert!(
            (a - b).abs() < eps,
            "assert_approx_eq falhou: |{a} - {b}| = {} >= {eps}",
            (a - b).abs()
        );
    }};
}

// ── Length ────────────────────────────────────────────────────────────────
#[test]
fn length_resolve_pt() {
    assert_eq!(Length::pt(12.0).resolve_pt(12.0), 12.0);
    assert_eq!(Length::em(1.5).resolve_pt(12.0), 18.0);
    assert_eq!(Length::em(2.0).resolve_pt(10.0), 20.0);
}

// ── Ratio ─────────────────────────────────────────────────────────────────
#[test]
fn ratio_percent_roundtrip() {
    let r = Ratio::from_percent(50.0);
    assert_approx_eq!(r.get(), 0.5);
    assert_approx_eq!(r.to_percent(), 50.0);
}

// ── Angle ─────────────────────────────────────────────────────────────────
#[test]
fn angle_deg_rad_usa_approx() {
    // A conversão deg→rad→deg pode introduzir ruído IEEE 754 — correcto e esperado.
    // assert_approx_eq! para a tolerância; assert_eq! não é usável aqui.
    let a = Angle::deg(180.0);
    assert_approx_eq!(a.to_rad(), std::f64::consts::PI);
    assert_approx_eq!(a.to_deg(), 180.0);
}

#[test]
fn angle_partial_eq_e_exacto() {
    // PartialEq em produção é exacto — dois Angle::deg(180.0) são iguais
    // porque produzem o mesmo f64 de radianos.
    let a1 = Angle::deg(180.0);
    let a2 = Angle::deg(180.0);
    assert_eq!(a1, a2);
    // Ângulos diferentes NÃO são iguais — sem tolerância embutida.
    let a3 = Angle::deg(180.0 + 1e-15);
    // a1 != a3 pode ou não ser verdade dependendo do arredondamento IEEE 754
    // — documentar o comportamento real no relatório, não assumir.
    let _ = a3;
}

// ── Color ─────────────────────────────────────────────────────────────────
#[test]
fn color_to_rgba_f32() {
    let (r, g, b, a) = Color::rgb(255, 0, 128).to_rgba_f32();
    assert_approx_eq!(r, 1.0, 1e-3);
    assert_approx_eq!(g, 0.0, 1e-3);
    assert_approx_eq!(b, 0.502, 1e-3);
    assert_approx_eq!(a, 1.0, 1e-3);
}

// ── Value::type_name ──────────────────────────────────────────────────────
#[test]
fn value_type_names_novos() {
    assert_eq!(Value::Length(Length::pt(12.0)).type_name(), "length");
    assert_eq!(Value::Ratio(Ratio(0.5)).type_name(),        "ratio");
    assert_eq!(Value::Angle(Angle::deg(90.0)).type_name(),  "angle");
    assert_eq!(Value::Color(Color::rgb(0, 0, 0)).type_name(), "color");
    assert_eq!(Value::Auto.type_name(),                     "auto");
}

// ── stdlib ────────────────────────────────────────────────────────────────
#[test]
fn stdlib_rgb_tres_args() {
    let r = native_rgb(&[Value::Int(255), Value::Int(0), Value::Int(128)]);
    assert_eq!(r, Ok(Value::Color(Color::rgb(255, 0, 128))));
}

#[test]
fn stdlib_rgb_quatro_args() {
    let r = native_rgb(&[
        Value::Int(255), Value::Int(0), Value::Int(0), Value::Int(200)
    ]);
    assert_eq!(r, Ok(Value::Color(Color::rgba(255, 0, 0, 200))));
}

#[test]
fn stdlib_rgb_out_of_range() {
    assert!(native_rgb(&[Value::Int(256), Value::Int(0), Value::Int(0)]).is_err());
    assert!(native_rgb(&[Value::Int(-1),  Value::Int(0), Value::Int(0)]).is_err());
}

#[test]
fn stdlib_luma() {
    let r = native_luma(&[Value::Int(128)]);
    assert_eq!(r, Ok(Value::Color(Color::rgb(128, 128, 128))));
}

// ── Pipeline ──────────────────────────────────────────────────────────────
#[test]
fn pipeline_rgb_em_let() {
    let world = MockWorld::new("#let c = rgb(255, 0, 0)");
    let src = world.source(world.main()).unwrap();
    let m = eval_for_test(&world, &src).unwrap();
    assert_eq!(m.scope().get("c"), Some(&Value::Color(Color::rgb(255, 0, 0))));
}

// ── Aritmética ────────────────────────────────────────────────────────────
#[test]
fn ratio_mul_int() {
    let r = eval_binary_op(BinOp::Mul, Value::Ratio(Ratio(0.5)), Value::Int(2));
    assert_eq!(r, Ok(Value::Ratio(Ratio(1.0))));
}

#[test]
fn length_add_pt_pt() {
    let r = eval_binary_op(
        BinOp::Add,
        Value::Length(Length::pt(6.0)),
        Value::Length(Length::pt(6.0)),
    );
    assert_eq!(r, Ok(Value::Length(Length::pt(12.0))));
}

#[test]
fn length_add_misto_retorna_err_propagavel() {
    // Verifica que o Err é retornado (não panic, não Value::None).
    let r = eval_binary_op(
        BinOp::Add,
        Value::Length(Length::pt(6.0)),
        Value::Length(Length::em(1.0)),
    );
    assert!(r.is_err(), "Pt+Em deve ser Err — recebeu: {:?}", r);
    // A mensagem deve mencionar as unidades para debugging útil
    let msg = &r.unwrap_err()[0].message;
    assert!(
        msg.contains("pt") && msg.contains("em"),
        "mensagem deve identificar as unidades: {msg:?}"
    );
}

#[test]
fn soma_mista_propaga_ate_eval() {
    // Documenta o comportamento do parser com "1pt + 1em".
    // Pode ser Ok(Value::None) se o parser não reconhecer "1pt" como Length,
    // ou Err se reconhecer e o eval_binary_op propagar.
    // O que NÃO pode acontecer: panic.
    let world = MockWorld::new("#let x = 1pt + 1em");
    let src = world.source(world.main()).unwrap();
    let result = eval_for_test(&world, &src);
    // Documentar o resultado real no relatório — não assumir.
    let _ = result;
}

// ── type() reconhece novos tipos ─────────────────────────────────────────
#[test]
fn stdlib_type_length() {
    let r = native_type(&[Value::Length(Length::pt(12.0))]);
    assert_eq!(r, Ok(Value::Str("length".into())));
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

# Sem novas deps em L1 de produção (approx é dev-only)
cargo tree -p typst-core --depth 1
grep "^approx" 01_core/Cargo.toml \
  && echo "VERIFICAR — approx deve estar em [dev-dependencies]" \
  || echo "OK — sem approx em deps de produção"
```

Critérios de conclusão:

- ADR-0028 registada ✓
- `assert_approx_eq!` apenas em `#[cfg(test)]` (nunca em produção) ✓
- `PartialEq` em `Angle`, `Ratio`, `Length`, `Color` é `derive` exacto ✓
- `Length { Pt(f64), Em(f64) }` com `resolve_pt` ✓
- `Ratio(f64)` com `from_percent` / `to_percent` ✓
- `Angle(f64)` com construtores `deg` / `rad` ✓
- `Color { Rgb, Rgba }` com `to_rgba_f32()` ✓
- `Value::Auto` (unit variant) ✓
- `type_name()` correcto para todas as variantes novas ✓
- `native_rgb` (3 e 4 args) e `native_luma` em stdlib ✓
- `rgb()` e `luma()` registados em `make_stdlib()` ✓
- Aritmética `Ratio * Int`, `Length::Pt + Length::Pt` ✓
- Soma mista `Pt + Em` → `Err` com mensagem que identifica as unidades ✓
- `length_add_misto_retorna_err_propagavel` passa — sem panic, sem `Value::None` ✓
- `pipeline_rgb_em_let` passa ✓
- Zero violations ✓
- `approx` em `[dev-dependencies]`, não em `[dependencies]` ✓
- Testes não regridem (368 base + novos) ✓

---

## Ao terminar, reportar

**Do diagnóstico:**

- Estrutura real de `Length` no original — newtype, enum, ou struct com campos `em` + `abs`?
- Se `Color` no original é enum ou struct com discriminante interno
- Se `Angle` usa radianos ou graus internamente no original
- Quais funções nativas de cor/comprimento o original expõe e quais ficam para Passos 26–27

**Da implementação:**

- Se `approx` foi adicionado como dev-dependency ou se foi criada a macro local
- Se `Angle::deg(180.0 + 1e-15) != Angle::deg(180.0)` — documentar o comportamento IEEE 754 real
- Se a soma mista `Pt + Em` produziu `Err` com as unidades na mensagem
- Se `soma_mista_propaga_ate_eval` foi `Ok` ou `Err` — depende do parser
- Se `resolve_pt` foi necessário já neste passo ou fica para o Passo 30 (StyleChain)

**Número total de testes e zero violations.**

**Go para Passo 26 — continuação de DEBT-4: funções nativas de conversão e cálculo
(`str()`, `int()`, `float()`, `calc.abs()`, `calc.pow()`, `calc.sqrt()`).**
