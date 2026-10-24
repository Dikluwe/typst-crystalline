//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/eval/rules/textual/matching.md
//! @prompt-hash d202e6df
//! @layer L1
//! @updated 2026-10-17
//!
//! Agrupamento, recorte e transformação de matches textuais.

use std::sync::Arc;

use crate::compiler::scopes::Scopes;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::engine::Engine;
use crate::entities::show::{Selector, ShowRule, Transformation};
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;
use crate::entities::style::Styles;
use crate::entities::value::Value;
use crate::entities::world_types::check_show_depth as route_check_show_depth;

use super::super::super::{call_dispatch, selector_matching, EvalContext};
use super::TextualSnapshot;

mod projection;

use projection::project_original_range;

pub(super) fn apply_textual_groups(
    parts: &mut [Content],
    snapshots: &[TextualSnapshot],
    style_boundaries: &[(usize, Styles)],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Vec<bool>> {
    let mut consumed = vec![false; parts.len()];
    let mut start = 0;
    while start < parts.len() {
        if textual_piece(&parts[start]).is_none() {
            start += 1;
            continue;
        }
        let snapshot = &snapshots[start];
        let mut end = start;
        let mut merged = String::new();
        let mut has_text = false;
        while end < parts.len()
            && (end == start
                || (!style_boundaries.iter().any(|(index, _)| *index == end)
                    && snapshot.same_group(&snapshots[end])))
        {
            let Some(piece) = textual_piece(&parts[end]) else {
                break;
            };
            has_text |=
                piece.chars().any(|ch| !ch.is_whitespace()) || has_real_text(&parts[end]);
            merged.push_str(&piece);
            end += 1;
        }

        if (has_text || (ctx.recipe_text_edges.0 && ctx.recipe_text_edges.1))
            && !merged.is_empty()
        {
            let rules: Vec<ShowRule> = snapshot
                .rules
                .iter()
                .filter(|rule| {
                    !snapshot.guards.contains(&rule.id)
                        && selector_matching::is_text_rule(&rule.selector)
                })
                .cloned()
                .collect();
            if rules.iter().any(|rule| {
                selector_matching::textual_selector_matches(&rule.selector, &merged)
            }) {
                let old_rules =
                    std::mem::replace(engine.show_rules, Arc::clone(&snapshot.rules));
                let old_styles = std::mem::replace(
                    engine.styles,
                    super::snapshot_styles(&snapshot.styles),
                );
                let old_guards =
                    std::mem::replace(engine.active_guards, snapshot.guards.clone());
                let original = parts[start..end].to_vec();
                let transformed = apply_group_text_rules(&original, &rules, ctx, engine);
                *engine.show_rules = old_rules;
                *engine.styles = old_styles;
                *engine.active_guards = old_guards;
                if let Some(replacement) = transformed? {
                    parts[start] = Content::sequence(replacement);
                    consumed[start + 1..end].fill(true);
                    for part in &mut parts[start + 1..end] {
                        *part = Content::Empty;
                    }
                }
            }
        }
        start = end.max(start + 1);
    }
    Ok(consumed)
}

/// Select by earliest input offset, with lexical precedence on a tie. The
/// cache contains one future match per rule, never state per input byte.
fn apply_group_text_rules(
    original: &[Content],
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Vec<Content>>> {
    route_check_show_depth(engine.route)?;
    let merged = original.iter().filter_map(textual_piece).collect::<String>();
    let significant = significant_bounds(original).unwrap_or((merged.len(), 0));
    let bounds = (
        if ctx.recipe_text_edges.0 { 0 } else { significant.0 },
        if ctx.recipe_text_edges.1 { merged.len() } else { significant.1 },
    );
    if bounds.0 >= bounds.1 {
        return Ok(None);
    }
    let mut first = rules
        .iter()
        .map(|rule| {
            if engine.active_guards.contains(&rule.id) {
                None
            } else {
                first_match(&rule.selector, &merged[bounds.0..bounds.1])
                    .map(|(a, b)| (bounds.0 + a, bounds.0 + b))
            }
        })
        .collect::<Vec<_>>();
    let mut cursor = 0;
    let mut out = Vec::new();
    let mut scopes = Scopes::new(None);
    let mut applied = false;
    loop {
        let mut selected = None;
        for (index, candidate) in first.iter().enumerate().rev() {
            if let Some((start, end)) = candidate {
                if selected.is_none_or(|(_, previous, _)| *start < previous) {
                    selected = Some((index, *start, *end));
                }
            }
        }
        let Some((index, start, end)) = selected else { break };
        applied = true;
        project_original_range(original, cursor, start, &mut out);
        let mut source = Vec::new();
        project_original_range(original, start, end, &mut source);
        let matched = Content::sequence(source);
        let rule = &rules[index];
        if ctx.occurrence_show_depth
            >= crate::entities::world_types::Route::MAX_SHOW_RULE_DEPTH
        {
            return Err(vec![SourceDiagnostic::error(
                matched.source_span(),
                "maximum show rule depth exceeded",
            )
            .with_hint("maybe a show rule matches its own output")
            .with_hint("maybe there are too deeply nested elements")]);
        }
        let guard_depth = engine.active_guards.len();
        engine.active_guards.push(rule.id);
        ctx.occurrence_show_depth += 1;
        let previous_edges = ctx.recipe_text_edges;
        ctx.recipe_text_edges =
            (start > bounds.0 || previous_edges.0, end < bounds.1 || previous_edges.1);
        let previous_defer = std::mem::replace(&mut ctx.defer_show_rules, true);
        let produced = transform_textual_match(
            rule,
            &merged[start..end],
            &matched,
            &mut scopes,
            ctx,
            engine,
        );
        ctx.defer_show_rules = previous_defer;
        let visual = produced.and_then(|content| {
            let content = super::coalesce_for_insertion(&content, ctx, engine)?;
            crate::compiler::realize::recipes::realize(&content, ctx, engine)
        });
        let product =
            visual.map(|visual| super::complete_product(matched, visual, ctx, engine));
        ctx.recipe_text_edges = previous_edges;
        ctx.occurrence_show_depth -= 1;
        engine.active_guards.truncate(guard_depth);
        out.push(product?);
        cursor = end;
        for (rule, candidate) in rules.iter().zip(&mut first) {
            if candidate.is_some_and(|(at, _)| at < cursor) {
                *candidate = if engine.active_guards.contains(&rule.id) {
                    None
                } else {
                    first_match(&rule.selector, &merged[cursor.min(bounds.1)..bounds.1])
                        .map(|(a, b)| (cursor + a, cursor + b))
                };
            }
        }
    }
    if applied {
        project_original_range(original, cursor, merged.len(), &mut out);
    }
    Ok(applied.then_some(out))
}

// Bounds retain explicit Text whitespace while excluding weak source edges.
fn significant_bounds(parts: &[Content]) -> Option<(usize, usize)> {
    let mut offset = 0;
    let mut bounds: Option<(usize, usize)> = None;
    for part in parts {
        let length = textual_piece(part).map_or(0, |text| text.len());
        let local = match part.source_body() {
            Content::Sequence(children) => significant_bounds(children),
            Content::Located(located) => {
                significant_bounds(std::slice::from_ref(&located.body))
            }
            Content::Realized(done) if !super::is_product(part) => {
                significant_bounds(std::slice::from_ref(&done.visual))
            }
            Content::Text(text) | Content::Symbol(text) if !text.is_empty() => {
                Some((0, text.len()))
            }
            Content::Source(_)
            | Content::ShowRecipe(_)
            | Content::Realized(_)
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
            | Content::Dynamic(_) => None,
        };
        if let Some((start, end)) = local {
            bounds = Some((bounds.map_or(offset + start, |old| old.0), offset + end));
        }
        offset += length;
    }
    bounds
}

fn first_match(selector: &Selector, text: &str) -> Option<(usize, usize)> {
    match selector {
        Selector::Text(pattern) if !pattern.is_empty() => {
            text.find(pattern).map(|start| (start, start + pattern.len()))
        }
        Selector::Regex(regex) => regex.captures_first(text).and_then(|first| {
            if first.start < first.end {
                Some((first.start, first.end))
            } else {
                selector_matching::textual_match_ranges(selector, text)
                    .into_iter()
                    .next()
            }
        }),
        Selector::Text(_)
        | Selector::NodeKind(_)
        | Selector::NativeElement(_)
        | Selector::DynKind(_)
        | Selector::Where { .. }
        | Selector::And(_)
        | Selector::Or(_)
        | Selector::Label(_) => None,
    }
}

fn transform_textual_match(
    rule: &ShowRule,
    matched: &str,
    original: &Content,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    match &rule.transform {
        Transformation::Func(func) => {
            let args = Args::positional(vec![Value::Content(
                Content::text(matched).with_source_span(original.source_span()),
            )]);
            match call_dispatch::apply_func(func.clone(), args, scopes, ctx, engine)? {
                Value::Content(content) => Ok(content),
                Value::Str(text) => Ok(Content::text(text.as_str())),
                other => {
                    let label = if matches!(rule.selector, Selector::Regex(_)) {
                        "show rule regex"
                    } else {
                        "show rule"
                    };
                    Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "{label} deve retornar Content ou String, recebeu {}",
                            other.type_name()
                        ),
                    )])
                }
            }
        }
        Transformation::Content(content) => Ok(content.clone()),
        Transformation::Str(text) => Ok(Content::text(text.as_str())),
        Transformation::Style(styles) => Ok(Content::styled(
            Content::text(matched).with_source_span(original.source_span()),
            styles.as_ref().clone(),
        )),
    }
}

