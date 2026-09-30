//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/html_frame.md
//! @prompt-hash 835c9c4f
//! @layer L1

use crate::entities::elements::pdf_artifact::ArtifactKind;
use crate::entities::geometry::{FillRule, ShapeKind};
use crate::entities::html::HtmlFrameElem;
use crate::entities::image_sizer::ImageSizer;
use crate::entities::introspector::{Introspector, TagIntrospector};
use crate::entities::layout_types::{
    FrameItem, Page, Point, Pt, SemanticKind, SemanticPlacement, TextStyle,
    TransformMatrix,
};
use crate::entities::page_canvas::PageFill;
use crate::entities::position::HtmlPosition;
use crate::entities::style_chain::StyleChain;
use comemo::Track;

use super::{FontMetrics, Layouter};

const CONTEXT_MARKER_PREFIX: &str = "\0p1493-context:";

#[derive(Debug)]
pub struct HtmlFrameLayout {
    pub page: Page,
    pub positions: Vec<(u64, HtmlPosition)>,
}

pub(super) fn context_marker(id: u64, point: Point) -> FrameItem {
    FrameItem::Semantic {
        kind: SemanticKind::Artifact(ArtifactKind::Other),
        placement: SemanticPlacement::Inline,
        alt: Some(format!("{CONTEXT_MARKER_PREFIX}{id}").into()),
        items: vec![FrameItem::Shape {
            pos: point,
            kind: ShapeKind::Rect,
            width: 0.0,
            height: 0.0,
            fill: None,
            stroke: None,
            fill_rule: FillRule::NonZero,
            parent_bbox_at_emit: None,
        }],
    }
}

fn context_marker_id(item: &FrameItem) -> Option<u64> {
    let FrameItem::Semantic {
        kind: SemanticKind::Artifact(ArtifactKind::Other),
        alt: Some(alt),
        items,
        ..
    } = item
    else {
        return None;
    };
    if items.len() != 1 {
        return None;
    }
    alt.strip_prefix(CONTEXT_MARKER_PREFIX)?.parse().ok()
}

pub(super) fn is_context_marker(item: &FrameItem) -> bool {
    context_marker_id(item).is_some()
}

pub(super) fn is_context_marker_only_tree(item: &FrameItem) -> bool {
    is_context_marker(item)
}

pub(super) fn items_have_material(items: &[FrameItem]) -> bool {
    items.iter().any(|item| !is_context_marker_only_tree(item) && !matches!(item, FrameItem::TextSeparator { .. }))
}

/// A linha pode existir semanticamente sem tinta: espaços literais pertencem
/// à composição textual, mas continuam neutros para materialidade visual.
pub(super) fn items_have_causal_line(items: &[FrameItem]) -> bool {
    items_have_material(items) || items.iter().any(|item| matches!(item,
        FrameItem::TextSeparator {
            origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
            ..
        }
    ))
}

pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<'_, M, S>,
    elem: &HtmlFrameElem,
) {
    layouter.layout_content(&elem.body);
}

fn visible_top<M: FontMetrics>(item: &FrameItem, metrics: &M) -> Option<f64> {
    if is_context_marker_only_tree(item) {
        return None;
    }
    Some(match item {
        FrameItem::TextSeparator { .. } => return None,
        FrameItem::Text { pos, style, .. } | FrameItem::TextShaped { pos, style, .. } => {
            pos.y.0 - metrics.text_edges(style.size, style).0 .0
        }
        FrameItem::Glyph { pos, glyph_id, size, style, .. } => {
            pos.y.0 - metrics.glyph_ink_bounds(*glyph_id, *size, style).0 .0
        }
        FrameItem::Line { start, end, thickness, .. } => {
            start.y.0.min(end.y.0) - thickness / 2.0
        }
        FrameItem::Image { pos, .. }
        | FrameItem::Shape { pos, .. }
        | FrameItem::Group { pos, .. }
        | FrameItem::Link { pos, .. } => pos.y.0,
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => items
            .iter()
            .find(|item| !is_context_marker_only_tree(item))
            .map(|item| super::helpers::item_pos(item).1)
            .unwrap_or(0.0),
    })
}

