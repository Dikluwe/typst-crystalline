//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid_placement.md
//! @prompt-hash 2a5d7af9
//! @layer L1
//! @updated 2026-10-07
//!
//! P224.C — Placement algorítmico Grid completo.
//! **Fecha DEBT-34e** (colspan/rowspan placement); DEBT-34d (Auto track
//! sizing greediness) FECHADO P233.
//!
//! P234 — `PlacedCell.body` preserva outer cell (`Content::GridCell {...}`
//! ou raw) em vez de strip wrapper. Permite consumer geometric P234
//! `layout_grid` extrair per-cell stroke/fill via match em `placed.body`
//! preservando precedência P230.
//!
//! Algoritmo paridade vanilla `layout/grid/cells.rs`:
//! - Cells `Content::GridCell` com `x`/`y` explícitos → posição fixada.
//! - Cells com `x: None` + `y: None` → auto-placement linear (left-to-right,
//!   top-to-bottom) procurando próxima posição livre.
//! - `colspan: Some(n)` / `rowspan: Some(n)` ocupam N colunas/linhas
//!   adjacentes (default 1).
//! - Conflito (2 cells na mesma posição) → erro hard.
//! - Cells "raw" (não-`GridCell`) tratadas como `colspan=1, rowspan=1`,
//!   sem placement explícito (auto-placement linear).
//!
//! Trabalho L1 puro (algorítmico; não toca layout geometric).
//! Layouter consome `Vec<PlacedCell>` para iteração.

use crate::entities::content::Content;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::span::Span;

/// Célula com posição resolvida pós-placement.
///
/// **P234** — `body` preserva outer cell (`Content::GridCell {...}` ou
/// raw) em vez de strip wrapper inner body. Consumer (P234 `layout_grid`)
/// extrai per-cell stroke/fill via match em `body` preservando
/// precedência P230. Para layout do conteúdo interno, consumer faz
/// `match &placed.body { GridCell { body, .. } => body.as_ref(),
/// other => other }`.
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedCell {
    /// Outer cell preservado (`Content::GridCell {...}` ou raw).
    pub body: Content,
    /// Linha 0-indexed onde a célula começa.
    pub row: usize,
    /// Coluna 0-indexed onde a célula começa.
    pub col: usize,
    /// Número de colunas ocupadas (>= 1).
    pub colspan: usize,
    /// Número de linhas ocupadas (>= 1).
    pub rowspan: usize,
}

/// Resolve placement de cells dentro de grid de `num_cols` colunas.
///
/// Algoritmo:
/// 1. Pass 1 — placement explicit: cells `GridCell` com `x` ou `y` Some
///    são posicionadas literalmente. Conflito 2-cells na mesma área → erro.
/// 2. Pass 2 — placement auto: cells restantes (sem x/y) são posicionadas
///    em ordem linear procurando primeira posição livre.
///
/// `num_cols` validação: cells com `colspan` excedendo `num_cols` rejeitadas.
#[derive(Clone, Copy)]
enum PlacementPolicy {
    GridExplicitFirst,
    TableDocumentOrder,
}

pub(crate) fn place_cells(
    cells: &[Content],
    num_cols: usize,
) -> SourceResult<Vec<PlacedCell>> {
    place(cells, num_cols, 0, PlacementPolicy::GridExplicitFirst)
}

pub(crate) fn place_table_cells(
    cells: &[Content],
    num_cols: usize,
    minimum_rows: usize,
) -> SourceResult<Vec<PlacedCell>> {
    place(cells, num_cols, minimum_rows, PlacementPolicy::TableDocumentOrder)
}

