//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual.md
//! @prompt-hash 58d62887
//! @layer L1
//! @updated 2026-10-05
//!
//! Coordenação da realização textual de regras show.

use std::sync::Arc;

use crate::entities::content::{Content, ShowRecipeContent};
use crate::entities::engine::Engine;
use crate::entities::show::ShowRule;
use crate::entities::source_result::SourceResult;
use crate::entities::style::Styles;
use crate::entities::style_chain::StyleChain;

use super::super::{selector_matching, EvalContext};
use super::apply_show_rules;

mod matching;
pub(crate) mod space;

/// Snapshot causal de show e style no instante em que uma ocorrência de
/// markup é avaliada. Não é exposto como valor da linguagem.
#[derive(Clone)]
pub(crate) struct TextualSnapshot {
    rules: Arc<[ShowRule]>,
    styles: StyleChain,
    guards: Vec<crate::entities::show::RuleId>,
}

impl TextualSnapshot {
    pub(crate) fn capture(engine: &Engine<'_>) -> Self {
        Self {
            rules: Arc::clone(&*engine.show_rules),
            styles: (*engine.styles).clone(),
            guards: (*engine.active_guards).clone(),
        }
    }

    fn same_group(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.rules, &other.rules)
            && self.styles.collapse() == other.styles.collapse()
            && self.guards == other.guards
    }
}

/// Intercepta os selectors de elemento de texto no momento da construção,
/// sem antecipar os selectors literais/regex que precisam do grupo realizado.
pub(crate) fn intercept_text_element(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !ctx.apply_show_rules {
        return Ok(content);
    }
    let rules: Vec<ShowRule> = engine
        .show_rules
        .iter()
        .filter(|rule| {
            selector_matching::is_node_rule(&rule.selector)
                && !selector_matching::is_par_rule(&rule.selector)
        })
        .cloned()
        .collect();
    apply_show_rules(content, &rules, ctx, engine)
}

/// No fluxo realizado, normaliza espaços antes dos selectors textuais. Um
/// ContentBlock ainda bruto preserva-os até ser interpolado; seu show local
/// atua provisoriamente no texto pré-normalizado. Slots consumidos tornam-se
/// Empty para que os índices das fronteiras lexicais de #set permaneçam válidos.
pub(crate) fn realize_textual_parts(
    parts: &mut [Content],
    snapshots: &[TextualSnapshot],
    style_boundaries: &[(usize, Styles)],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
    normalize_spaces: bool,
) -> SourceResult<Vec<bool>> {
    debug_assert_eq!(parts.len(), snapshots.len());
    let mut consumed = vec![false; parts.len()];
    if normalize_spaces {
        space::normalize_markup_spaces(parts);
    }
    if !ctx.apply_show_rules {
        return Ok(consumed);
    }

    if ctx.defer_show_rules {
        let mut start = 0;
        while start < parts.len() {
            let snapshot = &snapshots[start];
            let mut end = start + 1;
            while end < parts.len()
                && snapshot.same_group(&snapshots[end])
                && !style_boundaries.iter().any(|(index, _)| *index == end)
            {
                end += 1;
            }
            if snapshot
                .rules
                .iter()
                .any(|rule| selector_matching::is_text_rule(&rule.selector))
            {
                let body = Content::sequence(
                    parts[start..end]
                        .iter()
                        .filter(|part| !matches!(part, Content::Empty))
                        .cloned()
                        .collect(),
                );
                let mut recipe = ShowRecipeContent {
                    body,
                    textual: true,
                    context_id_floor: ctx.next_context_id,
                    rules: snapshot.rules.clone(),
                    styles: snapshot.styles.clone(),
                    guards: snapshot.guards.clone(),
                    file: engine.current_file,
                };
                recipe.body = coalesce_pending_text(&recipe.body, &recipe)?;
                parts[start] = Content::ShowRecipe(Arc::new(recipe));
                consumed[start + 1..end].fill(true);
                for part in &mut parts[start + 1..end] {
                    *part = Content::Empty;
                }
                ctx.has_deferred_show_rules = true;
            }
            start = end;
        }
        Ok(consumed)
    } else {
        matching::apply_textual_groups(parts, snapshots, style_boundaries, ctx, engine)
    }
}

/// Remove only redundant pending text packets from this same causal group.
/// Recipes with applicable element effects remain opaque until their owner runs.
pub(crate) fn coalesce_pending_text(
    body: &Content,
    scope: &ShowRecipeContent,
) -> SourceResult<Content> {
    body.map_content_pre(&mut |node| {
        let Content::ShowRecipe(nested) = node else { return Ok(None) };
        let same_environment = nested.file == scope.file
            && crate::compiler::realize::sequences::same_textual_transport(nested, scope);
        if nested.textual && Arc::ptr_eq(&nested.rules, &scope.rules) && same_environment
        {
            coalesce_pending_text(&nested.body, scope).map(Some)
        } else if !nested.textual
            && same_environment
            && nested
                .rules
                .iter()
                .map(|rule| rule.id)
                .eq(scope.rules.iter().map(|rule| rule.id))
            && matches!(nested.body.source_body(), Content::Text(_))
            && !nested.rules.iter().any(|rule| {
                selector_matching::is_node_rule(&rule.selector)
                    && selector_matching::selector_matches(&nested.body, &rule.selector)
            })
        {
            // Generic capture copies the slice; RuleIds retain its causal identity.
            Ok(Some(nested.body.clone()))
        } else {
            Ok(Some(node.clone()))
        }
    })
}

