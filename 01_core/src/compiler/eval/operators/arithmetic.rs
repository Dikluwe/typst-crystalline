//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/operators/arithmetic.md
//! @prompt-hash a642f695
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! Braços aritméticos (`+` `-` `*` `/`), lógica booleana (`and`/`or`),
//! operadores unários e o saneamento NaN de `Length`. A fronteira de erro é
//! do nó `error_formatting`.

use crate::entities::ast::expr::{BinOp, UnOp};
use crate::entities::content::Content;
use crate::entities::decimal::Decimal;
use crate::entities::duration::Duration;
use crate::entities::value::Value;

use super::error_formatting::{binary_mismatch, unary_mismatch};

/// Avalia uma operação aritmética/booleana com semântica Typst.
///
/// Semântica confirmada com `lab/typst-original/crates/typst-library/src/foundations/ops.rs`:
/// - Int/Int → Float (não truncamento): `5/2 = 2.5`
/// - Int overflow → Err (checked_add/sub/mul/neg, como no original)
/// - Float: IEEE 754 propagado silenciosamente (sem guarda NaN/Inf)
/// - Divisão por zero → Err explícito (gate pré-match)
pub(crate) fn apply_binary(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    if matches!(op, BinOp::Div) {
        match &rhs {
            Value::Int(0) => return Err("cannot divide by zero".into()),
            Value::Float(f) if *f == 0.0 => return Err("cannot divide by zero".into()),
            Value::Decimal(d) if d.0.is_zero() => {
                return Err("cannot divide by zero".into())
            }
            // P713 — paridade com `is_zero()` do vanilla (`foundations/ops.rs:344-359`),
            // que cobre `Length` no mesmo gate genérico usado por todas as divisões.
            Value::Length(l) if l.is_zero() => return Err("cannot divide by zero".into()),
            // P818 — `is_zero()` do vanilla cobre também `Relative`, `Ratio`
            // e `Angle` (medido: `#(50% / 0%)` → "cannot divide by zero").
            Value::Relative(r)
                if matches!((r.abs.is_zero(), r.rel == 0.0), (true, true)) =>
            {
                return Err("cannot divide by zero".into())
            }
            Value::Ratio(r) if r.0 == 0.0 => return Err("cannot divide by zero".into()),
            Value::Angle(a) if a.to_rad() == 0.0 => {
                return Err("cannot divide by zero".into())
            }
            _ => {}
        }
    }

    match (op, lhs, rhs) {
        // ── Adição ──────────────────────────────────────────────────────────
        (BinOp::Add, Value::Int(a), Value::Int(b)) => {
            Ok(Value::Int(a.checked_add(b).ok_or("number too large")?))
        }
        (BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (BinOp::Add, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a + b as f64)),
        (BinOp::Add, Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
        // P404 — aritmética Decimal homogénea.
        (BinOp::Add, Value::Decimal(a), Value::Decimal(b)) => {
            Ok(Value::Decimal(Decimal(a.0 + b.0)))
        }
        (BinOp::Add, Value::Decimal(a), Value::Int(b)) => {
            a.0.checked_add(Decimal::from_i64(b).0)
                .map(|value| Value::Decimal(Decimal(value)))
                .ok_or_else(|| "value is too large".to_string())
        }
        (BinOp::Add, Value::Int(a), Value::Decimal(b)) => Decimal::from_i64(a)
            .0
            .checked_add(b.0)
            .map(|value| Value::Decimal(Decimal(value)))
            .ok_or_else(|| "value is too large".to_string()),
        // P405 — adição Duration homogénea (Passo 850: com sinal).
        (BinOp::Add, Value::Duration(a), Value::Duration(b)) => {
            match a.nanos.checked_add(b.nanos) {
                Some(sum) => Ok(Value::Duration(Duration::from_nanos(sum))),
                None => Err("overflow em duration + duration".into()),
            }
        }
        (BinOp::Add, Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + b.as_str())),
        (BinOp::Add, Value::Content(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![a, b])))
        }
        // **P900** — causa real do "crash 2" catalogado em P894
        // (`typst-passo-894-relatorio.md`, `"⟨" + x + "|"` dentro de função
        // de modo math): faltavam estes braços de `Str`/`Symbol`/`Content`
        // cruzados no operador `+` — só existiam em `join()` (P728, acima
        // neste ficheiro), nunca portados para `eval_binary_op`. Paridade
        // vanilla (`foundations/ops.rs::add`, medido em `lab/typst-original`):
        // `Symbol + Symbol` produz `Str`; qualquer combinação envolvendo
        // `Content` produz `Content` (coerção de `Str`/`Symbol` para texto).
        (BinOp::Add, Value::Symbol(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a.value, b.value).into()))
        }
        (BinOp::Add, Value::Str(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a, b.value).into()))
        }
        (BinOp::Add, Value::Symbol(a), Value::Str(b)) => {
            Ok(Value::Str(format!("{}{}", a.value, b).into()))
        }
        (BinOp::Add, Value::Str(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![Content::text(a), b])))
        }
        (BinOp::Add, Value::Content(a), Value::Str(b)) => {
            Ok(Value::Content(Content::sequence(vec![a, Content::text(b)])))
        }
        (BinOp::Add, Value::Symbol(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![
                Content::text(a.value.to_string()),
                b,
            ])))
        }
        (BinOp::Add, Value::Content(a), Value::Symbol(b)) => {
            Ok(Value::Content(Content::sequence(vec![
                a,
                Content::text(b.value.to_string()),
            ])))
        }
        // P720 — Array/Dict + Array/Dict (paridade `impl Add for Array/Dict`,
        // vanilla `foundations/array.rs:1203-1216`, `dict.rs:388-404`):
        // concatenação ordenada (array) / merge com o lado direito a vencer
        // em colisão, posição da primeira ocorrência preservada (dict,
        // semântica nativa de `IndexMap::extend`).
        (BinOp::Add, Value::Array(mut a), Value::Array(b)) => {
            a.extend(b);
            Ok(Value::Array(a))
        }
        (BinOp::Add, Value::Dict(mut a), Value::Dict(b)) => {
            a.extend(b);
            Ok(Value::Dict(a))
        }
        // ── Subtracção ──────────────────────────────────────────────────────
        (BinOp::Sub, Value::Int(a), Value::Int(b)) => {
            Ok(Value::Int(a.checked_sub(b).ok_or("number too large")?))
        }
        (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (BinOp::Sub, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a - b as f64)),
        (BinOp::Sub, Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
        // P404 — aritmética Decimal homogénea.
        (BinOp::Sub, Value::Decimal(a), Value::Decimal(b)) => {
            Ok(Value::Decimal(Decimal(a.0 - b.0)))
        }
        (BinOp::Sub, Value::Decimal(a), Value::Int(b)) => {
            a.0.checked_sub(Decimal::from_i64(b).0)
                .map(|value| Value::Decimal(Decimal(value)))
                .ok_or_else(|| "value is too large".to_string())
        }
        (BinOp::Sub, Value::Int(a), Value::Decimal(b)) => Decimal::from_i64(a)
            .0
            .checked_sub(b.0)
            .map(|value| Value::Decimal(Decimal(value)))
            .ok_or_else(|| "value is too large".to_string()),
        // P405 — subtracção Duration homogénea (Passo 850: com sinal).
        (BinOp::Sub, Value::Duration(a), Value::Duration(b)) => {
            match a.nanos.checked_sub(b.nanos) {
                Some(diff) => Ok(Value::Duration(Duration::from_nanos(diff))),
                None => Err("underflow em duration - duration".into()),
            }
        }
        // ── Multiplicação ────────────────────────────────────────────────────
        (BinOp::Mul, Value::Int(a), Value::Int(b)) => {
            Ok(Value::Int(a.checked_mul(b).ok_or("number too large")?))
        }
        (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        (BinOp::Mul, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a * b as f64)),
        (BinOp::Mul, Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
        // P404 — aritmética Decimal homogénea.
        (BinOp::Mul, Value::Decimal(a), Value::Decimal(b)) => {
            Ok(Value::Decimal(Decimal(a.0 * b.0)))
        }
        (BinOp::Mul, Value::Decimal(a), Value::Int(b)) => {
            a.0.checked_mul(Decimal::from_i64(b).0)
                .map(|value| Value::Decimal(Decimal(value)))
                .ok_or_else(|| "value is too large".to_string())
        }
        (BinOp::Mul, Value::Int(a), Value::Decimal(b)) => Decimal::from_i64(a)
            .0
            .checked_mul(b.0)
            .map(|value| Value::Decimal(Decimal(value)))
            .ok_or_else(|| "value is too large".to_string()),
        // P405 — multiplicação Duration por Int/Float (Passo 850: com sinal).
        (BinOp::Mul, Value::Duration(d), Value::Int(n))
        | (BinOp::Mul, Value::Int(n), Value::Duration(d)) => {
            match d.nanos.checked_mul(n as i128) {
                Some(prod) => Ok(Value::Duration(Duration::from_nanos(prod))),
                None => Err("overflow em duration * int".into()),
            }
        }
        (BinOp::Mul, Value::Duration(d), Value::Float(f))
        | (BinOp::Mul, Value::Float(f), Value::Duration(d)) => {
            Ok(Value::Duration(Duration::from_nanos((d.nanos as f64 * f) as i128)))
        }
        // P722 — Array * Int / Int * Array (paridade `ops.rs:274-275` +
        // `Array::repeat`, vanilla `foundations/array.rs:140-147`):
        // repetição por `cycle().take(len * n)`; n < 0 → erro do cast
        // `Int → usize` ("number must be at least zero", int.rs:507);
        // overflow de `len * n` → "cannot repeat this array {n} times".
        (BinOp::Mul, Value::Array(a), Value::Int(n))
        | (BinOp::Mul, Value::Int(n), Value::Array(a)) => {
            if n < 0 {
                return Err("number must be at least zero".into());
            }
            let count = a
                .len()
                .checked_mul(n as usize)
                .ok_or_else(|| format!("cannot repeat this array {n} times"))?;
            Ok(Value::Array(a.iter().cloned().cycle().take(count).collect()))
        }
        // P818-e — Str * Int / Int * Str (paridade `Str::repeat`, vanilla
        // `foundations/str.rs:92-99` + `ops.rs:272-273`): repetição;
        // n < 0 → erro do cast `Int → usize` ("number must be at least
        // zero"); overflow de bytes*n → "cannot repeat this string
        // {n} times".
        (BinOp::Mul, Value::Str(s), Value::Int(n))
        | (BinOp::Mul, Value::Int(n), Value::Str(s)) => {
            if n < 0 {
                return Err("number must be at least zero".into());
            }
            if s.len().checked_mul(n as usize).is_none() {
                return Err(format!("cannot repeat this string {n} times"));
            }
            Ok(Value::Str(s.as_str().repeat(n as usize).into()))
        }
        // P818 — Angle * Int|Float / Int|Float * Angle (vanilla
        // `ops.rs:241-246`). Alcançável desde P817 (`calc.asin` e cia.
        // devolvem `angle`; literais `90deg` já existiam).
        (BinOp::Mul, Value::Angle(a), Value::Int(n))
        | (BinOp::Mul, Value::Int(n), Value::Angle(a)) => Ok(Value::Angle(
            crate::entities::layout_types::Angle::rad(a.to_rad() * n as f64),
        )),
        (BinOp::Mul, Value::Angle(a), Value::Float(f))
        | (BinOp::Mul, Value::Float(f), Value::Angle(a)) => {
            Ok(Value::Angle(crate::entities::layout_types::Angle::rad(a.to_rad() * f)))
        }
        // ── Divisão — Int/Int → Float (semântica Typst, não truncamento) ────
        (BinOp::Div, Value::Int(a), Value::Int(b)) => {
            Ok(Value::Float(a as f64 / b as f64))
        }
        (BinOp::Div, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
        (BinOp::Div, Value::Float(a), Value::Int(b)) => Ok(Value::Float(a / b as f64)),
        (BinOp::Div, Value::Int(a), Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
        // P404 — divisão Decimal homogénea (div/0 já verificado acima).
        (BinOp::Div, Value::Decimal(a), Value::Decimal(b)) => {
            Ok(Value::Decimal(Decimal(a.0 / b.0)))
        }
        (BinOp::Div, Value::Decimal(a), Value::Int(b)) => {
            a.0.checked_div(Decimal::from_i64(b).0)
                .map(|value| Value::Decimal(Decimal(value)))
                .ok_or_else(|| "value is too large".to_string())
        }
        (BinOp::Div, Value::Int(a), Value::Decimal(b)) => Decimal::from_i64(a)
            .0
            .checked_div(b.0)
            .map(|value| Value::Decimal(Decimal(value)))
            .ok_or_else(|| "value is too large".to_string()),
        // P405 — divisão Duration por Int/Float/Duration (Passo 850: com sinal).
        (BinOp::Div, Value::Duration(d), Value::Int(n)) => {
            if n == 0 {
                return Err("divisão por zero".into());
            }
            Ok(Value::Duration(Duration::from_nanos(d.nanos / n as i128)))
        }
        (BinOp::Div, Value::Duration(d), Value::Float(f)) => {
            if f == 0.0 {
                return Err("divisão por zero".into());
            }
            Ok(Value::Duration(Duration::from_nanos((d.nanos as f64 / f) as i128)))
        }
        (BinOp::Div, Value::Duration(a), Value::Duration(b)) => {
            if b.nanos == 0 {
                return Err("divisão por zero".into());
            }
            Ok(Value::Float(a.nanos as f64 / b.nanos as f64))
        }
        // P818-d — Relative / Relative (paridade `Rel::try_div`, vanilla
        // `layout/rel.rs:128-137` + `ops.rs:330`): rel ambos zero → rácio de
        // abs (mesma regra de `Length/Length`); abs ambos zero → rel/rel;
        // misto (incomensurável) → erro. **Não** cobre as divisões mistas
        // Length↔Relative/Ratio (scope-out L0 `ops.md` §P713).
        (BinOp::Div, Value::Relative(a), Value::Relative(b)) => {
            let result = if a.rel == 0.0 && b.rel == 0.0 {
                let (x, y) = (&a.abs, &b.abs);
                if x.abs.is_zero() && y.abs.is_zero() {
                    Some(x.em / y.em)
                } else if x.em == 0.0 && y.em == 0.0 {
                    Some(x.abs.to_pt() / y.abs.to_pt())
                } else {
                    None
                }
            } else if a.abs.is_zero() && b.abs.is_zero() {
                Some(a.rel / b.rel)
            } else {
                None
            };
            result
                .map(Value::Float)
                .ok_or_else(|| "cannot divide these two relative lengths".to_string())
        }
        // P818-d — Ratio / Ratio (vanilla `ops.rs:323`). `Value::Ratio` não
        // é produzível por sintaxe de utilizador no cristalino, mas o braço
        // é puro e fecha a tabela de `div` do vanilla.
        (BinOp::Div, Value::Ratio(a), Value::Ratio(b)) => Ok(Value::Float(a.0 / b.0)),
        // P818 — Angle / Int|Float e Angle / Angle (vanilla
        // `ops.rs:317-319`). Alcançável desde P817.
        (BinOp::Div, Value::Angle(a), Value::Int(n)) => Ok(Value::Angle(
            crate::entities::layout_types::Angle::rad(a.to_rad() / n as f64),
        )),
        (BinOp::Div, Value::Angle(a), Value::Float(f)) => {
            Ok(Value::Angle(crate::entities::layout_types::Angle::rad(a.to_rad() / f)))
        }
        (BinOp::Div, Value::Angle(a), Value::Angle(b)) => {
            Ok(Value::Float(a.to_rad() / b.to_rad()))
        }
        // ── Lógica booleana ──────────────────────────────────────────────────
        (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
        (BinOp::Or, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),
        // ── Tipos tipográficos (ADR-0028, ADR-0029) ──────────────────────────
        // Length + Length: sempre válido (abs + abs, em + em, mistos representáveis)
        (BinOp::Add, Value::Length(a), Value::Length(b)) => Ok(Value::Length(a + b)),
        // P841 (#31) — paridade vanilla `foundations/ops.rs:196`.
        (BinOp::Sub, Value::Length(a), Value::Length(b)) => Ok(Value::Length(a - b)),
        // P841 (#36) — paridade vanilla `foundations/ops.rs:194`.
        (BinOp::Sub, Value::Angle(a), Value::Angle(b)) => Ok(Value::Angle(
            crate::entities::layout_types::Angle::rad(a.to_rad() - b.to_rad()),
        )),
        // P841 (#37) — paridade vanilla `foundations/ops.rs:127`.
        (BinOp::Add, Value::Fraction(a), Value::Fraction(b)) => {
            Ok(Value::Fraction(a + b))
        }
        // P492 — Length + Color / Color + Length → Stroke (paridade vanilla stroke syntax).
        (BinOp::Add, Value::Length(l), Value::Color(c))
        | (BinOp::Add, Value::Color(c), Value::Length(l)) => {
            Ok(Value::Stroke(crate::entities::geometry::Stroke {
                paint: crate::entities::paint::Paint::Solid(c),
                thickness: l.abs.to_pt(),
                overhang: false,
                ..crate::entities::geometry::Stroke::default()
            }))
        }
        // P1229 — o shorthand público de stroke aceita qualquer Paint. Esta
        // fatia fecha Gradient nas duas ordens sem colapsar para primeira cor.
        (BinOp::Add, Value::Length(l), Value::Gradient(g))
        | (BinOp::Add, Value::Gradient(g), Value::Length(l)) => {
            Ok(Value::Stroke(crate::entities::geometry::Stroke {
                paint: crate::entities::paint::Paint::Gradient(g),
                thickness: l.abs.to_pt(),
                overhang: false,
                ..crate::entities::geometry::Stroke::default()
            }))
        }
        // Ratio * Int ou Int * Ratio → escala o rácio
        (BinOp::Mul, Value::Ratio(r), Value::Int(n)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() * n as f64)))
        }
        (BinOp::Mul, Value::Int(n), Value::Ratio(r)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(n as f64 * r.get())))
        }
        // ── P842 (#32) — aritmética de Ratio ─────────────────────────────
        // Desde P842 o literal percentual (`50%`) é `Value::Ratio` (paridade
        // vanilla); estes braços cobrem a tabela medida no vanilla
        // (`temp/p842/l1_ratio_arith*.typ`): Ratio ± Ratio → Ratio;
        // Ratio ± Length ↔ Relative; Ratio × Float → Ratio;
        // Ratio × Fraction → Fraction; Ratio / Int|Float → Ratio.
        (BinOp::Add, Value::Ratio(a), Value::Ratio(b)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(a.get() + b.get())))
        }
        (BinOp::Sub, Value::Ratio(a), Value::Ratio(b)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(a.get() - b.get())))
        }
        (BinOp::Add, Value::Ratio(r), Value::Length(l))
        | (BinOp::Add, Value::Length(l), Value::Ratio(r)) => {
            Ok(Value::Relative(crate::entities::rel::Rel { rel: r.get(), abs: l }))
        }
        (BinOp::Sub, Value::Ratio(r), Value::Length(l)) => {
            Ok(Value::Relative(crate::entities::rel::Rel { rel: r.get(), abs: -l }))
        }
        (BinOp::Sub, Value::Length(l), Value::Ratio(r)) => {
            Ok(Value::Relative(crate::entities::rel::Rel { rel: -r.get(), abs: l }))
        }
        (BinOp::Mul, Value::Ratio(r), Value::Float(f))
        | (BinOp::Mul, Value::Float(f), Value::Ratio(r)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() * f)))
        }
        // Medido no vanilla: `100% * 2fr` = `2fr`; type(50% * 2fr) = fraction.
        (BinOp::Mul, Value::Ratio(r), Value::Fraction(f))
        | (BinOp::Mul, Value::Fraction(f), Value::Ratio(r)) => {
            Ok(Value::Fraction(r.get() * f))
        }
        (BinOp::Div, Value::Ratio(r), Value::Int(n)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() / n as f64)))
        }
        (BinOp::Div, Value::Ratio(r), Value::Float(f)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() / f)))
        }
        // P725 — Length * Int|Float (as quatro combinações): escala uniforme
        // sobre `Length: Mul<f64>` (`entities/layout_types.rs:801-806`),
        // mesmo agrupamento do vanilla (`foundations/ops.rs:238-243`).
        // NaN → 0 por componente (paridade do efeito observável de
        // `Scalar::new`, vanilla `typst-utils/src/scalar.rs:30-32` — medido:
        // `repr(1pt * float.nan)` = `0pt`); inf propaga-se silenciosamente
        // (`repr(1pt * float.inf)` = `float.inf * 1pt`).
        (BinOp::Mul, Value::Length(a), Value::Int(b))
        | (BinOp::Mul, Value::Int(b), Value::Length(a)) => {
            Ok(Value::Length(sanitize_length_nan(a * b as f64)))
        }
        (BinOp::Mul, Value::Length(a), Value::Float(b))
        | (BinOp::Mul, Value::Float(b), Value::Length(a)) => {
            Ok(Value::Length(sanitize_length_nan(a * b)))
        }
        // ── Comprimentos relativos (P469) ────────────────────────────────────
        // Relative + Relative, Relative - Relative
        (BinOp::Add, Value::Relative(a), Value::Relative(b)) => {
            Ok(Value::Relative(a + b))
        }
        (BinOp::Sub, Value::Relative(a), Value::Relative(b)) => {
            Ok(Value::Relative(a - b))
        }
        // Relative + Length / Length + Relative
        (BinOp::Add, Value::Relative(r), Value::Length(l))
        | (BinOp::Add, Value::Length(l), Value::Relative(r)) => {
            Ok(Value::Relative(r + l))
        }
        // P492 — Relative + Color / Color + Relative → Stroke (usa parte absoluta).
        (BinOp::Add, Value::Relative(r), Value::Color(c))
        | (BinOp::Add, Value::Color(c), Value::Relative(r)) => {
            Ok(Value::Stroke(crate::entities::geometry::Stroke {
                paint: crate::entities::paint::Paint::Solid(c),
                thickness: r.abs.abs.to_pt(),
                overhang: false,
                ..crate::entities::geometry::Stroke::default()
            }))
        }
        // Relative - Length
        (BinOp::Sub, Value::Relative(r), Value::Length(l)) => Ok(Value::Relative(r - l)),
        // Relative * Int / Int * Relative
        (BinOp::Mul, Value::Relative(r), Value::Int(n))
        | (BinOp::Mul, Value::Int(n), Value::Relative(r)) => {
            Ok(Value::Relative(r * n as f64))
        }
        // Relative * Float / Float * Relative
        (BinOp::Mul, Value::Relative(r), Value::Float(f))
        | (BinOp::Mul, Value::Float(f), Value::Relative(r)) => Ok(Value::Relative(r * f)),
        // Relative / Int
        (BinOp::Div, Value::Relative(r), Value::Int(n)) => {
            Ok(Value::Relative(r / n as f64))
        }
        // Relative / Float
        (BinOp::Div, Value::Relative(r), Value::Float(f)) => Ok(Value::Relative(r / f)),
        // P713 — Length / Int, Length / Float: escala uniforme (já implementado
        // em `Length: Div<f64>`, `entities/layout_types.rs:808-813`), mesmo
        // agrupamento do vanilla (`foundations/ops.rs:312-314`).
        // **P739D** — NaN → 0 por componente (mesmo saneamento de P725 em Mul;
        // medido: `repr(1pt / (calc.inf - calc.inf))` → `0pt` no vanilla, o
        // cristalino propagava `float.nan * 1pt + ...`).
        (BinOp::Div, Value::Length(a), Value::Int(b)) => {
            Ok(Value::Length(sanitize_length_nan(a / b as f64)))
        }
        (BinOp::Div, Value::Length(a), Value::Float(b)) => {
            Ok(Value::Length(sanitize_length_nan(a / b)))
        }
        // P713 — Length / Length: paridade exacta com `Length::try_div` do
        // vanilla (`layout/length.rs:64-72`) — só divide se ambos os `abs`
        // forem zero (rácio de `em`) ou ambos os `em` forem zero (rácio de
        // `abs`); combinação mista (abs+em não-zero de ambos os lados,
        // incomensurável) é erro, não `None` silencioso.
        (BinOp::Div, Value::Length(a), Value::Length(b)) => {
            let result = if a.abs.is_zero() && b.abs.is_zero() {
                Some(a.em / b.em)
            } else if a.em == 0.0 && b.em == 0.0 {
                Some(a.abs.to_pt() / b.abs.to_pt())
            } else {
                None
            };
            result
                .map(Value::Float)
                .ok_or_else(|| "cannot divide these two lengths".to_string())
        }
        // ── Alinhamento (Passo 84.5, encerra DEBT-36) ────────────────────────
        // `center + bottom` → Align2D { h: Center, v: Bottom }.
        // Erro em conflito (semântica vanilla — não sobrescrita silenciosa):
        // dois H, dois V, ou qualquer combinação que tente sobrepor o mesmo
        // eixo retorna `Err`.
        (BinOp::Add, Value::Align(a), Value::Align(b)) => {
            let h_conflict = a.h.is_some() && b.h.is_some();
            let v_conflict = a.v.is_some() && b.v.is_some();
            if h_conflict && v_conflict {
                Err("cannot add two 2D alignments".to_string())
            } else if h_conflict {
                Err("cannot add two horizontal alignments".to_string())
            } else if v_conflict {
                Err("cannot add two vertical alignments".to_string())
            } else {
                Ok(Value::Align(crate::entities::layout_types::Align2D {
                    h: a.h.or(b.h),
                    v: a.v.or(b.v),
                }))
            }
        }
        // ── Fronteira — combinações inválidas ────────────────────────────
        (op, lhs, rhs) => Err(binary_mismatch(op, &lhs, &rhs)),
    }
}

