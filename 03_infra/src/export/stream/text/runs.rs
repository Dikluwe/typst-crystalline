//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/export/stream/text/runs.md
//! @prompt-hash feea8798
//! @layer L3
//! @updated 2026-09-29

use super::super::{FontScenario, PageContext};
use super::{push_shaped_spacing, remap_glyph_id, shaped_tr_ops, verbose_block_prefix};
use std::collections::HashMap;
use typst_core::entities::layout_types::FrameItem;

/// **P979** — dois itens fundem-se no mesmo `BT…ET` se forem ambos
/// `TextShaped` com a mesma baseline e todos os campos que afectam o
/// envelope verbose iguais (fonte, tamanho, cor, Tr/faux-bold, tracking,
//  eixos, direcção, upem). Campos irrelevantes ao envelope (lang, etc.)
/// não comparam — mas nada aqui pode mudar o output: só fundir quando é
/// garantidamente indistinguível.
fn verbose_run_compatible(a: &FrameItem, b: &FrameItem) -> bool {
    let (
        FrameItem::TextShaped {
            pos: pa,
            style: sa,
            units_per_em: ua,
            glyphs: ga,
            font: fa,
            ..
        },
        FrameItem::TextShaped {
            pos: pb,
            style: sb,
            units_per_em: ub,
            glyphs: gb,
            font: fb,
            ..
        },
    ) = (a, b)
    else {
        return false;
    };
    if ga.is_empty() || gb.is_empty() {
        return false;
    }
    if pa.y != pb.y || ua != ub || !crate::export::font_binding::same_font(fa, fb) {
        return false;
    }
    sa.size == sb.size
        && sa.fill == sb.fill
        && sa.weight == sb.weight
        && sa.tracking == sb.tracking
        && sa.bold == sb.bold
        && sa.italic == sb.italic
        && sa.font == sb.font
        && sa.variations == sb.variations
        && sa.dir == sb.dir
        && sa.math == sb.math
        && sa.math_size == sb.math_size
}

/// **P979** — fim (exclusivo) do run de texto que começa em `start`.
/// Devolve `start + 1` quando não há fusão possível. Runs só se formam
/// nos cenários com emissão por glifos (Cidfont/Multifont sem bitmap).
pub(in crate::export::stream) fn verbose_run_end(
    items: &[FrameItem],
    start: usize,
    ctx: &PageContext,
) -> usize {
    // **P1133** — itens `style.math` nunca formam run, também na produção:
    // a fronteira `TJ` inteira quantiza a posição absoluta do átomo e pode
    // alterar o render. O vanilla emite um bloco por fragmento math; prosa
    // continua a fundir por P979. O oracle conserva o mesmo split e aplica
    // depois apenas suas transformações de stream.
    if let FrameItem::TextShaped { style, .. } = &items[start] {
        if style.math {
            return start + 1;
        }
    }
    let supported = match &ctx.font_scenario {
        FontScenario::Cidfont { bitmap, .. } => bitmap.is_none(),
        FontScenario::Multifont { fonts, bindings, per_font_bitmap, .. } => {
            if let FrameItem::TextShaped { style, font, .. } = &items[start] {
                let Some(fi) = crate::export::font_binding::lookup(
                    bindings.as_ref(),
                    fonts,
                    style,
                    font.as_ref(),
                ) else {
                    return start + 1;
                };
                per_font_bitmap.get(fi).and_then(|b| b.as_ref()).is_none()
            } else {
                false
            }
        }
        FontScenario::Type1 => false,
    };
    if !supported || !matches!(items[start], FrameItem::TextShaped { .. }) {
        return start + 1;
    }
    let mut end = start + 1;
    while end < items.len() && verbose_run_compatible(&items[end - 1], &items[end]) {
        end += 1;
    }
    end
}

