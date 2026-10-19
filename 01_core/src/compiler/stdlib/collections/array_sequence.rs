//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/collections/array_sequence.md
//! @prompt-hash 141db996
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::array_sequence`.

use super::*;

pub(super) fn array_rev(arr: Vec<Value>) -> Value {
    Value::Array(arr.into_iter().rev().collect())
}

pub(super) fn array_zip(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    for name in args.named.keys() {
        if name != "exact" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    let exact = match args.named.get("exact") {
        None | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) => true,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };
    let mut arrays = Vec::with_capacity(args.items.len() + 1);
    arrays.push(arr);
    for value in args.items {
        match value {
            Value::Array(array) => arrays.push(array),
            other => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    format!("expected array, found {}", other.type_name()),
                )])
            }
        }
    }
    let first_len = arrays[0].len();
    if exact && arrays.iter().any(|array| array.len() != first_len) {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "array lengths differ with exact: true",
        )]);
    }
    let len = arrays.iter().map(Vec::len).min().unwrap_or(0);
    Ok(Value::Array(
        (0..len)
            .map(|index| {
                Value::Array(arrays.iter().map(|array| array[index].clone()).collect())
            })
            .collect(),
    ))
}

pub(super) fn array_enumerate(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let start = match args.items.first() {
        None => 0,
        Some(Value::Int(value)) => *value,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected integer, found {}", other.type_name()),
            )])
        }
    };
    Ok(Value::Array(
        arr.into_iter()
            .enumerate()
            .map(|(i, v)| Value::Array(vec![Value::Int(start + i as i64), v]))
            .collect(),
    ))
}

pub(super) fn array_chunks(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    for name in args.named.keys() {
        if name != "exact" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }
    let exact = match args.named.get("exact") {
        None | Some(Value::Bool(false)) => false,
        Some(Value::Bool(true)) => true,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };
    let n = expect_one_int(args, "array.chunks()")?;
    if n <= 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "array.chunks() requer inteiro positivo".to_string(),
        )]);
    }
    let n = n as usize;
    let chunks: Vec<Value> = arr
        .chunks(n)
        .filter(|chunk| !exact || chunk.len() == n)
        .map(|c| Value::Array(c.to_vec()))
        .collect();
    Ok(Value::Array(chunks))
}

pub(super) fn array_windows(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let n = expect_one_int(args, "array.windows()")?;
    if n <= 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "array.windows() requer inteiro positivo".to_string(),
        )]);
    }
    let n = n as usize;
    let windows: Vec<Value> = arr.windows(n).map(|w| Value::Array(w.to_vec())).collect();
    Ok(Value::Array(windows))
}

pub(super) fn array_flatten(arr: Vec<Value>) -> Value {
    fn push_flat(value: Value, output: &mut Vec<Value>) {
        match value {
            Value::Array(inner) => {
                for value in inner {
                    push_flat(value, output);
                }
            }
            other => output.push(other),
        }
    }
    let mut result = Vec::new();
    for item in arr {
        push_flat(item, &mut result);
    }
    Value::Array(result)
}

/// **P843 (#60)** — `array.join(separator?, last:?, default:?)` — paridade
/// vanilla `Array::join` (foundations/array.rs:754-786), medida em
/// `temp/p843/join*.typ`: separador posicional opcional (default `none`);
/// `last:` separador alternativo antes do último elemento; `default:`
/// devolvido para array vazio (vazio sem default → `none`). A concatenação
/// usa a op `join` da linguagem (mesma de code blocks), incluindo o erro
/// `cannot join X with Y` para tipos incompatíveis.
pub(super) fn array_join(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    for key in args.named.keys() {
        if key.as_str() != "last" && key.as_str() != "default" {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {}", key.as_str()),
            )]);
        }
    }
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }

    let span = args.span;
    let separator = args.items.into_iter().next().unwrap_or(Value::None);
    let default = args.named.get("default").cloned();
    let mut last = args.named.get("last").cloned();

    let len = arr.len();
    if len == 0 {
        return Ok(default.unwrap_or(Value::None));
    }

    let mut result = Value::None;
    for (i, value) in arr.into_iter().enumerate() {
        if i > 0 {
            let sep = if i + 1 == len && last.is_some() {
                last.take().unwrap()
            } else {
                separator.clone()
            };
            result = crate::compiler::eval::operators::join(result, sep)
                .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])?;
        }
        result = crate::compiler::eval::operators::join(result, value)
            .map_err(|msg| vec![SourceDiagnostic::error(span, msg)])?;
    }
    Ok(result)
}

pub(super) fn array_intersperse(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.is_empty() {
                "missing argument: separator"
            } else {
                "unexpected argument"
            },
        )]);
    }
    let separator = args.items[0].clone();
    let mut output = Vec::with_capacity(arr.len().saturating_mul(2).saturating_sub(1));
    for (index, value) in arr.into_iter().enumerate() {
        if index > 0 {
            output.push(separator.clone());
        }
        output.push(value);
    }
    Ok(Value::Array(output))
}

pub(super) fn array_split(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if !args.named.is_empty() || args.items.len() != 1 {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.is_empty() {
                "missing argument: at"
            } else {
                "unexpected argument"
            },
        )]);
    }
    let separator = &args.items[0];
    let mut groups = Vec::new();
    let mut current = Vec::new();
    for value in arr {
        if &value == separator {
            groups.push(Value::Array(std::mem::take(&mut current)));
        } else {
            current.push(value);
        }
    }
    groups.push(Value::Array(current));
    Ok(Value::Array(groups))
}

pub(super) fn array_to_dict(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    ensure_no_args(&args)?;
    let mut output = IndexMap::with_hasher(FxBuildHasher);
    for entry in arr {
        match entry {
            Value::Array(pair) if pair.len() == 2 => match pair.as_slice() {
                [Value::Str(key), value] => {
                    output.insert(key.clone(), value.clone());
                }
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "expected pair with string key",
                    )])
                }
            },
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    args.span,
                    "expected array of pairs",
                )])
            }
        }
    }
    Ok(Value::Dict(output))
}
