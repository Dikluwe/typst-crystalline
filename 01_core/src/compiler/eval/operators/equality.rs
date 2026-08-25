//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/operators/equality.md
//! @prompt-hash 746effb8
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! Igualdade da linguagem (`==`/`!=`, com coerção Int↔Float recursiva e
//! `Content` morfológico) e pertença (`in`/`not in`). A fronteira de erro é
//! do nó `error_formatting`.

use crate::entities::ast::expr::BinOp;
use crate::entities::value::Value;

use super::error_formatting::binary_mismatch;

/// Avalia `==`/`!=`/`in`/`not in` com a semântica da linguagem (ADR-0025:
/// distinta do `PartialEq` derivado, que fica para o Rust).
pub(crate) fn apply_binary(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    match (op, lhs, rhs) {
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
        } // ── Fronteira ───────────────────────────────────────────────────
        (op, lhs, rhs) => Err(binary_mismatch(op, &lhs, &rhs)),
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
        (Value::Content(x), Value::LocatedContent(y, _))
        | (Value::LocatedContent(x, _), Value::Content(y))
        | (Value::LocatedContent(x, _), Value::LocatedContent(y, _)) => {
            x.morph_canon() == y.morph_canon()
        }
        (a, b) => a == b,
    }
}
