//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/delimited.md
//! @prompt-hash 9abc28a3
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_delimited` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::engine::layout::FontMetrics;
use crate::entities::{content::Content, layout_types::TextStyle};

use super::MathBox;

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_delimited(
        &self,
        open: char,
        body: &Content,
        close: char,
        style: &TextStyle,
    ) -> MathBox {
        let body_box = self.layout_node(body, style);

        // Converter altura do corpo de pt para design units (P912: delimitadores
        // balanceados em MathDelimited/lr usam 2.0 * (ascent - axis).max(descent + axis))
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let max_extent_pt = (body_box.ascent - axis_pt).max(body_box.descent + axis_pt);
        let body_height_pt = 2.0 * max_extent_pt;
        let min_height_du = if style.size.val() > 0.0 {
            body_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };

        let open_box = self.layout_stretchy_delimiter(open, min_height_du, style);
        let close_box = self.layout_stretchy_delimiter(close, min_height_du, style);

        // **P919** — vanilla (`fenced.rs::layout_fenced`, confirmado por
        // leitura): não centra o grupo delimitado como um todo — os
        // delimitadores vêm pré-centrados (mecanismo próprio, equivalente a
        // `layout_stretchy_delimiter`) e o corpo mantém a sua própria
        // baseline, inalterada. Ver `delimited.md` §P919.
        self.hconcat(vec![open_box, body_box, close_box])
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
