//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/realize/mod.md
//! @prompt-hash 10bfeeea
//! @layer L1
//! @updated 2026-10-10

pub(in crate::compiler) mod bare_math;
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
        location: ctx.realization_locator.next(),
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
            Content::Realized(e) if matches!(e.visual, Content::Sequence(_)) => {
                parts.push(Content::Realized(Arc::new(
                    crate::entities::content::RealizedContent {
                        original: e.original.clone(),
                        visual: Content::Empty,
                    },
                )));
                flatten(&e.visual, parts);
            }
            other => parts.push(other.clone()),
        }
    }
    content.map_content(&mut |node| {
        if let Content::Sequence(children) = node {
            let needs_flattening = children.iter().any(|child| match child {
                Content::Sequence(_) => true,
                Content::Realized(e) => matches!(e.visual, Content::Sequence(_)),
                _ => false,
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
            _ => {}
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
        sequences::realize(content, &mut ctx, &mut engine)
            .and_then(|content| realize_table_insertions(content, &mut ctx, &mut engine))
            .and_then(|content| bare_math::prepare(&content, engine.styles))
            .and_then(|content| {
                content.map_content(&mut |node| {
                    if let Content::ShowRecipe(recipe) = node {
                        let old_styles =
                            std::mem::replace(engine.styles, recipe.styles.clone());
                        let old_rules =
                            std::mem::replace(engine.show_rules, recipe.rules.clone());
                        let old_guards = std::mem::replace(
                            engine.active_guards,
                            recipe.guards.clone(),
                        );
                        let old_file =
                            std::mem::replace(&mut engine.current_file, recipe.file);
                        let result = if recipe.textual {
                            crate::compiler::eval::rules::textual::realize_captured_text(
                                recipe.body.clone(),
                                &mut ctx,
                                &mut engine,
                            )
                        } else {
                            crate::compiler::eval::rules::apply_show_rules(
                                recipe.body.clone(),
                                &recipe.rules,
                                &mut ctx,
                                &mut engine,
                            )
                        };
                        *engine.styles = old_styles;
                        *engine.show_rules = old_rules;
                        *engine.active_guards = old_guards;
                        engine.current_file = old_file;
                        result
                            .map_err(|errors| {
                                anchor_recipe_errors(errors, &recipe.styles)
                            })
                            .map(Some)
                    } else if !matches!(node, Content::Located(_) | Content::Realized(_))
                        && extract_payload(node).is_some()
                    {
                        Ok(Some(locate(node.clone(), &mut ctx)))
                    } else {
                        Ok(None)
                    }
                })
            })
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
    let content = sequences::realize(&content, ctx, engine)?;
    if !bare_math::needs_preparation(&content) {
        return Ok(content);
    }
    observe_occurrences(&content, ctx);
    let content = bare_math::prepare(&content, engine.styles)?;
    let previous_realizing = ctx.realizing_show_rules;
    let previous_defer = ctx.defer_show_rules;
    ctx.realizing_show_rules = true;
    ctx.defer_show_rules = false;
    let captured = match engine.styles.custom("eval.context.recipe") {
        Some(crate::entities::value::Value::Content(Content::ShowRecipe(recipe))) => {
            Some(recipe.clone())
        }
        _ => None,
    };
    let previous_guards = captured
        .as_ref()
        .map(|recipe| std::mem::replace(engine.active_guards, recipe.guards.clone()));
    let previous_file = engine.current_file;
    let previous_rules = captured
        .as_ref()
        .map(|recipe| std::mem::replace(engine.show_rules, Arc::clone(&recipe.rules)));
    let rules = if let Some(recipe) = &captured {
        ctx.next_context_id = ctx.next_context_id.max(recipe.context_id_floor);
        engine.current_file = recipe.file;
        Arc::clone(&recipe.rules)
    } else {
        Arc::clone(&*engine.show_rules)
    };
    let result =
        crate::compiler::eval::rules::apply_show_rules(content, &rules, ctx, engine)
            .and_then(|content| finish_occurrences(&content, ctx));
    engine.current_file = previous_file;
    if let Some(rules) = previous_rules {
        *engine.show_rules = rules;
    }
    if let Some(guards) = previous_guards {
        *engine.active_guards = guards;
    }
    ctx.realizing_show_rules = previous_realizing;
    ctx.defer_show_rules = previous_defer;
    result
}

pub(crate) fn capture_context_recipe(
    content: Content,
    ctx: &EvalContext,
    engine: &Engine<'_>,
) -> Content {
    if !engine.show_rules.iter().any(|rule| {
        bare_math::captures_equation_rule(&rule.selector)
            || table_cells::captures_rule(&rule.selector)
    }) {
        return content;
    }
    let snapshot =
        Content::ShowRecipe(Arc::new(crate::entities::content::ShowRecipeContent {
            body: Content::Empty,
            textual: false,
            context_id_floor: ctx.next_context_id,
            rules: Arc::clone(&*engine.show_rules),
            styles: engine.styles.clone(),
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
