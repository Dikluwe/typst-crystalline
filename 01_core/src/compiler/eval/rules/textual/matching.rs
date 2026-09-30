//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/eval/rules/textual/matching.md
//! @prompt-hash 1dc051b7
//! @layer L1
//! @updated 2026-09-30
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
) -> SourceResult<()> {
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
            has_text |= matches!(&parts[end], Content::Text(_));
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
                    for part in &mut parts[start + 1..end] {
                        *part = Content::Empty;
                    }
                }
            }
        }
        start = end.max(start + 1);
    }
    Ok(())
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
        Content::Empty => Some(String::new()),
        Content::Text(text) => Some(text.to_string()),
        Content::Space | Content::SpaceFromNewline => Some(" ".to_string()),
        // A quebra explícita é fronteira destrutiva, nunca texto agregado.
        _ => None,
    }
}