fn collect_context_positions(
    items: &[FrameItem],
    transform: TransformMatrix,
    positions: &mut Vec<(u64, HtmlPosition)>,
) {
    for item in items {
        if let Some(id) = context_marker_id(item) {
            let Some(marker) = (match item {
                FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => items.first(),
                _ => None,
            }) else {
                continue;
            };
            let (x, y) = super::helpers::item_pos(marker);
            let (x, y) = transform.apply(x, y);
            positions.push((id, HtmlPosition { point: Point { x: Pt(x), y: Pt(y) } }));
            continue;
        }
        match item {
            FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } | FrameItem::Link { items, .. } => {
                collect_context_positions(items, transform, positions);
            }
            FrameItem::Group { pos, matrix, items, .. } => {
                let nested = transform
                    .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                    .concat(matrix);
                collect_context_positions(items, nested, positions);
            }
            _ => {}
        }
    }
}
fn strip_context_marker<M: FontMetrics>(
    item: FrameItem,
    metrics: &M,
) -> Option<FrameItem> {
    if is_context_marker(&item) {
        return None;
    }
    match item {
        FrameItem::TextReplacement { text, items } => {
            let items = items.into_iter()
                .filter_map(|item| strip_context_marker(item, metrics)).collect();
            Some(FrameItem::TextReplacement { text, items })
        }
        FrameItem::Semantic { kind, placement, alt, items } => {
            let items = items
                .into_iter()
                .filter_map(|item| strip_context_marker(item, metrics))
                .collect();
            Some(FrameItem::Semantic { kind, placement, alt, items })
        }
        FrameItem::Link { target, items, pos, size } => {
            let items = items
                .into_iter()
                .filter_map(|item| strip_context_marker(item, metrics))
                .collect();
            Some(FrameItem::Link { target, items, pos, size })
        }
        FrameItem::Group {
            pos,
            matrix,
            clip_mask,
            inner_width,
            inner_height,
            items,
        } => {
            let items = items
                .into_iter()
                .filter_map(|item| strip_context_marker(item, metrics))
                .collect();
            Some(FrameItem::Group {
                pos,
                matrix,
                clip_mask,
                inner_width,
                inner_height,
                items,
            })
        }
        other => Some(other),
    }
}

fn layout_intrinsic_impl<M: FontMetrics, S: ImageSizer>(
    elem: &HtmlFrameElem,
    chain: &StyleChain,
    metrics: M,
    sizer: S,
    capture_positions: bool,
) -> HtmlFrameLayout {
    let intr = TagIntrospector::empty();
    let intr_dyn: &dyn Introspector = &intr;
    let mut layouter = Layouter::new(metrics, sizer, chain.size(), intr_dyn.track());
    layouter.chain = chain.clone();
    layouter.style = TextStyle::from(chain);
    layouter.capture_html_context_markers = capture_positions;
    let (height, mut items, _, _, _) = layouter.layout_sub_frame(
        &elem.body,
        super::sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: f64::INFINITY,
            height: None,
            align_rtl: false,
            unconstrained_height: true,
        },
    );
    let mut positions = Vec::new();
    if capture_positions {
        collect_context_positions(&items, TransformMatrix::identity(), &mut positions);
    }

    let top = items
        .iter()
        .filter_map(|item| visible_top(item, &layouter.metrics))
        .fold(f64::INFINITY, f64::min);
    if top.is_finite() && top != 0.0 {
        for item in &mut items {
            super::helpers::offset_frame_item(item, 0.0, -top);
        }
    }

    if capture_positions {
        items = items
            .into_iter()
            .filter_map(|item| strip_context_marker(item, &layouter.metrics))
            .collect();
    }

    let refs: Vec<_> = items.iter().collect();
    let width = layouter.metrics.line_content_right(&refs);
    let page = Page {
        width,
        height,
        numbering: None,
        logical_number: 1,
        supplement: crate::entities::content::Content::Empty,
        bleed: Default::default(),
        fill: PageFill::None,
        background: vec![],
        foreground: vec![],
        items,
    };
    HtmlFrameLayout { page, positions }
}

pub(super) fn layout_intrinsic<M: FontMetrics, S: ImageSizer>(
    elem: &HtmlFrameElem,
    chain: &StyleChain,
    metrics: M,
    sizer: S,
) -> Page {
    layout_intrinsic_impl(elem, chain, metrics, sizer, false).page
}

