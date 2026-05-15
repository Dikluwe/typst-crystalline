//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math/layout.md
//! @prompt-hash c45536b1
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_root` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Point, Pt, TextStyle},
};
use crate::rules::layout::FontMetrics;

use super::{MathBox, offset_item};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_root(
        &self,
        index:    Option<&Content>,
        radicand: &Content,
        style:    &TextStyle,
    ) -> MathBox {
        // 1. Layout do radicando
        let rad_box = self.layout_node(radicand, style);

        // 3. Geometria da overline (calculada antes do radical para computar min_height)
        let line_thickness = self.constants.to_pt(
            self.constants.radical_rule_thickness, style.size
        ).val();
        let gap = self.constants.to_pt(
            self.constants.radical_vertical_gap, style.size
        ).val();

        // 2. Símbolo √ extensível — altura cobre radicando + gap + overline
        let rad_height_pt  = rad_box.ascent + rad_box.descent + gap + line_thickness;
        let min_height_du  = if style.size.val() > 0.0 {
            rad_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };
        let radical_box   = self.layout_stretchy_delimiter('√', min_height_du, style);
        let radical_width = radical_box.width;

        // 4. Dimensões totais
        //    ascent cobre: ascent do radicando + gap + espessura da linha
        let total_ascent  = rad_box.ascent + gap + line_thickness;
        let total_descent = rad_box.descent;
        let total_width   = radical_width + rad_box.width;

        let mut items = Vec::new();

        // 5a. Símbolo √ — deslocar para baixo para que a sua baseline alinhe
        //     com a baseline principal (total_ascent em coordenadas locais)
        let sym_dy = total_ascent - radical_box.ascent;
        for item in radical_box.items {
            items.push(offset_item(item, Pt(0.0), Pt(sym_dy)));
        }

        // 5b. Overline — linha horizontal no topo do radicando
        let overline_y = gap + line_thickness / 2.0;
        items.push(FrameItem::Line {
            start:     Point { x: Pt(radical_width),                    y: Pt(overline_y) },
            end:       Point { x: Pt(radical_width + rad_box.width),    y: Pt(overline_y) },
            thickness: line_thickness,
        });

        // 5c. Radicando — à direita do símbolo, deslocado abaixo da overline
        let rad_offset_y = gap + line_thickness;
        for item in rad_box.items {
            items.push(offset_item(item, Pt(radical_width), Pt(rad_offset_y)));
        }

        // 6. Índice opcional (para root(n, x))
        if let Some(idx_content) = index {
            let script_style = TextStyle {
                size: style.size * self.constants.script_percent_scale_down,
                ..style.clone()
            };
            let idx_box = self.layout_node(idx_content, &script_style);
            // Posicionar acima e à esquerda do símbolo: x=20% da largura do radical, y=0 (topo)
            let idx_x = radical_width * 0.2;
            for item in idx_box.items {
                items.push(offset_item(item, Pt(idx_x), Pt(0.0)));
            }
        }

        let result = MathBox { width: total_width, ascent: total_ascent, descent: total_descent, items };
        self.apply_axis_offset(result, style.size)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
