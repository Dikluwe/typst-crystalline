//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/quote.md
//! @prompt-hash c878b0d9
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
    use crate::compiler::lang::quotes::{localize_quotes, DEFAULT_QUOTES};
    let lang = layouter.chain.lang();
    // **P1055** — paridade vanilla `lab/typst-original/crates/typst-library/src/model/quote.rs:75-95`:
    // em modo bloco, aspas inteligentes padrão são suprimidas.
    // Quotes are automatic: on outside a block; a given or declared value wins.
    let quotes = if e.given & 2 != 0 { e.quotes } else { !e.block };
    let (open, close) = if quotes {
        match &lang {
            Some(l) => localize_quotes(l),
            None => DEFAULT_QUOTES,
        }
    } else {
        ("", "")
    };
    if e.block {
        // A block quote is a block: 2.4em above and 1.8em below.
        if super::html_frame::items_have_causal_line(&layouter.regions.current.current_line) {
            super::parbreak::layout(layouter);
        } else if layouter.regions.current.cursor_x.0 > layouter.page_config.margin.left {
            layouter.flush_line();
        }
        let size = layouter.style.size.0;
        if layouter.block_chain_active {
            layouter.prev_block_below_pending = layouter.prev_block_below_pending.max(2.4 * size);
        }
        let quote_indent_state =
            super::paragraph_indent::enter_fragment(layouter, &e.body);
        // A block quote is padded by 1em on both sides, every line included
        // (`model/quote.rs` show-set and `layout/rules.rs` QUOTE_RULE).
        let indent = layouter.style.size;
        let saved_start = layouter.regions.current.line_start_x;
        let saved_width = layouter.regions.current.width;
        layouter.regions.current.line_start_x = saved_start + indent;
        layouter.regions.current.width = saved_width - indent.0;
        layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
        if !open.is_empty() {
            layouter.layout_content(&Content::text(open));
        }
        layouter.layout_planned_child(
            crate::compiler::introspect::page_running::PlanEdge::QuoteBody,
            &e.body,
        );
        if !close.is_empty() {
            layouter.layout_content(&Content::text(close));
        }
        if let Some(a) = &e.attribution {
            layouter.flush_line();
            // The attribution sits 0.9em below, aligned to the end.
            // ref: lab/typst-original/crates/typst-layout/src/rules.rs:376
            let leading = layouter.style.size.0 * super::vanilla_defaults::PAR_LEADING;
            layouter.regions.current.cursor_y += Pt(0.9 * layouter.style.size.0 - leading);
            layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;
            let saved_alignment = layouter
                .explicit_line_alignment
                .replace(crate::entities::layout_types::HAlign::End);
            layouter.layout_content(&Content::text("— "));
            layouter.layout_planned_child(
                crate::compiler::introspect::page_running::PlanEdge::QuoteAttribution,
                a,
            );
            super::parbreak::layout(layouter);
            layouter.explicit_line_alignment = saved_alignment;
        }
        super::parbreak::layout(layouter);
        layouter.regions.current.line_start_x = saved_start;
        layouter.regions.current.width = saved_width;
        layouter.regions.current.cursor_x = saved_start;
        layouter.paragraph_indent = quote_indent_state;
        if layouter.block_chain_active {
            layouter.prev_block_below_pending = 1.8 * size;
        }
    } else {
        if !open.is_empty() {
            layouter.layout_content(&Content::text(open));
        }
        layouter.layout_planned_child(
            crate::compiler::introspect::page_running::PlanEdge::QuoteBody,
            &e.body,
        );
        if !close.is_empty() {
            layouter.layout_content(&Content::text(close));
        }
        // Outside a block the attribution is not shown, as in the language.
        let _ = &e.attribution;
    }
}
