//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/shapes/ellipse.md
//! @prompt-hash 6458e909
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
            thickness: crate::entities::layout_types::Length::pt(1.0),
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
/// Preserve full radius length until layout; numeric radii remain point extensions.
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

    fn radius_length(value: &Value, span: Span) -> SourceResult<Length> {
        use crate::compiler::eval::operators::error_formatting::vanilla_type_name;
        match value {
            Value::Float(value) => Ok(Length::pt(*value)),
            Value::Int(value) => Ok(Length::pt(*value as f64)),
            Value::Length(length) => Ok(*length),
            // Retained default-sizing debt: explicit Auto shares canonical zero.
            Value::Auto => Ok(Length::ZERO),
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
                format!("expected length or auto, found {}", vanilla_type_name(value)),
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
        Some(radius) => {
            let diameter =
                Value::Length(radius_length(radius, value_span("radius"))? * 2.0);
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
            thickness: crate::entities::layout_types::Length::pt(1.0),
            overhang: false,
            ..Stroke::default()
        })
    } else {
        parsed_stroke
    };

    Ok(Value::Content(Content::Shape(Arc::new(ShapeElem {
        origin: crate::entities::elements::shape::ShapeOrigin::Circle,
        rect_radius: None,
        rect_stroke: None,
        relative_line: None,
        kind: ShapeKind::Ellipse,
        width,
        height,
        fill,
        stroke: final_stroke,
        fill_rule: FillRule::NonZero,
    }))))
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
mod circle_origin_tests {
    use super::super::atomization_test_support::call;
    use super::*;
    use crate::compiler::eval::operators::equality::{apply_binary, values_eq};
    use crate::compiler::layout::{measure_content_real_with_baseline, FixedMetrics};
    use crate::entities::args::ArgOccurrence;
    use crate::entities::ast::expr::BinOp;
    use crate::entities::layout_types::FrameItem;
    use crate::entities::sides::Sides;
    use crate::entities::style::{Style, Styles};
    use crate::entities::style_chain::{StyleChain, StyleDelta};

