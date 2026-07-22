//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval/ops.md
//! @prompt-hash a6f9df18
//! @layer L1
//! @updated 2026-06-25
//!
//! Operadores binários e unários do eval. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).
//! P818: ordenação str/bool/array/length/relative/ratio/angle (`value_cmp`),
//! div `Relative/Relative` + `Ratio/Ratio`, `Str * Int`, eq `Length↔Relative`,
//! coerção Int↔Float recursiva (`values_eq`), ops de `Angle` (achado #5 de
//! P810).

use crate::entities::ast::expr::{BinOp, UnOp};
use crate::entities::bytes::Bytes;
use crate::entities::content::Content;
use crate::entities::decimal::Decimal;
use crate::entities::duration::Duration;
use crate::entities::value::Value;

/// Avalia uma operação binária com semântica Typst.
///
/// Semântica confirmada com `lab/typst-original/crates/typst-library/src/foundations/ops.rs`:
/// - Int/Int → Float (não truncamento): `5/2 = 2.5`
/// - Int overflow → Err (checked_add/sub/mul/neg, como no original)
/// - Float: IEEE 754 propagado silenciosamente (sem guarda NaN/Inf)
/// - Divisão por zero → Err explícito
/// - `Int == Float` — ADR-0025 Opção B: coerção em eval_binary_op,
///   derive(PartialEq) mantido para Rust
pub(crate) fn eval_binary_op(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    // Divisão por zero — verificar antes do match (como no original)
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
            Value::Relative(r) if r.abs.is_zero() && r.rel == 0.0 => {
                return Err("cannot divide by zero".into())
            }
            Value::Ratio(r) if r.0 == 0.0 => {
                return Err("cannot divide by zero".into())
            }
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
        // P405 — adição Duration homogénea.
        (BinOp::Add, Value::Duration(a), Value::Duration(b)) => {
            let sum = a.nanos as u128 + b.nanos as u128;
            if sum > u64::MAX as u128 {
                return Err("overflow em duration + duration".into());
            }
            Ok(Value::Duration(Duration::from_nanos(sum as u64)))
        }
        (BinOp::Add, Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + b.as_str())),
        (BinOp::Add, Value::Content(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![a, b])))
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
        // P405 — subtracção Duration homogénea (rejeita negativo).
        (BinOp::Sub, Value::Duration(a), Value::Duration(b)) => {
            if a.nanos < b.nanos {
                return Err(
                    "underflow em duration - duration (resultado negativo)".into()
                );
            }
            Ok(Value::Duration(Duration::from_nanos(a.nanos - b.nanos)))
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
        // P405 — multiplicação Duration por Int/Float.
        (BinOp::Mul, Value::Duration(d), Value::Int(n))
        | (BinOp::Mul, Value::Int(n), Value::Duration(d)) => {
            if n < 0 {
                return Err("duration * int negativo não suportado".into());
            }
            let prod = d.nanos as u128 * n as u128;
            if prod > u64::MAX as u128 {
                return Err("overflow em duration * int".into());
            }
            Ok(Value::Duration(Duration::from_nanos(prod as u64)))
        }
        (BinOp::Mul, Value::Duration(d), Value::Float(f))
        | (BinOp::Mul, Value::Float(f), Value::Duration(d)) => {
            if f < 0.0 {
                return Err("duration * float negativo não suportado".into());
            }
            Ok(Value::Duration(Duration::from_nanos((d.nanos as f64 * f) as u64)))
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
        | (BinOp::Mul, Value::Int(n), Value::Angle(a)) => {
            Ok(Value::Angle(crate::entities::layout_types::Angle::rad(
                a.to_rad() * n as f64,
            )))
        }
        (BinOp::Mul, Value::Angle(a), Value::Float(f))
        | (BinOp::Mul, Value::Float(f), Value::Angle(a)) => {
            Ok(Value::Angle(crate::entities::layout_types::Angle::rad(
                a.to_rad() * f,
            )))
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
        // P405 — divisão Duration por Int/Float/Duration.
        (BinOp::Div, Value::Duration(d), Value::Int(n)) => {
            if n == 0 {
                return Err("divisão por zero".into());
            }
            if n < 0 {
                return Err("duration / int negativo não suportado".into());
            }
            Ok(Value::Duration(Duration::from_nanos(d.nanos / n as u64)))
        }
        (BinOp::Div, Value::Duration(d), Value::Float(f)) => {
            if f == 0.0 {
                return Err("divisão por zero".into());
            }
            if f < 0.0 {
                return Err("duration / float negativo não suportado".into());
            }
            Ok(Value::Duration(Duration::from_nanos((d.nanos as f64 / f) as u64)))
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
        (BinOp::Div, Value::Angle(a), Value::Int(n)) => {
            Ok(Value::Angle(crate::entities::layout_types::Angle::rad(
                a.to_rad() / n as f64,
            )))
        }
        (BinOp::Div, Value::Angle(a), Value::Float(f)) => {
            Ok(Value::Angle(crate::entities::layout_types::Angle::rad(
                a.to_rad() / f,
            )))
        }
        (BinOp::Div, Value::Angle(a), Value::Angle(b)) => {
            Ok(Value::Float(a.to_rad() / b.to_rad()))
        }

        // ── Comparações ──────────────────────────────────────────────────────
        // ADR-0025: coerção Int↔Float em Eq/Neq e ordenação, como no original.
        // derive(PartialEq) mantido para IndexMap, testes Rust, e estruturas de dados —
        // mas eval_binary_op replica a semântica do Typst (1 == 1.0 → true).
        (BinOp::Eq, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) == b)),
        (BinOp::Eq, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a == (b as f64))),
        (BinOp::Neq, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) != b)),
        (BinOp::Neq, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a != (b as f64))),
        // P345 (ADR-0107): o `==` da linguagem sobre conteúdo é **morfológico** —
        // compara texto/markup/estilo semântico (`*bold*`) e **ignora** o estilo de
        // render (o `TextStyle` assado, o transporte β1, o numbering assado da chain)
        // via `Content::morph_canon`. Fecha o Achado 2 (`it.body == [a]` casa) na
        // camada da linguagem. **Não** toca o `derive(PartialEq)` do Rust — dois
        // sistemas (ADR-0025): a forma canônica é comparada com o `==` estrutural.
        (BinOp::Eq, Value::Content(a), Value::Content(b)) => {
            Ok(Value::Bool(a.morph_canon() == b.morph_canon()))
        }
        (BinOp::Neq, Value::Content(a), Value::Content(b)) => {
            Ok(Value::Bool(a.morph_canon() != b.morph_canon()))
        }
        // P684 — comparação Version directa sobre todos os componentes (zero-pad),
        // sem qualquer tratamento especial de `pre`/`build` (não existem no Typst).
        (BinOp::Eq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a != b)),
        // P785b — Comparação Ratio ↔ Relative (com abs zero, ex: 50% == (10pt + 50%).ratio)
        (BinOp::Eq, Value::Ratio(r), Value::Relative(rel))
        | (BinOp::Eq, Value::Relative(rel), Value::Ratio(r)) => {
            Ok(Value::Bool(rel.abs.is_zero() && (rel.rel - r.0).abs() < 1e-9))
        }
        (BinOp::Neq, Value::Ratio(r), Value::Relative(rel))
        | (BinOp::Neq, Value::Relative(rel), Value::Ratio(r)) => {
            Ok(Value::Bool(!rel.abs.is_zero() || (rel.rel - r.0).abs() >= 1e-9))
        }
        (BinOp::Eq, a, b) => Ok(Value::Bool(values_eq(&a, &b))),
        (BinOp::Neq, a, b) => Ok(Value::Bool(!values_eq(&a, &b))),
        // Ordenação: coerção Int↔Float confirmada no original (ops::compare)
        (BinOp::Lt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Lt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Lt, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
        (BinOp::Lt, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a < (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Lt, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 < b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Lt, Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a < b)),
        // P684 — ordenação Version lexicográfica zero-pad sobre os componentes.
        (BinOp::Lt, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Leq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) <= b)),
        (BinOp::Leq, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a <= (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Leq, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 <= b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Leq, Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a <= b)),
        // P406 — ordenação Version homogénea.
        (BinOp::Leq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Gt, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Gt, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Gt, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
        (BinOp::Gt, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a > (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Gt, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 > b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Gt, Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a > b)),
        // P406 — ordenação Version homogénea.
        (BinOp::Gt, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Geq, Value::Int(a), Value::Int(b)) => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Int(a), Value::Float(b)) => Ok(Value::Bool((a as f64) >= b)),
        (BinOp::Geq, Value::Float(a), Value::Int(b)) => Ok(Value::Bool(a >= (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Geq, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 >= b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Geq, Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a >= b)),
        // P406 — ordenação Version homogénea.
        (BinOp::Geq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a >= b)),

        // ── P818 — ordenação str/bool/array/length/relative/ratio/angle ────
        // Paridade vanilla `ops::compare` (`foundations/ops.rs:471-500`):
        // `Str` lexicográfica, `Bool` (false < true), `Array` lexicográfica
        // recursiva (`try_cmp_arrays`), `Length`/`Relative` (comparáveis
        // quando medidos na mesma componente), guards `Length ↔ Relative`
        // com parte relativa zero, `Ratio`, `Angle`. Pares incomparáveis
        // caem no mesmo erro de fronteira de sempre (paridade do observável
        // "é erro" — o vanilla também rejeita, `mismatch!("cannot compare…")`).
        (op @ (BinOp::Lt | BinOp::Leq | BinOp::Gt | BinOp::Geq), a, b) => {
            match value_cmp(&a, &b) {
                Some(ord) => Ok(Value::Bool(match op {
                    BinOp::Lt => ord == std::cmp::Ordering::Less,
                    BinOp::Leq => ord != std::cmp::Ordering::Greater,
                    BinOp::Gt => ord == std::cmp::Ordering::Greater,
                    BinOp::Geq => ord != std::cmp::Ordering::Less,
                    _ => unreachable!(),
                })),
                None => Err(format!(
                    "cannot apply {:?} to {} and {}",
                    op,
                    a.type_name(),
                    b.type_name()
                )),
            }
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
            }))
        }
        // Ratio * Int ou Int * Ratio → escala o rácio
        (BinOp::Mul, Value::Ratio(r), Value::Int(n)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() * n as f64)))
        }
        (BinOp::Mul, Value::Int(n), Value::Ratio(r)) => {
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(n as f64 * r.get())))
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

        // ── P706 — `in` / `not in` ────────────────────────────────────────────
        // `Str in Dict` testa se a chave existe; `Str in Str` testa substring;
        // `any in Array` testa igualdade de elemento (mesma coerção Int/Float
        // do `==`, `value_eq` abaixo — cobre arrays aninhados e tipos mistos
        // sem código extra, via `PartialEq` recursivo de `Value`).
        (BinOp::In, Value::Str(s), Value::Dict(d)) => {
            Ok(Value::Bool(d.contains_key(s.as_str())))
        }
        (BinOp::NotIn, Value::Str(s), Value::Dict(d)) => {
            Ok(Value::Bool(!d.contains_key(s.as_str())))
        }
        (BinOp::In, Value::Str(needle), Value::Str(haystack)) => {
            Ok(Value::Bool(haystack.as_str().contains(needle.as_str())))
        }
        (BinOp::NotIn, Value::Str(needle), Value::Str(haystack)) => {
            Ok(Value::Bool(!haystack.as_str().contains(needle.as_str())))
        }
        (BinOp::In, needle, Value::Array(arr)) => {
            Ok(Value::Bool(arr.iter().any(|item| value_eq(&needle, item))))
        }
        (BinOp::NotIn, needle, Value::Array(arr)) => {
            Ok(Value::Bool(!arr.iter().any(|item| value_eq(&needle, item))))
        }

        // ── Fronteira — tipos não migrados ou combinações inválidas ──────────
        (op, lhs, rhs) => Err(format!(
            "cannot apply {:?} to {} and {}",
            op,
            lhs.type_name(),
            rhs.type_name()
        )),
    }
}

