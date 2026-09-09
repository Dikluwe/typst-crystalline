#[cfg(test)]
mod p1331_tests {
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
    use crate::entities::layout_types::Length;
    use crate::entities::locator::Locator;
    use crate::entities::module::Module;
    use crate::entities::path::{RootedPath, VirtualPath, VirtualRoot};
    use crate::entities::rel::Rel;
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
        // The fixture has one virtual file at the project root. Resolution
        // constructs a value in RAM; it never reads a file or host path.
        fn resolve_path(
            &self,
            current_file: FileId,
            path: &str,
        ) -> Result<RootedPath, String> {
            if current_file != self.main() {
                return Err("unknown virtual file in P1331 fixture".into());
            }
            let vpath = VirtualPath::new(path)
                .map_err(|error| format!("invalid virtual path: {error:?}"))?;
            Ok(RootedPath::new(VirtualRoot::Project, vpath))
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

    fn rejection(kind: &str) -> String {
        format!("expected integer, float, length, angle, ratio, fraction, or decimal, found {kind}")
    }

    fn family_values(features: Features) -> Vec<(Value, &'static str)> {
        let world = TestWorld::new("");
        let mut values = Vec::new();
        for (expression, kind) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
            ("\"-19\"", "string"),
            ("\"1.25\"", "string"),
            ("\"1pt\"", "string"),
            ("\"\"", "string"),
            ("\"á\"", "string"),
            ("(1,2)", "array"),
            ("(a: 1)", "dictionary"),
            ("calc", "module"),
            ("(x => x)", "function"),
            ("datetime(year: 2026, month: 1, day: 2)", "datetime"),
            ("red", "color"),
            ("1pt + red", "stroke"),
            ("left + top", "alignment"),
            ("gradient.linear(red, blue)", "gradient"),
            ("regex(\"a\")", "regex"),
            ("tiling(size: (2pt, 2pt))[x]", "tiling"),
            ("bytes((1,2))", "bytes"),
            ("duration(seconds: 1)", "duration"),
            ("version(1,2)", "version"),
            ("selector(heading)", "selector"),
            ("sym.alpha", "symbol"),
            ("arguments(1)", "arguments"),
            ("state(\"p1331\", 1)", "state"),
            ("counter(heading)", "counter"),
            ("<x>", "label"),
            ("ltr", "direction"),
            ("location", "type"),
        ] {
            let (result, warnings) =
                eval_expression_with_features(&world, expression, features);
            assert!(warnings.is_empty(), "constructor {expression}: {warnings:?}");
            values.push((result.expect(expression), kind));
        }
        values.push((Value::Relative(Rel::zero()), "relative length"));
        values.push((
            Value::Relative(Rel { rel: 0.5, abs: Length::pt(2.0) }),
            "relative length",
        ));
        values.push((
            Value::Path(RootedPath::new(
                VirtualRoot::Project,
                VirtualPath::new("/p1331.typ").unwrap(),
            )),
            "path",
        ));
        // Preconstructed Location only: this does not test introspection production.
        values.push((Value::Location(Locator::new().next()), "location"));
        values
    }

