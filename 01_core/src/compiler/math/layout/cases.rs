//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/cases.md
//! @prompt-hash 5396f07a
//! @layer L1
//! @updated 2026-08-14
//!
//! Método `layout_cases` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::compiler::layout::FontMetrics;
use crate::entities::{
    content::Content,
    elements::math_cases::MathCasesElem,
    layout_types::{FrameItem, Length, Pt, TextStyle},
    rel::Rel,
    value::Value,
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
        let content = Content::math_cases(rows.to_vec(), delim, reverse, gap);
        let Content::MathCases(elem) = content else { unreachable!() };
        self.layout_cases_elem(&elem, style)
    }

    pub(super) fn layout_cases_elem(
        &self,
        elem: &MathCasesElem,
        style: &TextStyle,
    ) -> MathBox {
        let cascaded_delim = self
            .lexical_styles
            .and_then(|styles| styles.custom("math.cases.delim"))
            .map(|value| {
                crate::compiler::stdlib::math_cases_delim(
                    value,
                    crate::entities::span::Span::detached(),
                )
                .expect("validated math.cases.delim")
            });
        let cascaded_reverse = self
            .lexical_styles
            .and_then(|styles| styles.custom("math.cases.reverse"))
            .map(|value| match value {
                Value::Bool(value) => *value,
                other => {
                    panic!("invalid validated math.cases.reverse: {}", other.type_name())
                }
            });
        let cascaded_gap = self
            .lexical_styles
            .and_then(|styles| styles.custom("math.cases.gap"))
            .map(|value| match value {
                Value::Relative(value) => *value,
                Value::Ratio(value) => Rel { rel: value.get(), abs: Length::ZERO },
                Value::Length(value) => Rel { rel: 0.0, abs: *value },
                other => {
                    panic!("invalid validated math.cases.gap: {}", other.type_name())
                }
            });
        let delim = if elem.explicit.delim {
            elem.delim
        } else {
            cascaded_delim.unwrap_or((Some('{'), Some('}')))
        };
        let reverse = if elem.explicit.reverse {
            elem.reverse
        } else {
            cascaded_reverse.unwrap_or(false)
        };
        let gap = if elem.explicit.gap {
            elem.gap
        } else {
            cascaded_gap.unwrap_or(Rel { rel: 0.0, abs: Length::em(0.2) })
        };
        let rows = elem
            .children
            .iter()
            .cloned()
            .map(|child| vec![child])
            .collect::<Vec<_>>();

        // **P1042** — vanilla `resolve_cases` usa `Axes::with_y(elem.gap)` com `x = 0` (o espaçamento entre sub-colunas provém de `align_boundary_spacing`).
        let col_gap = self.metrics.advance(" ", style.size, style);
        let row_gap =
            Pt(gap.resolve(Length::pt(style.size.val())).resolve_pt(style.size.val()));

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
        for row in &rows {
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
            self.layout_grid_rows(&rows, GridAlign::Left, col_gap, row_gap, &cell_style)
        };

        let min_height_du = self.grid_delim_target_du(&grid_box, style);

        // **P1047** — delimitador elástico de acordo com `reverse` (se `reverse`, usa `delim.1` à direita; caso contrário `delim.0` à esquerda).
        let delim_char = if reverse { delim.1 } else { delim.0 };
        let delim_box = delim_char.map(|character| {
            self.layout_stretchy_delimiter(character, min_height_du, style)
        });

        // **P919** — centra APENAS a grelha no eixo matemático (vanilla
        // `table.rs:188`) — o delimitador já vem pré-centrado por
        // `layout_stretchy_delimiter` e não deve ser deslocado de novo.
        // `min_height_du` (acima) já usou o `grid_box` ORIGINAL — o
        // offset só acontece depois de já ter dimensionado o delimitador.
        // Ver `cases.md` §P919.
        let grid_box = self.apply_axis_offset(grid_box, style.size);

        // **P1042** — delimitador sem padding redundante (+0.1em removido per vanilla).
        let mut components = Vec::new();
        if !reverse {
            if let Some(left_box) = delim_box {
                components.push(left_box);
            }
            components.push(grid_box);
        } else {
            components.push(grid_box);
            if let Some(right_box) = delim_box {
                components.push(right_box);
            }
        }

        let ascent = components.iter().map(|b| b.ascent).fold(0.0, f64::max);
        let descent = components.iter().map(|b| b.descent).fold(0.0, f64::max);
        let mut items: Vec<FrameItem> = Vec::new();
        let mut x = Pt(0.0);
        for component in components {
            for item in component.items {
                // P1132e: todas as MathBox já partilham y=0 na baseline;
                // composição horizontal desloca somente o eixo X.
                items.push(offset_item(item, x, Pt(0.0)));
            }
            x = x + Pt(component.width);
        }

        MathBox { width: x.val(), ascent, descent, items }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
