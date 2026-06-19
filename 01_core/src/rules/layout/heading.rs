//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash af160bbb
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P377): o layout de `Heading` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.

use crate::entities::content::Content;
use crate::entities::elements::heading::HeadingElem;
use crate::entities::introspector::Introspector;
use crate::entities::layout_types::TextStyle;

use super::helpers::heading_scale;
use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um cabeçalho (`= Heading`): estilo bold escalado por nível,
/// prefixo numérico opcional (gate na chain, valor via Introspector), body.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    h:        &HeadingElem,
) {
    // Modelo D (P316): Heading delegado; re-bind dos campos.
    let level = &h.level;
    let body = &h.body;
    // P190F (M6 categoria Counters core): Layouter mutação
    // `self.counter.step_hierarchical` removida — counter hierárquico
    // populated via Introspector path location-aware (CounterRegistry
    // P184B + P185B). Layouter só lê via `formatted_counter_at`.
    let _ = level;  // preservar binding para uso abaixo

    let heading_size = layouter.font_size_pt * heading_scale(*level);
    let prev = layouter.style.clone();
    layouter.style = TextStyle { bold: true, italic: false, size: heading_size, ..TextStyle::default() };
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin { layouter.flush_line(); }

    // Prefixo numérico — apenas se numbering estiver activo.
    // F-5a de-bake (P364, §3a.9): o gate vive **só na chain**
    // (`#set heading(numbering:)` → `custom`, transportado por
    // `Content::Styled`); lido aqui de `layouter.chain`. O campo assado
    // `numbering_active` foi removido. O **valor** do contador segue
    // via Introspector (`formatted_counter_at`, P335 incondicional).
    let numbering_on = matches!(
        layouter.chain.custom("heading.numbering"),
        Some(crate::entities::value::Value::Bool(true)),
    );
    if numbering_on {
        let num_str = layouter.current_location
            .and_then(|loc| layouter.introspector
                .formatted_counter_at("heading", loc));
        if let Some(num_str) = num_str {
            let prefix = Content::text(format!("{}. ", num_str));
            layouter.layout_content(&prefix);
        }
    }

    layouter.layout_content(body);
    layouter.flush_line();
    layouter.style = prev;
}
