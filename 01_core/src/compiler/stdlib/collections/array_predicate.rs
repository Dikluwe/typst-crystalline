//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/array_predicate.md
//! @prompt-hash a4127ad9
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::array_predicate`.

use super::*;
use crate::compiler::eval::operators::equality::values_eq;
use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
use crate::compiler::eval::operators::ordering::compare;
use std::cmp::Ordering;

pub(super) fn array_sorted(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }

    let mut unknown = args.named.keys().filter(|k| !matches!(k.as_str(), "key" | "by"));
    if let Some(bad) = unknown.next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {}", bad.as_str()),
        )]);
    }

    let callable = |name: &str| -> SourceResult<Option<Func>> {
        match args.named.get(name) {
            None => Ok(None),
            Some(Value::Func(func)) => Ok(Some(func.clone())),
            Some(Value::Type(ty)) => type_as_callable(*ty).map(Some).ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected function, found {}", ty.name()),
                )]
            }),
            Some(other) => Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected function, found {}", other.type_name()),
            )]),
        }
    };
    let key = callable("key")?;
    let by = callable("by")?;

    let mut keyed = Vec::with_capacity(arr.len());
    for value in arr {
        let key_value = match &key {
            Some(func) => apply_func(
                func.clone(),
                Args::positional(vec![value.clone()]),
                scopes,
                ctx,
                engine,
            )?,
            None => value.clone(),
        };
        keyed.push((key_value, value));
    }

    if let Some(by) = by {
        let mut call = |a: &Value, b: &Value| -> SourceResult<bool> {
            match apply_func(
                by.clone(),
                Args::positional(vec![a.clone(), b.clone()]),
                scopes,
                ctx,
                engine,
            )? {
                Value::Bool(value) => Ok(value),
                other => Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected boolean, found {}", vanilla_type_name(&other)),
                )]),
            }
        };
        // Só troca quando o par inverso está em ordem: `<` e `<=` são estáveis.
        keyed = merge_sort(keyed, &mut |earlier, later| {
            Ok(call(&earlier.0, &later.0)? || !call(&later.0, &earlier.0)?)
        })?;
    } else {
        keyed = merge_sort(keyed, &mut |earlier, later| {
            compare(&later.0, &earlier.0)
                .map(Ordering::is_ge)
                .map_err(|error| vec![SourceDiagnostic::error(args.span, error)])
        })?;
    }

    Ok(Value::Array(keyed.into_iter().map(|(_, value)| value).collect()))
}

/// Ordenação estável que devolve o primeiro erro e aceita relações
/// inconsistentes sem pânico.
fn merge_sort<T>(
    mut items: Vec<T>,
    in_order: &mut impl FnMut(&T, &T) -> SourceResult<bool>,
) -> SourceResult<Vec<T>> {
    if items.len() < 2 {
        return Ok(items);
    }
    let right = items.split_off(items.len().div_ceil(2));
    let capacity = items.len() + right.len();
    let mut left = merge_sort(items, in_order)?.into_iter().peekable();
    let mut right = merge_sort(right, in_order)?.into_iter().peekable();
    let mut merged = Vec::with_capacity(capacity);
    while let (Some(earlier), Some(later)) = (left.peek(), right.peek()) {
        let take_left = in_order(earlier, later)?;
        merged.extend(if take_left { left.next() } else { right.next() });
    }
    merged.extend(left);
    merged.extend(right);
    Ok(merged)
}

pub(super) fn array_filter(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = args.span;
    let pred = expect_one_func(args, "array.filter()")?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let result = apply_func(
            pred.clone(),
            Args::positional(vec![v.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match result {
            Value::Bool(true) => out.push(v),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::Array(out))
}

pub(super) fn array_map(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "array.map()")?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let mapped =
            apply_func(func.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        out.push(mapped);
    }
    Ok(Value::Array(out))
}

pub(super) fn array_find(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = args.span;
    let pred = expect_one_func(args, "array.find()")?;
    for v in arr {
        let result = apply_func(
            pred.clone(),
            Args::positional(vec![v.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match result {
            Value::Bool(true) => return Ok(v),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::None)
}

pub(super) fn array_position(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.position()")?;
    for (index, value) in arr.into_iter().enumerate() {
        let result =
            apply_func(pred.clone(), Args::positional(vec![value]), scopes, ctx, engine)?;
        match result {
            Value::Bool(true) => return Ok(Value::Int(index as i64)),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::None)
}

pub(super) fn array_any(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let span = args.span;
    let pred = expect_one_func(args, "array.any()")?;
    for v in arr {
        let result =
            apply_func(pred.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        match result {
            Value::Bool(true) => return Ok(Value::Bool(true)),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::Bool(false))
}

pub(super) fn array_all(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.all()")?;
    array_all_with_pred(arr, pred, scopes, ctx, engine)
}

pub(super) fn array_all_with_pred(
    arr: Vec<Value>,
    pred: Func,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    for v in arr {
        let result =
            apply_func(pred.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        match result {
            Value::Bool(false) => return Ok(Value::Bool(false)),
            Value::Bool(true) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                    "expected boolean, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        &other,
                    )
                ),
                )])
            }
        }
    }
    Ok(Value::Bool(true))
}

pub(super) fn array_dedup(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    if !args.items.is_empty() || args.named.keys().any(|name| name.as_str() != "key") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let key = match args.named.get("key") {
        None => None,
        Some(Value::Func(func)) => Some(func.clone()),
        Some(Value::Type(t)) => type_as_callable(*t),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected function, found {}", other.type_name()),
            )])
        }
    };
    if key.is_none() {
        return Ok(array_dedup_values(arr));
    }
    let mut result = Vec::new();
    let mut keys = Vec::new();
    for item in arr {
        let item_key = if let Some(func) = &key {
            apply_func(
                func.clone(),
                Args::positional(vec![item.clone()]),
                scopes,
                ctx,
                engine,
            )?
        } else {
            item.clone()
        };
        if !keys.iter().any(|key| values_eq(key, &item_key)) {
            keys.push(item_key);
            result.push(item);
        }
    }
    Ok(Value::Array(result))
}

pub(super) fn array_dedup_values(arr: Vec<Value>) -> Value {
    let mut result = Vec::new();
    for item in arr {
        if !result.iter().any(|kept| values_eq(kept, &item)) {
            result.push(item);
        }
    }
    Value::Array(result)
}
