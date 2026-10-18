//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/word_layout.md
//! @prompt-hash c86912fb
//! @layer L1
//! @updated 2026-09-30

#![allow(deprecated)] // Preserve the existing Text fallback emission.
use super::{
    break_selection::{self, Decision},
    text_measure, FontMetrics, ImageSizer, Layouter,
};
use crate::entities::{
    corners::Corners,
    geometry::ShapeKind,
    layout_types::{FrameItem, Point, Pt},
};

pub(super) fn word<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: &str,
) {
    super::text_baseline::admit(layouter);
    if super::word_segment::word_needs_line_segmentation(text) {
        return super::word_segment::layout_segmented_word(layouter, text);
    }
    let width = text_measure::word(&layouter.metrics, text, &layouter.style);
    let hyphenate = match layouter.chain.custom("text.hyphenate") {
        Some(crate::entities::value::Value::Bool(value)) => *value,
        _ => matches!(
            layouter.chain.custom("par.justify"),
            Some(crate::entities::value::Value::Bool(true))
        ),
    };
    let mut boundary = layouter
        .text_tracking_state
        .boundary(&layouter.regions.current.current_line, &layouter.style);
    let decision = break_selection::word(
        &layouter.metrics,
        &layouter.style,
        text,
        width,
        layouter.regions.current.cursor_x.0 + boundary.0,
        layouter.regions.current.width - layouter.page_config.margin.right,
        layouter.regions.current.line_start_x.0,
        hyphenate,
    );
    match decision {
        Decision::Keep => {}
        Decision::Wrap => {
            boundary = Pt(0.0);
            layouter.flush_continued_line();
            super::text_baseline::admit(layouter);
        }
        Decision::Hyphenate { prefix, replacement, width, rest } => {
            layouter.regions.current.cursor_x += boundary;
            push_text(layouter, prefix.into(), width, Some(&replacement));
            layouter.regions.current.cursor_x += width;
            layouter.flush_continued_line();
            layouter.layout_word(&rest);
            return;
        }
    }
    layouter.regions.current.cursor_x += boundary;
    push_text(layouter, text.into(), width, None);
    layouter.regions.current.cursor_x += width;
}

pub(super) fn chunk<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: &str,
) {
    super::text_baseline::admit(layouter);
    let width = text_measure::word(&layouter.metrics, text, &layouter.style);
    let mut boundary = layouter
        .text_tracking_state
        .boundary(&layouter.regions.current.current_line, &layouter.style);
    if break_selection::should_wrap(
        width,
        layouter.regions.current.cursor_x.0 + boundary.0,
        layouter.regions.current.width - layouter.page_config.margin.right,
        layouter.regions.current.line_start_x.0,
    ) {
        boundary = Pt(0.0);
        layouter.flush_continued_line();
        super::text_baseline::admit(layouter);
    }
    layouter.regions.current.cursor_x += boundary;
    push_text(layouter, text.into(), width, None);
    layouter.regions.current.cursor_x += width;
}

