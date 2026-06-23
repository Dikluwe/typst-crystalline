//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout/link.md
//! @prompt-hash 6adfa0ae
//! @layer L1
//! @updated 2026-06-23
//!
//! P422 — Layout de `Content::Link`. Renderiza o body e envolve os itens
//! resultantes em `FrameItem::Link`, preservando o URL como metadado.


use crate::entities::elements::link::LinkElem;
use crate::entities::layout_types::FrameItem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma hiperligação: renderiza o body e envolve os itens em
/// `FrameItem::Link`. Cor/sublinhado diferidos para `FrameItem::Decoration`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &LinkElem,
) {
    let items_before = layouter.regions.current.current_items.len();
    let line_before = layouter.regions.current.current_line.len();

    layouter.layout_content(&e.body);

    // Coleta os items produzidos pelo body (inline em current_line e, se
    // houver flush, em current_items).
    let new_line: Vec<FrameItem> =
        layouter.regions.current.current_line.drain(line_before..).collect();
    let new_items: Vec<FrameItem> =
        layouter.regions.current.current_items.drain(items_before..).collect();

    let mut link_items = Vec::with_capacity(new_line.len() + new_items.len());
    link_items.extend(new_line);
    link_items.extend(new_items);

    layouter.regions.current.current_line.push(FrameItem::Link {
        url: e.url.clone(),
        items: link_items,
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::{FrameItem, Pt, TextStyle};
    use crate::entities::elements::link::LinkElem;
    use crate::rules::layout::{FixedMetrics, Layouter};
    use comemo::Track;
    use std::sync::Arc;

    #[test]
    fn p422_frame_item_link_construcao() {
        let item = FrameItem::Link {
            url: "https://x".into(),
            items: vec![FrameItem::Text {
                pos: crate::entities::layout_types::Point { x: Pt(0.0), y: Pt(0.0) },
                text: "x".into(),
                style: TextStyle::default(),
            }],
        };
        if let FrameItem::Link { url, .. } = item {
            assert_eq!(url.as_str(), "https://x");
        } else {
            panic!("esperado Link");
        }
    }

    #[test]
    fn p422_layout_link_body_texto() {
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_dyn.track());
        let elem = LinkElem {
            url: "https://example.com".into(),
            body: Content::text("Clique"),
        };
        layout(&mut layouter, &elem);
        // Após layout inline, o FrameItem::Link deve estar em current_line.
        assert_eq!(layouter.regions.current.current_line.len(), 1);
        assert!(
            matches!(layouter.regions.current.current_line[0], FrameItem::Link { .. }),
            "esperado FrameItem::Link"
        );
    }

    #[test]
    fn p422_layout_link_body_formatado() {
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_dyn.track());
        let elem = LinkElem {
            url: "https://example.com".into(),
            body: Content::Sequence(Arc::from(vec![
                Content::text("A "),
                Content::text("B"),
            ])),
        };
        layout(&mut layouter, &elem);
        assert_eq!(layouter.regions.current.current_line.len(), 1);
        if let FrameItem::Link { items, .. } = &layouter.regions.current.current_line[0] {
            assert!(!items.is_empty(), "body formatado deve produzir items");
        } else {
            panic!("esperado FrameItem::Link");
        }
    }
}
