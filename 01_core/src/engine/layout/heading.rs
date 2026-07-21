//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt 00_nucleo/prompts/engine/layout/heading.md
//! @prompt-hash ac6633ea
//! @layer L1
//! @updated 2026-06-24
//!
//! Atomização (ADR-0109, P377): o layout de `Heading` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). P451: suporte a patterns de numeração configuráveis.

use ecow::EcoString;

use crate::entities::content::Content;
use crate::entities::counter_format::format_counter;
use crate::entities::elements::heading::HeadingElem;
use crate::entities::introspector::Introspector;
use crate::entities::layout_types::TextStyle;
use crate::entities::value::Value;

use super::helpers::heading_scale;
use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de um cabeçalho (`= Heading`): estilo bold escalado por nível,
/// prefixo numérico opcional (gate na chain, valor via Introspector), body.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    h: &HeadingElem,
) {
    // Modelo D (P316): Heading delegado; re-bind dos campos.
    let level = &h.level;
    let body = &h.body;

    let heading_size = layouter.style.size * heading_scale(*level);
    let prev = layouter.style.clone();
    layouter.style = TextStyle {
        bold: true,
        italic: false,
        size: heading_size,
        ..TextStyle::default()
    };
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin {
        layouter.flush_line();
    }

    // Prefixo numérico — apenas se numbering estiver activo.
    // F-5a de-bake (P364, §3a.9): o gate vive **só na chain**
    // (`#set heading(numbering:)` → `custom`, transportado por
    // `Content::Styled`); lido aqui de `layouter.chain`.
    let numbering_on =
        matches!(layouter.chain.custom("heading.numbering"), Some(Value::Bool(true)),);
    if numbering_on {
        let pattern: Option<EcoString> =
            match layouter.chain.custom("heading.numbering.pattern") {
                Some(Value::Str(s)) => Some(s.clone()),
                _ => None,
            };
        let num_str = layouter.current_location.and_then(|loc| {
            if let Some(ref pattern) = pattern {
                // P451: tenta formatar com o pattern configurável. Se o
                // pattern tiver mais tokens do que valores disponíveis,
                // faz fallback para a formatação hierárquica default.
                if let Some(values) =
                    layouter.introspector.counter_values_at("heading", loc)
                {
                    format_counter(values, pattern).map(EcoString::from).or_else(|| {
                        layouter
                            .introspector
                            .formatted_counter_at("heading", loc)
                            .map(EcoString::from)
                    })
                } else {
                    layouter
                        .introspector
                        .formatted_counter_at("heading", loc)
                        .map(EcoString::from)
                }
            } else {
                layouter
                    .introspector
                    .formatted_counter_at("heading", loc)
                    .map(EcoString::from)
            }
        });
        if let Some(num_str) = num_str {
            // P451: quando usamos um pattern configurável e `format_counter`
            // consegue formatar, o próprio pattern transporta a pontuação
            // ("1.", "(a)", "1.1", etc.) — usamos só espaço. Quando não há
            // pattern ou `format_counter` fez fallback para
            // `formatted_counter_at`, mantemos o ". " histórico.
            let used_pattern = pattern.is_some()
                && layouter
                    .current_location
                    .and_then(|loc| {
                        layouter.introspector.counter_values_at("heading", loc)
                    })
                    .map_or(false, |values| {
                        values.len()
                            >= pattern
                                .as_ref()
                                .unwrap()
                                .chars()
                                .filter(|c| matches!(*c, '1' | 'I' | 'a' | 'A'))
                                .count()
                    });
            let suffix = if used_pattern { " " } else { ". " };
            let prefix = Content::text(format!("{}{}", num_str, suffix));
            layouter.layout_content(&prefix);
        }
    }

    layouter.layout_content(body);
    layouter.flush_line();
    layouter.style = prev;
}