fn push_text<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: ecow::EcoString,
    width: Pt,
    replacement: Option<&str>,
) {
    super::bibliography::emit_pending_prefix(layouter);
    if let Some(fill) = layouter.style.highlight.clone() {
        let (ascender, line_height) = layouter
            .metrics
            .vertical_metrics(layouter.style.size, &layouter.style);
        let size_pt = layouter.style.size.val();
        let extent_pt = layouter
            .style
            .highlight_extent
            .map(|e| e.resolve_pt(size_pt))
            .unwrap_or(0.0);
        let shape_kind = match layouter.style.highlight_radius {
            Some(r) if r.resolve_pt(size_pt) > 0.0 => super::resolve_shape_kind(
                &ShapeKind::RoundedRect { radii: Corners::uniform(r) },
                layouter.style.size,
            ),
            _ => ShapeKind::Rect,
        };
        layouter.regions.current.current_line.push(FrameItem::Shape {
            pos: Point {
                x: Pt(layouter.regions.current.cursor_x.0 - extent_pt),
                y: layouter.baseline_y() - ascender,
            },
            kind: shape_kind,
            width: width.0 + 2.0 * extent_pt,
            height: line_height.0,
            fill: Some(crate::entities::frame_paint::FramePaint::Solid(fill)),
            stroke: None,
            fill_rule: Default::default(),
            parent_bbox_at_emit: None,
        });
    }
    let visual = FrameItem::Text {
        pos: Point {
            x: layouter.regions.current.cursor_x,
            y: layouter.baseline_y(),
        },
        text,
        style: layouter.style.clone(),
        bidi: layouter.text_bidi.clone(),
    };
    let item = match replacement {
        Some(text) => {
            FrameItem::TextReplacement { text: text.into(), items: vec![visual] }
        }
        None => visual,
    };
    layouter.regions.current.current_line.push(item);
    layouter.text_tracking_state.record();
}

#[cfg(test)]
mod policy_tests {
    use super::*;
    use crate::entities::{content::Content, layout_types::TextStyle, value::Value};
    fn emitted_hyphen(policy: Option<Value>, justify: bool) -> bool {
        use comemo::Track;
        let intr = crate::entities::introspector::TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            crate::entities::image_sizer::NullImageSizer,
            12.0,
            (&intr as &dyn crate::entities::introspector::Introspector).track(),
        );
        l.page_config.width = 70.0;
        l.page_config.height = 400.0;
        l.page_config.margin = crate::entities::layout_types::PageMargins::uniform(0.0);
        l.page_config.margin_spec =
            crate::entities::layout_types::PageMarginSpec::uniform(0.0);
        l.regions.current.width = 70.0;
        l.regions.current.height = 400.0;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l.style = TextStyle::regular(Pt(12.0));
        l.style.lang = Some("pt".parse().unwrap());
        l.chain = l.chain.push_custom("par.justify", Value::Bool(justify));
        if let Some(value) = policy {
            l.chain = l.chain.push_custom("text.hyphenate", value);
        }
        l.layout_content(&Content::text("xx representação"));
        let doc = l.finish();
        let text = doc.pages.iter().map(|p| p.plain_text()).collect::<Vec<_>>().join(" ");
        assert_eq!(
            text.replace('\u{ad}', "").split_whitespace().collect::<String>(),
            "xxrepresentação"
        );
        text.contains('\u{ad}')
    }
    #[test]
    fn break_policy_absent_and_auto_follow_justify() {
        assert!(!emitted_hyphen(None, false));
        assert!(!emitted_hyphen(Some(Value::Auto), false));
        assert!(emitted_hyphen(None, true));
        assert!(emitted_hyphen(Some(Value::Auto), true));
    }
    #[test]
    fn break_policy_explicit_boolean_overrides_justify() {
        assert!(!emitted_hyphen(Some(Value::Bool(false)), true));
        assert!(emitted_hyphen(Some(Value::Bool(true)), false));
    }
}

/// Emit a measured fragment without choosing another break.
pub(super) fn prepared<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    text: &str,
    width: Pt,
    style: &crate::entities::layout_types::TextStyle,
    replacement: Option<&str>,
) {
    let previous = layouter.style.clone();
    layouter.style = style.clone();
    layouter.ensure_initial_baseline();
    push_text(layouter, text.into(), width, replacement);
    layouter.regions.current.cursor_x += width;
    layouter.style = previous;
}

