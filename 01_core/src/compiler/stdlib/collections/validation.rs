//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/validation.md
//! @prompt-hash 66e39c33
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::validation`.

use super::*;

pub(super) fn ensure_no_args(args: &Args) -> SourceResult<()> {
    if let Some(name) = args.named.keys().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    Ok(())
}

pub(super) fn array_no_args(
    arr: Vec<Value>,
    args: Args,
    operation: fn(Vec<Value>) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(arr))
}

pub(super) fn dict_no_args(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    operation: fn(IndexMap<EcoString, Value, FxBuildHasher>) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(dict))
}

pub(super) fn str_no_args(
    value: EcoString,
    args: Args,
    operation: fn(EcoString) -> Value,
) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    Ok(operation(value))
}

pub(super) fn optional_default(args: &Args) -> SourceResult<Option<Value>> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    for name in args.named.keys() {
        if name != "default" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    Ok(args.named.get("default").cloned())
}

pub(super) fn expect_str_value(
    args: Args,
    context: &str,
) -> SourceResult<(EcoString, Value)> {
    match args.items.as_slice() {
        [Value::Str(key), value] => Ok((key.clone(), value.clone())),
        [other, ..] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}: primeiro argumento deve ser string, recebeu {}",
                context,
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 2 argumentos posicionais", context),
        )]),
    }
}

/// P881 — converte um tipo chamável no construtor nativo correspondente,
/// permitindo `(0, 1).map(str)`, `(1, 2).map(int)`, etc., como no vanilla.
pub(super) fn type_as_callable(t: Type) -> Option<Func> {
    Some(match t {
        Type::Int => Func::native("int", native_int),
        Type::Float => Func::native("float", native_float),
        Type::Str => Func::native("str", native_str),
        Type::Type => Func::native("type", native_type),
        Type::Counter => Func::native("counter", native_counter),
        Type::State => Func::native("state", native_state),
        Type::Symbol => Func::native("symbol", native_symbol),
        Type::Bytes => Func::native("bytes", native_bytes),
        Type::Datetime => Func::native("datetime", native_datetime),
        _ => return None, // neutro: N16[β] — Value sem representação iterável retorna None
    })
}

pub(super) fn expect_one_func(args: Args, context: &str) -> SourceResult<Func> {
    let parameter = match context {
        "array.all()" | "array.any()" | "array.filter()" => "test",
        "array.find()" | "array.position()" => "searcher",
        "array.map()" => "mapper",
        "dict.map()" | "dict.filter()" | "arguments.map()" | "arguments.filter()" => {
            "mapper"
        }
        _ => "function",
    };
    if !args.named.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("the argument `{parameter}` is positional"),
        )]);
    }
    match args.items.as_slice() {
        [Value::Func(f)] => Ok(f.clone()),
        [Value::Type(t)] => type_as_callable(*t).ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                format!("{} espera função, recebeu {}", context, t.name()),
            )]
        }),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera função, recebeu {}", context, other.type_name()),
        )]),
        [] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("missing argument: {parameter}"),
        )]),
        _ => Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]),
    }
}

pub(super) fn expect_one_dict(
    args: Args,
    context: &str,
) -> SourceResult<IndexMap<EcoString, Value, FxBuildHasher>> {
    match args.items.as_slice() {
        [Value::Dict(d)] => Ok(d.clone()),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera dicionário, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

pub(super) fn expect_one_str(args: Args, context: &str) -> SourceResult<EcoString> {
    match args.items.as_slice() {
        [Value::Str(s)] => Ok(s.clone()),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera string, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

pub(super) fn expect_two_str(
    args: Args,
    context: &str,
) -> SourceResult<(EcoString, EcoString)> {
    match args.items.as_slice() {
        [Value::Str(a), Value::Str(b)] => Ok((a.clone(), b.clone())),
        [Value::Str(_), other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}: segundo argumento deve ser string, recebeu {}",
                context,
                other.type_name()
            ),
        )]),
        [other, ..] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "{}: primeiro argumento deve ser string, recebeu {}",
                context,
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 2 argumentos posicionais", context),
        )]),
    }
}

pub(super) fn expect_one_int(args: Args, context: &str) -> SourceResult<i64> {
    match args.items.as_slice() {
        [Value::Int(i)] => Ok(*i),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera inteiro, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}