/// **P728** — `join` de valores produzidos pelas expressões de um code
/// block (paridade vanilla `ops::join`, `foundations/ops.rs:24-45`).
/// `None` é identidade nos dois lados; texto/content/array/dict/args/bytes
/// concatenam ou fazem merge; qualquer outra combinação é erro de tipo
/// (`cannot join X with Y`, medido: `{ 1; 2 }` erra no vanilla).
pub(crate) fn join(lhs: Value, rhs: Value) -> Result<Value, String> {
    match (lhs, rhs) {
        (a, Value::None) => Ok(a),
        (Value::None, b) => Ok(b),
        (Value::Str(a), Value::Str(b)) => Ok(Value::Str(a + b.as_str())),
        (Value::Symbol(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a.ch, b.ch).into()))
        }
        (Value::Str(a), Value::Symbol(b)) => {
            Ok(Value::Str(format!("{}{}", a, b.ch).into()))
        }
        (Value::Symbol(a), Value::Str(b)) => {
            Ok(Value::Str(format!("{}{}", a.ch, b).into()))
        }
        (Value::Bytes(a), Value::Bytes(b)) => {
            let mut v = a.as_slice().to_vec();
            v.extend_from_slice(b.as_slice());
            Ok(Value::Bytes(Bytes::new(v)))
        }
        (Value::Content(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![a, b])))
        }
        (Value::Content(a), Value::Str(b)) => {
            Ok(Value::Content(Content::sequence(vec![a, Content::text(b)])))
        }
        (Value::Str(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![Content::text(a), b])))
        }
        (Value::Content(a), Value::Symbol(b)) => {
            Ok(Value::Content(Content::sequence(vec![
                a,
                Content::text(b.ch.to_string()),
            ])))
        }
        (Value::Symbol(a), Value::Content(b)) => {
            Ok(Value::Content(Content::sequence(vec![
                Content::text(a.ch.to_string()),
                b,
            ])))
        }
        (Value::Array(mut a), Value::Array(b)) => {
            a.extend(b);
            Ok(Value::Array(a))
        }
        (Value::Dict(mut a), Value::Dict(b)) => {
            a.extend(b);
            Ok(Value::Dict(a))
        }
        (Value::Args(mut a), Value::Args(b)) => {
            a.items.extend(b.items);
            a.named.extend(b.named);
            Ok(Value::Args(a))
        }
        (a, b) => Err(format!("cannot join {} with {}", a.type_name(), b.type_name())),
    }
}

