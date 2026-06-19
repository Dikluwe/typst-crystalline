//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 03f97ccc
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `Colbreak` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::colbreak::ColbreakElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `colbreak(...)`: downgrade a pagebreak literal (Opção β graded —
/// salto entre colunas reais é P-Layout-Fase4). `_e` ignorado (`weak` diferido).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    _e:       &ColbreakElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0 {
        layouter.flush_line();
    }
    layouter.new_page();
}
