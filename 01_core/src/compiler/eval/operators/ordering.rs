//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/operators/ordering.md
//! @prompt-hash 66db679f
//! @layer L1
//! @updated 2026-08-12
//!
//!
//! Ordenação da linguagem (`<` `<=` `>` `>=`) — paridade `ops::compare`
//! (`foundations/ops.rs:471-500`). Pares incomparáveis erram com o formato
//! verbatim do nó `error_formatting`.

use crate::entities::ast::expr::BinOp;
use crate::entities::value::Value;

use super::super::repr::repr_value;
use super::error_formatting::{binary_mismatch, vanilla_type_name};

enum CompareFailure {
    Values(String, String),
    Kinds(&'static str, &'static str),
    Types,
}

/// Avalia `<`/`<=`/`>`/`>=` pela ordenação única da linguagem.
pub(crate) fn apply_binary(op: BinOp, lhs: Value, rhs: Value) -> Result<Value, String> {
    use std::cmp::Ordering;
    let holds: fn(Ordering) -> bool = match op {
        BinOp::Lt => Ordering::is_lt,
        BinOp::Leq => Ordering::is_le,
        BinOp::Gt => Ordering::is_gt,
        BinOp::Geq => Ordering::is_ge,
        // Fronteira defensiva: o dispatcher só envia os 4 ops de ordenação.
        other => return Err(binary_mismatch(other, &lhs, &rhs)),
    };
    compare(&lhs, &rhs).map(|ordering| Value::Bool(holds(ordering)))
}

/// Ordem de dois valores da linguagem; ordem indefinida é erro.
pub(crate) fn compare(lhs: &Value, rhs: &Value) -> Result<std::cmp::Ordering, String> {
    value_cmp(lhs, rhs).map_err(|failure| match failure {
        CompareFailure::Values(a, b) => format!("cannot compare {a} with {b}"),
        CompareFailure::Kinds(a, b) => format!("cannot compare {a} and {b}"),
        CompareFailure::Types => format!(
            "cannot compare {} and {}",
            vanilla_type_name(lhs),
            vanilla_type_name(rhs)
        ),
    })
}

/// **P818** — comparação da linguagem (paridade vanilla `ops::compare`,
/// `foundations/ops.rs:471-500`). A falha distingue família reconhecida com
/// ordem parcial indefinida de combinação de tipos sem braço comparável.
fn value_cmp(a: &Value, b: &Value) -> Result<std::cmp::Ordering, CompareFailure> {
    match (a, b) {
        (Value::Bool(x), Value::Bool(y)) => Ok(x.cmp(y)),
        (Value::Int(x), Value::Int(y)) => Ok(x.cmp(y)),
        (Value::Float(x), Value::Float(y)) => partial_or_values(a, b, x.partial_cmp(y)),
        (Value::Int(x), Value::Float(y)) => {
            partial_or_values(&int_as_float(*x), b, (*x as f64).partial_cmp(y))
        }
        (Value::Float(x), Value::Int(y)) => {
            partial_or_values(a, &int_as_float(*y), x.partial_cmp(&(*y as f64)))
        }
        (Value::Decimal(x), Value::Decimal(y)) => Ok(x.0.cmp(&y.0)),
        (Value::Int(x), Value::Decimal(y)) => {
            Ok(crate::entities::decimal::Decimal::from_i64(*x).cmp(y))
        }
        (Value::Decimal(x), Value::Int(y)) => {
            Ok(x.cmp(&crate::entities::decimal::Decimal::from_i64(*y)))
        }
        (Value::Str(x), Value::Str(y)) => Ok(x.as_str().cmp(y.as_str())),
        (Value::Version(x), Value::Version(y)) => {
            partial_or_values(a, b, x.partial_cmp(y))
        }
        (Value::Duration(x), Value::Duration(y)) => {
            partial_or_values(a, b, x.partial_cmp(y))
        }
        (Value::Angle(x), Value::Angle(y)) => {
            partial_or_values(a, b, x.to_rad().partial_cmp(&y.to_rad()))
        }
        (Value::Ratio(x), Value::Ratio(y)) => {
            partial_or_values(a, b, x.0.partial_cmp(&y.0))
        }
        (Value::Ratio(x), Value::Relative(y)) if y.abs.is_zero() => {
            partial_or_values(a, b, x.0.partial_cmp(&y.rel))
        }
        (Value::Relative(x), Value::Ratio(y)) if x.abs.is_zero() => {
            partial_or_values(a, b, x.rel.partial_cmp(&y.0))
        }
        (Value::Fraction(x), Value::Fraction(y)) => {
            partial_or_values(a, b, x.partial_cmp(y))
        }
        (Value::Datetime(x), Value::Datetime(y)) => datetime_cmp(x, y),
        (Value::Length(x), Value::Length(y)) => {
            partial_or_values(a, b, length_partial_cmp(x, y))
        }
        (Value::Relative(x), Value::Relative(y)) => {
            partial_or_values(a, b, rel_partial_cmp(x, y))
        }
        // Guards do vanilla (`ops.rs:491-494`): Length ↔ Relative só é
        // comparável quando a parte relativa do `Relative` é zero.
        (Value::Length(l), Value::Relative(r)) if r.rel == 0.0 => {
            partial_or_values(a, &Value::Length(r.abs), length_partial_cmp(l, &r.abs))
        }
        (Value::Relative(r), Value::Length(l)) if r.rel == 0.0 => {
            partial_or_values(&Value::Length(r.abs), b, length_partial_cmp(&r.abs, l))
        }
        (Value::Array(x), Value::Array(y)) => cmp_arrays(x, y),
        _ => Err(CompareFailure::Types),
    }
}

