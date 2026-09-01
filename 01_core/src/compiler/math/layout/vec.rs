//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/vec.md
//! @prompt-hash fe3f2d14
//! @layer L1
//! @updated 2026-09-01
//!
//! Layout de `MathVec`: linhas em estilo denominador e gap relativo à região.

use crate::compiler::layout::FontMetrics;
use crate::entities::elements::math_vec::MathVecElem;
use crate::entities::layout_types::{FrameItem, HAlign, Pt, TextStyle};

use super::{offset_item, GridAlign, MathBox, MathLayouter};

pub(super) fn layout<M: FontMetrics>(
    layouter: &MathLayouter<'_, M>,
    elem: &MathVecElem,
    style: &TextStyle,
) -> MathBox {
    // O vetor vazio conserva identidade no conteúdo, mas não materializa
    // delimitadores nem caixa geométrica.
    if elem.children.is_empty() {
        return MathBox {
            width: 0.0,
            ascent: 0.0,
            descent: 0.0,
            items: vec![],
        };
    }

    let cell_style = layouter.denominator_style(style);
    let rows = elem
        .children
        .iter()
        .map(|child| vec![layouter.layout_node(child, &cell_style)])
        .collect();
    let align = match elem.align {
        HAlign::Left | HAlign::Start => GridAlign::Left,
        HAlign::Center => GridAlign::Center,
        HAlign::Right | HAlign::End => GridAlign::Right,
    };
    let relative = if layouter.region_height.val().is_finite() {
        elem.gap.rel * layouter.region_height.val()
    } else {
        0.0
    };
    let row_gap = relative + elem.gap.abs.resolve_pt(style.size.val());
    let grid =
        layouter.layout_grid_boxes(rows, align, Pt(0.0), Pt(row_gap), &[], &cell_style);

    let target = layouter.grid_delim_target_du(&grid, style);
    let empty = || MathBox {
        width: 0.0,
        ascent: 0.0,
        descent: 0.0,
        items: vec![],
    };
    let left = if elem.delim.0 == '\0' {
        empty()
    } else {
        layouter.layout_stretchy_delimiter(elem.delim.0, target, style)
    };
    let right = if elem.delim.1 == '\0' {
        empty()
    } else {
        layouter.layout_stretchy_delimiter(elem.delim.1, target, style)
    };
    let grid = layouter.apply_axis_offset(grid, style.size);

    let mut items: Vec<FrameItem> = Vec::new();
    let mut x = Pt(0.0);
    for item in &left.items {
        items.push(offset_item(item.clone(), x, Pt(0.0)));
    }
    x += Pt(left.width);
    for item in &grid.items {
        items.push(offset_item(item.clone(), x, Pt(0.0)));
    }
    x += Pt(grid.width);
    for item in &right.items {
        items.push(offset_item(item.clone(), x, Pt(0.0)));
    }
    x += Pt(right.width);

    MathBox {
        width: x.val(),
        ascent: grid.ascent.max(left.ascent).max(right.ascent),
        descent: grid.descent.max(left.descent).max(right.descent),
        items,
    }
}
