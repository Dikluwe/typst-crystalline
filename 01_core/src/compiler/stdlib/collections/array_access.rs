//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/array_access.md
//! @prompt-hash 03b9b169
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::array_access`.

use super::*;

pub(super) fn array_len(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(Value::Int(arr.len() as i64))
}

pub(super) fn array_first(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    arr.first().cloned().or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            "array is empty and no default value was specified",
        )]
    })
}

pub(super) fn array_last(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    arr.last().cloned().or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            "array is empty and no default value was specified",
        )]
    })
}

/// `array.at(index, default: value)` — P714. Paridade com o vanilla
/// (`foundations/array.rs:207-221`, `locate_opt`): índice negativo conta a
/// partir do fim (`len + index`); fora de limites usa `default` se
/// fornecido, senão erro (mesma mensagem do vanilla,
/// `out_of_bounds_no_default`).
pub(super) fn array_at(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if args.named.len() > 1
        || (args.named.len() == 1 && !args.named.contains_key("default"))
    {
        let bad = args
            .named
            .keys()
            .find(|k| k.as_str() != "default")
            .map(|k| k.as_str())
            .unwrap_or("?");
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("array.at() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }
    if args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: index")]);
    }
    let default = args.named.get("default").cloned();
    let index = expect_one_int(args, "array.at()")?;
    let len = arr.len() as i64;
    let resolved = if index >= 0 { Some(index) } else { len.checked_add(index) };
    let value = resolved
        .filter(|&v| v >= 0 && v < len)
        .and_then(|v| arr.get(v as usize).cloned());
    match value {
        Some(v) => Ok(v),
        None => match default {
            Some(v) => Ok(v),
            None => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "array index out of bounds (index: {index}, len: {len}) \
                     and no default value was specified"
                ),
            )]),
        },
    }
}

/// `array.slice(start, end?, count:)` — P730. Paridade com o vanilla
/// (`foundations/array.rs:279-300`, sobre `locate(index, end_ok: true)`
/// de `array.rs:122-137`): índices negativos contam a partir do fim
/// (`len + index`, `checked_add`); `start`/`end` efetivos admitem
/// `index == len`; `count:` equivale a `end = start_resolvido + count`
/// (mutuamente exclusivo com `end`); `end < start` → sub-array vazio
/// (clamp `max(start)`); fora de limites → erro com a mensagem exacta
/// do vanilla (`out_of_bounds`, com o índice original).
/// Medido: `(1,2,3,4).slice(1, 3)` → `(2, 3)`; `.slice(-2)` → `(3, 4)`;
/// `.slice(0, count: 2)` → `(1, 2)`; `.slice(1, count: -1)` → `()`.
pub(super) fn array_slice(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let mut positional = args.items.iter();
    let start = positional.next().and_then(|v| v.cast_int()).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            "array.slice(): start espera int".to_string(),
        )]
    })?;
    let end_positional = positional.next().and_then(|v| v.cast_int());
    let end_named = args.named.get("end").and_then(|v| v.cast_int());
    let count = args.named.get("count").and_then(|v| v.cast_int());
    let end = end_positional.or(end_named);

    if end.is_some() && count.is_some() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "`end` and `count` are mutually exclusive".to_string(),
        )]);
    }

    let len = arr.len() as i64;
    // `locate(index, end_ok: true)` do vanilla: negativo → len + index;
    // admite index == len; o erro reporta o índice original.
    let locate = |index: i64| -> Result<usize, Vec<SourceDiagnostic>> {
        let wrapped = if index >= 0 { Some(index) } else { len.checked_add(index) };
        wrapped
            .and_then(|v| usize::try_from(v).ok())
            .filter(|&v| v <= len as usize)
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!("array index out of bounds (index: {index}, len: {len})"),
                )]
            })
    };

    let start_idx = locate(start)?;
    let end_raw = end.or(count.map(|c| start_idx as i64 + c)).unwrap_or(len);
    let end_idx = locate(end_raw)?.max(start_idx);
    Ok(Value::Array(arr[start_idx..end_idx].to_vec()))
}

pub(super) fn array_contains(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.is_empty() {
                "missing argument: value"
            } else {
                "unexpected argument"
            },
        )]);
    }
    Ok(Value::Bool(arr.iter().any(|item| item == &args.items[0])))
}
