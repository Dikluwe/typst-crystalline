//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/matrix.md
//! @prompt-hash 0ba7a931
//! @layer L1
//! @updated 2026-09-17

use std::sync::Arc;

use ecow::EcoString;

use crate::compiler::math::layout::{offset_item, GridAlign, MathBox, MathLayouter};
use crate::compiler::stdlib::{apply_math_matrix_option, MathMatrixOptions};
use crate::entities::content::Content;
use crate::entities::elements::math_matrix::{MathMatrixAugment, MathMatrixElem};
use crate::entities::geometry::{LineCap, Stroke};
use crate::entities::layout_types::{FrameItem, HAlign, Length, Point, Pt, TextStyle};
use crate::entities::rel::Rel;
use crate::entities::span::Span;

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
        let row_gap = row_gap.or(gap).unwrap_or_else(|| Length::em(0.2));
        let column_gap = column_gap.or(gap).unwrap_or_else(|| Length::em(0.5));
        let elem = MathMatrixElem {
            rows: rows.to_vec(),
            delim: (Some(delim.0), Some(delim.1)),
            align: HAlign::Center,
            augment: augment.map(|offset| MathMatrixAugment {
                hline: Vec::new(),
                vline: vec![offset as i64],
                stroke: None,
            }),
            row_gap: Rel { rel: 0.0, abs: row_gap },
            column_gap: Rel { rel: 0.0, abs: column_gap },
            explicit: crate::entities::elements::math_matrix::MathMatrixExplicit {
                delim: true,
                align: false,
                augment: augment.is_some(),
                row_gap: true,
                column_gap: true,
            },
        };
        self.layout_matrix_elem(&elem, style)
    }

    pub(super) fn layout_matrix_elem(
        &self,
        elem: &MathMatrixElem,
        style: &TextStyle,
    ) -> MathBox {
        let effective = self.effective_matrix_options(elem);
        let font = style.size.val();
        let col_gap = resolve_gap(effective.column_gap, font);
        let row_gap = resolve_gap(effective.row_gap, font);
        let cell_style = self.denominator_style(style);

        let mut any_align = false;
        let mut split_rows = Vec::new();
        let mut align_boundaries = Vec::new();
        for row in &elem.rows {
            let mut out_row = Vec::new();
            let mut marks = Vec::new();
            for cell in row {
                let parts = split_cell_on_align_point(cell);
                any_align |= parts.len() > 1;
                for (index, part) in parts.into_iter().enumerate() {
                    out_row.push(part);
                    marks.push(index > 0);
                }
            }
            split_rows.push(out_row);
            align_boundaries.push(marks);
        }

        let boundary_measured: Vec<Vec<MathBox>> = split_rows
            .iter()
            .map(|row| row.iter().map(|cell| self.layout_node(cell, style)).collect())
            .collect();
        let mut measured: Vec<Vec<MathBox>> = split_rows
            .iter()
            .map(|row| {
                row.iter().map(|cell| self.layout_node(cell, &cell_style)).collect()
            })
            .collect();
        if any_align {
            for (row_index, row) in split_rows.iter().enumerate() {
                for column in 1..row.len() {
                    if !align_boundaries[row_index][column] {
                        continue;
                    }
                    let gap = self.align_boundary_spacing(
                        &row[column - 1],
                        &row[column],
                        &cell_style,
                    );
                    if let Some(left) = measured[row_index].get_mut(column - 1) {
                        left.width += gap;
                    }
                }
            }
        }

        let align = if any_align {
            GridAlign::Alternating
        } else {
            match effective.align {
                HAlign::Start | HAlign::Left => GridAlign::Left,
                HAlign::Center => GridAlign::Center,
                HAlign::Right | HAlign::End => GridAlign::Right,
            }
        };
        let geometry = GridGeometry::measure(
            self,
            &measured,
            &boundary_measured,
            col_gap,
            row_gap,
            &cell_style,
        );
        let boundaries = if any_align { align_boundaries.as_slice() } else { &[] };
        let grid_box = self.layout_grid_boxes(
            measured,
            align,
            Pt(col_gap),
            Pt(row_gap),
            boundaries,
            &cell_style,
        );

        let min_height_du = self.grid_delim_target_du(&grid_box, style);
        let left_box = effective
            .delim
            .0
            .map(|delimiter| {
                self.layout_stretchy_delimiter(delimiter, min_height_du, style)
            })
            .unwrap_or_else(empty_box);
        let right_box = effective
            .delim
            .1
            .map(|delimiter| {
                self.layout_stretchy_delimiter(delimiter, min_height_du, style)
            })
            .unwrap_or_else(empty_box);
        let axis_shift =
            self.constants.to_pt(self.constants.axis_height, style.size).val()
                - (grid_box.ascent - grid_box.descent) / 2.0;
        let grid_box = self.apply_axis_offset(grid_box, style.size);

        let mut items = Vec::new();
        let mut x = Pt(0.0);
        if effective.delim.0.is_some() {
            items.extend(
                left_box
                    .items
                    .iter()
                    .cloned()
                    .map(|item| offset_item(item, x, Pt(0.0))),
            );
            x = x + Pt(left_box.width);
        }
        let grid_origin_x = x;
        items.extend(
            grid_box
                .items
                .iter()
                .cloned()
                .map(|item| offset_item(item, grid_origin_x, Pt(0.0))),
        );

        if let Some(augment) = effective.augment {
            let (thickness, color, cap) = augment_stroke(&augment, font);
            for offset in augment.hline {
                if let Some(y) = geometry.horizontal_boundary(offset, -axis_shift) {
                    items.push(FrameItem::Line {
                        start: Point { x: grid_origin_x, y: Pt(y) },
                        end: Point { x: grid_origin_x + Pt(grid_box.width), y: Pt(y) },
                        thickness,
                        color: color.clone(),
                        cap,
                    });
                }
            }
            for offset in augment.vline {
                if let Some(boundary_x) = geometry.vertical_boundary(offset) {
                    let line_x = grid_origin_x + Pt(boundary_x);
                    items.push(FrameItem::Line {
                        start: Point { x: line_x, y: Pt(0.0) },
                        end: Point {
                            x: line_x,
                            y: Pt(grid_box.ascent + grid_box.descent),
                        },
                        thickness,
                        color: color.clone(),
                        cap,
                    });
                }
            }
        }

        x = x + Pt(grid_box.width);
        if effective.delim.1.is_some() {
            items.extend(
                right_box
                    .items
                    .iter()
                    .cloned()
                    .map(|item| offset_item(item, x, Pt(0.0))),
            );
            x = x + Pt(right_box.width);
        }
        MathBox {
            width: x.val(),
            ascent: grid_box.ascent.max(left_box.ascent).max(right_box.ascent),
            descent: grid_box.descent.max(left_box.descent).max(right_box.descent),
            items,
        }
    }

    fn effective_matrix_options(&self, elem: &MathMatrixElem) -> MathMatrixOptions {
        let mut inherited = MathMatrixOptions::default();
        inherited.explicit = Default::default();
        if let Some(chain) = self.lexical_styles {
            if let Some(value) = chain.custom("math.mat.gap") {
                let _ = apply_math_matrix_option(
                    &mut inherited,
                    "gap",
                    value,
                    Span::detached(),
                    false,
                );
            }
            for name in ["delim", "align", "augment", "row-gap", "column-gap"] {
                if let Some(value) = chain.custom(&format!("math.mat.{name}")) {
                    let _ = apply_math_matrix_option(
                        &mut inherited,
                        name,
                        value,
                        Span::detached(),
                        false,
                    );
                }
            }
        }

        let mut effective = inherited.clone();
        if elem.explicit.delim {
            effective.delim = elem.delim;
        }
        if elem.explicit.align {
            effective.align = elem.align;
        }
        if elem.explicit.row_gap {
            effective.row_gap = elem.row_gap;
        }
        if elem.explicit.column_gap {
            effective.column_gap = elem.column_gap;
        }
        if elem.explicit.augment {
            effective.augment = match (&elem.augment, &inherited.augment) {
                (None, _) => None,
                (Some(inner), Some(outer)) => Some(fold_augment(inner, outer)),
                (Some(inner), None) => Some(inner.clone()),
            };
        }
        effective.explicit = elem.explicit;
        effective
    }

    pub(super) fn align_boundary_spacing(
        &self,
        left: &Content,
        right: &Content,
        style: &TextStyle,
    ) -> f64 {
        let left_edge = edge_node(left, false);
        let right_edge = edge_node(right, true);
        let (_, left_class) = super::spacing::node_math_class(left_edge);
        let (right_class, _) = super::spacing::node_math_class(right_edge);
        match super::spacing::spacing_between_class(
            left_class,
            right_class,
            style.size.val(),
        ) {
            Some(value) => value,
            None => {
                let spaced = matches!(left_edge, Content::Text(_))
                    || matches!(right_edge, Content::Text(_))
                    || left_class == crate::entities::math_class::MathClass::Fence
                    || right_class == crate::entities::math_class::MathClass::Fence;
                if spaced {
                    self.metrics.advance(" ", style.size, style).val()
                } else {
                    0.0
                }
            }
        }
    }
}

