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
use unicode_normalization::UnicodeNormalization;

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
        (Value::Array(arr), "at") => Some(array_at(arr, args)),
        (Value::Array(arr), "slice") => Some(array_slice(arr, args)),
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
        (Value::Str(s), "char-len") => Some(Ok(char_len(s))),
        (Value::Str(s), "char-at") => Some(char_at(s, args)),
        (Value::Str(s), "char-slice") => Some(char_slice(s, args)),
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
        (Value::Str(s), "normalize") => Some(str_normalize(s, args)),
        (Value::Str(s), "position") => Some(str_position(s, args)),
        (Value::Str(s), "match") => Some(str_match(s, args)),
        (Value::Str(s), "matches") => Some(str_matches(s, args)),

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

/// `array.at(index, default: value)` — P714. Paridade com o vanilla
/// (`foundations/array.rs:207-221`, `locate_opt`): índice negativo conta a
/// partir do fim (`len + index`); fora de limites usa `default` se
/// fornecido, senão erro (mesma mensagem do vanilla,
/// `out_of_bounds_no_default`).
fn array_at(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    if args.named.len() > 1 || (args.named.len() == 1 && !args.named.contains_key("default")) {
        let bad = args.named.keys().find(|k| k.as_str() != "default")
            .map(|k| k.as_str()).unwrap_or("?");
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("array.at() argumento nomeado desconhecido: '{bad}'"),
        )]);
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

fn array_rev(arr: Vec<Value>) -> Value {
    Value::Array(arr.into_iter().rev().collect())
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
fn array_slice(arr: Vec<Value>, args: Args) -> SourceResult<Value> {
    let mut positional = args.items.iter();
    let start = positional
        .next()
        .and_then(|v| v.cast_int())
        .ok_or_else(|| {
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
    // Paridade vanilla (P690): `str.len()` conta bytes UTF-8, não chars.
    Value::Int(s.len() as i64)
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
    // Paridade vanilla (P690): índice em bytes. Negativo conta bytes do fim.
    // Erro se fora de limites ou se o índice não for fronteira de carácter.
    let index = expect_one_int(args, "str.at()")?;
    let st = s.as_str();
    let len = st.len() as i64;
    let idx = if index < 0 { len + index } else { index };
    if idx < 0 || idx >= len {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.at(): índice fora de limites (índice {index}, len {len})"),
        )]);
    }
    let i = idx as usize;
    if !st.is_char_boundary(i) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.at(): índice {index} não é uma fronteira de carácter"),
        )]);
    }
    let ch = st[i..].chars().next().ok_or_else(|| {
        vec![SourceDiagnostic::error(
            Span::detached(),
            "str.at(): índice fora de limites".to_string(),
        )]
    })?;
    Ok(Value::Str(ch.to_string().into()))
}

fn str_slice(s: EcoString, args: Args) -> SourceResult<Value> {
    // Paridade vanilla (P690): índices em bytes. Sem clamp — índice fora de
    // limites ou fora de fronteira de carácter é erro. `start > end` → "".
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

    let st = s.as_str();
    let len = st.len() as i64;
    let resolve = |x: i64| if x < 0 { len + x } else { x };
    let start_idx = resolve(start);
    let end_idx = match (end, count) {
        (Some(e), None) => resolve(e),
        (None, Some(c)) => start_idx + c.max(0),
        (None, None) => len,
        (Some(_), Some(_)) => unreachable!("end e count simultâneos já rejeitados acima"),
    };

    if start_idx < 0 || start_idx > len {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.slice(): índice fora de limites (start {start}, len {len})"),
        )]);
    }
    if end_idx < 0 || end_idx > len {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!("str.slice(): índice fora de limites (end, len {len})"),
        )]);
    }
    let si = start_idx as usize;
    let ei = end_idx as usize;
    if !st.is_char_boundary(si) || !st.is_char_boundary(ei) {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.slice(): índice não é uma fronteira de carácter".to_string(),
        )]);
    }
    if si > ei {
        return Ok(Value::Str(EcoString::new()));
    }
    Ok(Value::Str(st[si..ei].into()))
}