    #[test]
    fn p1331_native_every_rejected_family_complete_diagnostic_and_origins() {
        let source = Source::detached("aggregate occurrence origin decoy");
        let aggregate = Span::from_range(source.id(), 0..33);
        let occurrence = Span::from_range(source.id(), 10..20);
        let origin = Span::from_range(source.id(), 21..27);
        let decoy = Span::from_range(source.id(), 28..33);
        for features in profiles() {
            for (value, kind) in family_values(features) {
                for value_span in [origin, Span::detached()] {
                    let mut args = Args::from_parts(
                        vec![value.clone()],
                        Default::default(),
                        aggregate,
                    );
                    args.occurrences = Some(vec![
                        ArgOccurrence {
                            name: Some("metadata".into()),
                            value: Value::Int(1),
                            span: aggregate,
                            value_span: decoy,
                        },
                        ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: occurrence,
                            value_span,
                        },
                        ArgOccurrence {
                            name: None,
                            value: value.clone(),
                            span: decoy,
                            value_span: decoy,
                        },
                    ]);
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &rejection(kind),
                        value_span,
                    );
                }
                for occurrences in [
                    None,
                    Some(vec![]),
                    Some(vec![ArgOccurrence {
                        name: Some("metadata".into()),
                        value: value.clone(),
                        span: aggregate,
                        value_span: decoy,
                    }]),
                ] {
                    let mut args = Args::from_parts(
                        vec![value.clone()],
                        Default::default(),
                        aggregate,
                    );
                    args.occurrences = occurrences;
                    bare_diagnostic(
                        &native(&args, features).unwrap_err(),
                        &rejection(kind),
                        Span::detached(),
                    );
                }
            }
        }
    }

    #[test]
    fn p1331_native_guards_still_precede_every_rejected_family() {
        for features in profiles() {
            for (value, _) in family_values(features) {
                for count in [0, 1, 2] {
                    let mut args = Args::positional(vec![value.clone(); count]);
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
    }

    #[test]
    fn p1331_public_rejected_families_anchor_the_whole_argument() {
        for (expression, kind) in [
            ("true", "boolean"),
            ("false", "boolean"),
            ("none", "none"),
            ("auto", "auto"),
            ("\"-19\"", "string"),
            ("\"1.25\"", "string"),
            ("\"1pt\"", "string"),
            ("\"\"", "string"),
            ("\"á\"", "string"),
            ("(1,2)", "array"),
            ("(a: 1)", "dictionary"),
            ("calc", "module"),
            ("(x => x)", "function"),
            ("datetime(year: 2026, month: 1, day: 2)", "datetime"),
            ("red", "color"),
            ("1pt + red", "stroke"),
            ("left + top", "alignment"),
            ("gradient.linear(red, blue)", "gradient"),
            ("regex(\"a\")", "regex"),
            ("tiling(size: (2pt, 2pt))[x]", "tiling"),
            ("bytes((1,2))", "bytes"),
            ("duration(seconds: 1)", "duration"),
            ("version(1,2)", "version"),
            ("selector(heading)", "selector"),
            ("sym.alpha", "symbol"),
            ("arguments(1)", "arguments"),
            ("state(\"p1331\", 1)", "state"),
            ("counter(heading)", "counter"),
            ("<x>", "label"),
            ("ltr", "direction"),
            ("location", "type"),
            ("0% + 0pt", "relative length"),
            ("50% + 2pt", "relative length"),
            ("path(\"p1331.typ\")", "path"),
        ] {
            let text = format!("#calc.abs({expression})");
            public_error(&text, expression, &rejection(kind), None);
        }
    }

    #[test]
    fn p1331_public_real_origins_alias_with_spread_arguments_utf8() {
        for (text, anchor, kind, trace) in [
            ("#let ab=calc.abs\n#ab(false)", "false", "boolean", None),
            ("#let ab=calc.abs.with()\n#ab(\"x\")", "\"x\"", "string", None),
            ("#let ab=calc.abs.with(\"x\")\n#ab()", "\"x\"", "string", Some("ab()")),
            (
                "#let ab=calc.abs.with(false).with()\n#ab()",
                "false",
                "boolean",
                Some("ab()"),
            ),
            ("#calc.abs(..(false,))", "..(false,)", "boolean", None),
            (
                "#let aa=arguments(\"x\")\n#calc.abs(..aa)",
                "\"x\"",
                "string",
                Some("calc.abs(..aa)"),
            ),
            ("#let x=\"x\"\n#let y=\"x\"\n#calc.abs(y)", "y", "string", None),
            ("texto á\n#let café=\"á\"\n#calc.abs(\n café\n)", "café", "string", None),
        ] {
            public_error(text, anchor, &rejection(kind), trace);
        }
    }

    #[test]
    fn p1331_math_explicit_string_remains_string_without_numeric_coercion() {
        for text in ["$std.calc.abs(\"-1\")$", "#let ab=calc.abs\n$ab(\"-1\")$"] {
            public_error(text, "\"-1\"", &rejection("string"), None);
        }
    }

    #[test]
    fn p1331_warning_survives_rejection_with_all_fields() {
        let text = "#import std\n#calc.abs(false)";
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), 1);
            let w = &warnings[0];
            assert_eq!(w.message, "this import has no effect");
            assert_eq!(w.severity, Severity::Warning);
            assert!(w.hints.is_empty());
            assert!(w.trace.is_empty());
            assert_eq!(source.span_byte_range(w.span), Some(8..11));
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1);
            let d = &errors[0];
            assert_eq!(d.message, rejection("boolean"));
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            assert!(d.trace.is_empty());
            assert_eq!(source.span_byte_range(d.span), Some(22..27));
        }
    }

    #[test]
    fn p1331_existing_numeric_species_and_special_errors_remain() {
        let world = TestWorld::new("");
        for features in profiles() {
            for (expression, expected) in [
                ("(type(calc.abs(-19)), calc.abs(-19))", "(int, 19)"),
                ("(type(calc.abs(-1.25)), calc.abs(-1.25))", "(float, 1.25)"),
                (
                    "(type(calc.abs(decimal(\"-1.25\"))), calc.abs(decimal(\"-1.25\")))",
                    "(decimal, decimal(\"1.25\"))",
                ),
                ("(type(calc.abs(-2pt)), calc.abs(-2pt))", "(length, 2pt)"),
                ("(type(calc.abs(-2em)), calc.abs(-2em))", "(length, 2em)"),
                ("(type(calc.abs(-810deg)), calc.abs(-810deg))", "(angle, 810deg)"),
                ("(type(calc.abs(-250%)), calc.abs(-250%))", "(ratio, 250%)"),
                ("(type(calc.abs(-2fr)), calc.abs(-2fr))", "(fraction, 2fr)"),
            ] {
                let (result, warnings) = eval_expression_with_features(
                    &world,
                    &format!("repr({expression})"),
                    features,
                );
                assert!(warnings.is_empty(), "{expression}: {warnings:?}");
                match result.unwrap() {
                    Value::Str(actual) => assert_eq!(actual.as_str(), expected),
                    other => panic!("repr returned non-string {other:?}"),
                }
            }
            match native(
                &Args::positional(vec![Value::Decimal(Decimal::new(-125, 2))]),
                features,
            )
            .unwrap()
            {
                Value::Decimal(actual) => assert_eq!(actual.to_string(), "1.25"),
                other => panic!("decimal species changed: {other:?}"),
            }
        }
        public_error(
            "#calc.abs(-9223372036854775807 - 1)",
            "-9223372036854775807 - 1",
            "the result is too large",
            None,
        );
        public_error(
            "#calc.abs(2pt + 3em)",
            "2pt + 3em",
            "cannot take absolute value of this length",
            None,
        );
        public_error("#calc.abs([x])", "[x]", CONTENT_ERROR, None);
    }
    #[test]
    fn p1331_fixture_path_constructor_succeeds_before_abs() {
        // This sentinel does not call calc.abs. It establishes the constructor
        // precondition for the unchanged public rejection assertion.
        let world = TestWorld::new("");
        for features in profiles() {
            let (result, warnings) =
                eval_expression_with_features(&world, "path(\"p1331.typ\")", features);
            assert!(warnings.is_empty(), "Path fixture warnings: {warnings:?}");
            match result.expect("Path must be constructed before testing abs") {
                Value::Path(path) => {
                    assert!(matches!(path.root(), VirtualRoot::Project));
                    assert_eq!(path.vpath().get_with_slash(), "/p1331.typ");
                }
                other => panic!("Path constructor changed language type: {other:?}"),
            }
        }
    }
}
