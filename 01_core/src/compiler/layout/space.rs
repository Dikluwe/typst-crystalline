//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/space.md
//! @prompt-hash b80d5a47
//! @layer L1
//! @updated 2026-09-29

use super::{html_frame, FontMetrics, ImageSizer, Layouter};

use crate::entities::layout_types::{FrameItem, Point, TextSeparatorOrigin};

// O avanço é aplicado uma única vez. O carrier transporta sua proveniência.
pub(super) fn advance<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    origin: TextSeparatorOrigin,
) {
    let style = if origin == TextSeparatorOrigin::Literal {
        layouter.style.clone()
    } else {
        super::text::effective_style(layouter)
    };
    let width = super::text_measure::space(&layouter.metrics, &style);
    if html_frame::items_have_material(&layouter.regions.current.current_line)
        || origin == TextSeparatorOrigin::Literal
    {
        layouter.regions.current.cursor_x += layouter.text_tracking_state.boundary(
            &layouter.regions.current.current_line, &style,
        );
        let y = layouter.regions.current.cursor_y + crate::entities::layout_types::Pt(style.baseline_offset.resolve_pt(style.size.val()));
        layouter.regions.current.current_line.push(FrameItem::TextSeparator {
            pos: Point { x: layouter.regions.current.cursor_x, y },
            advance: width,
            style,
            origin,
        });
        layouter.text_tracking_state.record();
    }
    layouter.regions.current.cursor_x += width;
    layouter.justify_opportunities.push(layouter.regions.current.cursor_x.0);
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    origin: TextSeparatorOrigin,
) {
    // Não criar linha nem baseline por espaços iniciais de markup.
    if html_frame::items_have_material(&layouter.regions.current.current_line) {
        advance(layouter, origin);
        if !matches!(
            layouter.chain.custom("par.justify"),
            Some(crate::entities::value::Value::Bool(true))
        ) && layouter.regions.current.cursor_x.0
            > layouter.regions.current.width - layouter.page_config.margin.right
        {
            layouter.flush_line();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{Point, Pt},
    };
    use comemo::Track;

    fn make(intr: &dyn Introspector) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        Layouter::new(FixedMetrics, NullImageSizer, 11.0, intr.track())
    }

    #[test]
    fn atomized_spaces_suppress_initial_and_marker_only_advance() {
        let intr = TagIntrospector::empty();
        for space in [Content::Space, Content::SpaceFromNewline] {
            let mut layouter = make(&intr);
            let origin = layouter.regions.current.cursor_x;
            layouter.layout_content(&space);
            assert_eq!(layouter.regions.current.cursor_x, origin);
            layouter.regions.current.current_line.push(
                super::super::html_frame::context_marker(
                    7,
                    Point { x: origin, y: Pt(0.0) },
                ),
            );
            layouter.layout_content(&space);
            assert_eq!(layouter.regions.current.cursor_x, origin);
            assert!(layouter.justify_opportunities.is_empty());
        }
    }

    #[test]
    fn atomized_spaces_advance_and_wrap_after_material() {
        let intr = TagIntrospector::empty();
        for space in [Content::Space, Content::SpaceFromNewline] {
            let mut layouter = make(&intr);
            layouter.layout_content(&Content::text("A"));
            let before = layouter.regions.current.cursor_x;
            let width = layouter.space_width();
            layouter.layout_content(&space);
            assert_eq!(layouter.regions.current.cursor_x, before + width);
            assert_eq!(layouter.justify_opportunities, vec![(before + width).0]);
            layouter.regions.current.width =
                layouter.regions.current.cursor_x.0 + layouter.page_config.margin.right;
            layouter.layout_content(&space);
            assert!(!super::super::html_frame::items_have_material(
                &layouter.regions.current.current_line
            ));
            assert_eq!(layouter.finish().plain_text(), "A");
        }
    }
}

#[cfg(test)]
mod separator_contract_tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{content::Content, image_sizer::NullImageSizer, introspector::{TagIntrospector, Introspector}, layout_types::{FrameItem, TextSeparatorOrigin}};
    use comemo::Track;
    #[test]
    fn lexical_separator_contract_retains_origins_without_double_advance() {
        for (content, origin) in [(Content::Space, TextSeparatorOrigin::Markup), (Content::SpaceFromNewline, TextSeparatorOrigin::Newline), (Content::text(" "), TextSeparatorOrigin::Literal)] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(FixedMetrics, NullImageSizer, 11.0, (&intr as &dyn Introspector).track());
            l.layout_content(&Content::text("A"));
            let x = l.regions.current.cursor_x;
            let width = l.space_width();
            let y = l.baseline_y();
            let prefix = format!("{:?}", l.regions.current.current_line);
            l.layout_content(&content);
            assert_eq!(l.regions.current.cursor_x, x + width);
            let Some(FrameItem::TextSeparator { pos, advance, origin: actual, .. }) = l.regions.current.current_line.last() else { panic!("lexical separator carrier missing") };
            assert_eq!((*advance, *actual), (width, origin));
            assert_eq!((pos.x, pos.y), (x, y));
            assert_eq!(format!("{:?}", &l.regions.current.current_line[..l.regions.current.current_line.len()-1]), prefix);
        }
    }
    #[test]
    fn lexical_separator_contract_tail_rollback_restores_certified_prefix() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 11.0, (&intr as &dyn Introspector).track());
        l.layout_content(&Content::text("A"));
        let saved = super::super::RegionTailCheckpoint::capture(&l.regions.current);
        let before = format!("{:?}", l.regions.current.current_line);
        l.layout_content(&Content::Space);
        assert!(matches!(l.regions.current.current_line.last(), Some(FrameItem::TextSeparator { .. })));
        saved.restore(&mut l.regions.current);
        assert_eq!(format!("{:?}", l.regions.current.current_line), before);
    }
    #[test]
    fn lexical_separator_contract_does_not_infer_space_between_adjacent_text() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 11.0, (&intr as &dyn Introspector).track());
        l.layout_content(&Content::text("A"));
        l.layout_content(&Content::text("B"));
        assert!(!l.regions.current.current_line.iter().any(|i| matches!(i, FrameItem::TextSeparator { .. })));
    }
}
