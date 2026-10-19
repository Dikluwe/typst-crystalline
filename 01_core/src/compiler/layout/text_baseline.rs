//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/text_baseline.md
//! @prompt-hash 085b8d47
//! @layer L1
//! @updated 2026-09-29

use super::{FontMetrics, ImageSizer, Layouter};
use crate::entities::layout_types::{FrameItem, Pt};

/// Leading literal separators carry pairs but do not themselves demand a baseline.
pub(super) fn admit<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>) {
    let only_separators = !layouter.regions.current.current_line.is_empty()
        && !super::html_frame::items_have_material(
            &layouter.regions.current.current_line,
        );
    layouter.ensure_text_baseline();
    super::paragraph_indent::admit(layouter);
    if only_separators {
        for item in &mut layouter.regions.current.current_line {
            if let FrameItem::TextSeparator { pos, style, .. } = item {
                pos.y = layouter.regions.current.cursor_y
                    + Pt(style.baseline_offset.resolve_pt(style.size.0));
            }
        }
    }
}

/// Fecha uma linha de espaços literais com o estilo que produziu os carriers,
/// mesmo após o escopo textual restaurar o estilo exterior.
pub(super) fn admit_literal_line<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) {
    let style = layouter
        .regions
        .current
        .current_line
        .iter()
        .filter_map(|item| match item {
            FrameItem::TextSeparator {
                style,
                origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                ..
            } => Some(style),
            _ => None,
        })
        .max_by(|a, b| a.size.0.total_cmp(&b.size.0))
        .cloned();
    if let Some(style) = style {
        let outer = std::mem::replace(&mut layouter.style, style);
        admit(layouter);
        layouter.style = outer;
    }
}

#[cfg(test)]
mod leading_tracking_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Length, TextSeparatorOrigin},
    };
    use comemo::Track;
    #[test]
    fn tracked_literal_leading_spaces_keep_pairs_without_vertical_demand() {
        let intr = TagIntrospector::empty();
        for t in [0.2, -0.2] {
            let mut l = Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                12.0,
                (&intr as &dyn Introspector).track(),
            );
            l.style.tracking = Some(Length::pt(t));
            let x = l.regions.current.cursor_x.0;
            let w = super::super::text_measure::space(&l.metrics, &l.style).0;
            let pending = l.initial_baseline_pending;
            super::super::space::advance(&mut l, TextSeparatorOrigin::Literal);
            super::super::space::advance(&mut l, TextSeparatorOrigin::Literal);
            assert_eq!(l.initial_baseline_pending, pending);
            assert!((l.regions.current.cursor_x.0 - (x + 2.0 * w + t)).abs() < 1e-10);
            super::super::word_layout::chunk(&mut l, "abc");
            let FrameItem::Text { pos, .. } =
                l.regions.current.current_line.last().unwrap()
            else {
                panic!()
            };
            let text_pos = *pos;
            assert!((text_pos.x.0 - (x + 2.0 * w + 2.0 * t)).abs() < 1e-10);
            for item in &l.regions.current.current_line {
                if let FrameItem::TextSeparator { pos, .. } = item {
                    assert_eq!(pos.y, text_pos.y);
                }
            }
        }
    }
}
