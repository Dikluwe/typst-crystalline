//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/html/attr_cast.md
//! @prompt-hash bbb6ef64
//! @layer L1
//! @updated 2026-09-30

use super::*;

pub(super) fn cast_attr(
    kind: AttrKind,
    value: &Value,
    span: Span,
) -> SourceResult<Option<EcoString>> {
    let string = match kind {
        AttrKind::Str => require_str(value, span)?.into(),
        AttrKind::Int => match value {
            Value::Int(value) => value.to_string().into(),
            other => return type_error(span, "integer", other),
        },
        AttrKind::PositiveInt => match value {
            Value::Int(value) if *value > 0 => value.to_string().into(),
            Value::Int(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "number must be positive",
                )])
            }
            other => return type_error(span, "integer", other),
        },
        AttrKind::NonNegativeInt => match value {
            Value::Int(value) if *value >= 0 => value.to_string().into(),
            Value::Int(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "number must be at least zero",
                )])
            }
            other => return type_error(span, "integer", other),
        },
        AttrKind::Float => match value {
            Value::Float(value) => value.to_string().into(),
            Value::Int(value) => value.to_string().into(),
            other => return type_error(span, "float", other),
        },
        AttrKind::TrueFalse => bool_string(value, span, "true", "false")?.into(),
        AttrKind::Presence => match value {
            Value::Bool(true) => "".into(),
            Value::Bool(false) => return Ok(None),
            other => return type_error(span, "boolean", other),
        },
        AttrKind::ListStr => cast_list(value, span, None, false)?.into(),
        AttrKind::ListChar => cast_list(value, span, None, true)?.into(),
        AttrKind::Enum(values) => require_enum(value, span, values)?.into(),
        AttrKind::EnumList(values) => cast_list(value, span, Some(values), false)?.into(),
        AttrKind::EnumOrStr(values) => match value {
            Value::Str(value) => value.clone(),
            other => {
                let values = values
                    .iter()
                    .map(|value| format!("{value:?}"))
                    .collect::<Vec<_>>()
                    .join(", ");
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected {values}, or string, found {}",
                        vanilla_type_name(other)
                    ),
                )]);
            }
        },
        AttrKind::NoneEmptyOrEnum(values) => match value {
            Value::None => "".into(),
            Value::Str(value) if values.contains(&value.as_str()) => value.clone(),
            Value::Str(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    expected_none_or_enum(values),
                )])
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "{}, found {}",
                        expected_none_or_enum(values),
                        vanilla_type_name(other)
                    ),
                )])
            }
        },
        AttrKind::BoolOr(values) => match value {
            Value::Bool(value) => value.to_string().into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::BoolOrUndefined => match value {
            Value::Bool(value) => value.to_string().into(),
            Value::None => "undefined".into(),
            other => return union_error(span, "boolean or none", other),
        },
        AttrKind::NoneOrEnum(values) => match value {
            Value::None => "none".into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::OnOffOrNoneOrEnum(values) => match value {
            Value::Bool(value) => if *value { "on" } else { "off" }.into(),
            Value::None => "none".into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::OnOff => bool_string(value, span, "on", "off")?.into(),
        AttrKind::ContentEditable => match value {
            Value::Bool(value) => value.to_string().into(),
            _ => require_enum(value, span, &["plaintext-only"])?.into(),
        },
        AttrKind::Direction => match value {
            Value::Dir(Dir::LTR) => "ltr".into(),
            Value::Dir(Dir::RTL) => "rtl".into(),
            Value::Auto => "auto".into(),
            other => return union_error(span, "ltr, rtl, or auto", other),
        },
        AttrKind::PresenceOr(values) => match value {
            Value::Bool(true) => "".into(),
            Value::Bool(false) => return Ok(None),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::NoneOrStr => match value {
            Value::None => "".into(),
            _ => require_str(value, span)?.into(),
        },
        AttrKind::AutoOr(values) => match value {
            Value::Auto => "auto".into(),
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::Preload => match value {
            Value::None => "none".into(),
            Value::Auto => "auto".into(),
            Value::Str(value) if value == "metadata" => value.clone(),
            Value::Str(_) => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    "expected none, auto, or \"metadata\"",
                )])
            }
            other => {
                return Err(vec![SourceDiagnostic::error(
                    span,
                    format!(
                        "expected none, auto, or \"metadata\", found {}",
                        other.type_name()
                    ),
                )])
            }
        },
        AttrKind::NoneOrRole => match value {
            Value::None => "none".into(),
            _ => require_enum(value, span, ROLES)?.into(),
        },
        AttrKind::YesNo => bool_string(value, span, "yes", "no")?.into(),
        AttrKind::Datetime => cast_datetime(value, span)?,
        AttrKind::DurationOrDatetime => match value {
            Value::Duration(value) => cast_duration(*value)?.into(),
            Value::Datetime(_) => cast_datetime(value, span)?,
            other => return union_error(span, "duration or datetime", other),
        },
        AttrKind::Color => match value {
            Value::Color(value) => value.clone().to_hex().into(),
            other => return type_error(span, "color", other),
        },
        AttrKind::FloatListComma => cast_float_list(value, span, false)?.into(),
        AttrKind::StrListComma => cast_string_list(value, span, ",", true)?.into(),
        AttrKind::StrFloatDatetime => match value {
            Value::Str(value) => value.clone(),
            Value::Float(value) => value.to_string().into(),
            Value::Int(value) => value.to_string().into(),
            Value::Datetime(_) => cast_datetime(value, span)?,
            other => return union_error(span, "string, float, or datetime", other),
        },
        AttrKind::PositiveFloatOr(values) => match value {
            Value::Float(_) | Value::Int(_) => {
                cast_positive_float(value, span)?.to_string().into()
            }
            _ => require_enum(value, span, values)?.into(),
        },
        AttrKind::InputValue => match value {
            Value::Str(value) => value.clone(),
            Value::Float(value) => value.to_string().into(),
            Value::Int(value) => value.to_string().into(),
            Value::Datetime(_) => cast_datetime(value, span)?,
            Value::Color(value) => value.clone().to_hex().into(),
            Value::Array(_) => cast_string_list(value, span, ",", false)?.into(),
            other => {
                return union_error(
                    span,
                    "string, float, datetime, color, or array",
                    other,
                )
            }
        },
        AttrKind::SourceSizeList => {
            cast_complex_list(value, span, cast_source_size, ",")?.into()
        }
        AttrKind::ImageCandidateList => {
            cast_complex_list(value, span, cast_image_candidate, ",")?.into()
        }
        AttrKind::IconSizeList => {
            cast_complex_list(value, span, cast_icon_size, " ")?.into()
        }
    };
    Ok(Some(string))
}
