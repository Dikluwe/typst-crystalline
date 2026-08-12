//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval/operators.md
//! @prompt-hash 65936d4d
//! @layer L1
//! @updated 2026-08-12

//! Operadores binários e unários do eval. Extraído de `eval.rs` no Passo 96.1
//! conforme ADR-0037 (coesão por domínio).
//! P818: ordenação str/bool/array/length/relative/ratio/angle (`value_cmp`),
//! div `Relative/Relative` + `Ratio/Ratio`, `Str * Int`, eq `Length↔Relative`,
//! coerção Int↔Float recursiva (`values_eq`), ops de `Angle` (achado #5 de
//! P810).
//!
//! Passo 1002: fatiado em hub + 5 nós (arithmetic, equality, ordering,
//! error_formatting, join) — este ficheiro é só a tabela de despacho.

pub(crate) mod arithmetic;
pub(crate) mod equality;
pub(crate) mod error_formatting;
pub(crate) mod join;
pub(crate) mod ordering;

pub(crate) use arithmetic::eval_unary_op;
pub(crate) use join::join;

use crate::entities::ast::expr::BinOp;
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
    match op {
        BinOp::Add | BinOp::Sub | BinOp::Mul | BinOp::Div | BinOp::And | BinOp::Or => {
            arithmetic::apply_binary(op, lhs, rhs)
        }
        BinOp::Eq | BinOp::Neq | BinOp::In | BinOp::NotIn => equality::apply_binary(op, lhs, rhs),
        BinOp::Lt | BinOp::Leq | BinOp::Gt | BinOp::Geq => ordering::apply_binary(op, lhs, rhs),
        other => Err(error_formatting::binary_mismatch(other, &lhs, &rhs)),
    }
}
