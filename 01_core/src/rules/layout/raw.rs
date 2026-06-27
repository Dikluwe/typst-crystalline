//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash e6442e3f
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Raw` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::raw::RawElem;
use crate::entities::layout_types::{Pt, TextStyle};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de código `raw` (`` `...` ``): estilo a 90% sem bold/italic; em modo
/// bloco força nova linha e indenta. Cada palavra via `layout_word`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &RawElem,
) {
    let prev = layouter.style.clone();
    // Raw: tamanho 90%, sem bold/italic
    // DEBT: seleccionar fonte monospace real quando FontBook tiver uma
    layouter.style = TextStyle { bold: false, italic: false, size: layouter.font_size_pt * 0.9, ..TextStyle::default() };
    if e.block {
        if layouter.regions.current.cursor_x.0 > layouter.page_config.margin { layouter.flush_line(); }
        layouter.regions.current.cursor_x = Pt(layouter.page_config.margin) + layouter.font_size_pt;
    }
    for word in e.text.split_whitespace() { layouter.layout_word(word); }
    if e.block { layouter.flush_line(); }
    layouter.style = prev;
}