pub(super) fn layout_intrinsic_detailed<M: FontMetrics, S: ImageSizer>(
    elem: &HtmlFrameElem,
    chain: &StyleChain,
    metrics: M,
    sizer: S,
) -> HtmlFrameLayout {
    layout_intrinsic_impl(elem, chain, metrics, sizer, true)
}
#[cfg(test)]
mod p1493_tests {
    use super::*;
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::content::Content;
    use crate::entities::elements::context_block::ContextBlockElem;
    use crate::entities::elements::place::PlaceElem;
    use crate::entities::func::Func;
    use crate::entities::geometry::ShapeKind;
    use crate::entities::image_sizer::NullImageSizer;
    use crate::entities::layout_types::{Align2D, Length, PlaceScope};
    use crate::entities::sides::Sides;
    use crate::entities::value::Value;
    use std::sync::Arc;

    fn marker(id: u64) -> Content {
        Content::ContextBlock(Arc::new(ContextBlockElem {
            id,
            closure: Func::native("p1493-marker", |_ctx, _args, _world, _file| {
                Ok(Value::None)
            }),
        }))
    }

    fn assert_no_carrier(items: &[FrameItem]) {
        for item in items {
            assert!(
                !is_context_marker(item),
                "carrier privado sobreviveu na Page: {item:?}"
            );
            match item {
                FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::Group { items, .. } => assert_no_carrier(items),
                _ => {}
            }
        }
    }

    fn detailed_preserving_legacy_page(body: Content) -> HtmlFrameLayout {
        let elem = HtmlFrameElem { body };
        let legacy = layout_intrinsic(
            &elem,
            &StyleChain::default_chain(),
            FixedMetrics,
            NullImageSizer,
        );
        let detailed = layout_intrinsic_detailed(
            &elem,
            &StyleChain::default_chain(),
            FixedMetrics,
            NullImageSizer,
        );

        assert_eq!(legacy.width, detailed.page.width, "largura mudou");
        assert_eq!(legacy.height, detailed.page.height, "altura mudou");
        assert_eq!(
            format!("{legacy:#?}"),
            format!("{:#?}", detailed.page),
            "a Page detalhada deve ser estruturalmente idêntica à fachada legada"
        );
        assert_no_carrier(&detailed.page.background);
        assert_no_carrier(&detailed.page.items);
        assert_no_carrier(&detailed.page.foreground);
        detailed
    }

    fn position_of(detailed: &HtmlFrameLayout, id: u64) -> Point {
        detailed
            .positions
            .iter()
            .find_map(|(candidate, position)| {
                (*candidate == id).then_some(position.point)
            })
            .unwrap_or_else(|| panic!("posição P1493 ausente para ContextBlock {id}"))
    }

    fn assert_close(actual: f64, expected: f64, label: &str) {
        assert!(
            (actual - expected).abs() < 0.01,
            "{label}: esperado {expected}, obtido {actual}"
        );
    }

    #[test]
    fn p1493_layout_detalhado_transporta_place_e_remove_carrier() {
        let body = Content::Place(Arc::new(PlaceElem {
            alignment: Align2D::from_string("top-left"),
            dx: 30.0,
            dy: 20.0,
            scope: PlaceScope::Column,
            float: false,
            clearance: None,
            body: marker(1493),
        }));
        let elem = HtmlFrameElem { body };
        let detailed = layout_intrinsic_detailed(
            &elem,
            &StyleChain::default_chain(),
            FixedMetrics,
            NullImageSizer,
        );

        assert_eq!(detailed.positions.len(), 1);
        assert_eq!(detailed.positions[0].0, 1493);
        assert!((detailed.positions[0].1.point.x.val() - 30.0).abs() < 0.01);
        assert!(
            (detailed.positions[0].1.point.y.val() - 20.0).abs() < 0.01,
            "y medido: {}",
            detailed.positions[0].1.point.y.val()
        );
        assert!(
            detailed.page.items.is_empty(),
            "carrier privado não pode sobreviver na Page: {:?}",
            detailed.page.items
        );
        assert_eq!(detailed.page.width, 0.0);
    }