fn place(
    cells: &[Content],
    num_cols: usize,
    minimum_rows: usize,
    policy: PlacementPolicy,
) -> SourceResult<Vec<PlacedCell>> {
    if num_cols == 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid_placement: num_cols deve ser >= 1",
        )]);
    }
    let table = matches!(policy, PlacementPolicy::TableDocumentOrder);
    let fields = |c: &Content| match c {
        Content::TableCell(e) if table => {
            let (x, y, c, r) = e.placement();
            (x, y, c, r, e.source_span)
        }
        Content::GridCell(e) if !table => {
            (e.x, e.y, e.colspan, e.rowspan, Span::detached())
        }
        _ => (None, None, None, None, Span::detached()),
    };
    let mut ordered = cells.iter().collect::<Vec<_>>();
    if !table {
        ordered.sort_by_key(|c| {
            let (x, y, _, _, _) = fields(c);
            x.is_none() && y.is_none()
        });
    }
    let mut occupied: Vec<Vec<bool>> = Vec::new();
    let mut placed = Vec::new();
    let mut cursor = 0usize;
    for cell in ordered {
        let (x, y, cs, rs, span) = fields(cell);
        let colspan = cs.unwrap_or(1).max(1);
        let rowspan = rs.unwrap_or(1).max(1);
        let fail = |msg: &str| vec![SourceDiagnostic::error(span, msg)];
        if x.is_some_and(|c| c >= num_cols) {
            return Err(fail(&format!(
                "cell could not be placed at invalid column {}",
                x.unwrap()
            )));
        }
        let badspan = || {
            vec![SourceDiagnostic::error(
                span,
                "cell's colspan would cause it to exceed the available column(s)",
            )
            .with_hint(
                "try placing the cell in another position or reducing its colspan",
            )]
        };
        if colspan > num_cols
            || x.is_some_and(|c| c.checked_add(colspan).is_none_or(|end| end > num_cols))
        {
            return Err(badspan());
        }
        let (row, col) = match (x, y) {
            (Some(c), Some(r)) => (r, c),
            (Some(c), None) => {
                (find_next_free_row(&occupied, c, if table { 1 } else { colspan }), c)
            }
            (None, Some(r)) => {
                let c = if table {
                    (0..num_cols).find(|&c| r >= occupied.len() || !occupied[r][c])
                } else {
                    Some(0)
                };
                let c = if table {
                    c.ok_or_else(|| {
                        vec![SourceDiagnostic::error(
                            span,
                            format!(
                                "cell could not be placed in row {r} because it was full"
                            ),
                        )
                        .with_hint("try specifying your cells in a different order")]
                    })?
                } else {
                    c.unwrap_or(0)
                };
                if c + colspan > num_cols {
                    return Err(badspan());
                }
                (r, c)
            }
            (None, None) => loop {
                let r = cursor / num_cols;
                let c = cursor % num_cols;
                if table {
                    if r >= occupied.len() || !occupied[r][c] {
                        if c + colspan > num_cols {
                            return Err(badspan());
                        }
                        break (r, c);
                    }
                    cursor = cursor
                        .checked_add(1)
                        .ok_or_else(|| fail("cell position too large"))?;
                    continue;
                }
                if c + colspan > num_cols {
                    cursor = cursor
                        .checked_add(num_cols - c)
                        .ok_or_else(|| fail("cell position too large"))?;
                    continue;
                }
                let end = r
                    .checked_add(rowspan)
                    .ok_or_else(|| fail("cell position too large"))?;
                let fits = (r..end).all(|rr| {
                    (c..c + colspan).all(|cc| rr >= occupied.len() || !occupied[rr][cc])
                });
                if fits {
                    break (r, c);
                }
                cursor = cursor
                    .checked_add(1)
                    .ok_or_else(|| fail("cell position too large"))?;
            },
        };
        let end = row
            .checked_add(rowspan)
            .ok_or_else(|| fail("cell position too large"))?;
        end.checked_mul(num_cols)
            .ok_or_else(|| fail("cell position too large"))?;
        occupied
            .try_reserve(end.saturating_sub(occupied.len()))
            .map_err(|_| fail("cell position too large"))?;
        ensure_rows(&mut occupied, end, num_cols);
        for (row_offset, slots) in occupied[row..end].iter().enumerate() {
            let r = row + row_offset;
            for (col_offset, &used) in slots[col..col + colspan].iter().enumerate() {
                let c = col + col_offset;
                if used {
                    let diag = if r == row && c == col {
                        SourceDiagnostic::error(
                            span,
                            format!(
                                "attempted to place a second cell at column {c}, row {r}"
                            ),
                        )
                        .with_hint("try specifying your cells in a different order")
                    } else {
                        SourceDiagnostic::error(span,format!("cell would span a previously placed cell at column {c}, row {r}")).with_hint("try specifying your cells in a different order or reducing the cell's rowspan or colspan")
                    };
                    return Err(vec![diag]);
                }
            }
        }
        for slots in &mut occupied[row..end] {
            slots[col..col + colspan].fill(true);
        }
        placed.push(PlacedCell { body: cell.clone(), row, col, colspan, rowspan });
        if x.is_none() && y.is_none() {
            cursor = row
                .checked_mul(num_cols)
                .and_then(|n| {
                    n.checked_add(if table && colspan == num_cols {
                        colspan.checked_mul(rowspan)?
                    } else {
                        col + colspan
                    })
                })
                .ok_or_else(|| fail("cell position too large"))?;
        }
    }
    if table {
        occupied
            .try_reserve(minimum_rows.saturating_sub(occupied.len()))
            .map_err(|_| {
                vec![SourceDiagnostic::error(Span::detached(), "cell position too large")]
            })?;
        ensure_rows(&mut occupied, minimum_rows, num_cols);
        for (row, line) in occupied.iter().enumerate() {
            for (col, &used) in line.iter().enumerate() {
                if !used {
                    placed.push(PlacedCell {
                        body: Content::Empty,
                        row,
                        col,
                        colspan: 1,
                        rowspan: 1,
                    });
                }
            }
        }
        placed.sort_by_key(|c| (c.row, c.col));
    }
    Ok(placed)
}

