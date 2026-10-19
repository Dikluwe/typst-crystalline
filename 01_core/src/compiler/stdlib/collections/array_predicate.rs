//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/array_predicate.md
//! @prompt-hash 0837f784
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::array_predicate`.

use super::*;

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
        // Stable insertion sort keeps callback evaluation deterministic and
        // lets evaluation errors escape without hiding them in `Ord` glue.
        for index in 1..keyed.len() {
            let mut cursor = index;
            while cursor > 0 {
                let result = apply_func(
                    by.clone(),
                    Args::positional(vec![
                        keyed[cursor].0.clone(),
                        keyed[cursor - 1].0.clone(),
                    ]),
                    scopes,
                    ctx,
                    engine,
                )?;
                match result {
                    Value::Bool(true) => keyed.swap(cursor, cursor - 1),
                    Value::Bool(false) => break,
                    other => {
                        return Err(vec![SourceDiagnostic::error(
                            args.span,
                            format!(
                                "expected boolean, found {}",
                                crate::compiler::eval::operators::error_formatting::vanilla_type_name(&other)
                            ),
                        )])
                    }
                }
                cursor -= 1;
            }
        }
    } else {
        let mut cmp_err: Option<EcoString> = None;
        keyed.sort_by(|a, b| match value_cmp(&a.0, &b.0) {
            Ok(ordering) => ordering,
            Err(error) => {
                cmp_err = Some(error);
                Ordering::Equal
            }
        });
        if let Some(error) = cmp_err {
            return Err(vec![SourceDiagnostic::error(args.span, error)]);
        }
    }

    Ok(Value::Array(keyed.into_iter().map(|(_, value)| value).collect()))
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
        if !keys.contains(&item_key) {
            keys.push(item_key);
            result.push(item);
        }
    }
    Ok(Value::Array(result))
}

pub(super) fn array_dedup_values(arr: Vec<Value>) -> Value {
    let mut result = Vec::new();
    for item in arr {
        if !result.contains(&item) {
            result.push(item);
        }
    }
    Value::Array(result)
}

/// Comparação parcial entre valores, alinhada com o Typst vanilla para os
/// tipos primitivos suportados em P466: `int`, `float` e `str`.
///
/// **P652** — devolve `Err` quando os valores não são comparáveis (tipos
/// incompatíveis ou `NaN`), em vez de assumir `Ordering::Equal` em silêncio.
pub(super) fn value_cmp(a: &Value, b: &Value) -> Result<Ordering, EcoString> {
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => {
            a.partial_cmp(b).ok_or_else(|| "cannot compare integers".into())
        }
        (Value::Int(i), Value::Float(f)) => (*i as f64)
            .partial_cmp(f)
            .ok_or_else(|| "cannot compare integer and float".into()),
        (Value::Float(f), Value::Int(i)) => f
            .partial_cmp(&(*i as f64))
            .ok_or_else(|| "cannot compare float and integer".into()),
        (Value::Float(a), Value::Float(b)) => {
            a.partial_cmp(b).ok_or_else(|| "cannot compare floats".into())
        }
        (Value::Str(a), Value::Str(b)) => Ok(a.cmp(b)),
        _ => Err(format!(
            "cannot compare {} and {}",
            crate::compiler::eval::operators::error_formatting::vanilla_type_name(b),
            crate::compiler::eval::operators::error_formatting::vanilla_type_name(a)
        )
        .into()),
    }
}
