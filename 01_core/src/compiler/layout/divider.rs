//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/divider.md
//! @prompt-hash ca21d39a
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Divider` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::divider::DividerElem;
use crate::entities::frame_paint::{FramePaint as Paint, FrameStroke as Stroke};
use crate::entities::geometry::ShapeKind;
use crate::entities::layout_types::{Color, FrameItem, Point, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `divider` (`---`): emite uma linha horizontal de margem a margem
/// e avança o cursor vertical. `_e` ignorado (singleton estrutural).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    _e: &DividerElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin.left {
        layouter.flush_line();
    }
    let margin = layouter.page_config.margin.left;
    // BLOCK_SPACING, ver vanilla_defaults.rs
    let spacing = Pt(layouter.style.size.val() * super::vanilla_defaults::BLOCK_SPACING);
    layouter.regions.current.cursor_y += spacing;
    // ref: lab/typst-original/crates/typst-library/src/visualize/line.rs:20
    let width_pt = if layouter.regions.current.width.is_finite() {
        layouter.regions.current.width - 2.0 * margin
    } else {
        // The rule fills the completed region; it cannot supply its intrinsic width.
        layouter
            .pending_dividers
            .push((layouter.regions.current.current_items.len(), margin));
        0.0
    };
    layouter.regions.current.current_items.push(FrameItem::Shape {
        pos: Point {
            x: Pt(margin),
            y: layouter.regions.current.cursor_y,
        },
        kind: ShapeKind::Line { dx: width_pt, dy: 0.0 },
        width: width_pt,
        height: 0.5,
        fill: None,
        stroke: Some(Stroke {
            paint: Paint::Solid(Color::rgb(0, 0, 0)),
            thickness: 0.5,
            overhang: false,
            ..Stroke::default()
        }),
        fill_rule: Default::default(),
        parent_bbox_at_emit: None,
    });
    layouter.regions.current.cursor_y += spacing;
}

/// Settle only certified divider occurrences after their region width is known.
pub(super) fn resolve<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    final_width: f64,
) {
    resolve_from(layouter, 0, final_width);
}

/// A composed child settles only its own suffix before grouping its items.
pub(super) fn resolve_from<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    first_item: usize,
    final_width: f64,
) {
    let items = &mut layouter.regions.current.current_items;
    layouter.pending_dividers.retain(|&(index, margin)| {
        if index < first_item {
            return true;
        }
        let FrameItem::Shape { kind: ShapeKind::Line { dx, .. }, width, .. } =
            &mut items[index]
        else {
            unreachable!("pending divider must retain its owning region item");
        };
        *width = (final_width - 2.0 * margin).max(0.0);
        *dx = *width;
        false
    });
}

#[cfg(test)]
mod auto_width_tests {
    use super::*;
    use crate::entities::{
        content::{Content, SetPageDelta},
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageDimension, PageMarginSpec, PagedDocument},
    };
    use comemo::Track;
    use std::sync::Arc;

    fn document(body: Content, width: PageDimension) -> PagedDocument {
        super::super::layout(&Content::sequence(vec![
            Content::SetPage(Arc::new(SetPageDelta {
                width: Some(width),
                margin: Some(PageMarginSpec::uniform(10.0)),
                ..SetPageDelta::default()
            })),
            body,
        ]))
    }

    fn rule_widths(doc: &PagedDocument) -> Vec<f64> {
        super::super::tests::frame_items_recursive(&doc.pages[0].items)
            .iter()
            .filter_map(|item| match item {
                FrameItem::Shape { kind: ShapeKind::Line { dx, .. }, width, .. } => {
                    assert_eq!(*dx, *width);
                    Some(*width)
                }
                _ => None,
            })
            .collect()
    }

    #[test]
    fn auto_width_divider_uses_complete_body_extent() {
        let doc = document(
            Content::sequence(vec![
                Content::text("SHORT"),
                Content::divider(),
                Content::text("A MUCH LONGER SECOND CONTENT"),
                Content::divider(),
                Content::text("TAIL"),
            ]),
            PageDimension::Auto,
        );
        let control =
            document(Content::text("A MUCH LONGER SECOND CONTENT"), PageDimension::Auto);
        assert!(doc.pages[0].width.is_finite(), "width={}", doc.pages[0].width);
        assert!((doc.pages[0].width - control.pages[0].width).abs() < 1e-8);
        let widths = rule_widths(&doc);
        assert_eq!(widths.len(), 2);
        assert!(widths
            .iter()
            .all(|w| *w > 0.0 && (*w - (doc.pages[0].width - 20.0)).abs() < 1e-8));
        let text = doc.pages[0].plain_text();
        assert!(
            text.contains("SHORT") && text.contains("SECOND") && text.contains("TAIL"),
            "{text}"
        );
    }

    #[test]
    fn auto_width_divider_resolves_each_committed_page() {
        let doc = document(
            Content::sequence(vec![
                Content::text("FIRST"),
                Content::divider(),
                Content::pagebreak(false, None),
                Content::text("LONGER SECOND"),
                Content::divider(),
            ]),
            PageDimension::Auto,
        );
        assert_eq!(doc.pages.len(), 2);
        for page in &doc.pages {
            assert!(page.width.is_finite());
            for item in &page.items {
                if let FrameItem::Shape { kind: ShapeKind::Line { dx, .. }, .. } = item {
                    assert!(*dx > 0.0 && (*dx - (page.width - 20.0)).abs() < 1e-8);
                }
            }
        }
    }

    #[test]
    fn fixed_width_divider_keeps_original_extent() {
        let doc = document(
            Content::sequence(vec![
                Content::text("A"),
                Content::divider(),
                Content::text("B"),
            ]),
            PageDimension::Length(200.0),
        );
        assert_eq!(doc.pages[0].width, 200.0);
        assert_eq!(rule_widths(&doc), vec![180.0]);
    }

    #[test]
    fn auto_width_divider_isolates_child_region() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = f64::INFINITY;
        l.regions.current.width = f64::INFINITY;
        l.layout_content(&Content::text("PARENT WIDER THAN CHILD"));
        l.layout_content(&Content::divider());
        let (_, items, _, _, _) = l.layout_sub_frame(
            &Content::sequence(vec![Content::text("CHILD"), Content::divider()]),
            super::super::SubLayoutRegion {
                origin_x: l.page_config.margin.left,
                width: f64::INFINITY,
                height: None,
                align_rtl: false,
                unconstrained_height: true,
            },
        );
        let child_width = items
            .iter()
            .find_map(|item| match item {
                FrameItem::Shape { kind: ShapeKind::Line { dx, .. }, .. } => Some(*dx),
                _ => None,
            })
            .unwrap();
        assert!(child_width.is_finite() && child_width > 0.0);
        let doc = l.finish();
        assert!(doc.pages[0].width.is_finite());
        assert!(rule_widths(&doc)[0] > child_width);
    }

    #[test]
    fn auto_width_divider_discards_hidden_child_requests() {
        let doc = document(
            Content::sequence(vec![
                Content::text("VISIBLE PREFIX"),
                Content::divider(),
                Content::Hide(Arc::new(crate::entities::elements::hide::HideElem {
                    body: Content::sequence(vec![
                        Content::text("HIDDEN"),
                        Content::divider(),
                    ]),
                })),
                Content::text("VISIBLE SUFFIX"),
            ]),
            PageDimension::Auto,
        );
        assert!(doc.pages[0].width.is_finite());
        assert_eq!(rule_widths(&doc).len(), 1);
        assert!(rule_widths(&doc)[0] > 0.0);
        assert!(!doc.pages[0].plain_text().contains("HIDDEN"));
    }

    #[test]
    fn auto_width_divider_survives_rejected_suffix() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = f64::INFINITY;
        l.regions.current.width = f64::INFINITY;
        l.layout_content(&Content::text("PREFIX"));
        l.layout_content(&Content::divider());
        let checkpoint = l.capture_flow_suffix_checkpoint();
        l.layout_content(&Content::divider());
        l.restore_flow_suffix_checkpoint(checkpoint);
        l.layout_content(&Content::text("SUFFIX"));
        let doc = l.finish();
        assert!(doc.pages[0].width.is_finite());
        assert_eq!(rule_widths(&doc).len(), 1);
        assert!(rule_widths(&doc)[0] > 0.0);
    }
}
