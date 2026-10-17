//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/collections/bytes.md
//! @prompt-hash a4aad7ac
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::bytes`.

use super::*;

#[cfg(test)]
pub(super) fn try_dispatch_bytes_method(
    bytes: Bytes,
    method: &str,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> Option<SourceResult<Value>> {
    match method {
        "len" => Some(bytes_len(bytes, args, call_spans)),
        "at" => Some(bytes_at(bytes, args, call_spans)),
        "slice" => Some(bytes_slice(bytes, args, call_spans)),
        _ => None,
    }
}

pub(super) fn unexpected_named(
    args: &Args,
    allowed: &[&str],
    call_spans: Option<&CollectionCallSpans>,
) -> Option<SourceDiagnostic> {
    args.named
        .keys()
        .find(|name| !allowed.contains(&name.as_str()))
        .map(|name| {
            let span = call_spans
                .map(|spans| spans.named_or(name.as_str(), args.span))
                .unwrap_or(args.span);
            SourceDiagnostic::error(span, format!("unexpected argument: {name}"))
        })
}

pub(super) fn bytes_len(
    bytes: Bytes,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> SourceResult<Value> {
    if let Some(err) = unexpected_named(&args, &[], call_spans) {
        return Err(vec![err]);
    }
    if !args.items.is_empty() {
        let span = call_spans
            .map(|spans| spans.positional_or(0, args.span))
            .unwrap_or(args.span);
        return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
    }
    Ok(Value::Int(bytes.len() as i64))
}

pub(super) fn bytes_at(
    bytes: Bytes,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> SourceResult<Value> {
    let call_span = call_spans.map(|spans| spans.call_or(args.span)).unwrap_or(args.span);
    let Some(index_value) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(call_span, "missing argument: index")]);
    };
    let index = match index_value {
        Value::Int(index) => *index,
        other => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.positional_or(0, args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };
    if args.items.len() > 1 {
        let span = call_spans
            .map(|spans| spans.positional_or(1, args.span))
            .unwrap_or(args.span);
        return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
    }
    if let Some(err) = unexpected_named(&args, &["default"], call_spans) {
        return Err(vec![err]);
    }

    let len = bytes.len() as i64;
    let resolved = if index >= 0 { Some(index) } else { len.checked_add(index) };
    if let Some(byte) = resolved
        .filter(|&value| value >= 0 && value < len)
        .and_then(|value| bytes.as_slice().get(value as usize))
    {
        return Ok(Value::Int((*byte).into()));
    }
    if let Some(default) = args.named.get("default") {
        return Ok(default.clone());
    }
    Err(vec![SourceDiagnostic::error(
        call_span,
        format!(
            "byte index out of bounds (index: {index}, len: {len}) and no default value was specified"
        ),
    )])
}

pub(super) fn bytes_slice(
    bytes: Bytes,
    args: Args,
    call_spans: Option<&CollectionCallSpans>,
) -> SourceResult<Value> {
    let call_span = call_spans.map(|spans| spans.call_or(args.span)).unwrap_or(args.span);
    let Some(start_value) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(call_span, "missing argument: start")]);
    };
    let start = match start_value {
        Value::Int(start) => *start,
        other => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.positional_or(0, args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };
    let end = match args.items.get(1) {
        None | Some(Value::None) => None,
        Some(Value::Int(end)) => Some(*end),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.positional_or(1, args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer or none, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };
    if args.items.len() > 2 {
        let span = call_spans
            .map(|spans| spans.positional_or(2, args.span))
            .unwrap_or(args.span);
        return Err(vec![SourceDiagnostic::error(span, "unexpected argument")]);
    }
    if let Some(err) = unexpected_named(&args, &["count"], call_spans) {
        return Err(vec![err]);
    }
    let count = match args.named.get("count") {
        None => None,
        Some(Value::Int(count)) => Some(*count),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                call_spans
                    .map(|spans| spans.named_or("count", args.span))
                    .unwrap_or(args.span),
                format!(
                    "expected integer, found {}",
                    crate::compiler::eval::operators::error_formatting::vanilla_type_name(
                        other
                    )
                ),
            )])
        }
    };

    let len = bytes.len() as i64;
    let locate = |index: i64| -> Result<usize, Vec<SourceDiagnostic>> {
        let wrapped = if index >= 0 { Some(index) } else { len.checked_add(index) };
        wrapped
            .and_then(|value| usize::try_from(value).ok())
            .filter(|&value| value <= bytes.len())
            .ok_or_else(|| {
                vec![SourceDiagnostic::error(
                    call_span,
                    format!("byte index out of bounds (index: {index}, len: {len})"),
                )]
            })
    };

    let start_index = locate(start)?;
    let end_raw = end
        .or(count.map(|count| (start_index as i64).wrapping_add(count)))
        .unwrap_or(len);
    let end_index = locate(end_raw)?.max(start_index);
    Ok(Value::Bytes(Bytes::new(bytes.as_slice()[start_index..end_index].to_vec())))
}
