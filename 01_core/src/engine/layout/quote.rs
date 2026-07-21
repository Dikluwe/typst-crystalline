//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash a54abe5a
//! @layer L1
//! @updated 2026-06-19
//!
//! Atomização (ADR-0109, P381): o layout de `Quote` movido do monólito
//! `layout_content` para o arquivo da feature (forma B). Content-preserving.

use crate::entities::content::Content;
use crate::entities::elements::quote::QuoteElem;
use crate::entities::layout_types::Pt;

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout de `quote(...)`: aspas lang-aware (se `quotes`), body, e atribuição
/// (`— ...`). Modo bloco indenta e força nova linha; inline fica em-linha.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &QuoteElem,
) {
    use crate::engine::lang::quotes::{localize_quotes, DEFAULT_QUOTES};
    let lang = layouter.chain.lang();
    let (open, close) = if e.quotes {
        match &lang {
            Some(l) => localize_quotes(l),
            None => DEFAULT_QUOTES,
        }
    } else {
        ("", "")
    };
    if e.block {
        if layouter.regions.current.cursor_x.0 > layouter.page_config.margin {
            layouter.flush_line();
        }
        let margin_pt = Pt(layouter.page_config.margin);
        layouter.regions.current.cursor_x = margin_pt + layouter.style.size * 1.5;
        if !open.is_empty() {
            layouter.layout_content(&Content::text(open));
        }
        layouter.layout_content(&e.body);
        if !close.is_empty() {
            layouter.layout_content(&Content::text(close));
        }
        if let Some(a) = &e.attribution {
            layouter.flush_line();
            layouter.regions.current.cursor_x = margin_pt + layouter.style.size * 1.5;
            layouter.layout_content(&Content::text("— "));
            layouter.layout_content(a);
        }
        layouter.flush_line();
        layouter.regions.current.cursor_x = margin_pt;
    } else {
        if !open.is_empty() {
            layouter.layout_content(&Content::text(open));
        }
        layouter.layout_content(&e.body);
        if !close.is_empty() {
            layouter.layout_content(&Content::text(close));
        }
        if let Some(a) = &e.attribution {
            layouter.layout_content(&Content::text(" — "));
            layouter.layout_content(a);
        }
    }
}
