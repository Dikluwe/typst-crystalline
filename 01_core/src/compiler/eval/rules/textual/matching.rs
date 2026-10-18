//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual/matching.md
//! @prompt-hash 91d489a0
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

use projection::{append_flat_sequence, project_original_range};

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
            has_text |= piece.chars().any(|ch| !ch.is_whitespace());
            merged.push_str(&piece);
            end += 1;
        }

        if has_text && !merged.is_empty() {
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
                let old_styles =
                    std::mem::replace(engine.styles, snapshot.styles.clone());
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

/// Regex conserva a precedência reversa vigente; o primeiro selector regex
/// que casa transforma o grupo. Literais correm depois na ordem de declaração.
/// Cada passe observa somente a entrada desse passe: a própria substituição
/// não regressa à mesma regra, mas regras posteriores podem ver o seu texto.
fn apply_group_text_rules(
    original: &[Content],
    rules: &[ShowRule],
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Option<Vec<Content>>> {
    route_check_show_depth(engine.route)?;
    let mut scopes = Scopes::new(None);
    let mut work = original.to_vec();
    let mut applied = false;

    for rule in rules.iter().rev() {
        if !matches!(rule.selector, Selector::Regex(_)) {
            continue;
        }
        let (next, matched) =
            splice_rule_over_runs(&work, rule, &mut scopes, ctx, engine)?;
        if matched {
            work = next;
            applied = true;
            break;
        }
    }
    for rule in rules {
        if !matches!(rule.selector, Selector::Text(_)) {
            continue;
        }
        let (next, matched) =
            splice_rule_over_runs(&work, rule, &mut scopes, ctx, engine)?;
        if matched {
            work = next;
            applied = true;
        }
    }
    Ok(applied.then_some(work))
}

fn splice_rule_over_runs(
    input: &[Content],
    rule: &ShowRule,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<(Vec<Content>, bool)> {
    let mut out = Vec::new();
    let mut applied = false;
    let mut start = 0;
    while start < input.len() {
        if textual_piece(&input[start]).is_none() {
            out.push(input[start].clone());
            start += 1;
            continue;
        }
        let mut end = start;
        let mut merged = String::new();
        while end < input.len() {
            let Some(piece) = textual_piece(&input[end]) else {
                break;
            };
            merged.push_str(&piece);
            end += 1;
        }
        let ranges = selector_matching::textual_match_ranges(&rule.selector, &merged);
        if ranges.is_empty() {
            out.extend_from_slice(&input[start..end]);
        } else {
            applied = true;
            let source = &input[start..end];
            let mut cursor = 0;
            for (match_start, match_end) in ranges {
                project_original_range(source, cursor, match_start, &mut out);
                append_flat_sequence(
                    &mut out,
                    transform_textual_match(
                        rule,
                        &merged[match_start..match_end],
                        scopes,
                        ctx,
                        engine,
                    )?,
                );
                cursor = match_end;
            }
            project_original_range(source, cursor, merged.len(), &mut out);
        }
        start = end;
    }
    Ok((out, applied))
}

fn transform_textual_match(
    rule: &ShowRule,
    matched: &str,
    scopes: &mut Scopes<'_>,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    match &rule.transform {
        Transformation::Func(func) => {
            let args = Args::positional(vec![Value::Content(Content::text(matched))]);
            engine.active_guards.push(rule.id);
            let result =
                call_dispatch::apply_func(func.clone(), args, scopes, ctx, engine);
            engine.active_guards.pop();
            match result? {
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
        Transformation::Style(_) => Ok(Content::text(matched)),
    }
}

fn textual_piece(content: &Content) -> Option<String> {
    match content {
        Content::Realized(e) => textual_piece(&e.visual),
        Content::Source(e) => textual_piece(&e.body),
        Content::Located(e) => textual_piece(&e.body),
        Content::Sequence(parts) => parts
            .iter()
            .map(textual_piece)
            .collect::<Option<Vec<_>>>()
            .map(|v| v.concat()),
        Content::Empty => Some(String::new()),
        Content::Text(text) => Some(text.to_string()),
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
