//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/term_item.md
//! @prompt-hash ebd7d4c1
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `TermItem` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::content::Content;
use crate::entities::elements::h_space::{HSpaceElem, Spacing};
use crate::entities::elements::term_item::TermItemElem;
use crate::entities::layout_types::{Length, TextStyle};
use crate::entities::style::{Style, Styles};
use std::sync::Arc;

use super::{FontMetrics, ImageSizer, Layouter};

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TermItemElem,
) {
    if let Some(frame) = layouter.list_frame.take() {
        let hanging = frame.hanging;
        let separator = frame.separator.clone();
        super::terms::item_body(layouter, frame, &e.description, |layouter, _| {
            if hanging != 0.0 {
                layouter.layout_content(&Content::HSpace(Arc::new(HSpaceElem {
                    amount: Spacing::Absolute(Length::pt(-hanging)),
                    weak: false,
                    weak_explicit: true,
                })));
            }
            pair(layouter, e, &separator);
        });
        return;
    }
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }
    layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
    pair(layouter, e, &crate::compiler::stdlib::list_like::separator());
    layouter.flush_line();
    layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
}

/// Termo forte, separador e descrição, na linha corrente.
fn pair<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TermItemElem,
    separator: &Content,
) {
    let chain = layouter.chain.clone();
    let style = layouter.style.clone();
    layouter.chain = layouter.chain.push_styles(&Styles::from_iter([Style::bold(true)]));
    layouter.style = TextStyle::from(&layouter.chain);
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::TermItemTerm,
        &e.term,
    );
    layouter.chain = chain;
    layouter.style = style;
    layouter.layout_content(separator);
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::TermItemDescription,
        &e.description,
    );
}

#[cfg(test)]
mod tests {
    use crate::compiler::layout::{layout, tests::frame_items_recursive};
    use crate::entities::content::Content;
    use crate::entities::layout_types::FrameItem;

    fn texts(content: &Content) -> Vec<String> {
        frame_items_recursive(&layout(content).pages[0].items)
            .into_iter()
            .filter_map(|item| match item {
                FrameItem::Text { text, .. } => Some(text.to_string()),
                FrameItem::TextReplacement { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Group { .. }
                | FrameItem::Link { .. } => None,
            })
            .collect()
    }

    #[test]
    fn term_precedes_its_description() {
        let item = Content::term_item(Content::text("a"), Content::text("b"));
        assert_eq!(texts(&item), ["a", "b"]);
    }
}
