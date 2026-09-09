#[cfg(test)]
mod p1325_tests {
    use crate::compiler::eval::{eval_with_full_error_target_and_features, EvalTarget};
    use crate::contracts::world::World;
    use crate::entities::compiler_features::{Feature, Features};
    use crate::entities::element_registry::ElementRegistry;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::source_result::Severity;
    use crate::entities::world_types::Route;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Routines, Traced,
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

    fn check(text: &str, needle: &str, message: &str) {
        for mask in 0..4 {
            let world = TestWorld {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            };
            let mut features = Features::empty();
            if mask & 1 != 0 {
                features.enable(Feature::Html);
            }
            if mask & 2 != 0 {
                features.enable(Feature::A11yExtras);
            }
            let routines = Routines::new();
            let traced = Traced::default();
            let mut sink = Sink::new();
            let route = Route::root();
            let registry = ElementRegistry::new();
            let result = eval_with_full_error_target_and_features(
                &routines,
                &world,
                traced.track(),
                sink.track_mut(),
                route.track(),
                &world.source,
                &registry,
                false,
                EvalTarget::Paged,
                features,
            );
            let errors = match result {
                Ok(_) => panic!("expected error for {text}"),
                Err(errors) => errors,
            };
            assert_eq!(errors.len(), 1, "{text}");
            let error = &errors[0];
            assert_eq!(error.severity, Severity::Error);
            assert_eq!(error.message, message, "{text}");
            assert!(error.hints.is_empty(), "{text}");
            assert!(error.trace.is_empty(), "{text}");
            assert!(sink.into_diagnostics().is_empty(), "{text}");
            let start = text.rfind(needle).unwrap();
            assert_eq!(
                world.source.span_byte_range(error.span),
                Some(start..start + needle.len()),
                "{text}"
            );
        }
    }

    #[test]
    fn p1325_dict_direct_alias_multiline_unicode() {
        for (text, field) in [
            ("#let result = (x: 1).nope", "nope"),
            ("#let d = (x: 1)\n#let result = d.long-missing-field", "long-missing-field"),
            (
                "#let d = (x: 1)\n#let alias = d\n#let result = (alias).ausência",
                "ausência",
            ),
            ("#let result = (\n  (x: 1)\n).absent", "absent"),
        ] {
            check(text, field, &format!("dictionary does not contain key \"{field}\""));
        }
    }

    #[test]
    fn p1325_content_direct_alias_and_known_lookup_residual() {
        for (text, field, kind) in [
            ("#let result = [x].nope", "nope", "text"),
            ("#let result = strong[x].absent", "absent", "strong"),
            (
                "#let c = strong[x]\n#let alias = c\n#let result = alias.ausência",
                "ausência",
                "strong",
            ),
            ("#let result = [x].text", "text", "text"),
        ] {
            check(text, field, &format!("{kind} does not have field \"{field}\""));
        }
    }

    #[test]
    fn p1325_float_fields_without_reflection() {
        for (text, field) in [
            ("#let result = (1.0).nope", "nope"),
            ("#let f = 1.0\n#let result = f.is-infinite", "is-infinite"),
            ("#let f = 1.0\n#let result = f.ausência", "ausência"),
            ("#let result = (1.0).is-nan", "is-nan"),
        ] {
            check(text, field, "cannot access fields on type float");
        }
    }

    #[test]
    fn p1325_positive_lookup_values_and_morphology() {
        let world = TestWorld {
            source: Source::detached(""),
            library: Library::new(),
            book: FontBook::new(),
        };
        for expression in [
            "(x: 1).x == 1",
            "(ausência: 17).ausência == 17",
            "{ let d = (body: strong[x]); d.body == strong[x] }",
            "strong[x].body == [x]",
            "(1.0).is-nan() == false",
            "{ let d = (x: (y: 4)); d.x.y == 4 }",
        ] {
            let (result, warnings) =
                crate::compiler::eval::eval_expression(&world, expression);
            assert!(warnings.is_empty());
            assert!(
                matches!(result, Ok(crate::entities::value::Value::Bool(true))),
                "{expression}: {result:?}"
            );
        }
    }
}
#[cfg(test)]
mod p1325_located_tests {
    use crate::compiler::eval::{eval_markup, EvalContext};
    use crate::compiler::layout::FixedMetrics;
    use crate::compiler::scopes::Scopes;
    use crate::contracts::world::World;
    use crate::entities::content::Content;
    use crate::entities::engine::Engine;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::locator::Locator;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::style_chain::StyleChain;
    use crate::entities::value::{IntrospectedContent, Value};
    use crate::entities::world_types::Route;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use comemo::Track;
    use std::sync::Arc;

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

    #[test]
    fn p1325_located_ast_preserves_whole_access_for_snapshot_and_legacy() {
        for snapshot in [false, true] {
            let text = "#subject.nope";
            let world = TestWorld {
                source: Source::detached(text),
                library: Library::new(),
                book: FontBook::new(),
            };
            let mut scopes = Scopes::new(None);
            let mut locator = Locator::new();
            let content = IntrospectedContent::new(
                Content::Text("x".into()),
                snapshot.then(Default::default),
            );
            scopes.define("subject", Value::LocatedContent(content, locator.next()));
            let mut ctx = EvalContext::new();
            let route = Route::root().with_id(world.source.id());
            let mut styles = StyleChain::default_chain();
            let mut rules = Arc::from([]);
            let mut guards = Vec::new();
            let mut sink_local = Sink::new();
            let mut sink = sink_local.track_mut();
            let mut engine = Engine {
                world: &world,
                font_metrics: &FixedMetrics,
                route: route.track(),
                styles: &mut styles,
                show_rules: &mut rules,
                active_guards: &mut guards,
                current_file: world.source.id(),
                sink: &mut sink,
            };
            let errors =
                eval_markup(world.source.root(), &mut scopes, &mut ctx, &mut engine)
                    .unwrap_err();
            assert_eq!(errors.len(), 1);
            assert_eq!(errors[0].message, "text does not have field \"nope\"");
            assert_eq!(world.source.span_byte_range(errors[0].span), Some(1..text.len()));
            assert!(errors[0].hints.is_empty());
            assert!(errors[0].trace.is_empty());
        }
    }
}
