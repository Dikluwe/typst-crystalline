//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/heading.md
//! @prompt-hash 8a58cd31
//! @layer L1
//! @updated 2026-08-18
//!
//! Atomização (ADR-0109, P377): o layout de `Heading` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). P451: suporte a patterns de numeração configuráveis.

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::counter::CounterKey;
use crate::entities::counter_format::format_counter;
use crate::entities::element_kind::ElementKind;
use crate::entities::elements::heading::HeadingElem;
use crate::entities::introspector::Introspector;
use crate::entities::layout_types::TextStyle;
use crate::entities::selector::Selector;
use crate::entities::value::Value;

use super::helpers::heading_scale;
use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um cabeçalho (`= Heading`): estilo bold escalado por nível,
/// prefixo numérico opcional (gate na chain, valor via Introspector), body.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    h: &HeadingElem,
) {
    use crate::entities::layout_types::Pt;

    // Modelo D (P316): Heading delegado; re-bind dos campos.
    let level = &h.level;
    let body = &h.body;

    // A heading interrupts the preceding paragraph even without a blank line.
    super::parbreak::layout(layouter);
    let font_base = layouter.style.size.val();
    let scale = heading_scale(*level);
    let heading_size = layouter.style.size * scale;

    let above_em = if *level == 1 { 1.8 } else { 1.44 };
    let above_pt = font_base * above_em;
    // **P1063** — below é relativo ao tamanho base do corpo (0.75 * font_base),
    // simétrico à divisão por escala de `above`. Provado pelo avanço com block(above: 2em).
    let below_pt = font_base * 0.75;

    let prev = layouter.style.clone();
    let prev_chain = layouter.chain.clone();
    let heading_style = TextStyle {
        bold: true,
        size: heading_size,
        heading_level: Some(*level),
        ..prev.clone()
    };

    // **P1107** — Colapso de entrada (above) unificado via protocolo genérico:
    // Eliminação das constantes empíricas (9.2950, 3.9160, 1.6632) do P1063.
    // O `text_edges` deve usar `heading_style` (bold ativo) para obter o cap_height exato da face Bold.
    let (top, _) = layouter.metrics.text_edges(heading_size, &heading_style);
    if layouter.block_chain_active || layouter.flow_placement_y().is_some() {
        let gap = super::block::collapse_margin(
            layouter.prev_block_below_pending,
            layouter.prev_margin_is_parbreak,
            above_pt,
            false,
        );
        let prev_descent = layouter.prev_block_equation_descent;
        let prev_bottom = layouter
            .last_block_descent_y
            .map(|b| b.max(layouter.prev_line_baseline + prev_descent))
            .unwrap_or(layouter.prev_line_baseline + prev_descent);
        layouter.regions.current.cursor_y = Pt(prev_bottom + gap) + top;
    } else {
        let origin = layouter
            .block_place_region
            .map_or(layouter.page_config.margin.top, |region| region.y.0);
        layouter.regions.current.cursor_y = Pt(origin) + top;
    }
    // **P1063/P1119** — Limpar flag de parbreak antes de layout_content para evitar
    // que ensure_initial_baseline (branch asc_diff) sobrescreva o cursor_y já fixado
    // pelo protocolo P1107 acima.
    layouter.prev_margin_is_parbreak = false;
    layouter.prev_block_below_pending = 0.0;
    layouter.initial_baseline_pending = false;
    layouter.block_chain_active = false;
    layouter.line_assumed_ascent = Some(top.0);

    layouter.chain = layouter.chain.push(crate::entities::style_chain::StyleDelta {
        size: Some(crate::entities::layout_types::Length::pt(heading_size.val())),
        ..crate::entities::style_chain::StyleDelta::empty()
    });
    layouter.style = heading_style;

    // Prefixo numérico — apenas se numbering estiver activo.
    let numbering_on =
        matches!(layouter.chain.custom("heading.numbering"), Some(Value::Bool(true)),);
    if numbering_on {
        let pattern: Option<EcoString> =
            match layouter.chain.custom("heading.numbering.pattern") {
                Some(Value::Str(s)) => Some(s.clone()),
                _ => None,
            };
        let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
        let legacy_prefix = |loc| {
            layouter
                .introspector
                .formatted_counter_at(&heading_key, loc)
                .map(|n| EcoString::from(format!("{}.", n)))
        };
        let prefix_text = layouter.current_location.and_then(|loc| match pattern {
            Some(ref pat) => layouter
                .introspector
                .counter_values_at(&heading_key, loc)
                .and_then(|values| format_counter(values, pat))
                .map(EcoString::from)
                .or_else(|| legacy_prefix(loc)),
            None => legacy_prefix(loc),
        });
        if let Some(prefix_text) = prefix_text {
            let prefix = Content::text(prefix_text.as_str());
            layouter.layout_content(&prefix);
            layouter.layout_content(&Content::h_space(
                crate::entities::layout_types::Length::em(0.3),
                true,
            ));
        }
    }

    super::paragraph_indent::end(layouter);
    let heading_indent = std::mem::replace(
        &mut layouter.paragraph_indent,
        super::paragraph_indent::State::inline(),
    );
    layouter.layout_planned_child(
        crate::compiler::introspect::page_running::PlanEdge::HeadingBody,
        body,
    );
    let has_line =
        super::html_frame::items_have_material(&layouter.regions.current.current_line);
    let empty_baseline = (!has_line && layouter.flow_placement_y().is_none())
        .then_some(layouter.regions.current.cursor_y.0);
    layouter.flush_line();
    if let Some(baseline) = empty_baseline {
        layouter.prev_line_baseline = baseline;
    }
    let end = layouter.flow_placement_y().unwrap_or(layouter.prev_line_baseline);
    layouter.prev_block_equation_descent = (end - layouter.prev_line_baseline).max(0.0);
    layouter.last_block_descent_y = Some(end);

    // **P1104/P1107** — Colapso de saída (below) unificado via protocolo genérico
    layouter.prev_block_below_pending = below_pt;
    layouter.block_chain_active = true;
    layouter.prev_margin_is_parbreak = false;

    layouter.paragraph_indent = heading_indent;
    layouter.style = prev;
    layouter.chain = prev_chain;
}

