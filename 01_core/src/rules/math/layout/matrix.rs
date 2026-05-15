//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math/layout.md
//! @prompt-hash c45536b1
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_matrix` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Pt, TextStyle},
};
use crate::rules::layout::FontMetrics;

use super::{MathBox, offset_item};
use super::GridAlign;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_matrix(
        &self,
        rows:  &[Vec<Content>],
        delim: (char, char),
        style: &TextStyle,
    ) -> MathBox {
        let col_gap   = style.size * 0.5;
        let grid_box  = self.layout_grid_rows(rows, GridAlign::Center, col_gap, style);

        // Converter altura da grelha de Pt para Design Units para layout_stretchy_delimiter.
        let grid_height_pt = grid_box.ascent + grid_box.descent;
        let min_height_du  = if style.size.val() > 0.0 {
            grid_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };

        let left_box  = self.layout_stretchy_delimiter(delim.0, min_height_du, style);
        let right_box = self.layout_stretchy_delimiter(delim.1, min_height_du, style);

        // Composição horizontal com padding entre delimitadores e grelha.
        let padding = style.size * 0.1;
        let mut items: Vec<FrameItem> = Vec::new();
        let mut x = Pt(0.0);

        for item in left_box.items.iter() {
            items.push(offset_item(item.clone(), x, Pt(0.0)));
        }
        x = x + Pt(left_box.width) + padding;

        for item in grid_box.items.iter() {
            items.push(offset_item(item.clone(), x, Pt(0.0)));
        }
        x = x + Pt(grid_box.width) + padding;

        for item in right_box.items.iter() {
            items.push(offset_item(item.clone(), x, Pt(0.0)));
        }
        let total_width = (x + Pt(right_box.width)).val();

        let result = MathBox {
            width:   total_width,
            ascent:  grid_box.ascent,
            descent: grid_box.descent,
            items,
        };
        self.apply_axis_offset(result, style.size)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
