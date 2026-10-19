//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/str_transform.md
//! @prompt-hash 10d1069f
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::str_transform`.

use super::*;

/// **P692** — `str.normalize(form:)`: normalização Unicode. `form` (named) ∈
/// {"nfc", "nfd", "nfkc", "nfkd"}; default "nfc". Não aceita argumentos posicionais.
pub(super) fn str_normalize(s: EcoString, args: Args) -> SourceResult<Value> {
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

pub(super) fn str_replace(s: EcoString, args: Args) -> SourceResult<Value> {
    if args.named.keys().any(|name| name.as_str() != "count") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let count = match args.named.get("count") {
        None => None,
        Some(Value::Int(count)) if *count >= 0 => Some(*count as usize),
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected non-negative integer, found {}", other.type_name()),
            )])
        }
    };
    let (old, new) = expect_two_str(args, "str.replace()")?;
    let replaced = match count {
        Some(count) => s.replacen(old.as_str(), new.as_str(), count),
        None => s.replace(old.as_str(), new.as_str()),
    };
    Ok(Value::Str(replaced))
}

pub(super) fn str_trim_dispatch(s: EcoString, args: Args) -> SourceResult<Value> {
    if args.items.len() > 1 {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    for name in args.named.keys() {
        if !matches!(name.as_str(), "at" | "repeat") {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("unexpected argument: {name}"),
            )]);
        }
    }

    let repeat = match args.named.get("repeat") {
        None | Some(Value::Bool(true)) => true,
        Some(Value::Bool(false)) => false,
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected boolean, found {}", other.type_name()),
            )])
        }
    };
    let (trim_start, trim_end) = match args.named.get("at") {
        None | Some(Value::None) => (true, true),
        Some(Value::Align(align)) => {
            use crate::entities::layout_types::HAlign;
            match (align.h, align.v) {
                (Some(HAlign::Start), None) => (true, false),
                (Some(HAlign::End), None) => (false, true),
                _ => {
                    return Err(vec![SourceDiagnostic::error(
                        args.span,
                        "expected start or end alignment",
                    )])
                }
            }
        }
        Some(other) => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected alignment, found {}", other.type_name()),
            )])
        }
    };

    let Some(pattern) = args.items.first() else {
        let result = match (trim_start, trim_end) {
            (true, true) => s.trim(),
            (true, false) => s.trim_start(),
            (false, true) => s.trim_end(),
            (false, false) => s.as_str(),
        };
        return Ok(Value::Str(result.into()));
    };
    let Value::Str(pattern) = pattern else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("expected string, found {}", pattern.type_name()),
        )]);
    };
    if pattern.is_empty() {
        return Ok(Value::Str(s));
    }

    let mut result = s.as_str();
    if trim_start {
        result = if repeat {
            result.trim_start_matches(pattern.as_str())
        } else {
            result.strip_prefix(pattern.as_str()).unwrap_or(result)
        };
    }
    if trim_end {
        result = if repeat {
            result.trim_end_matches(pattern.as_str())
        } else {
            result.strip_suffix(pattern.as_str()).unwrap_or(result)
        };
    }
    Ok(Value::Str(result.into()))
}

pub(super) fn str_split(s: EcoString, args: Args) -> SourceResult<Value> {
    if args.items.is_empty() && args.named.is_empty() {
        return Ok(Value::Array(vec![Value::Str(s)]));
    }
    let sep = expect_one_str(args, "str.split()")?;
    Ok(Value::Array(s.split(sep.as_str()).map(|part| Value::Str(part.into())).collect()))
}

pub(super) fn str_repeat(s: EcoString, args: Args) -> SourceResult<Value> {
    let n = expect_one_int(args, "str.repeat()")?;
    if n < 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "str.repeat() requer inteiro não-negativo".to_string(),
        )]);
    }
    Ok(Value::Str(s.repeat(n as usize)))
}

pub(super) fn str_to_upper(s: EcoString) -> Value {
    Value::Str(s.to_uppercase())
}

pub(super) fn str_to_lower(s: EcoString) -> Value {
    Value::Str(s.to_lowercase())
}

pub(super) fn str_to_unicode(s: EcoString) -> SourceResult<Value> {
    let mut chars = s.chars();
    match (chars.next(), chars.next()) {
        (Some(character), None) => Ok(Value::Int(character as i64)),
        _ => Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "expected exactly one character",
        )]),
    }
}

pub(super) fn str_rev(s: EcoString) -> Value {
    Value::Str(s.chars().rev().collect::<String>().into())
}
