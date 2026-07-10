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
use crate::entities::regex::Regex;
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
        (Value::Array(arr), "len") => Some(Ok(Value::Int(arr.len() as i64))),
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
        (Value::Dict(dict), "insert") => Some(dict_insert(dict, args)),
        (Value::Dict(dict), "at") => Some(dict_at(dict, args)),
        (Value::Dict(dict), "len") => Some(Ok(dict_len(dict))),
        (Value::Dict(dict), "map") => Some(dict_map(dict, args, scopes, ctx, engine)),
        (Value::Dict(dict), "filter") => Some(dict_filter(dict, args, scopes, ctx, engine)),

        // ── str ──────────────────────────────────────────────────────────────
        (Value::Str(s), "len") => Some(Ok(str_len(s))),
        (Value::Str(s), "first") => Some(Ok(str_first(s))),
        (Value::Str(s), "last") => Some(Ok(str_last(s))),
        (Value::Str(s), "at") => Some(str_at(s, args)),
        (Value::Str(s), "slice") => Some(str_slice(s, args)),
        (Value::Str(s), "clusters") => Some(Ok(str_clusters(s))),
        (Value::Str(s), "contains") => Some(str_contains(s, args)),
        (Value::Str(s), "starts-with") => Some(str_starts_with(s, args)),
        (Value::Str(s), "ends-with") => Some(str_ends_with(s, args)),
        (Value::Str(s), "find") => Some(str_find(s, args)),
        (Value::Str(s), "replace") => Some(str_replace(s, args)),
        (Value::Str(s), "trim") => Some(Ok(str_trim(s))),
        (Value::Str(s), "split") => Some(str_split(s, args)),
        (Value::Str(s), "repeat") => Some(str_repeat(s, args)),
        (Value::Str(s), "to-upper") => Some(Ok(str_to_upper(s))),
        (Value::Str(s), "to-lower") => Some(Ok(str_to_lower(s))),
        (Value::Str(s), "to-unicode") => Some(Ok(str_to_unicode(s))),
        (Value::Str(s), "rev") => Some(Ok(str_rev(s))),
        (Value::Str(s), "codepoints") => Some(Ok(str_codepoints(s))),
        (Value::Str(s), "position") => Some(str_position(s, args)),
        (Value::Str(s), "match") => Some(str_match(s, args)),

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
    // `array.sorted()` não aceita argumentos posicionais; a única named arg
    // válida é `key`. Qualquer outra nomeada é rejeitada, alinhando com o
    // Typst vanilla.
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "unexpected argument".to_string(),
        )]);
    }

    let mut unknown = args.named.keys().filter(|k| k.as_str() != "key");
    if let Some(bad) = unknown.next() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("unexpected argument: {}", bad.as_str()),
        )]);
    }

    let key = match args.named.get("key") {
        Some(Value::Func(f)) => Some(f.clone()),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "array.sorted() expects function for key, received {}",
                    other.type_name()
                ),
            )]);
        }
        None => None,
    };

    let mut sorted = arr;
    let mut cmp_err: Option<EcoString> = None;

    if let Some(key_fn) = key {
        let mut keyed: Vec<(Value, Value)> = Vec::with_capacity(sorted.len());
        for v in sorted {
            let k = apply_func(key_fn.clone(), Args::positional(vec![v.clone()]), scopes, ctx, engine)?;
            keyed.push((k, v));
        }
        keyed.sort_by(|a, b| {
            if cmp_err.is_some() {
                return Ordering::Equal;
            }
            match value_cmp(&a.0, &b.0) {
                Ok(ord) => ord,
                Err(e) => {
                    cmp_err = Some(e);
                    Ordering::Equal
                }
            }
        });
        if let Some(e) = cmp_err {
            return Err(vec![SourceDiagnostic::error(Span::detached(), e)]);
        }
        Ok(Value::Array(keyed.into_iter().map(|(_, v)| v).collect()))
    } else {
        sorted.sort_by(|a, b| {
            if cmp_err.is_some() {
                return Ordering::Equal;
            }
            match value_cmp(a, b) {
                Ok(ord) => ord,
                Err(e) => {
                    cmp_err = Some(e);
                    Ordering::Equal
                }
            }
        });
        if let Some(e) = cmp_err {
            return Err(vec![SourceDiagnostic::error(Span::detached(), e)]);
        }
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

fn str_len(s: EcoString) -> Value {
    Value::Int(s.chars().count() as i64)
}

fn str_first(s: EcoString) -> Value {
    s.chars()
        .next()
        .map(|c| Value::Str(c.to_string().into()))
        .unwrap_or(Value::None)
}

fn str_last(s: EcoString) -> Value {
    s.chars()
        .last()
        .map(|c| Value::Str(c.to_string().into()))
        .unwrap_or(Value::None)
}

fn str_at(s: EcoString, args: Args) -> SourceResult<Value> {
    let index = expect_one_int(args, "str.at()")?;
    let chars: Vec<char> = s.chars().collect();
    let idx = if index < 0 {
        (chars.len() as i64 + index) as usize
    } else {
        index as usize
    };
    if idx >= chars.len() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.at(): índice fora de limites".to_string(),
        )]);
    }
    Ok(Value::Str(chars[idx].to_string().into()))
}

