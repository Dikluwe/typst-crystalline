#[cfg(test)]
mod p1330_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalContext, EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
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
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("abs".into())));
            }
        }
    }

    const OVERFLOW: &str = "the result is too large";

    #[test]
    fn p1330_native_large_noninteger_species_do_not_inherit_integer_limit() {
        for features in profiles() {
            for (input, expected) in [
                (-9223372036854775808.0, 9223372036854775808.0),
                (-1e30, 1e30),
                (-0.0, 0.0),
            ] {
                match native(&Args::positional(vec![Value::Float(input)]), features)
                    .unwrap()
                {
                    Value::Float(actual) => assert_eq!(actual, expected),
                    other => panic!("float abs changed species: {other:?}"),
                }
            }
            match native(
                &Args::positional(vec![Value::Decimal(Decimal::new(i64::MIN, 0))]),
                features,
            )
            .unwrap()
            {
                Value::Decimal(actual) => {
                    assert_eq!(actual.to_string(), "9223372036854775808");
                }
                other => panic!("decimal abs changed species: {other:?}"),
            }
        }
    }

    #[test]
    fn p1330_native_integer_limits_exact_species_and_magnitude() {
        for features in profiles() {
            // Explicit expected values avoid reusing the implementation formula.
            for (input, expected) in [
                (i64::MIN + 1, i64::MAX),
                (i64::MIN + 2, i64::MAX - 1),
                (-9007199254740993, 9007199254740993),
                (-2147483648, 2147483648),
                (-1, 1),
                (0, 0),
                (1, 1),
                (2147483648, 2147483648),
                (9007199254740993, 9007199254740993),
                (i64::MAX - 1, i64::MAX - 1),
                (i64::MAX, i64::MAX),
            ] {
                match native(&Args::positional(vec![Value::Int(input)]), features)
                    .unwrap()
                {
                    Value::Int(actual) => assert_eq!(actual, expected, "input {input}"),
                    other => panic!("integer abs changed species: {other:?}"),
                }
            }
            bare_diagnostic(
                &native(&Args::positional(vec![Value::Int(i64::MIN)]), features)
                    .unwrap_err(),
                OVERFLOW,
                Span::detached(),
            );
        }
    }

    #[test]
    fn p1330_native_overflow_origin_first_positional_and_no_fabrication() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for value_span in [origin, Span::detached()] {
                let mut args = Args::from_parts(
                    vec![Value::Int(i64::MIN)],
                    Default::default(),
                    aggregate,
                );
                // Metadata is intentionally inconsistent with the named view to
                // distinguish occurrence filtering from the named-argument guard.
                args.occurrences = Some(vec![
                    ArgOccurrence {
                        name: Some("metadata".into()),
                        value: Value::Int(1),
                        span: aggregate,
                        value_span: decoy,
                    },
                    ArgOccurrence {
                        name: None,
                        value: Value::Int(i64::MIN),
                        span: occurrence,
                        value_span,
                    },
                    ArgOccurrence {
                        name: None,
                        value: Value::Int(i64::MIN),
                        span: decoy,
                        value_span: decoy,
                    },
                ]);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    OVERFLOW,
                    value_span,
                );
            }
            for occurrences in [None, Some(vec![])] {
                let mut args = Args::from_parts(
                    vec![Value::Int(i64::MIN)],
                    Default::default(),
                    aggregate,
                );
                args.occurrences = occurrences;
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    OVERFLOW,
                    Span::detached(),
                );
            }
        }
    }

    #[test]
    fn p1330_native_named_and_arity_guards_precede_overflow() {
        for features in profiles() {
            for count in [0, 1, 2] {
                let mut args = Args::positional(vec![Value::Int(i64::MIN); count]);
                args.named.insert("bad".into(), Value::Int(1));
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    "argumento nomeado inesperado: 'bad'",
                    Span::detached(),
                );
                if count != 1 {
                    args.named.clear();
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &format!("calc.abs() requer 1 argumento, recebeu {count}"),
                        Span::detached(),
                    );
                }
            }
        }
    }

    #[test]
    fn p1330_public_integer_boundaries_and_positive_routes() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-9223372036854775807)", i64::MAX),
                ("calc.abs(-9223372036854775807 + 1)", i64::MAX - 1),
                ("calc.abs(9223372036854775807)", i64::MAX),
                ("calc.abs(-9007199254740993)", 9007199254740993),
                ("calc.abs(-0)", 0),
                ("{let ab=calc.abs; ab(-1)}", 1),
                ("{let ab=calc.abs.with(-19).with(); ab()}", 19),
                ("{let aa=arguments(-19); calc.abs(..aa)}", 19),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                match result.unwrap() {
                    Value::Int(actual) => assert_eq!(actual, expected, "{expr}"),
                    other => panic!("{expr}: wrong species {other:?}"),
                }
            }
        }
    }

    #[test]
    fn p1330_public_overflow_real_origins_alias_with_arguments_utf8() {
        for (text, anchor, trace) in [
            ("#calc.abs(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with()\n#ab(-9223372036854775807 - 1)", "-9223372036854775807 - 1", None),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1)\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#let ab = calc.abs.with(-9223372036854775807 - 1).with()\n#ab()", "-9223372036854775807 - 1", Some("ab()")),
            ("#calc.abs(..(-9223372036854775807 - 1,))", "..(-9223372036854775807 - 1,)", None),
            ("#let aa = arguments(-9223372036854775807 - 1)\n#calc.abs(..aa)", "-9223372036854775807 - 1", Some("calc.abs(..aa)")),
            ("#let x = -9223372036854775807 - 1\n#let y = -9223372036854775807 - 1\n#calc.abs(y)", "y", None),
            ("texto á\n#let café = -9223372036854775807 - 1\n#calc.abs(\n café\n)", "café", None),
        ] {
            public_error(text, anchor, OVERFLOW, trace);
        }
    }

    #[test]
    fn p1330_math_spelling_remains_content_without_integer_coercion() {
        for text in [
            "$std.calc.abs(-9223372036854775807 - 1)$",
            "#let ab = calc.abs\n$ab(-9223372036854775807 - 1)$",
        ] {
            public_error(text, "-9223372036854775807 - 1", CONTENT_ERROR, None);
        }
    }

    #[test]
    fn p1330_warning_survives_overflow_and_boundary_success() {
        for features in profiles() {
            for (text, fails) in [
                ("#import std\n#calc.abs(-9223372036854775807)", false),
                ("#import std\n#calc.abs(-9223372036854775807 - 1)", true),
            ] {
                let (source, result, warnings) = full(text, features);
                assert_eq!(warnings.len(), 1);
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                assert_eq!(source.span_byte_range(w.span), Some(8..11));
                if fails {
                    let errors = result.unwrap_err();
                    assert_eq!(errors.len(), 1);
                    let d = &errors[0];
                    assert_eq!(d.message, OVERFLOW);
                    assert_eq!(d.severity, Severity::Error);
                    assert!(d.hints.is_empty());
                    assert!(d.trace.is_empty());
                    let start = text.find("-9223372036854775807 - 1").unwrap();
                    assert_eq!(
                        source.span_byte_range(d.span),
                        Some(start..start + "-9223372036854775807 - 1".len())
                    );
                } else {
                    assert!(result.is_ok());
                }
            }
        }
    }
}