pub(super) fn textual_piece(content: &Content) -> Option<String> {
    match content {
        Content::Realized(e) => {
            if super::is_product(content) {
                None
            } else {
                textual_piece(&e.visual)
            }
        }
        Content::Source(e) => textual_piece(&e.body),
        Content::Located(e) => textual_piece(&e.body),
        Content::Sequence(parts) => parts
            .iter()
            .map(textual_piece)
            .collect::<Option<Vec<_>>>()
            .map(|v| v.concat()),
        Content::Empty => Some(String::new()),
        Content::Text(text) | Content::Symbol(text) => Some(text.to_string()),
        Content::Space | Content::SpaceFromNewline => Some(" ".to_string()),
        // A quebra explícita é fronteira destrutiva, nunca texto agregado.
        Content::ShowRecipe(_)
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
        | Content::Dynamic(_) => None,
    }
}

fn has_real_text(content: &Content) -> bool {
    match content {
        Content::Text(text) | Content::Symbol(text) => !text.is_empty(),
        Content::Source(e) => has_real_text(&e.body),
        Content::Located(e) => has_real_text(&e.body),
        Content::Realized(e) => !super::is_product(content) && has_real_text(&e.visual),
        Content::Sequence(parts) => parts.iter().any(has_real_text),
        Content::Empty | Content::Space | Content::SpaceFromNewline => false,
        Content::ShowRecipe(_)
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
    }
}

