//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/textual.md
//! @prompt-hash 5f5d4ef8
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

const BASELINE: &str = "eval.textual.baseline";
const PRODUCT: &str = "eval.textual.product";

#[derive(Clone)]
pub(crate) struct LexicalFrame {
    rules: Arc<[ShowRule]>,
}

pub(crate) fn enter_scope(ctx: &mut EvalContext, engine: &Engine<'_>) -> usize {
    let depth = ctx.textual_frames.len();
    ctx.textual_frames
        .push(LexicalFrame { rules: Arc::clone(engine.show_rules) });
    depth
}

pub(crate) fn leave_scope(ctx: &mut EvalContext, depth: usize) {
    ctx.textual_frames.truncate(depth);
}

/// The metadata is data only: its empty body is never executed. Its styles
/// retain the original persistent chain, so removing transport does not
/// collapse relative lengths or duplicate style levels.
pub(crate) fn capture_styles(
    styles: &StyleChain,
    rules: &Arc<[ShowRule]>,
    ctx: &EvalContext,
    file: crate::entities::file_id::FileId,
) -> StyleChain {
    let base = ctx.textual_frames.last().map_or(rules, |frame| &frame.rules);
    let metadata = ShowRecipeContent {
        body: Content::Empty,
        textual: false,
        context_id_floor: 0,
        rules: Arc::clone(base),
        styles: styles.clone(),
        guards: vec![],
        file,
    };
    styles.push_custom(
        BASELINE,
        crate::entities::value::Value::Content(Content::ShowRecipe(Arc::new(metadata))),
    )
}

fn baseline(styles: &StyleChain) -> Option<&ShowRecipeContent> {
    match styles.custom(BASELINE) {
        Some(crate::entities::value::Value::Content(Content::ShowRecipe(data))) => {
            Some(data)
        }
        _ => None,
    }
}

pub(crate) fn captured_styles(recipe: &ShowRecipeContent) -> StyleChain {
    baseline(&recipe.styles)
        .map_or_else(|| recipe.styles.clone(), |data| data.styles.clone())
}

pub(super) fn snapshot_styles(styles: &StyleChain) -> StyleChain {
    baseline(styles).map_or_else(|| styles.clone(), |data| data.styles.clone())
}

pub(crate) fn compose_rules(
    recipe: &ShowRecipeContent,
    inherited: &Arc<[ShowRule]>,
) -> Arc<[ShowRule]> {
    let Some(base) = baseline(&recipe.styles) else { return Arc::clone(&recipe.rules) };
    let owned = recipe
        .rules
        .iter()
        .filter(|rule| !base.rules.iter().any(|old| old.id == rule.id));
    let mut combined = None;
    for rule in owned {
        if inherited.iter().any(|old| old.id == rule.id) {
            continue;
        }
        combined.get_or_insert_with(|| inherited.to_vec()).push(rule.clone());
    }
    combined.map_or_else(|| Arc::clone(inherited), Arc::from)
}

pub(crate) fn is_product(content: &Content) -> bool {
    let Content::Realized(done) = content.source_body() else { return false };
    let Content::ShowRecipe(origin) = done.original.source_body() else { return false };
    matches!(
        origin.styles.custom(PRODUCT),
        Some(crate::entities::value::Value::Bool(true))
    )
}

pub(crate) fn complete_product(
    original: Content,
    visual: Content,
    ctx: &EvalContext,
    engine: &Engine<'_>,
) -> Content {
    let origin = ShowRecipeContent {
        body: original,
        textual: true,
        context_id_floor: ctx.next_context_id,
        rules: Arc::clone(engine.show_rules),
        styles: capture_styles(
            engine.styles,
            engine.show_rules,
            ctx,
            engine.current_file,
        )
        .push_custom(PRODUCT, crate::entities::value::Value::Bool(true)),
        guards: engine.active_guards.clone(),
        file: engine.current_file,
    };
    Content::Realized(Arc::new(crate::entities::content::RealizedContent {
        original: Content::ShowRecipe(Arc::new(origin)),
        visual,
    }))
}

fn contextual_root(content: &Content) -> bool {
    match content.source_body() {
        Content::Styled(styled) => contextual_root(&styled.body),
        Content::ContextBlock(_) => true,
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
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
        | Content::Dynamic(_) => false,
    }
}

pub(crate) fn capture_local_output(
    body: Content,
    ctx: &mut EvalContext,
    engine: &Engine<'_>,
) -> Content {
    if contextual_root(&body) {
        return body;
    }
    let Some(frame) = ctx.textual_frames.last() else { return body };
    if !engine
        .show_rules
        .iter()
        .any(|rule| !frame.rules.iter().any(|base| base.id == rule.id))
    {
        return body;
    }
    ctx.has_deferred_show_rules = true;
    Content::ShowRecipe(Arc::new(ShowRecipeContent {
        body,
        textual: false,
        context_id_floor: ctx.next_context_id,
        rules: Arc::clone(engine.show_rules),
        styles: capture_styles(
            engine.styles,
            engine.show_rules,
            ctx,
            engine.current_file,
        ),
        guards: engine.active_guards.clone(),
        file: engine.current_file,
    }))
}