fn datetime_kind(value: &crate::entities::world_types::Datetime) -> &'static str {
    match (value.year().is_some(), value.hour().is_some()) {
        (true, true) => "datetime",
        (true, false) => "date",
        (false, true) => "time",
        (false, false) => unreachable!("Datetime sempre possui data ou hora"),
    }
}

fn datetime_cmp(
    a: &crate::entities::world_types::Datetime,
    b: &crate::entities::world_types::Datetime,
) -> Result<std::cmp::Ordering, CompareFailure> {
    let a_kind = datetime_kind(a);
    let b_kind = datetime_kind(b);
    if a_kind != b_kind {
        return Err(CompareFailure::Kinds(a_kind, b_kind));
    }

    Ok(match a_kind {
        "date" => (a.year(), a.month(), a.day()).cmp(&(b.year(), b.month(), b.day())),
        "time" => {
            (a.hour(), a.minute(), a.second()).cmp(&(b.hour(), b.minute(), b.second()))
        }
        "datetime" => (a.year(), a.month(), a.day(), a.hour(), a.minute(), a.second())
            .cmp(&(b.year(), b.month(), b.day(), b.hour(), b.minute(), b.second())),
        _ => unreachable!(),
    })
}

/// O texto do erro usa float só quando a conversão é exacta.
fn int_as_float(i: i64) -> Value {
    let float = i as f64;
    if float as i128 == i128::from(i) {
        Value::Float(float)
    } else {
        Value::Int(i)
    }
}

fn partial_or_values(
    a: &Value,
    b: &Value,
    ordering: Option<std::cmp::Ordering>,
) -> Result<std::cmp::Ordering, CompareFailure> {
    ordering.ok_or_else(|| CompareFailure::Values(repr_value(a), repr_value(b)))
}