fn resolve_gap(value: Rel<Length>, font: f64) -> f64 {
    value.resolve(Length::pt(font)).resolve_pt(font)
}

fn empty_box() -> MathBox {
    MathBox {
        width: 0.0,
        ascent: 0.0,
        descent: 0.0,
        items: Vec::new(),
    }
}

fn fold_augment(
    inner: &MathMatrixAugment,
    outer: &MathMatrixAugment,
) -> MathMatrixAugment {
    let stroke = match (&inner.stroke, &outer.stroke) {
        (None, Some(outer)) => Some(outer.clone()),
        (Some(inner), Some(outer)) => Some(fold_stroke(inner, outer)),
        (Some(inner), None) => Some(inner.clone()),
        (None, None) => None,
    };
    MathMatrixAugment {
        hline: inner.hline.clone(),
        vline: inner.vline.clone(),
        stroke,
    }
}

fn fold_stroke(inner: &Stroke, outer: &Stroke) -> Stroke {
    let mut result = inner.clone();
    let defaults = Stroke::default();
    if !inner.specified.paint && inner.paint == defaults.paint {
        result.paint = outer.paint.clone();
    }
    if !inner.specified.thickness && inner.thickness == defaults.thickness {
        result.thickness = outer.thickness;
    }
    if !inner.specified.cap {
        result.cap = outer.cap;
    }
    if !inner.specified.join {
        result.join = outer.join;
    }
    if !inner.specified.dash {
        result.dash = outer.dash.clone();
    }
    if !inner.specified.miter_limit {
        result.miter_limit = outer.miter_limit;
    }
    result
}

