//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash cedb58ca
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Bibliography` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::content::Content;
use crate::entities::elements::bibliography::BibliographyElem;

use super::{format_bib_entry, FontMetrics, ImageSizer, Layouter};

/// Layout de `bibliography(...)`: renderiza o title (se presente) e cada entry
/// formatada (`format_bib_entry`), uma por linha. Placeholder per ADR-0033.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    b:        &BibliographyElem,
) {
    if let Some(t) = &b.title {
        layouter.layout_content(t);
        layouter.flush_line();
    }
    for e in &b.entries {
        let line = format_bib_entry(e);
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}
