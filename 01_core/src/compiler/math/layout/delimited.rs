//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/delimited.md
//! @prompt-hash 84c07e81
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_delimited` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
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

        // Converter altura do corpo de pt para design units. Cercas comuns são
        // balanceadas no eixo; o frac-like sem barra de `binom` segue o braço
        // `balanced=false` do vanilla e usa a altura real do corpo.
        let axis_pt = self.constants.to_pt(self.constants.axis_height, style.size).val();
        let balanced = !matches!(body, Content::MathFrac(e) if !e.line);
        let body_height_pt = if balanced {
            let max_extent_pt =
                (body_box.ascent - axis_pt).max(body_box.descent + axis_pt);
            2.0 * max_extent_pt
        } else {
            body_box.height()
        };
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
