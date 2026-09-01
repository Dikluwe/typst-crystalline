//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/smartquote.md
//! @prompt-hash 8c7c6a0f
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `SmartQuote` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Estado dedicado (`smartquote_*_open`) acedido por descendência de módulo.

use crate::entities::elements::smartquote::SmartQuoteElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma aspa inteligente: alterna open/close per-documento (lang-aware
/// para duplas via `localize_quotes`; simples sempre ASCII). Emite o glyph
/// directo via `layout_word` (unidade indivisível — preserva NBSP de FR).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &SmartQuoteElem,
) {
    let alternative = e.alternative.unwrap_or_else(|| {
        match layouter.chain.custom("smartquote.alternative") {
            Some(crate::entities::value::Value::Bool(value)) => *value,
            _ => false,
        }
    });
    let inherited_quotes = layouter.chain.custom("smartquote.quotes").and_then(|value| {
        crate::compiler::lang::quotes::parse_smartquote_quotes(
            value,
            crate::entities::span::Span::detached(),
        )
        .ok()
    });
    let quotes = e.quotes.as_ref().or(inherited_quotes.as_ref());
    let pair = crate::compiler::lang::quotes::resolve_smartquote_pair(
        layouter.style.lang.as_ref(),
        alternative,
        quotes,
        e.double,
    );
    let glyph = if e.double {
        let g = if layouter.smartquote_double_open {
            pair.open.as_str()
        } else {
            pair.close.as_str()
        };
        layouter.smartquote_double_open = !layouter.smartquote_double_open;
        g
    } else {
        let g = if layouter.smartquote_single_open {
            pair.open.as_str()
        } else {
            pair.close.as_str()
        };
        layouter.smartquote_single_open = !layouter.smartquote_single_open;
        g
    };
    layouter.layout_word(glyph);
}