    #[test]
    fn p1493_detailed_page_e_identica_ao_legado_em_wrappers_vazios() {
        let cases = [
            ("link", 14931, Content::link("https://example.test/p1493", marker(14931))),
            (
                "transform",
                14932,
                Content::transform(TransformMatrix::identity(), marker(14932)),
            ),
        ];

        for (name, id, body) in cases {
            let detailed = detailed_preserving_legacy_page(body);
            assert_eq!(detailed.positions.len(), 1, "{name}: posição duplicada");
            let point = position_of(&detailed, id);
            assert!(
                point.x.val().is_finite() && point.y.val().is_finite(),
                "{name}: posição deve permanecer finita: {point:?}"
            );
        }
    }

    #[test]
    fn p1493_box_transporta_inset_e_preserva_baseline_sem_mudar_page() {
        fn boxed(id: u64, left: f64) -> Content {
            Content::boxed(
                Content::sequence(vec![marker(id), Content::text("x")]),
                Some(Length::pt(30.0)),
                Some(Length::pt(20.0)),
                Sides::new(
                    Length::pt(left),
                    Length::pt(5.0),
                    Length::pt(2.0),
                    Length::pt(4.0),
                ),
                Length::pt(3.0),
            )
        }

        let at_origin = detailed_preserving_legacy_page(boxed(14933, 0.0));
        let inset = detailed_preserving_legacy_page(boxed(14934, 7.0));
        let origin_point = position_of(&at_origin, 14933);
        let inset_point = position_of(&inset, 14934);

        assert_close(inset_point.x.val() - origin_point.x.val(), 7.0, "inset.left");
        assert_close(inset_point.y.val(), origin_point.y.val(), "inset.left não move y");
        assert!(inset.page.height > 0.0, "baseline/altura explícita devem sobreviver");
    }

