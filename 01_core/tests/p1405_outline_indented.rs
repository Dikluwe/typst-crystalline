//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1405_outline_indented_tests.md
//! @prompt-hash af6b4562
//! @layer L1
//! @updated 2026-09-15
//!
//! Harness focal e limitado do contrato P1405.

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;
    use std::sync::Arc;

    use comemo::Track;
    use indexmap::IndexMap;
    use rustc_hash::FxBuildHasher;
    use typst_core::compiler::eval::EvalContext;
    use typst_core::compiler::layout::FixedMetrics;
    use typst_core::compiler::scopes::Scopes;
    use typst_core::compiler::stdlib::native_outline_entry_indented;
    use typst_core::contracts::world::World;
    use typst_core::entities::args::Args;
    use typst_core::entities::content::Content;
    use typst_core::entities::dir::Dir;
    use typst_core::entities::engine::Engine;
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::font_book::FontBook;
    use typst_core::entities::func::Func;
    use typst_core::entities::layout_types::Length;
    use typst_core::entities::show::{RuleId, ShowRule};
    use typst_core::entities::sink::Sink;
    use typst_core::entities::source::Source;
    use typst_core::entities::source_result::SourceResult;
    use typst_core::entities::span::Span;
    use typst_core::entities::style_chain::StyleChain;
    use typst_core::entities::value::Value;
    use typst_core::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library, Route,
    };

    #[derive(Default)]
    struct NullWorld {
        library: Library,
        book: FontBook,
    }

    impl World for NullWorld {
        fn library(&self) -> &Library {
            &self.library
        }
        fn book(&self) -> &FontBook {
            &self.book
        }
        fn main(&self) -> FileId {
            FileId::from_raw(NonZeroU16::new(1).unwrap())
        }
        fn source(&self, _: FileId) -> FileResult<Source> {
            Err(FileError::NotFound)
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> {
            Err(FileError::NotFound)
        }
        fn font(&self, _: usize) -> Option<Font> {
            None
        }
        fn today(
            &self,
            _: Option<typst_core::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    fn call_indented(
        mut styles: StyleChain,
        ctx: &mut EvalContext,
        entry: Content,
        prefix: Value,
        inner: Value,
        gap: Option<Length>,
    ) -> SourceResult<Value> {
        let world = NullWorld::default();
        let route = Route::root();
        let mut show_rules: Arc<[ShowRule]> = Arc::from([]);
        let mut guards: Vec<RuleId> = Vec::new();
        let mut sink_value = Sink::new();
        let mut sink = sink_value.track_mut();
        let metrics = FixedMetrics;
        let mut engine = Engine {
            world: &world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut show_rules,
            active_guards: &mut guards,
            current_file: world.main(),
            sink: &mut sink,
        };
        let mut named: IndexMap<_, _, FxBuildHasher> = IndexMap::default();
        if let Some(gap) = gap {
            named.insert("gap".into(), Value::Length(gap));
        }
        let args = Args::from_parts(
            vec![Value::Content(entry), prefix, inner],
            named,
            Span::detached(),
        );
        native_outline_entry_indented(
            ctx,
            &args,
            &world,
            world.main(),
            &mut Scopes::new(None),
            &mut engine,
        )
    }

    fn parent_styles(indent: Value, key: i64) -> StyleChain {
        StyleChain::default_chain()
            .push_custom("outline.parent.indent", indent)
            .push_custom("outline.parent.key", Value::Int(key))
    }

    fn entry(level: usize) -> Content {
        Content::outline_entry(
            level,
            Content::heading(level.try_into().unwrap(), Content::text("H")),
            None,
        )
    }

    #[test]
    fn fixed_indent_multiplies_zero_based_level_and_preserves_content() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let value = call_indented(
            parent_styles(Value::Length(Length::em(2.0)), 7),
            &mut ctx,
            entry(2),
            Value::Content(Content::strong(Content::text("P"))),
            Value::Content(Content::emph(Content::text("I"))),
            Some(Length::em(0.5)),
        )
        .unwrap();
        let Value::Content(Content::Block(block)) = value else {
            panic!("esperado block")
        };
        assert!(block.inset.left.em >= 2.0);
        assert_eq!(block.inset.right, Length::ZERO);
        assert_eq!(block.body.plain_text(), "PI");
    }

    #[test]
    fn auto_uses_previous_level_and_current_maximum() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        ctx.introspector
            .outline_prefix_widths
            .insert(7, vec![Some(13.2), Some(26.4)]);
        let value = call_indented(
            parent_styles(Value::Auto, 7),
            &mut ctx,
            entry(2),
            Value::Content(Content::text("P")),
            Value::Content(Content::text("I")),
            None,
        )
        .unwrap();
        let Value::Content(Content::Block(block)) = value else {
            panic!("esperado block")
        };
        assert!((block.inset.left.resolve_pt(11.0) - 39.6).abs() < 1e-9);
        assert_eq!(block.body.plain_text(), "PI");
    }

    fn triple_indent(
        _ctx: &mut EvalContext,
        args: &Args,
        _world: &dyn World,
        _file: FileId,
    ) -> SourceResult<Value> {
        let Some(Value::Int(level)) = args.items.first() else {
            panic!("nível inteiro")
        };
        Ok(Value::Length(Length::em(*level as f64 * 3.0)))
    }

    fn invalid_indent(
        _ctx: &mut EvalContext,
        _args: &Args,
        _world: &dyn World,
        _file: FileId,
    ) -> SourceResult<Value> {
        Ok(Value::Int(3))
    }

    #[test]
    fn callback_receives_zero_based_level_once() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let value = call_indented(
            parent_styles(Value::Func(Func::native("triple", triple_indent)), 3),
            &mut ctx,
            entry(2),
            Value::None,
            Value::Content(Content::text("I")),
            None,
        )
        .unwrap();
        let Value::Content(Content::Block(block)) = value else {
            panic!("esperado block")
        };
        assert_eq!(block.inset.left, Length::em(3.0));
        assert_eq!(block.body, Content::text("I"));
    }

    #[test]
    fn missing_parent_matches_contextual_contract() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let error = call_indented(
            StyleChain::default_chain(),
            &mut ctx,
            entry(1),
            Value::Content(Content::text("P")),
            Value::Content(Content::text("I")),
            None,
        )
        .unwrap_err();
        assert_eq!(error[0].message, "must be called within the context of an outline");
    }

    #[test]
    fn auto_without_prefix_uses_only_previous_level_fallback() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let value = call_indented(
            parent_styles(Value::Auto, 11),
            &mut ctx,
            entry(2),
            Value::None,
            Value::Content(Content::text("I")),
            None,
        )
        .unwrap();
        let Value::Content(Content::Block(block)) = value else {
            panic!("esperado block")
        };
        assert!((block.inset.left.resolve_pt(11.0) - 13.2).abs() < 1e-9);
        assert_eq!(block.body, Content::text("I"));
    }

    #[test]
    fn rtl_uses_logical_right_inset() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let styles = parent_styles(Value::Length(Length::em(2.0)), 12)
            .push_custom("text.dir", Value::Dir(Dir::RTL));
        let value = call_indented(
            styles,
            &mut ctx,
            entry(2),
            Value::Content(Content::text("P")),
            Value::Content(Content::text("I")),
            None,
        )
        .unwrap();
        let Value::Content(Content::Block(block)) = value else {
            panic!("esperado block")
        };
        assert_eq!(block.inset.left, Length::ZERO);
        assert!(block.inset.right.resolve_pt(11.0) > 22.0);
    }

    #[test]
    fn false_indent_does_not_accumulate_levels_and_gap_is_observed() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let first = call_indented(
            parent_styles(Value::Bool(false), 13),
            &mut ctx,
            entry(1),
            Value::Content(Content::text("P")),
            Value::Content(Content::text("I")),
            None,
        )
        .unwrap();
        let second = call_indented(
            parent_styles(Value::Bool(false), 13),
            &mut ctx,
            entry(3),
            Value::Content(Content::text("P")),
            Value::Content(Content::text("I")),
            Some(Length::em(1.0)),
        )
        .unwrap();
        let Value::Content(Content::Block(first)) = first else {
            panic!("esperado block")
        };
        let Value::Content(Content::Block(second)) = second else {
            panic!("esperado block")
        };
        assert!(
            (second.inset.left.resolve_pt(11.0)
                - first.inset.left.resolve_pt(11.0)
                - 5.5)
                .abs()
                < 1e-9
        );
    }

    #[test]
    fn callback_must_return_length() {
        let mut ctx = EvalContext::new();
        ctx.in_context = true;
        let error = call_indented(
            parent_styles(Value::Func(Func::native("invalid", invalid_indent)), 14),
            &mut ctx,
            entry(1),
            Value::None,
            Value::Content(Content::text("I")),
            None,
        )
        .unwrap_err();
        assert_eq!(error[0].message, "expected length, found int");
    }
}
