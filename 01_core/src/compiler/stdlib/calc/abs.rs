//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/calc/abs.md
//! @prompt-hash fa0c6461
//! @layer L1
//! @updated 2026-09-30

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
                _ => other.type_name(),
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
}
