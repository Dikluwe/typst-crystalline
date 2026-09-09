#[cfg(test)]
mod p1328_tests {
    use crate::compiler::eval::{
        EvalContext, EvalTarget, eval_expression_with_features,
        eval_with_full_error_target_and_features,
    };
    use crate::contracts::world::World;
    use crate::entities::args::{ArgOccurrence, Args};
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::content::Content;
    use crate::entities::decimal::Decimal;
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::locator::Locator;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::span::Span;
    use crate::entities::value::{IntrospectedContent, Value};
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
    fn content_values() -> Vec<Value> {
        let c = Content::strong(Content::text("-1"));
        vec![
            Value::Content(c.clone()),
            Value::LocatedContent(
                IntrospectedContent::new(c, None),
                Locator::new().next(),
            ),
        ]
    }
    #[test]
    fn p1328_native_content_both_representations_value_span_only() {
        let source = Source::detached("aggregated occurrence value other");
        let aggregate = Span::from_range(source.id(), 0..32);
        let occurrence = Span::from_range(source.id(), 11..21);
        let origin = Span::from_range(source.id(), 22..27);
        for features in profiles() {
            for value in content_values() {
                assert_eq!(value.type_name(), "content");
                let args = Args::from_occurrences(
                    aggregate,
                    vec![ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: origin,
                    }],
                );
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    origin,
                );
            }
        }
    }
    #[test]
    fn p1328_native_missing_or_detached_origin_is_not_fabricated() {
        let source = Source::detached("aggregate occurrence");
        let aggregate = Span::from_range(source.id(), 0..20);
        let occurrence = Span::from_range(source.id(), 10..20);
        for features in profiles() {
            for value in content_values() {
                let args =
                    Args::from_parts(vec![value.clone()], Default::default(), aggregate);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    Span::detached(),
                );
                let args = Args::from_occurrences(
                    aggregate,
                    vec![ArgOccurrence {
                        name: None,
                        value,
                        span: occurrence,
                        value_span: Span::detached(),
                    }],
                );
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    CONTENT_ERROR,
                    Span::detached(),
                );
            }
        }
    }
    #[test]
    fn p1328_native_guards_before_content_and_numeric_controls() {
        for features in profiles() {
            let value = Value::Content(Content::empty());
            let mut args = Args::positional(vec![value.clone(), value.clone()]);
            args.named.insert("unexpected".into(), Value::Int(1));
            bare_diagnostic(
                &native(&args, features).unwrap_err(),
                "argumento nomeado inesperado: 'unexpected'",
                Span::detached(),
            );
            for count in [0, 2] {
                let args = Args::positional(vec![value.clone(); count]);
                bare_diagnostic(
                    &native(&args, features).unwrap_err(),
                    &format!("calc.abs() requer 1 argumento, recebeu {count}"),
                    Span::detached(),
                );
            }
            for (input, expected) in [
                (Value::Int(-19), Value::Int(19)),
                (Value::Int(i64::MIN), Value::Int(i64::MAX)),
                (Value::Float(-1.25), Value::Float(1.25)),
                (
                    Value::Decimal(Decimal::new(-125, 2)),
                    Value::Decimal(Decimal::new(125, 2)),
                ),
            ] {
                assert_eq!(
                    native(&Args::positional(vec![input]), features).unwrap(),
                    expected
                );
            }
        }
    }
    fn public_error(text: &str, anchor: &str, call_trace: Option<&str>) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, CONTENT_ERROR, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}: {d:?}");
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}: {d:?}"
            );
            assert_eq!(d.trace.len(), usize::from(call_trace.is_some()), "{text}: {d:?}");
            if let Some(call) = call_trace {
                let start = text.rfind(call).unwrap();
                assert_eq!(
                    source.span_byte_range(d.trace[0].span),
                    Some(start..start + call.len())
                );
                // Dispatcher name is a measured pre-existing debt; native adds no trace.
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some("calc.abs".into())));
            }
        }
    }
    #[test]
    fn p1328_public_math_markup_alias_origins_utf8_all_profiles() {
        for (text, anchor) in [
            ("#calc.abs([])", "[]"),
            ("#calc.abs([*bold*])", "[*bold*]"),
            ("$std.calc.abs(-1)$", "-1"),
            ("#let ab = calc.abs\n$ab(-1)$", "-1"),
            ("#let ab = calc.abs\n#ab([x])", "[x]"),
            ("#let ab = calc.abs.with()\n#ab([x])", "[x]"),
            ("#let x = [x]\n#let y = [x]\n#calc.abs(y)", "y"),
            ("#let café = [á]\n#calc.abs(\n café\n)", "café"),
            ("texto á\n#calc.abs([β])", "[β]"),
            ("#calc.abs(..([x],))", "..([x],)"),
        ] {
            public_error(text, anchor, None);
        }
    }
    #[test]
    fn p1328_public_with_and_argument_spread_keep_prebound_origin() {
        for (text, anchor, call) in [
            ("#let ab = calc.abs.with([x])\n#ab()", "[x]", "ab()"),
            ("#let ab = calc.abs.with([x]).with()\n#ab()", "[x]", "ab()"),
            ("#let aa = arguments([x])\n#calc.abs(..aa)", "[x]", "calc.abs(..aa)"),
        ] {
            public_error(text, anchor, Some(call));
        }
    }
    #[test]
    fn p1328_public_warning_survives_error_all_profiles() {
        let text = "#import std\n#calc.abs([x])";
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), 1, "{warnings:?}");
            let w = &warnings[0];
            assert_eq!(w.message, "this import has no effect");
            assert_eq!(w.severity, Severity::Warning);
            assert!(w.hints.is_empty());
            assert!(w.trace.is_empty());
            assert_eq!(source.span_byte_range(w.span), Some(8..11));
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, CONTENT_ERROR);
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            let start = text.find("[x]").unwrap();
            assert_eq!(source.span_byte_range(d.span), Some(start..start + 3));
        }
    }
    #[test]
    fn p1328_public_numeric_and_untouched_rejections_all_profiles() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expr, expected) in [
                ("calc.abs(-19)", Value::Int(19)),
                ("calc.abs(-1.25)", Value::Float(1.25)),
                ("calc.abs(decimal(\"-1.25\"))", Value::Decimal(Decimal::new(125, 2))),
                ("calc.abs(-9223372036854775807 - 1)", Value::Int(i64::MAX)),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                assert_eq!(result.unwrap(), expected, "{expr}");
            }
            for (expr, message, name) in [
                (
                    "calc.abs(\"x\")",
                    "calc.abs() requer Int ou Float, recebeu str",
                    "calc.abs",
                ),
                (
                    "calc.abs(sym.alpha)",
                    "calc.abs() requer Int ou Float, recebeu symbol",
                    "calc.abs",
                ),
                (
                    "calc.abs(-2pt)",
                    "calc.abs() requer Int ou Float, recebeu length",
                    "calc.abs",
                ),
                (
                    "calc.abs(-2deg)",
                    "calc.abs() requer Int ou Float, recebeu angle",
                    "calc.abs",
                ),
                (
                    "calc.abs(-2%)",
                    "calc.abs() requer Int ou Float, recebeu ratio",
                    "calc.abs",
                ),
                (
                    "calc.abs(-2fr)",
                    "calc.abs() requer Int ou Float, recebeu fraction",
                    "calc.abs",
                ),
                (
                    "calc.sqrt([x])",
                    "calc.sqrt(): esperava Int ou Float, recebeu content",
                    "calc.sqrt",
                ),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}: {warnings:?}");
                let errors = result.unwrap_err();
                assert_eq!(errors.len(), 1, "{expr}: {errors:?}");
                let d = &errors[0];
                assert_eq!(d.message, message);
                assert_eq!(d.severity, Severity::Error);
                assert!(d.span.is_detached(), "{expr}: {d:?}");
                assert!(d.hints.is_empty());
                assert_eq!(d.trace.len(), 1);
                assert_eq!(d.trace[0].v, Tracepoint::Call(Some(name.into())));
                let source = Source::new_with_parser(
                    world.main(),
                    expr.into(),
                    crate::compiler::parse::parse_code,
                );
                assert_eq!(source.span_byte_range(d.trace[0].span), Some(0..expr.len()));
            }
        }
    }
}