#[cfg(test)]
mod a2_margin_priority_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, Length},
        sides::Sides,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn title_y(ambient: f64, level: u8, below: Option<f64>, initial: bool) -> f64 {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom("par.spacing", Value::Length(Length::pt(ambient)));
        let original_size = l.style.size;
        if !initial {
            let block = crate::entities::elements::block::BlockElem {
                body: Content::text("BEFORE"),
                width: None,
                height: None,
                inset: Sides::uniform(Length::ZERO),
                breakable: true,
                outset: Sides::uniform(Length::ZERO),
                radius: crate::entities::corners::Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: below.map(Length::pt),
                sticky: false,
            };
            l.layout_content(&Content::Block(Arc::new(block)));
        }
        l.layout_content(&Content::heading(level, Content::text("TITLE")));
        assert_eq!(l.style.size, original_size);
        let doc = l.finish();
        doc.pages
            .iter()
            .flat_map(|page| {
                crate::compiler::layout::tests::frame_items_recursive(&page.items)
            })
            .find_map(|item| match item {
                FrameItem::Text { text, pos, .. } if text.as_str() == "TITLE" => {
                    Some(pos.y.0)
                }
                _ => None,
            })
            .expect("heading survives")
    }

    #[test]
    fn a2_margin_heading_above_overrides_automatic_previous_margin() {
        for level in [1, 2, 3] {
            for ambient in [0.0, 5.0, 24.0, 60.0] {
                assert_eq!(
                    title_y(ambient, level, None, false),
                    title_y(ambient, level, Some(0.0), false),
                    "level={level}, ambient={ambient}"
                );
                assert_eq!(
                    title_y(ambient, level, None, true),
                    title_y(0.0, level, None, true)
                );
            }
        }
    }

    #[test]
    fn a2_margin_heading_keeps_larger_explicit_previous_margin() {
        for level in [1, 2, 3] {
            let above: f64 = 11.0 * if level == 1 { 1.8 } else { 1.44 };
            let zero = title_y(24.0, level, Some(0.0), false);
            for below in [3.0, 30.0] {
                let shift = (below - above).max(0.0);
                assert!(
                    (title_y(24.0, level, Some(below), false) - zero - shift).abs()
                        < 1e-8
                );
            }
        }
    }
}

#[cfg(test)]
mod readiness_flow_tests {
    use super::*;
    use crate::entities::{
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::FrameItem,
    };
    use comemo::Track;

