//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual.md
//! @prompt-hash 42e4d7c5
//! @layer L1
//! @updated 2026-09-30
//!
//! Coordenação da realização textual de regras show.

use std::sync::Arc;

use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::show::ShowRule;
use crate::entities::source_result::SourceResult;
use crate::entities::style::Styles;
use crate::entities::style_chain::StyleChain;

use super::super::{selector_matching, EvalContext};
use super::apply_show_rules;

mod matching;
mod space;

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
) -> SourceResult<()> {
    debug_assert_eq!(parts.len(), snapshots.len());
    if normalize_spaces {
        space::normalize_markup_spaces(parts);
    }
    if !ctx.apply_show_rules {
        return Ok(());
    }

    matching::apply_textual_groups(parts, snapshots, style_boundaries, ctx, engine)
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
}
