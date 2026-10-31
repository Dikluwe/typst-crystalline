//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/whole_recipe.md
//! @prompt-hash 33656163
//! @layer L1
//! @updated 2026-10-29

use super::*;
use crate::entities::args::Args;
use crate::entities::show::{RuleId, Transformation};

pub(super) struct WholeRecipe {
    transform: Transformation,
    styles: StyleChain,
    rules: Arc<[ShowRule]>,
    guards: Vec<RuleId>,
    file: FileId,
    span: Span,
    call_span: Span,
}

impl WholeRecipe {
    pub(super) fn capture(
        rule: crate::entities::ast::code::ShowRule<'_>,
        scopes: &mut Scopes<'_>,
        ctx: &mut EvalContext,
        engine: &mut Engine<'_>,
    ) -> SourceResult<Self> {
        let transform = rules::capture_show_transformation(rule, scopes, ctx, engine)?;
        Ok(Self {
            transform,
            styles: engine.styles.clone(),
            rules: engine.show_rules.clone(),
            guards: engine.active_guards.clone(),
            file: engine.current_file,
            span: rule.transform().span(),
            call_span: rule.to_untyped().span(),
        })
    }

    pub(super) fn apply(
        self,
        body: Content,
        ctx: &mut EvalContext,
        engine: &mut Engine<'_>,
    ) -> SourceResult<Content> {
        let Self {
            transform,
            styles,
            rules,
            guards,
            file,
            span,
            call_span,
        } = self;
        let previous_styles = std::mem::replace(engine.styles, styles);
        let previous_rules = std::mem::replace(engine.show_rules, rules);
        let previous_guards = std::mem::replace(engine.active_guards, guards);
        let previous_file = std::mem::replace(&mut engine.current_file, file);
        let previous_defer = std::mem::replace(&mut ctx.defer_show_rules, true);
        let caller_flow = ctx.flow.take();
        let result = (|| {
            let (mut content, raw_display) = match transform {
                Transformation::Content(content) => (content, false),
                Transformation::Str(text) => (Content::text(text.as_str()), false),
                Transformation::Style(styles) => {
                    (Content::styled(body, (*styles).clone()), false)
                }
                Transformation::Func(func) => {
                    let origin = if func.diagnostic_span().is_detached() {
                        span
                    } else {
                        func.diagnostic_span()
                    };
                    let mut args = Args::positional(vec![Value::Content(body)]);
                    args.span = origin;
                    let mut scopes = Scopes::new(None);
                    let result = call_dispatch::apply_func(
                        func.clone(),
                        args,
                        &mut scopes,
                        ctx,
                        engine,
                    )
                    .map_err(|mut errors| {
                        for error in &mut errors {
                            if error.span.is_detached() {
                                error.span = origin;
                            }
                        }
                        errors
                    });
                    let value = call_dispatch::trace_plugin::trace_call(
                        result,
                        func.name(),
                        call_span,
                        engine,
                    )?;
                    let raw_display =
                        matches!(value, Value::Bool(_) | Value::Auto | Value::Length(_));
                    (
                        value_to_display_content(value).unwrap_or(Content::Empty),
                        raw_display,
                    )
                }
            };
            if raw_display {
                content = rules::intercept_content(content, ctx, engine)?;
            }
            content =
                crate::compiler::realize::capture_table_insertion(content, ctx, engine)?;
            content = crate::compiler::realize::sequences::capture_at(
                content, span, ctx, engine,
            );
            content = rules::textual::capture_local_output(content, ctx, engine);
            Ok(content.with_source_span(span))
        })();
        ctx.flow = caller_flow;
        ctx.defer_show_rules = previous_defer;
        *engine.styles = previous_styles;
        *engine.show_rules = previous_rules;
        *engine.active_guards = previous_guards;
        engine.current_file = previous_file;
        result
    }
}

