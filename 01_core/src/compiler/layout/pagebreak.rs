//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash bf8f0b19
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `Pagebreak` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::pagebreak::PagebreakElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `pagebreak(...)`: flush + força nova página; se `to` exige
/// paridade e a próxima página não bate, insere uma página vazia adicional.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &PagebreakElem,
) {
    // 1. Termina linha em curso (caso contrário fica meio-render).
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }
    // 2. Força nova página (mesmo se actual está vazia — vanilla
    //    pagebreak() é "event" sempre observável).
    layouter.new_page();
    // 3. Se `to` exige paridade específica, verifica; se não bate,
    //    insere página vazia adicional para ajustar.
    if let Some(parity) = e.to {
        let next_page_number = layouter.pages.len() + 1;
        if !parity.matches(next_page_number) {
            layouter.new_page();
        }
    }
}
