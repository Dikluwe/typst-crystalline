//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash 3331d6ba
//! @layer L1
//! @updated 2026-06-23
//!
//! Atomização (ADR-0109, P381): o layout de `Cite` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.
//!
//! **P418** — renderização CSL via cache pré-computado. Fallback local por
//! `form` quando o cache não contém a key.

use crate::entities::citation_form::CitationForm;
use crate::entities::citation_style::CitationStyle;
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
    // P468: default numérico para bibliografia; None ↔ Numeric fallback.
    let style = e.style.unwrap_or_default();

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

    // Fallback local P418/P468.
    let key = &e.key;
    let entry = layouter.introspector.bib_entry_for_key(key);
    // P468: número de citação — citation_number (order of first appearance) se
    // disponível; caso contrário bib_number (legado assign_number); fallback [key].
    let numeric_n = |intr: &_| -> String {
        crate::entities::introspector::Introspector::citation_number_for_key(intr, key)
            .or_else(|| crate::entities::introspector::Introspector::bib_number_for_key(intr, key))
            .map(|n| format!("[{}]", n))
            .unwrap_or_else(|| format!("[{}]", key))
    };
    let text = match (style, form, entry) {
        // Normal+Numeric: só usa número se a entry bibliográfica existe.
        (CitationStyle::Numeric, CitationForm::Normal, Some(_)) => numeric_n(&*layouter.introspector),
        (CitationStyle::Numeric, CitationForm::Normal, None) => format!("[{}]", key),
        (CitationStyle::Numeric, CitationForm::Prose, Some(e)) => {
            let n = crate::entities::introspector::Introspector::citation_number_for_key(
                &*layouter.introspector, key,
            )
            .or_else(|| crate::entities::introspector::Introspector::bib_number_for_key(
                &*layouter.introspector, key,
            ))
            .map(|n| n.to_string())
            .unwrap_or_else(|| key.clone());
            format!("{} [{}]", e.author, n)
        }
        (CitationStyle::Numeric, CitationForm::Author, Some(e)) => e.author.clone(),
        (CitationStyle::Numeric, CitationForm::Year, Some(e)) => e.year.to_string(),
        (_, CitationForm::Normal, _) => format!("[{}]", key),
        (_, CitationForm::Prose, Some(e)) => format!("{} ({})", e.author, e.year),
        (_, CitationForm::Author, Some(e)) => e.author.clone(),
        (_, CitationForm::Year, Some(e)) => e.year.to_string(),
        (_, _, None) => format!("[{}]", key),
    };
    layouter.layout_content(&Content::text(text));
    if let Some(s) = &e.supplement {
        layouter.layout_content(s);
    }
}