/// Snapshot causal de show e style no instante em que uma ocorrência de
/// markup é avaliada. Não é exposto como valor da linguagem.
#[derive(Clone)]
pub(crate) struct TextualSnapshot {
    rules: Arc<[ShowRule]>,
    styles: StyleChain,
    guards: Vec<crate::entities::show::RuleId>,
}

impl TextualSnapshot {
    pub(crate) fn capture(engine: &Engine<'_>, ctx: &EvalContext) -> Self {
        Self {
            rules: Arc::clone(&*engine.show_rules),
            styles: capture_styles(
                engine.styles,
                engine.show_rules,
                ctx,
                engine.current_file,
            ),
            guards: (*engine.active_guards).clone(),
        }
    }

    fn same_group(&self, other: &Self) -> bool {
        Arc::ptr_eq(&self.rules, &other.rules)
            && self.styles.collapse() == other.styles.collapse()
            && self.guards == other.guards
            && match (baseline(&self.styles), baseline(&other.styles)) {
                (Some(left), Some(right)) => {
                    left.rules.iter().map(|r| r.id).eq(right.rules.iter().map(|r| r.id))
                }
                (None, None) => true,
                (Some(_), None) | (None, Some(_)) => false,
            }
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
/// ContentBlock ainda bruto preserva-os até ser interpolado; suas receitas
/// permanecem dados até a inserção causal. Slots consumidos tornam-se
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

    if ctx.defer_show_rules || !normalize_spaces {
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
            && matches!(nested.body.source_body(), Content::Text(_) | Content::Symbol(_))
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

pub(crate) fn coalesce_for_insertion(
    body: &Content,
    ctx: &EvalContext,
    engine: &Engine<'_>,
) -> SourceResult<Content> {
    let scope = ShowRecipeContent {
        body: Content::Empty,
        textual: true,
        context_id_floor: ctx.next_context_id,
        rules: Arc::clone(engine.show_rules),
        styles: engine.styles.clone(),
        guards: engine.active_guards.clone(),
        file: engine.current_file,
    };
    body.map_content_pre(&mut |node| {
        if is_product(node) {
            return Ok(Some(node.clone()));
        }
        let Content::ShowRecipe(nested) = node else { return Ok(None) };
        let mut guards = nested.guards.clone();
        for id in engine.active_guards.iter() {
            if !guards.contains(id) {
                guards.push(*id);
            }
        }
        let effective = ShowRecipeContent {
            rules: compose_rules(nested, engine.show_rules),
            styles: captured_styles(nested),
            guards,
            ..nested.as_ref().clone()
        };
        let compatible = effective.file == scope.file
            && Arc::ptr_eq(&effective.rules, &scope.rules)
            && crate::compiler::realize::sequences::same_textual_transport(
                &effective, &scope,
            );
        if !compatible || crate::compiler::realize::sequences::marked(nested) {
            return Ok(Some(node.clone()));
        }
        let body = coalesce_for_insertion(&effective.body, ctx, engine)?;
        if effective.textual {
            return Ok(Some(body));
        }
        let mut node_effect = false;
        if can_group(&body) {
            let _ = body.map_content_pre(&mut |node| {
                let target = match node.source_body() {
                    Content::Symbol(text) if ctx.recipe_math_depth == 0 => {
                        Content::Text(text.clone())
                    }
                    _ => node.clone(),
                };
                node_effect |= effective.rules.iter().any(|rule| {
                    selector_matching::is_node_rule(&rule.selector)
                        && selector_matching::selector_matches(&target, &rule.selector)
                });
                Ok(None)
            });
        }
        Ok(Some(if can_group(&body) && !node_effect { body } else { node.clone() }))
    })
}

pub(crate) fn can_group(content: &Content) -> bool {
    matching::textual_piece(content).is_some()
}

fn emitted_group_parts(content: &Content, out: &mut Vec<Content>) {
    match content {
        Content::Sequence(children) => {
            for child in children.iter() {
                emitted_group_parts(child, out);
            }
        }
        Content::Source(source) => {
            let start = out.len();
            emitted_group_parts(&source.body, out);
            for part in &mut out[start..] {
                *part = part.clone().with_source_span(source.span);
            }
        }
        Content::Realized(done) if !is_product(content) => {
            out.push(Content::Realized(Arc::new(
                crate::entities::content::RealizedContent {
                    original: done.original.clone(),
                    visual: Content::Empty,
                },
            )));
            emitted_group_parts(&done.visual, out);
        }
        Content::ShowRecipe(_)
        | Content::Realized(_)
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
        | Content::Dynamic(_) => out.push(content.clone()),
    }
}

pub(crate) fn realize_captured_text(
    content: Content,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    if !engine.show_rules.iter().any(|rule| {
        selector_matching::is_text_rule(&rule.selector)
            && !engine.active_guards.contains(&rule.id)
    }) {
        return Ok(content);
    }
    let original = content.clone();
    let mut parts = Vec::new();
    emitted_group_parts(&content, &mut parts);
    space::normalize_markup_spaces(&mut parts);
    let snapshots = vec![TextualSnapshot::capture(engine, ctx); parts.len()];
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
