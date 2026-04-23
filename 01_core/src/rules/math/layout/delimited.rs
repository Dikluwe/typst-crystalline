//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/math/layout.md
//! @prompt-hash 38b51727
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_delimited` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::entities::{
    content::Content,
    layout_types::TextStyle,
};
use crate::rules::layout::FontMetrics;

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_delimited(
        &self,
        open:  char,
        body:  &Content,
        close: char,
        style: &TextStyle,
    ) -> MathBox {
        let body_box = self.layout_node(body, style);

        // Converter altura do corpo de pt para design units
        let body_height_pt = body_box.ascent + body_box.descent;
        let min_height_du  = if style.size.val() > 0.0 {
            body_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };

        let open_box  = self.layout_stretchy_delimiter(open,  min_height_du, style);
        let close_box = self.layout_stretchy_delimiter(close, min_height_du, style);

        let result = self.hconcat(vec![open_box, body_box, close_box]);
        self.apply_axis_offset(result, style.size)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
