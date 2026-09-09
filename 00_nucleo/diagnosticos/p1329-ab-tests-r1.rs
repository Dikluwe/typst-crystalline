#[cfg(test)]
mod p1329_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::{Abs, Angle, Length, Ratio};
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;

    const CONTENT_ERROR: &str = "expected integer, float, length, angle, ratio, fraction, or decimal, found content";

    struct TestWorld {
        source: Source,
        library: Library,
        book: FontBook,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            }
        }
    }
    impl World for TestWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            self.source.id()
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Ok(self.source.clone())
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }
    fn profiles() -> Vec<Features> {
        (0..4)
            .map(|bits| {
                let mut f = Features::empty();
                if bits & 1 != 0 {
                    f.enable(Feature::Html);
                }
                if bits & 2 != 0 {
                    f.enable(Feature::A11yExtras);
                }
                f
            })
            .collect()
    }
    fn full(
        text: &str,
        features: Features,
    ) -> (Source, SourceResult<Module>, Vec<SourceDiagnostic>) {
        let world = TestWorld::new(text);
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &world.source,
            &ElementRegistry::new(),
            false,
            EvalTarget::Paged,
            features,
        );
        (world.source.clone(), result, sink.into_diagnostics())
    }
    fn bare_diagnostic(errors: &[SourceDiagnostic], message: &str, span: Span) {
        assert_eq!(errors.len(), 1, "{errors:?}");
        let d = &errors[0];
        assert_eq!(d.message, message);
        assert_eq!(d.severity, Severity::Error);
        assert!(d.hints.is_empty(), "{d:?}");
        assert!(d.trace.is_empty(), "{d:?}");
        assert_eq!(d.span, span);
    }
    fn native(args: &Args, features: Features) -> SourceResult<Value> {
        let world = TestWorld::new("");
        let mut ctx = EvalContext::new();
        ctx.features = features;
        super::calc_abs(&mut ctx, args, &world, world.main())
    }

    const MIXED_ERROR: &str = "cannot take absolute value of this length";

    // Observe the public language species and scalar magnitude, not render bytes
    // or Value's structural PartialEq. NaN is observed as a class, never equality.
    fn magnitude(actual: f64, expected: f64) {
        if expected.is_nan() {
            assert!(actual.is_nan(), "expected NaN, got {actual}");
        } else if expected.is_infinite() {
            assert!(actual.is_infinite() && actual.is_sign_positive(), "{actual}");
        } else {
            assert!(
                (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0),
                "expected {expected}, got {actual}"
            );
        }
    }
    fn semantic(actual: Value, expected: Value) {
        assert_eq!(actual.type_name(), expected.type_name());
        match (actual, expected) {
            (Value::Length(a), Value::Length(e)) => {
                magnitude(a.abs.to_pt(), e.abs.to_pt());
                magnitude(a.em, e.em);
            }
            (Value::Angle(a), Value::Angle(e)) => magnitude(a.to_rad(), e.to_rad()),
            (Value::Ratio(a), Value::Ratio(e)) => magnitude(a.get(), e.get()),
            (Value::Fraction(a), Value::Fraction(e)) => magnitude(a, e),
            (Value::Float(a), Value::Float(e)) => magnitude(a, e),
            (a, e) => panic!("unexpected semantic comparison: {a:?} / {e:?}"),
        }
    }
    #[test]
    fn p1329_native_scalar_species_signs_zero_and_overturn() {
        for features in profiles() {
            for scalar in [-720.25_f64, -3.5, -0.0, 0.0, 2.25, 810.0] {
                for (input, expected) in [
                    (
                        Value::Length(Length::pt(scalar)),
                        Value::Length(Length::pt(scalar.abs())),
                    ),
                    (
                        Value::Length(Length::em(scalar)),
                        Value::Length(Length::em(scalar.abs())),
                    ),
                    (
                        Value::Angle(Angle::rad(scalar)),
                        Value::Angle(Angle::rad(scalar.abs())),
                    ),
                    (Value::Ratio(Ratio(scalar)), Value::Ratio(Ratio(scalar.abs()))),
                    (Value::Fraction(scalar), Value::Fraction(scalar.abs())),
                ] {
                    semantic(
                        native(&Args::positional(vec![input]), features).unwrap(),
                        expected,
                    );
                }
            }
            for (abs, em) in [(-0.0, -4.5), (-4.5, -0.0), (0.0, -0.0), (-0.0, -0.0)] {
                semantic(
                    native(
                        &Args::positional(vec![Value::Length(Length {
                            abs: Abs::pt(abs),
                            em,
                        })]),
                        features,
                    )
                    .unwrap(),
                    Value::Length(Length { abs: Abs::pt(abs.abs()), em: em.abs() }),
                );
            }
        }
    }
    #[test]
    fn p1329_native_nonfinite_values_are_already_constructed() {
        for features in profiles() {
            for scalar in [f64::NAN, f64::NEG_INFINITY, f64::INFINITY] {
                for (input, expected) in [
                    (
                        Value::Length(Length::pt(scalar)),
                        Value::Length(Length::pt(scalar.abs())),
                    ),
                    (
                        Value::Length(Length::em(scalar)),
                        Value::Length(Length::em(scalar.abs())),
                    ),
                    (
                        Value::Angle(Angle::rad(scalar)),
                        Value::Angle(Angle::rad(scalar.abs())),
                    ),
                    (Value::Ratio(Ratio(scalar)), Value::Ratio(Ratio(scalar.abs()))),
                    (Value::Fraction(scalar), Value::Fraction(scalar.abs())),
                    (Value::Float(scalar), Value::Float(scalar.abs())),
                ] {
                    semantic(
                        native(&Args::positional(vec![input]), features).unwrap(),
                        expected,
                    );
                }
                for (abs, em) in [(scalar, 1.0), (1.0, scalar), (scalar, scalar)] {
                    bare_diagnostic(
                        &native(
                            &Args::positional(vec![Value::Length(Length {
                                abs: Abs::pt(abs),
                                em,
                            })]),
                            features,
                        )
                        .unwrap_err(),
                        MIXED_ERROR,
                        Span::detached(),
                    );
                }
            }
        }
    }
    #[test]
    fn p1329_native_mixed_all_signs_value_origin_and_synthetic_traps() {
        let source = Source::detached("aggregate occurrence value other");
        let aggregate = Span::from_range(source.id(), 0..32);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..26);
        for features in profiles() {
            for (abs, em) in
                [(2.0, 3.0), (-2.0, -3.0), (2.0, -3.0), (-2.0, 3.0), (1e-100, 1e-100)]
            {
                let value = Value::Length(Length { abs: Abs::pt(abs), em });
                for value_span in [origin, Span::detached()] {
                    let args = Args::from_occurrences(
                        aggregate,
                        vec![ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: occurrence,
                            value_span,
                        }],
                    );
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        MIXED_ERROR,
                        value_span,
                    );
                }
                let args =
                    Args::from_parts(vec![value.clone()], Default::default(), aggregate);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    MIXED_ERROR,
                    Span::detached(),
                );
                let mut empty_occurrences = args.clone();
                empty_occurrences.occurrences = Some(Vec::new());
                bare_diagnostic(
                    &native(&empty_occurrences, features).unwrap_err(),
                    MIXED_ERROR,
                    Span::detached(),
                );
                // Synthetic occurrence metadata deliberately precedes the first
                // positional origin: it must not hijack the argument's anchor.
                let mut positional_after_named = args;
                positional_after_named.occurrences = Some(vec![
                    ArgOccurrence {
                        name: Some("metadata".into()),
                        value: Value::Int(1),
                        span: aggregate,
                        value_span: occurrence,
                    },
                    ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: origin,
                    },
                ]);
                bare_diagnostic(
                    &native(&positional_after_named, features).unwrap_err(),
                    MIXED_ERROR,
                    origin,
                );
            }
        }
    }
    #[test]
    fn p1329_native_guards_precede_dimension_dispatch() {
        for features in profiles() {
            for value in [
                Value::Length(Length::pt(-2.0)),
                Value::Length(Length { abs: Abs::pt(2.0), em: 3.0 }),
                Value::Angle(Angle::deg(-2.0)),
                Value::Ratio(Ratio(-2.0)),
                Value::Fraction(-2.0),
            ] {
                let mut args = Args::positional(vec![value.clone(), value.clone()]);
                args.named.insert("bad".into(), Value::Int(1));
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    "argumento nomeado inesperado: 'bad'",
                    Span::detached(),
                );
                args.named.clear();
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    "calc.abs() requer 1 argumento, recebeu 2",
                    Span::detached(),
                );
            }
        }
    }
    #[test]
    fn p1329_public_units_species_alias_with_spread() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-2cm)", Value::Length(Length::pt(7200.0 / 127.0))),
                ("calc.abs(-2mm)", Value::Length(Length::pt(720.0 / 127.0))),
                ("calc.abs(-2in)", Value::Length(Length::pt(144.0))),
                ("calc.abs(-2em)", Value::Length(Length::em(2.0))),
                ("calc.abs(0pt - 3em)", Value::Length(Length::em(3.0))),
                ("calc.abs(-2pt + 0em)", Value::Length(Length::pt(2.0))),
                ("calc.abs(-0em)", Value::Length(Length::ZERO)),
                ("calc.abs(-2rad)", Value::Angle(Angle::rad(2.0))),
                ("calc.abs(-810deg)", Value::Angle(Angle::deg(810.0))),
                ("calc.abs(-250%)", Value::Ratio(Ratio(2.5))),
                ("calc.abs(-2.5fr)", Value::Fraction(2.5)),
                ("{let ab=calc.abs; ab(-2em)}", Value::Length(Length::em(2.0))),
                (
                    "{let ab=calc.abs.with(-2rad).with(); ab()}",
                    Value::Angle(Angle::rad(2.0)),
                ),
                ("calc.abs(..(-250%,))", Value::Ratio(Ratio(2.5))),
                ("{let aa=arguments(-2.5fr); calc.abs(..aa)}", Value::Fraction(2.5)),
                ("{let café = -2em;\n calc.abs(café)}", Value::Length(Length::em(2.0))),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                semantic(result.unwrap(), expected);
            }
        }
    }
    fn public_error(text: &str, anchor: &str, message: &str, call: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
            assert_eq!(d.trace.len(), usize::from(call.is_some()), "{text}: {d:?}");
            if let Some(call) = call {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("calc.abs".into())));
            }
        }
    }
    #[test]
    fn p1329_public_mixed_origins_all_signs_utf8_alias_with_spread() {
        for (text, anchor, call) in [
            ("#calc.abs(2pt + 3em)", "2pt + 3em", None),
            ("#calc.abs(-2pt - 3em)", "-2pt - 3em", None),
            ("#calc.abs(2pt - 3em)", "2pt - 3em", None),
            ("#calc.abs(-2pt + 3em)", "-2pt + 3em", None),
            ("#let ab = calc.abs\n#ab(2pt + 3em)", "2pt + 3em", None),
            ("#let ab = calc.abs.with()\n#ab(2pt + 3em)", "2pt + 3em", None),
            ("#let ab = calc.abs.with(2pt + 3em)\n#ab()", "2pt + 3em", Some("ab()")),
            (
                "#let ab = calc.abs.with(2pt + 3em).with()\n#ab()",
                "2pt + 3em",
                Some("ab()"),
            ),
            ("#calc.abs(..(2pt + 3em,))", "..(2pt + 3em,)", None),
            (
                "#let aa = arguments(2pt + 3em)\n#calc.abs(..aa)",
                "2pt + 3em",
                Some("calc.abs(..aa)"),
            ),
            ("#let x = 2pt + 3em\n#let y = 2pt + 3em\n#calc.abs(y)", "y", None),
            ("texto á\n#let café = 2pt + 3em\n#calc.abs(\n café\n)", "café", None),
        ] {
            public_error(text, anchor, MIXED_ERROR, call);
        }
    }
    #[test]
    fn p1329_math_dimensional_text_stays_content() {
        for (text, anchor) in [
            ("$std.calc.abs(-2deg)$", "-2deg"),
            ("#let ab = calc.abs\n$ab(-2deg)$", "-2deg"),
        ] {
            public_error(text, anchor, CONTENT_ERROR, None);
        }
    }
    #[test]
    fn p1329_warning_survives_both_success_and_mixed_error() {
        for features in profiles() {
            for (text, success) in [
                ("#import std\n#calc.abs(-2em)", true),
                ("#import std\n#calc.abs(2pt + 3em)", false),
            ] {
                let (source, result, warnings) = full(text, features);
                assert_eq!(warnings.len(), 1);
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                assert_eq!(source.span_byte_range(w.span), Some(8..11));
                if success {
                    assert!(result.is_ok());
                } else {
                    let start = text.find("2pt + 3em").unwrap();
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1);
                    assert_eq!(errors[0].message, MIXED_ERROR);
                    assert_eq!(errors[0].severity, Severity::Error);
                    assert!(errors[0].hints.is_empty());
                    assert!(errors[0].trace.is_empty());
                    assert_eq!(
                        source.span_byte_range(errors[0].span),
                        Some(start..start + 9)
                    );
                }
            }
        }
    }
}
