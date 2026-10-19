
#[cfg(test)]
mod v16_c_paragraph_probe {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
    struct MemoryWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl World for MemoryWorld {
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
    fn world(text: &str) -> MemoryWorld {
        let id = FileId::from_raw(std::num::NonZeroU16::new(1).unwrap());
        MemoryWorld {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(id, text.into()),
        }
    }

    fn inserted(input: Content) -> Content {
        use crate::compiler::eval::{EvalContext, EvalTarget, Scopes};
        use crate::entities::world_types::{Route, Sink};
        use comemo::Track;
        let world = world("#show par: it => {metadata(\"PAR\");it};#injected");
        let library = Library::with_global(crate::compiler::eval::make_stdlib(
            &crate::contracts::world::SysInputs::default(),
        ));
        let mut library = library;
        library.global.define(
            "text",
            Value::Func(crate::entities::func::Func::native(
                "text",
                crate::compiler::stdlib::native_text,
            )),
        );
        let mut scopes = Scopes::new(Some(&library));
        scopes.define("injected", Value::Content(input));
        let metrics = crate::compiler::layout::FixedMetrics;
        let route = Route::root().with_id(world.main());
        let mut styles = crate::entities::style_chain::StyleChain::default_chain();
        let mut rules = Arc::from([]);
        let mut guards = Vec::new();
        let mut sink = Sink::new();
        let mut tracked = sink.track_mut();
        let mut engine = crate::entities::engine::Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked,
        };
        let mut ctx = EvalContext::new();
        ctx.defer_show_rules = true;
        let Value::Content(content) = crate::compiler::eval::eval_markup(
            world.source.root(),
            &mut scopes,
            &mut ctx,
            &mut engine,
        )
        .expect("source-level insertion must evaluate") else {
            panic!("expected content")
        };
        let registry = crate::entities::element_registry::ElementRegistry::new();
        let previous = crate::entities::introspector::TagIntrospector::empty();
        crate::compiler::realize::realize_document(
            &content,
            &world,
            &world.source,
            &registry,
            &previous,
            false,
            EvalTarget::Paged,
            crate::entities::compiler_features::Features::default(),
        )
        .0
        .expect("actual realization must complete")
        .visual
    }
    fn observations(content: &Content) -> (usize, Vec<String>) {
        let mut effects = 0;
        let mut paragraphs = Vec::new();
        content.visit_content_static(&mut |node| {
            if let Content::Metadata(m) = node {
                if m.value.as_ref() == &Value::Str("PAR".into()) {
                    effects += 1;
                }
            }
            if let Content::Par { body } = node {
                paragraphs.push(body.plain_text().to_string());
            }
            true
        });
        (effects, paragraphs)
    }
    #[test]
    fn v16_c_paragraph_carrier_flow_diagnostic() {
        use crate::entities::content::{LocatedContent, RealizedContent};
        let par = Content::par(Content::text("A"));
        let location = crate::entities::locator::Locator::new().next();
        let cases = [
            ("raw", par.clone()),
            (
                "located",
                Content::Located(Arc::new(LocatedContent {
                    location,
                    body: par.clone(),
                })),
            ),
            (
                "visual_par",
                Content::Realized(Arc::new(RealizedContent {
                    original: Content::text("ORIGINAL"),
                    visual: par.clone(),
                })),
            ),
            (
                "original_par",
                Content::Realized(Arc::new(RealizedContent {
                    original: par,
                    visual: Content::text("VISUAL"),
                })),
            ),
        ];
        let records = cases
            .into_iter()
            .map(|(name, input)| {
                let flow = realize_flow(&Content::sequence(vec![
                    Content::text("PREFIX"),
                    input.clone(),
                    Content::text("SUFFIX"),
                ]));
                let flow = Content::sequence(flow);
                let output = inserted(input);
                (name, observations(&flow), observations(&output))
            })
            .collect::<Vec<_>>();
        panic!("temporary carrier diagnostic: {records:?}");
    }
}
