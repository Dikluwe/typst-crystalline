//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout/link.md
//! @prompt-hash f34dd97a
//! @layer L1
//! @updated 2026-06-23
//!
//! P422 — Layout de `Content::Link`. Renderiza o body e envolve os itens
//! resultantes em `FrameItem::Link`, preservando o URL como metadado.


use crate::entities::elements::link::LinkElem;
use crate::entities::layout_types::{FrameItem, Point, Pt, Size};

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

    let (pos, size) = link_bbox(&link_items, &layouter.metrics);

    layouter.regions.current.current_line.push(FrameItem::Link {
        target: crate::entities::layout_types::LinkTarget::Url(e.url.clone()),
        items: link_items,
        pos,
        size,
    });
}

/// Calcula a bounding-box acumulada dos `items` para preencher `pos`/`size`
/// do `FrameItem::Link`.  Para texto mede o avanço via `FontMetrics`;
/// para grupos/links aninhados usa os campos já existentes.
pub(super) fn link_bbox<M: FontMetrics>(items: &[FrameItem], metrics: &M) -> (Point, Size) {
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    let mut expand = |x: f64, y: f64, w: f64, h: f64| {
        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x + w);
        max_y = max_y.max(y + h);
    };

    for item in items {
        match item {
            FrameItem::Text { pos, text, style } => {
                let w = metrics.advance(text.as_str(), style.size).0;
                let (_, line_h) = metrics.vertical_metrics(style.size);
                expand(pos.x.0, pos.y.0, w, line_h.0);
            }
            FrameItem::Glyph { pos, x_advance, size, .. } => {
                expand(pos.x.0, pos.y.0, x_advance.0, size.0);
            }
            FrameItem::Shape { pos, width, height, .. } => {
                expand(pos.x.0, pos.y.0, *width, *height);
            }
            FrameItem::Image { pos, width, height, .. } => {
                expand(pos.x.0, pos.y.0, width.0, height.0);
            }
            FrameItem::Line { start, end, thickness, .. } => {
                let x0 = start.x.0.min(end.x.0);
                let y0 = start.y.0.min(end.y.0);
                let x1 = start.x.0.max(end.x.0);
                let y1 = start.y.0.max(end.y.0);
                expand(x0, y0, x1 - x0, (y1 - y0).max(*thickness));
            }
            FrameItem::Group { pos, inner_width, inner_height, .. } => {
                expand(pos.x.0, pos.y.0, *inner_width, *inner_height);
            }
            FrameItem::Link { pos, size, .. } => {
                expand(pos.x.0, pos.y.0, size.width.0, size.height.0);
            }
        }
    }

    if min_x.is_infinite() {
        return (Point::ZERO, Size { width: Pt(0.0), height: Pt(0.0) });
    }

    (
        Point { x: Pt(min_x), y: Pt(min_y) },
        Size { width: Pt(max_x - min_x), height: Pt(max_y - min_y) },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::content::Content;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::{FrameItem, Pt, Size, TextStyle};
    use crate::entities::elements::link::LinkElem;
    use crate::rules::layout::{FixedMetrics, Layouter};
    use comemo::Track;
    use std::sync::Arc;

    #[test]
    fn p422_frame_item_link_construcao() {
        use crate::entities::layout_types::LinkTarget;
        let item = FrameItem::Link {
            target: LinkTarget::Url("https://x".into()),
            items: vec![FrameItem::Text {
                pos: crate::entities::layout_types::Point { x: Pt(0.0), y: Pt(0.0) },
                text: "x".into(),
                style: TextStyle::default(),
            }],
            pos: Point::ZERO,
            size: Size { width: Pt(0.0), height: Pt(0.0) },
        };
        if let FrameItem::Link { target: LinkTarget::Url(url), .. } = item {
            assert_eq!(url.as_str(), "https://x");
        } else {
            panic!("esperado Link");
        }
    }

    #[test]
    fn p424_layout_link_tem_bbox_positiva() {
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let mut layouter = Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_dyn.track());
        let elem = LinkElem {
            url: "https://example.com".into(),
            body: Content::text("Clique"),
        };
        layout(&mut layouter, &elem);
        if let FrameItem::Link { pos, size, .. } = &layouter.regions.current.current_line[0] {
            assert!(size.width.0 > 0.0, "link deve ter largura positiva");
            assert!(size.height.0 > 0.0, "link deve ter altura positiva");
            assert!(pos.x.0 >= 0.0, "link x deve ser não-negativo");
            assert!(pos.y.0 >= 0.0, "link y deve ser não-negativo");
        } else {
            panic!("esperado FrameItem::Link");
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