    fn at(start: usize) -> Span {
        Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            start..start + 1,
        )
    }

    fn named(value: Value) -> Args {
        Args::from_occurrences(
            at(1),
            vec![ArgOccurrence {
                name: Some("radius".into()),
                value,
                span: at(4),
                value_span: at(5),
            }],
        )
    }

    fn circle(value: Value) -> Value {
        call(native_circle, named(value)).unwrap()
    }
    fn content(value: Value) -> Content {
        let Value::Content(value) = value else { panic!("expected content") };
        value
    }
    fn ellipse(diameter: Value) -> Value {
        let mut args = Args::positional(vec![]);
        args.named.insert("width".into(), diameter.clone());
        args.named.insert("height".into(), diameter);
        call(native_ellipse, args).unwrap()
    }
    fn assert_language(a: &Value, b: &Value, expected: bool) {
        assert_eq!(values_eq(a, b), expected);
        assert_eq!(
            apply_binary(BinOp::Eq, a.clone(), b.clone()),
            Ok(Value::Bool(expected))
        );
        assert_eq!(
            apply_binary(BinOp::Neq, a.clone(), b.clone()),
            Ok(Value::Bool(!expected))
        );
    }
    fn assert_diagnostic(args: Args, span: Span, message: &str) {
        let diagnostics = call(native_circle, args).unwrap_err();
        assert_eq!(diagnostics, vec![SourceDiagnostic::error(span, message)]);
    }

    #[test]
    fn circle_origin_rejects_malformed_radius_at_causal_value_span() {
        for (value, kind) in [
            (Value::None, "none"),
            (Value::Bool(true), "boolean"),
            (Value::Str("x".into()), "string"),
            (Value::Array(vec![]), "array"),
            (Value::Dict(Default::default()), "dictionary"),
            (Value::Ratio(crate::entities::layout_types::Ratio(0.2)), "ratio"),
            (
                Value::Relative(crate::entities::rel::Rel {
                    rel: 0.2,
                    abs: Length::pt(1.0),
                }),
                "relative length",
            ),
            (Value::Fraction(1.0), "fraction"),
            (Value::Content(Content::Empty), "content"),
        ] {
            assert_diagnostic(
                named(value),
                at(5),
                &format!("expected length or auto, found {kind}"),
            );
        }
    }

    #[test]
    fn circle_origin_radius_diagnostic_uses_latest_occurrence_and_fallback_span() {
        let mut args = named(Value::Int(2));
        let mut occurrences = args.occurrences.take().unwrap();
        occurrences.push(ArgOccurrence {
            name: Some("radius".into()),
            value: Value::Bool(false),
            span: at(8),
            value_span: at(9),
        });
        let args = Args::from_occurrences(at(1), occurrences);
        assert_diagnostic(args, at(9), "expected length or auto, found boolean");
        let mut args = named(Value::Bool(false));
        args.occurrences.as_mut().unwrap()[0].value_span = Span::detached();
        assert_diagnostic(args, at(4), "expected length or auto, found boolean");
        let mut args = Args::positional(vec![]);
        args.span = at(1);
        args.named.insert("radius".into(), Value::Bool(false));
        assert_diagnostic(args, at(1), "expected length or auto, found boolean");
    }

    #[test]
    fn circle_origin_radius_error_precedes_paint_error() {
        let mut args = named(Value::Bool(true));
        args.named.insert("fill".into(), Value::Bool(false));
        args.named.insert("stroke".into(), Value::Bool(false));
        assert_diagnostic(args, at(5), "expected length or auto, found boolean");
    }

    #[test]
    fn circle_origin_name_validation_still_precedes_radius() {
        let mut args = named(Value::Bool(true));
        args.named.insert("unrecognized".into(), Value::None);
        assert_diagnostic(
            args,
            at(1),
            "argumento nomeado inesperado em circle(): 'unrecognized'",
        );
    }

    #[test]
    fn circle_origin_em_radius_reaches_actual_layout_at_occurrence_font() {
        let circle = content(circle(Value::Length(Length::em(2.0))));
        for (font, diameter) in [(10.0, 40.0), (20.0, 80.0)] {
            let chain = StyleChain::default_chain().push(StyleDelta {
                size: Some(Length::pt(font)),
                ..StyleDelta::empty()
            });
            assert_eq!(
                measure_content_real_with_baseline(&circle, &chain, &FixedMetrics),
                (diameter, diameter, diameter)
            );
        }
    }

    fn emitted_shapes(items: &[FrameItem], out: &mut Vec<(f64, f64, bool, f64)>) {
        for item in items {
            match item {
                FrameItem::Shape {
                    kind: ShapeKind::Ellipse,
                    width,
                    height,
                    fill,
                    stroke,
                    ..
                } => out.push((
                    *width,
                    *height,
                    fill.is_some(),
                    stroke.as_ref().map_or(0.0, |stroke| stroke.thickness),
                )),
                FrameItem::Group { items, .. } | FrameItem::Link { items, .. } => {
                    emitted_shapes(items, out)
                }
                FrameItem::TextReplacement { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. } => {}
            }
        }
    }

    #[test]
    fn circle_origin_mixed_radius_paints_at_each_font_through_block_and_box() {
        let mut args = named(Value::Length(Length::pt(2.0) + Length::em(1.5)));
        args.named.insert("fill".into(), Value::Color(Color::rgb(12, 34, 56)));
        args.named.insert("stroke".into(), Value::Length(Length::pt(2.0)));
        let circle = content(call(native_circle, args).unwrap());
        for wrap in 0..3 {
            let occurrences = [10.0, 20.0].map(|font| {
                let body = match wrap {
                    0 => circle.clone(),
                    1 => Content::block(
                        circle.clone(),
                        None,
                        None,
                        Sides::uniform(Length::ZERO),
                        false,
                    ),
                    _ => Content::boxed(
                        circle.clone(),
                        None,
                        None,
                        Sides::uniform(Length::ZERO),
                        Length::ZERO,
                    ),
                };
                Content::styled(
                    Box::new(body),
                    Styles::from_iter([Style::Size(Length::pt(font))]),
                )
            });
            let document =
                crate::compiler::layout::layout(&Content::sequence(occurrences.to_vec()));
            let mut shapes = vec![];
            for page in &document.pages {
                emitted_shapes(&page.items, &mut shapes);
            }
            assert_eq!(
                shapes,
                vec![(34.0, 34.0, true, 2.0), (64.0, 64.0, true, 2.0)],
                "route {wrap}"
            );
        }
    }

    #[test]
    fn circle_origin_preserves_absolute_and_numeric_geometry() {
        for value in [Value::Length(Length::pt(2.0)), Value::Float(2.0), Value::Int(2)] {
            let circle = content(circle(value));
            assert_eq!(
                measure_content_real_with_baseline(
                    &circle,
                    &StyleChain::default_chain(),
                    &FixedMetrics
                ),
                (4.0, 4.0, 4.0)
            );
        }
    }

    #[test]
    fn circle_origin_preserves_absolute_language_identity_boundaries() {
        let c = circle(Value::Length(Length::pt(2.0)));
        for numeric in [Value::Int(2), Value::Float(2.0)] {
            assert_language(&c, &circle(numeric), true);
        }
        assert_language(&c, &ellipse(Value::Length(Length::pt(4.0))), false);
        assert_language(&c, &circle(Value::Length(Length::pt(3.0))), false);
        assert_language(&c, &c.clone(), true);
    }

    #[test]
    fn circle_origin_preserves_em_language_identity_boundary() {
        assert_language(
            &circle(Value::Length(Length::em(2.0))),
            &ellipse(Value::Length(Length::em(4.0))),
            false,
        );
    }

    #[test]
    fn circle_origin_distinguishes_numeric_ellipse_and_em_or_mixed_radii() {
        assert_language(&circle(Value::Int(2)), &ellipse(Value::Int(4)), false);
        let em = circle(Value::Length(Length::em(2.0)));
        assert_language(&em, &circle(Value::Length(Length::em(1.0))), false);
        assert_language(&em, &circle(Value::Length(Length::ZERO)), false);
        let mixed = circle(Value::Length(Length::pt(2.0) + Length::em(1.0)));
        assert_language(
            &mixed,
            &circle(Value::Length(Length::pt(2.0) + Length::em(2.0))),
            false,
        );
        assert_language(&mixed, &circle(Value::Length(Length::pt(2.0))), false);
        assert_language(&mixed, &mixed.clone(), true);
    }

    #[test]
    fn circle_origin_language_identity_survives_containers_membership_and_wrappers() {
        let a = circle(Value::Int(2));
        let alias = circle(Value::Length(Length::pt(2.0)));
        let other = ellipse(Value::Int(4));
        for (b, same) in [(alias, true), (other, false)] {
            assert_language(
                &Value::Array(vec![a.clone()]),
                &Value::Array(vec![b.clone()]),
                same,
            );
            assert_language(
                &Value::Dict([("shape".into(), a.clone())].into_iter().collect()),
                &Value::Dict([("shape".into(), b.clone())].into_iter().collect()),
                same,
            );
            assert_eq!(
                apply_binary(BinOp::In, a.clone(), Value::Array(vec![b.clone()])),
                Ok(Value::Bool(same))
            );
            assert_eq!(
                apply_binary(BinOp::NotIn, a.clone(), Value::Array(vec![b.clone()])),
                Ok(Value::Bool(!same))
            );
            for wrap in 0..4 {
                let wrap = |body: Content| match wrap {
                    0 => body.with_source_span(at(22)),
                    1 => Content::strong(body),
                    2 => Content::emph(body),
                    _ => Content::styled(
                        Box::new(body),
                        Styles::from_iter([Style::Size(Length::pt(50.0))]),
                    ),
                };
                assert_language(
                    &Value::Content(wrap(content(a.clone()))),
                    &Value::Content(wrap(content(b.clone()))),
                    same,
                );
            }
        }
        assert_language(
            &Value::Content(Content::strong(content(a.clone()))),
            &Value::Content(Content::emph(content(a))),
            false,
        );
    }

    #[test]
    fn circle_origin_retains_explicit_auto_zero_and_omission_debt() {
        let auto = circle(Value::Auto);
        for zero in [Value::Int(0), Value::Float(0.0), Value::Length(Length::ZERO)] {
            assert_language(&auto, &circle(zero), true);
        }
        assert_language(
            &auto,
            &call(native_circle, Args::positional(vec![])).unwrap(),
            false,
        );
        assert_eq!(
            measure_content_real_with_baseline(
                &content(auto),
                &StyleChain::default_chain(),
                &FixedMetrics
            ),
            (0.0, 0.0, 0.0)
        );
    }

    #[test]
    fn circle_origin_preserves_signed_and_nonfinite_point_extensions() {
        for radius in [0.0, -2.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            for input in [Value::Float(radius), Value::Length(Length::pt(radius))] {
                let Content::Shape(shape) = content(circle(input)) else {
                    panic!("shape")
                };
                for dimension in [&shape.width, &shape.height] {
                    let actual = match dimension.as_deref().unwrap() {
                        Value::Float(value) => *value,
                        Value::Length(value) => value.resolve_pt(12.0),
                        other => panic!("unexpected dimension {other:?}"),
                    };
                    if radius.is_nan() {
                        assert!(actual.is_nan());
                    } else {
                        assert_eq!(actual, radius * 2.0);
                    }
                }
            }
        }
    }

    #[test]
    fn circle_origin_preserves_fill_stroke_and_absence() {
        for fill in [None, Some(Color::rgb(12, 34, 56))] {
            for explicit_none in [false, true] {
                let mut args = named(Value::Length(Length::pt(2.0)));
                if let Some(color) = &fill {
                    args.named.insert("fill".into(), Value::Color(color.clone()));
                }
                if explicit_none {
                    args.named.insert("stroke".into(), Value::None);
                }
                let Content::Shape(shape) = content(call(native_circle, args).unwrap())
                else {
                    panic!("shape")
                };
                assert_eq!(shape.fill, fill.clone().map(Paint::Solid));
                assert_eq!(shape.stroke.is_some(), fill.is_none() && !explicit_none);
                if let Some(stroke) = &shape.stroke {
                    assert_eq!(
                        stroke.thickness,
                        crate::entities::layout_types::Length::pt(1.0)
                    );
                }
            }
        }
    }

    #[test]
    fn circle_origin_native_and_generic_constructor_invariants() {
        use crate::entities::elements::shape::ShapeOrigin;
        let mut inputs = vec![Args::positional(vec![])];
        inputs.extend(
            [
                Value::Auto,
                Value::Int(2),
                Value::Float(2.0),
                Value::Length(Length::em(2.0)),
                Value::Length(Length::pt(2.0) + Length::em(1.0)),
            ]
            .map(named),
        );
        for (index, args) in inputs.into_iter().enumerate() {
            let Content::Shape(shape) = content(call(native_circle, args).unwrap())
            else {
                panic!("shape")
            };
            assert_eq!(shape.origin, ShapeOrigin::Circle);
            assert_eq!(shape.kind, ShapeKind::Ellipse);
            assert_eq!(shape.width, shape.height);
            if index == 0 {
                assert!(shape.width.is_none());
            } else {
                assert!(matches!(shape.width.as_deref(), Some(Value::Length(_))));
            }
        }
        let Content::Shape(shape) = content(ellipse(Value::Length(Length::em(4.0))))
        else {
            panic!("shape")
        };
        assert_eq!(shape.origin, ShapeOrigin::Geometry);
        for generic in [
            Content::shape(ShapeKind::Ellipse, None, None, None, None),
            Content::shape_with_radius(
                ShapeKind::Rect,
                None,
                None,
                None,
                None,
                Corners::uniform(Length::ZERO),
            ),
        ] {
            let Content::Shape(shape) = generic else { panic!("shape") };
            assert_eq!(shape.origin, ShapeOrigin::Geometry);
        }
    }

    #[test]
    fn circle_origin_terminal_maps_and_clone_retain_origin() {
        use crate::entities::elements::{shape::ShapeOrigin, Element};
        let Content::Shape(shape) = content(circle(Value::Length(Length::em(2.0))))
        else {
            panic!("shape")
        };
        let copied = shape.as_ref().clone();
        assert_eq!(copied.origin, ShapeOrigin::Circle);
        let mut content_visits = 0;
        let mapped = copied
            .map_content(&mut |_| {
                content_visits += 1;
                Ok(None)
            })
            .unwrap();
        let mut text_visits = 0;
        let text_mapped = copied.map_text(&mut |_| {
            text_visits += 1;
            "changed".into()
        });
        for mapped in [mapped, text_mapped] {
            let Content::Shape(mapped) = mapped else { panic!("shape") };
            assert_eq!(mapped.origin, ShapeOrigin::Circle);
            assert_eq!(*mapped, copied);
        }
        assert_eq!((content_visits, text_visits), (0, 0));
    }

    #[test]
    fn circle_origin_structural_relation_and_hash_include_only_changed_origin() {
        use crate::entities::elements::shape::ShapeOrigin;
        use std::hash::{Hash, Hasher};
        let Content::Shape(shape) = content(circle(Value::Int(2))) else {
            panic!("shape")
        };
        let circle = shape.as_ref().clone();
        let mut geometry = circle.clone();
        geometry.origin = ShapeOrigin::Geometry;
        assert_ne!(circle, geometry);
        let digest = |shape: &ShapeElem| {
            let mut hash = std::collections::hash_map::DefaultHasher::new();
            shape.hash(&mut hash);
            hash.finish()
        };
        assert_eq!(digest(&circle), digest(&circle.clone()));
        assert_ne!(digest(&circle), digest(&geometry), "this controlled structural pair must include origin; this is not a hash-uniqueness claim");
        let a = crate::compiler::layout::layout(&Content::Shape(Arc::new(circle)));
        let b = crate::compiler::layout::layout(&Content::Shape(Arc::new(geometry)));
        let measure = |document: &crate::entities::layout_types::PagedDocument| {
            let mut shapes = vec![];
            for page in &document.pages {
                emitted_shapes(&page.items, &mut shapes);
            }
            shapes
        };
        assert_eq!(measure(&a), measure(&b), "render geometry is selected by kind");
    }

    #[test]
    fn circle_origin_corrects_float_ellipse_alias_in_language_compositions() {
        let a = circle(Value::Length(Length::pt(2.0)));
        let other = ellipse(Value::Float(4.0));
        assert_language(&a, &other, false);
        assert_language(
            &Value::Array(vec![a.clone()]),
            &Value::Array(vec![other.clone()]),
            false,
        );
        assert_language(
            &Value::Dict([("shape".into(), a.clone())].into_iter().collect()),
            &Value::Dict([("shape".into(), other.clone())].into_iter().collect()),
            false,
        );
        assert_eq!(
            apply_binary(BinOp::In, a.clone(), Value::Array(vec![other.clone()])),
            Ok(Value::Bool(false))
        );
        assert_eq!(
            apply_binary(BinOp::NotIn, a.clone(), Value::Array(vec![other.clone()])),
            Ok(Value::Bool(true))
        );
        for wrap in 0..4 {
            let wrap = |body: Content| match wrap {
                0 => body.with_source_span(at(22)),
                1 => Content::strong(body),
                2 => Content::emph(body),
                _ => Content::styled(
                    Box::new(body),
                    Styles::from_iter([Style::Size(Length::pt(50.0))]),
                ),
            };
            assert_language(
                &Value::Content(wrap(content(a.clone()))),
                &Value::Content(wrap(content(other.clone()))),
                false,
            );
        }
        assert_language(
            &Value::Content(content(a.clone()).with_source_span(at(22))),
            &Value::Content(content(a).with_source_span(at(30))),
            true,
        );
    }

    #[test]
    fn circle_origin_rejects_remaining_value_classes_at_causal_span() {
        use crate::entities::bytes::Bytes;
        use crate::entities::color::SpotColorant;
        use crate::entities::counter::{Counter, CounterKey};
        use crate::entities::decimal::Decimal;
        use crate::entities::dir::Dir;
        use crate::entities::duration::Duration;
        use crate::entities::element_kind::ElementKind;
        use crate::entities::func::Func;
        use crate::entities::gradient::{Gradient, GradientStop};
        use crate::entities::label::Label;
        use crate::entities::layout_types::{Align2D, Angle, Ratio};
        use crate::entities::locator::Locator;
        use crate::entities::module::Module;
        use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
        use crate::entities::regex::Regex;
        use crate::entities::scope::Scope;
        use crate::entities::selector::Selector;
        use crate::entities::state::State;
        use crate::entities::symbol::Symbol;
        use crate::entities::tiling::{Tiling, TilingBody};
        use crate::entities::value::{IntrospectedContent, Type};
        use crate::entities::version::Version;
        use crate::entities::world_types::Datetime;

        let location = Locator::new().next();
        let black = Color::rgb(0, 0, 0);
        let cases = [
            (Value::Module(Module::new("fixture", Scope::new())), "module"),
            (Value::Datetime(Datetime::new_date(2026, 10, 19).unwrap()), "datetime"),
            (Value::Func(Func::native("circle", native_circle)), "function"),
            (
                Value::LocatedContent(
                    IntrospectedContent::new(Content::Empty, None),
                    location,
                ),
                "content",
            ),
            (Value::Angle(Angle::deg(30.0)), "angle"),
            (Value::Color(black.clone()), "color"),
            (Value::Stroke(Arc::new(Stroke::default())), "stroke"),
            (Value::Align(Align2D::default()), "alignment"),
            (Value::Location(location), "location"),
            (
                Value::Gradient(Gradient::linear(
                    vec![
                        GradientStop::new(black.clone(), Ratio(0.0)),
                        GradientStop::new(Color::rgb(255, 255, 255), Ratio(1.0)),
                    ],
                    Angle::deg(0.0),
                )),
                "gradient",
            ),
            (Value::Regex(Regex::new("x").unwrap()), "regex"),
            (
                Value::Tiling(Arc::new(Tiling::new(TilingBody::Color(black.clone())))),
                "tiling",
            ),
            (Value::Bytes(Bytes::new(vec![1, 2])), "bytes"),
            (Value::Decimal(Decimal::new(2, 0)), "decimal"),
            (Value::Duration(Duration::SECOND), "duration"),
            (Value::Version(Arc::new(Version::new(0, 15, 1))), "version"),
            (Value::Selector(Selector::Kind(ElementKind::Heading)), "selector"),
            (Value::Symbol(Symbol::new("x", "x")), "symbol"),
            (Value::Args(Args::positional(vec![])), "arguments"),
            (
                Value::State(State {
                    key: "fixture".into(),
                    init: Box::new(Value::Int(0)),
                }),
                "state",
            ),
            (
                Value::Counter(Counter { key: CounterKey::Str("fixture".into()) }),
                "counter",
            ),
            (Value::Label(Label("fixture".into())), "label"),
            (Value::Dir(Dir::LTR), "direction"),
            (
                Value::Path(RootedPath::new(
                    VirtualRoot::Project,
                    VirtualPath::new("fixture.typ").unwrap(),
                )),
                "path",
            ),
            (Value::Type(Type::Int), "type"),
            (
                Value::SpotColorant(SpotColorant {
                    name: Some("fixture".into()),
                    fallback: black,
                }),
                "spot colorant",
            ),
        ];
        assert_eq!(cases.len(), 26);
        for (value, name) in cases {
            assert_diagnostic(
                named(value),
                at(5),
                &format!("expected length or auto, found {name}"),
            );
        }
    }
}
