//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/realize/sequences.md
//! @prompt-hash b3bc750b
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

pub(in crate::compiler) fn marked(recipe: &ShowRecipeContent) -> bool {
    matches!(recipe.styles.custom(MARKER), Some(Value::Bool(true)))
}

/// Sequence guards and its private marker do not create a new text scope.
pub(crate) fn same_textual_transport(
    left: &ShowRecipeContent,
    right: &ShowRecipeContent,
) -> bool {
    let normalize = |scope: &ShowRecipeContent| {
        let mut styles =
            crate::compiler::eval::rules::textual::captured_styles(scope).collapse();
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

fn recipe(body: Content, scope: Snapshot, ctx: &EvalContext) -> Content {
    Content::ShowRecipe(Arc::new(ShowRecipeContent {
        body,
        textual: false,
        context_id_floor: ctx.next_context_id,
        styles: crate::compiler::eval::rules::textual::capture_styles(
            &scope.styles,
            &scope.rules,
            ctx,
            scope.file,
        )
        .push_custom(MARKER, Value::Bool(true)),
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
        recipe(body.with_source_span(origin), scope, ctx)
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

pub(super) fn realize_root(
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
            return super::recipes::realize_children(body, ctx, engine);
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
            ctx,
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
            super::recipes::realize(&product, ctx, engine)
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
