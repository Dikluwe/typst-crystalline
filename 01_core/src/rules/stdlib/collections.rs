//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/collections.md
//! @layer L1
//! @updated 2026-06-25
//!
//! Métodos de instância para os tipos de coleção `array`, `dict` e `str`.
//! Materializado no Passo 466 (P466).

use std::cmp::Ordering;

use ecow::EcoString;
use indexmap::IndexMap;
use rustc_hash::FxBuildHasher;

use crate::entities::args::Args;
use crate::entities::engine::Engine;
use crate::entities::func::Func;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::rules::eval::closures::apply_func;
use crate::rules::eval::EvalContext;
use crate::rules::scopes::Scopes;

/// Tenta despachar uma chamada de método de coleção (`array`, `dict`, `str`).
///
/// Retorna `Some(Result)` se o par `(target, method)` for reconhecido;
/// `None` caso contrário, permitindo que o caller continue com o dispatch
/// genérico de funções.
pub(crate) fn try_dispatch_collection_method(
    target: Value,
    method: &str,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> Option<SourceResult<Value>> {
    match (target, method) {
        // ── array ────────────────────────────────────────────────────────────
        (Value::Array(arr), "first") => Some(Ok(array_first(arr))),
        (Value::Array(arr), "last") => Some(Ok(array_last(arr))),
        (Value::Array(arr), "rev") => Some(Ok(array_rev(arr))),
        (Value::Array(arr), "sum") => Some(array_sum(arr)),
        (Value::Array(arr), "sorted") => Some(array_sorted(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "filter") => Some(array_filter(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "map") => Some(array_map(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "find") => Some(array_find(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "any") => Some(array_any(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "all") => Some(array_all(arr, args, scopes, ctx, engine)),
        (Value::Array(arr), "zip") => Some(array_zip(arr, args)),
        (Value::Array(arr), "enumerate") => Some(Ok(array_enumerate(arr))),
        (Value::Array(arr), "dedup") => Some(Ok(array_dedup(arr))),
        (Value::Array(arr), "chunks") => Some(array_chunks(arr, args)),
        (Value::Array(arr), "windows") => Some(array_windows(arr, args)),
        (Value::Array(arr), "flatten") => Some(Ok(array_flatten(arr))),
        (Value::Array(arr), "fold") => Some(array_fold(arr, args, scopes, ctx, engine)),

        // ── dict ─────────────────────────────────────────────────────────────
        (Value::Dict(dict), "keys") => Some(Ok(dict_keys(dict))),
        (Value::Dict(dict), "values") => Some(Ok(dict_values(dict))),
        (Value::Dict(dict), "pairs") => Some(Ok(dict_pairs(dict))),
        (Value::Dict(dict), "remove") => Some(dict_remove(dict, args)),
        (Value::Dict(dict), "update") => Some(dict_update(dict, args)),
        (Value::Dict(dict), "at") => Some(dict_at(dict, args)),

        // ── str ──────────────────────────────────────────────────────────────
        (Value::Str(s), "contains") => Some(str_contains(s, args)),
        (Value::Str(s), "starts-with") => Some(str_starts_with(s, args)),
        (Value::Str(s), "ends-with") => Some(str_ends_with(s, args)),
        (Value::Str(s), "find") => Some(str_find(s, args)),
        (Value::Str(s), "replace") => Some(str_replace(s, args)),
        (Value::Str(s), "trim") => Some(Ok(str_trim(s))),
        (Value::Str(s), "split") => Some(str_split(s, args)),
        (Value::Str(s), "repeat") => Some(str_repeat(s, args)),

        _ => None,
    }
}

// ── array helpers ───────────────────────────────────────────────────────────

fn array_first(arr: Vec<Value>) -> Value {
    arr.first().cloned().unwrap_or(Value::None)
}

fn array_last(arr: Vec<Value>) -> Value {
    arr.last().cloned().unwrap_or(Value::None)
}

fn array_rev(arr: Vec<Value>) -> Value {
    Value::Array(arr.into_iter().rev().collect())
}

fn array_sum(arr: Vec<Value>) -> SourceResult<Value> {
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
    Ok(if has_float {
        Value::Float(sum_float)
    } else {
        Value::Int(sum_int)
    })
}

fn array_sorted(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let key = if args.items.is_empty() {
        None
    } else {
        match args.items.as_slice() {
            [Value::Func(f)] => Some(f.clone()),
            [other] => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    format!(
                        "array.sorted() espera função como chave, recebeu {}",
                        other.type_name()
                    ),
                )]);
            }
            _ => {
                return Err(vec![SourceDiagnostic::error(
                    Span::detached(),
                    "array.sorted() aceita no máximo 1 argumento".to_string(),
                )]);
            }
        }
    };

    let mut sorted = arr;
    if let Some(key_fn) = key {
        let mut keyed: Vec<(Value, Value)> = Vec::with_capacity(sorted.len());
        for v in sorted {
            let k = apply_func(key_fn.clone(), Args::positional(vec![v.clone()]), scopes, ctx, engine)?;
            keyed.push((k, v));
        }
        keyed.sort_by(|a, b| value_cmp(&a.0, &b.0).unwrap_or(Ordering::Equal));
        Ok(Value::Array(keyed.into_iter().map(|(_, v)| v).collect()))
    } else {
        sorted.sort_by(|a, b| value_cmp(a, b).unwrap_or(Ordering::Equal));
        Ok(Value::Array(sorted))
    }
}

fn array_filter(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.filter()")?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let result = apply_func(pred.clone(), Args::positional(vec![v.clone()]), scopes, ctx, engine)?;
        if result.truthy() {
            out.push(v);
        }
    }
    Ok(Value::Array(out))
}

fn array_map(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "array.map()")?;
    let mut out = Vec::with_capacity(arr.len());
    for v in arr {
        let mapped = apply_func(func.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        out.push(mapped);
    }
    Ok(Value::Array(out))
}

fn array_find(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.find()")?;
    for v in arr {
        let result = apply_func(pred.clone(), Args::positional(vec![v.clone()]), scopes, ctx, engine)?;
        if result.truthy() {
            return Ok(v);
        }
    }
    Ok(Value::None)
}

fn array_any(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.any()")?;
    for v in arr {
        let result = apply_func(pred.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        if result.truthy() {
            return Ok(Value::Bool(true));
        }
    }
    Ok(Value::Bool(false))
}

fn array_all(
    arr: Vec<Value>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let pred = expect_one_func(args, "array.all()")?;
    for v in arr {
        let result = apply_func(pred.clone(), Args::positional(vec![v]), scopes, ctx, engine)?;
        if !result.truthy() {
            return Ok(Value::Bool(false));
        }
    }
    Ok(Value::Bool(true))
}

fn array_zip(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let other = expect_one_array(args, "array.zip()")?;
    let zipped: Vec<Value> = arr
        .into_iter()
        .zip(other)
        .map(|(a, b)| Value::Array(vec![a, b]))
        .collect();
    Ok(Value::Array(zipped))
}

fn array_enumerate(arr: Vec<Value>) -> Value {
    Value::Array(
        arr.into_iter()
            .enumerate()
            .map(|(i, v)| Value::Array(vec![Value::Int(i as i64), v]))
            .collect(),
    )
}

fn array_dedup(arr: Vec<Value>) -> Value {
    let mut result = Vec::new();
    for item in arr {
        if result.last() != Some(&item) {
            result.push(item);
        }
    }
    Value::Array(result)
}

fn array_chunks(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let n = expect_one_int(args, "array.chunks()")?;
    if n <= 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "array.chunks() requer inteiro positivo".to_string(),
        )]);
    }
    let n = n as usize;
    let chunks: Vec<Value> = arr.chunks(n).map(|c| Value::Array(c.to_vec())).collect();
    Ok(Value::Array(chunks))
}