/// **P818-b** — comparação lexicográfica de arrays (paridade
/// `try_cmp_arrays`, vanilla `foundations/ops.rs:514-530`): elemento a
/// elemento com a comparação completa; prefixo igual → o mais curto é menor.
fn cmp_arrays(x: &[Value], y: &[Value]) -> Result<std::cmp::Ordering, CompareFailure> {
    for (u, v) in x.iter().zip(y.iter()) {
        match value_cmp(u, v) {
            Ok(std::cmp::Ordering::Equal) => continue,
            other => return other,
        }
    }
    Ok(x.len().cmp(&y.len()))
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::decimal::Decimal;

    #[test]
    fn nan_is_unordered_in_every_operator() {
        for op in [BinOp::Lt, BinOp::Leq, BinOp::Gt, BinOp::Geq] {
            for (a, b) in [
                (Value::Float(f64::NAN), Value::Float(1.0)),
                (Value::Int(1), Value::Float(f64::NAN)),
                (Value::Float(f64::NAN), Value::Int(1)),
                (Value::Float(f64::NAN), Value::Float(f64::NAN)),
            ] {
                assert!(apply_binary(op, a, b).is_err());
            }
        }
    }
    use crate::entities::layout_types::{Length, Ratio};
    use crate::entities::rel::Rel;
    use crate::entities::world_types::Datetime;

    #[test]
    fn p1216_incomparabilidade_preserva_valores_e_operadores() {
        let cases = [BinOp::Lt, BinOp::Leq, BinOp::Gt, BinOp::Geq];
        for op in cases {
            assert_eq!(
                apply_binary(
                    op,
                    Value::Length(Length::pt(1.0)),
                    Value::Length(Length::em(1.0))
                )
                .unwrap_err(),
                "cannot compare 1pt with 1em"
            );
        }

        assert_eq!(
            apply_binary(
                BinOp::Lt,
                Value::Length(Length::em(1.0)),
                Value::Length(Length::pt(1.0))
            )
            .unwrap_err(),
            "cannot compare 1em with 1pt"
        );
    }

    #[test]
    fn p1216_array_propaga_primeira_causa_incomparavel() {
        assert_eq!(
            apply_binary(
                BinOp::Lt,
                Value::Array(vec![Value::Length(Length::pt(1.0))]),
                Value::Array(vec![Value::Length(Length::em(1.0))])
            )
            .unwrap_err(),
            "cannot compare 1pt with 1em"
        );
    }

    #[test]
    fn p1216_tipos_distintos_e_controles_validos() {
        assert_eq!(
            apply_binary(BinOp::Lt, Value::Int(1), Value::Str("1".into())).unwrap_err(),
            "cannot compare integer and string"
        );
        assert_eq!(
            apply_binary(
                BinOp::Lt,
                Value::Length(Length::pt(1.0)),
                Value::Length(Length::pt(2.0))
            ),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(
                BinOp::Lt,
                Value::Array(vec![Value::Int(1), Value::Int(2)]),
                Value::Array(vec![Value::Int(1), Value::Int(3)])
            ),
            Ok(Value::Bool(true))
        );
    }

    fn assert_all_ordering_ops(lower: Value, higher: Value) {
        assert_eq!(
            apply_binary(BinOp::Lt, lower.clone(), higher.clone()),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(BinOp::Leq, lower.clone(), lower.clone()),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(BinOp::Gt, higher.clone(), lower.clone()),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(BinOp::Geq, higher.clone(), higher),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn p1217_fraction_e_decimal_int_sao_ordenaveis() {
        assert_all_ordering_ops(Value::Fraction(1.0), Value::Fraction(2.0));
        assert_all_ordering_ops(Value::Decimal(Decimal::from_i64(1)), Value::Int(2));
        assert_all_ordering_ops(Value::Int(1), Value::Decimal(Decimal::from_i64(2)));
        assert_eq!(
            apply_binary(BinOp::Lt, Value::Decimal(Decimal::new(15, 1)), Value::Int(2)),
            Ok(Value::Bool(true))
        );
    }

    #[test]
    fn p1217_ratio_relative_respeita_guard_e_direcao() {
        let pure = || Value::Relative(Rel::from_percent(20.0) + Length::pt(0.0));
        assert_all_ordering_ops(Value::Ratio(Ratio::from_percent(10.0)), pure());
        assert_eq!(
            apply_binary(BinOp::Gt, pure(), Value::Ratio(Ratio::from_percent(10.0))),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(
                BinOp::Lt,
                Value::Ratio(Ratio::from_percent(10.0)),
                Value::Relative(Rel::from_percent(20.0) + Length::pt(1.0))
            )
            .unwrap_err(),
            "cannot compare ratio and relative length"
        );
    }

    #[test]
    fn p1217_datetime_ordena_mesmo_kind_e_nomeia_kinds_distintos() {
        let date_a = Datetime::new_date(2020, 1, 1).unwrap();
        let date_b = Datetime::new_date(2021, 1, 1).unwrap();
        let time_a = Datetime::new_time(1, 0, 0).unwrap();
        let time_b = Datetime::new_time(2, 0, 0).unwrap();
        let dt_a = Datetime::new_datetime(2020, 1, 1, 1, 0, 0).unwrap();
        let dt_b = Datetime::new_datetime(2020, 1, 1, 2, 0, 0).unwrap();
        assert_all_ordering_ops(Value::Datetime(date_a), Value::Datetime(date_b));
        assert_all_ordering_ops(Value::Datetime(time_a), Value::Datetime(time_b));
        assert_all_ordering_ops(Value::Datetime(dt_a), Value::Datetime(dt_b));
        assert_eq!(
            apply_binary(BinOp::Lt, Value::Datetime(date_a), Value::Datetime(time_a))
                .unwrap_err(),
            "cannot compare date and time"
        );
        assert_eq!(
            apply_binary(BinOp::Lt, Value::Datetime(time_a), Value::Datetime(dt_a))
                .unwrap_err(),
            "cannot compare time and datetime"
        );
    }

    #[test]
    fn p1217_arrays_propagam_novos_ramos_e_kinds() {
        assert_eq!(
            apply_binary(
                BinOp::Lt,
                Value::Array(vec![Value::Decimal(Decimal::from_i64(1))]),
                Value::Array(vec![Value::Int(2)])
            ),
            Ok(Value::Bool(true))
        );
        assert_eq!(
            apply_binary(
                BinOp::Lt,
                Value::Array(vec![Value::Datetime(
                    Datetime::new_date(2020, 1, 1).unwrap()
                )]),
                Value::Array(vec![Value::Datetime(Datetime::new_time(1, 0, 0).unwrap())])
            )
            .unwrap_err(),
            "cannot compare date and time"
        );
    }
}
