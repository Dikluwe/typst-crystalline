//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/heading.md
//! @prompt-hash 41fd8b3d
//! @layer L1
//! @updated 2026-06-24
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
        let heading_key = CounterKey::Selector(Selector::Kind(ElementKind::Heading));
        // **P1036** — duas vias, cada uma com o seu separador:
        //
        // - **com pattern**: `format_counter` sobre os valores hierárquicos
        //   completos. O resultado usa-se **verbatim** — o pattern já
        //   transporta toda a pontuação ("1.", "(a)", "1.1") — e o separador
        //   para o body é um espaço simples. A heurística anterior
        //   (`used_pattern`, que recomputava `values.len() >= nº de tokens`)
        //   desapareceu: com `"1.1"` e um só valor emitia `1. Alpha` onde o
        //   vanilla emite `1 Alpha`.
        // - **sem pattern**: forma legada `formatted_counter_at` ("1.2.3") com
        //   o separador histórico ". ".
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
    layouter.flush_line();
    layouter.style = prev;
}
