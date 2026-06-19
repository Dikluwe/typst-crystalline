//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/atomizacao_elementos.md
//! @prompt-hash cedb58ca
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Cite` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::citation_form::CitationForm;
use crate::entities::content::Content;
use crate::entities::elements::cite::CiteElem;
use crate::entities::introspector::Introspector;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `cite(...)`: render placeholder por `form` — número `[N]` (Normal,
/// via Introspector), `autor (ano)` (Prose), autor, ou ano. Mais supplement.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &CiteElem,
) {
    let key = &e.key;
    let supplement = &e.supplement;
    let form = &e.form;
    // P190B (M6) — consumer Introspector path completo (fallback legacy removido).
    let resolved_form = form.unwrap_or_default();
    let entry = layouter.introspector.bib_entry_for_key(key);
    let text = match (resolved_form, entry) {
        (CitationForm::Normal, _) => {
            // P190B: Introspector path apenas — sem fallback legacy.
            layouter.introspector
                .bib_number_for_key(key)
                .map(|n| format!("[{}]", n))
                .unwrap_or_else(|| format!("[{}]", key))
        }
        (CitationForm::Prose,  Some(e))   => format!("{} ({})", e.author, e.year),
        (CitationForm::Author, Some(e))   => e.author.clone(),
        (CitationForm::Year,   Some(e))   => e.year.to_string(),
        (_, None)                         => format!("[{}]", key),
    };
    layouter.layout_content(&Content::text(text));
    if let Some(s) = supplement {
        layouter.layout_content(s);
    }
}
