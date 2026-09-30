//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/separator.md
//! @prompt-hash 3e5c6f19
//! @layer L3
//! @updated 2026-09-28

use super::{emit_text_pdf, emit_text_pdf_verbose, PageContext, StreamMode};
use typst_core::entities::layout_types::TextStyle;

pub(super) fn emit(ops: &mut String, x: f64, pdf_y: f64, style: &TextStyle, ctx: &PageContext) {
    match ctx.mode {
        StreamMode::Compact => emit_text_pdf(ops, x, pdf_y, " ", style, &ctx.font_scenario),
        StreamMode::Verbose => emit_text_pdf_verbose(ops, x, pdf_y, " ", style, &ctx.font_scenario),
    }
}
