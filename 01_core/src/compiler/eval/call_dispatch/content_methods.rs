//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/call_dispatch/content_methods.md
//! @prompt-hash a6ea2180
//! @layer L1
//! @updated 2026-10-16

use super::*;

/// Intercepta Location, Content e OutlineEntry após os demais métodos.
pub(super) fn try_content_methods(
    call: FuncCallNode<'_>,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    cached_receiver: &mut Option<Value>,
) -> SourceResult<Option<Value>> {
    // **P792** — Métodos de `Location`: `loc.page()`, `loc.position()`,
    // `loc.page-numbering()` — paridade vanilla `location.rs #[scope]`.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "page" | "position" | "page-numbering") {
            let target = super::call::resolve_receiver(
                access,
                cached_receiver,
                scopes,
                ctx,
                engine,
            )?;
            if let Value::Location(loc) = target {
                let _args = eval_args(call.args(), scopes, ctx, engine)?;
                return eval_location_method(*loc, method, ctx, call.span()).map(Some);
            }
        }
    }

    // **P829-B** — Métodos de instância de `Value::Content`:
    // `func`/`has`/`at`/`fields`/`location` — paridade vanilla `Content`
    // `#[scope]` (`foundations/content/mod.rs:510-590`). Corre depois de
    // todos os despachos legítimos acima (nenhum intercepta estes nomes para
    // Content) e antes do fallback P815, que de outra forma reportaria
    // `element strong has no method `func`` (medido m14/b1).
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "func" | "has" | "at" | "fields" | "location") {
            let target = super::call::resolve_receiver(
                access,
                cached_receiver,
                scopes,
                ctx,
                engine,
            )?;
            if let Value::Content(c) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_content_method(c, method, args, call.span())
                    .map(Some);
            }
            if let Value::LocatedContent(c, loc) = target {
                let args = eval_args(call.args(), scopes, ctx, engine)?;
                return bindings::eval_introspected_content_method_at(
                    c,
                    *loc,
                    method,
                    args,
                    call.span(),
                )
                .map(Some);
            }
        }
    }

    // P1402/P1403 — métodos próprios da entry.
    if let Expr::FieldAccess(access) = call.callee() {
        let method = access.field().as_str();
        if matches!(method, "body" | "prefix" | "page" | "inner" | "indented") {
            let target = super::call::resolve_receiver(
                access,
                cached_receiver,
                scopes,
                ctx,
                engine,
            )?;
            let content = match target {
                Value::Content(content) => Some(content),
                Value::LocatedContent(content, _) => Some(content.content()),
                Value::None
                | Value::Bool(..)
                | Value::Int(..)
                | Value::Float(..)
                | Value::Str(..)
                | Value::Array(..)
                | Value::Dict(..)
                | Value::Module(..)
                | Value::Datetime(..)
                | Value::Func(..)
                | Value::Auto
                | Value::Length(..)
                | Value::Relative(..)
                | Value::Ratio(..)
                | Value::Angle(..)
                | Value::Color(..)
                | Value::Stroke(..)
                | Value::Fraction(..)
                | Value::Align(..)
                | Value::Location(..)
                | Value::Gradient(..)
                | Value::Regex(..)
                | Value::Tiling(..)
                | Value::Bytes(..)
                | Value::Decimal(..)
                | Value::Duration(..)
                | Value::Version(..)
                | Value::Selector(..)
                | Value::Symbol(..)
                | Value::Args(..)
                | Value::State(..)
                | Value::Counter(..)
                | Value::Label(..)
                | Value::Dir(..)
                | Value::Path(..)
                | Value::Type(..)
                | Value::SpotColorant(..) => None,
            };
            if let Some(content @ crate::entities::content::Content::OutlineEntry(_)) =
                content.and_then(outline_entry_receiver)
            {
                let mut args = eval_args(call.args(), scopes, ctx, engine)?;
                if method == "body" {
                    return bindings::eval_outline_entry_body_method(
                        content,
                        args,
                        call.span(),
                    )
                    .map(Some);
                }
                args.items.insert(0, Value::Content(content.clone()));
                args.occurrences = None;
                let world = engine.world;
                let current_file = engine.current_file;
                return match method {
                    "prefix" => {
                        native_outline_entry_prefix(ctx, &args, world, current_file)
                    }
                    "page" => native_outline_entry_page(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    "inner" => native_outline_entry_inner(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    "indented" => native_outline_entry_indented(
                        ctx,
                        &args,
                        world,
                        current_file,
                        scopes,
                        engine,
                    ),
                    _ => unreachable!("outline entry method already checked"),
                }
                .map(Some);
            }
        }
    }

    Ok(None)
}

fn outline_entry_receiver(
    content: &crate::entities::content::Content,
) -> Option<&crate::entities::content::Content> {
    use crate::entities::content::Content;
    let content = content.source_body();
    match content {
        Content::Located(located) => outline_entry_receiver(&located.body),
        Content::OutlineEntry(_) => Some(content),
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::Sequence(_)
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::Symbol(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Styled(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => None,
    }
}

#[cfg(test)]
mod v16_walk_receiver_tests {
    use super::*;
    use crate::entities::{
        content::{Content, LocatedContent, RealizedContent},
        file_id::FileId,
        locator::Locator,
        span::Span,
    };
    use std::sync::Arc;

    fn source(body: Content) -> Content {
        body.with_source_span(Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            2..5,
        ))
    }
    fn located(body: Content) -> Content {
        Content::Located(Arc::new(LocatedContent {
            location: Locator::new().next(),
            body,
        }))
    }

    #[test]
    fn v16_walk_receiver_resolves_borrowed_entry_through_source_and_located() {
        let entry =
            Content::outline_entry(1, Content::heading(1, Content::text("h")), None);
        for content in
            [entry.clone(), source(located(entry.clone())), located(source(entry))]
        {
            let resolved = outline_entry_receiver(&content).expect("entry receiver lost");
            let Content::OutlineEntry(entry) = resolved else { panic!("entry lost") };
            assert_eq!(entry.level, 1);
            assert_eq!(entry.body().unwrap().plain_text(), "h");
        }
    }

    #[test]
    fn v16_walk_receiver_does_not_unwrap_other_semantic_nodes() {
        let entry =
            Content::outline_entry(1, Content::heading(1, Content::text("h")), None);
        for content in [
            Content::text("x"),
            source(located(Content::strong(entry.clone()))),
            Content::Realized(Arc::new(RealizedContent {
                original: entry,
                visual: Content::text("v"),
            })),
        ] {
            assert!(outline_entry_receiver(&content).is_none());
        }
    }
}

#[cfg(test)]
mod v16_c_structural_tests {
    use super::*;

    use crate::contracts::world::World;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use crate::entities::{file_id::FileId, font_book::FontBook, source::Source};
    struct TestWorld {
        library: Library,
        book: FontBook,
        source: Source,
    }
    impl TestWorld {
        fn new(text: &str) -> Self {
            Self {
                library: Library::new(),
                book: FontBook::new(),
                source: Source::new(
                    FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
                    text.into(),
                )
                .expect("valid Source test fixture"),
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
        fn include_source(&self, _: FileId, _: &str) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
        fn include_path(
            &self,
            _: &crate::entities::path::RootedPath,
        ) -> Result<Source, String> {
            panic!("refused include accessed World")
        }
    }

    fn with_engine(run: impl FnOnce(&mut Scopes<'_>, &mut EvalContext, &mut Engine<'_>)) {
        use comemo::Track;
        let world = TestWorld::new("");
        let library =
            Library::with_global(crate::compiler::stdlib::make_stdlib_with_features(
                &crate::contracts::world::SysInputs::default(),
                crate::entities::compiler_features::Features::empty(),
            ));
        let mut scopes = Scopes::new(Some(&library));
        let mut ctx = EvalContext::new();
        let mut styles = crate::entities::style_chain::StyleChain::default_chain();
        let mut rules = std::sync::Arc::from([]);
        let mut guards = Vec::new();
        let route = crate::entities::world_types::Route::root().with_id(world.main());
        let metrics = crate::compiler::layout::FixedMetrics;
        let mut sink = crate::entities::sink::Sink::new();
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut tracked_sink,
        };
        run(&mut scopes, &mut ctx, &mut engine);
    }

    #[test]
    fn v16_c_dispatch_refusals_preserve_cache_and_skip_arguments() {
        use crate::entities::ast::AstNode;
        let source = Source::new_with_parser(
            TestWorld::new("").main(),
            "receiver.body(panic(\"argument executed\"))".into(),
            crate::compiler::parse::parse_code,
        )
        .expect("valid Source test fixture");
        let call = source
            .root()
            .children()
            .find_map(|n| {
                if let Some(Expr::FuncCall(c)) = Expr::from_untyped(n) {
                    Some(c)
                } else {
                    None
                }
            })
            .unwrap();
        let values = all_values();
        with_engine(|scopes, ctx, engine| {
            for value in values {
                let before = value.clone();
                let mut cached = Some(value);
                let result =
                    try_content_methods(call, scopes, ctx, engine, &mut cached).unwrap();
                assert!(result.is_none());
                assert_eq!(cached, Some(before));
            }
        });
    }
    fn all_values() -> Vec<Value> {
        let world = TestWorld::new("");
        let mut values = vec![];
        for expression in [
            "none",
            "true",
            "42",
            "1.5",
            "\"A\"",
            "(1,)",
            "(a:1)",
            "std",
            "datetime(year:2020,month:1,day:1)",
            "(x)=>x",
            "[x]",
            "auto",
            "1pt",
            "1em + 10%",
            "10%",
            "1deg",
            "red",
            "stroke()",
            "1fr",
            "left",
            "gradient.linear(red,blue)",
            "regex(\"x\")",
            "tiling(size:(1pt,1pt),[x])",
            "bytes((1,2))",
            "decimal(\"1\")",
            "duration(seconds:1)",
            "version(1,2)",
            "heading.where()",
            "sym.alpha",
            "arguments(1,a:2)",
            "state(\"x\",0)",
            "counter(\"x\")",
            "<x>",
            "ltr",
            "int",
            "color.spot(\"x\",red)",
        ] {
            let (value, _) = crate::compiler::eval::eval_expression(&world, expression);
            values
                .push(value.unwrap_or_else(|errors| panic!("{expression}: {errors:?}")));
        }
        values.push(Value::Path(crate::entities::path::RootedPath::new(
            crate::entities::path::VirtualRoot::Project,
            crate::entities::path::VirtualPath::new("/a").unwrap(),
        )));
        let location = crate::entities::locator::Locator::new().next();
        values.push(Value::Location(location));
        values.push(Value::LocatedContent(
            crate::entities::value::IntrospectedContent::new(
                crate::entities::content::Content::Empty,
                None,
            ),
            location,
        ));
        let kinds: std::collections::HashSet<_> =
            values.iter().map(std::mem::discriminant).collect();
        assert_eq!(values.len(), 39);
        assert_eq!(kinds.len(), 39, "each current Value variant must be exercised");
        values
    }
}
