//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/eval/operators/ordering.md
//! @prompt-hash 1fee0969
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! Ordenação da linguagem (`<` `<=` `>` `>=`) — paridade `ops::compare`
//! (`foundations/ops.rs:471-500`). Pares incomparáveis erram com o formato
//! verbatim do nó `error_formatting`.

use crate::entities::ast::expr::BinOp;
use crate::entities::value::Value;

use super::error_formatting::{binary_mismatch, vanilla_type_name};

/// Avalia `<`/`<=`/`>`/`>=`: braços específicos primeiro, braço combinado
/// via `value_cmp` depois; `None` → erro de fronteira "cannot compare".
pub(crate) fn apply_binary(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    match (op, lhs, rhs) {
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
                    // P842 (#39) — formato verbatim do vanilla
                    // (`mismatch!("cannot compare {} and {}", …)`,
                    // `foundations/ops.rs:500`), nomes longos de tipo.
                    "cannot compare {} and {}",
                    vanilla_type_name(&a),
                    vanilla_type_name(&b)
                )),
            }
        }
        // ── Fronteira defensiva — o dispatcher (mod.rs) só envia os 4 ops de ordenação ──
        (op, a, b) => Err(binary_mismatch(op, &a, &b)),
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
