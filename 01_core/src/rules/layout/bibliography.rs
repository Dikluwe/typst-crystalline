//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3b19c546
//! @layer L1
//! @updated 2026-06-23
//!
//! Atomização (ADR-0109, P381): o layout de `Bibliography` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//!
//! **P418** — renderização CSL via cache pré-computado em
//! `layout_with_introspector`. Fallback para `format_bib_entry` quando o cache
//! não está disponível (ex: style inválido ou entries vazias).

use crate::entities::content::Content;
use crate::entities::elements::bibliography::BibliographyElem;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `bibliography(...)`: renderiza o title (se presente) e a lista de
/// referências. Com cache CSL disponível, usa a bibliografia pré-renderizada;
/// senão usa o fallback local `format_bib_entry`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    b: &BibliographyElem,
) {
    if let Some(t) = &b.title {
        layouter.layout_content(t);
        layouter.flush_line();
    }

    if let Some(bib_content) = layouter
        .bib_render_cache
        .as_ref()
        .and_then(|c| c.bibliography.clone())
    {
        layouter.layout_content(&bib_content);
        return;
    }

    // Fallback P418: formatação local simples quando CSL não produz output.
    for e in &b.entries {
        let line = super::format_bib_entry(e);
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}
