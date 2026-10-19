//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/str_search.md
//! @prompt-hash fec2f6a3
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::str_search`.

use super::*;

/// **P689** — `str.position(hay)`: índice em **bytes** da primeira ocorrência,
/// ou `none`. Aceita `str` (substring literal) ou `regex` (primeiro match).
pub(super) fn str_position(s: EcoString, args: Args) -> SourceResult<Value> {
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
            format!("str.position() espera str ou regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.position() requer 1 argumento posicional".to_string(),
        )]),
    }
}

/// Constrói o dict `{start, end, text, captures}` de um match (índices em bytes).
/// Partilhado por `str.match` (P689) e `str.matches` (P692).
pub(super) fn match_dict(
    start: usize,
    end: usize,
    text: &str,
    captures: Vec<Option<String>>,
) -> Value {
    let mut dict: IndexMap<EcoString, Value, FxBuildHasher> = IndexMap::default();
    dict.insert("start".into(), Value::Int(start as i64));
    dict.insert("end".into(), Value::Int(end as i64));
    dict.insert("text".into(), Value::Str(text.to_string().into()));
    dict.insert(
        "captures".into(),
        Value::Array(
            captures
                .into_iter()
                .map(|c| match c {
                    Some(s) => Value::Str(s.into()),
                    None => Value::None,
                })
                .collect(),
        ),
    );
    Value::Dict(dict)
}

/// **P689** — `str.match(pattern)`: primeiro match, como dict
/// `{start, end, text, captures}` com índices em **bytes**, ou `none`.
/// Aceita `str` (literal; `captures` vazio) ou `regex` (capturas em ordem
/// posicional, grupos nomeados inclusive). (**P693**: alargado de `regex`-only
/// para `str | regex`, espelhando `str_matches`.)
pub(super) fn str_match(s: EcoString, args: Args) -> SourceResult<Value> {
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
            format!("str.match() espera str ou regex, recebeu {}", other.type_name()),
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
pub(super) fn str_matches(s: EcoString, args: Args) -> SourceResult<Value> {
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
            format!("str.matches() espera str ou regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.matches() requer 1 argumento posicional".to_string(),
        )]),
    }
}

pub(super) fn str_contains(s: EcoString, args: Args) -> SourceResult<Value> {
    let substr = expect_one_str(args, "str.contains()")?;
    Ok(Value::Bool(s.contains(substr.as_str())))
}

pub(super) fn str_starts_with(s: EcoString, args: Args) -> SourceResult<Value> {
    let prefix = expect_one_str(args, "str.starts-with()")?;
    Ok(Value::Bool(s.starts_with(prefix.as_str())))
}

pub(super) fn str_ends_with(s: EcoString, args: Args) -> SourceResult<Value> {
    let suffix = expect_one_str(args, "str.ends-with()")?;
    Ok(Value::Bool(s.ends_with(suffix.as_str())))
}

pub(super) fn str_find(s: EcoString, args: Args) -> SourceResult<Value> {
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
            format!("str.find() espera str ou regex, recebeu {}", other.type_name()),
        )]),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.find() requer 1 argumento posicional".to_string(),
        )]),
    }
}