/// Procura próxima linha livre na coluna `col_start` (com `colspan` colunas
/// contíguas). Usado pelo pass 1 quando `y: None` mas `x: Some`.
fn find_next_free_row(occupied: &[Vec<bool>], col_start: usize, colspan: usize) -> usize {
    for r in 0..=occupied.len() {
        if r == occupied.len() {
            return r; // próxima linha não-ocupada.
        }
        let mut all_free = true;
        for c in col_start..col_start + colspan {
            if c < occupied[r].len() && occupied[r][c] {
                all_free = false;
                break;
            }
        }
        if all_free {
            return r;
        }
    }
    occupied.len()
}

/// Garante que `occupied` tem pelo menos `target_rows` linhas; expande
/// preenchendo com `false`.
fn ensure_rows(occupied: &mut Vec<Vec<bool>>, target_rows: usize, num_cols: usize) {
    while occupied.len() < target_rows {
        occupied.push(vec![false; num_cols]);
    }
}

pub(crate) fn table_groups(
    e: &crate::entities::elements::table::TableElem,
    cols: usize,
) -> SourceResult<(Vec<Content>, usize, usize)> {
    super::grid::row_groups::table_groups(e, cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p224_placement_auto_linear() {
        // 4 cells raw → posicionadas (0,0) (0,1) (1,0) (1,1) em 2 cols.
        let cells = vec![
            Content::text("A"),
            Content::text("B"),
            Content::text("C"),
            Content::text("D"),
        ];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed.len(), 4);
        assert_eq!((placed[0].row, placed[0].col), (0, 0));
        assert_eq!((placed[1].row, placed[1].col), (0, 1));
        assert_eq!((placed[2].row, placed[2].col), (1, 0));
        assert_eq!((placed[3].row, placed[3].col), (1, 1));
    }

    #[test]
    fn p224_placement_explicit_x_y() {
        // GridCell { x: Some(1), y: Some(1) } → posicionada literal.
        let cells = vec![Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("X"),
                x: Some(1),
                y: Some(1),
                colspan: None,
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ))];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed.len(), 1);
        assert_eq!((placed[0].row, placed[0].col), (1, 1));
    }

    #[test]
    fn p224_placement_colspan_ocupa_adjacente() {
        // GridCell colspan=2 ocupa 2 colunas; cell auto seguinte vai p/ row 1.
        let cells = vec![
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("WIDE"),
                    x: None,
                    y: None,
                    colspan: Some(2),
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            )),
            Content::text("next"),
        ];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed.len(), 2);
        assert_eq!(placed[0].colspan, 2);
        assert_eq!((placed[0].row, placed[0].col), (0, 0));
        assert_eq!((placed[1].row, placed[1].col), (1, 0));
    }

    #[test]
    fn p224_placement_rowspan_ocupa_linhas() {
        // GridCell rowspan=2 + cell raw em (1,0) conflito? Não — cell raw
        // wrap para (1,1) porque (1,0) está ocupada por rowspan.
        // Wait — placement auto: cursor avança col por col. Após cell 1
        // (rowspan=2 em (0,0)+(1,0)), cursor está em col=1. cell 2 vai
        // p/ (0,1). cell 3 vai p/ (1,1).
        let cells = vec![
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("TALL"),
                    x: None,
                    y: None,
                    colspan: None,
                    rowspan: Some(2),
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            )),
            Content::text("b"),
            Content::text("c"),
        ];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed.len(), 3);
        assert_eq!(placed[0].rowspan, 2);
        assert_eq!((placed[0].row, placed[0].col), (0, 0));
        assert_eq!((placed[1].row, placed[1].col), (0, 1));
        assert_eq!((placed[2].row, placed[2].col), (1, 1));
    }

    #[test]
    fn p224_placement_conflito_explicit_explicit_rejeita() {
        let cells = vec![
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("X"),
                    x: Some(0),
                    y: Some(0),
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            )),
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("Y"),
                    x: Some(0),
                    y: Some(0),
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            )),
        ];
        let r = place_cells(&cells, 2);
        assert!(r.is_err(), "conflito 2-cells em (0,0) deve falhar");
    }

    #[test]
    fn p224_placement_colspan_excede_num_cols_rejeita() {
        let cells = vec![Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("HUGE"),
                x: None,
                y: None,
                colspan: Some(5),
                rowspan: None,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ))];
        let r = place_cells(&cells, 2);
        assert!(r.is_err(), "colspan=5 > num_cols=2 deve falhar");
    }

    #[test]
    fn p224_placement_mistura_auto_e_explicit() {
        // (0,0) raw → cell A
        // (0,1) explicit → cell B
        // auto next → (1,0) cell C
        let cells = vec![
            Content::text("A"),
            Content::GridCell(std::sync::Arc::new(
                crate::entities::elements::grid_cell::GridCellElem {
                    body: Content::text("B"),
                    x: Some(1),
                    y: Some(0),
                    colspan: None,
                    rowspan: None,
                    stroke: None,
                    fill: None,
                    align: None,
                    inset: None,
                    breakable: None,
                },
            )),
            Content::text("C"),
        ];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed.len(), 3);
        // Pass 1: explicit cell B em (0,1).
        // Pass 2: auto A em (0,0), C em (1,0).
        let positions: Vec<(usize, usize)> =
            placed.iter().map(|p| (p.row, p.col)).collect();
        // Order of placed depends on processing order: explicit first then auto.
        assert!(positions.contains(&(0, 1)), "B em (0,1)");
        assert!(positions.contains(&(0, 0)), "A em (0,0)");
        assert!(positions.contains(&(1, 0)), "C em (1,0)");
    }

    // ── P822 — mensagens de erro em paridade verbatim com o vanilla ─────
    // Achado #9(b) de P810. Referências vanilla
    // (`typst-library/src/layout/grid/resolve.rs`):
    // - conflito célula↔célula: resolve.rs:1500-1504;
    // - span sobre célula prévia: resolve.rs:1522-1530;
    // - coluna inválida: resolve.rs:2221-2225;
    // - colspan overflow: resolve.rs:1413-1419.
    // Mensagens e hints medidos nos dois binários (sonda P822, temp/p822/).

    /// Helper local dos testes P822: constrói `Content::GridCell` com os
    /// campos de placement pedidos (cosméticos a None).
    fn p822_cell(
        x: Option<usize>,
        y: Option<usize>,
        colspan: Option<usize>,
        rowspan: Option<usize>,
    ) -> Content {
        Content::GridCell(std::sync::Arc::new(
            crate::entities::elements::grid_cell::GridCellElem {
                body: Content::text("C"),
                x,
                y,
                colspan,
                rowspan,
                stroke: None,
                fill: None,
                align: None,
                inset: None,
                breakable: None,
            },
        ))
    }

    #[test]
    fn p822_conflito_segunda_celula_mensagem_vanilla() {
        let cells = vec![
            p822_cell(Some(0), Some(0), None, None),
            p822_cell(Some(0), Some(0), None, None),
        ];
        let err = place_cells(&cells, 2).unwrap_err();
        assert_eq!(
            err[0].message, "attempted to place a second cell at column 0, row 0",
            "paridade vanilla resolve.rs:1502"
        );
        assert_eq!(
            err[0].hints,
            vec!["try specifying your cells in a different order".to_string()],
            "hint em paridade vanilla resolve.rs:1503"
        );
    }

    #[test]
    fn p822_span_sobre_celula_previa_mensagem_vanilla() {
        // A em (1,0); B em (0,0) com colspan=2 — a origem de B está livre,
        // mas a posição spanned (1,0) está ocupada.
        let cells = vec![
            p822_cell(Some(1), Some(0), None, None),
            p822_cell(Some(0), Some(0), Some(2), None),
        ];
        let err = place_cells(&cells, 2).unwrap_err();
        assert_eq!(
            err[0].message, "cell would span a previously placed cell at column 1, row 0",
            "paridade vanilla resolve.rs:1524-1526"
        );
        assert_eq!(
            err[0].hints,
            vec![
                "try specifying your cells in a different order or reducing the cell's rowspan or colspan"
                    .to_string()
            ],
            "hint em paridade vanilla resolve.rs:1527-1528"
        );
    }

    #[test]
    fn p822_coluna_invalida_mensagem_vanilla() {
        // x explícito fora da grid: erro dedicado, sem hint (medido no
        // vanilla — sonda P822, caso e_invalid_column).
        let cells = vec![p822_cell(Some(5), Some(0), None, None)];
        let err = place_cells(&cells, 2).unwrap_err();
        assert_eq!(
            err[0].message, "cell could not be placed at invalid column 5",
            "paridade vanilla resolve.rs:2223"
        );
        assert!(err[0].hints.is_empty(), "vanilla não emite hint aqui");
    }

    #[test]
    fn p822_colspan_overflow_explicito_mensagem_vanilla() {
        // x=1 + colspan=2 em grid de 2 colunas (cabe a coluna, não o span).
        let cells = vec![p822_cell(Some(1), None, Some(2), None)];
        let err = place_cells(&cells, 2).unwrap_err();
        assert_eq!(
            err[0].message,
            "cell's colspan would cause it to exceed the available column(s)",
            "paridade vanilla resolve.rs:1415"
        );
        assert_eq!(
            err[0].hints,
            vec!["try placing the cell in another position or reducing its colspan"
                .to_string()],
            "hint em paridade vanilla resolve.rs:1416-1417"
        );
    }

    #[test]
    fn p822_colspan_overflow_auto_mensagem_vanilla() {
        let cells = vec![p822_cell(None, None, Some(5), None)];
        let err = place_cells(&cells, 2).unwrap_err();
        assert_eq!(
            err[0].message,
            "cell's colspan would cause it to exceed the available column(s)",
            "paridade vanilla resolve.rs:1415"
        );
        assert_eq!(
            err[0].hints,
            vec!["try placing the cell in another position or reducing its colspan"
                .to_string()],
            "hint em paridade vanilla resolve.rs:1416-1417"
        );
    }

    // ── P1043: Pares de independência testcase() para grid_placement.rs:85 ───

    #[test]
    fn p1043_grid_placement_x_only_isolada() {
        // C1=T, C2=F: e.x.is_some() && e.y.is_none() -> explicit
        let cells = vec![p822_cell(Some(1), None, None, None)];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed[0].col, 1);
        assert_eq!(placed[0].row, 0);
    }

    #[test]
    fn p1043_grid_placement_y_only_isolada() {
        // C1=F, C2=T: e.x.is_none() && e.y.is_some() -> explicit
        let cells = vec![p822_cell(None, Some(1), None, None)];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed[0].col, 0);
        assert_eq!(placed[0].row, 1);
    }

    #[test]
    fn p1043_grid_placement_auto_isolada() {
        // C1=F, C2=F: e.x.is_none() && e.y.is_none() -> auto
        let cells = vec![p822_cell(None, None, None, None)];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed[0].col, 0);
        assert_eq!(placed[0].row, 0);
    }
}
