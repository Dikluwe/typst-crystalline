//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/link.md
//! @prompt-hash ae77aefb
//! @layer L1
//! @updated 2026-06-23
//!
//! P422 — Layout de `Content::Link`. Renderiza o body e envolve os itens
//! resultantes em `FrameItem::Link`, preservando o URL como metadado.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::elements::link::LinkElem;
use crate::entities::layout_types::{FrameItem, Point, Pt, Size};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma hiperligação: renderiza o body e envolve os itens em
/// `FrameItem::Link`. Cor/sublinhado diferidos para `FrameItem::Decoration`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &LinkElem,
) {
    layouter.layout_link_capture(
        &e.body,
        crate::entities::layout_types::LinkTarget::Url(e.url.clone()),
        false,
    );
}

/// Calcula a bounding-box acumulada dos `items` para preencher `pos`/`size`
/// do `FrameItem::Link`.  Para texto mede o avanço via `FontMetrics`;
/// para grupos/links aninhados usa os campos já existentes.
pub(super) fn link_bbox<M: FontMetrics>(
    items: &[FrameItem],
    metrics: &M,
) -> (Point, Size) {
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
        if super::html_frame::is_context_marker(item) {
            continue;
        }
        match item {
            FrameItem::TextSeparator { .. } => {}
            FrameItem::Text { pos, text, style, .. } => {
                let w = metrics.advance(text.as_str(), style.size, style).0;
                let (ascender, line_height) = metrics.vertical_metrics(style.size, style);
                let asc_pt = ascender.val();
                let h_pt = line_height.val();
                expand(pos.x.0, pos.y.0 - asc_pt, w, h_pt);
            }
            FrameItem::TextShaped { pos, text, style, .. } => {
                let w = metrics.advance(text.as_str(), style.size, style).0;
                let (ascender, line_height) = metrics.vertical_metrics(style.size, style);
                let asc_pt = ascender.val();
                let h_pt = line_height.val();
                expand(pos.x.0, pos.y.0 - asc_pt, w, h_pt);
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
            FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => {
                let (pos, size) = link_bbox(items, metrics);
                expand(pos.x.0, pos.y.0, size.width.0, size.height.0);
            }
        }
    }

    if min_x.is_infinite() {
        return (Point::ZERO, Size { width: Pt(0.0), height: Pt(0.0) });
    }

    (
        Point { x: Pt(min_x), y: Pt(min_y) },
        Size {
            width: Pt(max_x - min_x),
            height: Pt(max_y - min_y),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::content::Content;
    use crate::entities::elements::link::LinkElem;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::introspector::{Introspector, TagIntrospector};
    use crate::entities::layout_types::{FrameItem, Pt, Size, TextStyle};
    use comemo::Track;
    use std::sync::Arc;

    #[test]
    fn text_link_bbox_uses_total_line_height_without_adding_ascender() {
        let item = FrameItem::Text {
            pos: Point { x: Pt(10.0), y: Pt(20.0) },
            text: "AB".into(),
            style: TextStyle::regular(Pt(10.0)),
            bidi: None,
        };
        let (pos, size) = link_bbox(&[item], &FixedMetrics);
        assert_eq!(
            (pos.x.0, pos.y.0, size.width.0, size.height.0),
            (10.0, 12.0, 12.0, 12.0)
        );
    }

    #[test]
    fn shaped_link_bbox_uses_total_line_height_without_adding_ascender() {
        let item = FrameItem::TextShaped {
            pos: Point { x: Pt(10.0), y: Pt(20.0) },
            text: "AB".into(),
            style: TextStyle::regular(Pt(10.0)),
            bidi: None,
            glyphs: vec![],
            units_per_em: 1000,
            font: None,
        };
        let (pos, size) = link_bbox(&[item], &FixedMetrics);
        assert_eq!(
            (pos.x.0, pos.y.0, size.width.0, size.height.0),
            (10.0, 12.0, 12.0, 12.0)
        );
    }

    #[test]
    fn p422_frame_item_link_construcao() {
        use crate::entities::layout_types::LinkTarget;
        let item = FrameItem::Link {
            target: LinkTarget::Url("https://x".into()),
            items: vec![FrameItem::Text {
                pos: crate::entities::layout_types::Point { x: Pt(0.0), y: Pt(0.0) },
                text: "x".into(),
                style: TextStyle::default(),
                bidi: None,
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
        let mut layouter =
            Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_dyn.track());
        let elem = LinkElem {
            url: "https://example.com".into(),
            body: Content::text("Clique"),
        };
        layout(&mut layouter, &elem);
        let doc = layouter.finish();
        if let FrameItem::Link { pos, size, .. } = &doc.pages[0].items[0] {
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
        let mut layouter =
            Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_dyn.track());
        let elem = LinkElem {
            url: "https://example.com".into(),
            body: Content::text("Clique"),
        };
        layout(&mut layouter, &elem);
        let doc = layouter.finish();
        // O documento final publica o fragmento clicável com o conteúdo íntegro.
        assert_eq!(doc.pages[0].items.len(), 1);
        assert!(
            matches!(doc.pages[0].items[0], FrameItem::Link { .. }),
            "esperado FrameItem::Link"
        );
    }

    #[test]
    fn p422_layout_link_body_formatado() {
        let intr = TagIntrospector::empty();
        let intr_dyn: &dyn Introspector = &intr;
        let mut layouter =
            Layouter::new(FixedMetrics, NullImageSizer, 12.0, intr_dyn.track());
        let elem = LinkElem {
            url: "https://example.com".into(),
            body: Content::Sequence(Arc::from(vec![
                Content::text("A "),
                Content::text("B"),
            ])),
        };
        layout(&mut layouter, &elem);
        let doc = layouter.finish();
        assert_eq!(doc.pages[0].items.len(), 1);
        if let FrameItem::Link { items, .. } = &doc.pages[0].items[0] {
            assert!(!items.is_empty(), "body formatado deve produzir items");
        } else {
            panic!("esperado FrameItem::Link");
        }
    }
}

#[cfg(test)]
mod fragment_regressions {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::{
        content::Content,
        corners::Corners,
        elements::{boxed::BoxedElem, columns::ColumnsElem, linebreak::LinebreakElem},
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        label::Label,
        layout_types::{
            Length, LinkTarget, PageMarginSpec, PageMargins, PagedDocument,
            TransformMatrix,
        },
        sides::Sides,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn url(body: Content) -> Content {
        Content::Link(Arc::new(LinkElem { url: "https://outer.example".into(), body }))
    }
    fn br() -> Content {
        Content::Linebreak(Arc::new(LinebreakElem {
            justify: false,
            justify_explicit: false,
        }))
    }
    fn lines() -> Content {
        Content::sequence(vec![
            Content::text("ONE"),
            br(),
            Content::text("TWO"),
            br(),
            Content::text("THREE"),
            br(),
            Content::text("FOUR"),
        ])
    }
    fn render_with(
        width: f64,
        height: f64,
        f: impl FnOnce(&mut Layouter<'_, FixedMetrics, NullImageSizer>),
    ) -> PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = width;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = width;
        l.regions.current.height = height;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        f(&mut l);
        l.finish()
    }
    fn render(body: Content, width: f64, height: f64) -> PagedDocument {
        render_with(width, height, |l| l.layout_content(&body))
    }
    #[derive(Debug, PartialEq)]
    struct Ink {
        page: usize,
        text: String,
        x: f64,
        y: f64,
        target: Option<String>,
    }
    fn inspect(
        items: &[FrameItem],
        page: usize,
        m: TransformMatrix,
        target: Option<String>,
        out: &mut Vec<Ink>,
    ) {
        for item in items {
            match item {
                FrameItem::Text { pos, text, .. }
                | FrameItem::TextShaped { pos, text, .. } => {
                    let (x, y) = m.apply(pos.x.0, pos.y.0);
                    out.push(Ink {
                        page,
                        text: text.to_string(),
                        x,
                        y,
                        target: target.clone(),
                    });
                }
                FrameItem::Group { pos, matrix, items, .. } => inspect(
                    items,
                    page,
                    m.concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                        .concat(matrix),
                    target.clone(),
                    out,
                ),
                FrameItem::Link { target: t, items, pos, size } => {
                    assert!(pos.x.0.is_finite() && pos.y.0.is_finite());
                    assert!(size.width.0 >= 0.0 && size.height.0 >= 0.0);
                    let name = match t {
                        LinkTarget::Url(u) => u.to_string(),
                        LinkTarget::Destination(l) => format!("label:{}", l.0),
                    };
                    inspect(items, page, m, Some(name), out);
                }
                FrameItem::Semantic { alt, items, .. } => {
                    assert!(
                        !alt.as_deref().is_some_and(|s| s.starts_with("\0tekt-link:")),
                        "private boundary leaked"
                    );
                    inspect(items, page, m, target.clone(), out);
                }
                FrameItem::TextReplacement { items, .. } => {
                    inspect(items, page, m, target.clone(), out)
                }
                FrameItem::TextSeparator { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Line { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }
    fn ink(doc: &PagedDocument) -> Vec<Ink> {
        let mut v = vec![];
        for (i, p) in doc.pages.iter().enumerate() {
            inspect(&p.items, i + 1, TransformMatrix::identity(), None, &mut v);
        }
        v
    }
    fn same_geometry(a: &PagedDocument, b: &PagedDocument) {
        assert_eq!(a.pages.len(), b.pages.len());
        let mut x = ink(a);
        let mut y = ink(b);
        for p in &mut x {
            p.target = None;
        }
        for p in &mut y {
            p.target = None;
        }
        assert_eq!(x, y, "link must not alter content order/geometry");
    }
    #[test]
    fn fragment_link_admission_on_next_page_keeps_prefix_outside() {
        let make = |linked| {
            render_with(100.0, 40.0, |l| {
                l.layout_content(&Content::text("PREFIX"));
                l.flush_line();
                l.layout_content(&if linked {
                    url(Content::text("BODY"))
                } else {
                    Content::text("BODY")
                });
            })
        };
        let plain = make(false);
        let linked = make(true);
        same_geometry(&plain, &linked);
        let items = ink(&linked);
        assert_eq!(items[0].target, None);
        assert_eq!(items[1].target.as_deref(), Some("https://outer.example"));
        assert_eq!(items[1].page, 2);
    }
    #[test]
    fn fragment_link_wrap_preserves_external_prefix_and_suffix() {
        let body = Content::text("ALFA BETA GAMA DELTA");
        let make = |body| {
            Content::sequence(vec![Content::text("P "), body, Content::text(" Q")])
        };
        let plain = render(make(body.clone()), 70.0, 130.0);
        let linked = render(make(url(body)), 70.0, 130.0);
        same_geometry(&plain, &linked);
        for i in ink(&linked) {
            let expected = if i.text.trim() == "P" || i.text.trim() == "Q" {
                None
            } else {
                Some("https://outer.example")
            };
            assert_eq!(i.target.as_deref(), expected, "{}", i.text);
        }
    }
    #[test]
    fn fragment_link_forced_lines_remain_on_their_pages() {
        let plain = render(lines(), 90.0, 50.0);
        let linked = render(url(lines()), 90.0, 50.0);
        same_geometry(&plain, &linked);
        assert!(linked.pages.len() > 1);
        assert!(ink(&linked)
            .iter()
            .all(|i| i.target.as_deref() == Some("https://outer.example")));
    }
    #[test]
    fn fragment_link_rich_box_preserves_local_coordinates() {
        let body = Content::Boxed(Arc::new(BoxedElem {
            body: lines(),
            width: Some(Length::pt(60.0)),
            width_fraction: None,
            height: None,
            inset: Sides::uniform(Length::pt(2.0)),
            baseline: Length::pt(0.0),
            outset: Sides::uniform(Length::pt(0.0)),
            radius: Corners::uniform(Length::pt(0.0)),
            clip: false,
            fill: None,
            stroke: None,
        }));
        let plain = render(body.clone(), 140.0, 160.0);
        let linked = render(url(body), 140.0, 160.0);
        same_geometry(&plain, &linked);
        assert!(ink(&linked).iter().all(|i| i.target.is_some()));
    }
    #[test]
    fn fragment_link_columns_keep_each_fragment_in_its_region() {
        let body = Content::Columns(Arc::new(ColumnsElem {
            count: 2,
            gutter: Some(Length::pt(6.0)),
            body: Content::text(
                "ONE TWO THREE FOUR FIVE SIX SEVEN EIGHT NINE TEN ELEVEN TWELVE",
            ),
            page_columns: true,
        }));
        let plain = render(body.clone(), 140.0, 65.0);
        let linked = render(url(body), 140.0, 65.0);
        same_geometry(&plain, &linked);
        assert!(ink(&linked).iter().all(|i| i.target.is_some()));
    }
    #[test]
    fn fragment_internal_destination_crosses_pages_without_collecting_neighbors() {
        let plain = render_with(100.0, 50.0, |l| {
            l.layout_content(&Content::text("P"));
            l.layout_content(&lines());
            l.layout_content(&Content::text("Q"));
        });
        let linked = render_with(100.0, 50.0, |l| {
            l.layout_content(&Content::text("P"));
            super::super::outline::layout_destination_link(
                l,
                &lines(),
                Label("target".into()),
            );
            l.layout_content(&Content::text("Q"));
        });
        same_geometry(&plain, &linked);
        for i in ink(&linked) {
            assert_eq!(
                i.target.as_deref(),
                if i.text == "P" || i.text == "Q" { None } else { Some("label:target") }
            );
        }
    }
    #[test]
    fn fragment_link_empty_body_keeps_auto_geometry_empty() {
        let plain = render(Content::Empty, 100.0, 100.0);
        let linked = render(url(Content::Empty), 100.0, 100.0);
        same_geometry(&plain, &linked);
        assert!(
            linked.pages[0]
                .items
                .iter()
                .all(|i| !matches!(i, FrameItem::Link { .. })),
            "empty body has no clickable area"
        );
    }
    #[test]
    fn fragment_link_nested_target_and_external_suffix_stay_distinct() {
        let inner = Content::Link(Arc::new(LinkElem {
            url: "https://inner.example".into(),
            body: Content::text("INNER"),
        }));
        let body = Content::sequence(vec![
            Content::text("OUTER "),
            inner,
            Content::text(" END"),
        ]);
        let plain = render(
            Content::sequence(vec![body.clone(), Content::text(" SUFFIX")]),
            160.0,
            100.0,
        );
        let linked = render(
            Content::sequence(vec![url(body), Content::text(" SUFFIX")]),
            160.0,
            100.0,
        );
        same_geometry(&plain, &linked);
        for i in ink(&linked) {
            let t = match i.text.trim() {
                "INNER" => Some("https://inner.example"),
                "SUFFIX" => None,
                _ => Some("https://outer.example"),
            };
            assert_eq!(i.target.as_deref(), t, "{}", i.text);
        }
    }
    #[test]
    fn fragment_link_checkpoint_discards_aborted_destination() {
        let doc = render_with(120.0, 100.0, |l| {
            l.layout_content(&Content::text("P "));
            let checkpoint = l.capture_flow_suffix_checkpoint();
            l.layout_content(&Content::Link(Arc::new(LinkElem {
                url: "https://aborted.example".into(),
                body: Content::text("ABORT"),
            })));
            l.restore_flow_suffix_checkpoint(checkpoint);
            l.layout_content(&url(Content::text("KEEP")));
        });
        let data = ink(&doc);
        assert_eq!(data.iter().map(|i| i.text.as_str()).collect::<String>(), "PKEEP");
        assert_eq!(data[0].target, None);
        assert_eq!(data[1].target.as_deref(), Some("https://outer.example"));
    }
    fn float_text(text: &str) -> Content {
        use crate::entities::{
            elements::place::PlaceElem,
            layout_types::{Align2D, HAlign, PlaceScope, VAlign},
        };
        Content::Place(Arc::new(PlaceElem {
            alignment: Align2D { h: Some(HAlign::Left), v: Some(VAlign::Top) },
            dx: 0.0,
            dy: 0.0,
            scope: PlaceScope::Parent,
            float: true,
            clearance: Some(Length::pt(0.0)),
            body: Content::text(text),
        }))
    }
    #[test]
    fn fragment_link_deferred_float_retains_its_lexical_target() {
        let plain = render(
            Content::sequence(vec![float_text("FLOAT"), Content::text("BODY")]),
            120.0,
            100.0,
        );
        let linked = render(
            Content::sequence(vec![url(float_text("FLOAT")), Content::text("BODY")]),
            120.0,
            100.0,
        );
        same_geometry(&plain, &linked);
        for i in ink(&linked) {
            assert_eq!(
                i.target.as_deref(),
                if i.text == "FLOAT" { Some("https://outer.example") } else { None }
            );
        }
    }
    #[test]
    fn fragment_link_unrelated_pending_float_does_not_inherit_active_target() {
        let plain =
            render(Content::sequence(vec![float_text("FLOAT"), lines()]), 120.0, 50.0);
        let linked = render(
            Content::sequence(vec![float_text("FLOAT"), url(lines())]),
            120.0,
            50.0,
        );
        same_geometry(&plain, &linked);
        for i in ink(&linked) {
            assert_eq!(
                i.target.as_deref(),
                if i.text == "FLOAT" { None } else { Some("https://outer.example") }
            );
        }
    }
    #[test]
    fn fragment_link_page_layers_do_not_inherit_main_flow_target() {
        let doc = render_with(120.0, 50.0, |l| {
            l.page_config.background = Some(Content::text("BG"));
            l.page_config.foreground = Some(Content::Link(Arc::new(LinkElem {
                url: "https://canvas.example".into(),
                body: Content::text("FG"),
            })));
            l.page_config.footer = crate::entities::page_running::PageMarginal::Content(
                Arc::new(Content::text("FOOT")),
            );
            l.layout_content(&url(lines()));
        });
        assert!(doc.pages.len() > 1);
        for (i, page) in doc.pages.iter().enumerate() {
            let mut data = vec![];
            inspect(
                &page.background,
                i + 1,
                TransformMatrix::identity(),
                None,
                &mut data,
            );
            inspect(
                &page.foreground,
                i + 1,
                TransformMatrix::identity(),
                None,
                &mut data,
            );
            for item in data {
                assert_eq!(
                    item.target.as_deref(),
                    if item.text == "FG" { Some("https://canvas.example") } else { None },
                    "{}",
                    item.text
                );
            }
        }
    }
}
