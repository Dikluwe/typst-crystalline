//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/cases.md
//! @prompt-hash 6802f7a3
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_cases` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Pt, TextStyle},
};

use super::GridAlign;
use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_cases(
        &self,
        rows: &[Vec<Content>],
        style: &TextStyle,
    ) -> MathBox {
        let col_gap = style.size * 0.5;
        // **P923b** — `row_gap` resolvido contra o estilo exterior, igual ao
        // `DEFAULT_ROW_GAP = 0.2em` do `CasesElem` no vanilla.
        let row_gap = style.size * 0.2;

        // **P923** — ramos de `cases` renderizados em estilo de denominador
        // (vanilla `resolve_cases` → `resolve_cells` aplica
        // `style_for_denominator`): `MathSize` desce um nível e `cramped` é
        // forçado a `true`. **P945** — a descida passa a ser **por nível**
        // (`denominator_style`, `_comum.md` §P945), mesma correcção e causa
        // raiz partilhada com `matrix.rs` — ver `matrix.md` §P945.
        let cell_style = self.denominator_style(style);

        let grid_box =
            self.layout_grid_rows(rows, GridAlign::Left, col_gap, row_gap, &cell_style);

        let min_height_du = self.grid_delim_target_du(&grid_box, style);

        let left_box = self.layout_stretchy_delimiter('{', min_height_du, style);
        let padding = style.size * 0.1;

        // **P919** — centra APENAS a grelha no eixo matemático (vanilla
        // `table.rs:188`) — o delimitador `{` já vem pré-centrado por
        // `layout_stretchy_delimiter` e não deve ser deslocado de novo.
        // `min_height_du` (acima) já usou o `grid_box` ORIGINAL — o
        // offset só acontece depois de já ter dimensionado o delimitador.
        // Ver `cases.md` §P919.
        let grid_box = self.apply_axis_offset(grid_box, style.size);

        let mut items: Vec<FrameItem> = Vec::new();
        let mut x = Pt(0.0);

        for item in left_box.items.into_iter() {
            items.push(offset_item(item, x, Pt(0.0)));
        }
        x = x + Pt(left_box.width) + padding;

        for item in grid_box.items.into_iter() {
            items.push(offset_item(item, x, Pt(0.0)));
        }
        let total_width = (x + Pt(grid_box.width)).val();

        MathBox {
            width: total_width,
            ascent: grid_box.ascent,
            descent: grid_box.descent,
            items,
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
