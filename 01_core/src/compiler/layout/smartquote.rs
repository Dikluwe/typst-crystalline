//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 59c9666b
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
    let glyph: &str = if e.double {
        let (open, close) = match &layouter.style.lang {
            Some(l) => crate::compiler::lang::quotes::localize_quotes(l),
            None => crate::compiler::lang::quotes::DEFAULT_QUOTES,
        };
        let g = if layouter.smartquote_double_open { open } else { close };
        layouter.smartquote_double_open = !layouter.smartquote_double_open;
        g
    } else {
        // Aspas simples — paridade P155 §A.1.2: always ASCII,
        // smart-apostrophes scope-out.
        layouter.smartquote_single_open = !layouter.smartquote_single_open;
        "'"
    };
    layouter.layout_word(glyph);
}
