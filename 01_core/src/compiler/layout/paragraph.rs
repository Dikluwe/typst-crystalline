//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/paragraph.md
//! @prompt-hash 4609e67f
//! @layer L1
//! @updated 2026-09-30

use super::{html_frame, parbreak, FontMetrics, ImageSizer, Layouter};
use crate::entities::{content::Content, layout_types::Pt, value::Value};

pub(super) fn is_paragraph(content: &Content) -> bool {
    match content {
        Content::Par { .. } => true,
        Content::Styled(styled) => is_paragraph(&styled.body),
        _ => false,
    }
}

fn is_empty_body(content: &Content) -> bool {
    match content {
        Content::Empty | Content::Space | Content::SpaceFromNewline => true,
        Content::Text(text) => text.is_empty(),
        Content::Sequence(seq) => seq.iter().all(is_empty_body),
        Content::Styled(styled) => is_empty_body(&styled.body),
        _ => false,
    }
}

fn contains_empty_text(content: &Content) -> bool {
    match content {
        Content::Text(text) => text.is_empty(),
        Content::Sequence(seq) => seq.iter().any(contains_empty_text),
        Content::Styled(styled) => contains_empty_text(&styled.body),
        _ => false,
    }
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &Content,
) {
    let had_previous_text =
        html_frame::items_have_material(&layouter.regions.current.current_line)
            || layouter.terminal_paragraph_line;
    parbreak::layout(layouter);
    let empty_body = is_empty_body(body);
    if !empty_body && !super::paragraph_plan::try_layout(layouter, body) {
        layouter.layout_content(body);
    }
    if empty_body {
        let size = layouter.style.size.val();
        let spacing = match layouter.chain.custom("par.spacing") {
            Some(Value::Length(length)) => length.resolve_pt(size),
            _ => size * super::vanilla_defaults::PAR_SPACING,
        };
        let virtual_top = if contains_empty_text(body) {
            layouter.metrics.text_edges(layouter.style.size, &layouter.style).0 .0
        } else {
            0.0
        };
        if layouter.initial_baseline_pending {
            layouter.empty_paragraph_weak_spacing += spacing;
        }
        if had_previous_text {
            layouter.regions.current.cursor_y =
                Pt(layouter.prev_line_baseline + spacing + virtual_top);
        } else {
            layouter.regions.current.cursor_y += Pt(spacing + virtual_top);
        }
        if let Some(bottom) = layouter.last_block_descent_y {
            layouter.last_block_descent_y = Some(bottom.max(layouter.regions.current.cursor_y.0));
        }
        layouter.prev_block_below_pending += spacing + virtual_top;
        layouter.block_chain_active = true;
        layouter.prev_margin_is_parbreak = true;
        layouter.last_flush_advance = 0.0;
        layouter.terminal_paragraph_line = false;
    } else if html_frame::items_have_material(&layouter.regions.current.current_line) {
        parbreak::layout(layouter);
        layouter.terminal_paragraph_line = true;
    }
}

