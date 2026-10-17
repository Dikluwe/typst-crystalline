//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph_plan.md
//! @prompt-hash 39212544
//! @layer L1
//! @updated 2026-10-08

use super::{
    html_frame, paragraph_emit, paragraph_prepare,
    paragraph_select::{self, Strategy},
    FontMetrics, ImageSizer, Layouter,
};
use crate::entities::{content::Content, value::Value};
fn strategy<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> Option<Strategy> {
    match l.chain.custom("par.linebreaks") {
        Some(Value::Str(s)) if s.as_str() == "simple" => Some(Strategy::Simple),
        Some(Value::Str(s)) if s.as_str() == "optimized" => Some(Strategy::Optimized),
        Some(Value::Auto) => {
            Some(if matches!(l.chain.custom("par.justify"), Some(Value::Bool(true))) {
                Strategy::Optimized
            } else {
                Strategy::Simple
            })
        }
        None => {
            Some(if matches!(l.chain.custom("par.justify"), Some(Value::Bool(true))) {
                Strategy::Optimized
            } else {
                Strategy::Simple
            })
        }
        _ => None,
    }
}
fn integral_styled(parts: &[Content]) -> bool {
    let mut material = parts.iter().filter(|part| {
        !matches!(part, Content::Empty | Content::Space | Content::SpaceFromNewline)
    });
    matches!(material.next(), Some(Content::Styled(_))) && material.next().is_none()
}
fn defer_integral_styled(body: &Content) -> bool {
    matches!(body, Content::Styled(_))
        || matches!(body,Content::Sequence(parts) if integral_styled(parts))
}
fn implicit_regional_only<M: FontMetrics, S: ImageSizer>(l: &Layouter<M, S>) -> bool {
    l.chain.custom("par.linebreaks").is_none()
        && !matches!(l.chain.custom("par.justify"), Some(Value::Bool(true)))
        && !matches!(l.chain.custom("text.hyphenate"), Some(Value::Bool(true)))
}
fn single_overlong_word(p: &paragraph_prepare::Prepared, width: f64) -> bool {
    matches!(p.words.as_slice(), [word] if word.before.is_empty() && word.width > width)
}
fn options<M: FontMetrics, S: ImageSizer>(
    l: &Layouter<M, S>,
) -> Option<(Strategy, f64, f64, bool)> {
    let strategy = strategy(l)?;
    let width = l.regions.current.width
        - l.page_config.margin.right
        - l.regions.current.line_start_x.0;
    if !width.is_finite()
        || width <= 0.0
        || l.column_mode
        || l.page_columns.is_some()
        || l.regions.cell.is_some()
        || !l.floats_pending.is_empty()
        || l.is_sub_frame
        || l.smallcaps
        || html_frame::items_have_material(&l.regions.current.current_line)
        || l.regions.current.cursor_x != l.regions.current.line_start_x
    {
        return None;
    }
    let justify = matches!(l.chain.custom("par.justify"), Some(Value::Bool(true)));
    let font = super::text::effective_style(l).size.0;
    if !font.is_finite() || font <= 0.0 {
        return None;
    }
    Some((strategy, width, font, justify))
}
pub(super) fn try_layout<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    body: &Content,
) -> bool {
    if defer_integral_styled(body) {
        return false;
    }
    let Some((strategy, width, font, justify)) = options(l) else { return false };
    let Some(p) = paragraph_prepare::prepare(l, body) else { return false };
    let regional_only = implicit_regional_only(l);
    let costs = crate::compiler::text_costs::resolve(&l.chain);
    if p.words.iter().any(|w| w.hyphenation.is_some()) {
        let lines =
            super::paragraph_hyphen::select(&p, width, font, justify, strategy, costs);
        if regional_only && lines.len() < 2 && !single_overlong_word(&p, width) {
            return false;
        }
        paragraph_emit::emit_prepared_lines(l, &lines, width, justify);
    } else {
        let lines = paragraph_select::select(&p, width, font, justify, strategy, costs);
        if regional_only && lines.len() < 2 && !single_overlong_word(&p, width) {
            return false;
        }
        paragraph_emit::emit(l, &p, &lines, width, justify);
    }
    true
}
pub(super) fn try_parts<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    parts: &[Content],
) -> bool {
    let Some((strategy, width, font, justify)) = options(l) else { return false };
    let regional_only = implicit_regional_only(l);
    if parts
        .split(|part| matches!(part, Content::Parbreak))
        .any(integral_styled)
    {
        return false;
    }
    let costs = crate::compiler::text_costs::resolve(&l.chain);
    let mut prepared = Vec::new();
    for segment in parts.split(|part| matches!(part, Content::Parbreak)) {
        if segment.iter().all(|part| {
            matches!(part, Content::Empty | Content::Space | Content::SpaceFromNewline)
        }) {
            prepared.push(None);
            continue;
        }
        let Some(p) = paragraph_prepare::prepare_parts(l, segment) else { return false };
        if p.words.iter().any(|w| w.hyphenation.is_some()) {
            let lines = super::paragraph_hyphen::select(
                &p, width, font, justify, strategy, costs,
            );
            prepared.push(Some((p, Vec::new(), Some(lines))));
        } else {
            let lines =
                paragraph_select::select(&p, width, font, justify, strategy, costs);
            prepared.push(Some((p, lines, None)));
        }
    }
    if prepared.iter().all(Option::is_none) {
        return false;
    }
    if regional_only
        && prepared.iter().flatten().all(|(p, lines, hyphenated)| {
            hyphenated.as_ref().map_or(lines.len(), Vec::len) < 2
                && !single_overlong_word(p, width)
        })
    {
        return false;
    }
    for (index, segment) in prepared.iter().enumerate() {
        if index > 0 {
            super::parbreak::layout(l);
        }
        if let Some((p, lines, hyphenated)) = segment {
            if let Some(lines) = hyphenated {
                paragraph_emit::emit_prepared_lines(l, lines, width, justify);
            } else {
                paragraph_emit::emit(l, p, lines, width, justify);
            }
        }
    }
    true
}

#[cfg(test)]
mod contiguous_overlong_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        elements::context_block::ContextBlockElem,
        func::Func,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMarginSpec, PageMargins},
    };
    use comemo::Track;
    use std::sync::Arc;

    #[test]
    fn adjacent_numeric_fragments_use_one_overlong_line() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 100.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        let mut parts = vec![Content::Space];
        parts.extend((0..100).map(|i| Content::text(i.to_string())));
        parts.push(Content::Space);
        assert!(try_parts(&mut l, &parts));
        let doc = l.finish();
        assert_eq!(
            doc.pages[0].plain_text(),
            (0..100).map(|i| i.to_string()).collect::<String>()
        );
    }

    #[test]
    fn contextual_markers_are_neutral_only_in_paginated_layout() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 100.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 100.0;
        let marker = Content::ContextBlock(Arc::new(ContextBlockElem {
            id: 0,
            closure: Func::native("unused", |_, _, _, _| panic!("must not execute")),
        }));
        let mut parts = vec![Content::Space];
        parts.extend((0..100).map(|i| {
            Content::sequence(vec![marker.clone(), Content::text(i.to_string())])
        }));
        parts.push(Content::Space);
        assert!(try_parts(&mut l, &parts));
        let mut html = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        html.capture_html_context_markers = true;
        assert!(!try_parts(&mut html, &parts));
    }
}
