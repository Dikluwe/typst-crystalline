//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/mod.md
//! @prompt-hash 87403de5
//! @layer L1
//! @updated 2026-10-10

pub(in crate::compiler) mod bare_math;
pub mod context_occurrences;
pub(in crate::compiler) mod figure;
pub(in crate::compiler) mod figure_kind;
pub(in crate::compiler) mod recipes;
pub mod reference_supplements;
pub(in crate::compiler) mod sequences;
mod table_cells;

use crate::compiler::eval::{EvalContext, EvalTarget};
use crate::compiler::introspect::{
    extract_payload::extract_payload, introspect_with_introspector,
};
use crate::compiler::layout::FixedMetrics;
use crate::contracts::world::World;
use crate::entities::compiler_features::Features;
use crate::entities::content::{Content, LocatedContent};
use crate::entities::element_registry::ElementRegistry;
use crate::entities::engine::Engine;
use crate::entities::introspector::TagIntrospector;
use crate::entities::show::ShowRule;
use crate::entities::source::Source;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::style_chain::StyleChain;
use crate::entities::world_types::{Route, Sink};
use comemo::Track;
use std::sync::Arc;

pub struct RealizedDocument {
    pub visual: Content,
    pub introspector: TagIntrospector,
}

pub(crate) fn locate(content: Content, ctx: &mut EvalContext) -> Content {
    if matches!(content, Content::Located(_)) {
        return content;
    }
    Content::Located(Arc::new(LocatedContent {
        location: context_occurrences::next(ctx),
        body: content,
    }))
}

fn finish_occurrences(content: &Content, ctx: &mut EvalContext) -> SourceResult<Content> {
    content.map_content(&mut |node| {
        if !matches!(node, Content::Located(_) | Content::Realized(_))
            && extract_payload(node).is_some()
        {
            Ok(Some(locate(node.clone(), ctx)))
        } else {
            Ok(None)
        }
    })
}

fn normalize_insertions(content: &Content) -> SourceResult<Content> {
    fn flatten(content: &Content, parts: &mut Vec<Content>) {
        match content {
            Content::Sequence(children) => {
                for child in children.iter() {
                    flatten(child, parts);
                }
            }
            Content::Realized(e)
                if !crate::compiler::eval::rules::textual::is_product(content) =>
            {
                if matches!(e.visual, Content::Sequence(_)) {
                    parts.push(Content::Realized(Arc::new(
                        crate::entities::content::RealizedContent {
                            original: e.original.clone(),
                            visual: Content::Empty,
                        },
                    )));
                    flatten(&e.visual, parts);
                } else {
                    parts.push(content.clone());
                }
            }
            other => parts.push(other.clone()),
        }
    }
    content.map_content(&mut |node| {
        if let Content::Sequence(children) = node {
            let needs_flattening = children.iter().any(|child| match child {
                Content::Sequence(_) => true,
                Content::Realized(e) => {
                    !crate::compiler::eval::rules::textual::is_product(child)
                        && matches!(e.visual, Content::Sequence(_))
                },
                // These direct-child carriers are not flattenable sequences.
                Content::Source(_)
                | Content::ShowRecipe(_)
                | Content::Located(_)
                | Content::Empty
                | Content::Text(_)
                | Content::Space
                | Content::SpaceFromNewline
                | Content::Parbreak
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
                | Content::OutlineEntry(_)
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
                | Content::Dynamic(_) => false,
            });
            if needs_flattening {
                let mut parts = Vec::with_capacity(children.len());
                for child in children.iter() { flatten(child, &mut parts); }
                crate::compiler::eval::rules::textual::space::normalize_markup_spaces(&mut parts);
                Ok(Some(Content::sequence(parts)))
            } else if children.len() < 2 {
                // Even manually constructed degenerate sequences must canonicalize.
                Ok(Some(Content::sequence(children.to_vec())))
            } else {
                Ok(crate::compiler::eval::rules::textual::space::normalize_markup_spaces_if_changed(children)
                    .map(Content::sequence))
            }
        } else {
            Ok(None)
        }
    })
}

