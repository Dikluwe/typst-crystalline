//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash 2be1ad5d
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P380): o layout de `Terms` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::elements::terms::TermsElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de uma lista de definições (`/ termo: descrição`): flush + itera os
/// itens, delegando cada um ao `layout_content`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TermsElem,
) {
    if layouter.regions.current.cursor_x.0 > layouter.page_config.margin {
        layouter.flush_line();
    }
    for item in e.items.iter() {
        layouter.layout_content(item);
    }
}
