#[cfg(test)]
mod p1326_tests {
    use crate::compiler::eval::{
        eval_expression_with_features, eval_with_full_error_target_and_features,
    };
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::module::Module;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::{Severity, SourceDiagnostic, SourceResult};
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
        let source = Source::detached(text);
        let world = TestWorld {
            source: source.clone(),
            library: Library::new(),
            book: FontBook::new(),
        };
        let mut sink = Sink::new();
        let result = eval_with_full_error_target_and_features(
            &Routines::new(),
            &world,
            Traced::default().track(),
            sink.track_mut(),
            Route::root().track(),
            &source,
            &ElementRegistry::new(),
            false,
            crate::compiler::eval::EvalTarget::Paged,
            features,
        );
        (source, result, sink.into_diagnostics())
    }
    fn error(text: &str, anchor: &str, message: &str) {
        for features in profiles() {
            let (source, result, warnings) = full(text, features);
            assert!(warnings.is_empty(), "{text}: {warnings:?}");
            let errors = result.unwrap_err();
            assert_eq!(errors.len(), 1, "{text}");
            let d = &errors[0];
            assert_eq!(d.message, message, "{text}");
            assert_eq!(d.severity, Severity::Error);
            assert!(d.hints.is_empty(), "{text}: {:?}", d.hints);
            assert!(d.trace.is_empty(), "{text}: {:?}", d.trace);
            let start = text.rfind(anchor).unwrap();
            assert_eq!(
                source.span_byte_range(d.span),
                Some(start..start + anchor.len()),
                "{text}"
            );
        }
    }
    #[test]
    fn p1326_full_ast_closure_message_and_exact_identifier() {
        for (text, field) in [
            ("#let f(x) = x\n#f.nope", "nope"),
            ("#((x => x).absent)", "absent"),
            ("#let f(x) = x\n#let alias = f\n#alias.unknown", "unknown"),
            ("#let f(x) = x\n#f.with(7).nope", "nope"),
            (
                "#let f(x, y) = x+y\n#f.with(7).with(8).missing-long-field",
                "missing-long-field",
            ),
            ("#let função(x) = x\n#função.café", "café"),
            ("#let f(x) = x\n#(\n  f\n  .ausência\n)", "ausência"),
            ("#let f(x) = panic(\"must not run\")\n#f.nope(7)", "nope"),
            ("#let f(x) = x\n#let alias = f.with(4)\n#alias.nope()", "nope"),
        ] {
            error(text, field, "cannot access fields on user-defined functions");
        }
    }
    #[test]
    fn p1326_full_ast_native_module_dict_content_float_boundaries() {
        for (text, field, message) in [
            ("#json.nope", "nope", "function `json` does not contain field `nope`"),
            ("#csv.nope", "nope", "function `csv` does not contain field `nope`"),
            ("#calc.nope", "nope", "module `calc` does not contain `nope`"),
            ("#((x: 1).nope)", "nope", "dictionary does not contain key \"nope\""),
            ("#(strong[x].nope)", "nope", "strong does not have field \"nope\""),
            ("#((1.0).nope)", "nope", "cannot access fields on type float"),
        ] {
            error(text, field, message);
        }
    }
    #[test]
    fn p1326_existing_function_calls_with_alias_args_and_kind() {
        for features in profiles() {
            let world = TestWorld {
                source: Source::detached(""),
                library: Library::new(),
                book: FontBook::new(),
            };
            for (expr, expected) in [
                ("{let f(x)=x+1; f(8)}", Value::Int(9)),
                ("{let f(x,y)=x+y; let a=f.with(8); a(3)}", Value::Int(11)),
                ("{let f(x,y)=x+y; f.with(8).with(3)()}", Value::Int(11)),
                ("{let f(x)=x; type(f)==function}", Value::Bool(true)),
                ("{let f(x)=x; let a=f; a(19)}", Value::Int(19)),
                ("json.encode((x: 1))", Value::Str("{\"x\":1}".into())),
            ] {
                let (actual, warnings) =
                    eval_expression_with_features(&world, expr, features);
                assert!(warnings.is_empty(), "{expr}");
                assert_eq!(actual.unwrap(), expected, "{expr}");
            }
        }
    }
}
