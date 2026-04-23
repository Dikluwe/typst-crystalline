//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math/layout.md
//! @prompt-hash 38b51727
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_assembly` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::entities::layout_types::{FrameItem, Point, Pt, TextStyle};
use crate::rules::layout::FontMetrics;

use super::MathBox;
use crate::entities::glyph_variants::GlyphAssembly;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_assembly(
        &self,
        c:              char,
        assembly:       GlyphAssembly,
        _target_advance: f64,
        style:          &TextStyle,
    ) -> MathBox {
        if assembly.is_empty() {
            let text: ecow::EcoString = c.to_string().into();
            return self.layout_text_node(&text, style);
        }

        let scale = style.size.val() / self.constants.upem;
        let mut items  = Vec::new();
        let mut max_advance  = 0.0_f64;

        // Empilhar peças de baixo para cima (bottom → top em coords do MathBox)
        // No MathBox, y=0 é o topo e y=ascent é a baseline.
        // Vamos acumular posições de baixo para cima e depois inverter.
        let mut piece_positions: Vec<(f64, u16, f64)> = Vec::new(); // (y_from_bottom, glyph_id, advance_pt)

        let mut y_cursor = 0.0_f64;
        let n = assembly.parts.len();
        for (i, part) in assembly.parts.iter().enumerate() {
            let advance_pt = part.full_advance as f64 * scale;
            let x_advance  = Pt(advance_pt);

            // Sobreposição com a peça seguinte
            let overlap = if i + 1 < n {
                let next = &assembly.parts[i + 1];
                (part.end_connector as f64).min(next.start_connector as f64) * scale
            } else {
                0.0
            };

            piece_positions.push((y_cursor, part.glyph_id, x_advance.val()));
            max_advance = max_advance.max(x_advance.val());
            y_cursor += advance_pt - overlap;
        }
        let total_height = y_cursor;

        // Converter coordenadas: y_from_bottom → y no MathBox (topo = 0)
        for (y_from_bottom, glyph_id, x_advance_val) in piece_positions {
            let y_in_box = total_height - y_from_bottom - x_advance_val.min(total_height);
            items.push(FrameItem::Glyph {
                pos:       Point { x: Pt(0.0), y: Pt(y_in_box.max(0.0)) },
                glyph_id,
                x_advance: Pt(x_advance_val),
                size:      style.size,
            });
        }

        MathBox {
            width:   max_advance,
            ascent:  total_height,
            descent: 0.0,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
