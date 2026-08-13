//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/bibliography.md
//! @prompt-hash 55c89e3a
//! @layer L1
//! @updated 2026-06-25
//!
//! Atomização (ADR-0109, P381): o layout de `Bibliography` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//!
//! **P418** — renderização CSL via cache pré-computado em
//! `layout_with_introspector`. Fallback para `format_bib_entry` quando o cache
//! não está disponível (ex: style inválido ou entries vazias).

use crate::entities::content::Content;
use crate::entities::elements::bibliography::BibliographyElem;
use crate::entities::introspector::Introspector;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `bibliography(...)`: renderiza o title (se presente) e a lista de
/// referências. Com cache CSL disponível, usa a bibliografia pré-renderizada;
/// senão usa o fallback local `format_bib_entry`.
///
/// **P468** — fallback reordena entries por `citation_order` e prefixa com `[N]`.
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

    // Fallback P418/P468: reordena entries por ordem de primeira citação.
    // Entries não citadas ficam no final (na ordem original).
    let citation_order: Vec<String> = layouter.introspector.citation_order().to_vec();
    let mut ordered: Vec<_> = b.entries.iter().collect();
    ordered.sort_by_key(|e| {
        citation_order.iter().position(|k| k == &e.key).unwrap_or(usize::MAX)
    });
    for (idx, e) in ordered.iter().enumerate() {
        let n = idx + 1;
        let body = super::format_bib_entry_body(e);
        // **P472** — back-refs: lista de posições onde a entry foi citada.
        let refs = layouter.introspector.back_refs_for_key(&e.key);
        let back_ref_str = if refs.is_empty() {
            String::new()
        } else {
            let cited: String =
                refs.iter().map(|p| format!("[{}]", p)).collect::<Vec<_>>().join("");
            format!(" ↑{}", cited)
        };
        let line = format!("[{}] {}{}", n, body, back_ref_str);
        layouter.layout_content(&Content::text(line));
        layouter.flush_line();
    }
}
