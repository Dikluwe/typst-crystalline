#[cfg(test)]
mod p1332_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
        EvalTarget,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{
        Severity, SourceDiagnostic, SourceResult, Tracepoint,
    };
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route, Routines, Traced,
    };
    use comemo::Track;
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

    #[test]
    fn p1332_intrinsic_name_follows_lookup_alias_import_and_nested_with() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expression in [
                "calc.abs",
                "std.calc.abs",
                "{let a=calc.abs; a}",
                "{import calc: abs; abs}",
                "calc.abs.with()",
                "calc.abs.with(-19)",
                "calc.abs.with().with(-19)",
                "{import calc: abs; let a=abs.with(-19).with(); a}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty(), "{expression}: {warnings:?}");
                match result.expect(expression) {
                    Value::Func(function) => {
                        assert_eq!(function.name(), Some("abs"), "{expression}")
                    }
                    other => panic!("function species lost: {expression}: {other:?}"),
                }
            }
            for (expression, expected) in [
                ("calc.sqrt", "calc.sqrt"),
                ("calc.pow", "calc.pow"),
                ("calc.sqrt.with(-1)", "calc.sqrt"),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.unwrap() {
                    Value::Func(function) => assert_eq!(function.name(), Some(expected)),
                    other => panic!("other function species lost: {other:?}"),
                }
            }
        }
    }
    #[test]
    fn p1332_language_identity_and_with_result_are_preserved() {
        let world = TestWorld::new("");
        for features in profiles() {
            for expression in [
                "calc.abs == std.calc.abs",
                "{import calc: abs; let a=abs; a == calc.abs}",
                "calc.abs != calc.sqrt",
                "calc.abs != calc.pow",
                "calc.abs != (x => x)",
                "{let a=calc.abs.with(-19); let b=a; a == b}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                assert!(
                    matches!(result.expect(expression), Value::Bool(true)),
                    "{expression}"
                );
            }
            for expression in [
                "{import calc: abs; abs(-9007199254740993)}",
                "{import calc: abs; let a=abs.with().with(-9007199254740993); a()}",
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.expect(expression) {
                    Value::Int(actual) => assert_eq!(actual, 9007199254740993),
                    other => panic!("exact integer result changed: {other:?}"),
                }
            }
            for (expression, expected) in [
                ("repr(calc.abs)", "abs"),
                ("{import calc: abs; repr(abs)}", "abs"),
                ("{let a=calc.abs; repr(a)}", "abs"),
                ("repr(calc.abs.with(-1))", "(..) => .."),
                ("repr(calc.abs.with().with(-1))", "(..) => .."),
                ("repr(calc.sqrt)", "sqrt"),
            ] {
                let (result, warnings) =
                    eval_expression_with_features(&world, expression, features);
                assert!(warnings.is_empty());
                match result.expect(expression) {
                    Value::Str(actual) => assert_eq!(actual.as_str(), expected),
                    other => panic!("repr result species changed: {other:?}"),
                }
            }
        }
    }

    fn traced_error(
        text: &str,
        anchor: Option<&str>,
        message: &str,
        call: &str,
        name: &str,
        warning: bool,
    ) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert_eq!(warnings.len(), usize::from(warning), "{text}: {warnings:?}");
            if warning {
                let w = &warnings[0];
                assert_eq!(w.message, "this import has no effect");
                assert_eq!(w.severity, Severity::Warning);
                assert!(w.hints.is_empty());
                assert!(w.trace.is_empty());
                let start = text.find("std").unwrap();
                assert_eq!(source.span_byte_range(w.span), Some(start..start + 3));
            }
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}: {errors:?}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty());
            if let Some(anchor) = anchor {
                let start = text.rfind(anchor).unwrap();
                assert_eq!(
                    source.span_byte_range(d.span),
                    Some(start..start + anchor.len()),
                    "{text}"
                );
            } else {
                assert!(d.span.is_detached(), "{text}: {d:?}");
            }
            if message == "missing argument: value" {
                assert!(d.trace.is_empty(), "{text}: {d:?}");
                continue;
            }
            assert_eq!(d.trace.len(), 1, "{text}: {d:?}");
            assert_eq!(d.trace[0].v, Tracepoint::Call(Some(name.into())));
            let start = text.rfind(call).unwrap();
            assert_eq!(
                source.span_byte_range(d.trace[0].span),
                Some(start..start + call.len())
            );
        }
    }

    #[test]
    fn p1332_imported_abs_external_origins_all_error_families() {
        for (value, message) in [
            ("[x]", "expected integer, float, length, angle, ratio, fraction, or decimal, found content"),
            ("2pt + 3em", "cannot take absolute value of this length"),
            ("-9223372036854775807 - 1", "the result is too large"),
            ("false", "expected integer, float, length, angle, ratio, fraction, or decimal, found boolean"),
            ("\"x\"", "expected integer, float, length, angle, ratio, fraction, or decimal, found string"),
        ] {
            for text in [
                format!("#import calc: abs\n#let a=abs.with({value})\n#a()"),
                format!("#import calc: abs\n#let a=abs.with({value}).with()\n#a()"),
                format!("#import calc: abs\n#let aa=arguments({value})\n#abs(..aa)"),
            ] {
                let call = if text.ends_with("#a()") { "a()" } else { "abs(..aa)" };
                traced_error(&text, Some(value), message, call, "abs", false);
            }
        }
    }
    #[test]
    fn p1332_trace_rename_retains_warning_utf8_and_guard_debt() {
        traced_error(
            "#import std\n#let café=calc.abs.with([á])\n#café()",
            Some("[á]"),
            "expected integer, float, length, angle, ratio, fraction, or decimal, found content",
            "café()", "abs", true,
        );
        for (text, message, call) in [
            ("#calc.abs()", "missing argument: value", "calc.abs()"),
            (
                "#let a=calc.abs.with()\n#a()",
                "missing argument: value",
                "a()",
            ),
            (
                "#let a=calc.abs.with(false, 1)\n#a()",
                "expected integer, float, length, angle, ratio, fraction, or decimal, found boolean",
                "a()",
            ),
            (
                "#let a=calc.abs.with(false, bad: 1)\n#a()",
                "expected integer, float, length, angle, ratio, fraction, or decimal, found boolean",
                "a()",
            ),
        ] {
            traced_error(text, if text.contains("with(false,") { Some("false") } else { Some(call) }, message, call, "abs", false);
        }
        traced_error(
            "#let a=calc.sqrt.with(\"x\")\n#a()",
            None,
            "calc.sqrt(): esperava Int ou Float, recebeu str",
            "a()",
            "calc.sqrt",
            false,
        );
    }
}
