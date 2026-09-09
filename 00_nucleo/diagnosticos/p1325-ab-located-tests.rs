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
    use crate::entities::route::Route;
    use crate::entities::sink::Sink;
    use crate::entities::source::Source;
    use crate::entities::style::StyleChain;
    use crate::entities::value::{IntrospectedContent, Value};
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