/// **P979** — emissão de um run fundido (≥2 itens): um só envelope, um só
/// array `TJ`; os gaps entre itens são ajustes `TJ` computados das
/// posições (com correcção do arredondamento acumulado — o cursor segue a
/// aritmética exacta do renderer, avanço a avanço).
pub(in crate::export::stream) fn emit_verbose_text_run(
    ops: &mut String,
    run: &[FrameItem],
    page_height: f64,
    ctx: &PageContext,
) {
    let FrameItem::TextShaped { pos: pos0, style, font, .. } = &run[0] else { return };
    let base_y = page_height - pos0.y.val();
    verbose_block_prefix(ops, pos0.x.val(), base_y, &style.fill);
    ops.push_str(&shaped_tr_ops(style, font.as_ref()));
    push_shaped_spacing(ops);
    match &ctx.font_scenario {
        FontScenario::Cidfont { glyph_mapping, glyph_to_nominal, .. } => {
            ops.push_str(&format!(
                "/F1 {:.1} Tf
",
                style.size.val()
            ));
            ops.push_str(
                "1 0 0 -1 0 0 Tm
[ ",
            );
            push_run_tj_entries(ops, run, glyph_mapping, glyph_to_nominal);
            ops.push_str(
                "] TJ
ET
Q
",
            );
        }
        FontScenario::Multifont {
            fonts,
            bindings,
            per_font_glyph_mapping,
            per_font_glyph_to_nominal,
            ..
        } => {
            let Some(fi) = crate::export::font_binding::lookup(
                bindings.as_ref(),
                fonts,
                style,
                font.as_ref(),
            ) else {
                return;
            };
            ops.push_str(&format!(
                "/F{} {:.1} Tf
",
                fi + 1,
                style.size.val()
            ));
            ops.push_str(
                "1 0 0 -1 0 0 Tm
[ ",
            );
            push_run_tj_entries(
                ops,
                run,
                &per_font_glyph_mapping[fi],
                &per_font_glyph_to_nominal[fi],
            );
            ops.push_str(
                "] TJ
ET
Q
",
            );
        }
        FontScenario::Type1 => unreachable!("runs Type1 não se formam — verbose_run_end"),
    }
}

/// **P979** — entries `TJ` de um run fundido. Dentro de cada item: o
/// delta model de P486/P520/P548 (idem `push_tj_glyph_entries`). Na
/// fronteira entre itens: um ajuste extra que leva o ponto de texto
/// exactamente ao `pos.x` do item seguinte — computado contra o cursor
/// **com o arredondamento aplicado pelo renderer** (cada ajuste TJ é um
/// inteiro em milésimos de em), para que a deriva de arredondamento nunca
/// acumule além de um quantum por fronteira.
fn push_run_tj_entries(
    ops: &mut String,
    run: &[FrameItem],
    glyph_mapping: &HashMap<u16, u16>,
    glyph_to_nominal: &HashMap<u16, i32>,
) {
    let mut cursor: Option<f64> = None;
    for item in run {
        let FrameItem::TextShaped { pos, glyphs, style, units_per_em, .. } = item else {
            continue;
        };
        let size = style.size.val();
        let upm = (*units_per_em).max(1) as f64;
        if let Some(cur) = cursor {
            // Número TJ = milésimos de em SUBTRAÍDOS da posição: para levar
            // o ponto de `cur` a `pos.x`, o número é (cur − pos.x)/size×1000.
            let fractional =
                style.tracking.map(|t| t.resolve_pt(size) != 0.0).unwrap_or(false);
            let scale = if fractional { 100000.0 } else { 1.0 };
            let boundary =
                (((cur - pos.x.val()) / size * 1000.0) * scale).round() / scale;
            if boundary != 0.0 {
                if fractional {
                    ops.push_str(&format!("{boundary:.5} "));
                } else {
                    ops.push_str(&format!("{boundary:.0} "));
                }
                cursor = Some(cur - boundary / 1000.0 * size);
            }
        } else {
            cursor = Some(pos.x.val());
        }
        for (i, g) in glyphs.iter().enumerate() {
            if g.x_offset != 0 {
                let xoff_tu = -(g.x_offset as f64 / upm * 1000.0);
                ops.push_str(&format!("{:.0} ", xoff_tu));
            }
            let nominal =
                glyph_to_nominal.get(&g.glyph_id).copied().unwrap_or(g.x_advance);
            let residual = crate::shaper::tracking::residual(glyphs, i, style, upm);
            let residual_tu =
                if residual == 0.0 { 0.0 } else { residual / size * 1000.0 };
            let integer_tu = ((nominal - g.x_advance) as f64 / upm * 1000.0).round();
            let advance_tu = if residual_tu == 0.0 {
                integer_tu
            } else {
                ((integer_tu - residual_tu) * 100000.0).round() / 100000.0
            };
            let new_gid = if glyph_mapping.is_empty() {
                g.glyph_id
            } else {
                remap_glyph_id(g.glyph_id, glyph_mapping)
            };
            if residual_tu == 0.0 {
                ops.push_str(&format!("<{:04X}> {:.0} ", new_gid, advance_tu));
            } else {
                ops.push_str(&format!("<{:04X}> {:.5} ", new_gid, advance_tu));
            }
            // Avanço tal como o renderer o aplica (com o ajuste arredondado).
            let applied = (nominal as f64 / upm - advance_tu / 1000.0) * size;
            cursor = cursor.map(|c| c + applied);
        }
    }
}
