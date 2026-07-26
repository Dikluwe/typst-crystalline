//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/math/layout/matrix.md
//! @prompt-hash 5ddaca7c
//! @layer L1
//! @updated 2026-04-23
//!
//! Método `layout_matrix` de `MathLayouter`. Extraído de `math/layout/mod.rs`
//! no Passo 96.8 conforme ADR-0037.

use crate::engine::layout::FontMetrics;
use crate::entities::{
    content::Content,
    layout_types::{FrameItem, Pt, TextStyle},
};

use super::GridAlign;
use super::{offset_item, MathBox};

impl<'a, M: FontMetrics> super::MathLayouter<'a, M> {
    pub(super) fn layout_matrix(
        &self,
        rows: &[Vec<Content>],
        delim: (char, char),
        style: &TextStyle,
    ) -> MathBox {
        let col_gap = style.size * 0.5;

        // **P825 (sub-achado D de P810 §12)** — `&` dentro de células de
        // `mat`: parte a célula em colunas de alinhamento
        // (`LeftRightAlternator::Right` do vanilla — pares à direita,
        // ímpares à esquerda). Sem `&` em nenhuma célula, comportamento
        // inalterado (colunas centradas — paridade medida em P810 §12).
        let mut any_align = false;
        let mut split_rows: Vec<Vec<Content>> = Vec::with_capacity(rows.len());
        let mut align_boundaries: Vec<Vec<bool>> = Vec::with_capacity(rows.len());
        for row in rows {
            let mut out_row: Vec<Content> = Vec::new();
            let mut marks: Vec<bool> = Vec::new();
            for cell in row {
                let parts = split_cell_on_align_point(cell);
                let n = parts.len();
                for (i, part) in parts.into_iter().enumerate() {
                    out_row.push(part);
                    // Marca o limite ANTES da célula: `true` só para
                    // limites produzidos por `&` (não por vírgula).
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
            // Medir as células e incorporar o espaçamento de classe do
            // limite `&` na largura da célula par à esquerda (paridade
            // vanilla `run.rs:76-94`: o lspace do item da coluna ímpar —
            // ex.: THICK antes de `=` — é movido para a célula par).
            let mut grid_boxes: Vec<Vec<MathBox>> = split_rows
                .iter()
                .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
                .collect();
            for (row_idx, row) in split_rows.iter().enumerate() {
                for col_idx in 1..row.len() {
                    if !align_boundaries[row_idx][col_idx] {
                        continue;
                    }
                    let gap = align_boundary_spacing(
                        &row[col_idx - 1],
                        &row[col_idx],
                        style.size.val(),
                    );
                    if let Some(left_box) =
                        grid_boxes[row_idx].get_mut(col_idx - 1)
                    {
                        left_box.width += gap;
                    }
                }
            }
            self.layout_grid_boxes(
                grid_boxes,
                GridAlign::Alternating,
                col_gap,
                &align_boundaries,
                style,
            )
        } else {
            self.layout_grid_rows(rows, GridAlign::Center, col_gap, style)
        };

        // Converter altura da grelha de Pt para Design Units para layout_stretchy_delimiter (P912: margem de 10%).
        let grid_height_pt = (grid_box.ascent + grid_box.descent) * 1.1;
        let min_height_du = if style.size.val() > 0.0 {
            grid_height_pt * self.constants.upem / style.size.val()
        } else {
            0.0
        };

        let left_box = if delim.0 != '\0' {
            self.layout_stretchy_delimiter(delim.0, min_height_du, style)
        } else {
            MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: Vec::new() }
        };
        let right_box = if delim.1 != '\0' {
            self.layout_stretchy_delimiter(delim.1, min_height_du, style)
        } else {
            MathBox { width: 0.0, ascent: 0.0, descent: 0.0, items: Vec::new() }
        };

        // Composição horizontal com padding entre delimitadores e grelha.
        let padding = style.size * 0.1;
        let mut items: Vec<FrameItem> = Vec::new();
        let mut x = Pt(0.0);

        if delim.0 != '\0' {
            for item in left_box.items.iter() {
                items.push(offset_item(item.clone(), x, Pt(0.0)));
            }
            x = x + Pt(left_box.width) + padding;
        }

        for item in grid_box.items.iter() {
            items.push(offset_item(item.clone(), x, Pt(0.0)));
        }
        x = x + Pt(grid_box.width);

        if delim.1 != '\0' {
            x = x + padding;
            for item in right_box.items.iter() {
                items.push(offset_item(item.clone(), x, Pt(0.0)));
            }
            x = x + Pt(right_box.width);
        }

        let result = MathBox {
            width: x.val(),
            ascent: grid_box.ascent,
            descent: grid_box.descent,
            items,
        };
        self.apply_axis_offset(result, style.size)
    }
}

/// **P825** — parte o `Content` de uma célula de `mat` nos
/// `MathAlignPoint` (`&`). Sem `&`, devolve a célula inalterada (1 parte).
fn split_cell_on_align_point(cell: &Content) -> Vec<Content> {
    if let Content::MathSequence(items) = cell {
        if items.iter().any(|c| matches!(c, Content::MathAlignPoint(_))) {
            let mut parts: Vec<Vec<Content>> = vec![vec![]];
            for item in items.iter() {
                match item {
                    Content::MathAlignPoint(_) => parts.push(vec![]),
                    other => parts.last_mut().unwrap().push(other.clone()),
                }
            }
            return parts
                .into_iter()
                .map(|part| match part.len() {
                    0 => Content::Empty,
                    1 => part.into_iter().next().unwrap(),
                    _ => Content::MathSequence(part.into()),
                })
                .collect();
        }
    }
    vec![cell.clone()]
}

/// **P825** — primeiro/último nó "real" de uma célula (para a classe do
/// limite). `MathSequence` → primeiro/último item; `Empty`/outros → a
/// própria célula.
fn edge_node<'c>(cell: &'c Content, first: bool) -> &'c Content {
    if let Content::MathSequence(items) = cell {
        let edge = if first { items.first() } else { items.last() };
        if let Some(node) = edge {
            return node;
        }
    }
    cell
}

/// **P825** — espaçamento de classe no limite `&` entre duas células
/// (paridade `alignment_lspace` do vanilla, `run.rs:363-373`: o lspace do
/// primeiro item da célula seguinte — ex.: THICK antes de uma relação).
fn align_boundary_spacing(left: &Content, right: &Content, size_pt: f64) -> f64 {
    let (_, l_rclass) = super::spacing::node_math_class(edge_node(left, false));
    let (r_lclass, _) = super::spacing::node_math_class(edge_node(right, true));
    super::spacing::spacing_between(l_rclass, r_lclass, size_pt)
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {}
}
