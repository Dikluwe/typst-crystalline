//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/matrix.md
//! @prompt-hash a1b2c3d4
//! @layer L1
//! @updated 2026-08-21

use std::sync::Arc;

use ecow::EcoString;

use crate::compiler::layout::vanilla_defaults;
use crate::compiler::math::layout::symbols;
use crate::compiler::math::layout::{offset_item, GridAlign, MathBox, MathLayouter};
use crate::entities::content::Content;
use crate::entities::layout_types::{Color, FrameItem, Length, Point, Pt, TextStyle};

impl<'a, M: crate::compiler::layout::FontMetrics> MathLayouter<'a, M> {
    pub(super) fn layout_matrix(
        &self,
        rows: &[Vec<Content>],
        delim: (char, char),
        row_gap: Option<Length>,
        column_gap: Option<Length>,
        gap: Option<Length>,
        augment: Option<usize>,
        style: &TextStyle,
    ) -> MathBox {
        let font = style.size.val();
        let col_gap = column_gap
            .or(gap)
            .map(|l| l.resolve_pt(font))
            .unwrap_or(0.5 * font);
        let row_gap = row_gap
            .or(gap)
            .map(|l| l.resolve_pt(font))
            .unwrap_or(0.2 * font);

        let cell_style = self.denominator_style(style);

        let mut any_align = false;
        let mut split_rows: Vec<Vec<Content>> = Vec::new();
        let mut align_boundaries: Vec<Vec<bool>> = Vec::new();

        for row in rows {
            let mut out_row: Vec<Content> = Vec::new();
            let mut marks: Vec<bool> = Vec::new();
            for cell in row {
                let parts = split_cell_on_align_point(cell);
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
            let mut grid_boxes: Vec<Vec<MathBox>> = split_rows
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|cell| self.layout_node(cell, &cell_style))
                        .collect()
                })
                .collect();
            for (row_idx, row) in split_rows.iter().enumerate() {
                for col_idx in 1..row.len() {
                    if !align_boundaries[row_idx][col_idx] {
                        continue;
                    }
                    let gap = self.align_boundary_spacing(
                        &row[col_idx - 1],
                        &row[col_idx],
                        &cell_style,
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
                Pt(col_gap),
                Pt(row_gap),
                &align_boundaries,
                &cell_style,
            )
        } else {
            self.layout_grid_rows(rows, GridAlign::Center, Pt(col_gap), Pt(row_gap), &cell_style)
        };

        let min_height_du = self.grid_delim_target_du(&grid_box, style);

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

        let grid_box = self.apply_axis_offset(grid_box, style.size);

        let delim_pad = 0.0;

        let mut items: Vec<FrameItem> = Vec::new();
        let mut x = Pt(0.0);

        if delim.0 != '\0' {
            for item in left_box.items.iter() {
                items.push(offset_item(item.clone(), x, Pt(0.0)));
            }
            x = x + Pt(left_box.width) + Pt(delim_pad);
        }

        let grid_origin_x = x;
        for item in grid_box.items.iter() {
            items.push(offset_item(item.clone(), x, Pt(0.0)));
        }

        // Se houver augment (linha vertical divisória de matriz aumentada)
        if let Some(aug_col) = augment {
            let num_cols = rows.iter().map(|r| r.len()).max().unwrap_or(0);
            if aug_col > 0 && aug_col < num_cols {
                let mut col_widths = vec![0.0_f64; num_cols];
                let min_col_w = 0.5 * font;
                for row in rows {
                    for (c_idx, cell) in row.iter().enumerate() {
                        let b = self.layout_node(cell, &cell_style);
                        col_widths[c_idx] = col_widths[c_idx].max(b.width).max(min_col_w);
                    }
                }
                let mut aug_x = 0.0_f64;
                for c in 0..aug_col {
                    aug_x += col_widths[c];
                    if c + 1 < aug_col {
                        aug_x += col_gap;
                    }
                }
                aug_x += col_gap / 2.0;

                let line_x = grid_origin_x + Pt(aug_x);
                let line_y_top = Pt(-grid_box.ascent);
                let line_y_bot = Pt(grid_box.descent);

                let stroke_width = 0.05 * font;
                items.push(FrameItem::Line {
                    start: Point { x: line_x, y: line_y_top },
                    end: Point { x: line_x, y: line_y_bot },
                    thickness: stroke_width,
                    color: None,
                });
            }
        }

        x = x + Pt(grid_box.width);

        if delim.1 != '\0' {
            for item in right_box.items.iter() {
                items.push(offset_item(item.clone(), x, Pt(0.0)));
            }
            x = x + Pt(right_box.width);
        }

        let ascent = grid_box.ascent.max(left_box.ascent).max(right_box.ascent);
        let descent = grid_box.descent.max(left_box.descent).max(right_box.descent);

        MathBox {
            width: x.val(),
            ascent,
            descent,
            items,
        }
    }

    pub(super) fn align_boundary_spacing(
        &self,
        left: &Content,
        right: &Content,
        style: &TextStyle,
    ) -> f64 {
        let left_edge = edge_node(left, false);
        let right_edge = edge_node(right, true);
        let (_, l_rclass) = super::spacing::node_math_class(left_edge);
        let (r_lclass, _) = super::spacing::node_math_class(right_edge);
        match super::spacing::spacing_between_class(l_rclass, r_lclass, style.size.val()) {
            Some(v) => v,
            None => {
                let is_spaced = matches!(left_edge, Content::Text(_))
                    || matches!(right_edge, Content::Text(_))
                    || l_rclass == crate::entities::math_class::MathClass::Fence
                    || r_lclass == crate::entities::math_class::MathClass::Fence;
                if is_spaced {
                    self.metrics.advance(" ", style.size, style).val()
                } else {
                    0.0
                }
            }
        }
    }
}

pub(super) fn split_cell_on_align_point(cell: &Content) -> Vec<Content> {
    match cell {
        Content::MathSequence(seq) => {
            let mut parts: Vec<Vec<Content>> = Vec::new();
            let mut current: Vec<Content> = Vec::new();
            for item in seq.iter() {
                if matches!(item, Content::MathAlignPoint(_)) {
                    parts.push(std::mem::take(&mut current));
                } else {
                    current.push(item.clone());
                }
            }
            parts.push(current);
            parts
                .into_iter()
                .map(|p| match p.len() {
                    0 => Content::MathSequence(Arc::from(Vec::new())),
                    1 => p.into_iter().next().unwrap(),
                    _ => Content::MathSequence(Arc::from(p)),
                })
                .collect()
        }
        Content::MathAlignPoint(_) => vec![
            Content::MathSequence(Arc::from(Vec::new())),
            Content::MathSequence(Arc::from(Vec::new())),
        ],
        _ => vec![cell.clone()],
    }
}

pub(super) fn edge_node<'c>(cell: &'c Content, first: bool) -> &'c Content {
    if let Content::MathSequence(items) = cell {
        let edge = if first { items.first() } else { items.last() };
        if let Some(node) = edge {
            return node;
        }
    }
    cell
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_matrix_stub() {
        assert!(true);
    }
}
