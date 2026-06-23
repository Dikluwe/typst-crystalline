//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3b19c546
//! @layer L1
//! @updated 2026-06-23
//!
//! Atomização (ADR-0109, P381): o layout de `Cite` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//!
//! **P418** — renderização CSL via cache pré-computado. Fallback local por
//! `form` quando o cache não contém a key.

use crate::entities::citation_form::CitationForm;
use crate::entities::content::Content;
use crate::entities::elements::cite::CiteElem;
use crate::entities::introspector::Introspector;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `cite(...)`: usa cache CSL se disponível; senão fallback por form.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &CiteElem,
) {
    let form = e.form.unwrap_or_default();

    if let Some(content) = layouter
        .bib_render_cache
        .as_ref()
        .and_then(|c| c.citations.get(&form))
        .and_then(|m| m.get(&e.key))
        .cloned()
    {
        layouter.layout_content(&content);
        if let Some(s) = &e.supplement {
            layouter.layout_content(s);
        }
        return;
    }

    // Fallback local P418 (mantém compatibilidade com tests antigos).
    let key = &e.key;
    let entry = layouter.introspector.bib_entry_for_key(key);
    let text = match (form, entry) {
        (CitationForm::Normal, _) => layouter
            .introspector
            .bib_number_for_key(key)
            .map(|n| format!("[{}]", n))
            .unwrap_or_else(|| format!("[{}]", key)),
        (CitationForm::Prose, Some(e)) => format!("{} ({})", e.author, e.year),
        (CitationForm::Author, Some(e)) => e.author.clone(),
        (CitationForm::Year, Some(e)) => e.year.to_string(),
        (_, None) => format!("[{}]", key),
    };
    layouter.layout_content(&Content::text(text));
    if let Some(s) = &e.supplement {
        layouter.layout_content(s);
    }
}
