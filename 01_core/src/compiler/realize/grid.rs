//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/realize/grid.md
//! @prompt-hash 63996cbf
//! @layer L1
//! @updated 2026-11-25
//!
//! Preparation of `grid` at its insertion point: declared `set grid`
//! properties and per-cell values given as arrays or functions.

use std::sync::Arc;

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::grid::{GridChildOrder, GridElem};
use crate::entities::elements::grid_cell::GridCellElem;
use crate::entities::engine::Engine;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Properties that may apply per cell.
const CELLED: [&str; 4] = ["fill", "stroke", "align", "inset"];

/// A grid built by the constructor whose declarations are not applied yet.
pub(super) fn needs_preparation(e: &GridElem) -> bool {
    e.spec.as_ref().is_some_and(|spec| !spec.prepared)
}

/// The value of a per-cell property for the cell at column `x`, row `y`.
pub(super) fn cell_value(
    value: &Value,
    x: usize,
    y: usize,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Value> {
    match value {
        Value::Array(items) if items.is_empty() => Ok(Value::None),
        Value::Array(items) => Ok(items[x % items.len()].clone()),
        Value::Func(func) => {
            let mut scopes = crate::compiler::scopes::Scopes::new(None);
            crate::compiler::eval::call_dispatch::apply_func(
                func.clone(),
                Args::positional(vec![Value::Int(x as i64), Value::Int(y as i64)]),
                &mut scopes,
                ctx,
                engine,
            )
        }
        other => Ok(other.clone()),
    }
}

/// Applies a per-cell value to a grid cell that does not set it itself.
fn apply(cell: &mut GridCellElem, name: &str, value: &Value) -> SourceResult<()> {
    let span = crate::entities::span::Span::detached();
    let error = |expected: &str| {
        vec![SourceDiagnostic::error(
            span,
            format!(
                "expected {expected}, found {}",
                crate::compiler::eval::operators::error_formatting::vanilla_type_name(value)
            ),
        )]
    };
    match name {
        "fill" if cell.fill.is_none() => match value {
            Value::Color(color) => cell.fill = Some(color.clone()),
            Value::None | Value::Auto => {}
            _ => return Err(error("color, gradient, tiling, or none")),
        },
        "stroke" if cell.stroke.is_none() => match value {
            Value::None | Value::Auto => {}
            _ => {
                cell.stroke =
                    Some(crate::compiler::stdlib::extract_stroke(value, "grid", "stroke")?)
            }
        },
        "align" if cell.align.is_none() => match value {
            Value::Align(align) => cell.align = Some(*align),
            Value::Auto | Value::None => {}
            _ => return Err(error("alignment or auto")),
        },
        "inset" if cell.inset.is_none() => {
            let sides = crate::entities::elements::table_cell::TableCellProperties::style_inset(
                value,
            )
            .ok_or_else(|| error("relative length or dictionary"))?;
            let zero = crate::entities::layout_types::Length::ZERO;
            cell.inset = Some(crate::entities::sides::Sides::new(
                sides.left.unwrap_or(zero),
                sides.top.unwrap_or(zero),
                sides.right.unwrap_or(zero),
                sides.bottom.unwrap_or(zero),
            ));
        }
        _ => {}
    }
    Ok(())
}

/// The positional children of a grid in source order, as the constructor took them.
fn children(e: &GridElem) -> Vec<Value> {
    let order = e.source_order.clone().unwrap_or_else(|| {
        (0..e.cells.len()).map(GridChildOrder::Cell).collect()
    });
    order
        .iter()
        .filter_map(|item| match item {
            GridChildOrder::Cell(i) => e.cells.get(*i).cloned(),
            GridChildOrder::Header => e.header.clone(),
            GridChildOrder::Footer => e.footer.clone(),
            GridChildOrder::HLine(i) => {
                e.hlines.get(*i).map(|h| Content::GridHLine(Arc::new(h.clone())))
            }
            GridChildOrder::VLine(i) => {
                e.vlines.get(*i).map(|v| Content::GridVLine(Arc::new(v.clone())))
            }
        })
        .map(Value::Content)
        .collect()
}

/// Applies `set grid` and per-cell values at the insertion point.
pub(super) fn prepare(
    e: &GridElem,
    ctx: &mut EvalContext,
    engine: &mut Engine<'_>,
) -> SourceResult<Content> {
    let Some(spec) = &e.spec else { return Ok(Content::Grid(Arc::new(e.clone()))) };
    let mut args = Args::positional(children(e));
    let mut declared = false;
    for name in crate::compiler::stdlib::GRID_PROPERTIES {
        if let Some(value) = spec.named(name) {
            args.named.insert(name.into(), value.clone());
        } else if let Some(value) = engine.styles.custom(&format!("grid.{name}")) {
            args.named.insert(name.into(), value.clone());
            declared = true;
        }
    }
    // `gutter` declared by `set` yields to the specific gutters given here.
    let mut grid = if declared {
        let world = engine.world;
        let file = engine.current_file;
        match crate::compiler::stdlib::native_grid(ctx, &args, world, file)? {
            Value::Content(Content::Grid(grid)) => grid.as_ref().clone(),
            _ => e.clone(),
        }
    } else {
        e.clone()
    };
    let celled: Vec<_> = CELLED
        .iter()
        .filter_map(|name| {
            args.named
                .get(*name)
                .filter(|value| {
                    matches!(value, Value::Array(_) | Value::Func(_))
                        || (*name == "stroke" && matches!(value, Value::Dict(_)))
                })
                .map(|value| (*name, value.clone()))
        })
        .collect();
    // Declared cell values apply before the grid's own per-cell values.
    let declared_cell: Vec<(&str, Value)> = CELLED
        .iter()
        .filter_map(|name| {
            engine
                .styles
                .custom(&format!("grid.cell.{name}"))
                .map(|value| (*name, value.clone()))
        })
        .collect();
    if !celled.is_empty() || !declared_cell.is_empty() {
        let columns = grid.columns.len().max(1);
        let placed = crate::compiler::layout::grid_placement::place_grid_cells(
            &grid.cells,
            columns,
        )?;
        for (index, x, y) in placed {
            // A free slot becomes an empty cell so that it is painted too.
            let source = index.map_or(Content::Empty, |index| grid.cells[index].clone());
            let mut cell = match source.source_body() {
                Content::GridCell(cell) => cell.as_ref().clone(),
                _ => GridCellElem {
                    body: source.clone(),
                    x: None,
                    y: None,
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            };
            for (name, value) in &declared_cell {
                apply(&mut cell, name, value)?;
            }
            for (name, value) in &celled {
                let value = cell_value(value, x, y, ctx, engine)?;
                apply(&mut cell, name, &value)?;
            }
            match index {
                Some(index) => grid.cells[index] = Content::GridCell(Arc::new(cell)),
                None => {
                    if let Some(order) = grid.source_order.as_mut() {
                        order.push(GridChildOrder::Cell(grid.cells.len()));
                    }
                    grid.cells.push(Content::GridCell(Arc::new(cell)));
                }
            }
        }
    }
    let mut spec = grid.spec.as_deref().cloned().unwrap_or_default();
    spec.prepared = true;
    grid.spec = Some(Arc::new(spec));
    Ok(Content::Grid(Arc::new(grid)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arrays_cycle_by_column() {
        assert_eq!(CELLED.len(), 4);
        let value = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        let Value::Array(items) = &value else { unreachable!() };
        assert_eq!(items[3 % items.len()], Value::Int(2));
    }

    #[test]
    fn built_grids_need_no_preparation() {
        let grid = GridElem {
            source_order: None,
            columns: vec![],
            rows: vec![],
            cells: vec![],
            hlines: vec![],
            vlines: vec![],
            gutter: None,
            align: None,
            inset: crate::entities::sides::Sides::uniform(
                crate::entities::layout_types::Length::ZERO,
            ),
            header: None,
            footer: None,
            stroke: None,
            fill: None,
            spec: None,
        };
        assert!(!needs_preparation(&grid));
    }
}