#[cfg(test)]
mod tests {
    #![allow(deprecated)]
    use super::super::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, PageMarginSpec, PageMargins, Pt, TextStyle},
        style::{Style, Styles},
        value::Value,
    };
    use comemo::Track;

    pub(super) fn make(
        intr: &dyn Introspector,
        justify: bool,
    ) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr.track());
        l.chain = l.chain.push_styles(&Styles::from_iter([Style::Size(Pt(12.0))]));
        l.chain = l
            .chain
            .push_custom("par.justify", Value::Bool(justify))
            .push_custom("text.hyphenate", Value::Bool(false));
        l.style = TextStyle::from(&l.chain);
        l.page_config.width = 70.0;
        l.page_config.height = 400.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 70.0;
        l.regions.current.height = 400.0;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l
    }
    fn texts(items: &[FrameItem], out: &mut Vec<(String, f64, f64)>) {
        for item in items {
            match item {
                FrameItem::Text { text, pos, .. } if !text.is_empty() => {
                    out.push((text.to_string(), pos.x.0, pos.y.0))
                }
                FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. }
                | FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. } => texts(items, out),
                _ => {}
            }
        }
    }
    pub(super) fn positions(
        doc: &crate::entities::layout_types::PagedDocument,
    ) -> Vec<(String, f64, f64)> {
        let mut out = Vec::new();
        for p in &doc.pages {
            texts(&p.items, &mut out)
        }
        out
    }
    #[test]
    fn composition_justifies_only_material_continuation() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, true);
        l.layout_content(&Content::text("aa bb ccccccc"));
        let p = positions(&l.finish());
        let a = p.iter().find(|(s, _, _)| s == "aa").unwrap();
        let b = p.iter().find(|(s, _, _)| s == "bb").unwrap();
        let last = p.iter().find(|(s, _, _)| s == "ccccccc").unwrap();
        assert!((a.1 - 0.0).abs() < 1e-8);
        assert!((b.1 + 14.4 - 70.0).abs() < 1e-8);
        assert_eq!(a.2, b.2);
        assert_eq!(last.1, 0.0);
        assert!(last.1 + 50.4 < 70.0);
        assert!(last.2 > b.2);
    }
    #[test]
    fn composition_final_literal_spaces_keep_natural_width() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, true);
        l.layout_content(&Content::text("aa bb                 "));
        let p = positions(&l.finish());
        assert_eq!(p.len(), 2);
        assert!((p[1].1 - 21.6).abs() < 1e-8);
        assert_eq!(p[0].2, p[1].2);
    }
    #[test]
    fn composition_native_par_has_before_and_after_boundaries() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::text("A"));
        l.layout_content(&Content::par(Content::text("B")));
        l.layout_content(&Content::text("C"));
        let p = positions(&l.finish());
        assert_eq!(p.len(), 3);
        for item in &p {
            assert_eq!(item.1, 0.0)
        }
        assert!((p[1].2 - p[0].2 - 22.8).abs() < 1e-8);
        assert!((p[2].2 - p[1].2 - 22.8).abs() < 1e-8);
    }
    #[test]
    fn composition_empty_paragraph_reserves_spacing_without_ink() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::par(Content::Empty));
        l.layout_content(&Content::text("A"));
        let p = positions(&l.finish());
        assert_eq!(p.len(), 1);
        assert!((p[0].2 - 22.8).abs() < 1e-8);
    }
    #[test]
    fn composition_terminal_paragraph_auto_height_is_causal() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::par(Content::text("A")));
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        assert!((doc.pages[0].height - 8.4).abs() < 1e-8);
    }
    #[test]
    fn composition_empty_terminal_auto_height_keeps_explicit_spacing() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::text("A"));
        l.layout_content(&Content::par(Content::Empty));
        let doc = l.finish();
        assert_eq!(doc.pages.len(), 1);
        assert!((doc.pages[0].height - 22.8).abs() < 1e-8);
    }
    #[test]
    fn composition_rollback_restores_terminal_paragraph_geometry_and_prefix() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, true);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::par(Content::text("A")));
        assert!(!super::super::html_frame::items_have_material(
            &l.regions.current.current_line
        ));
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.layout_content(&Content::text("trial words overflow more"));
        l.restore_flow_suffix_checkpoint(checkpoint);
        let doc = l.finish();
        assert_eq!(
            positions(&doc).iter().map(|p| p.0.as_str()).collect::<Vec<_>>(),
            vec!["A"]
        );
        assert_eq!(doc.pages.len(), 1);
        assert!((doc.pages[0].height - 8.4).abs() < 1e-8);
    }
    #[test]
    fn composition_empty_styled_terminal_auto_height_keeps_explicit_spacing() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.page_config.height = f64::INFINITY;
        l.regions.current.height = f64::INFINITY;
        l.layout_content(&Content::text("A"));
        l.layout_content(&Content::styled(
            Content::par(Content::Empty),
            Styles::new().push_custom(
                "par.leading",
                Value::Length(crate::entities::layout_types::Length::pt(2.0)),
            ),
        ));
        let doc = l.finish();
        assert!((doc.pages[0].height - 22.8).abs() < 1e-8);
    }

    #[test]
    fn composition_empty_only_auto_height_discards_weak_spacing() {
        for (body, height) in [(Content::Empty, 0.0), (Content::text(""), 8.4)] {
            let intr = TagIntrospector::empty();
            let mut l = make(&intr, false);
            l.page_config.height = f64::INFINITY;
            l.regions.current.height = f64::INFINITY;
            l.layout_content(&Content::par(body));
            let doc = l.finish();
            assert!(
                (doc.pages[0].height - height).abs() < 1e-8,
                "{:?}",
                doc.pages[0].height
            );
        }
    }
    #[test]
    fn composition_empty_text_reserves_virtual_top_edge() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::par(Content::text("")));
        l.layout_content(&Content::text("A"));
        let p = positions(&l.finish());
        assert!((p[0].2 - 31.2).abs() < 1e-8);
    }
}