fn observe_occurrences(content: &Content, ctx: &mut EvalContext) {
    let _ = content.map_content(&mut |node| {
        match node {
            Content::Located(e) => {
                ctx.realization_locator.observe(e.location);
                if let Content::ContextBlock(block) = &e.body {
                    ctx.next_context_id =
                        ctx.next_context_id.max(block.id.saturating_add(1));
                }
            }
            Content::ContextBlock(block) => {
                ctx.next_context_id = ctx.next_context_id.max(block.id.saturating_add(1))
            }
            Content::ShowRecipe(recipe) => {
                if sequences::marked(recipe) {
                    ctx.has_deferred_show_rules = true;
                }
                ctx.next_context_id = ctx.next_context_id.max(recipe.context_id_floor);
                for rule in recipe.rules.iter() {
                    ctx.next_rule_id = ctx.next_rule_id.max(rule.id.saturating_add(1));
                }
            }
            Content::Realized(e) => observe_occurrences(&e.original, ctx),
            // No node-local reservation; the mapper still visits descendants.
            Content::Source(_)
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
            | Content::OutlineEntry(_)
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
            | Content::Dynamic(_) => {}
        }
        Ok(None)
    });
}

#[expect(
    clippy::too_many_arguments,
    reason = "C020: Contrato público de realização reproduzido literalmente no L0: conteúdo, recursos, source, registry, snapshot anterior e configuração são explícitos"
)]
pub fn realize_document(
    content: &Content,
    world: &dyn World,
    source: &Source,
    _registry: &ElementRegistry,
    previous: &TagIntrospector,
    full_error: bool,
    target: EvalTarget,
    features: Features,
) -> (SourceResult<RealizedDocument>, Vec<SourceDiagnostic>) {
    let mut ctx = EvalContext::new();
    ctx.full_error = full_error;
    ctx.target = target;
    ctx.features = features;
    ctx.introspector = previous.clone();
    ctx.realizing_show_rules = true;
    observe_occurrences(content, &mut ctx);
    let route = Route::root().with_id(source.id());
    let mut styles = StyleChain::default_chain();
    let mut rules: Arc<[ShowRule]> = Arc::from([]);
    let mut guards = Vec::new();
    let metrics = FixedMetrics;
    let mut sink = Sink::new();
    let result = {
        let mut tracked_sink = sink.track_mut();
        let mut engine = Engine {
            world,
            font_metrics: &metrics,
            route: route.track(),
            styles: &mut styles,
            show_rules: &mut rules,
            active_guards: &mut guards,
            current_file: source.id(),
            sink: &mut tracked_sink,
        };
        recipes::realize(content, &mut ctx, &mut engine)
            .and_then(|visual| finish_occurrences(&visual, &mut ctx))
            .and_then(|visual| normalize_insertions(&visual))
            .map(|visual| {
                let visual = visual.wrap_page_columns();
                RealizedDocument {
                    introspector: introspect_with_introspector(&visual),
                    visual,
                }
            })
    };
    (result, sink.into_diagnostics())
}

/// Realize a contribution using its existing causal Engine and allocation context.
pub fn realize_math_insertions(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if ctx.defer_show_rules {
        return Ok(content);
    }
    observe_occurrences(&content, ctx);
    let captured = context_recipe(engine.styles).cloned();
    let previous_realizing = std::mem::replace(&mut ctx.realizing_show_rules, true);
    let previous_defer = std::mem::replace(&mut ctx.defer_show_rules, false);
    let previous_file = engine.current_file;
    let previous_rules = captured.as_ref().map(|recipe| {
        ctx.next_context_id = ctx.next_context_id.max(recipe.context_id_floor);
        engine.current_file = recipe.file;
        std::mem::replace(engine.show_rules, Arc::clone(&recipe.rules))
    });
    let previous_guards = captured.as_ref().map(|recipe| {
        let mut guards = engine.active_guards.clone();
        for id in &recipe.guards {
            if !guards.contains(id) {
                guards.push(*id);
            }
        }
        std::mem::replace(engine.active_guards, guards)
    });
    let result = recipes::realize(&content, ctx, engine);
    if let Some(rules) = previous_rules {
        *engine.show_rules = rules;
    }
    if let Some(guards) = previous_guards {
        *engine.active_guards = guards;
    }
    engine.current_file = previous_file;
    ctx.realizing_show_rules = previous_realizing;
    ctx.defer_show_rules = previous_defer;
    result
}

