//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/large_operator.md
//! @prompt-hash f15b52dd
//! @layer L1
//! @updated 2026-09-30

use ecow::EcoString;

use crate::compiler::layout::FontMetrics;
use crate::entities::layout_types::{FrameItem, Point, Pt, TextStyle};

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    /// Seleciona a construção vertical Large da fonte para Display.
    pub(super) fn layout_large_operator_display(
        &self,
        c: char,
        style: &TextStyle,
    ) -> MathBox {
        let text: EcoString = c.to_string().into();
        let base = self.layout_text_node(&text, style);

        if self.block {
            let target_du = self.constants.display_operator_min_height;
            let target_pt = self.constants.to_pt(target_du, style.size).val();
            if base.ascent + base.descent >= target_pt {
                return self.apply_axis_offset(base, style.size);
            }
            let variants = self.metrics.vertical_glyph_variants(c, style);
            // Sem variante suficiente, o vanilla monta as peças da fonte ou
            // conserva a maior variante disponível quando não há assembly.
            let picked = if let Some(picked) = variants.select_variant(target_du) {
                Some(picked)
            } else {
                let assembly = self.metrics.vertical_glyph_assembly(c, style);
                if !assembly.is_empty() {
                    let mut composed =
                        self.layout_assembly(c, assembly, target_du, style);
                    // The parts paint one Large operator. Preserve its causal
                    // textual identity before export while keeping all visual
                    // glyph positions and the MathBox geometry unchanged.
                    let parts = std::mem::take(&mut composed.items);
                    composed.items = vec![FrameItem::TextReplacement {
                        text: text.clone(),
                        items: parts,
                    }];
                    return composed;
                }
                variants.variants.last()
            };
            if let Some(picked) = picked {
                let x_advance = style.size * (picked.hor_advance / self.constants.upem);
                let (ink_up, ink_down) =
                    self.metrics.glyph_ink_bounds(picked.glyph_id, style.size, style);
                let b = MathBox {
                    width: x_advance.val(),
                    ascent: ink_up.val(),
                    descent: ink_down.val(),
                    items: vec![FrameItem::Glyph {
                        pos: Point { x: Pt(0.0), y: Pt(0.0) },
                        glyph_id: picked.glyph_id,
                        x_advance,
                        size: style.size,
                        style: style.clone(),
                        base_char: c,
                    }],
                };
                // **P1136** — operadores Large são sempre centrados no
                // eixo; a compensação interna preserva a tinta.
                return self.apply_axis_offset(b, style.size);
            }
        }

        // Inline/script, ou Display sem construção: glifo base.
        self.apply_axis_offset(base, style.size)
    }
}

#[cfg(test)]
mod tests {
    use crate::compiler::layout::FixedMetrics;
    use crate::entities::layout_types::{FrameItem, Pt, TextStyle};

    use super::super::MathLayouter;

    #[test]
    fn sem_construcao_da_fonte_preserva_operador_textual() {
        let style = TextStyle::regular(Pt(12.0));
        let layouter = MathLayouter::new(&FixedMetrics, true, &style);
        let result = layouter.layout_large_operator_display('∑', &style);
        assert!(result.items.iter().any(|item| matches!(item,
            FrameItem::Text { text, .. } if text.as_str() == "∑"
        )));
    }
}
