//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/context_block.md
//! @prompt-hash 5b7f80aa
//! @layer L1
//! @updated 2026-09-26

use super::{html_frame, FontMetrics, ImageSizer, Layouter};
use crate::entities::elements::context_block::ContextBlockElem;
use crate::entities::layout_types::Point;

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    elem: &ContextBlockElem,
) {
    if layouter.capture_html_context_markers {
        // Sub-frame callers already rebase their physical coordinates. A
        // private marker follows that same transform, without a second ascent.
        let origin_y = if layouter.is_sub_frame {
            0.0
        } else {
            layouter
                .metrics
                .vertical_metrics(layouter.style.size, &layouter.style)
                .0
                 .0
        };
        let point = Point {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y
                - crate::entities::layout_types::Pt(origin_y),
        };
        layouter
            .regions
            .current
            .current_line
            .push(html_frame::context_marker(elem.id, point));
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::layout::{FixedMetrics, FontMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Point},
    };
    use comemo::Track;

    fn make(intr: &dyn Introspector) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        Layouter::new(FixedMetrics, NullImageSizer, 11.0, intr.track())
    }

    #[test]
    fn atomized_context_marker_is_opt_in_and_never_runs_closure() {
        use crate::entities::{elements::context_block::ContextBlockElem, func::Func};
        let intr = TagIntrospector::empty();
        let elem = ContextBlockElem {
            id: 42,
            closure: Func::native("must-not-run", |_, _, _, _| {
                panic!("layout executed closure")
            }),
        };
        for capture in [false, true] {
            let mut layouter = make(&intr);
            layouter.capture_html_context_markers = capture;
            let x = layouter.regions.current.cursor_x;
            let y = layouter.regions.current.cursor_y;
            let (ascender, _) = layouter
                .metrics
                .vertical_metrics(layouter.style.size, &layouter.style);
            layouter.layout_content(&Content::ContextBlock(std::sync::Arc::new(
                elem.clone(),
            )));
            assert_eq!(layouter.regions.current.cursor_x, x);
            assert_eq!(layouter.regions.current.cursor_y, y);
            let html_markers: Vec<_> = layouter.regions.current.current_line.iter().filter(|item| {
                matches!(item, FrameItem::Semantic { alt: Some(alt), .. } if alt.starts_with("\0p1493-context:"))
            }).collect();
            assert_eq!(html_markers.len(), usize::from(capture));
            assert!(!super::super::html_frame::items_have_material(
                &layouter.regions.current.current_line
            ));
            if capture {
                match html_markers[0] {
                    FrameItem::Semantic { alt, items, .. } => {
                        assert_eq!(alt.as_deref(), Some("\0p1493-context:42"));
                        assert_eq!(items.len(), 1);
                        match &items[0] {
                            FrameItem::Shape { pos, width, height, .. } => {
                                assert_eq!(*pos, Point { x, y: y - ascender });
                                assert_eq!((*width, *height), (0.0, 0.0));
                            }
                            _ => panic!("expected zero-size marker"),
                        }
                    }
                    _ => panic!("expected semantic carrier"),
                }
                assert!(!super::super::html_frame::items_have_material(
                    &layouter.regions.current.current_line
                ));
            }
        }
    }
}
