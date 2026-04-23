//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/eval.md
//! @prompt-hash 19073424
//! @layer L1
//! @updated 2026-04-22
//!
//! Operadores binários e unários do eval. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).

use crate::entities::ast::expr::{BinOp, UnOp};
use crate::entities::content::Content;
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
        (BinOp::Add, Value::Str(a),   Value::Str(b))   => Ok(Value::Str(a + b.as_str())),
        (BinOp::Add, Value::Content(a), Value::Content(b)) =>
            Ok(Value::Content(Content::sequence(vec![a, b]))),

        // ── Subtracção ──────────────────────────────────────────────────────
        (BinOp::Sub, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_sub(b).ok_or("number too large")?)),
        (BinOp::Sub, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a - b)),
        (BinOp::Sub, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a - b as f64)),
        (BinOp::Sub, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 - b)),

        // ── Multiplicação ────────────────────────────────────────────────────
        (BinOp::Mul, Value::Int(a),   Value::Int(b))   =>
            Ok(Value::Int(a.checked_mul(b).ok_or("number too large")?)),
        (BinOp::Mul, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a * b)),
        (BinOp::Mul, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a * b as f64)),
        (BinOp::Mul, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 * b)),

        // ── Divisão — Int/Int → Float (semântica Typst, não truncamento) ────
        (BinOp::Div, Value::Int(a),   Value::Int(b))   => Ok(Value::Float(a as f64 / b as f64)),
        (BinOp::Div, Value::Float(a), Value::Float(b)) => Ok(Value::Float(a / b)),
        (BinOp::Div, Value::Float(a), Value::Int(b))   => Ok(Value::Float(a / b as f64)),
        (BinOp::Div, Value::Int(a),   Value::Float(b)) => Ok(Value::Float(a as f64 / b)),

        // ── Comparações ──────────────────────────────────────────────────────
        // ADR-0025: coerção Int↔Float em Eq/Neq e ordenação, como no original.
        // derive(PartialEq) mantido para IndexMap, testes Rust, e estruturas de dados —
        // mas eval_binary_op replica a semântica do Typst (1 == 1.0 → true).
        (BinOp::Eq,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) == b)),
        (BinOp::Eq,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a == (b as f64))),
        (BinOp::Neq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) != b)),
        (BinOp::Neq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a != (b as f64))),
        (BinOp::Eq,  a, b) => Ok(Value::Bool(a == b)),
        (BinOp::Neq, a, b) => Ok(Value::Bool(a != b)),
        // Ordenação: coerção Int↔Float confirmada no original (ops::compare)
        (BinOp::Lt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a < b)),
        (BinOp::Lt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a < b)),
        (BinOp::Lt,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) < b)),
        (BinOp::Lt,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a < (b as f64))),
        (BinOp::Leq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a <= b)),
        (BinOp::Leq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) <= b)),
        (BinOp::Leq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a <= (b as f64))),
        (BinOp::Gt,  Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a > b)),
        (BinOp::Gt,  Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a > b)),
        (BinOp::Gt,  Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) > b)),
        (BinOp::Gt,  Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a > (b as f64))),
        (BinOp::Geq, Value::Int(a),   Value::Int(b))   => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Float(a), Value::Float(b)) => Ok(Value::Bool(a >= b)),
        (BinOp::Geq, Value::Int(a),   Value::Float(b)) => Ok(Value::Bool((a as f64) >= b)),
        (BinOp::Geq, Value::Float(a), Value::Int(b))   => Ok(Value::Bool(a >= (b as f64))),

        // ── Lógica booleana ──────────────────────────────────────────────────
        (BinOp::And, Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a && b)),
        (BinOp::Or,  Value::Bool(a), Value::Bool(b)) => Ok(Value::Bool(a || b)),

        // ── Tipos tipográficos (ADR-0028, ADR-0029) ──────────────────────────
        // Length + Length: sempre válido (abs + abs, em + em, mistos representáveis)
        (BinOp::Add, Value::Length(a), Value::Length(b)) =>
            Ok(Value::Length(a + b)),
        // Ratio * Int ou Int * Ratio → escala o rácio
        (BinOp::Mul, Value::Ratio(r), Value::Int(n)) =>
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(r.get() * n as f64))),
        (BinOp::Mul, Value::Int(n), Value::Ratio(r)) =>
            Ok(Value::Ratio(crate::entities::layout_types::Ratio(n as f64 * r.get()))),

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

        // ── Fronteira — tipos não migrados ou combinações inválidas ──────────
        (op, lhs, rhs) => Err(format!(
            "cannot apply {:?} to {} and {}",
            op, lhs.type_name(), rhs.type_name()
        )),
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
        (UnOp::Neg, Value::Length(l)) => {
            use crate::entities::layout_types::{Abs, Length};
            Ok(Value::Length(Length { abs: Abs(-l.abs.to_pt()), em: -l.em }))
        }
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