#[cfg(test)]
mod tests {
    use crate::contracts::world::World;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::layout_types::{FrameItem, FrameVisitor};
    use crate::entities::module::Module;
    use crate::entities::source::Source;
    use crate::entities::source_result::{SourceDiagnostic, SourceResult};
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };

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
                .expect("valid selectorless source fixture"),
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

    fn run(text: &str) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
        let world = TestWorld::new(text);
        let (result, sink) =
            super::super::tests::eval_for_test_keep_sink(&world, &world.source);
        (result, sink.into_diagnostics())
    }

    fn plain(text: &str) -> String {
        run(text)
            .0
            .expect("whole-tail source must evaluate")
            .content()
            .expect("markup content")
            .plain_text()
            .trim()
            .to_string()
    }

    fn sizes(text: &str) -> Vec<(String, f64)> {
        let module = run(text).0.expect("style source must evaluate");
        let doc = crate::compiler::layout::layout(module.content().unwrap());
        struct Sizes(Vec<(String, f64)>);
        impl FrameVisitor for Sizes {
            fn visit_text(&mut self, item: &FrameItem) {
                if let FrameItem::Text { text, style, .. } = item {
                    if !text.trim().is_empty() {
                        self.0.push((text.trim().to_string(), style.size.0));
                    }
                }
            }
        }
        let mut visitor = Sizes(Vec::new());
        for page in &doc.pages {
            visitor.visit_items(&page.items);
        }
        visitor.0
    }

    fn error_contains(text: &str, message: &str) {
        let errors = run(text).0.expect_err("source must fail at measured operation");
        assert!(errors.iter().any(|error| error.message.contains(message)), "{errors:?}");
    }

    fn error_origin(text: &str, token: &str, message: &str) {
        let world = TestWorld::new(text);
        let errors = super::super::tests::eval_for_test(&world, &world.source)
            .expect_err("measured argument error");
        let error = errors
            .iter()
            .find(|error| error.message.contains(message))
            .unwrap_or_else(|| panic!("causal argument error missing: {errors:?}"));
        let start = text.find(token).unwrap();
        assert_eq!(
            world.source.span_byte_range(error.span),
            Some(start..start + token.len())
        );
    }

    #[test]
    fn selectorless_markup_identity_preserves_body() {
        assert_eq!(plain("#show: body => body\nBODY"), "BODY");
    }

    #[test]
    fn selectorless_markup_nested_tails_close_inside_out() {
        assert_eq!(
            plain("#show: body => [OUTER #body]\n#show: body => [INNER #body]\nBODY"),
            "OUTER INNER BODY"
        );
    }

    #[test]
    fn selectorless_prefix_stays_outside_tail() {
        assert_eq!(
            plain("PREFIX\n#show: body => [LABEL #body]\nBODY"),
            "PREFIX LABEL BODY"
        );
    }

    #[test]
    fn selectorless_code_prefix_preserves_join() {
        assert_eq!(
            plain("#{[PREFIX]; show: body => [LABEL #body]; [BODY]}"),
            "PREFIXLABEL BODY"
        );
    }

    #[test]
    fn selectorless_code_nested_tails_close_inside_out() {
        assert_eq!(
            plain("#{show: body => [OUTER #body]; show: body => [INNER #body]; [BODY]}"),
            "OUTER INNER BODY"
        );
    }

    #[test]
    fn selectorless_markup_declaration_style_and_body_style_are_distinct() {
        assert_eq!(sizes("#set text(size:10pt)\n#show: body => [LABEL #body]\n#set text(size:20pt)\nBODY"), vec![("LABEL".into(),10.0),("BODY".into(),20.0)]);
    }

    #[test]
    fn selectorless_code_declaration_style_and_body_style_are_distinct() {
        assert_eq!(sizes("#set text(size:10pt)\n#{show: body => [LABEL #body]; set text(size:20pt); [BODY]}"), vec![("LABEL".into(),10.0),("BODY".into(),20.0)]);
    }

    #[test]
    fn selectorless_same_index_style_boundaries_keep_lexical_order() {
        assert_eq!(sizes("#set text(size:12pt)\n#show: body => [OUTER #body]\n#set text(size:14pt)\n#show: body => [INNER #body]\n#set text(size:18pt)\nBODY"), vec![("OUTER".into(),12.0),("INNER".into(),14.0),("BODY".into(),18.0)]);
    }

    #[test]
    fn selectorless_bound_function_template_is_callable() {
        assert_eq!(
            sizes("#show: text.with(size:20pt)\nBODY"),
            vec![("BODY".into(), 20.0)]
        );
    }

    #[test]
    fn selectorless_none_discards_tail() {
        assert_eq!(plain("#show: body => none\nBODY"), "");
    }

    #[test]
    fn selectorless_scalar_result_uses_display() {
        assert_eq!(plain("#show: body => 123\nBODY"), "123");
    }

    #[test]
    fn selectorless_constant_string_replaces_tail() {
        assert_eq!(plain("#show: \"REPLACED\"\nBODY"), "REPLACED");
    }

    #[test]
    fn selectorless_whole_show_set_styles_body() {
        assert_eq!(
            sizes("#show: set text(size:20pt)\nBODY"),
            vec![("BODY".into(), 20.0)]
        );
    }

    #[test]
    fn selectorless_function_captures_original_variables() {
        assert_eq!(plain("#let prefix = \"OLD\"\n#show: body => [#prefix #body]\n#let prefix = \"NEW\"\nBODY"), "OLD BODY");
    }

    #[test]
    fn selectorless_rich_strong_is_not_flattened() {
        let module = run("#show: body => strong(body)\nBODY").0.unwrap();
        let mut strong = 0;
        module.content().unwrap().visit_content_static(&mut |content| {
            if matches!(content, crate::entities::content::Content::Strong(_)) {
                strong += 1;
            }
            true
        });
        assert_eq!(strong, 1);
    }

    #[test]
    fn selectorless_identity_keeps_heading_morphology() {
        let module = run("#show: body => body\n= TITLE").0.unwrap();
        let mut headings = 0;
        module.content().unwrap().visit_content_static(&mut |content| {
            if matches!(content, crate::entities::content::Content::Heading(_)) {
                headings += 1;
            }
            true
        });
        assert_eq!(headings, 1);
    }

    #[test]
    fn selectorless_hides_selected_child_callbacks() {
        assert_eq!(
            plain("#show strong: body => panic(\"HIDDEN\")\n#show: body => [OK]\n*bad*"),
            "OK"
        );
    }

    #[test]
    fn selectorless_inside_selected_callback_hides_children() {
        assert_eq!(plain("#show heading: it => {show: body => [OK]; strong[bad]}\n#show strong: body => panic(\"HIDDEN\")\n= TITLE"), "OK");
    }

    #[test]
    fn selectorless_does_not_hide_ordinary_body_evaluation() {
        error_contains("#show: body => [OK]\n#strong(panic(\"BODY\"))", "BODY");
    }

    #[test]
    fn selectorless_body_error_precedes_application_error() {
        error_contains("#show: body => panic(\"APPLY\")\n#panic(\"BODY\")", "BODY");
    }

    #[test]
    fn selectorless_invalid_transform_precedes_body() {
        error_origin(
            "#show: 1\n#panic(\"BODY\")",
            "1",
            "expected content or function, found integer",
        );
    }

    #[test]
    fn selectorless_native_argument_origin_is_transform() {
        error_origin("#show: assert\nBODY", "assert", "expected boolean");
    }

    #[test]
    fn selectorless_closure_arity_origin_is_parameter_list() {
        error_origin("#show: () => []\nBODY", "()", "unexpected argument");
        error_origin(
            "#show: (body, extra) => body\nBODY",
            "(body, extra)",
            "missing argument: extra",
        );
    }

    #[test]
    fn selectorless_alias_error_keeps_definition_and_show_trace() {
        let text = "#let f = assert\n#show: f\nBODY";
        let world = TestWorld::new(text);
        let errors =
            super::super::tests::eval_for_test(&world, &world.source).unwrap_err();
        let error = errors
            .iter()
            .find(|x| x.message.contains("expected boolean"))
            .unwrap_or_else(|| panic!("native alias error missing: {errors:?}"));
        let start = text.find("assert").unwrap();
        assert_eq!(world.source.span_byte_range(error.span), Some(start..start + 6));
        assert_eq!(error.trace.len(), 1);
        assert_eq!(
            world.source.span_byte_range(error.trace[0].span),
            Some(text.find("show:").unwrap()..text.find("\nBODY").unwrap())
        );
    }

    #[test]
    fn selectorless_missing_argument_is_checked_after_body() {
        error_contains("#show: (body, extra) => body\n#panic(\"BODY\")", "BODY");
    }

    #[test]
    fn selectorless_return_preserves_result_and_discard_warning() {
        let (result, warnings) =
            run("#let f() = {show: body => [WRAP #body]; return [RESULT]}\n#f()");
        assert_eq!(result.unwrap().content().unwrap().plain_text().trim(), "RESULT");
        assert_eq!(
            warnings
                .iter()
                .filter(|x| x.message
                    == "this return unconditionally discards the content before it")
                .count(),
            1
        );
    }

    #[test]
    fn selectorless_break_retains_transformed_prefix() {
        assert_eq!(
            plain("#for i in range(1) {show: body => [WRAP #body]; [LOST]; break}"),
            "WRAP LOST"
        );
    }

    #[test]
    fn selectorless_open_return_tail_still_applies() {
        error_contains(
            "#let f() = {show: body => panic(\"APPLY\"); [LOST]; return [RESULT]}\n#f()",
            "APPLY",
        );
    }

    #[test]
    fn selectorless_open_break_tail_still_applies() {
        error_contains(
            "#for i in range(1) {show: body => panic(\"APPLY\"); [LOST]; break}",
            "APPLY",
        );
    }

    #[test]
    fn selectorless_eighty_sequential_identities_have_no_recursive_tail_limit() {
        assert_eq!(plain(&("#show: body => body\n".repeat(80) + "BODY")), "BODY");
    }

    #[test]
    fn selectorless_256_sequential_identities_have_no_recursive_tail_limit() {
        assert_eq!(plain(&("#show: body => body\n".repeat(256) + "BODY")), "BODY");
    }

    #[test]
    fn selectorless_identity_does_not_repeat_selected_text_rule() {
        assert_eq!(plain("#show \"A\": \"AA\"\n#show: body => body\nA"), "AA");
    }

    #[test]
    fn selectorless_new_string_obeys_lexical_selected_rule() {
        assert_eq!(plain("#show \"A\": \"B\"\n#show: body => \"A\"\nTAIL"), "B");
    }

    #[test]
    fn selectorless_existing_selected_show_control() {
        assert_eq!(plain("#show \"BODY\": \"TEXT\"\nBODY"), "TEXT");
    }
}