fn augment_stroke(
    augment: &MathMatrixAugment,
    font: f64,
) -> (f64, Option<crate::entities::layout_types::Color>, LineCap) {
    match &augment.stroke {
        Some(stroke) => (stroke.thickness, Some(stroke.paint.to_color()), stroke.cap),
        None => (0.05 * font, None, LineCap::Square),
    }
}

struct GridGeometry {
    col_widths: Vec<f64>,
    boundary_col_widths: Vec<f64>,
    row_ascent: Vec<f64>,
    row_descent: Vec<f64>,
    boundary_ascent: Vec<f64>,
    boundary_descent: Vec<f64>,
    row_baselines: Vec<f64>,
    column_gap: f64,
    width: f64,
    height: f64,
}

impl GridGeometry {
    fn measure<M: crate::compiler::layout::FontMetrics>(
        layouter: &MathLayouter<'_, M>,
        rows: &[Vec<MathBox>],
        boundary_rows: &[Vec<MathBox>],
        column_gap: f64,
        row_gap: f64,
        style: &TextStyle,
    ) -> Self {
        let columns = rows.iter().map(Vec::len).max().unwrap_or(0);
        let mut col_widths = vec![0.0_f64; columns];
        for row in rows {
            for (column, cell) in row.iter().enumerate() {
                col_widths[column] = col_widths[column].max(cell.width);
            }
        }
        let mut boundary_col_widths = vec![0.0_f64; columns];
        for row in boundary_rows {
            for (column, cell) in row.iter().enumerate() {
                boundary_col_widths[column] = boundary_col_widths[column].max(cell.width);
            }
        }
        let paren_style = TextStyle { cramped: true, ..style.clone() };
        let paren = layouter.layout_text_node(&EcoString::from("("), &paren_style);
        let row_ascent: Vec<_> = rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.ascent)
                    .fold(0.0, f64::max)
                    .max(paren.ascent)
            })
            .collect();
        let row_descent: Vec<_> = rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.descent)
                    .fold(0.0, f64::max)
                    .max(paren.descent)
            })
            .collect();
        let boundary_ascent = boundary_rows
            .iter()
            .map(|row| row.iter().map(|cell| cell.ascent).fold(0.0, f64::max))
            .collect();
        let boundary_descent = boundary_rows
            .iter()
            .map(|row| row.iter().map(|cell| cell.descent).fold(0.0, f64::max))
            .collect();
        let mut row_baselines = vec![0.0; rows.len()];
        for index in 1..rows.len() {
            row_baselines[index] = row_baselines[index - 1]
                + row_descent[index - 1]
                + row_gap
                + row_ascent[index];
        }
        let width = col_widths.iter().sum::<f64>()
            + columns.saturating_sub(1) as f64 * column_gap;
        let height = match (row_ascent.first(), row_descent.last(), row_baselines.last())
        {
            (Some(first), Some(last), Some(baseline)) => first + baseline + last,
            _ => 0.0,
        };
        Self {
            col_widths,
            boundary_col_widths,
            row_ascent,
            row_descent,
            boundary_ascent,
            boundary_descent,
            row_baselines,
            column_gap,
            width,
            height,
        }
    }

    fn normalize(offset: i64, dimension: usize) -> Option<usize> {
        let normalized = if offset < 0 { dimension as i64 + offset } else { offset };
        (0..=dimension as i64)
            .contains(&normalized)
            .then_some(normalized as usize)
    }

    fn vertical_boundary(&self, offset: i64) -> Option<f64> {
        let index = Self::normalize(offset, self.col_widths.len())?;
        if index == 0 {
            return Some(0.0);
        }
        if index == self.col_widths.len() {
            return Some(self.width);
        }
        Some(
            self.col_widths[..index].iter().sum::<f64>()
                + (index - 1) as f64 * self.column_gap
                + self.column_gap / 2.0
                + (self.boundary_col_widths[index - 1] - self.col_widths[index - 1])
                    / 2.0,
        )
    }

    fn horizontal_boundary(&self, offset: i64, interior_shift: f64) -> Option<f64> {
        let index = Self::normalize(offset, self.row_baselines.len())?;
        if index == 0 {
            return Some(0.0);
        }
        if index == self.row_baselines.len() {
            return Some(self.height);
        }
        let upper_bottom =
            self.row_baselines[index - 1] + self.boundary_descent[index - 1];
        let lower_top = self.row_baselines[index] - self.boundary_ascent[index];
        Some((upper_bottom + lower_top) / 2.0 + interior_shift)
    }
}

pub(super) fn split_cell_on_align_point(cell: &Content) -> Vec<Content> {
    match cell {
        Content::MathSequence(sequence) => {
            let mut parts = Vec::new();
            let mut current = Vec::new();
            for item in sequence.iter() {
                if matches!(item, Content::MathAlignPoint(_)) {
                    parts.push(std::mem::take(&mut current));
                } else {
                    current.push(item.clone());
                }
            }
            parts.push(current);
            parts
                .into_iter()
                .map(|part| match part.len() {
                    0 => Content::MathSequence(Arc::from(Vec::new())),
                    1 => part.into_iter().next().unwrap(),
                    _ => Content::MathSequence(Arc::from(part)),
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

pub(super) fn edge_node<'a>(cell: &'a Content, first: bool) -> &'a Content {
    if let Content::MathSequence(items) = cell {
        if let Some(node) = if first { items.first() } else { items.last() } {
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