#[cfg(test)]
mod identity_tests {
    use super::super::FixedMetrics;
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMarginSpec, PageMargins, TextStyle},
        value::Value,
    };
    use comemo::Track;

    fn selected_text(global: bool) -> String {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 70.0;
        l.page_config.height = 400.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
        l.regions.current.width = 70.0;
        l.regions.current.height = 400.0;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l.style = TextStyle::regular(Pt(12.0));
        l.style.lang = Some("pt".parse().unwrap());
        l.chain = l.chain.push_custom("text.hyphenate", Value::Bool(true));
        l.chain = l.chain.push_custom("par.linebreaks", Value::Str("simple".into()));
        if global {
            l.layout_content(&Content::text("xx representação"));
        } else {
            word(&mut l, "xx");
            l.regions.current.cursor_x += Pt(6.0);
            word(&mut l, "representação");
        }
        l.flush_line();
        chunk(&mut l, "literal-");
        let doc = l.finish();
        doc.pages.iter().map(|p| p.plain_text()).collect::<Vec<_>>().join(" ")
    }
    #[test]
    fn discretionary_identity_global_retains_shy_and_literal_hyphen() {
        let text = selected_text(true);
        assert!(text.contains('\u{ad}'), "{text:?}");
        assert_eq!(text.matches('-').count(), 1, "{text:?}");
        assert_eq!(
            text.replace('\u{ad}', "").split_whitespace().collect::<String>(),
            "xxrepresentaçãoliteral-"
        );
    }
    #[test]
    fn discretionary_identity_incremental_retains_shy_and_literal_hyphen() {
        let text = selected_text(false);
        assert!(text.contains('\u{ad}'), "{text:?}");
        assert_eq!(text.matches('-').count(), 1, "{text:?}");
        assert_eq!(
            text.replace('\u{ad}', "").split_whitespace().collect::<String>(),
            "xxrepresentaçãoliteral-"
        );
    }
}

#[cfg(test)]
mod tracking_boundary_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Length, PageMargins, TextSeparatorOrigin, TextStyle},
    };
    use comemo::Track;
    fn make(
        intr: &dyn Introspector,
    ) -> Layouter<'_, super::super::FixedMetrics, NullImageSizer> {
        let mut l =
            Layouter::new(super::super::FixedMetrics, NullImageSizer, 12.0, intr.track());
        l.style = TextStyle::regular(Pt(12.0));
        l.style.tracking = Some(Length::pt(0.2));
        l.page_config.margin = PageMargins::uniform(0.0);
        l.regions.current.width = 400.0;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l
    }
    #[test]
    fn tracked_fragment_boundary_affects_selection_and_resets_after_wrap() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr);
        let w = text_measure::word(&l.metrics, "aa", &l.style).0;
        l.regions.current.width = 2.0 * w + 0.1;
        word(&mut l, "aa");
        let first_y = l.baseline_y();
        word(&mut l, "aa");
        assert!(l.baseline_y().0 > first_y.0);
        let FrameItem::Text { pos, .. } = l.regions.current.current_line.last().unwrap()
        else {
            panic!()
        };
        assert_eq!(pos.x, Pt(0.0));
        assert!((l.regions.current.cursor_x.0 - w).abs() < 1e-10);
    }
    #[test]
    fn tracked_repeated_spaces_and_adjacent_fragments_count_pairs_once() {
        let intr = TagIntrospector::empty();
        for t in [0.2, -0.2, 0.0] {
            for n in 0..=3 {
                let mut l = make(&intr);
                l.style.tracking = Some(Length::pt(t));
                let w = text_measure::word(&l.metrics, "aa", &l.style).0;
                let space = text_measure::space(&l.metrics, &l.style).0;
                word(&mut l, "aa");
                for _ in 0..n {
                    super::super::space::advance(&mut l, TextSeparatorOrigin::Literal);
                }
                chunk(&mut l, "aa");
                let FrameItem::Text { pos, .. } =
                    l.regions.current.current_line.last().unwrap()
                else {
                    panic!()
                };
                assert!(
                    (pos.x.0 - (w + n as f64 * space + (n + 1) as f64 * t)).abs() < 1e-10
                );
                assert!(
                    (l.regions.current.cursor_x.0
                        - (2.0 * w + n as f64 * space + (n + 1) as f64 * t))
                        .abs()
                        < 1e-10
                );
            }
        }
    }
}