    #[test]
    fn readiness_heading_closes_preceding_prose() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.layout_content(&Content::heading(1, Content::text("FIRST")));
        l.layout_content(&Content::text("BEFORE"));
        l.layout_content(&Content::heading(2, Content::text("SECOND")));
        l.layout_content(&Content::text("AFTER"));
        let doc = l.finish();
        let y = |name: &str| {
            doc.pages
                .iter()
                .flat_map(|p| {
                    crate::compiler::layout::tests::frame_items_recursive(&p.items)
                })
                .find_map(|i| match i {
                    FrameItem::Text { text, pos, .. } if text == name => Some(pos.y.0),
                    _ => None,
                })
                .unwrap()
        };
        assert!(y("FIRST") < y("BEFORE"));
        assert!(y("BEFORE") < y("SECOND"), "preceding prose must close before heading");
        assert!(y("SECOND") < y("AFTER"));
    }

    #[test]
    fn readiness_heading_inside_block_uses_actual_inner_origin_and_restores_parent() {
        use crate::entities::{
            layout_types::{Length, PageMarginSpec, PageMargins, Pt},
            sides::Sides,
        };
        let intr = TagIntrospector::empty();
        let mut positions = Vec::new();
        for inset in [0.0, 10.0] {
            let mut l = Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            l.page_config.width = 300.0;
            l.page_config.height = 300.0;
            l.page_config.margin = PageMargins::uniform(20.0);
            l.page_config.margin_spec = PageMarginSpec::uniform(20.0);
            l.regions.current.width = 300.0;
            l.regions.current.height = 300.0;
            l.regions.current.line_start_x = Pt(20.0);
            l.regions.current.cursor_x = Pt(20.0);
            l.regions.current.cursor_y = Pt(20.0);
            l.layout_content(&Content::heading(1, Content::text("FIRST")));
            l.layout_content(&Content::block(
                Content::heading(2, Content::text("INNER")),
                None,
                None,
                Sides::new(Length::ZERO, Length::pt(inset), Length::ZERO, Length::ZERO),
                true,
            ));
            l.layout_content(&Content::heading(2, Content::text("LAST")));
            assert!(
                l.block_place_region.is_none(),
                "the nested Block must restore the parent region"
            );
            let doc = l.finish();
            assert_eq!(doc.pages.len(), 1);
            let items = crate::compiler::layout::tests::frame_items_recursive(
                &doc.pages[0].items,
            );
            let y = ["FIRST", "INNER", "LAST"].map(|needle| {
                let found: Vec<_> = items
                    .iter()
                    .filter_map(|item| match item {
                        FrameItem::Text { text, pos, .. } if text == needle => {
                            Some(pos.y.0)
                        }
                        _ => None,
                    })
                    .collect();
                assert_eq!(found.len(), 1, "original word {needle} once");
                found[0]
            });
            assert!(y[1] > y[0], "nested Heading overlaps predecessor: {y:?}");
            assert!(
                y[2] > y[1],
                "the outer Heading must follow the completed Block: {y:?}"
            );
            positions.push(y);
        }
        assert!((positions[1][0] - positions[0][0]).abs() < 1e-8);
        for index in [1, 2] {
            assert!((positions[1][index]-positions[0][index]-10.0).abs()<1e-8,"inset must advance nested and following titles exactly once: {positions:?}");
        }
    }

    #[test]
    fn readiness_heading_publishes_committed_lower_edge() {
        let intr = TagIntrospector::empty();
        for level in [1, 2, 3] {
            let mut l = Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            l.style.bottom_edge =
                Some(crate::entities::layout_types::TextEdge::Metric("descender".into()));
            l.layout_content(&Content::heading(level, Content::text("TITLE")));
            let end = l.flow_placement_y().expect("committed heading end");
            let below = l.prev_block_below_pending;
            let top = l.metrics.text_edges(l.style.size, &l.style).0 .0;
            l.layout_content(&Content::text("BODY"));
            assert!(
                (l.regions.current.cursor_y.0 - (end + below + top)).abs() < 1e-8,
                "following prose must start below the heading's physical edge"
            );
        }
    }
}

#[cfg(test)]
mod readiness_number_gap {
    use super::*;
    use crate::entities::layout_types::FrameItem;

    #[test]
    fn readiness_flow_tests_number_gap_is_three_tenths_of_heading_em() {
        let doc =
            super::super::layout(&Content::heading_numbered(1, Content::text("TITLE")));
        let items: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|p| crate::compiler::layout::tests::frame_items_recursive(&p.items))
            .collect();
        let (x, size) = items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { text, pos, style, .. } if text.trim() == "1." => {
                    Some((pos.x.0, style.size.0))
                }
                _ => None,
            })
            .expect("number prefix");
        let title = items
            .iter()
            .find_map(|i| match i {
                FrameItem::Text { text, pos, .. } if text == "TITLE" => Some(pos.x.0),
                _ => None,
            })
            .expect("title body");
        assert!(
            (title - x - 2.0 * 0.6 * size - 0.3 * size).abs() < 1e-8,
            "generated heading separator is 0.3em regardless of space glyph width"
        );
    }
}