/// **P690** — `str.char-len()`: número de chars (codepoints). Extensão cristalina
/// não-portável; preserva a convenção pré-P690 de `str.len()`.
fn char_len(s: EcoString) -> Value {
    Value::Int(s.chars().count() as i64)
}

/// **P690** — `str.char-at(index)`: char no índice em chars. Extensão cristalina
/// não-portável; preserva a convenção pré-P690 de `str.at()`.
fn char_at(s: EcoString, args: Args) -> SourceResult<Value> {
    let index = expect_one_int(args, "str.char-at()")?;
    let chars: Vec<char> = s.chars().collect();
    let idx = if index < 0 {
        (chars.len() as i64 + index) as usize
    } else {
        index as usize
    };
    if idx >= chars.len() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.char-at(): índice fora de limites".to_string(),
        )]);
    }
    Ok(Value::Str(chars[idx].to_string().into()))
}

/// **P690** — `str.char-slice(start, end?, count?)`: substring em chars (clamp aos
/// limites). Extensão cristalina não-portável; preserva a convenção pré-P690 de
/// `str.slice()`.
fn char_slice(s: EcoString, args: Args) -> SourceResult<Value> {
    let mut positional = args.items.iter();
    let start = positional
        .next()
        .and_then(|v| v.cast_int())
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                "str.char-slice(): start espera int".to_string(),
            )]
        })?;
    let end_positional = positional.next().and_then(|v| v.cast_int());
    let end_named = args.named.get("end").and_then(|v| v.cast_int());
    let count = args.named.get("count").and_then(|v| v.cast_int());
    let end = end_positional.or(end_named);

    if end.is_some() && count.is_some() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.char-slice(): não pode especificar end e count simultaneamente".to_string(),
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

/// Constrói o dict `{start, end, text, captures}` de um match (índices em bytes).
/// Partilhado por `str.match` (P689) e `str.matches` (P692).
fn match_dict(start: usize, end: usize, text: &str, captures: Vec<String>) -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("start".into(), Value::Int(start as i64));
    dict.insert("end".into(), Value::Int(end as i64));
    dict.insert("text".into(), Value::Str(text.to_string().into()));
    dict.insert(
        "captures".into(),
        Value::Array(captures.into_iter().map(|c| Value::Str(c.into())).collect()),
    );
    Value::Dict(dict)
}

/// **P689** — `str.match(pattern)`: primeiro match, como dict
/// `{start, end, text, captures}` com índices em **bytes**, ou `none`.
/// Aceita `str` (literal; `captures` vazio) ou `regex` (capturas em ordem
/// posicional, grupos nomeados inclusive). (**P693**: alargado de `regex`-only
/// para `str | regex`, espelhando `str_matches`.)
fn str_match(s: EcoString, args: Args) -> SourceResult<Value> {
    match args.items.as_slice() {
        [Value::Str(pat)] => Ok(s
            .match_indices(pat.as_str())
            .next()
            .map(|(i, m)| match_dict(i, i + m.len(), m, vec![]))
            .unwrap_or(Value::None)),
        [Value::Regex(re)] => Ok(re
            .captures_first(s.as_str())
            .map(|m| match_dict(m.start, m.end, &m.text, m.captures))
            .unwrap_or(Value::None)),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "str.match() espera str ou regex, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.match() requer 1 argumento posicional".to_string(),
        )]),
    }
}

