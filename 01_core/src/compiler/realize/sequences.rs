//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/realize/sequences.md
//! @prompt-hash 4f0108eb
//! @layer L1
//! @updated 2026-10-17

use super::*;
use crate::compiler::eval::selector_matching::{is_sequence_rule, selector_matches};
use crate::entities::content::ShowRecipeContent;
use crate::entities::show::{RuleId, Transformation};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::value::Value;

const MARKER: &str = "eval.sequence.occurrence";

#[derive(Clone)]
pub(crate) struct Snapshot {
    styles: StyleChain,
    rules: Arc<[ShowRule]>,
    guards: Vec<RuleId>,
    file: crate::entities::file_id::FileId,
}

pub(crate) fn snapshot(engine: &Engine<'_>) -> Option<Snapshot> {
    if !engine.show_rules.iter().any(|rule| is_sequence_rule(&rule.selector)) {
        return None;
    }
    Some(Snapshot {
        styles: engine.styles.clone(),
        rules: engine.show_rules.clone(),
        guards: engine
            .active_guards
            .iter()
            .copied()
            .filter(|id| {
                !engine
                    .show_rules
                    .iter()
                    .any(|rule| rule.id == *id && is_sequence_rule(&rule.selector))
            })
            .collect(),
        file: engine.current_file,
    })
}

pub(super) fn marked(recipe: &ShowRecipeContent) -> bool {
    matches!(recipe.styles.custom(MARKER), Some(Value::Bool(true)))
}

/// Sequence guards and its private marker do not create a new text scope.
pub(crate) fn same_textual_transport(
    left: &ShowRecipeContent,
    right: &ShowRecipeContent,
) -> bool {
    let normalize = |scope: &ShowRecipeContent| {
        let mut styles = scope.styles.collapse();
        styles.custom.retain(|(key, _)| key.as_str() != MARKER);
        let guards: Vec<RuleId> = scope
            .guards
            .iter()
            .copied()
            .filter(|id| {
                !scope
                    .rules
                    .iter()
                    .any(|rule| rule.id == *id && is_sequence_rule(&rule.selector))
            })
            .collect();
        (styles, guards)
    };
    normalize(left) == normalize(right)
}

fn recipe(body: Content, scope: Snapshot, floor: u64) -> Content {
    Content::ShowRecipe(Arc::new(ShowRecipeContent {
        body,
        textual: false,
        context_id_floor: floor,
        styles: scope.styles.push_custom(MARKER, Value::Bool(true)),
        rules: scope.rules,
        guards: scope.guards,
        file: scope.file,
    }))
}

pub(crate) fn capture_scope(
    body: Content,
    scope: Snapshot,
    origin: Span,
    ctx: &mut EvalContext,
) -> Content {
    if ctx.apply_show_rules
        && matches!(body.source_body(), Content::Empty | Content::Sequence(_))
    {
        ctx.has_deferred_show_rules = true;
        recipe(body.with_source_span(origin), scope, ctx.next_context_id)
    } else {
        body
    }
}

pub(crate) fn capture(
    body: Content,
    ctx: &mut EvalContext,
    engine: &Engine<'_>,
) -> Content {
    capture_at(body, Span::detached(), ctx, engine)
}

/// Add origin only to an actual Sequence capture; other values retain their shape.
pub(crate) fn capture_at(
    body: Content,
    origin: Span,
    ctx: &mut EvalContext,
    engine: &Engine<'_>,
) -> Content {
    if !ctx.apply_show_rules {
        return body;
    }
    match snapshot(engine) {
        Some(scope) => capture_scope(body, scope, origin, ctx),
        None => body,
    }
}

/// Marked outer recipes are decided before descending into any hidden children.
pub(crate) fn realize(
    content: &Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.has_deferred_show_rules && snapshot(engine).is_none() {
        return Ok(content.clone());
    }
    visit(content, false, ctx, engine)
}

