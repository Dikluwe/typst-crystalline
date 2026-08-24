//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/heading.md
//! @prompt-hash 41fd8b3d
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
        let gap = layouter.prev_block_below_pending.max(above_pt);
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

    layouter.layout_content(body);
    let heading_baseline = layouter.regions.current.cursor_y.0;
    let (_, bottom) = layouter.metrics.text_edges(heading_size, &layouter.style);
    let heading_descent = bottom.0.abs();
    layouter.flush_line();
    layouter.prev_line_baseline = heading_baseline;
    layouter.prev_block_equation_descent = 0.0;

    // **P1104/P1107** — Colapso de saída (below) unificado via protocolo genérico
    layouter.prev_block_below_pending = below_pt;
    layouter.block_chain_active = true;
    layouter.prev_margin_is_parbreak = false;

    layouter.style = prev;
}
