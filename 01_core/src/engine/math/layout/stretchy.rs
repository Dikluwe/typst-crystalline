//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/stretchy.md
//! @prompt-hash ac81e392
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_stretchy_delimiter` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::engine::layout::FontMetrics;
use crate::entities::layout_types::{FrameItem, Point, TextStyle};

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_stretchy_delimiter(
        &self,
        c: char,
        min_height_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        let variants = self.metrics.vertical_glyph_variants(c, style);

        // P912: subtrair DELIM_SHORT_FALL = 0.1em (0.1 * upem em design units) da dimensão alvo
        let short_fall_du = 0.1 * self.constants.upem;
        let target_du = (min_height_du - short_fall_du).max(0.0);

        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();

        if let Some((glyph_id, advance_du)) = variants.select_with_advance(target_du)
        {
            let height_pt = style.size.val() * (advance_du / self.constants.upem);
            let half_h = height_pt / 2.0;
            let ascent = axis_pt + half_h;
            let descent = (half_h - axis_pt).max(0.0);
            let shift_y = axis_pt - half_h;

            // Variante encontrada
            if let Some(mapped_char) = self.metrics.glyph_to_char(glyph_id) {
                // Mapeamento Unicode disponível — emitir como Text
                let text: ecow::EcoString = mapped_char.to_string().into();
                let mut b = self.layout_text_node(&text, style);
                b.ascent = ascent;
                b.descent = descent;
                b.items = b
                    .items
                    .into_iter()
                    .map(|item| super::offset_item(item, crate::entities::layout_types::Pt(0.0), crate::entities::layout_types::Pt(shift_y)))
                    .collect();
                return b;
            } else {
                // Sem mapeamento — emitir como Glyph
                let x_advance = style.size * (advance_du / self.constants.upem);
                return MathBox {
                    width: x_advance.val(),
                    ascent,
                    descent,
                    items: vec![FrameItem::Glyph {
                        pos: Point {
                            x: crate::entities::layout_types::Pt(0.0),
                            y: crate::entities::layout_types::Pt(shift_y),
                        },
                        glyph_id,
                        x_advance,
                        size: style.size,
                        style: style.clone(),
                        base_char: c,
                    }],
                };
            }
        }

        // Nenhuma variante suficiente — tentar GlyphAssembly
        let assembly = self.metrics.vertical_glyph_assembly(c, style);
        if !assembly.is_empty() {
            return self.layout_assembly(c, assembly, min_height_du, style);
        }

        // Fallback: glifo base
        let text: ecow::EcoString = c.to_string().into();
        self.layout_text_node(&text, style)
    }

    /// **P906** — esticamento no eixo X (`underbrace`/`overbrace`/
    /// `underbracket`/`overbracket`, acentos largos). Espelha
    /// `layout_stretchy_delimiter` acima, trocando `vertical_glyph_variants`/
    /// `vertical_glyph_assembly` por `horizontal_glyph_variants`/
    /// `horizontal_glyph_assembly` e `min_height_du` por `min_width_du` — ver
    /// `math/layout/stretchy.md` §P906.
    pub(super) fn layout_stretchy_glyph_horizontal(
        &self,
        c: char,
        min_width_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        let variants = self.metrics.horizontal_glyph_variants(c, style);

        // P912: subtrair DELIM_SHORT_FALL = 0.1em (0.1 * upem em design units) da dimensão alvo
        let short_fall_du = 0.1 * self.constants.upem;
        let target_du = (min_width_du - short_fall_du).max(0.0);

        if let Some((glyph_id, advance_du)) = variants.select_with_advance(target_du)
        {
            // Variante encontrada
            if let Some(mapped_char) = self.metrics.glyph_to_char(glyph_id) {
                // Mapeamento Unicode disponível — emitir como Text
                let text: ecow::EcoString = mapped_char.to_string().into();
                return self.layout_text_node(&text, style);
            } else {
                // Sem mapeamento — emitir como Glyph
                let x_advance = style.size * (advance_du / self.constants.upem);
                let (ascent, _) = self.metrics.vertical_metrics(style.size, style);
                return MathBox {
                    width: x_advance.val(),
                    ascent: ascent.val(),
                    descent: 0.0,
                    items: vec![FrameItem::Glyph {
                        pos: Point::ZERO,
                        glyph_id,
                        x_advance,
                        size: style.size,
                        style: style.clone(),
                        base_char: c,
                    }],
                };
            }
        }

        // Nenhuma variante suficiente — tentar GlyphAssembly
        let assembly = self.metrics.horizontal_glyph_assembly(c, style);
        if !assembly.is_empty() {
            return self.layout_assembly_horizontal(c, assembly, min_width_du, style);
        }

        // Fallback: glifo base
        let text: ecow::EcoString = c.to_string().into();
        self.layout_text_node(&text, style)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
