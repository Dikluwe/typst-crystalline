//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/heading.md
//! @prompt-hash f26fbc75
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
    if layouter.block_chain_active {
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
        layouter.regions.current.cursor_y = Pt(layouter.page_config.margin.top) + top;
    }
    // **P1063/P1119** — Limpar flag de parbreak antes de layout_content para evitar
    // que ensure_initial_baseline (branch asc_diff) sobrescreva o cursor_y já fixado
    // pelo protocolo P1107 acima.
    layouter.prev_margin_is_parbreak = false;
    layouter.prev_block_below_pending = 0.0;
    layouter.initial_baseline_pending = false;

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
                .map(|n| EcoString::from(format!("{}. ", n)))
        };
        let prefix_text = layouter.current_location.and_then(|loc| match pattern {
            Some(ref pat) => layouter
                .introspector
                .counter_values_at(&heading_key, loc)
                .and_then(|values| format_counter(values, pat))
                .map(|s| EcoString::from(format!("{} ", s)))
                .or_else(|| legacy_prefix(loc)),
            None => legacy_prefix(loc),
        });
        if let Some(prefix_text) = prefix_text {
            let prefix = Content::text(prefix_text.as_str());
            layouter.layout_content(&prefix);
        }
    }

    super::paragraph_indent::end(layouter);
    let heading_indent = std::mem::replace(
        &mut layouter.paragraph_indent,
        super::paragraph_indent::State::inline(),
    );
    layouter.layout_content(body);
    let heading_baseline = layouter.regions.current.cursor_y.0;
    layouter.flush_line();
    layouter.prev_line_baseline = heading_baseline;
    layouter.prev_block_equation_descent = 0.0;

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