fn str_slice(s: EcoString, args: Args) -> SourceResult<Value> {
    let mut positional = args.items.iter();
    let start = positional
        .next()
        .and_then(|v| v.cast_int())
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                "str.slice(): start espera int".to_string(),
            )]
        })?;
    let end_positional = positional.next().and_then(|v| v.cast_int());
    let end_named = args.named.get("end").and_then(|v| v.cast_int());
    let count = args.named.get("count").and_then(|v| v.cast_int());
    let end = end_positional.or(end_named);

    if end.is_some() && count.is_some() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.slice(): não pode especificar end e count simultaneamente".to_string(),
        )]);
    }

    let chars: Vec<char> = s.chars().collect();
    let start_idx = if start < 0 {
        (chars.len() as i64 + start) as usize
    } else {
        start as usize
    };
    let start_idx = start_idx.min(chars.len());
    let end_idx = match (end, count) {
        (Some(e), None) => {
            let e = if e < 0 {
                (chars.len() as i64 + e) as usize
            } else {
                e as usize
            };
            e.min(chars.len())
        }
        (None, Some(c)) => (start_idx + c.max(0) as usize).min(chars.len()),
        (None, None) => chars.len(),
        (Some(_), Some(_)) => unreachable!("end e count simultâneos já rejeitados acima"),
    };
    let result: String = chars[start_idx..end_idx.max(start_idx)].iter().collect();
    Ok(Value::Str(result.into()))
}

fn str_clusters(s: EcoString) -> Value {
    Value::Array(
        s.chars()
            .map(|c| Value::Str(c.to_string().into()))
            .collect(),
    )
}

/// **P689** — `str.codepoints()`: array de strings, um por char (scalar value).
/// Equivalente ao `clusters` simplificado do cristalino (paridade vanilla para
/// texto sem grapheme clusters multi-char).
fn str_codepoints(s: EcoString) -> Value {
    Value::Array(
        s.chars()
            .map(|c| Value::Str(c.to_string().into()))
            .collect(),
    )
}

/// **P689** — `str.position(hay)`: índice em **bytes** da primeira ocorrência,
/// ou `none`. Aceita `str` (substring literal) ou `regex` (primeiro match).
fn str_position(s: EcoString, args: Args) -> SourceResult<Value> {
    match args.items.as_slice() {
        [Value::Str(sub)] => Ok(s
            .find(sub.as_str())
            .map(|i| Value::Int(i as i64))
            .unwrap_or(Value::None)),
        [Value::Regex(re)] => Ok(re
            .captures_first(s.as_str())
            .map(|m| Value::Int(m.start as i64))
            .unwrap_or(Value::None)),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "str.position() espera str ou regex, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.position() requer 1 argumento posicional".to_string(),
        )]),
    }
}

/// **P689** — `str.match(pattern)`: primeiro match da regex, como dict
/// `{start, end, text, captures}` com índices em **bytes**, ou `none`.
/// Capturas em ordem posicional (grupos nomeados inclusive).
fn str_match(s: EcoString, args: Args) -> SourceResult<Value> {
    match args.items.as_slice() {
        [Value::Regex(re)] => match re.captures_first(s.as_str()) {
            Some(m) => {
                let mut dict: IndexMap<EcoString, Value, FxBuildHasher> =
                    IndexMap::default();
                dict.insert("start".into(), Value::Int(m.start as i64));
                dict.insert("end".into(), Value::Int(m.end as i64));
                dict.insert("text".into(), Value::Str(m.text.into()));
                dict.insert(
                    "captures".into(),
                    Value::Array(
                        m.captures
                            .into_iter()
                            .map(|c| Value::Str(c.into()))
                            .collect(),
                    ),
                );
                Ok(Value::Dict(dict))
            }
            None => Ok(Value::None),
        },
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.match() espera regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.match() requer 1 argumento posicional (regex)".to_string(),
        )]),
    }
}

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

fn str_to_upper(s: EcoString) -> Value {
    Value::Str(s.to_uppercase().into())
}

fn str_to_lower(s: EcoString) -> Value {
    Value::Str(s.to_lowercase().into())
}

