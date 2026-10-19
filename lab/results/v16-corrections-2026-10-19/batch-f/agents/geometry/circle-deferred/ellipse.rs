//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/ellipse.md
//! @prompt-hash 7ebce296
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// `ellipse(width?, height?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, ... }`.
///
/// Mesmo padrão de fallback que `native_rect`.
pub fn native_ellipse(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["width", "height", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em ellipse(): '{}'", key),
            )]);
        }
    }

    let width = args.named.get("width").cloned().map(Box::new);
    let height = args.named.get("height").cloned().map(Box::new);
    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    let parsed_stroke = args
        .named
        .get("stroke")
        .filter(|value| !matches!(value, Value::None))
        .map(|value| parse_shape_stroke(value, "ellipse"))
        .transpose()?;

    let final_stroke = if fill.is_none()
        && parsed_stroke.is_none()
        && !matches!(args.named.get("stroke"), Some(Value::None))
    {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::shape(
        ShapeKind::Ellipse,
        width,
        height,
        fill,
        final_stroke,
    )))
}

/// `circle(radius?, fill?, stroke?)` → `Content::Shape { kind: Ellipse, width==height }`.
///
/// Double the full radius into width and height; resolve em only during layout.
pub fn native_circle(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    for key in args.named.keys() {
        if !["radius", "fill", "stroke"].contains(&key.as_str()) {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("argumento nomeado inesperado em circle(): '{}'", key),
            )]);
        }
    }

    fn radius_diameter(val: &Value, span: Span) -> SourceResult<Value> {
        use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
        match val {
            Value::Float(f) => Ok(Value::Float(*f * 2.0)),
            Value::Int(i) => Ok(Value::Float(*i as f64 * 2.0)),
            Value::Length(length) => Ok(Value::Length(*length * 2.0)),
            // The existing generic Shape fallback cannot represent circle Auto
            // sizing yet. Preserve this measured debt; do not invent eval 30pt.
            Value::Auto => Ok(Value::Float(0.0)),
            Value::None
            | Value::Bool(_)
            | Value::Str(_)
            | Value::Array(_)
            | Value::Dict(_)
            | Value::Module(_)
            | Value::Datetime(_)
            | Value::Func(_)
            | Value::Content(_)
            | Value::LocatedContent(_, _)
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
            | Value::SpotColorant(_) => Err(vec![SourceDiagnostic::error(
                span,
                format!("expected length or auto, found {}", vanilla_type_name(val)),
            )]),
        }
    }
    let value_span = |name: &str| {
        args.occurrences
            .as_ref()
            .and_then(|occurrences| {
                occurrences.iter().rev().find(|arg| arg.name.as_deref() == Some(name))
            })
            .map_or(args.span, |arg| arg.value_span.or(arg.span).or(args.span))
    };

    let (width, height) = match args.named.get("radius") {
        Some(r) => {
            let diameter = radius_diameter(r, value_span("radius"))?;
            (Some(Box::new(diameter.clone())), Some(Box::new(diameter)))
        }
        None => (None, None),
    };

    let fill = args
        .named
        .get("fill")
        .map(|value| parse_paint(value, args.span))
        .transpose()?
        .flatten();

    let parsed_stroke = args
        .named
        .get("stroke")
        .filter(|value| !matches!(value, Value::None))
        .map(|value| parse_shape_stroke(value, "circle"))
        .transpose()?;

    let final_stroke = if fill.is_none()
        && parsed_stroke.is_none()
        && !matches!(args.named.get("stroke"), Some(Value::None))
    {
        Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::shape(
        ShapeKind::Ellipse,
        width,
        height,
        fill,
        final_stroke,
    )))
}

#[cfg(test)]
mod tests {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn preserves_shape_behavior() {
        assert!(matches!(
            call(native_ellipse, Args::positional(vec![])).unwrap(),
            Value::Content(Content::Shape(_))
        ));
        assert!(matches!(
            call(native_circle, Args::positional(vec![])).unwrap(),
            Value::Content(Content::Shape(_))
        ));
    }
}

#[cfg(test)]
mod optional_stroke_regressions {
    use super::super::atomization_test_support::call;
    use super::*;

    #[test]
    fn explicit_none_stays_absent_with_and_without_fill() {
        for fill in [None, Some(Value::Color(Color::rgb(0, 0, 0)))] {
            let mut args = Args::positional(vec![]);
            args.named.insert("stroke".into(), Value::None);
            if let Some(fill) = fill {
                args.named.insert("fill".into(), fill);
            }
            let Value::Content(Content::Shape(shape)) =
                call(native_ellipse, args).unwrap()
            else {
                panic!("expected shape");
            };
            assert!(shape.stroke.is_none());
        }
        let Value::Content(Content::Shape(shape)) =
            call(native_ellipse, Args::positional(vec![])).unwrap()
        else {
            panic!("expected shape");
        };
        assert!(shape.stroke.is_some(), "omission keeps the unfilled default");
        for fill in [None, Some(Value::Color(Color::rgb(0, 0, 0)))] {
            let mut args = Args::positional(vec![]);
            args.named.insert("stroke".into(), Value::None);
            if let Some(fill) = fill {
                args.named.insert("fill".into(), fill);
            }
            let Value::Content(Content::Shape(shape)) =
                call(native_circle, args).unwrap()
            else {
                panic!("expected shape");
            };
            assert!(shape.stroke.is_none());
        }
        let Value::Content(Content::Shape(shape)) =
            call(native_circle, Args::positional(vec![])).unwrap()
        else {
            panic!("expected shape");
        };
        assert!(shape.stroke.is_some(), "omission keeps the unfilled default");
    }
}

