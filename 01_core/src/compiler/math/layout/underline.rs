//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/underline.md
//! @prompt-hash 6badab91
//! @layer L1
//! @updated 2026-09-01
//!
//! Geometria do underline matemático derivada das constantes OpenType MATH.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    elements::math_underline::{MathOverlineElem, MathUnderlineElem},
    layout_types::{FrameItem, Point, Pt, TextStyle},
};

use super::MathBox;

/// Dispõe o body no estilo math corrente e acrescenta a regra inferior.
pub(super) fn layout<M: FontMetrics>(
    layouter: &super::MathLayouter<'_, M>,
    elem: &MathUnderlineElem,
    style: &TextStyle,
) -> MathBox {
    let mut body = layouter.layout_node(&elem.body, style);
    let size = style.size;
    let gap = layouter
        .constants
        .to_pt(layouter.constants.underbar_vertical_gap, size)
        .val();
    let thickness = layouter
        .constants
        .to_pt(layouter.constants.underbar_rule_thickness, size)
        .val();
    let extra_descender = layouter
        .constants
        .to_pt(layouter.constants.underbar_extra_descender, size)
        .val();

    // O vanilla reduz a largura da regra pela correção itálica do body.
    // Uma correção negativa, portanto, alarga a regra sem alterar a caixa.
    let italics_correction = body
        .items
        .iter()
        .rev()
        .find_map(|item| match item {
            FrameItem::Glyph { glyph_id, size, style, .. } => {
                Some(layouter.metrics.italics_correction(*glyph_id, *size, style).val())
            }
            FrameItem::TextShaped { glyphs, style, .. } => glyphs.last().map(|glyph| {
                layouter
                    .metrics
                    .italics_correction(glyph.glyph_id, style.size, style)
                    .val()
            }),
            FrameItem::Text { text, style, .. } => text.chars().last().map(|c| {
                layouter.metrics.char_italics_correction(c, style.size, style).val()
            }),
            FrameItem::TextReplacement { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Semantic { .. }
            | FrameItem::Line { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. } => None,
        })
        .unwrap_or(0.0);
    let line_width = (body.width - italics_correction).max(0.0);
    let line_y = body.descent + gap + thickness / 2.0;
    body.items.push(FrameItem::Line {
        start: Point { x: Pt(0.0), y: Pt(line_y) },
        end: Point { x: Pt(line_width), y: Pt(line_y) },
        thickness,
        color: style.fill.clone(),
        cap: crate::entities::geometry::LineCap::Butt,
    });
    body.descent += gap + thickness + extra_descender;
    body
}

/// Dispõe o body em cramped style e acrescenta a regra superior MATH.
pub(super) fn layout_overline<M: FontMetrics>(
    layouter: &super::MathLayouter<'_, M>,
    elem: &MathOverlineElem,
    style: &TextStyle,
) -> MathBox {
    let cramped_style = TextStyle { cramped: true, ..style.clone() };
    let mut body = layouter.layout_node(&elem.body, &cramped_style);
    let size = style.size;
    let gap = layouter
        .constants
        .to_pt(layouter.constants.overbar_vertical_gap, size)
        .val();
    let thickness = layouter
        .constants
        .to_pt(layouter.constants.overbar_rule_thickness, size)
        .val();
    let extra_ascender = layouter
        .constants
        .to_pt(layouter.constants.overbar_extra_ascender, size)
        .val();

    let line_y = -(body.ascent + gap + thickness / 2.0);
    body.items.push(FrameItem::Line {
        start: Point { x: Pt(0.0), y: Pt(line_y) },
        end: Point { x: Pt(body.width), y: Pt(line_y) },
        thickness,
        color: style.fill.clone(),
        cap: crate::entities::geometry::LineCap::Butt,
    });
    body.ascent += gap + thickness + extra_ascender;
    body
}