fn array_windows(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
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

fn array_flatten(arr: Vec<Value>) -> Value {
    let mut result = Vec::new();
    for item in arr {
        if let Value::Array(inner) = item {
            result.extend(inner);
        } else {
            result.push(item);
        }
    }
    Value::Array(result)
}

fn array_fold(
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
                format!("array.fold() espera função como reducer, recebeu {}", other.type_name()),
            )]);
        }
        None => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "array.fold() requer start e reducer como argumentos posicionais".to_string(),
            )]);
        }
    };
    let mut acc = start;
    for item in arr {
        acc = apply_func(reducer.clone(), Args::positional(vec![acc, item]), scopes, ctx, engine)?;
    }
    Ok(acc)
}

// ── dict helpers ────────────────────────────────────────────────────────────

fn dict_keys(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(dict.into_keys().map(Value::Str).collect())
}

fn dict_values(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(dict.into_values().collect())
}

fn dict_pairs(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(
        dict.into_iter()
            .map(|(k, v)| Value::Array(vec![Value::Str(k), v]))
            .collect(),
    )
}

fn dict_remove(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    let key = expect_one_str(args, "dict.remove()")?;
    Ok(dict.shift_remove(&key).unwrap_or(Value::None))
}

fn dict_update(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    let other = expect_one_dict(args, "dict.update()")?;
    dict.extend(other);
    Ok(Value::Dict(dict))
}

