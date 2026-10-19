//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/arguments.md
//! @prompt-hash afec4d2f
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::arguments`.

use super::*;

pub(super) fn arguments_len(arguments: Args, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    let len = arguments
        .occurrences
        .as_ref()
        .map_or_else(|| arguments.items.len() + arguments.named.len(), Vec::len);
    Ok(Value::Int(len as i64))
}

pub(super) fn arguments_pos(arguments: Args, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(Value::Array(arguments.items))
}

pub(super) fn arguments_named(arguments: Args, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(Value::Dict(arguments.named))
}

pub(super) fn arguments_at(arguments: Args, args: Args) -> SourceResult<Value> {
    if args.named.keys().any(|name| name.as_str() != "default") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let default = args.named.get("default").cloned();
    let Some(key) = args.items.first() else {
        return Err(vec![SourceDiagnostic::error(args.span, "missing argument: key")]);
    };
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let value = match key {
        Value::Int(index) => {
            let len = arguments.items.len() as i64;
            let resolved =
                if *index >= 0 { Some(*index) } else { len.checked_add(*index) };
            resolved
                .filter(|index| *index >= 0 && *index < len)
                .and_then(|index| arguments.items.get(index as usize).cloned())
        }
        Value::Str(name) => arguments.named.get(name).cloned(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer or string, found {}", other.type_name()),
            )])
        }
    };
    value.or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            "argument key is invalid and no default value was specified",
        )]
    })
}

pub(super) fn arguments_filter(
    arguments: Args,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let test = expect_one_func(args, "arguments.filter()")?;
    let mut occurrences = Vec::new();
    for occurrence in arguments.occurrence_sequence() {
        let keep = apply_func(
            test.clone(),
            Args::positional(vec![occurrence.value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match keep {
            Value::Bool(true) => occurrences.push(occurrence),
            Value::Bool(false) => {}
            other => {
                return Err(vec![SourceDiagnostic::error(
                    test.diagnostic_span(),
                    format!("expected boolean, found {}", vanilla_type_name(&other)),
                )])
            }
        }
    }
    Ok(Value::Args(Args::from_occurrences(Span::detached(), occurrences)))
}

pub(super) fn arguments_map(
    arguments: Args,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let mapper = expect_one_func(args, "arguments.map()")?;
    let mut occurrences = arguments.occurrence_sequence();
    for occurrence in &mut occurrences {
        occurrence.value = apply_func(
            mapper.clone(),
            Args::positional(vec![occurrence.value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        occurrence.value_span = Span::detached();
    }
    Ok(Value::Args(Args::from_occurrences(Span::detached(), occurrences)))
}
