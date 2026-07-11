//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval/ops.md
//! @prompt-hash 146bda0e
//! @layer L1
//! @updated 2026-06-25
//!
//! Operadores binários e unários do eval. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use crate::entities::ast::expr::{BinOp, UnOp};
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
            Value::Int(0)   => return Err("cannot divide by zero".into()),
            Value::Float(f) if *f == 0.0 => return Err("cannot divide by zero".into()),
            Value::Decimal(d) if d.0.is_zero() => return Err("cannot divide by zero".into()),
            _ => {}
        }
    }

    match (op, lhs, rhs) {
        // ── Adição ──────────────────────────────────────────────────────────
        (BinOp::Add, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_add(b).ok_or("number too large")?)),
        (BinOp::Add, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a + b)),
        (BinOp::Add, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a + b as f64)),
        (BinOp::Add, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 + b)),
        // P404 — aritmética Decimal homogénea.
        (BinOp::Add, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(Decimal(a.0 + b.0))),
        // P405 — adição Duration homogénea.
        (BinOp::Add, Value::Duration(a), Value::Duration(b)) => {
            let sum = a.nanos as u128 + b.nanos as u128;
            if sum > u64::MAX as u128 {
                return Err("overflow em duration + duration".into());
            }
            Ok(Value::Duration(Duration::from_nanos(sum as u64)))
        },
        (BinOp::Add, Value::Str(a),   Value::Str(b))   => Ok(Value::Str(a + b.as_str())),
        (BinOp::Add, Value::Content(a), Value::Content(b)) =>
            Ok(Value::Content(Content::sequence(vec![a, b]))),

        // ── Subtracção ──────────────────────────────────────────────────────
        (BinOp::Sub, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_sub(b).ok_or("number too large")?)),
        (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (BinOp::Sub, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a - b as f64)),
        (BinOp::Sub, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 - b)),
        // P404 — aritmética Decimal homogénea.
        (BinOp::Sub, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(Decimal(a.0 - b.0))),
        // P405 — subtracção Duration homogénea (rejeita negativo).
        (BinOp::Sub, Value::Duration(a), Value::Duration(b)) => {
            if a.nanos < b.nanos {
                return Err("underflow em duration - duration (resultado negativo)".into());
            }
            Ok(Value::Duration(Duration::from_nanos(a.nanos - b.nanos)))
        },

        // ── Multiplicação ────────────────────────────────────────────────────
        (BinOp::Mul, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_mul(b).ok_or("number too large")?)),
        (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        (BinOp::Mul, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a * b as f64)),
        (BinOp::Mul, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 * b)),
        // P404 — aritmética Decimal homogénea.
        (BinOp::Mul, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(Decimal(a.0 * b.0))),
        // P405 — multiplicação Duration por Int/Float.
        (BinOp::Mul, Value::Duration(d), Value::Int(n)) | (BinOp::Mul, Value::Int(n), Value::Duration(d)) => {
            if n < 0 {
                return Err("duration * int negativo não suportado".into());
            }
            let prod = d.nanos as u128 * n as u128;
            if prod > u64::MAX as u128 {
                return Err("overflow em duration * int".into());
            }
            Ok(Value::Duration(Duration::from_nanos(prod as u64)))
        }
        (BinOp::Mul, Value::Duration(d), Value::Float(f)) | (BinOp::Mul, Value::Float(f), Value::Duration(d)) => {
            if f < 0.0 {
                return Err("duration * float negativo não suportado".into());
            }
            Ok(Value::Duration(Duration::from_nanos((d.nanos as f64 * f) as u64)))
        },

        // ── Divisão — Int/Int → Float (semântica Typst, não truncamento) ────
        (BinOp::Div, Value::Int(a),   Value::Int(b))   => Ok(Value::Float(a as f64 / b as f64)),
        (BinOp::Div, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
        (BinOp::Div, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a / b as f64)),
        (BinOp::Div, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 / b)),
        // P404 — divisão Decimal homogénea (div/0 já verificado acima).
        (BinOp::Div, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Decimal(Decimal(a.0 / b.0))),
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
        },

        // ── Comparações ──────────────────────────────────────────────────────
        // ADR-0025: coerção Int↔Float em Eq/Neq e ordenação, como no original.
        // derive(PartialEq) mantido para IndexMap, testes Rust, e estruturas de dados —
        // mas eval_binary_op replica a semântica do Typst (1 == 1.0 → true).
        (BinOp::Eq,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) == b)),
        (BinOp::Eq,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a == (b as f64))),
        (BinOp::Neq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) != b)),
        (BinOp::Neq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a != (b as f64))),
        // P345 (ADR-0107): o `==` da linguagem sobre conteúdo é **morfológico** —
        // compara texto/markup/estilo semântico (`*bold*`) e **ignora** o estilo de
        // render (o `TextStyle` assado, o transporte β1, o numbering assado da chain)
        // via `Content::morph_canon`. Fecha o Achado 2 (`it.body == [a]` casa) na
        // camada da linguagem. **Não** toca o `derive(PartialEq)` do Rust — dois
        // sistemas (ADR-0025): a forma canônica é comparada com o `==` estrutural.
        (BinOp::Eq,  Value::Content(a), Value::Content(b)) =>
            Ok(Value::Bool(a.morph_canon() == b.morph_canon())),
        (BinOp::Neq, Value::Content(a), Value::Content(b)) =>
            Ok(Value::Bool(a.morph_canon() != b.morph_canon())),
        // P684 — comparação Version directa sobre todos os componentes (zero-pad),
        // sem qualquer tratamento especial de `pre`/`build` (não existem no Typst).
        (BinOp::Eq,  Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a != b)),
        (BinOp::Eq,  a, b) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, a, b) => Ok(Value::Bool(a != b)),
        // Ordenação: coerção Int↔Float confirmada no original (ops::compare)
        (BinOp::Lt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a < b)),
        (BinOp::Lt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Lt,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
        (BinOp::Lt,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a < (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Lt,  Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 < b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Lt,  Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a < b)),
        // P684 — ordenação Version lexicográfica zero-pad sobre os componentes.
        (BinOp::Lt,  Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Leq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) <= b)),
        (BinOp::Leq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a <= (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Leq, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 <= b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Leq, Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a <= b)),
        // P406 — ordenação Version homogénea.
        (BinOp::Leq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Gt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a > b)),
        (BinOp::Gt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Gt,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
        (BinOp::Gt,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a > (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Gt,  Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 > b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Gt,  Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a > b)),
        // P406 — ordenação Version homogénea.
        (BinOp::Gt,  Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Geq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) >= b)),
        (BinOp::Geq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a >= (b as f64))),
        // P404 — ordenação Decimal homogénea.
        (BinOp::Geq, Value::Decimal(a), Value::Decimal(b)) => Ok(Value::Bool(a.0 >= b.0)),
        // P405 — ordenação Duration homogénea.
        (BinOp::Geq, Value::Duration(a), Value::Duration(b)) => Ok(Value::Bool(a >= b)),
        // P406 — ordenação Version homogénea.
        (BinOp::Geq, Value::Version(a), Value::Version(b)) => Ok(Value::Bool(a >= b)),

        // ── Lógica booleana ──────────────────────────────────────────────────
        (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
        (BinOp::Or,  Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),

        // ── Tipos tipográficos (ADR-0028, ADR-0029) ──────────────────────────
        // Length + Length: sempre válido (abs + abs, em + em, mistos representáveis)
        (BinOp::Add, Value::Length(a), Value::Length(b)) =>
            Ok(Value::Length(a + b)),
        // P492 — Length + Color / Color + Length → Stroke (paridade vanilla stroke syntax).
        (BinOp::Add, Value::Length(l), Value::Color(c)) |
        (BinOp::Add, Value::Color(c), Value::Length(l)) =>
            Ok(Value::Stroke(crate::entities::geometry::Stroke {
                paint: crate::entities::paint::Paint::Solid(c),
                thickness: l.abs.to_pt(),
                overhang: false,
            })),
        // Ratio * Int ou Int * Ratio → escala o rácio
        (BinOp::Mul, Value::Ratio(r), Value::Int(n)) =>
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() * n as f64))),
        (BinOp::Mul, Value::Int(n), Value::Ratio(r)) =>
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(n as f64 * r.get()))),

        // ── Comprimentos relativos (P469) ────────────────────────────────────
        // Relative + Relative, Relative - Relative
        (BinOp::Add, Value::Relative(a), Value::Relative(b)) =>
            Ok(Value::Relative(a + b)),
        (BinOp::Sub, Value::Relative(a), Value::Relative(b)) =>
            Ok(Value::Relative(a - b)),
        // Relative + Length / Length + Relative
        (BinOp::Add, Value::Relative(r), Value::Length(l)) |
        (BinOp::Add, Value::Length(l), Value::Relative(r)) =>
            Ok(Value::Relative(r + l)),
        // P492 — Relative + Color / Color + Relative → Stroke (usa parte absoluta).
        (BinOp::Add, Value::Relative(r), Value::Color(c)) |
        (BinOp::Add, Value::Color(c), Value::Relative(r)) =>
            Ok(Value::Stroke(crate::entities::geometry::Stroke {
                paint: crate::entities::paint::Paint::Solid(c),
                thickness: r.abs.abs.to_pt(),
                overhang: false,
            })),
        // Relative - Length
        (BinOp::Sub, Value::Relative(r), Value::Length(l)) =>
            Ok(Value::Relative(r - l)),
        // Relative * Int / Int * Relative
        (BinOp::Mul, Value::Relative(r), Value::Int(n)) |
        (BinOp::Mul, Value::Int(n), Value::Relative(r)) =>
            Ok(Value::Relative(r * n as f64)),
        // Relative * Float / Float * Relative
        (BinOp::Mul, Value::Relative(r), Value::Float(f)) |
        (BinOp::Mul, Value::Float(f), Value::Relative(r)) =>
            Ok(Value::Relative(r * f)),
        // Relative / Int
        (BinOp::Div, Value::Relative(r), Value::Int(n)) =>
            Ok(Value::Relative(r / n as f64)),
        // Relative / Float
        (BinOp::Div, Value::Relative(r), Value::Float(f)) =>
            Ok(Value::Relative(r / f)),

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
        (BinOp::In, Value::Str(s), Value::Dict(d)) =>
            Ok(Value::Bool(d.contains_key(s.as_str()))),
        (BinOp::NotIn, Value::Str(s), Value::Dict(d)) =>
            Ok(Value::Bool(!d.contains_key(s.as_str()))),
        (BinOp::In, Value::Str(needle), Value::Str(haystack)) =>
            Ok(Value::Bool(haystack.as_str().contains(needle.as_str()))),
        (BinOp::NotIn, Value::Str(needle), Value::Str(haystack)) =>
            Ok(Value::Bool(!haystack.as_str().contains(needle.as_str()))),
        (BinOp::In, needle, Value::Array(arr)) =>
            Ok(Value::Bool(arr.iter().any(|item| value_eq(&needle, item)))),
        (BinOp::NotIn, needle, Value::Array(arr)) =>
            Ok(Value::Bool(!arr.iter().any(|item| value_eq(&needle, item)))),

        // ── Fronteira — tipos não migrados ou combinações inválidas ──────────
        (op, lhs, rhs) => Err(format!(
            "cannot apply {:?} to {} and {}",
            op, lhs.type_name(), rhs.type_name()
        )),
    }
}

