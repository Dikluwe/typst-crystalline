//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/page.md
//! @prompt-hash 1c19f043
//! @layer L3
//! @updated 2026-09-26

use super::{
    draw_item_top, emit_verbose_text_run, spot_fill_operator, verbose_run_end,
    PageContext, StreamMode,
};
use typst_core::entities::layout_types::FrameItem;
use typst_core::entities::layout_types::Page;

// **P281** — Stream builder unificado (substitui build_page_stream_type1/
// cidfont/multifont). Despacha Text/Glyph por `ctx.font_scenario`;
// Line/Image/Shape/Group são scenario-independent.
pub(in crate::export) fn build_page_stream(page: &Page, ctx: &PageContext) -> Vec<u8> {
    let mut ops = String::new();
    let page_height = page.height;

    if let typst_core::entities::page_canvas::PageFill::Paint(paint) = &page.fill {
        if ctx.ua1 {
            ops.push_str("/Artifact << /Type /Layout >> BDC\n");
        }
        let opacity = if let typst_core::entities::paint::Paint::Solid(c) = paint {
            super::opacity_operator(c.to_rgba_f32().3, 1.0)
        } else {
            String::new()
        };
        if !opacity.is_empty() {
            ops.push_str("q\n");
            ops.push_str(&opacity);
        }
        if let typst_core::entities::paint::Paint::Solid(color) = paint {
            if let Some(op) = spot_fill_operator(color) {
                ops.push_str(&op);
            } else {
                let (r, g, b, _) = color.to_rgba_f32();
                ops.push_str(&format!("{r:.5} {g:.5} {b:.5} rg\n"));
            }
        } else {
            let (r, g, b, _) = paint.to_color().to_rgba_f32();
            ops.push_str(&format!("{r:.5} {g:.5} {b:.5} rg\n"));
        }
        ops.push_str(&format!(
            "0 0 {:.5} {:.5} re f\n",
            page.canvas_width(),
            page.canvas_height(),
        ));
        if !opacity.is_empty() {
            ops.push_str("Q\n");
        }
        if ctx.ua1 {
            ops.push_str("EMC\n");
        }
    }
    if !page.bleed.is_zero() {
        ops.push_str(&format!(
            "q\n1 0 0 1 {:.5} {:.5} cm\n",
            page.bleed.left, page.bleed.bottom,
        ));
    }

    // **P979** — agrupamento de runs de texto (modo verbose): itens
    // `TextShaped` consecutivos com o mesmo envelope e a mesma baseline
    // fundem-se num só `BT…ET` (paridade vanilla: um bloco por linha de
    // estilo uniforme — `stream.md` §P979).
    // Cada folha UA1 conserva o próprio MCID; o agrupamento P979 não o atravessa.
    let verbose = matches!(ctx.mode, StreamMode::Verbose) && !ctx.ua1;
    emit_page_items(&mut ops, &page.background, page_height, ctx, verbose);
    emit_page_items(&mut ops, &page.items, page_height, ctx, verbose);
    emit_page_items(&mut ops, &page.foreground, page_height, ctx, verbose);

    if !page.bleed.is_zero() {
        ops.push_str("Q\n");
    }
    ops.into_bytes()
}

pub(super) fn emit_page_items(
    ops: &mut String,
    items: &[FrameItem],
    page_height: f64,
    ctx: &PageContext,
    verbose: bool,
) {
    let mut i = 0;
    while i < items.len() {
        if let Some(second) = items.get(i + 1) {
            if let Some(pair) = super::draw::joiner_pair(&items[i], second, ctx) {
                *ops = super::draw::draw_joiner_pair_top(
                    std::mem::take(ops),
                    &items[i],
                    second,
                    page_height,
                    ctx,
                    pair,
                );
                i += 2;
                continue;
            }
        }
        if verbose {
            let run_end = verbose_run_end(items, i, ctx);
            if run_end > i + 1 {
                emit_verbose_text_run(ops, &items[i..run_end], page_height, ctx);
                i = run_end;
                continue;
            }
        }
        *ops = draw_item_top(std::mem::take(ops), &items[i], page_height, ctx);
        i += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use typst_core::entities::content::Content;

    #[test]
    fn empty_page_has_no_paint_operators() {
        let images = HashMap::new();
        let patterns = HashMap::new();
        let ctx = PageContext::type1(&images, &[], &patterns, &[], StreamMode::Compact);
        let page = Page {
            width: 100.0,
            height: 100.0,
            numbering: None,
            logical_number: 1,
            supplement: Content::Empty,
            bleed: Default::default(),
            fill: Default::default(),
            background: vec![],
            foreground: vec![],
            items: vec![],
        };
        assert!(build_page_stream(&page, &ctx).is_empty());
    }
}
