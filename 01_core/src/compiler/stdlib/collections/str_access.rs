//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/collections/str_access.md
//! @prompt-hash e6e6cdd1
//! @layer L1
//! @updated 2026-09-30
//!
//! Algoritmos de `collections::str_access`.

use super::*;

pub(super) fn str_len(s: EcoString) -> Value {
    // Paridade vanilla (P690): `str.len()` conta bytes UTF-8, não chars.
    Value::Int(s.len() as i64)
}

pub(super) fn str_first(s: EcoString, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    s.chars()
        .next()
        .map(|c| Value::Str(c.to_string().into()))
        .or(default)
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "string is empty and no default value was specified",
            )]
        })
}

pub(super) fn str_last(s: EcoString, args: Args) -> SourceResult<Value> {
    let default = optional_default(&args)?;
    s.chars()
        .last()
        .map(|c| Value::Str(c.to_string().into()))
        .or(default)
        .ok_or_else(|| {
            vec![SourceDiagnostic::error(
                args.span,
                "string is empty and no default value was specified",
            )]
        })
}

pub(super) fn str_at(s: EcoString, args: Args) -> SourceResult<Value> {
    // Paridade vanilla (P690): índice em bytes. Negativo conta bytes do fim.
    // Erro se fora de limites ou se o índice não for fronteira de carácter.
    if args.named.keys().any(|name| name.as_str() != "default") {
        return Err(vec![SourceDiagnostic::error(args.span, "unexpected argument")]);
    }
    let default = args.named.get("default").cloned();
    let index = expect_one_int(args.clone(), "str.at()")?;
    let st = s.as_str();
    let len = st.len() as i64;
    let idx = if index < 0 { len + index } else { index };
    if idx < 0 || idx >= len {
        return default.ok_or_else(|| {
            vec![SourceDiagnostic::error(
                Span::detached(),
                format!("str.at(): índice fora de limites (índice {index}, len {len})"),
            )]
        });
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

pub(super) fn str_slice(s: EcoString, args: Args) -> SourceResult<Value> {
    // Paridade vanilla (P690): índices em bytes. Sem clamp — índice fora de
    // limites ou fora de fronteira de carácter é erro. `start > end` → "".
    let mut positional = args.items.iter();
    let start = positional.next().and_then(|v| v.cast_int()).ok_or_else(|| {
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
pub(super) fn char_len(s: EcoString) -> Value {
    Value::Int(s.chars().count() as i64)
}

/// **P690** — `str.char-at(index)`: char no índice em chars. Extensão cristalina
/// não-portável; preserva a convenção pré-P690 de `str.at()`.
pub(super) fn char_at(s: EcoString, args: Args) -> SourceResult<Value> {
    let index = expect_one_int(args, "str.char-at()")?;
    let chars: Vec<char> = s.chars().collect();
    let idx =
        if index < 0 { (chars.len() as i64 + index) as usize } else { index as usize };
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
pub(super) fn char_slice(s: EcoString, args: Args) -> SourceResult<Value> {
    let mut positional = args.items.iter();
    let start = positional.next().and_then(|v| v.cast_int()).ok_or_else(|| {
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
            "str.char-slice(): não pode especificar end e count simultaneamente"
                .to_string(),
        )]);
    }

    let chars: Vec<char> = s.chars().collect();
    let start_idx =
        if start < 0 { (chars.len() as i64 + start) as usize } else { start as usize };
    let start_idx = start_idx.min(chars.len());
    let end_idx = match (end, count) {
        (Some(e), None) => {
            let e = if e < 0 { (chars.len() as i64 + e) as usize } else { e as usize };
            e.min(chars.len())
        }
        (None, Some(c)) => (start_idx + c.max(0) as usize).min(chars.len()),
        (None, None) => chars.len(),
        (Some(_), Some(_)) => unreachable!("end e count simultâneos já rejeitados acima"),
    };
    let result: String = chars[start_idx..end_idx.max(start_idx)].iter().collect();
    Ok(Value::Str(result.into()))
}

pub(super) fn str_clusters(s: EcoString) -> Value {
    Value::Array(s.graphemes(true).map(|g| Value::Str(g.into())).collect())
}

/// **P689** — `str.codepoints()`: array de strings, um por char (scalar value).
/// Distinto de `clusters`: um item por scalar Unicode, inclusive quando vários
/// scalars formam um único grapheme cluster.
pub(super) fn str_codepoints(s: EcoString) -> Value {
    Value::Array(s.chars().map(|c| Value::Str(c.to_string().into())).collect())
}