/// **P706** — igualdade de valor usada por `in`/`not in` sobre `Array`.
/// Mesma coerção Int/Float do `BinOp::Eq` (medido: `1 in (1.0, 2.0)` → `true`
/// no vanilla); tudo o resto delega ao `PartialEq` derivado de `Value`
/// (recursivo — cobre array-de-arrays sem código extra).
fn value_eq(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Int(a), Value::Float(b)) => (*a as f64) == *b,
        (Value::Float(a), Value::Int(b)) => *a == (*b as f64),
        (a, b) => a == b,
    }
}

/// Avalia uma operação unária com semântica Typst.
///
/// Int negation usa `checked_neg` para retornar Err em overflow
/// (mesma política do original).
pub(crate) fn eval_unary_op(op: UnOp, operand: Value) -> Result<Value, String> {
    match (op, operand) {
        (UnOp::Neg, Value::Int(i))   =>
            Ok(Value::Int(i.checked_neg().ok_or("number too large")?)),
        (UnOp::Neg, Value::Float(f)) => Ok(Value::Float(-f)),
        // P404 — negação Decimal.
        (UnOp::Neg, Value::Decimal(d)) => Ok(Value::Decimal(Decimal(-d.0))),
        (UnOp::Neg, Value::Length(l)) => {
            use crate::entities::layout_types::{Abs, Length};
            Ok(Value::Length(Length { abs: Abs(-l.abs.to_pt()), em: -l.em }))
        }
        (UnOp::Neg, Value::Relative(r)) => Ok(Value::Relative(-r)),
        (UnOp::Not, Value::Bool(b))  => Ok(Value::Bool(!b)),
        (UnOp::Pos, Value::Int(i))   => Ok(Value::Int(i)),
        (UnOp::Pos, Value::Float(f)) => Ok(Value::Float(f)),
        (UnOp::Pos, Value::Length(l)) => Ok(Value::Length(l)),
        (op, operand) => Err(format!(
            "cannot apply {:?} to {}",
            op, operand.type_name()
        )),
    }
}