/// **P706** — igualdade de valor usada por `in`/`not in` sobre `Array`.
/// Mesma coerção Int/Float do `BinOp::Eq` (medido: `1 in (1.0, 2.0)` → `true`
/// no vanilla); tudo o resto delega ao `PartialEq` derivado de `Value`
/// (recursivo — cobre array-de-arrays sem código extra).
/// **P818-h** — delega em [`values_eq`]: a coerção passa a propagar-se a
/// elementos aninhados (medido: `(1,) in ((1.0,), (2,))` → `true` no vanilla).
fn value_eq(a: &Value, b: &Value) -> bool {
    values_eq(a, b)
}

/// **P818** — igualdade da linguagem com coerção `Int ↔ Float` **recursiva**
/// (paridade vanilla: `Value::eq` é `ops::equal`, que coage, e
/// `Array`/`Dict` comparam elemento a elemento com ela — medido:
/// `(1,2) == (1.0,2.0)` → `true`, `(a: 1) == (a: 1.0)` → `true`).
/// Cobre ainda as igualdades mistas `Length ↔ Relative` (rel zero,
/// `ops.rs:458-460` — P818-f) e `Ratio ↔ Relative` (abs zero, mesma
/// tolerância do braço dedicado P785b) em posição aninhada, e `Content`
/// morfológico (P345) aninhado. O resto delega no `PartialEq` derivado.
fn values_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(i), Value::Float(f)) | (Value::Float(f), Value::Int(i)) => {
            (*i as f64) == *f
        }
        (Value::Array(x), Value::Array(y)) => {
            x.len() == y.len() && x.iter().zip(y.iter()).all(|(u, v)| values_eq(u, v))
        }
        (Value::Dict(x), Value::Dict(y)) => {
            x.len() == y.len()
                && x.iter().all(|(k, v)| y.get(k).is_some_and(|w| values_eq(v, w)))
        }
        (Value::Length(l), Value::Relative(r))
        | (Value::Relative(r), Value::Length(l)) => l == &r.abs && r.rel == 0.0,
        (Value::Ratio(rat), Value::Relative(rel))
        | (Value::Relative(rel), Value::Ratio(rat)) => {
            rel.abs.is_zero() && (rel.rel - rat.0).abs() < 1e-9
        }
        (Value::Content(x), Value::Content(y)) => x.morph_canon() == y.morph_canon(),
        (a, b) => a == b,
    }
}