pub(crate) fn capture_context_recipe(
    content: Content,
    ctx: &EvalContext,
    engine: &Engine<'_>,
) -> Content {
    if engine.show_rules.is_empty() {
        return content;
    }
    let snapshot =
        Content::ShowRecipe(Arc::new(crate::entities::content::ShowRecipeContent {
            body: Content::Empty,
            textual: false,
            context_id_floor: ctx.next_context_id,
            rules: Arc::clone(&*engine.show_rules),
            styles: crate::compiler::eval::rules::textual::capture_styles(
                engine.styles,
                engine.show_rules,
                ctx,
                engine.current_file,
            ),
            guards: engine.active_guards.clone(),
            file: engine.current_file,
        }));
    Content::styled(
        content,
        crate::entities::style::Styles::new().push_custom(
            "eval.context.recipe",
            crate::entities::value::Value::Content(snapshot),
        ),
    )
}

fn context_recipe(
    styles: &StyleChain,
) -> Option<&Arc<crate::entities::content::ShowRecipeContent>> {
    match styles.custom("eval.context.recipe") {
        Some(crate::entities::value::Value::Content(Content::ShowRecipe(recipe))) => {
            Some(recipe)
        }
        _ => None,
    }
}

/// Seal the effective insertion rules without executing the delayed closure.
fn prepare_context_recipe(
    content: Content,
    ctx: &EvalContext,
    engine: &Engine<'_>,
) -> Content {
    use crate::compiler::eval::rules::textual;
    use crate::entities::content::ShowRecipeContent;
    use crate::entities::value::Value;

    let captured = context_recipe(engine.styles);
    let rules = captured.map_or_else(
        || Arc::clone(engine.show_rules),
        |recipe| textual::compose_rules(recipe, engine.show_rules),
    );
    if rules.is_empty() {
        return content;
    }
    let mut guards = engine.active_guards.clone();
    if let Some(recipe) = captured {
        for id in &recipe.guards {
            if !guards.contains(id) {
                guards.push(*id);
            }
        }
    }
    let snapshot = Content::ShowRecipe(Arc::new(ShowRecipeContent {
        body: Content::Empty,
        textual: false,
        context_id_floor: captured.map_or(ctx.next_context_id, |recipe| {
            ctx.next_context_id.max(recipe.context_id_floor)
        }),
        rules,
        styles: engine.styles.clone(),
        guards,
        file: captured.map_or(engine.current_file, |recipe| recipe.file),
    }));
    Content::styled(
        content,
        crate::entities::style::Styles::new()
            .push_custom("eval.context.recipe", Value::Content(snapshot)),
    )
}

fn anchor_recipe_errors(
    mut errors: Vec<SourceDiagnostic>,
    styles: &StyleChain,
) -> Vec<SourceDiagnostic> {
    if let Some(crate::entities::value::Value::Int(raw)) =
        styles.custom("eval.math.origin")
    {
        if let Some(bits) = std::num::NonZeroU64::new(*raw as u64) {
            let span = crate::entities::span::Span::from_raw(bits);
            for error in &mut errors {
                if error.span.is_detached() {
                    error.span = span;
                }
            }
        }
    }
    errors
}

/// Resolve logical table insertions using their causal evaluation context.
pub fn realize_table_insertions(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    table_cells::realize(content, ctx, engine)
}

pub(crate) fn table_cells_needed(content: &Content) -> bool {
    table_cells::needs_preparation(content)
}

