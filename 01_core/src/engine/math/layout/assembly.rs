//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/assembly.md
//! @prompt-hash 8e9c3d48
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_assembly` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::engine::layout::FontMetrics;
use crate::entities::layout_types::{FrameItem, Point, Pt, TextStyle};

use super::MathBox;
use crate::entities::glyph_variants::{GlyphAssembly, GlyphPart};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_assembly(
        &self,
        c: char,
        assembly: GlyphAssembly,
        target_advance_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        if assembly.is_empty() {
            let text: ecow::EcoString = c.to_string().into();
            return self.layout_text_node(&text, style);
        }

        let scale = style.size.val() / self.constants.upem;
        let target_pt = target_advance_du * scale;

        // P913 — Algoritmo do vanilla para determinar o número de repetições
        // (`repeat`) das peças extensoras (`is_extender == true`) e a razão de
        // espalhamento (`ratio`).
        const MAX_REPEATS: usize = 1024;
        let mut full_pt;
        let mut ratio = 0.0_f64;
        let mut repeat = 0_usize;

        loop {
            full_pt = 0.0;
            ratio = 0.0;

            let mut parts_vec: Vec<&GlyphPart> = Vec::new();
            for part in &assembly.parts {
                let count = if part.is_extender { repeat } else { 1 };
                for _ in 0..count {
                    parts_vec.push(part);
                }
            }

            let mut growable_pt = 0.0_f64;
            let n = parts_vec.len();
            for i in 0..n {
                let part = parts_vec[i];
                let advance_pt = part.full_advance as f64 * scale;
                let mut advance = advance_pt;
                if i + 1 < n {
                    let next = parts_vec[i + 1];
                    let max_overlap =
                        (part.end_connector as f64).min(next.start_connector as f64) * scale;
                    advance -= max_overlap;
                    growable_pt += max_overlap;
                }
                full_pt += advance;
            }

            if full_pt < target_pt && growable_pt > 0.0 {
                let delta = target_pt - full_pt;
                ratio = (delta / growable_pt).min(1.0);
                full_pt += ratio * growable_pt;
            }

            if target_pt <= full_pt || repeat >= MAX_REPEATS {
                break;
            }

            repeat += 1;
        }

        let mut parts_vec: Vec<&GlyphPart> = Vec::new();
        for part in &assembly.parts {
            let count = if part.is_extender { repeat } else { 1 };
            for _ in 0..count {
                parts_vec.push(part);
            }
        }

        let mut items = Vec::new();
        let mut max_advance = 0.0_f64;
        let mut piece_positions: Vec<(f64, u16, f64)> = Vec::new();

        let mut y_cursor = 0.0_f64;
        let n = parts_vec.len();
        for i in 0..n {
            let part = parts_vec[i];
            let advance_pt = part.full_advance as f64 * scale;

            let overlap = if i + 1 < n {
                let next = parts_vec[i + 1];
                let max_overlap =
                    (part.end_connector as f64).min(next.start_connector as f64) * scale;
                max_overlap - ratio * max_overlap
            } else {
                0.0
            };

            piece_positions.push((y_cursor, part.glyph_id, advance_pt));
            max_advance = max_advance.max(advance_pt);
            y_cursor += advance_pt - overlap;
        }
        let total_height = y_cursor;

        for (y_from_bottom, glyph_id, x_advance_val) in piece_positions {
            let y_in_box = total_height - y_from_bottom - x_advance_val.min(total_height);
            items.push(FrameItem::Glyph {
                pos: Point {
                    x: Pt(0.0),
                    y: Pt(y_in_box.max(0.0)),
                },
                glyph_id,
                x_advance: Pt(x_advance_val),
                size: style.size,
                style: style.clone(),
                base_char: c,
            });
        }

        MathBox {
            width: max_advance,
            ascent: total_height,
            descent: 0.0,
            items,
        }
    }

    pub(super) fn layout_assembly_horizontal(
        &self,
        c: char,
        assembly: GlyphAssembly,
        target_advance_du: f64,
        style: &TextStyle,
    ) -> MathBox {
        if assembly.is_empty() {
            let text: ecow::EcoString = c.to_string().into();
            return self.layout_text_node(&text, style);
        }

        let scale = style.size.val() / self.constants.upem;
        let target_pt = target_advance_du * scale;

        const MAX_REPEATS: usize = 1024;
        let mut full_pt;
        let mut ratio = 0.0_f64;
        let mut repeat = 0_usize;

        loop {
            full_pt = 0.0;
            ratio = 0.0;

            let mut parts_vec: Vec<&GlyphPart> = Vec::new();
            for part in &assembly.parts {
                let count = if part.is_extender { repeat } else { 1 };
                for _ in 0..count {
                    parts_vec.push(part);
                }
            }

            let mut growable_pt = 0.0_f64;
            let n = parts_vec.len();
            for i in 0..n {
                let part = parts_vec[i];
                let advance_pt = part.full_advance as f64 * scale;
                let mut advance = advance_pt;
                if i + 1 < n {
                    let next = parts_vec[i + 1];
                    let max_overlap =
                        (part.end_connector as f64).min(next.start_connector as f64) * scale;
                    advance -= max_overlap;
                    growable_pt += max_overlap;
                }
                full_pt += advance;
            }

            if full_pt < target_pt && growable_pt > 0.0 {
                let delta = target_pt - full_pt;
                ratio = (delta / growable_pt).min(1.0);
                full_pt += ratio * growable_pt;
            }

            if target_pt <= full_pt || repeat >= MAX_REPEATS {
                break;
            }

            repeat += 1;
        }

        let mut parts_vec: Vec<&GlyphPart> = Vec::new();
        for part in &assembly.parts {
            let count = if part.is_extender { repeat } else { 1 };
            for _ in 0..count {
                parts_vec.push(part);
            }
        }

        let mut items = Vec::new();
        let mut x_cursor = 0.0_f64;

        let n = parts_vec.len();
        for i in 0..n {
            let part = parts_vec[i];
            let advance_pt = part.full_advance as f64 * scale;
            let x_advance = Pt(advance_pt);

            items.push(FrameItem::Glyph {
                pos: Point {
                    x: Pt(x_cursor),
                    y: Pt(0.0),
                },
                glyph_id: part.glyph_id,
                x_advance,
                size: style.size,
                style: style.clone(),
                base_char: c,
            });

            let overlap = if i + 1 < n {
                let next = parts_vec[i + 1];
                let max_overlap =
                    (part.end_connector as f64).min(next.start_connector as f64) * scale;
                max_overlap - ratio * max_overlap
            } else {
                0.0
            };
            x_cursor += advance_pt - overlap;
        }

        let (ascent, _) = self.metrics.vertical_metrics(style.size, style);

        MathBox {
            width: x_cursor,
            ascent: ascent.val(),
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
