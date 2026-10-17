//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/abs.md
//! @prompt-hash 13ee9da4
//! @layer L1
//! @updated 2026-10-17

use super::*;

pub(crate) fn calc_abs(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    let value = match args.items.as_slice() {
        [Value::Int(i), ..] => i.checked_abs().map(Value::Int).ok_or_else(|| {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| occurrences.iter().find(|arg| arg.name.is_none()))
                .map(|arg| arg.value_span)
                .unwrap_or_else(Span::detached);
            vec![SourceDiagnostic::error(span, "the result is too large")]
        }),
        [Value::Float(f), ..] => Ok(Value::Float(f.abs())),
        // P817-D — paridade vanilla `calc.abs(decimal)` → decimal.
        [Value::Decimal(d), ..] => Ok(Value::Decimal(d.abs())),
        [Value::Length(v), ..] => {
            if v.abs.is_zero() || v.em == 0.0 {
                Ok(Value::Length(Length {
                    abs: Abs::pt(v.abs.to_pt().abs()),
                    em: v.em.abs(),
                }))
            } else {
                let span = args
                    .occurrences
                    .as_ref()
                    .and_then(|occurrences| {
                        occurrences.iter().find(|arg| arg.name.is_none())
                    })
                    .map(|arg| arg.value_span)
                    .unwrap_or_else(Span::detached);
                Err(vec![SourceDiagnostic::error(
                    span,
                    "cannot take absolute value of this length",
                )])
            }
        }
        [Value::Angle(v), ..] => Ok(Value::Angle(Angle::rad(v.to_rad().abs()))),
        [Value::Ratio(v), ..] => Ok(Value::Ratio(Ratio(v.get().abs()))),
        [Value::Fraction(v), ..] => Ok(Value::Fraction(v.abs())),
        [Value::Content(_) | Value::LocatedContent(_, _), ..] => {
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| occurrences.iter().find(|arg| arg.name.is_none()))
                .map(|arg| arg.value_span)
                .unwrap_or_else(Span::detached);
            Err(vec![SourceDiagnostic::error(
                span,
                "expected integer, float, length, angle, ratio, fraction, or decimal, found content",
            )])
        }
        [other, ..] => {
            let found = match other {
                Value::Str(_) => "string",
                Value::Bool(_) => "boolean",
                Value::None
                | Value::Int(_)
                | Value::Float(_)
                | Value::Array(_)
                | Value::Dict(_)
                | Value::Module(_)
                | Value::Datetime(_)
                | Value::Func(_)
                | Value::Content(_)
                | Value::LocatedContent(_, _)
                | Value::Auto
                | Value::Length(_)
                | Value::Relative(_)
                | Value::Ratio(_)
                | Value::Angle(_)
                | Value::Color(_)
                | Value::Stroke(_)
                | Value::Fraction(_)
                | Value::Align(_)
                | Value::Location(_)
                | Value::Gradient(_)
                | Value::Regex(_)
                | Value::Tiling(_)
                | Value::Bytes(_)
                | Value::Decimal(_)
                | Value::Duration(_)
                | Value::Version(_)
                | Value::Selector(_)
                | Value::Symbol(_)
                | Value::Args(_)
                | Value::State(_)
                | Value::Counter(_)
                | Value::Label(_)
                | Value::Dir(_)
                | Value::Path(_)
                | Value::Type(_)
                | Value::SpotColorant(_) => other.type_name(),
            };
            let span = args
                .occurrences
                .as_ref()
                .and_then(|occurrences| occurrences.iter().find(|arg| arg.name.is_none()))
                .map(|arg| arg.value_span)
                .unwrap_or_else(Span::detached);
            Err(vec![SourceDiagnostic::error(
                span,
                format!(
                    "expected integer, float, length, angle, ratio, fraction, or decimal, found {found}"
                ),
            )])
        }
        [] => {
            let named_value = match &args.occurrences {
                Some(occurrences) => occurrences
                    .iter()
                    .find(|arg| arg.name.as_deref() == Some("value"))
                    .map(|arg| arg.span),
                None => args.named.contains_key("value").then(Span::detached),
            };
            return Err(vec![match named_value {
                Some(span) => {
                    SourceDiagnostic::error(span, "the argument `value` is positional")
                        .with_hint("try removing `value:`")
                }
                None => SourceDiagnostic::error(args.span, "missing argument: value"),
            }]);
        }
    }?;
    let remainder = match &args.occurrences {
        Some(occurrences) => {
            let mut consumed = false;
            occurrences
                .iter()
                .find(|arg| {
                    if !consumed && arg.name.is_none() {
                        consumed = true;
                        false
                    } else {
                        true
                    }
                })
                .map(|arg| (arg.name.as_deref(), arg.span))
        }
        None if args.items.len() > 1 => Some((None, Span::detached())),
        None => args
            .named
            .keys()
            .next()
            .map(|name| (Some(name.as_str()), Span::detached())),
    };
    if let Some((name, span)) = remainder {
        let message = match name {
            Some(name) => format!("unexpected argument: {name}"),
            None => "unexpected argument".into(),
        };
        return Err(vec![SourceDiagnostic::error(span, message)]);
    }
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::super::calc_test_support::call;
    use super::*;

    #[test]
    fn preserves_modeled_result() {
        assert_eq!(call(calc_abs, vec![Value::Int(-7)]).unwrap(), Value::Int(7));
    }

    #[test]
    fn v16_abs_language_aliases_keep_complete_error_and_detached_origin() {
        for (value, name) in
            [(Value::Str("x".into()), "string"), (Value::Bool(false), "boolean")]
        {
            let errors = call(calc_abs, vec![value]).unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, format!("expected integer, float, length, angle, ratio, fraction, or decimal, found {name}"));
            assert_eq!(errors[0].span, Span::detached());
        }
    }
    #[test]
    fn v16_abs_nominal_families_keep_canonical_error_names() {
        for (value, name) in [
            (Value::None, "none"),
            (Value::Auto, "auto"),
            (Value::Array(vec![]), "array"),
            (Value::Dict(Default::default()), "dictionary"),
            (Value::Color(crate::entities::layout_types::Color::rgb(255, 0, 0)), "color"),
            (Value::Type(crate::entities::value::Type::Int), "type"),
        ] {
            let errors = call(calc_abs, vec![value]).unwrap_err();
            assert_eq!(errors[0].message,format!("expected integer, float, length, angle, ratio, fraction, or decimal, found {name}"));
        }
    }
    #[test]
    fn v16_abs_invalid_value_and_overflow_precede_leftovers() {
        for (value,message) in [
            (Value::Bool(false),"expected integer, float, length, angle, ratio, fraction, or decimal, found boolean"),
            (Value::Int(i64::MIN),"the result is too large"),
        ] {
            let errors=call(calc_abs,vec![value,Value::Int(2)]).unwrap_err();
            assert_eq!(errors[0].message,message);
        }
    }
    #[test]
    fn v16_abs_kinds_zero_sign_and_length_domain_remain() {
        use crate::entities::decimal::Decimal;
        for (value, expected) in [
            (Value::Int(-3), Value::Int(3)),
            (
                Value::Decimal(Decimal::from_str("-2.5").unwrap()),
                Value::Decimal(Decimal::from_str("2.5").unwrap()),
            ),
            (Value::Length(Length::pt(-3.0)), Value::Length(Length::pt(3.0))),
            (Value::Length(Length::em(-2.0)), Value::Length(Length::em(2.0))),
        ] {
            assert_eq!(call(calc_abs, vec![value]).unwrap(), expected);
        }
        let Value::Float(zero) = call(calc_abs, vec![Value::Float(-0.0)]).unwrap() else {
            panic!("kind changed")
        };
        assert_eq!(zero, 0.0);
        assert!(zero.is_sign_positive());
        let errors =
            call(calc_abs, vec![Value::Length(Length { abs: Abs::pt(-3.0), em: -2.0 })])
                .unwrap_err();
        assert_eq!(errors[0].message, "cannot take absolute value of this length");
    }
}
