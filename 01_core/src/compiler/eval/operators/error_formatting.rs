//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/operators/error_formatting.md
//! @prompt-hash 7cf373d3
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! Mensagens de fronteira do dispatcher de operadores — formatos verbatim
//! do vanilla com os nomes longos de tipo (a mensagem é o observável).

use crate::entities::ast::expr::BinOp;
use crate::entities::value::Value;

/// **P842 (#39)** — mensagem de fronteira binária no formato verbatim do
/// vanilla (`ops::add/sub/mul/div`, `foundations/ops.rs:170,214,284,340`).
/// `Sub` inverte a ordem dos operandos ("cannot subtract {rhs} from {lhs}").
pub(crate) fn binary_mismatch(op: BinOp, lhs: &Value, rhs: &Value) -> String {
    let (a, b) = (vanilla_type_name(lhs), vanilla_type_name(rhs));
    match op {
        BinOp::Add => format!("cannot add {a} and {b}"),
        BinOp::Sub => format!("cannot subtract {b} from {a}"),
        BinOp::Mul => format!("cannot multiply {a} with {b}"),
        BinOp::Div => format!("cannot divide {a} by {b}"),
        _ => format!("cannot apply {op:?} to {a} and {b}"),
    }
}

/// **P842 (#39)** — nome longo do tipo como nas mensagens do vanilla
/// (`long_name` de cada `#[ty]`: `integer`, `boolean`, `string`,
/// `relative length`, …). Distinto de `Value::type_name()` (nomes curtos
/// do `type()`/`repr`).
pub(crate) fn vanilla_type_name(v: &Value) -> &'static str {
    match v {
        Value::None => "none",
        Value::Auto => "auto",
        Value::Bool(_) => "boolean",
        Value::Int(_) => "integer",
        Value::Float(_) => "float",
        Value::Str(_) => "string",
        Value::Array(_) => "array",
        Value::Dict(_) => "dictionary",
        Value::Module(_) => "module",
        Value::Datetime(_) => "datetime",
        Value::Func(_) => "function",
        Value::Content(_) => "content",
        Value::Length(_) => "length",
        Value::Relative(_) => "relative length",
        Value::Ratio(_) => "ratio",
        Value::Angle(_) => "angle",
        Value::Color(_) => "color",
        Value::Stroke(_) => "stroke",
        Value::Fraction(_) => "fraction",
        Value::Align(_) => "alignment",
        Value::Location(_) => "location",
        Value::Gradient(_) => "gradient",
        Value::Regex(_) => "regex",
        Value::Tiling(_) => "tiling",
        Value::Bytes(_) => "bytes",
        Value::Decimal(_) => "decimal",
        Value::Duration(_) => "duration",
        Value::Version(_) => "version",
        Value::Selector(_) => "selector",
        Value::Symbol(_) => "symbol",
        Value::Args(_) => "arguments",
        Value::State(_) => "state",
        Value::Counter(_) => "counter",
        Value::Label(_) => "label",
        Value::Dir(_) => "direction",
        Value::Type(_) => "type",
    }
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::eval_binary_op;

    /// **P842 (achado #39 de P831)** — fronteira genérica com os formatos
    /// verbatim do vanilla (`foundations/ops.rs:170,214,284,340,500`) e os
    /// nomes longos de tipo (`integer`, `direction`, `relative length`, …).
    /// Medido nos dois binários (`temp/p842/l8_probe_*.typ`).
    #[test]
    fn p842_l8_fronteira_mensagens_vanilla() {
        let dir = || Value::Dir(crate::entities::dir::Dir::LTR);
        // O caso do achado: `2 * ltr`.
        assert_eq!(
            eval_binary_op(BinOp::Mul, Value::Int(2), dir()).unwrap_err(),
            "cannot multiply integer with direction"
        );
        // Ordem invertida preservada: `ltr * 2`.
        assert_eq!(
            eval_binary_op(BinOp::Mul, dir(), Value::Int(2)).unwrap_err(),
            "cannot multiply direction with integer"
        );
        // Add: "cannot add {a} and {b}".
        assert_eq!(
            eval_binary_op(
                BinOp::Add,
                Value::Length(crate::entities::layout_types::Length::em(1.0)),
                dir()
            )
            .unwrap_err(),
            "cannot add length and direction"
        );
        // Sub: "cannot subtract {b} from {a}" (ordem invertida no vanilla).
        assert_eq!(
            eval_binary_op(
                BinOp::Sub,
                Value::Length(crate::entities::layout_types::Length::pt(1.0)),
                dir()
            )
            .unwrap_err(),
            "cannot subtract direction from length"
        );
        // Div: "cannot divide {a} by {b}".
        assert_eq!(
            eval_binary_op(BinOp::Div, Value::Int(1), dir()).unwrap_err(),
            "cannot divide integer by direction"
        );
        // Comparação: "cannot compare {a} and {b}".
        assert_eq!(
            eval_binary_op(BinOp::Lt, dir(), Value::Int(2)).unwrap_err(),
            "cannot compare direction and integer"
        );
        // Nomes longos: bool→boolean, str→string, relative→relative length.
        assert_eq!(
            eval_binary_op(BinOp::Add, Value::Bool(true), Value::Int(1)).unwrap_err(),
            "cannot add boolean and integer"
        );
        assert_eq!(
            eval_binary_op(
                BinOp::Add,
                Value::Relative(crate::entities::rel::Rel::from_percent(50.0)
                    + crate::entities::layout_types::Length::pt(1.0)),
                dir()
            )
            .unwrap_err(),
            "cannot add relative length and direction"
        );
    }
}
