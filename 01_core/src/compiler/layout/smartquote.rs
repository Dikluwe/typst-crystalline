//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/smartquote.md
//! @prompt-hash d4602ae9
//! @layer L1
//! @updated 2026-10-06
//!
//! Atomização (ADR-0109, P381): o layout de `SmartQuote` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//! Contexto lógico compartilhado, acedido por descendência de módulo.

use crate::entities::elements::smartquote::SmartQuoteElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Resolve a aspa semântica sob o estilo efetivo, emitindo glyph indivisível.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &SmartQuoteElem,
) {
    let previous_style = layouter.style.clone();
    layouter.style = super::text::effective_style(layouter);
    match crate::compiler::lang::quotes::resolve_smart_quote(
        &mut layouter.smartquote_context,
        e,
        &layouter.chain,
        layouter.style.lang.as_ref(),
    ) {
        Ok(glyph) => layouter.layout_word(&glyph),
        Err(errors) => layouter.layout_errors.extend(errors),
    }
    layouter.style = previous_style;
}