#[cfg(test)]
mod v16_whitespace_tests {
    use super::*;
    use crate::entities::content::{LocatedContent, RealizedContent};
    use crate::entities::{file_id::FileId, location::Location};
    fn carriers(body: Content) -> Vec<Content> {
        let span = Span::from_range(
            FileId::from_raw(std::num::NonZeroU16::new(1).unwrap()),
            1..2,
        );
        vec![
            body.clone().with_source_span(span),
            Content::Located(Arc::new(LocatedContent {
                body: body.clone(),
                location: Location::from_raw(9),
            })),
            Content::Realized(Arc::new(RealizedContent {
                original: Content::linebreak(),
                visual: body,
            })),
        ]
    }
    #[test]
    fn v16_whitespace_matching_preserves_pieces() {
        for (body, expected) in [
            (Content::text("alpha"), "alpha"),
            (Content::Space, " "),
            (Content::SpaceFromNewline, " "),
            (Content::Empty, ""),
            (
                Content::sequence(vec![
                    Content::text("a"),
                    Content::Space,
                    Content::text("b"),
                ]),
                "a b",
            ),
        ] {
            for content in carriers(body) {
                assert_eq!(textual_piece(&content).as_deref(), Some(expected));
            }
        }
    }
    #[test]
    fn v16_whitespace_matching_keeps_boundaries() {
        for body in [
            Content::linebreak(),
            Content::Parbreak,
            Content::styled(Box::new(Content::text("x")), Styles::default()),
            Content::MathIdent("x".into()),
            Content::sequence(vec![
                Content::text("a"),
                Content::Parbreak,
                Content::text("b"),
            ]),
        ] {
            for content in carriers(body) {
                assert!(textual_piece(&content).is_none());
            }
        }
    }
}