/// **P818** — comparação da linguagem (paridade vanilla `ops::compare`,
/// `foundations/ops.rs:471-500`). `None` = par incomparável (o chamador
/// emite o erro de fronteira, que é também o observável do vanilla).
fn value_cmp(a: &Value, b: &Value) -> Option<std::cmp::Ordering> {
    match (a, b) {
        (Value::Bool(x), Value::Bool(y)) => Some(x.cmp(y)),
        (Value::Int(x), Value::Int(y)) => Some(x.cmp(y)),
        (Value::Float(x), Value::Float(y)) => x.partial_cmp(y),
        (Value::Int(x), Value::Float(y)) => (*x as f64).partial_cmp(y),
        (Value::Float(x), Value::Int(y)) => x.partial_cmp(&(*y as f64)),
        (Value::Decimal(x), Value::Decimal(y)) => Some(x.0.cmp(&y.0)),
        (Value::Str(x), Value::Str(y)) => Some(x.as_str().cmp(y.as_str())),
        (Value::Version(x), Value::Version(y)) => x.partial_cmp(y),
        (Value::Duration(x), Value::Duration(y)) => x.partial_cmp(y),
        (Value::Angle(x), Value::Angle(y)) => x.to_rad().partial_cmp(&y.to_rad()),
        (Value::Ratio(x), Value::Ratio(y)) => x.0.partial_cmp(&y.0),
        (Value::Length(x), Value::Length(y)) => length_partial_cmp(x, y),
        (Value::Relative(x), Value::Relative(y)) => rel_partial_cmp(x, y),
        // Guards do vanilla (`ops.rs:491-494`): Length ↔ Relative só é
        // comparável quando a parte relativa do `Relative` é zero.
        (Value::Length(l), Value::Relative(r)) if r.rel == 0.0 => {
            length_partial_cmp(l, &r.abs)
        }
        (Value::Relative(r), Value::Length(l)) if r.rel == 0.0 => {
            length_partial_cmp(&r.abs, l)
        }
        (Value::Array(x), Value::Array(y)) => cmp_arrays(x, y),
        _ => None,
    }
}

