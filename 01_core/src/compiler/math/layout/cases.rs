//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/cases.md
//! @prompt-hash 426beb18
//! @layer L1
//! @updated 2026-08-14
//!
//! Método `layout_cases` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Length, Pt, TextStyle},
};

use super::GridAlign;
use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_cases(
        &self,
        rows: &[Vec<Content>],
        delim: (char, char),
        reverse: bool,
        gap: Option<Length>,
        style: &TextStyle,
    ) -> MathBox {
        // **P1042** — vanilla `resolve_cases` usa `Axes::with_y(elem.gap)` com `x = 0` (o espaçamento entre sub-colunas provém de `align_boundary_spacing`).
        let col_gap = self.metrics.advance(" ", style.size, style);
        // **P1047** — `row_gap` resolvido contra `gap` se fornecido pelo elemento, ou fallback a `style.size * 0.2` (`DEFAULT_ROW_GAP = 0.2em` do vanilla).
        let row_gap = match gap {
            Some(g) => Pt(g.resolve_pt(style.size.val())),
            None => style.size * 0.2,
        };

        // **P923** — ramos de `cases` renderizados em estilo de denominador
        // (vanilla `resolve_cases` → `resolve_cells` aplica
        // `style_for_denominator`): `MathSize` desce um nível e `cramped` é
        // forçado a `true`. **P945** — a descida passa a ser **por nível**
        // (`denominator_style`, `_comum.md` §P945), mesma correcção e causa
        // raiz partilhada com `matrix.rs` — ver `matrix.md` §P945.
        let cell_style = self.denominator_style(style);

        // **P1042** — suporte nativo a `&` dentro dos ramos de `cases`: parte
        // a célula em pontos de alinhamento com espaçamento de símbolo natural
        // (reaproveita `split_cell_on_align_point` e `align_boundary_spacing` de `matrix.rs`).
        let mut any_align = false;
        let mut split_rows: Vec<Vec<Content>> = Vec::with_capacity(rows.len());
        let mut align_boundaries: Vec<Vec<bool>> = Vec::with_capacity(rows.len());
        for row in rows {
            let mut out_row: Vec<Content> = Vec::new();
            let mut marks: Vec<bool> = Vec::new();
            for cell in row {
                let parts = super::matrix::split_cell_on_align_point(cell);
                let n = parts.len();
                for (i, part) in parts.into_iter().enumerate() {
                    out_row.push(part);
                    marks.push(i > 0);
                }
                if n > 1 {
                    any_align = true;
                }
            }
            split_rows.push(out_row);
            align_boundaries.push(marks);
        }

        let grid_box = if any_align {
            self.layout_grid_rows(
                &split_rows,
                GridAlign::Left,
                col_gap,
                row_gap,
                &cell_style,
            )
        } else {
            self.layout_grid_rows(rows, GridAlign::Left, col_gap, row_gap, &cell_style)
        };

        let min_height_du = self.grid_delim_target_du(&grid_box, style);

        // **P1047** — delimitador elástico de acordo com `reverse` (se `reverse`, usa `delim.1` à direita; caso contrário `delim.0` à esquerda).
        let delim_char = if reverse { delim.1 } else { delim.0 };
        let delim_box = if delim_char != '\0' {
            Some(self.layout_stretchy_delimiter(delim_char, min_height_du, style))
        } else {
            None
        };

        // **P919** — centra APENAS a grelha no eixo matemático (vanilla
        // `table.rs:188`) — o delimitador já vem pré-centrado por
        // `layout_stretchy_delimiter` e não deve ser deslocado de novo.
        // `min_height_du` (acima) já usou o `grid_box` ORIGINAL — o
        // offset só acontece depois de já ter dimensionado o delimitador.
        // Ver `cases.md` §P919.
        let grid_box = self.apply_axis_offset(grid_box, style.size);

        // **P1042** — delimitador sem padding redundante (+0.1em removido per vanilla).
        let mut items: Vec<FrameItem> = Vec::new();
        let mut x = Pt(0.0);

        if !reverse {
            if let Some(left_box) = delim_box {
                for item in left_box.items.into_iter() {
                    items.push(offset_item(item, x, Pt(0.0)));
                }
                x = x + Pt(left_box.width);
            }
            for item in grid_box.items.into_iter() {
                items.push(offset_item(item, x, Pt(0.0)));
            }
            x = x + Pt(grid_box.width);
        } else {
            for item in grid_box.items.into_iter() {
                items.push(offset_item(item, x, Pt(0.0)));
            }
            x = x + Pt(grid_box.width);
            if let Some(right_box) = delim_box {
                for item in right_box.items.into_iter() {
                    items.push(offset_item(item, x, Pt(0.0)));
                }
                x = x + Pt(right_box.width);
            }
        }

        MathBox {
            width: x.val(),
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
