//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/stretchy.md
//! @prompt-hash 563c0ea7
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

        if let Some((glyph_id, advance_du)) = variants.select_with_advance(min_height_du)
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

        if let Some((glyph_id, advance_du)) = variants.select_with_advance(min_width_du)
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
