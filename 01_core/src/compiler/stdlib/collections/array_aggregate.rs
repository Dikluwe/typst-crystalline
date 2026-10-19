//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/array_aggregate.md
//! @prompt-hash cf8caa6c
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::array_aggregate`.

use super::*;

pub(super) fn array_sum(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    if arr.is_empty() {
        return default.ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "cannot sum an empty array without a default value",
            )]
        });
    }
    let mut sum_int = 0i64;
    let mut sum_float = 0.0f64;
    let mut has_float = false;
    for v in arr {
        match v {
            Value::Int(i) => {
                if has_float {
                    sum_float += i as f64;
                } else {
                    sum_int = sum_int.checked_add(i).ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            Span::detached(),
                            "array.sum(): overflow de inteiro".to_string(),
                        )]
                    })?;
                }
            }
            Value::Float(f) => {
                sum_float += f + sum_int as f64;
                sum_int = 0;
                has_float = true;
            }
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("array.sum() não suporta valor {}", v.type_name()),
                )]);
            }
        }
    }
    Ok(if has_float { Value::Float(sum_float) } else { Value::Int(sum_int) })
}

pub(super) fn array_product(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    if arr.is_empty() {
        return default.ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "cannot calculate the product of an empty array without a default value",
            )]
        });
    }
    let mut product = Value::Int(1);
    for value in arr {
        product = crate::compiler::eval::operators::eval_binary_op(
            crate::entities::ast::expr::BinOp::Mul,
            product,
            value,
        )
        .map_err(|message| vec![SourceDiagnostic::error(args.span, message)])?;
    }
    Ok(product)
}

pub(super) fn array_fold(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mut items = args.items.into_iter();
    let start = items.next().unwrap_or(Value::None);
    let reducer = match items.next() {
        Some(Value::Func(f)) => f,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "array.fold() espera função como reducer, recebeu {}",
                    other.type_name()
                ),
            )]);
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "array.fold() requer start e reducer como argumentos posicionais"
                    .to_string(),
            )]);
        }
    };
    let mut acc = start;
    for item in arr {
        acc = apply_func(
            reducer.clone(),
            Args::positional(vec![acc, item]),
            scopes,
            ctx,
            engine,
        )?;
    }
    Ok(acc)
}

pub(super) fn array_reduce(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let reducer = expect_one_func(args, "array.reduce()")?;
    let mut values = arr.into_iter();
    let Some(mut acc) = values.next() else {
        return Ok(Value::None);
    };
    for item in values {
        acc = apply_func(
            reducer.clone(),
            Args::positional(vec![acc, item]),
            scopes,
            ctx,
            engine,
        )?;
    }
    Ok(acc)
}