/// **P818-b** — comparação lexicográfica de arrays (paridade
/// `try_cmp_arrays`, vanilla `foundations/ops.rs:514-530`): elemento a
/// elemento com a comparação completa; prefixo igual → o mais curto é menor.
fn cmp_arrays(x: &[Value], y: &[Value]) -> Option<std::cmp::Ordering> {
    for (u, v) in x.iter().zip(y.iter()) {
        match value_cmp(u, v) {
            Some(std::cmp::Ordering::Equal) => continue,
            other => return other,
        }
    }
    Some(x.len().cmp(&y.len()))
}

/// **P818-g** — ordem parcial de `Length` (paridade
/// `layout/length.rs:195-204`): comparável só quando medida numa única
/// componente (ambos `em` zero → rácio de `abs`; ambos `abs` zero → `em`).
fn length_partial_cmp(
    a: &crate::entities::layout_types::Length,
    b: &crate::entities::layout_types::Length,
) -> Option<std::cmp::Ordering> {
    if a.em == 0.0 && b.em == 0.0 {
        a.abs.to_pt().partial_cmp(&b.abs.to_pt())
    } else if a.abs.is_zero() && b.abs.is_zero() {
        a.em.partial_cmp(&b.em)
    } else {
        None
    }
}

/// **P818-g** — ordem parcial de `Rel<Length>` (paridade
/// `layout/rel.rs:193-202`): `rel` ambos zero → compara `abs`;
/// `abs` ambos zero → compara `rel`; misto → incomparável.
fn rel_partial_cmp(
    a: &crate::entities::rel::Rel<crate::entities::layout_types::Length>,
    b: &crate::entities::rel::Rel<crate::entities::layout_types::Length>,
) -> Option<std::cmp::Ordering> {
    if a.rel == 0.0 && b.rel == 0.0 {
        length_partial_cmp(&a.abs, &b.abs)
    } else if a.abs.is_zero() && b.abs.is_zero() {
        a.rel.partial_cmp(&b.rel)
    } else {
        None
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
        (UnOp::Not, Value::Bool(b)) => Ok(Value::Bool(!b)),
        (UnOp::Pos, Value::Int(i)) => Ok(Value::Int(i)),
        (UnOp::Pos, Value::Float(f)) => Ok(Value::Float(f)),
        (UnOp::Pos, Value::Length(l)) => Ok(Value::Length(l)),
        (op, operand) => Err(format!("cannot apply {:?} to {}", op, operand.type_name())),
    }
}


// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::layout_types::{Angle, Ratio};

    /// **P832 (achado #59)** — braços `Neg` em paridade com o vanilla
    /// (`foundations/ops.rs:80-84`: Angle/Ratio/Relative/Fraction/Duration).
    /// `Duration` fica de fora de propósito: a representação cristalina é
    /// `u64` de nanossegundos (sem sinal) — ver relatório de P832.
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
}
