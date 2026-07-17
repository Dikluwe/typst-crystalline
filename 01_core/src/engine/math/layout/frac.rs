//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/frac.md
//! @prompt-hash 0038ff0b
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_frac` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
};
use crate::engine::layout::FontMetrics;

use super::MathBox;


impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_frac(&self, num: &Content, den: &Content, style: &TextStyle) -> MathBox {
        let sub_style = TextStyle {
            size: style.size * self.constants.script_percent_scale_down,
            ..style.clone()
        };

        let num_box = self.layout_node(num, &sub_style);
        let den_box = self.layout_node(den, &sub_style);

        let width = num_box.width.max(den_box.width);

        let rule_thickness = self.constants.to_pt(
            self.constants.fraction_rule_thickness, style.size
        ).val();
        let gap = self.constants.to_pt(
            self.constants.fraction_num_gap, style.size
        ).val();

        // ascent cobre todo o numerador + espaço + metade da linha
        let ascent  = num_box.height() + gap + rule_thickness / 2.0;
        // descent cobre metade da linha + espaço + todo o denominador
        let descent = den_box.height() + gap + rule_thickness / 2.0;

        // Centrar horizontalmente
        let num_x = (width - num_box.width) / 2.0;
        let den_x = (width - den_box.width) / 2.0;

        // Numerador: topo do MathBox (local_y = 0)
        let num_y = 0.0_f64;
        // Denominador: abaixo do numerador + linha de fracção
        let den_y = num_box.height() + gap + rule_thickness + gap;
        // Linha de fracção: centro entre numerador e denominador
        let rule_local_y = num_box.height() + gap + rule_thickness / 2.0;

        let mut items = Vec::new();

        for mut item in num_box.items {
            match item {
                FrameItem::Text { ref mut pos, .. }
                | FrameItem::TextShaped { ref mut pos, .. } => {
                    pos.x = Pt(pos.x.val() + num_x);
                    pos.y = Pt(pos.y.val() + num_y);
                }
                _ => {}
            }
            items.push(item);
        }

        // Linha de fracção posicionada entre numerador e denominador.
        items.push(FrameItem::Line {
            start:     Point { x: Pt(0.0),    y: Pt(rule_local_y) },
            end:       Point { x: Pt(width),  y: Pt(rule_local_y) },
            thickness: rule_thickness,
            // P285: math frac sem stroke explícito — preserva preto bit-exact.
            color:     None,
        });

        for mut item in den_box.items {
            match item {
                FrameItem::Text { ref mut pos, .. }
                | FrameItem::TextShaped { ref mut pos, .. } => {
                    pos.x = Pt(pos.x.val() + den_x);
                    pos.y = Pt(pos.y.val() + den_y);
                }
                _ => {}
            }
            items.push(item);
        }

        self.apply_axis_offset(MathBox { width, ascent, descent, items }, style.size)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