fn visit(
    content: &Content,
    skip_root: bool,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let mut first = true;
    content.map_content_pre(&mut |node| {
        let at_root = std::mem::replace(&mut first, false);
        if skip_root && at_root && matches!(node, Content::Empty | Content::Sequence(_)) {
            return Ok(None);
        }
        let captured;
        let pending = if let Content::ShowRecipe(pending) = node {
            pending
        } else if matches!(node, Content::Empty | Content::Sequence(_)) {
            captured = capture(node.clone(), ctx, engine);
            let Content::ShowRecipe(pending) = &captured else { return Ok(None) };
            pending
        } else {
            return Ok(None);
        };
        let previous_styles = std::mem::replace(
            engine.styles,
            pending.styles.push_custom(MARKER, Value::Bool(false)),
        );
        let previous_rules = std::mem::replace(engine.show_rules, pending.rules.clone());
        let previous_guards = std::mem::replace(
            engine.active_guards,
            pending
                .guards
                .iter()
                .copied()
                .filter(|id| {
                    !pending
                        .rules
                        .iter()
                        .any(|rule| rule.id == *id && is_sequence_rule(&rule.selector))
                })
                .collect(),
        );
        let previous_file = std::mem::replace(&mut engine.current_file, pending.file);
        let previous_realizing = std::mem::replace(&mut ctx.realizing_show_rules, true);
        let previous_defer = std::mem::replace(&mut ctx.defer_show_rules, false);
        let result = if marked(pending) {
            realize_root(&pending.body, &pending.guards, ctx, engine)
        } else {
            visit(&pending.body, pending.textual, ctx, engine).and_then(|body| {
                let body = if pending.textual {
                    crate::compiler::eval::rules::textual::coalesce_pending_text(
                        &body, pending,
                    )?
                } else {
                    body
                };
                let mut recipe = pending.as_ref().clone();
                recipe.body = body;
                Ok(Content::ShowRecipe(Arc::new(recipe)))
            })
        };
        *engine.styles = previous_styles;
        *engine.show_rules = previous_rules;
        *engine.active_guards = previous_guards;
        engine.current_file = previous_file;
        ctx.realizing_show_rules = previous_realizing;
        ctx.defer_show_rules = previous_defer;
        result.map(Some)
    })
}

fn realize_root(
    body: &Content,
    guards: &[RuleId],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let rules = engine.show_rules.clone();
    let mut consumed = guards.to_vec();
    let mut styles = StyleChain::empty();
    let mut has_styles = false;
    for rule in rules.iter() {
        if consumed.contains(&rule.id)
            || !is_sequence_rule(&rule.selector)
            || !selector_matches(body, &rule.selector)
        {
            continue;
        }
        if let Transformation::Style(delta) = &rule.transform {
            styles = styles.push_styles(delta);
            consumed.push(rule.id);
            has_styles = true;
        }
    }
    let style_delta = Styles::from_delta(styles.collapse());
    let previous_styles = engine.styles.clone();
    if has_styles {
        *engine.styles = engine.styles.push_styles(&style_delta);
    }
    let selected = rules.iter().rev().find(|rule| {
        !consumed.contains(&rule.id)
            && is_sequence_rule(&rule.selector)
            && selector_matches(body, &rule.selector)
            && !matches!(rule.transform, Transformation::Style(_))
    });
    let result = (|| {
        let Some(rule) = selected else {
            return visit(body, true, ctx, engine);
        };
        if ctx.occurrence_show_depth >= Route::MAX_SHOW_RULE_DEPTH {
            let mut error = SourceDiagnostic::error(
                body.source_span(),
                "maximum show rule depth exceeded",
            )
            .with_hint("maybe a show rule matches its own output")
            .with_hint("maybe there are too deeply nested elements");
            if ctx.full_error {
                error = error.with_hint(if ctx.occurrence_show_path.contains(&body.morph_canon()) {
                    "erro completo: recursão CÍCLICA — uma forma de conteúdo repetiu-se no caminho de revisitação (a regra de #show reescreve para algo que reaparece)"
                } else {
                    "erro completo: recursão NÃO-CONVERGENTE — passou do limite sem repetir nem estabilizar (verifique se a regra termina, ou se é recursão legítima profunda)"
                });
            }
            return Err(vec![error]);
        }
        consumed.push(rule.id);
        let protected = recipe(
            body.clone(),
            Snapshot {
                styles: engine.styles.clone(),
                rules: rules.clone(),
                guards: consumed,
                file: engine.current_file,
            },
            ctx.next_context_id,
        );
        ctx.occurrence_show_depth += 1;
        if ctx.full_error {
            ctx.occurrence_show_path.push(body.morph_canon());
        }
        let product = crate::compiler::eval::rules::apply::apply_sequence_rule_once(
            protected, rule, ctx, engine,
        )
        .and_then(|product| {
            let product = capture(product, ctx, engine);
            realize(&product, ctx, engine)
        });
        if ctx.full_error {
            ctx.occurrence_show_path.pop();
        }
        ctx.occurrence_show_depth -= 1;
        product
    })();
    *engine.styles = previous_styles;
    result.map(
        |visual| if has_styles { Content::styled(visual, style_delta) } else { visual },
    )
}