#[cfg(test)]
mod batch_f_tests {

    use super::super::atomization_test_support::call;
    use super::*;

    fn at(start: usize) -> Span {
        Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            start..start + 1,
        )
    }
    fn named(name: &str, value: Value) -> Args {
        use crate::entities::args::ArgOccurrence;
        Args::from_occurrences(
            at(1),
            vec![
                ArgOccurrence {
                    name: None,
                    value: Value::Content(Content::Empty),
                    span: at(2),
                    value_span: at(3),
                },
                ArgOccurrence {
                    name: Some(name.into()),
                    value,
                    span: at(4),
                    value_span: at(5),
                },
            ],
        )
    }

    #[test]
    fn batch_f_circle_rejects_invalid_radius_at_value_span() {
        for (value, kind) in [
            (Value::None, "none"),
            (Value::Bool(true), "boolean"),
            (Value::Str("x".into()), "string"),
            (Value::Array(vec![]), "array"),
            (Value::Ratio(crate::entities::layout_types::Ratio(0.2)), "ratio"),
            (Value::Fraction(1.0), "fraction"),
        ] {
            let errors = call(native_circle, named("radius", value)).unwrap_err();
            assert_eq!(
                errors[0].message,
                format!("expected length or auto, found {kind}")
            );
            assert_eq!(errors[0].span, at(5));
        }
    }
    #[test]
    fn batch_f_circle_em_radius_reaches_actual_layout_at_occurrence_font() {
        use crate::compiler::layout::{measure_content_real_with_baseline, FixedMetrics};
        use crate::entities::style_chain::{StyleChain, StyleDelta};
        let Value::Content(circle) =
            call(native_circle, named("radius", Value::Length(Length::em(2.0)))).unwrap()
        else {
            panic!("circle")
        };
        for (font, diameter) in [(10.0, 40.0), (20.0, 80.0)] {
            let chain = StyleChain::default_chain().push(StyleDelta {
                size: Some(Length::pt(font)),
                ..StyleDelta::empty()
            });
            let (width, height, baseline) =
                measure_content_real_with_baseline(&circle, &chain, &FixedMetrics);
            assert_eq!((width, height, baseline), (diameter, diameter, diameter));
        }
    }
    #[test]
    fn batch_f_circle_preserves_absolute_and_numeric_geometry() {
        use crate::compiler::layout::{measure_content_real_with_baseline, FixedMetrics};
        use crate::entities::style_chain::StyleChain;
        for value in [Value::Length(Length::pt(2.0)), Value::Float(2.0), Value::Int(2)] {
            let Value::Content(circle) =
                call(native_circle, named("radius", value)).unwrap()
            else {
                panic!("circle")
            };
            let result = measure_content_real_with_baseline(
                &circle,
                &StyleChain::default_chain(),
                &FixedMetrics,
            );
            assert_eq!(result, (4.0, 4.0, 4.0));
        }
    }
    #[test]
    fn batch_f_circle_radius_error_precedes_paint_error() {
        let mut args = named("radius", Value::Bool(true));
        args.named.insert("fill".into(), Value::Bool(false));
        let errors = call(native_circle, args).unwrap_err();
        assert_eq!(errors[0].message, "expected length or auto, found boolean");
        assert_eq!(errors[0].span, at(5));
    }

    #[test]
    fn batch_f_circle_preserves_absolute_language_identity_boundaries() {
        use crate::compiler::eval::operators::equality::values_eq;
        let circle =
            call(native_circle, named("radius", Value::Length(Length::pt(2.0)))).unwrap();
        let numeric = call(native_circle, named("radius", Value::Int(2))).unwrap();
        assert!(
            values_eq(&circle, &numeric),
            "the deliberate numeric radius extension changed"
        );
        let mut ellipse = named("width", Value::Length(Length::pt(4.0)));
        ellipse.named.insert("height".into(), Value::Length(Length::pt(4.0)));
        let ellipse = call(native_ellipse, ellipse).unwrap();
        assert!(
            !values_eq(&circle, &ellipse),
            "circle became equal to a different element constructor"
        );
    }
    #[test]
    fn batch_f_circle_preserves_em_language_identity_boundary() {
        use crate::compiler::eval::operators::equality::values_eq;
        let circle =
            call(native_circle, named("radius", Value::Length(Length::em(2.0)))).unwrap();
        let mut ellipse = named("width", Value::Length(Length::em(4.0)));
        ellipse.named.insert("height".into(), Value::Length(Length::em(4.0)));
        let ellipse = call(native_ellipse, ellipse).unwrap();
        assert!(
            !values_eq(&circle, &ellipse),
            "circle became equal to a different element constructor"
        );
    }
}
