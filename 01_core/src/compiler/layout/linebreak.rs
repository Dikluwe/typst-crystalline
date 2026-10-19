//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/linebreak.md
//! @prompt-hash 69633b38
//! @layer L1
//! @updated 2026-10-06
//!
//! Consumer atomizado de `LinebreakElem` (ADR-0109, forma B).

use crate::entities::elements::linebreak::LinebreakElem;
use crate::entities::layout_types::{
    FrameItem, Point, Pt, SemanticKind, SemanticPlacement,
};

use super::{FontMetrics, ImageSizer, Layouter};

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &LinebreakElem,
) {
    layouter.smartquote_context.observe_text("\n");
    if let Some(cursor) = layouter.bidi_cursor.as_mut() {
        cursor.next();
    }
    if !super::html_frame::items_have_causal_line(&layouter.regions.current.current_line)
    {
        super::text_baseline::admit(layouter);
        layouter.regions.current.current_line.push(FrameItem::Text {
            pos: Point {
                x: layouter.regions.current.cursor_x,
                y: layouter.baseline_y(),
            },
            text: "".into(),
            style: layouter.style.clone(),
            bidi: None,
        });
    }
    if let Some((x, y)) = layouter
        .regions
        .current
        .current_line
        .iter()
        .find(|item| !super::html_frame::is_context_marker(item))
        .map(super::helpers::item_pos)
    {
        // P1140.12 — conserva a causa do flush para o reflow bidi. O filho
        // vazio fornece baseline sem produzir texto ou desenho.
        layouter.regions.current.current_line.push(FrameItem::Semantic {
            kind: SemanticKind::ExplicitLinebreakBoundary,
            placement: SemanticPlacement::Inline,
            alt: None,
            items: vec![FrameItem::Text {
                pos: Point { x: Pt(x), y: Pt(y) },
                text: "".into(),
                style: layouter.style.clone(),
                bidi: None,
            }],
        });
    }
    if elem.justify {
        layouter.justify_current_line();
    }
    layouter.flush_line();
}

#[cfg(test)]
mod empty_line_tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::FrameItem,
    };
    use comemo::Track;
    fn br() -> Content {
        Content::Linebreak(std::sync::Arc::new(super::LinebreakElem {
            justify: false,
            justify_explicit: false,
        }))
    }
    #[test]
    fn authored_empty_lines_reserve_the_same_advance_as_material_lines() {
        let intr = TagIntrospector::empty();
        let make = || {
            Layouter::new(
                FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            )
        };
        let ys = |d: crate::entities::layout_types::PagedDocument| {
            d.pages[0]
                .items
                .iter()
                .filter_map(|it| {
                    if let FrameItem::Text { pos, text, .. } = it {
                        (!text.is_empty()).then_some(pos.y.0)
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
        };
        let mut normal = make();
        normal.layout_content(&Content::sequence(vec![
            Content::text("A"),
            br(),
            Content::text("B"),
        ]));
        let y = ys(normal.finish());
        let advance = y[1] - y[0];
        for count in [1, 2] {
            let mut l = make();
            for _ in 0..count {
                l.layout_content(&br());
            }
            l.layout_content(&Content::text("B"));
            let actual = ys(l.finish());
            assert_eq!(actual.len(), 1);
            assert!(
                (actual[0] - (y[0] + count as f64 * advance)).abs() < 1e-6,
                "count={count} actual={actual:?} material={y:?}"
            );
        }
    }
}