/// **P692** — `str.matches(pattern)`: array de dicts `{start, end, text, captures}`
/// (índices em **bytes**), um por ocorrência não sobreposta; `[]` se nenhuma.
/// Aceita `str` (literal, via `match_indices`) ou `regex` (via `captures_all`).
fn str_matches(s: EcoString, args: Args) -> SourceResult<Value> {
    match args.items.as_slice() {
        [Value::Str(pat)] => {
            let arr: Vec<Value> = s
                .match_indices(pat.as_str())
                .map(|(i, m)| match_dict(i, i + m.len(), m, vec![]))
                .collect();
            Ok(Value::Array(arr))
        }
        [Value::Regex(re)] => {
            let arr: Vec<Value> = re
                .captures_all(s.as_str())
                .into_iter()
                .map(|m| match_dict(m.start, m.end, &m.text, m.captures))
                .collect();
            Ok(Value::Array(arr))
        }
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "str.matches() espera str ou regex, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.matches() requer 1 argumento posicional".to_string(),
        )]),
    }
}

/// **P692** — `str.normalize(form:)`: normalização Unicode. `form` (named) ∈
/// {"nfc", "nfd", "nfkc", "nfkd"}; default "nfc". Não aceita argumentos posicionais.
fn str_normalize(s: EcoString, args: Args) -> SourceResult<Value> {
    if !args.items.is_empty() {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.normalize(): não aceita argumentos posicionais (use form:)".to_string(),
        )]);
    }
    let form = match args.named.get("form") {
        None => "nfc".to_string(),
        Some(Value::Str(f)) => f.to_string(),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "str.normalize(): form espera str, recebeu {}",
                    other.type_name()
                ),
            )])
        }
    };
    let normalized: String = match form.as_str() {
        "nfc" => s.nfc().collect(),
        "nfd" => s.nfd().collect(),
        "nfkc" => s.nfkc().collect(),
        "nfkd" => s.nfkd().collect(),
        other => {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "str.normalize(): forma desconhecida '{other}' (esperado nfc, nfd, nfkc, nfkd)"
                ),
            )])
        }
    };
    Ok(Value::Str(normalized.into()))
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
    // Paridade vanilla (P691): devolve a **substring** encontrada (texto do
    // primeiro match), ou `none`. Aceita `str` (substring literal) ou `regex`
    // (texto do match). O índice da ocorrência continua disponível via
    // `str.position()`.
    match args.items.as_slice() {
        [Value::Str(sub)] => Ok(s
            .find(sub.as_str())
            .map(|i| Value::Str(s[i..i + sub.len()].to_string().into()))
            .unwrap_or(Value::None)),
        [Value::Regex(re)] => Ok(re
            .captures_first(s.as_str())
            .map(|m| Value::Str(m.text.into()))
            .unwrap_or(Value::None)),
        [other] => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            format!(
                "str.find() espera str ou regex, recebeu {}",
                other.type_name()
            ),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.find() requer 1 argumento posicional".to_string(),
        )]),
    }
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

    // ── P714 — array.at(index, default:) ──────────────────────────────────────

    #[test]
    fn p714_array_at_indice_positivo() {
        let arr = vec![Value::Int(10), Value::Int(20), Value::Int(30)];
        let args = make_args(vec![Value::Int(1)], None);
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(20));
    }

    #[test]
    fn p714_array_at_indice_negativo_conta_do_fim() {
        // Paridade `locate_opt`: -1 -> len + (-1) = último elemento.
        let arr = vec![Value::Int(10), Value::Int(20), Value::Int(30)];
        let args = make_args(vec![Value::Int(-1)], None);
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(30));

        let arr = vec![Value::Int(10), Value::Int(20), Value::Int(30)];
        let args = make_args(vec![Value::Int(-3)], None);
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(10));
    }

    #[test]
    fn p714_array_at_fora_de_limites_com_default() {
        let arr = vec![Value::Int(10), Value::Int(20)];
        let args = make_args(vec![Value::Int(5)], Some(("default", Value::Int(99))));
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(99));

        // Negativo além do início também usa default.
        let arr = vec![Value::Int(10), Value::Int(20)];
        let args = make_args(vec![Value::Int(-5)], Some(("default", Value::Int(-1))));
        assert_eq!(array_at(arr, args).unwrap(), Value::Int(-1));
    }

    #[test]
    fn p714_array_at_fora_de_limites_sem_default_erra() {
        let arr = vec![Value::Int(10), Value::Int(20)];
        let args = make_args(vec![Value::Int(5)], None);
        let err = array_at(arr, args).unwrap_err();
        assert!(
            err[0].message.contains("out of bounds") && err[0].message.contains("no default"),
            "mensagem inesperada: {:?}", err[0].message
        );
    }

    #[test]
    fn p714_array_at_named_desconhecido_rejeitado() {
        let arr = vec![Value::Int(1)];
        let mut args = make_args(vec![Value::Int(0)], None);
        args.named.insert("foo".into(), Value::Int(1));
        assert!(array_at(arr, args).is_err());
    }

    #[test]
    fn p714_array_at_vazio_sem_default_erra() {
        let arr: Vec<Value> = vec![];
        let args = make_args(vec![Value::Int(0)], None);
        assert!(array_at(arr, args).is_err());
    }

    // ── P730 — array.slice(start, end?, count:) ──────────────────────────────
    // Paridade vanilla `foundations/array.rs:279-300` (sobre
    // `locate(index, end_ok: true)`). Medições vanilla no corpo dos testes.

    fn ints(v: &[i64]) -> Vec<Value> {
        v.iter().map(|&i| Value::Int(i)).collect()
    }

    #[test]
    fn p730_array_slice_start_end() {
        // Vanilla: (1,2,3,4).slice(1, 3) → (2, 3)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(1), Value::Int(3)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[2, 3])));
    }

    #[test]
    fn p730_array_slice_so_start() {
        // Vanilla: (1,2,3,4).slice(1) → (2, 3, 4)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(1)], None);
        assert_eq!(
            array_slice(arr, args).unwrap(),
            Value::Array(ints(&[2, 3, 4]))
        );
    }

    #[test]
    fn p730_array_slice_negativos_contam_do_fim() {
        // Vanilla: (1,2,3,4).slice(-2) → (3, 4); .slice(-3, -1) → (2, 3)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(-2)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[3, 4])));

        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(-3), Value::Int(-1)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[2, 3])));
    }

    #[test]
    fn p730_array_slice_count() {
        // Vanilla: (1,2,3,4).slice(0, count: 2) → (1, 2)
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(0)], Some(("count", Value::Int(2))));
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(ints(&[1, 2])));
    }

    #[test]
    fn p730_array_slice_count_negativo_vazio() {
        // Vanilla: (1,2,3,4).slice(1, count: -1) → ()
        // (end efetivo = 1 + (-1) = 0 → locate ok → max(start) → [1..1])
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(1)], Some(("count", Value::Int(-1))));
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn p730_array_slice_end_menor_que_start_vazio() {
        // Vanilla: (1,2,3,4).slice(3, 1) → (); .slice(4) → (); .slice(0, 4) → tudo
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(3), Value::Int(1)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));

        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(4)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));

        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(0), Value::Int(4)], None);
        assert_eq!(
            array_slice(arr, args).unwrap(),
            Value::Array(ints(&[1, 2, 3, 4]))
        );
    }

    #[test]
    fn p730_array_slice_array_vazio() {
        // Vanilla: ().slice(0) → ()
        let arr: Vec<Value> = vec![];
        let args = make_args(vec![Value::Int(0)], None);
        assert_eq!(array_slice(arr, args).unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn p730_array_slice_end_e_count_mutuamente_exclusivos() {
        // Vanilla: "`end` and `count` are mutually exclusive"
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(
            vec![Value::Int(1), Value::Int(2)],
            Some(("count", Value::Int(2))),
        );
        let err = array_slice(arr, args).unwrap_err();
        assert!(
            err[0].message.contains("mutually exclusive"),
            "msg: {}",
            err[0].message
        );
    }

    #[test]
    fn p730_array_slice_fora_de_limites_erra() {
        // Vanilla: "array index out of bounds (index: 10, len: 4)"
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(10)], None);
        let err = array_slice(arr, args).unwrap_err();
        assert!(
            err[0].message.contains("out of bounds (index: 10, len: 4)"),
            "msg: {}",
            err[0].message
        );
    }

    #[test]
    fn p730_array_slice_count_fora_de_limites_erra() {
        // Vanilla: (1,2,3,4).slice(0, count: 99) →
        // "array index out of bounds (index: 99, len: 4)"
        let arr = ints(&[1, 2, 3, 4]);
        let args = make_args(vec![Value::Int(0)], Some(("count", Value::Int(99))));
        let err = array_slice(arr, args).unwrap_err();
        assert!(
            err[0].message.contains("out of bounds (index: 99, len: 4)"),
            "msg: {}",
            err[0].message
        );
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
        // match com Int (tipo errado) → erro (P693: match aceita str | regex; Int continua a ser rejeitado)
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_match("abc".into(), a).is_err());
    }

    // ── P690 — str.len/at/slice em bytes; char-len/char-at/char-slice ─────────

    #[test]
    fn p690_str_len_bytes() {
        // ASCII: char == byte, sem efeito visível
        assert_eq!(str_len("abc".into()), Value::Int(3));
        assert_eq!(str_len("".into()), Value::Int(0));
        // multi-byte: "café" = 5 bytes (c-a-f-é, é ocupa 2)
        assert_eq!(str_len("café".into()), Value::Int(5));
        assert_eq!(str_len("éabc".into()), Value::Int(5));
    }

    #[test]
    fn p690_str_at_bytes() {
        // ASCII
        assert_eq!(
            str_at("abc".into(), make_args(vec![Value::Int(1)], None)).unwrap(),
            Value::Str("b".into())
        );
        // multi-byte: "éabc" (é=0-1, a=2, b=3, c=4) → byte 2 = 'a'
        assert_eq!(
            str_at("éabc".into(), make_args(vec![Value::Int(2)], None)).unwrap(),
            Value::Str("a".into())
        );
        assert_eq!(
            str_at("éabc".into(), make_args(vec![Value::Int(0)], None)).unwrap(),
            Value::Str("é".into())
        );
        // negativo conta bytes do fim: "café" (c=0,a=1,f=2,é=3-4), -1 → byte 4
        // (2º byte do 'é', non-boundary) → erro
        assert!(str_at("café".into(), make_args(vec![Value::Int(-1)], None)).is_err());
        // non-boundary: byte 1 de "éabc" (meio do 'é') → erro
        assert!(str_at("éabc".into(), make_args(vec![Value::Int(1)], None)).is_err());
        // out of bounds: índice >= len bytes
        assert!(str_at("éabc".into(), make_args(vec![Value::Int(10)], None)).is_err());
        // negativo além do início
        assert!(str_at("éabc".into(), make_args(vec![Value::Int(-6)], None)).is_err());
    }

    #[test]
    fn p690_str_slice_bytes() {
        // ASCII
        assert_eq!(
            str_slice("abcdef".into(), make_args(vec![Value::Int(1), Value::Int(4)], None))
                .unwrap(),
            Value::Str("bcd".into())
        );
        // multi-byte: "éabc".slice(2,4) bytes = "ab"
        assert_eq!(
            str_slice("éabc".into(), make_args(vec![Value::Int(2), Value::Int(4)], None))
                .unwrap(),
            Value::Str("ab".into())
        );
        // count em bytes: "éabc".slice(0, count: 2) = "é"
        assert_eq!(
            str_slice("éabc".into(), make_args(vec![Value::Int(0)], Some(("count", Value::Int(2)))))
                .unwrap(),
            Value::Str("é".into())
        );
        // negativo: "éabc".slice(-3, -1) → bytes 2..4 = "ab"
        assert_eq!(
            str_slice("éabc".into(), make_args(vec![Value::Int(-3), Value::Int(-1)], None))
                .unwrap(),
            Value::Str("ab".into())
        );
        // start > end → ""
        assert_eq!(
            str_slice("abcdef".into(), make_args(vec![Value::Int(4), Value::Int(2)], None))
                .unwrap(),
            Value::Str("".into())
        );
        // end + count simultâneos → erro
        assert!(str_slice(
            "abc".into(),
            {
                let mut a = make_args(vec![Value::Int(0), Value::Int(2)], None);
                a.named.insert("count".into(), Value::Int(1));
                a
            }
        )
        .is_err());
        // out of bounds (sem clamp): end > len → erro
        assert!(str_slice("éabc".into(), make_args(vec![Value::Int(0), Value::Int(100)], None))
            .is_err());
        // negativo além do início → erro
        assert!(str_slice("éabc".into(), make_args(vec![Value::Int(-10), Value::Int(2)], None))
            .is_err());
        // non-boundary → erro
        assert!(str_slice("éabc".into(), make_args(vec![Value::Int(1), Value::Int(4)], None))
            .is_err());
    }

    #[test]
    fn p690_str_at_position_consistencia() {
        // A prova: combinar position (bytes) com at (bytes) sobre texto não-ASCII
        // produz o carácter correcto. Pré-P690, at era char → dava "a".
        let s: EcoString = "café mais texto".into();
        let pos = str_position(s.clone(), make_args(vec![Value::Str("m".into())], None)).unwrap();
        assert_eq!(pos, Value::Int(6)); // byte 6 (c-a-f-é-space-m)
        let ch = str_at(s, make_args(vec![pos], None)).unwrap();
        assert_eq!(ch, Value::Str("m".into()));
    }

    #[test]
    fn p690_char_len_at_slice_preserva_chars() {
        // char-len conta chars
        assert_eq!(char_len("café".into()), Value::Int(4));
        assert_eq!(char_len("abc".into()), Value::Int(3));
        // char-at indexa por char: "éabc" char 2 = 'b'
        assert_eq!(
            char_at("éabc".into(), make_args(vec![Value::Int(2)], None)).unwrap(),
            Value::Str("b".into())
        );
        // char-at negativo do fim: "café" char -1 = 'é'
        assert_eq!(
            char_at("café".into(), make_args(vec![Value::Int(-1)], None)).unwrap(),
            Value::Str("é".into())
        );
        assert!(char_at("éabc".into(), make_args(vec![Value::Int(10)], None)).is_err());
        // char-slice por char: "éabc".char-slice(2, 4) = "bc"
        assert_eq!(
            char_slice("éabc".into(), make_args(vec![Value::Int(2), Value::Int(4)], None))
                .unwrap(),
            Value::Str("bc".into())
        );
        // char-slice com count e clamp (comportamento pré-P690)
        assert_eq!(
            char_slice("éabc".into(), make_args(vec![Value::Int(0)], Some(("count", Value::Int(2)))))
                .unwrap(),
            Value::Str("éa".into())
        );
    }

    // ── P691 — str.find devolve substring / none (str ou regex) ───────────────

    #[test]
    fn p691_str_find_substring_e_none() {
        // substring literal encontrada → devolve a substring
        let a = make_args(vec![Value::Str("mais".into())], None);
        assert_eq!(
            str_find("café mais texto".into(), a).unwrap(),
            Value::Str("mais".into())
        );
        // multi-byte: "xéy".find("é") → "é"
        let a = make_args(vec![Value::Str("é".into())], None);
        assert_eq!(str_find("xéy".into(), a).unwrap(), Value::Str("é".into()));
        let a = make_args(vec![Value::Str("fé".into())], None);
        assert_eq!(str_find("café".into(), a).unwrap(), Value::Str("fé".into()));
        // não encontrado → none
        let a = make_args(vec![Value::Str("inexistente".into())], None);
        assert_eq!(str_find("café mais texto".into(), a).unwrap(), Value::None);
        // string vazia → "" (paridade vanilla)
        let a = make_args(vec![Value::Str("".into())], None);
        assert_eq!(str_find("abc".into(), a).unwrap(), Value::Str("".into()));
    }

    #[test]
    fn p691_str_find_regex() {
        // regex → texto do primeiro match
        let a = make_args(vec![Value::Regex(Regex::new("m..s").unwrap())], None);
        assert_eq!(
            str_find("café mais texto".into(), a).unwrap(),
            Value::Str("mais".into())
        );
        // regex sem match → none
        let a = make_args(vec![Value::Regex(Regex::new("z+").unwrap())], None);
        assert_eq!(str_find("café".into(), a).unwrap(), Value::None);
    }

    #[test]
    fn p691_str_find_tipo_e_aridade() {
        // tipo errado (Int) → erro
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_find("abc".into(), a).is_err());
        // aridade errada (0 args) → erro
        let a = make_args(vec![], None);
        assert!(str_find("abc".into(), a).is_err());
    }

    // ── P692 — str.matches / str.normalize ────────────────────────────────────

    fn dict_get<'a>(v: &'a Value, key: &str) -> &'a Value {
        match v {
            Value::Dict(d) => d.get(key).unwrap(),
            other => panic!("esperado dict, recebeu {:?}", other),
        }
    }

    #[test]
    fn p692_str_matches_regex() {
        let a = make_args(vec![Value::Regex(Regex::new(r"\w+").unwrap())], None);
        let arr = match str_matches("um dois três quatro".into(), a).unwrap() {
            Value::Array(a) => a,
            other => panic!("esperado array, recebeu {:?}", other),
        };
        assert_eq!(arr.len(), 4);
        assert_eq!(dict_get(&arr[0], "start"), &Value::Int(0));
        assert_eq!(dict_get(&arr[0], "end"), &Value::Int(2));
        assert_eq!(dict_get(&arr[0], "text"), &Value::Str("um".into()));
        // "três" começa no byte 8 (dois=3..7, espaço=7, três=8..13 — ê ocupa 2 bytes)
        assert_eq!(dict_get(&arr[2], "start"), &Value::Int(8));
        assert_eq!(dict_get(&arr[2], "end"), &Value::Int(13));
        assert_eq!(dict_get(&arr[2], "text"), &Value::Str("três".into()));
    }

    #[test]
    fn p692_str_matches_str_literal_e_vazio() {
        // literal: "abab".matches("ab") → 2 matches, captures vazias
        let a = make_args(vec![Value::Str("ab".into())], None);
        let arr = match str_matches("abab".into(), a).unwrap() {
            Value::Array(a) => a,
            other => panic!("esperado array, recebeu {:?}", other),
        };
        assert_eq!(arr.len(), 2);
        assert_eq!(dict_get(&arr[0], "start"), &Value::Int(0));
        assert_eq!(dict_get(&arr[1], "start"), &Value::Int(2));
        assert_eq!(dict_get(&arr[0], "captures"), &Value::Array(vec![]));
        // sem match → array vazio
        let a = make_args(vec![Value::Regex(Regex::new("z").unwrap())], None);
        assert_eq!(str_matches("abc".into(), a).unwrap(), Value::Array(vec![]));
    }

    #[test]
    fn p692_str_matches_captures() {
        let a = make_args(vec![Value::Regex(Regex::new(r"([a-z])(\d)").unwrap())], None);
        let arr = match str_matches("a1b2".into(), a).unwrap() {
            Value::Array(a) => a,
            other => panic!("esperado array, recebeu {:?}", other),
        };
        assert_eq!(arr.len(), 2);
        assert_eq!(dict_get(&arr[0], "text"), &Value::Str("a1".into()));
        assert_eq!(
            dict_get(&arr[0], "captures"),
            &Value::Array(vec![Value::Str("a".into()), Value::Str("1".into())])
        );
        assert_eq!(
            dict_get(&arr[1], "captures"),
            &Value::Array(vec![Value::Str("b".into()), Value::Str("2".into())])
        );
    }

    #[test]
    fn p692_str_normalize_forms() {
        let none = make_args(vec![], None);
        let nfc = make_args(vec![], Some(("form", Value::Str("nfc".into()))));
        let nfd = make_args(vec![], Some(("form", Value::Str("nfd".into()))));
        let nfkc = make_args(vec![], Some(("form", Value::Str("nfkc".into()))));
        let nfkd = make_args(vec![], Some(("form", Value::Str("nfkd".into()))));
        // default == nfc → "café" composto (5 bytes)
        let def = str_normalize("café".into(), none).unwrap();
        assert_eq!(def, Value::Str("café".into()));
        assert_eq!(def, str_normalize("café".into(), nfc).unwrap());
        // nfd → "cafe" + combining acute (5 chars, 6 bytes)
        let d = str_normalize("café".into(), nfd).unwrap();
        assert_eq!(d, Value::Str("cafe\u{301}".into()));
        assert_eq!(str_len(match &d { Value::Str(s) => s.clone(), _ => panic!() }), Value::Int(6));
        // nfkc == nfc para "café"; nfkd == nfd
        assert_eq!(str_normalize("café".into(), nfkc).unwrap(), Value::Str("café".into()));
        assert_eq!(str_normalize("café".into(), nfkd).unwrap(), Value::Str("cafe\u{301}".into()));
    }

    #[test]
    fn p692_str_normalize_erros() {
        // forma desconhecida → erro
        let a = make_args(vec![], Some(("form", Value::Str("nfx".into()))));
        assert!(str_normalize("café".into(), a).is_err());
        // form com tipo errado → erro
        let a = make_args(vec![], Some(("form", Value::Int(1))));
        assert!(str_normalize("café".into(), a).is_err());
        // argumento posicional → erro
        let a = make_args(vec![Value::Str("nfc".into())], None);
        assert!(str_normalize("café".into(), a).is_err());
    }

    // ── P693 — str.match aceita str | regex ───────────────────────────────────

    #[test]
    fn p693_str_match_str_literal() {
        // "abcabc".match("bc") → primeiro "bc" em start 1, end 3, captures vazio
        let a = make_args(vec![Value::Str("bc".into())], None);
        let d = str_match("abcabc".into(), a).unwrap();
        assert_eq!(dict_get(&d, "start"), &Value::Int(1));
        assert_eq!(dict_get(&d, "end"), &Value::Int(3));
        assert_eq!(dict_get(&d, "text"), &Value::Str("bc".into()));
        assert_eq!(dict_get(&d, "captures"), &Value::Array(vec![]));

        // não encontrado → none
        let a = make_args(vec![Value::Str("z".into())], None);
        assert_eq!(str_match("abc".into(), a).unwrap(), Value::None);

        // string vazia → match no início (start 0, end 0, text "")
        let a = make_args(vec![Value::Str("".into())], None);
        let d = str_match("abc".into(), a).unwrap();
        assert_eq!(dict_get(&d, "start"), &Value::Int(0));
        assert_eq!(dict_get(&d, "end"), &Value::Int(0));
        assert_eq!(dict_get(&d, "text"), &Value::Str("".into()));
    }

    #[test]
    fn p693_str_match_regex_sem_regressao() {
        // regex com capturas continua a funcionar (paridade P689)
        let a = make_args(vec![Value::Regex(Regex::new(r"(a)(b)").unwrap())], None);
        let d = str_match("xab".into(), a).unwrap();
        assert_eq!(dict_get(&d, "start"), &Value::Int(1));
        assert_eq!(dict_get(&d, "text"), &Value::Str("ab".into()));
        assert_eq!(
            dict_get(&d, "captures"),
            &Value::Array(vec![Value::Str("a".into()), Value::Str("b".into())])
        );
        // tipo errado (Int) → erro
        let a = make_args(vec![Value::Int(1)], None);
        assert!(str_match("abc".into(), a).is_err());
    }

}
