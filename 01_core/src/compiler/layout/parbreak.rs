//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/parbreak.md
//! @prompt-hash fe25bb58
//! @layer L1
//! @updated 2026-10-06
//!
//! Materialização transparente da fronteira de parágrafo (P1140.13).

use crate::entities::layout_types::{
    FrameItem, Point, Pt, SemanticKind, SemanticPlacement,
};

use super::{html_frame, FontMetrics, ImageSizer, Layouter};

pub(super) fn mark_boundary<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) {
    let Some((x, y)) = layouter
        .regions
        .current
        .current_line
        .first()
        .map(super::helpers::item_pos)
    else {
        return;
    };

    layouter.regions.current.current_line.push(FrameItem::Semantic {
        kind: SemanticKind::ParbreakBoundary,
        placement: SemanticPlacement::Block,
        alt: None,
        items: vec![FrameItem::Text {
            pos: Point { x: Pt(x), y: Pt(y) },
            text: "".into(),
            style: layouter.style.clone(),
            bidi: None,
        }],
    });
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(layouter: &mut Layouter<M, S>) {
    layouter.smartquote_context.reset_paragraph();
    let had_items =
        html_frame::items_have_causal_line(&layouter.regions.current.current_line);
    if had_items {
        if !html_frame::items_have_material(&layouter.regions.current.current_line) {
            super::text_baseline::admit_literal_line(layouter);
        }
        mark_boundary(layouter);
        layouter.flush_line();
        let flow_end = layouter.flow_placement_y();
        if let Some(end) = flow_end {
            // The next paragraph's weak-margin branch starts from the actual
            // settled line bottom, including an inline object's descent.
            layouter.prev_block_equation_descent =
                (end - layouter.prev_line_baseline).max(0.0);
        }
        // O espaçamento que prepara o próximo parágrafo não estende uma
        // página auto se nenhum conteúdo vier depois desta fronteira.
        layouter.last_block_descent_y = Some(
            layouter
                .last_block_descent_y
                .unwrap_or(layouter.prev_line_baseline)
                .max(layouter.prev_line_baseline),
        );
        let font_size = layouter.style.size.val();
        use super::vanilla_defaults::{PAR_LEADING, PAR_SPACING};
        use crate::entities::value::Value;
        let spacing_pt = match layouter.chain.custom("par.spacing") {
            Some(Value::Length(l)) => l.resolve_pt(font_size),
            // PAR_SPACING, ver vanilla_defaults.rs
            _ => font_size * PAR_SPACING,
        };
        let leading_pt = match layouter.chain.custom("par.leading") {
            Some(Value::Length(l)) => l.resolve_pt(font_size),
            // PAR_LEADING, ver vanilla_defaults.rs
            _ => layouter
                .style
                .leading
                .map(|l| l.resolve_pt(font_size))
                .unwrap_or(font_size * PAR_LEADING),
        };
        let extra_spacing = (spacing_pt - leading_pt).max(0.0);
        let gap = if layouter.block_chain_active {
            layouter.prev_block_below_pending.max(extra_spacing)
        } else {
            extra_spacing
        };
        let advance = (gap - layouter.prev_block_below_pending).max(0.0);
        layouter.regions.current.cursor_y += Pt(advance);
        layouter.prev_block_below_pending = if layouter.block_chain_active {
            layouter.prev_block_below_pending.max(spacing_pt)
        } else {
            spacing_pt
        };
        layouter.block_chain_active = true;
        layouter.prev_margin_is_parbreak = true;
        if let Some(end) = flow_end {
            layouter.record_flow_placement_end(end);
        }
    }
    super::paragraph_indent::end(layouter);
}

#[cfg(test)]
mod tests {
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Point, Pt},
    };
    use comemo::Track;

    fn make(intr: &dyn Introspector) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        Layouter::new(FixedMetrics, NullImageSizer, 11.0, intr.track())
    }

    #[test]
    fn atomized_parbreak_ignores_empty_and_marker_only_lines() {
        let intr = TagIntrospector::empty();
        let mut layouter = make(&intr);
        let y = layouter.regions.current.cursor_y;
        layouter.layout_content(&Content::Parbreak);
        layouter
            .regions
            .current
            .current_line
            .push(super::super::html_frame::context_marker(7, Point { x: Pt(0.0), y }));
        layouter.layout_content(&Content::Parbreak);
        assert_eq!(layouter.regions.current.cursor_y, y);
        assert!(!layouter.block_chain_active);
        assert!(!layouter.prev_margin_is_parbreak);
    }

    #[test]
    fn atomized_parbreak_marks_once_and_collapses_pending_margin() {
        use crate::entities::layout_types::SemanticKind;
        let intr = TagIntrospector::empty();
        let mut layouter = make(&intr);
        layouter.layout_content(&Content::text("Before"));
        layouter.block_chain_active = true;
        layouter.prev_block_below_pending = 40.0;
        layouter.layout_content(&Content::Parbreak);
        assert_eq!(layouter.prev_block_below_pending, 40.0);
        assert!(layouter.prev_margin_is_parbreak);
        layouter.layout_content(&Content::Parbreak);
        layouter.layout_content(&Content::text("After"));
        let doc = layouter.finish();
        assert!(doc.plain_text().contains("Before"));
        assert!(doc.plain_text().contains("After"));
        assert_eq!(
            doc.pages
                .iter()
                .flat_map(|p| &p.items)
                .filter(|i| matches!(
                    i,
                    FrameItem::Semantic { kind: SemanticKind::ParbreakBoundary, .. }
                ))
                .count(),
            1
        );
    }
}

#[cfg(test)]
mod causal_empty_lines_tests {
    use super::super::{FixedMetrics, FontMetrics, Layouter};
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{PageMargins, Pt},
    };
    use comemo::Track;

    fn make(intr: &dyn Introspector) -> Layouter<'_, FixedMetrics, NullImageSizer> {
        let mut l = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr.track());
        l.page_config.width = f64::INFINITY;
        l.page_config.height = f64::INFINITY;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.regions.current.width = f64::INFINITY;
        l.regions.current.cursor_x = Pt(0.0);
        l.regions.current.line_start_x = Pt(0.0);
        l.regions.current.cursor_y = Pt(0.0);
        l
    }

    #[test]
    fn terminal_parbreak_does_not_extend_auto_page_after_text() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr);
        let top = l.metrics.text_edges(l.style.size, &l.style).0 .0;
        l.layout_content(&Content::sequence(vec![
            Content::text("abc"),
            Content::Parbreak,
        ]));
        let page = &l.finish().pages[0];
        assert!((page.height - top).abs() < 1e-9);
    }

    #[test]
    fn literal_only_paragraph_closes_before_next_literal_line() {
        let intr = TagIntrospector::empty();
        let mut l = make(&intr);
        let width = l.metrics.advance(" ", l.style.size, &l.style).0;
        l.layout_content(&Content::sequence(vec![
            Content::text("  "),
            Content::Parbreak,
            Content::text(" "),
        ]));
        let size = l.style.size.0;
        let page = &l.finish().pages[0];
        assert!((page.width - 2.0 * width).abs() < 1e-9);
        assert!(page.height > 1.5 * size);
    }
}