pub(crate) fn capture_table_insertion(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules || !table_cells::needs_preparation(&content) {
        return Ok(content);
    }
    if ctx.defer_show_rules {
        ctx.has_deferred_show_rules = true;
        Ok(Content::ShowRecipe(Arc::new(crate::entities::content::ShowRecipeContent {
            body: content,
            textual: false,
            context_id_floor: ctx.next_context_id,
            rules: Arc::clone(&*engine.show_rules),
            styles: engine.styles.clone(),
            guards: engine.active_guards.clone(),
            file: engine.current_file,
        })))
    } else {
        realize_table_insertions(content, ctx, engine)
    }
}

#[cfg(test)]
mod finish_occurrences_tests {
    use super::*;
    use crate::entities::content::SourceContent;
    use crate::entities::file_id::FileId;
    use crate::entities::location::Location;
    use crate::entities::span::Span;
    use std::num::NonZeroU16;

    #[test]
    fn prepared_table_is_reused_and_unprepared_table_is_located() {
        let table = Content::table(
            vec![],
            vec![],
            vec![Content::table_cell(Content::text("cell"), None, None, None, None)],
        );
        let prepared = Content::Located(Arc::new(LocatedContent {
            location: Location::from_raw(47),
            body: table.clone(),
        }));
        let mut ctx = EvalContext::new();
        let result = finish_occurrences(&prepared, &mut ctx).unwrap();
        if let (Content::Located(before), Content::Located(after)) = (&prepared, &result)
        {
            assert!(Arc::ptr_eq(before, after));
        } else {
            panic!("expected prepared table");
        }

        let mut ctx = EvalContext::new();
        assert!(matches!(
            finish_occurrences(&table, &mut ctx).unwrap(),
            Content::Located(_)
        ));
    }

    #[test]
    fn nested_source_keeps_the_inner_origin() {
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let inner_span = Span::from_range(file, 1..2);
        let outer_span = Span::from_range(file, 3..4);
        let inner = Content::Source(Arc::new(SourceContent {
            body: Content::text("body"),
            span: inner_span,
        }));
        let outer =
            Content::Source(Arc::new(SourceContent { body: inner, span: outer_span }));
        let mut ctx = EvalContext::new();
        let result = finish_occurrences(&outer, &mut ctx).unwrap();
        assert_eq!(result.source_span(), inner_span);
    }
}

#[cfg(test)]
mod insertion_reuse_tests {
    use super::*;
    use crate::entities::style::Styles;

    #[test]
    fn normalization_reuses_canonical_sequences() {
        for middle in [
            Content::text("middle"),
            Content::Space,
            Content::SpaceFromNewline,
            Content::styled(Box::new(Content::Space), Styles::default()),
        ] {
            let input = Content::sequence(vec![
                Content::text("left"),
                middle,
                Content::text("right"),
            ]);
            let output = normalize_insertions(&input).unwrap();
            match (&input, &output) {
                (Content::Sequence(a), Content::Sequence(b)) => {
                    assert!(Arc::ptr_eq(a, b))
                }
                _ => panic!("expected sequence"),
            }
        }
    }

    #[test]
    fn normalization_keeps_flattening_and_degenerate_canonicalization() {
        assert!(matches!(
            normalize_insertions(&Content::Sequence(vec![].into())).unwrap(),
            Content::Empty
        ));
        assert_eq!(
            normalize_insertions(&Content::Sequence(vec![Content::text("one")].into()))
                .unwrap()
                .plain_text(),
            "one"
        );
        let original = Content::text("origin");
        let realized =
            Content::Realized(Arc::new(crate::entities::content::RealizedContent {
                original: original.clone(),
                visual: Content::sequence(vec![Content::text("a"), Content::text("b")]),
            }));
        let input = Content::sequence(vec![realized, Content::text("c")]);
        let output = normalize_insertions(&input).unwrap();
        assert_eq!(output.plain_text(), "abc");
        let Content::Sequence(parts) = output else { panic!("sequence") };
        assert_eq!(parts.len(), 4);
        assert!(
            matches!(&parts[0], Content::Realized(e) if e.original == original && matches!(e.visual, Content::Empty))
        );
        assert!(parts[1..].iter().all(|p| matches!(p, Content::Text(_))));
    }
}

