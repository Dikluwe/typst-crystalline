//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html/attr_values.md
//! @prompt-hash 5e2ea794
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn require_str(value: &Value, span: Span) -> SourceResult<&str> {
    match value {
        Value::Str(value) => Ok(value),
        other => type_error(span, "string", other),
    }
}

pub(super) fn require_enum<'a>(
    value: &'a Value,
    span: Span,
    values: &'static [&str],
) -> SourceResult<&'a str> {
    let Value::Str(value) = value else {
        return Err(vec![SourceDiagnostic::error(
            span,
            format!("{}, found {}", expected_enum(values), vanilla_type_name(value)),
        )]);
    };
    let value = value.as_str();
    if values.contains(&value) {
        Ok(value)
    } else {
        Err(vec![SourceDiagnostic::error(span, expected_enum(values))])
    }
}

pub(super) fn expected_enum(values: &[&str]) -> String {
    format!("expected {}", expected_enum_body(values))
}

pub(super) fn expected_none_or_enum(values: &[&str]) -> String {
    let quoted = values.iter().map(|value| format!("{value:?}")).collect::<Vec<_>>();
    if quoted.is_empty() {
        "expected none".into()
    } else {
        format!("expected {}, or none", quoted.join(", "))
    }
}

pub(super) fn expected_enum_body(values: &[&str]) -> String {
    let quoted = values.iter().map(|value| format!("{value:?}")).collect::<Vec<_>>();
    match quoted.as_slice() {
        [] => "valid string".into(),
        [one] => one.clone(),
        [first, second] => format!("{first} or {second}"),
        [head @ .., last] => format!("{}, or {last}", head.join(", ")),
    }
}

pub(super) fn bool_string<'a>(
    value: &Value,
    span: Span,
    yes: &'a str,
    no: &'a str,
) -> SourceResult<&'a str> {
    match value {
        Value::Bool(true) => Ok(yes),
        Value::Bool(false) => Ok(no),
        other => type_error(span, "boolean", other),
    }
}

pub(super) fn cast_list(
    value: &Value,
    span: Span,
    allowed: Option<&'static [&'static str]>,
    chars: bool,
) -> SourceResult<String> {
    let is_array = matches!(value, Value::Array(_));
    let items: &[Value] = match value {
        Value::Array(items) => items,
        value => std::slice::from_ref(value),
    };
    let mut out = Vec::with_capacity(items.len());
    for item in items {
        let item = require_str(item, span)?;
        if chars && item.chars().count() != 1 {
            return Err(vec![SourceDiagnostic::error(
                span,
                "expected a single character",
            )]);
        }
        if let Some(allowed) = allowed {
            if !allowed.contains(&item) {
                return Err(vec![SourceDiagnostic::error(span, expected_enum(allowed))]);
            }
        }
        if is_array && item.contains(' ') {
            return Err(vec![SourceDiagnostic::error(
                span,
                "array item may not contain a space",
            )]);
        }
        out.push(item);
    }
    Ok(out.join(" "))
}

pub(super) fn cast_positive_float(value: &Value, span: Span) -> SourceResult<f64> {
    let number = match value {
        Value::Float(value) => *value,
        Value::Int(value) => *value as f64,
        other => return type_error(span, "float", other),
    };
    if number > 0.0 {
        Ok(number)
    } else {
        Err(vec![SourceDiagnostic::error(span, "number must be positive")])
    }
}

pub(super) fn cast_datetime(value: &Value, span: Span) -> SourceResult<EcoString> {
    let Value::Datetime(value) = value else {
        return type_error(span, "datetime", value);
    };
    let date = match (value.year(), value.month(), value.day()) {
        (Some(year), Some(month), Some(day)) => {
            Some(format!("{year:04}-{month:02}-{day:02}"))
        }
        _ => None,
    };
    let time = match (value.hour(), value.minute(), value.second()) {
        (Some(hour), Some(minute), Some(0)) => Some(format!("{hour:02}:{minute:02}")),
        (Some(hour), Some(minute), Some(second)) => {
            Some(format!("{hour:02}:{minute:02}:{second:02}"))
        }
        _ => None,
    };
    match (date, time) {
        (Some(date), Some(time)) => Ok(format!("{date}T{time}").into()),
        (Some(date), None) => Ok(date.into()),
        (None, Some(time)) => Ok(time.into()),
        (None, None) => Err(vec![SourceDiagnostic::error(span, "invalid datetime")]),
    }
}

pub(super) fn cast_duration(
    value: crate::entities::duration::Duration,
) -> SourceResult<String> {
    const SECOND: i128 = 1_000_000_000;
    const MINUTE: i128 = 60 * SECOND;
    const HOUR: i128 = 60 * MINUTE;
    const DAY: i128 = 24 * HOUR;
    const WEEK: i128 = 7 * DAY;

    let negative = value.nanos < 0;
    let mut remaining = value.nanos.abs();
    let weeks = remaining / WEEK;
    remaining %= WEEK;
    let days = remaining / DAY;
    remaining %= DAY;
    let hours = remaining / HOUR;
    remaining %= HOUR;
    let minutes = remaining / MINUTE;
    remaining %= MINUTE;
    let seconds = remaining as f64 / SECOND as f64;
    let mut parts = Vec::new();
    if weeks > 0 {
        parts.push(format!("{weeks}w"));
    }
    if days > 0 {
        parts.push(format!("{days}d"));
    }
    if hours > 0 {
        parts.push(format!("{hours}h"));
    }
    if minutes > 0 {
        parts.push(format!("{minutes}m"));
    }
    if seconds > 0.0 || parts.is_empty() {
        parts.push(format!("{seconds}s"));
    }
    let rendered = parts.join(" ");
    Ok(if negative { format!("-{rendered}") } else { rendered })
}