    #[test]
    fn p1493_carrier_apos_espaco_nao_altera_largura_nem_pivo_do_transform() {
        fn scaled(body: Content) -> Content {
            Content::transform(TransformMatrix::scale(2.0, 1.0), body)
        }

        let without_space =
            detailed_preserving_legacy_page(scaled(Content::sequence(vec![
                Content::text("x"),
                marker(14939),
            ])));
        let with_space =
            detailed_preserving_legacy_page(scaled(Content::sequence(vec![
                Content::text("x"),
                Content::Space,
                marker(14935),
            ])));
        let without_point = position_of(&without_space, 14939);
        let with_point = position_of(&with_space, 14935);
        let chain = StyleChain::default_chain();
        let style = TextStyle::from(&chain);
        let scaled_space = FixedMetrics.advance(" ", style.size, &style).val() * 2.0;

        fn visible(mut page: crate::entities::layout_types::Page) -> String {
            fn strip(items: &mut Vec<FrameItem>) {
                items.retain(|item| !matches!(item, FrameItem::TextSeparator { .. }));
                for item in items { match item {
                    FrameItem::Group { items, .. } | FrameItem::Link { items, .. } | FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } => strip(items),
                    _ => {}
                } }
            }
            strip(&mut page.items);
            format!("{page:#?}")
        }
        assert_eq!(
            visible(without_space.page.clone()),
            visible(with_space.page.clone()),
            "trailing space e carrier não podem alterar a Page visível",
        );
        assert_close(
            with_point.x.val() - without_point.x.val(),
            scaled_space,
            "o carrier deve receber o avanço do trailing space escalado",
        );
        assert_close(
            with_point.y.val(),
            without_point.y.val(),
            "trailing space não move y",
        );
        assert_eq!(with_space.page.plain_text(), "x");
    }

    fn anchor() -> Content {
        Content::place(
            Align2D::from_string("top-left"),
            0.0,
            0.0,
            PlaceScope::Column,
            false,
            None,
            Content::shape(
                ShapeKind::Rect,
                Some(Box::new(Value::Length(Length::pt(1.0)))),
                Some(Box::new(Value::Length(Length::pt(1.0)))),
                None,
                None,
            ),
        )
    }

    fn transformed_marker(body: Content) -> Content {
        Content::sequence(vec![
            anchor(),
            Content::place(
                Align2D::from_string("top-left"),
                10.0,
                5.0,
                PlaceScope::Column,
                false,
                None,
                body,
            ),
        ])
    }

    #[test]
    fn p1493_separadores_apos_contexto_nao_criam_avanco_ou_altura_fantasma() {
        let bare = detailed_preserving_legacy_page(marker(14942));
        assert_eq!(bare.positions.len(), 1);
        assert_eq!(bare.page.width, 0.0);
        assert_eq!(bare.page.height, 0.0);
        assert!(bare.page.items.is_empty());

        for (name, id, separator) in
            [("space", 14943, Content::Space), ("parbreak", 14944, Content::Parbreak)]
        {
            let detailed = detailed_preserving_legacy_page(Content::sequence(vec![
                marker(id),
                separator,
            ]));

            assert_eq!(
                detailed.positions.len(),
                1,
                "{name}: carrier deve produzir uma única posição"
            );
            assert_eq!(
                format!("{:#?}", detailed.page),
                format!("{:#?}", bare.page),
                "{name}: separador estrutural não pode alterar a Page"
            );
            assert_eq!(detailed.page.width, 0.0, "{name}: avanço fantasma");
            assert_eq!(detailed.page.height, 0.0, "{name}: altura fantasma");
            assert!(
                detailed.page.items.is_empty(),
                "{name}: separador não pode materializar itens"
            );
        }
    }

    #[test]
    fn p1493_link_preserva_translacao_causal_exata_do_carrier() {
        let id = 14945;
        let detailed =
            detailed_preserving_legacy_page(transformed_marker(Content::transform(
                TransformMatrix::translate(11.0, 5.0),
                Content::link("https://example.test/p1493-link", marker(id)),
            )));

        assert_eq!(detailed.positions.len(), 1, "Link não pode duplicar o carrier");
        let point = position_of(&detailed, id);
        assert_close(point.x.val(), 21.0, "place.x + translate.x");
        assert_close(point.y.val(), 10.0, "place.y + translate.y");
    }

    #[test]
    fn p1493_transformacoes_aninhadas_compoem_matrizes_sem_deriva() {
        let base = detailed_preserving_legacy_page(transformed_marker(marker(14936)));
        let flat = detailed_preserving_legacy_page(transformed_marker(
            Content::transform(TransformMatrix::translate(11.0, 5.0), marker(14937)),
        ));
        let nested =
            detailed_preserving_legacy_page(transformed_marker(Content::transform(
                TransformMatrix::translate(4.0, 2.0),
                Content::transform(TransformMatrix::translate(7.0, 3.0), marker(14938)),
            )));

        let base_point = position_of(&base, 14936);
        let flat_point = position_of(&flat, 14937);
        let nested_point = position_of(&nested, 14938);
        assert_close(flat_point.x.val(), nested_point.x.val(), "nested x == flat x");
        assert_close(flat_point.y.val(), nested_point.y.val(), "nested y == flat y");
        assert_close(flat_point.x.val() - base_point.x.val(), 11.0, "delta x");
        assert_close(flat_point.y.val() - base_point.y.val(), 5.0, "delta y");

        let scale = TransformMatrix::scale(2.0, 1.0);
        let translate = TransformMatrix::translate(7.0, 0.0);
        let ordered_flat = detailed_preserving_legacy_page(transformed_marker(
            Content::transform(scale.concat(&translate), marker(14940)),
        ));
        let ordered_nested = detailed_preserving_legacy_page(transformed_marker(
            Content::transform(scale, Content::transform(translate, marker(14941))),
        ));
        let ordered_flat_point = position_of(&ordered_flat, 14940);
        let ordered_nested_point = position_of(&ordered_nested, 14941);
        assert_close(
            ordered_nested_point.x.val(),
            ordered_flat_point.x.val(),
            "scale ∘ translate: nested x == flat x",
        );
        assert_close(
            ordered_nested_point.y.val(),
            ordered_flat_point.y.val(),
            "scale ∘ translate: nested y == flat y",
        );
        assert_close(
            ordered_nested_point.x.val() - base_point.x.val(),
            14.0,
            "scale ∘ translate: delta x ordenado",
        );
        assert_close(
            ordered_nested_point.y.val() - base_point.y.val(),
            0.0,
            "scale ∘ translate: delta y neutro",
        );
        assert!(nested_point.x.val().is_finite() && nested_point.y.val().is_finite());
    }
}