#[cfg(test)]
mod batch_g_tests {
    use super::*;
    use crate::entities::content::{RealizedContent, ShowRecipeContent};
    use crate::entities::elements::context_block::ContextBlockElem;
    use crate::entities::file_id::FileId;
    use crate::entities::func::Func;
    use crate::entities::location::Location;
    use crate::entities::show::{NodeKind, Selector, Transformation};
    use crate::entities::value::Value;
    use std::num::NonZeroU16;

    fn recipe(body: Content, product: bool) -> Content {
        let styles = if product {
            StyleChain::default_chain()
                .push_custom("eval.textual.product", Value::Bool(true))
        } else {
            StyleChain::default_chain()
        };
        Content::ShowRecipe(Arc::new(ShowRecipeContent {
            body,
            textual: true,
            context_id_floor: 0,
            rules: Arc::from([]),
            styles,
            guards: vec![],
            file: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        }))
    }

    #[test]
    fn batch_g_normalization_retains_protected_and_nonsequence_products() {
        for protected in [false, true] {
            for visual in [
                Content::text("v"),
                Content::sequence(vec![Content::text("a"), Content::text("b")]),
            ] {
                let original = recipe(Content::text("origin"), protected);
                let input = Content::sequence(vec![
                    Content::Realized(Arc::new(RealizedContent {
                        original: original.clone(),
                        visual: visual.clone(),
                    })),
                    Content::text("tail"),
                ]);
                let output = normalize_insertions(&input).unwrap();
                let Content::Sequence(parts) = output else { panic!("sequence lost") };
                let should_flatten = !protected && matches!(visual, Content::Sequence(_));
                assert_eq!(parts.len(), if should_flatten { 4 } else { 2 });
                let Content::Realized(first) = &parts[0] else { panic!("origin lost") };
                assert_eq!(first.original, original);
                if should_flatten {
                    assert!(matches!(first.visual, Content::Empty));
                    assert_eq!(parts[1].plain_text(), "a");
                    assert_eq!(parts[2].plain_text(), "b");
                } else {
                    assert_eq!(first.visual, visual);
                }
                assert_eq!(parts.last().unwrap().plain_text(), "tail");
            }
        }
    }

    #[test]
    fn batch_g_observation_reserves_located_context_and_both_realized_sides() {
        let context = |id| {
            Content::ContextBlock(Arc::new(ContextBlockElem {
                id,
                closure: Func::native("must-not-run", |_, _, _, _| {
                    panic!("ID observation executed a closure")
                }),
            }))
        };
        let located = |location, body| {
            Content::Located(Arc::new(LocatedContent {
                location: Location::from_raw(location),
                body,
            }))
        };
        let input = Content::title(Content::Realized(Arc::new(RealizedContent {
            original: located(47, context(71)),
            visual: located(53, context(83)),
        })));
        let mut ctx = EvalContext::new();
        observe_occurrences(&input, &mut ctx);
        assert_eq!(ctx.next_context_id, 84);
        assert_eq!(ctx.realization_locator.next(), Location::from_raw(54));
    }

    #[test]
    fn batch_g_observation_reserves_captured_rule_floor_and_sequence_marker() {
        let Content::ShowRecipe(base) = recipe(Content::text("body"), false) else {
            unreachable!()
        };
        let mut snapshot = base.as_ref().clone();
        snapshot.context_id_floor = 101;
        snapshot.rules = Arc::from([ShowRule {
            id: 123,
            selector: Selector::NodeKind(NodeKind::Heading),
            transform: Transformation::Content(Content::text("unused")),
        }]);
        snapshot.styles = snapshot
            .styles
            .push_custom("eval.sequence.occurrence", Value::Bool(true));
        let tree = Content::title(Content::ShowRecipe(Arc::new(snapshot)));
        let mut ctx = EvalContext::new();
        observe_occurrences(&tree, &mut ctx);
        assert_eq!(ctx.next_context_id, 101);
        assert_eq!(ctx.next_rule_id, 124);
        assert!(ctx.has_deferred_show_rules);
        assert_eq!(ctx.realization_locator.next(), Location::from_raw(0));
    }
}