fn str_to_unicode(s: EcoString) -> Value {
    Value::Array(s.chars().map(|c| Value::Int(c as i64)).collect())
}

fn str_rev(s: EcoString) -> Value {
    Value::Str(s.chars().rev().collect::<String>().into())
}

fn dict_insert(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    let (key, value) = expect_str_value(args, "dict.insert()")?;
    dict.insert(key, value);
    Ok(Value::Dict(dict))
}

fn dict_len(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Int(dict.len() as i64)
}

/// `dict.map((k, v) => ...)` — aplica função a cada par chave-valor.
fn dict_map(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "dict.map()")?;
    let mut result = IndexMap::with_capacity_and_hasher(dict.len(), FxBuildHasher);
    for (key, value) in dict {
        let mapped = apply_func(
            func.clone(),
            Args::positional(vec![Value::Str(key.clone()), value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        result.insert(key, mapped);
    }
    Ok(Value::Dict(result))
}

/// `dict.filter((k, v) => bool)` — mantém pares que satisfazem o predicado.
fn dict_filter(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "dict.filter()")?;
    let mut result = IndexMap::with_capacity_and_hasher(dict.len(), FxBuildHasher);
    for (key, value) in dict {
        let keep = apply_func(
            func.clone(),
            Args::positional(vec![Value::Str(key.clone()), value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        if keep.truthy() {
            result.insert(key, value);
        }
    }
    Ok(Value::Dict(result))
}

// ── argument helpers ────────────────────────────────────────────────────────

fn expect_str_value(args: Args, context: &str) -> SourceResult<(EcoString, Value)> {
    match args.items.as_slice() {
        [Value::Str(key), value] => Ok((key.clone(), value.clone())),
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
///
/// **P652** — devolve `Err` quando os valores não são comparáveis (tipos
/// incompatíveis ou `NaN`), em vez de assumir `Ordering::Equal` em silêncio.
fn value_cmp(a: &Value, b: &Value) -> Result<Ordering, EcoString> {
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
        (Value::Float(a), Value::Float(b)) => a
            .partial_cmp(b)
            .ok_or_else(|| "cannot compare floats".into()),
        (Value::Str(a), Value::Str(b)) => Ok(a.cmp(b)),
        _ => Err(format!(
            "cannot compare {} and {}",
            a.type_name(),
            b.type_name()
        )
        .into()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_args(items: Vec<Value>, named: Option<(&str, Value)>) -> Args {
        let mut args = Args { items, named: IndexMap::default() };
        if let Some((k, v)) = named {
            args.named.insert(k.into(), v);
        }
        args
    }

    #[test]
    fn p495_dict_at_default_named() {
        let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        dict.insert("a".into(), Value::Int(1));
        dict.insert("b".into(), Value::Int(2));

        // key existente → valor
        let args = make_args(vec![Value::Str("a".into())], None);
        assert_eq!(dict_at(dict.clone(), args).unwrap(), Value::Int(1));

        // key ausente com default → default
        let args = make_args(
            vec![Value::Str("z".into())],
            Some(("default", Value::Int(99))),
        );
        assert_eq!(dict_at(dict.clone(), args).unwrap(), Value::Int(99));

        // key ausente sem default → erro
        let args = make_args(vec![Value::Str("z".into())], None);
        assert!(dict_at(dict, args).is_err());
    }

    #[test]
    fn p495_dict_at_named_desconhecido_rejeitado() {
        let dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
        let mut args = make_args(vec![Value::Str("a".into())], None);
        args.named.insert("foo".into(), Value::Int(1));
        assert!(dict_at(dict, args).is_err());
    }

    // ── P496 — métodos estruturais de array ───────────────────────────────────

    #[test]
    fn p496_array_dedup_remove_duplicados_adjacentes() {
        let arr = vec![
            Value::Int(3), Value::Int(1), Value::Int(4), Value::Int(1),
            Value::Int(5), Value::Int(9), Value::Int(2), Value::Int(6),
        ];
        assert_eq!(
            array_dedup(arr),
            Value::Array(vec![
                Value::Int(3), Value::Int(1), Value::Int(4), Value::Int(1),
                Value::Int(5), Value::Int(9), Value::Int(2), Value::Int(6),
            ])
        );

        let arr2 = vec![
            Value::Int(1), Value::Int(1), Value::Int(2), Value::Int(2),
            Value::Int(2), Value::Int(3),
        ];
        assert_eq!(
            array_dedup(arr2),
            Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
    }

    #[test]
    fn p496_array_chunks_divide_em_blocos() {
        let arr = vec![
            Value::Int(3), Value::Int(1), Value::Int(4), Value::Int(1),
            Value::Int(5), Value::Int(9), Value::Int(2), Value::Int(6),
        ];
        let result = array_chunks(arr, make_args(vec![Value::Int(3)], None)).unwrap();
        let expected = Value::Array(vec![
            Value::Array(vec![Value::Int(3), Value::Int(1), Value::Int(4)]),
            Value::Array(vec![Value::Int(1), Value::Int(5), Value::Int(9)]),
            Value::Array(vec![Value::Int(2), Value::Int(6)]),
        ]);
        assert_eq!(result, expected);
    }

    #[test]
    fn p496_array_windows_janelas_deslizantes() {
        let arr = vec![
            Value::Int(3), Value::Int(1), Value::Int(4), Value::Int(1),
        ];
        let result = array_windows(arr, make_args(vec![Value::Int(3)], None)).unwrap();
        let expected = Value::Array(vec![
            Value::Array(vec![Value::Int(3), Value::Int(1), Value::Int(4)]),
            Value::Array(vec![Value::Int(1), Value::Int(4), Value::Int(1)]),
        ]);
        assert_eq!(result, expected);
    }

    // ── P689 — str.codepoints / str.position / str.match ────────────────────

    #[test]
    fn p689_str_codepoints_chars() {
        assert_eq!(
            str_codepoints("abc".into()),
            Value::Array(vec![
                Value::Str("a".into()),
                Value::Str("b".into()),
                Value::Str("c".into()),
            ])
        );
        // multibyte (café precomposto) → 4 codepoints, paridade vanilla
        assert_eq!(
            str_codepoints("café".into()),
            Value::Array(vec![
                Value::Str("c".into()),
                Value::Str("a".into()),
                Value::Str("f".into()),
                Value::Str("é".into()),
            ])
        );
        assert_eq!(str_codepoints("".into()), Value::Array(vec![]));
    }

    #[test]
    fn p689_str_position_str_e_regex_byte_indices() {
        // str: índice em bytes; não encontrado → none
        let a = make_args(vec![Value::Str("b".into())], None);
        assert_eq!(str_position("abc".into(), a).unwrap(), Value::Int(1));
        let a = make_args(vec![Value::Str("z".into())], None);
        assert_eq!(str_position("abc".into(), a).unwrap(), Value::None);
        // multibyte: "xéy" — y no byte 3 (x=1, é=2)
        let a = make_args(vec![Value::Str("y".into())], None);
        assert_eq!(str_position("xéy".into(), a).unwrap(), Value::Int(3));
        // regex: paridade — também byte index
        let a = make_args(vec![Value::Regex(Regex::new("y").unwrap())], None);
        assert_eq!(str_position("xéy".into(), a).unwrap(), Value::Int(3));
        let a = make_args(vec![Value::Regex(Regex::new("z").unwrap())], None);
        assert_eq!(str_position("abc".into(), a).unwrap(), Value::None);
    }

    #[test]
    fn p689_str_match_dict_e_captures() {
        // sem match → none
        let a = make_args(vec![Value::Regex(Regex::new("z").unwrap())], None);
        assert_eq!(str_match("abc".into(), a).unwrap(), Value::None);

        // match simples → dict {start, end, text, captures}
        let a = make_args(vec![Value::Regex(Regex::new("b").unwrap())], None);
        let d = match str_match("abc".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(d.get("start"), Some(&Value::Int(1)));
        assert_eq!(d.get("end"), Some(&Value::Int(2)));
        assert_eq!(d.get("text"), Some(&Value::Str("b".into())));
        assert_eq!(d.get("captures"), Some(&Value::Array(vec![])));

        // multibyte: "xéy" match é → start 1, end 3 (bytes)
        let a = make_args(vec![Value::Regex(Regex::new("é").unwrap())], None);
        let d = match str_match("xéy".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(d.get("start"), Some(&Value::Int(1)));
        assert_eq!(d.get("end"), Some(&Value::Int(3)));

        // captures posicionais (inclui grupos nomeados em ordem)
        let a = make_args(vec![Value::Regex(Regex::new("(a)(b)(c)").unwrap())], None);
        let d = match str_match("abc".into(), a).unwrap() {
            Value::Dict(d) => d,
            other => panic!("esperado dict, recebeu {:?}", other),
        };
        assert_eq!(
            d.get("captures"),
            Some(&Value::Array(vec![
                Value::Str("a".into()),
                Value::Str("b".into()),
                Value::Str("c".into()),
            ]))
        );
    }

    #[test]
    fn p689_str_position_match_tipo_errado() {
        // position com Int → erro
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_position("abc".into(), a).is_err());
        // match com Str (não regex) → erro
        let a = make_args(vec![Value::Str("b".into())], None);
        assert!(str_match("abc".into(), a).is_err());
    }

}