/// **P725** — saneamento NaN → 0 por componente de `Length`: paridade do
/// efeito observável de `Scalar::new` no vanilla
/// (`typst-utils/src/scalar.rs:30-32`), onde `Abs`/`Em` embrulham `Scalar`
/// e toda a aritmética converte NaN em 0.0 (medido: `repr(1pt * float.nan)`
/// = `0pt`, `repr(1em * float.nan)` = `0pt`). Inf não é tocado
/// (`repr(1em * float.inf)` = `float.inf * 1em`). Fica no braço do eval
/// (não em `Length::mul`/`Length::div`) por disciplina um-bug-por-passo.
/// **P739D** — estendido a `Length / Int|Float` (medido alcançável via
/// `calc.inf - calc.inf` → NaN: `repr(1pt / NaN)` → `0pt` no vanilla).
fn sanitize_length_nan(
    l: crate::entities::layout_types::Length,
) -> crate::entities::layout_types::Length {
    fn san(x: f64) -> f64 {
        if x.is_nan() {
            0.0
        } else {
            x
        }
    }
    crate::entities::layout_types::Length {
        abs: crate::entities::layout_types::Abs(san(l.abs.to_pt())),
        em: san(l.em),
    }
}

/// Avalia uma operação unária com semântica Typst.
///
/// Int negation usa `checked_neg` para retornar Err em overflow
/// (mesma política do original).
pub(crate) fn eval_unary_op(op: UnOp, operand: Value) -> Result<Value, String> {
    match (op, operand) {
        (UnOp::Neg, Value::Int(i)) => {
            Ok(Value::Int(i.checked_neg().ok_or("number too large")?))
        }
        (UnOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
        // P404 — negação Decimal.
        (UnOp::Neg, Value::Decimal(d)) => Ok(Value::Decimal(Decimal(-d.0))),
        (UnOp::Neg, Value::Length(l)) => {
            use crate::entities::layout_types::{Abs, Length};
            Ok(Value::Length(Length { abs: Abs(-l.abs.to_pt()), em: -l.em }))
        }
        (UnOp::Neg, Value::Relative(r)) => Ok(Value::Relative(-r)),
        // P832 (achado #59) — paridade vanilla `foundations/ops.rs:80-84`.
        (UnOp::Neg, Value::Angle(a)) => {
            Ok(Value::Angle(crate::entities::layout_types::Angle::rad(-a.to_rad())))
        }
        (UnOp::Neg, Value::Ratio(r)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(-r.get())))
        }
        (UnOp::Neg, Value::Fraction(f)) => Ok(Value::Fraction(-f)),
        // P850 — `Duration` com sinal.
        (UnOp::Neg, Value::Duration(d)) => Ok(Value::Duration(-d)),
        (UnOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (UnOp::Pos, Value::Int(i)) => Ok(Value::Int(i)),
        (UnOp::Pos, Value::Float(f)) => Ok(Value::Float(f)),
        (UnOp::Pos, Value::Decimal(d)) => Ok(Value::Decimal(d)),
        (UnOp::Pos, Value::Length(l)) => Ok(Value::Length(l)),
        (UnOp::Pos, Value::Angle(a)) => Ok(Value::Angle(a)),
        (UnOp::Pos, Value::Ratio(r)) => Ok(Value::Ratio(r)),
        (UnOp::Pos, Value::Relative(r)) => Ok(Value::Relative(r)),
        (UnOp::Pos, Value::Fraction(f)) => Ok(Value::Fraction(f)),
        (op, operand) => Err(unary_mismatch(op, &operand)),
    }
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::super::eval_binary_op;
    use super::*;
    use crate::entities::layout_types::{Angle, Ratio};

    /// **P832/P850 (achado #59)** — braços `Neg` em paridade com o vanilla
    /// (`foundations/ops.rs:80-84`: Angle/Ratio/Relative/Fraction/Duration).
    #[test]
    fn neg_angle() {
        match eval_unary_op(UnOp::Neg, Value::Angle(Angle::deg(15.0))) {
            Ok(Value::Angle(a)) => assert!((a.to_deg() + 15.0).abs() < 1e-9),
            other => panic!("esperado Ok(Angle(-15deg)), obteve {other:?}"),
        }
    }

    #[test]
    fn neg_ratio() {
        match eval_unary_op(UnOp::Neg, Value::Ratio(Ratio::from_percent(50.0))) {
            Ok(Value::Ratio(r)) => assert!((r.to_percent() + 50.0).abs() < 1e-9),
            other => panic!("esperado Ok(Ratio(-50%)), obteve {other:?}"),
        }
    }

    #[test]
    fn neg_fraction() {
        match eval_unary_op(UnOp::Neg, Value::Fraction(1.0)) {
            Ok(Value::Fraction(f)) => assert_eq!(f, -1.0),
            other => panic!("esperado Ok(Fraction(-1)), obteve {other:?}"),
        }
    }

    #[test]
    fn neg_duration() {
        use crate::entities::duration::Duration;
        match eval_unary_op(UnOp::Neg, Value::Duration(Duration::from_seconds(3))) {
            Ok(Value::Duration(d)) => assert_eq!(d.nanos, -3_000_000_000),
            other => panic!("esperado Ok(Duration(-3s)), obteve {other:?}"),
        }
    }

    #[test]
    fn p1218_pos_completa_tipos_vanilla() {
        use crate::entities::layout_types::Length;
        use crate::entities::rel::Rel;
        let values = [
            Value::Decimal(Decimal::new(15, 1)),
            Value::Angle(Angle::deg(30.0)),
            Value::Ratio(Ratio::from_percent(20.0)),
            Value::Relative(Rel::from_percent(20.0) + Length::pt(1.0)),
            Value::Fraction(1.0),
        ];
        for value in values {
            assert_eq!(eval_unary_op(UnOp::Pos, value.clone()), Ok(value));
        }
    }

    #[test]
    fn p1218_fronteiras_unarias_verbatim() {
        use crate::entities::world_types::Datetime;
        assert_eq!(
            eval_unary_op(UnOp::Pos, Value::Bool(true)).unwrap_err(),
            "cannot apply '+' to boolean"
        );
        assert_eq!(
            eval_unary_op(UnOp::Pos, Value::Str("x".into())).unwrap_err(),
            "cannot apply unary '+' to string"
        );
        assert_eq!(
            eval_unary_op(UnOp::Neg, Value::Bool(true)).unwrap_err(),
            "cannot apply '-' to boolean"
        );
        assert_eq!(
            eval_unary_op(
                UnOp::Neg,
                Value::Datetime(Datetime::new_date(2020, 1, 1).unwrap())
            )
            .unwrap_err(),
            "cannot apply unary '-' to datetime"
        );
        assert_eq!(
            eval_unary_op(UnOp::Not, Value::Int(1)).unwrap_err(),
            "cannot apply 'not' to integer"
        );
    }

    #[test]
    fn p1219_decimal_int_oito_direcoes_exatas() {
        let d = |s: &str| Value::Decimal(Decimal::from_str(s).unwrap());
        let cases = [
            (BinOp::Add, d("1.5"), Value::Int(2), d("3.5")),
            (BinOp::Add, Value::Int(2), d("1.5"), d("3.5")),
            (BinOp::Sub, d("5.5"), Value::Int(2), d("3.5")),
            (BinOp::Sub, Value::Int(2), d("5.5"), d("-3.5")),
            (BinOp::Mul, d("1.5"), Value::Int(3), d("4.5")),
            (BinOp::Mul, Value::Int(3), d("1.5"), d("4.5")),
            (BinOp::Div, d("7.5"), Value::Int(2), d("3.750")),
            (BinOp::Div, Value::Int(2), d("0.5"), d("4.0")),
        ];
        for (op, lhs, rhs, expected) in cases {
            assert_eq!(eval_binary_op(op, lhs, rhs), Ok(expected));
        }
    }

    #[test]
    fn p1219_decimal_int_precisao_zero_e_overflow() {
        let d = |s: &str| Value::Decimal(Decimal::from_str(s).unwrap());
        assert_eq!(
            eval_binary_op(BinOp::Add, d("1"), Value::Int(9_007_199_254_740_993)),
            Ok(d("9007199254740994"))
        );
        assert_eq!(
            eval_binary_op(BinOp::Div, d("1"), Value::Int(0)).unwrap_err(),
            "cannot divide by zero"
        );
        assert_eq!(
            eval_binary_op(BinOp::Div, Value::Int(1), d("0.00")).unwrap_err(),
            "cannot divide by zero"
        );
        assert_eq!(
            eval_binary_op(BinOp::Add, d("79228162514264337593543950335"), Value::Int(1))
                .unwrap_err(),
            "value is too large"
        );
    }

    /// **P841 (achados #31/#36/#37)** — braços aritméticos em falta,
    /// paridade vanilla `foundations/ops.rs:127,194,196`.
    #[test]
    fn p841_sub_length() {
        // 2em - 5em = -3em; 10pt - 3pt = 7pt (componentes abs/em separadas).
        use crate::entities::layout_types::{Abs, Length};
        let a = Value::Length(Length { abs: Abs(0.0), em: 2.0 });
        let b = Value::Length(Length { abs: Abs(0.0), em: 5.0 });
        match eval_binary_op(BinOp::Sub, a, b) {
            Ok(Value::Length(l)) => {
                assert_eq!(l.em, -3.0);
                assert_eq!(l.abs.to_pt(), 0.0);
            }
            other => panic!("esperado Ok(Length(-3em)), obteve {other:?}"),
        }
        let a = Value::Length(Length { abs: Abs(10.0), em: 0.0 });
        let b = Value::Length(Length { abs: Abs(3.0), em: 0.0 });
        match eval_binary_op(BinOp::Sub, a, b) {
            Ok(Value::Length(l)) => assert_eq!(l.abs.to_pt(), 7.0),
            other => panic!("esperado Ok(Length(7pt)), obteve {other:?}"),
        }
    }

    #[test]
    fn p841_sub_angle() {
        // 90deg - 45deg = 45deg.
        let a = Value::Angle(Angle::deg(90.0));
        let b = Value::Angle(Angle::deg(45.0));
        match eval_binary_op(BinOp::Sub, a, b) {
            Ok(Value::Angle(r)) => assert!((r.to_deg() - 45.0).abs() < 1e-9),
            other => panic!("esperado Ok(Angle(45deg)), obteve {other:?}"),
        }
    }

    #[test]
    fn p841_add_fraction() {
        // 1fr + 2fr = 3fr.
        match eval_binary_op(BinOp::Add, Value::Fraction(1.0), Value::Fraction(2.0)) {
            Ok(Value::Fraction(f)) => assert_eq!(f, 3.0),
            other => panic!("esperado Ok(Fraction(3)), obteve {other:?}"),
        }
    }

    /// **P900** — causa real do "crash 2" de P894: `"⟨" + x + "|"` dentro de
    /// uma função de modo math (`#let bra(x) = "⟨" + x + "|"`, `x` vinculado
    /// a `Content`) falhava com `cannot add string and content` — o operador
    /// `+` (`eval_binary_op`, `BinOp::Add`) não tinha braços para `Str`↔
    /// `Content` (nem para `Symbol`↔`Str`/`Symbol`↔`Content`/`Symbol`↔
    /// `Symbol`), ao contrário do `join()` (P728, `Content::sequence`) já
    /// existente neste mesmo ficheiro, que já cobria exactamente estas
    /// combinações. Paridade vanilla confirmada por leitura directa
    /// (`lab/typst-original/crates/typst-library/src/foundations/ops.rs::add`,
    /// linhas 129-138) — vanilla coerce `Str`/`Symbol` para `Content`
    /// (`TextElem::packed`/`SymbolElem::packed`) e soma como `Content + Content`;
    /// `Symbol + Symbol`/`Symbol + Str`/`Str + Symbol` produzem `Str`.
    /// Cristalino não tem `TextElem`/`SymbolElem` (arquitectura mais simples);
    /// usa `Content::sequence`/`Content::text` directamente, mesma semântica
    /// observável, mecânica diferente por propósito (ADR-0107).
    #[test]
    fn p900_add_str_content_symbol_combinacoes() {
        use crate::entities::symbol::Symbol;

        let sym_a = || Value::Symbol(Symbol::new('⟨', "sym_a"));
        let sym_b = || Value::Symbol(Symbol::new('|', "sym_b"));

        // Caso mínimo exacto de P894/P900: Str + Content.
        match eval_binary_op(
            BinOp::Add,
            Value::Str("⟨".into()),
            Value::Content(Content::text("x")),
        ) {
            Ok(Value::Content(c)) => {
                assert_eq!(c.plain_text(), "⟨x", "Str + Content deve preservar ordem")
            }
            other => panic!("esperado Ok(Content), obteve {other:?}"),
        }

        // Ordem inversa: Content + Str.
        match eval_binary_op(
            BinOp::Add,
            Value::Content(Content::text("x")),
            Value::Str("|".into()),
        ) {
            Ok(Value::Content(c)) => {
                assert_eq!(c.plain_text(), "x|", "Content + Str deve preservar ordem")
            }
            other => panic!("esperado Ok(Content), obteve {other:?}"),
        }

        // Symbol + Symbol → Str (paridade vanilla: format_str!("{a}{b}")).
        match eval_binary_op(BinOp::Add, sym_a(), sym_b()) {
            Ok(Value::Str(s)) => assert_eq!(s.as_str(), "⟨|"),
            other => panic!("esperado Ok(Str), obteve {other:?}"),
        }

        // Str + Symbol e Symbol + Str → Str.
        match eval_binary_op(BinOp::Add, Value::Str("x".into()), sym_b()) {
            Ok(Value::Str(s)) => assert_eq!(s.as_str(), "x|"),
            other => panic!("esperado Ok(Str), obteve {other:?}"),
        }
        match eval_binary_op(BinOp::Add, sym_a(), Value::Str("x".into())) {
            Ok(Value::Str(s)) => assert_eq!(s.as_str(), "⟨x"),
            other => panic!("esperado Ok(Str), obteve {other:?}"),
        }

        // Content + Symbol e Symbol + Content → Content.
        match eval_binary_op(BinOp::Add, Value::Content(Content::text("x")), sym_b()) {
            Ok(Value::Content(c)) => assert_eq!(c.plain_text(), "x|"),
            other => panic!("esperado Ok(Content), obteve {other:?}"),
        }
        match eval_binary_op(BinOp::Add, sym_a(), Value::Content(Content::text("x"))) {
            Ok(Value::Content(c)) => assert_eq!(c.plain_text(), "⟨x"),
            other => panic!("esperado Ok(Content), obteve {other:?}"),
        }
    }

    // ── P1043: Pares de independência testcase() para arithmetic.rs:40 ──────

    use crate::entities::layout_types::Length;
    use crate::entities::rel::Rel;

    #[test]
    fn p1043_arithmetic_div_relative_both_zero_isolada() {
        // C1=T, C2=T: r.abs.is_zero() && r.rel == 0.0 -> erro de divisão por zero
        let rel_zero = Value::Relative(Rel { rel: 0.0, abs: Length::ZERO });
        let res = apply_binary(BinOp::Div, Value::Int(10), rel_zero);
        assert_eq!(res.unwrap_err(), "cannot divide by zero");
    }

    #[test]
    fn p1043_arithmetic_div_relative_abs_zero_rel_nonzero_isolada() {
        // C1=T, C2=F: r.abs.is_zero() && r.rel != 0.0 -> não cai no erro de divisão por zero
        let rel_non_zero = Value::Relative(Rel { rel: 0.5, abs: Length::ZERO });
        let res = apply_binary(BinOp::Div, Value::Int(10), rel_non_zero);
        assert!(res.is_err());
        assert_ne!(res.unwrap_err(), "cannot divide by zero");
    }

    #[test]
    fn p1043_arithmetic_div_relative_abs_nonzero_rel_zero_isolada() {
        // C1=F, C2=_: !r.abs.is_zero() && r.rel == 0.0 -> não cai no erro de divisão por zero
        let rel_abs_non_zero = Value::Relative(Rel { rel: 0.0, abs: Length::pt(10.0) });
        let res = apply_binary(BinOp::Div, Value::Int(10), rel_abs_non_zero);
        assert!(res.is_err());
        assert_ne!(res.unwrap_err(), "cannot divide by zero");
    }
}
