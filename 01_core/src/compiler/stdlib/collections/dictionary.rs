//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/collections/dictionary.md
//! @prompt-hash 64eaeac8
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::dictionary`.

use super::*;

pub(super) fn dict_keys(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(dict.into_keys().map(Value::Str).collect())
}

pub(super) fn dict_values(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(dict.into_values().collect())
}

pub(super) fn dict_pairs(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Array(
        dict.into_iter()
            .map(|(k, v)| Value::Array(vec![Value::Str(k), v]))
            .collect(),
    )
}

pub(super) fn dict_remove(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    if args.named.keys().any(|name| name.as_str() != "default") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let default = args.named.get("default").cloned();
    let key = expect_one_str(args.clone(), "dict.remove()")?;
    dict.shift_remove(&key).or(default).ok_or_else(|| {
        vec![SourceDiagnostic::error(
            args.span,
            format!("dictionary does not contain key \"{key}\""),
        )]
    })
}

pub(super) fn dict_update(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    let other = expect_one_dict(args, "dict.update()")?;
    dict.extend(other);
    Ok(Value::Dict(dict))
}

/// `dict.at(key, default: value)` — P491.
/// Retorna o valor associado a `key` ou `default` se a chave não existir.
pub(super) fn dict_at(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    // Validar named args antes de consumir `args`.
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

pub(super) fn dict_insert(
    mut dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
) -> SourceResult<Value> {
    let (key, value) = expect_str_value(args, "dict.insert()")?;
    dict.insert(key, value);
    Ok(Value::Dict(dict))
}

pub(super) fn dict_len(dict: IndexMap<EcoString, Value, FxBuildHasher>) -> Value {
    Value::Int(dict.len() as i64)
}

/// `dict.map(v => ...)` — aplica função a cada valor e preserva a chave.
pub(super) fn dict_map(
    dict: IndexMap<EcoString, Value, FxBuildHasher>,
    args: Args,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    let func = expect_one_func(args, "dict.map()")?;
    let mut result = IndexMap::with_capacity_and_hasher(dict.len(), FxBuildHasher);
    for (key, value) in dict {
        let mapped =
            apply_func(func.clone(), Args::positional(vec![value]), scopes, ctx, engine)?;
        result.insert(key, mapped);
    }
    Ok(Value::Dict(result))
}

/// `dict.filter(v => bool)` — mantém pares cujo valor satisfaz o predicado.
pub(super) fn dict_filter(
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
            Args::positional(vec![value.clone()]),
            scopes,
            ctx,
            engine,
        )?;
        match keep {
            Value::Bool(true) => {
                result.insert(key, value);
            }
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
    Ok(Value::Dict(result))
}
