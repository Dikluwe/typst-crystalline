
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
        Span::from_range(FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()), start..start + 1)
    }

    fn named(value: Value) -> Args {
        Args::from_occurrences(at(1), vec![ArgOccurrence {
            name: Some("radius".into()), value, span: at(4), value_span: at(5),
        }])
    }

    fn circle(value: Value) -> Value { call(native_circle, named(value)).unwrap() }
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
        assert_eq!(apply_binary(BinOp::Eq, a.clone(), b.clone()), Ok(Value::Bool(expected)));
        assert_eq!(apply_binary(BinOp::Neq, a.clone(), b.clone()), Ok(Value::Bool(!expected)));
    }
    fn assert_diagnostic(args: Args, span: Span, message: &str) {
        let diagnostics = call(native_circle, args).unwrap_err();
        assert_eq!(diagnostics, vec![SourceDiagnostic::error(span, message)]);
    }

    #[test]
    fn circle_origin_rejects_malformed_radius_at_causal_value_span() {
        for (value, kind) in [
            (Value::None, "none"), (Value::Bool(true), "boolean"),
            (Value::Str("x".into()), "string"), (Value::Array(vec![]), "array"),
            (Value::Dict(Default::default()), "dictionary"),
            (Value::Ratio(crate::entities::layout_types::Ratio(0.2)), "ratio"),
            (Value::Relative(crate::entities::rel::Rel { rel: 0.2, abs: Length::pt(1.0) }), "relative length"),
            (Value::Fraction(1.0), "fraction"), (Value::Content(Content::Empty), "content"),
        ] {
            assert_diagnostic(named(value), at(5), &format!("expected length or auto, found {kind}"));
        }
    }

    #[test]
    fn circle_origin_radius_diagnostic_uses_latest_occurrence_and_fallback_span() {
        let mut args = named(Value::Int(2));
        let mut occurrences = args.occurrences.take().unwrap();
        occurrences.push(ArgOccurrence { name: Some("radius".into()), value: Value::Bool(false), span: at(8), value_span: at(9) });
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
        assert_diagnostic(args, at(1), "argumento nomeado inesperado em circle(): 'unrecognized'");
    }

    #[test]
    fn circle_origin_em_radius_reaches_actual_layout_at_occurrence_font() {
        let circle = content(circle(Value::Length(Length::em(2.0))));
        for (font, diameter) in [(10.0, 40.0), (20.0, 80.0)] {
            let chain = StyleChain::default_chain().push(StyleDelta {
                size: Some(Length::pt(font)), ..StyleDelta::empty()
            });
            assert_eq!(measure_content_real_with_baseline(&circle, &chain, &FixedMetrics), (diameter, diameter, diameter));
        }
    }

    fn emitted_shapes(items: &[FrameItem], out: &mut Vec<(f64, f64, bool, f64)>) {
        for item in items {
            match item {
                FrameItem::Shape { kind: ShapeKind::Ellipse, width, height, fill, stroke, .. } => out.push((*width, *height, fill.is_some(), stroke.as_ref().map_or(0.0, |stroke| stroke.thickness))),
                FrameItem::Group { items, .. } | FrameItem::Link { items, .. } => emitted_shapes(items, out),
                _ => {}
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
                    1 => Content::block(circle.clone(), None, None, Sides::uniform(Length::ZERO), false),
                    _ => Content::boxed(circle.clone(), None, None, Sides::uniform(Length::ZERO), Length::ZERO),
                };
                Content::styled(Box::new(body), Styles::from_iter([Style::Size(Length::pt(font))]))
            });
            let document = crate::compiler::layout::layout(&Content::sequence(occurrences.to_vec()));
            let mut shapes = vec![];
            for page in &document.pages { emitted_shapes(&page.items, &mut shapes); }
            assert_eq!(shapes, vec![(34.0, 34.0, true, 2.0), (64.0, 64.0, true, 2.0)], "route {wrap}");
        }
    }

    #[test]
    fn circle_origin_preserves_absolute_and_numeric_geometry() {
        for value in [Value::Length(Length::pt(2.0)), Value::Float(2.0), Value::Int(2)] {
            let circle = content(circle(value));
            assert_eq!(measure_content_real_with_baseline(&circle, &StyleChain::default_chain(), &FixedMetrics), (4.0, 4.0, 4.0));
        }
    }

    #[test]
    fn circle_origin_preserves_absolute_language_identity_boundaries() {
        let c = circle(Value::Length(Length::pt(2.0)));
        for numeric in [Value::Int(2), Value::Float(2.0)] { assert_language(&c, &circle(numeric), true); }
        assert_language(&c, &ellipse(Value::Length(Length::pt(4.0))), false);
        assert_language(&c, &circle(Value::Length(Length::pt(3.0))), false);
        assert_language(&c, &c.clone(), true);
    }

    #[test]
    fn circle_origin_preserves_em_language_identity_boundary() {
        assert_language(&circle(Value::Length(Length::em(2.0))), &ellipse(Value::Length(Length::em(4.0))), false);
    }

    #[test]
    fn circle_origin_distinguishes_numeric_ellipse_and_em_or_mixed_radii() {
        assert_language(&circle(Value::Int(2)), &ellipse(Value::Int(4)), false);
        let em = circle(Value::Length(Length::em(2.0)));
        assert_language(&em, &circle(Value::Length(Length::em(1.0))), false);
        assert_language(&em, &circle(Value::Length(Length::ZERO)), false);
        let mixed = circle(Value::Length(Length::pt(2.0) + Length::em(1.0)));
        assert_language(&mixed, &circle(Value::Length(Length::pt(2.0) + Length::em(2.0))), false);
        assert_language(&mixed, &circle(Value::Length(Length::pt(2.0))), false);
        assert_language(&mixed, &mixed.clone(), true);
    }

    #[test]
    fn circle_origin_language_identity_survives_containers_membership_and_wrappers() {
        let a = circle(Value::Int(2));
        let alias = circle(Value::Length(Length::pt(2.0)));
        let other = ellipse(Value::Int(4));
        for (b, same) in [(alias, true), (other, false)] {
            assert_language(&Value::Array(vec![a.clone()]), &Value::Array(vec![b.clone()]), same);
            assert_language(&Value::Dict([("shape".into(), a.clone())].into_iter().collect()), &Value::Dict([("shape".into(), b.clone())].into_iter().collect()), same);
            assert_eq!(apply_binary(BinOp::In, a.clone(), Value::Array(vec![b.clone()])), Ok(Value::Bool(same)));
            assert_eq!(apply_binary(BinOp::NotIn, a.clone(), Value::Array(vec![b.clone()])), Ok(Value::Bool(!same)));
            for wrap in 0..4 {
                let wrap = |body: Content| match wrap {
                    0 => body.with_source_span(at(22)),
                    1 => Content::strong(body),
                    2 => Content::emph(body),
                    _ => Content::styled(Box::new(body), Styles::from_iter([Style::Size(Length::pt(50.0))])),
                };
                assert_language(&Value::Content(wrap(content(a.clone()))), &Value::Content(wrap(content(b.clone()))), same);
            }
        }
        assert_language(&Value::Content(Content::strong(content(a.clone()))), &Value::Content(Content::emph(content(a))), false);
    }

    #[test]
    fn circle_origin_retains_explicit_auto_zero_and_omission_debt() {
        let auto = circle(Value::Auto);
        for zero in [Value::Int(0), Value::Float(0.0), Value::Length(Length::ZERO)] { assert_language(&auto, &circle(zero), true); }
        assert_language(&auto, &call(native_circle, Args::positional(vec![])).unwrap(), false);
        assert_eq!(measure_content_real_with_baseline(&content(auto), &StyleChain::default_chain(), &FixedMetrics), (0.0, 0.0, 0.0));
    }

    #[test]
    fn circle_origin_preserves_signed_and_nonfinite_point_extensions() {
        for radius in [0.0, -2.0, f64::INFINITY, f64::NEG_INFINITY, f64::NAN] {
            for input in [Value::Float(radius), Value::Length(Length::pt(radius))] {
                let Content::Shape(shape) = content(circle(input)) else { panic!("shape") };
                for dimension in [&shape.width, &shape.height] {
                    let actual = match dimension.as_deref().unwrap() {
                        Value::Float(value) => *value,
                        Value::Length(value) => value.resolve_pt(12.0),
                        other => panic!("unexpected dimension {other:?}"),
                    };
                    if radius.is_nan() { assert!(actual.is_nan()); }
                    else { assert_eq!(actual, radius * 2.0); }
                }
            }
        }
    }

    #[test]
    fn circle_origin_preserves_fill_stroke_and_absence() {
        for fill in [None, Some(Color::rgb(12, 34, 56))] {
            for explicit_none in [false, true] {
                let mut args = named(Value::Length(Length::pt(2.0)));
                if let Some(color) = &fill { args.named.insert("fill".into(), Value::Color(color.clone())); }
                if explicit_none { args.named.insert("stroke".into(), Value::None); }
                let Content::Shape(shape) = content(call(native_circle, args).unwrap()) else { panic!("shape") };
                assert_eq!(shape.fill, fill.clone().map(Paint::Solid));
                assert_eq!(shape.stroke.is_some(), fill.is_none() && !explicit_none);
                if let Some(stroke) = &shape.stroke { assert_eq!(stroke.thickness, 1.0); }
            }
        }
    }
}