pub(crate) fn realize_captured_text(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let original = content.clone();
    let mut parts = match content {
        Content::Sequence(parts) => parts.to_vec(),
        other => vec![other],
    };
    let snapshots = vec![TextualSnapshot::capture(engine); parts.len()];
    let consumed =
        matching::apply_textual_groups(&mut parts, &snapshots, &[], ctx, engine)?;
    Ok(Content::Realized(Arc::new(crate::entities::content::RealizedContent {
        original,
        visual: Content::sequence(
            parts
                .into_iter()
                .zip(consumed)
                .filter_map(|(part, consumed)| (!consumed).then_some(part))
                .collect(),
        ),
    })))
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::TextualSnapshot;
    use crate::entities::show::ShowRule;
    use crate::entities::style_chain::{StyleChain, StyleDelta};

    #[test]
    fn snapshot_group_requires_rule_identity_style_and_guards() {
        let rules: Arc<[ShowRule]> = Arc::from(Vec::<ShowRule>::new());
        let baseline = TextualSnapshot {
            rules: Arc::clone(&rules),
            styles: StyleChain::empty(),
            guards: vec![],
        };
        let same = baseline.clone();
        assert!(baseline.same_group(&same));

        let distinct_rules = TextualSnapshot {
            rules: Arc::from(Vec::<ShowRule>::new()),
            ..baseline.clone()
        };
        assert!(!baseline.same_group(&distinct_rules));

        let changed_style = TextualSnapshot {
            styles: baseline
                .styles
                .push(StyleDelta { bold: Some(true), ..StyleDelta::empty() }),
            ..baseline.clone()
        };
        assert!(!baseline.same_group(&changed_style));

        let changed_guards = TextualSnapshot { guards: vec![42], ..baseline.clone() };
        assert!(!baseline.same_group(&changed_guards));
    }
    #[test]
    fn coalescing_preserves_real_styles_other_guards_and_element_callbacks() {
        use crate::compiler::eval::bindings::native_sequence;
        use crate::entities::content::{Content, ShowRecipeContent};
        use crate::entities::file_id::FileId;
        use crate::entities::func::Func;
        use crate::entities::show::{NodeKind, Selector, Transformation};
        use crate::entities::value::Value;
        use std::num::NonZeroU16;
        let scope = ShowRecipeContent {
            body: Content::text("A"),
            textual: true,
            context_id_floor: 0,
            rules: vec![
                ShowRule {
                    id: 1,
                    selector: Selector::NativeElement(Func::native(
                        "sequence",
                        native_sequence,
                    )),
                    transform: Transformation::Content(Content::text("S")),
                },
                ShowRule {
                    id: 2,
                    selector: Selector::NodeKind(NodeKind::Strong),
                    transform: Transformation::Content(Content::text("B")),
                },
                ShowRule {
                    id: 3,
                    selector: Selector::Text("A".into()),
                    transform: Transformation::Content(Content::text("AA")),
                },
            ]
            .into(),
            styles: StyleChain::empty(),
            guards: vec![],
            file: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        };
        let wrap = |recipe| Content::ShowRecipe(Arc::new(recipe));
        let emitted = ShowRecipeContent {
            guards: vec![1],
            styles: scope
                .styles
                .push_custom("eval.sequence.occurrence", Value::Bool(false)),
            ..scope.clone()
        };
        assert!(matches!(
            super::coalesce_pending_text(&wrap(emitted), &scope).unwrap(),
            Content::Text(_)
        ));
        for blocked in [
            ShowRecipeContent { guards: vec![1, 2], ..scope.clone() },
            ShowRecipeContent { guards: vec![3], ..scope.clone() },
            ShowRecipeContent { guards: vec![42], ..scope.clone() },
            ShowRecipeContent {
                styles: scope
                    .styles
                    .push(StyleDelta { bold: Some(true), ..StyleDelta::empty() }),
                ..scope.clone()
            },
            ShowRecipeContent {
                rules: Arc::from(scope.rules.to_vec()),
                ..scope.clone()
            },
            ShowRecipeContent {
                file: FileId::from_raw(NonZeroU16::new(2).unwrap()),
                ..scope.clone()
            },
        ] {
            assert!(matches!(
                super::coalesce_pending_text(&wrap(blocked), &scope).unwrap(),
                Content::ShowRecipe(_)
            ));
        }
        let generic = ShowRecipeContent {
            textual: false,
            rules: Arc::from(scope.rules.to_vec()),
            ..scope.clone()
        };
        assert!(matches!(
            super::coalesce_pending_text(&wrap(generic), &scope).unwrap(),
            Content::Text(_)
        ));
        let mut text_rules = scope.rules.to_vec();
        text_rules.push(ShowRule {
            id: 4,
            selector: Selector::NativeElement(Func::native(
                "text",
                crate::compiler::stdlib::native_text,
            )),
            transform: Transformation::Content(Content::text("CALLBACK")),
        });
        let active = ShowRecipeContent { rules: text_rules.into(), ..scope.clone() };
        let generic = ShowRecipeContent { textual: false, ..active.clone() };
        assert!(matches!(
            super::coalesce_pending_text(&wrap(generic), &active).unwrap(),
            Content::ShowRecipe(_)
        ));
    }
}