/// `dict.at(key, default: value)` — P491.
/// Retorna o valor associado a `key` ou `default` se a chave não existir.
fn dict_at(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    // Validar named args antes de consumir `args`.
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("default")) {
        let bad = args.named.keys().find(|k| k.as_str() != "default")
            .map(|k| k.as_str()).unwrap_or("?");
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("dict.at() argumento nomeado desconhecido: '{bad}'"),
        )]);
    }
    let default = args.named.get("default").cloned();
    let key = expect_one_str(args, "dict.at()")?;
    match dict.get(&key) {
        Some(value) => Ok(value.clone()),
        None => match default {
            Some(value) => Ok(value),
            None => Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("dicionário não contém a chave {:?}", key.as_str()),
            )]),
        },
    }
}

// ── str helpers ─────────────────────────────────────────────────────────────

fn str_contains(s: EcoString, args: Args) -> SourceResult<Value> {
    let substr = expect_one_str(args, "str.contains()")?;
    Ok(Value::Bool(s.contains(substr.as_str())))
}

fn str_starts_with(s: EcoString, args: Args) -> SourceResult<Value> {
    let prefix = expect_one_str(args, "str.starts-with()")?;
    Ok(Value::Bool(s.starts_with(prefix.as_str())))
}

fn str_ends_with(s: EcoString, args: Args) -> SourceResult<Value> {
    let suffix = expect_one_str(args, "str.ends-with()")?;
    Ok(Value::Bool(s.ends_with(suffix.as_str())))
}

fn str_find(s: EcoString, args: Args) -> SourceResult<Value> {
    let substr = expect_one_str(args, "str.find()")?;
    Ok(s.find(substr.as_str())
        .map(|i| Value::Int(i as i64))
        .unwrap_or(Value::None))
}

fn str_replace(s: EcoString, args: Args) -> SourceResult<Value> {
    let (old, new) = expect_two_str(args, "str.replace()")?;
    Ok(Value::Str(s.replace(old.as_str(), new.as_str()).into()))
}

fn str_trim(s: EcoString) -> Value {
    Value::Str(s.trim().into())
}

fn str_split(s: EcoString, args: Args) -> SourceResult<Value> {
    let sep = expect_one_str(args, "str.split()")?;
    Ok(Value::Array(
        s.split(sep.as_str())
            .map(|part| Value::Str(part.into()))
            .collect(),
    ))
}

fn str_repeat(s: EcoString, args: Args) -> SourceResult<Value> {
    let n = expect_one_int(args, "str.repeat()")?;
    if n < 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.repeat() requer inteiro não-negativo".to_string(),
        )]);
    }
    Ok(Value::Str(s.repeat(n as usize).into()))
}

// ── argument helpers ────────────────────────────────────────────────────────

fn expect_one_func(args: Args, context: &str) -> SourceResult<Func> {
    match args.items.as_slice() {
        [Value::Func(f)] => Ok(f.clone()),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera função, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

fn expect_one_array(args: Args, context: &str) -> SourceResult<Vec<Value>> {
    match args.items.as_slice() {
        [Value::Array(a)] => Ok(a.clone()),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} espera array, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 1 argumento posicional", context),
        )]),
    }
}

fn expect_one_dict(
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

fn expect_one_str(args: Args, context: &str) -> SourceResult<EcoString> {
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

fn expect_two_str(args: Args, context: &str) -> SourceResult<(EcoString, EcoString)> {
    match args.items.as_slice() {
        [Value::Str(a), Value::Str(b)] => Ok((a.clone(), b.clone())),
        [Value::Str(_), other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}: segundo argumento deve ser string, recebeu {}", context, other.type_name()),
        )]),
        [other, ..] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{}: primeiro argumento deve ser string, recebeu {}", context, other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("{} requer 2 argumentos posicionais", context),
        )]),
    }
}

fn expect_one_int(args: Args, context: &str) -> SourceResult<i64> {
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

// ── value comparison (para sorted) ──────────────────────────────────────────

/// Comparação parcial entre valores, alinhada com o Typst vanilla para os
/// tipos primitivos suportados em P466: `int`, `float` e `str`.
fn value_cmp(a: &Value, b: &Value) -> Option<Ordering> {
    match (a, b) {
        (Value::Int(a), Value::Int(b)) => a.partial_cmp(b),
        (Value::Int(i), Value::Float(f)) => (*i as f64).partial_cmp(f),
        (Value::Float(f), Value::Int(i)) => f.partial_cmp(&(*i as f64)),
        (Value::Float(a), Value::Float(b)) => a.partial_cmp(b),
        (Value::Str(a), Value::Str(b)) => Some(a.cmp(b)),
        _ => None,
    }
}