#[cfg(test)]
mod global_selection_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, value::Value,
    };
    fn lines(strategy: &str) -> Vec<String> {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.regions.current.width = 124.0;
        l.page_config.width = 124.0;
        l.chain = l.chain.push_custom("par.linebreaks", Value::Str(strategy.into()));
        l.layout_content(&Content::par(Content::text("a bbbbb cc ddddddd eeeee ff g hhhhhhh iiiiiiiii jjjjjjjjj kkkkkk llllll m nnn ooo")));
        let words = positions(&l.finish());
        let mut lines: Vec<(f64, Vec<String>)> = Vec::new();
        for (word, _, y) in words {
            if lines.last().is_none_or(|line| line.0 != y) {
                lines.push((y, Vec::new()));
            }
            lines.last_mut().unwrap().1.push(word);
        }
        lines.into_iter().map(|(_, w)| w.join(" ")).collect()
    }
    #[test]
    fn global_selection_strategy_changes_breaks_under_identical_metrics() {
        let simple = lines("simple");
        let optimized = lines("optimized");
        assert_eq!(simple.join(" "), optimized.join(" "));
        assert_ne!(simple, optimized, "linebreaks must not be discarded");
    }
}

#[cfg(test)]
mod configured_scope_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, style::Styles, value::Value,
    };
    fn run(outer: bool) -> Vec<(String, f64, f64)> {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.regions.current.width = 124.0;
        l.page_config.width = 124.0;
        let text=Content::text("a bbbbb cc ddddddd eeeee ff g hhhhhhh iiiiiiiii jjjjjjjjj kkkkkk llllll m nnn ooo");
        let styled = Content::styled(
            text,
            Styles::new().push_custom("par.linebreaks", Value::Str("optimized".into())),
        );
        let content = if outer {
            Content::sequence(vec![Content::Empty, styled, Content::SpaceFromNewline])
        } else {
            styled
        };
        l.layout_content(&content);
        assert!(!l.paragraph_planning_suppressed);
        positions(&l.finish())
    }
    #[test]
    fn global_selection_terminal_scope_after_configuration_is_complete() {
        assert_eq!(run(true), run(false));
    }
}

#[cfg(test)]
mod guard_checkpoint_tests {
    use super::tests::make;
    use crate::entities::{content::Content, introspector::TagIntrospector};
    #[test]
    fn paragraph_planning_guard_survives_rejected_suffix() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.layout_content(&Content::par(Content::text("prefix")));
        l.paragraph_planning_suppressed = true;
        let saved = l.capture_flow_suffix_checkpoint();
        l.paragraph_planning_suppressed = false;
        l.layout_content(&Content::text("suffix"));
        l.restore_flow_suffix_checkpoint(saved);
        assert!(l.paragraph_planning_suppressed);
        assert_eq!(
            super::tests::positions(&l.finish())
                .into_iter()
                .map(|p| p.0)
                .collect::<Vec<_>>(),
            vec!["prefix"]
        );
    }
}

#[cfg(test)]
mod multi_paragraph_selection_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, value::Value,
    };
    fn lines(explicit: bool) -> Vec<String> {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.regions.current.width = 124.0;
        l.page_config.width = 124.0;
        l.chain = l.chain.push_custom("par.linebreaks", Value::Str("optimized".into()));
        let body=Content::text("a bbbbb cc ddddddd eeeee ff g hhhhhhh iiiiiiiii jjjjjjjjj kkkkkk llllll m nnn ooo");
        let content = if explicit {
            Content::sequence(vec![Content::par(body.clone()), Content::par(body)])
        } else {
            Content::sequence(vec![body.clone(), Content::Parbreak, body])
        };
        l.layout_content(&content);
        let words = positions(&l.finish());
        let mut lines: Vec<(f64, Vec<String>)> = Vec::new();
        for (w, _, y) in words {
            if lines.last().is_none_or(|l| l.0 != y) {
                lines.push((y, Vec::new()));
            }
            lines.last_mut().unwrap().1.push(w);
        }
        lines.into_iter().map(|(_, w)| w.join(" ")).collect()
    }
    #[test]
    fn global_selection_prepares_each_complete_paragraph_before_emitting() {
        assert_eq!(lines(false), lines(true));
    }
}

#[cfg(test)]
mod global_hyphenation_origin_tests {
    use super::tests::{make, positions};
    use crate::entities::{
        content::Content, introspector::TagIntrospector, value::Value,
    };
    #[test]
    fn global_hyphenation_splits_overlong_word_at_line_origin() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr, false);
        l.chain = l
            .chain
            .push_custom("text.hyphenate", Value::Bool(true))
            .push_custom("par.linebreaks", Value::Str("simple".into()));
        l.layout_content(&Content::par(Content::text("representation")));
        let positions = positions(&l.finish());
        assert!(positions.iter().any(|(text, _, _)| text.contains('-')));
        assert_eq!(
            positions
                .iter()
                .map(|(t, _, _)| t.replace('-', ""))
                .collect::<String>(),
            "representation"
        );
        assert!(positions.last().unwrap().2 > positions[0].2);
    }
}