pub(super) fn cast_float_list(
    value: &Value,
    span: Span,
    shorthand: bool,
) -> SourceResult<String> {
    let items: &[Value] = match value {
        Value::Array(items) => items,
        value if shorthand => std::slice::from_ref(value),
        other => return type_error(span, "array", other),
    };
    items
        .iter()
        .map(|item| match item {
            Value::Float(value) => Ok(value.to_string()),
            Value::Int(value) => Ok(value.to_string()),
            other => type_error(span, "float", other),
        })
        .collect::<SourceResult<Vec<_>>>()
        .map(|items| items.join(", "))
}

pub(super) fn cast_string_list(
    value: &Value,
    span: Span,
    separator: &str,
    shorthand: bool,
) -> SourceResult<String> {
    let items: &[Value] = match value {
        Value::Array(items) => items,
        value if shorthand => std::slice::from_ref(value),
        other => return type_error(span, "array", other),
    };
    let items = items
        .iter()
        .map(|item| require_str(item, span).map(str::to_owned))
        .collect::<SourceResult<Vec<_>>>()?;
    if matches!(value, Value::Array(_))
        && items.iter().any(|item| item.contains(separator))
    {
        return Err(vec![SourceDiagnostic::error(
            span,
            if separator == "," {
                "array item may not contain a comma"
            } else {
                "array item may not contain a space"
            },
        )]);
    }
    Ok(items.join(if separator == "," { ", " } else { separator }))
}

pub(super) fn cast_complex_list(
    value: &Value,
    span: Span,
    cast: fn(&Value, Span) -> SourceResult<String>,
    separator: &str,
) -> SourceResult<String> {
    let Value::Array(items) = value else {
        return type_error(span, "array", value);
    };
    let items = items
        .iter()
        .map(|item| cast(item, span))
        .collect::<SourceResult<Vec<_>>>()?;
    if items.iter().any(|item| item.contains(separator)) {
        return Err(vec![SourceDiagnostic::error(
            span,
            if separator == "," {
                "array item may not contain a comma"
            } else {
                "array item may not contain a space"
            },
        )]);
    }
    Ok(items.join(if separator == "," { ", " } else { separator }))
}

pub(super) fn cast_icon_size(value: &Value, span: Span) -> SourceResult<String> {
    let Value::Array(items) = value else {
        return type_error(span, "array", value);
    };
    let [Value::Int(width), Value::Int(height)] = items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "expected an array of two integers",
        )]);
    };
    if *width < 0 || *height < 0 {
        return Err(vec![SourceDiagnostic::error(span, "number must be at least zero")]);
    }
    Ok(format!("{width}x{height}"))
}

pub(super) fn cast_image_candidate(value: &Value, span: Span) -> SourceResult<String> {
    let Value::Dict(dict) = value else {
        return type_error(span, "dictionary", value);
    };
    if dict
        .keys()
        .any(|key| !matches!(key.as_str(), "src" | "width" | "density"))
    {
        return Err(vec![SourceDiagnostic::error(span, "unexpected key in dictionary")]);
    }
    let Some(Value::Str(src)) = dict.get("src") else {
        return Err(vec![SourceDiagnostic::error(span, "expected string at key `src`")]);
    };
    if src.is_empty() {
        return Err(vec![SourceDiagnostic::error(span, "`src` must not be empty")]);
    }
    if src.starts_with(',') || src.ends_with(',') {
        return Err(vec![SourceDiagnostic::error(
            span,
            "`src` must not start or end with a comma",
        )]);
    }
    let width = match dict.get("width") {
        Some(Value::Int(value)) if *value > 0 => Some(*value),
        Some(Value::Int(_)) => {
            return Err(vec![SourceDiagnostic::error(span, "number must be positive")])
        }
        Some(other) => return type_error(span, "integer", other),
        None => None,
    };
    let density = match dict.get("density") {
        Some(value) => Some(cast_positive_float(value, span)?),
        None => None,
    };
    match (width, density) {
        (Some(_), Some(_)) => Err(vec![SourceDiagnostic::error(
            span,
            "cannot specify both `width` and `density`",
        )]),
        (Some(width), None) => Ok(format!("{src} {width}w")),
        (None, Some(density)) => Ok(format!("{src} {density}d")),
        (None, None) => Ok(src.to_string()),
    }
}

pub(super) fn cast_source_size(value: &Value, span: Span) -> SourceResult<String> {
    let Value::Dict(dict) = value else {
        return type_error(span, "dictionary", value);
    };
    if dict.keys().any(|key| !matches!(key.as_str(), "condition" | "size")) {
        return Err(vec![SourceDiagnostic::error(span, "unexpected key in dictionary")]);
    }
    let Some(Value::Str(condition)) = dict.get("condition") else {
        return Err(vec![SourceDiagnostic::error(
            span,
            "expected string at key `condition`",
        )]);
    };
    let Some(Value::Length(size)) = dict.get("size") else {
        return Err(vec![SourceDiagnostic::error(span, "expected length at key `size`")]);
    };
    Ok(format!(
        "({condition}) {}",
        crate::compiler::stdlib::foundations::str::format_length(size)
    ))
}

pub(super) fn type_error<T>(
    span: Span,
    expected: &str,
    found: &Value,
) -> SourceResult<T> {
    Err(vec![SourceDiagnostic::error(
        span,
        format!("expected {expected}, found {}", vanilla_type_name(found)),
    )])
}

pub(super) fn union_error<T>(
    span: Span,
    expected: &str,
    found: &Value,
) -> SourceResult<T> {
    type_error(span, expected, found)
}
