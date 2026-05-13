//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 089621fc
//! @layer L1
//! @updated 2026-05-13
//!
//! P224.C — Placement algorítmico Grid completo.
//! **Fecha DEBT-34e** (colspan/rowspan placement); DEBT-34d (Auto track
//! sizing greediness) é refino algorítmico distinto NÃO endereçado em P224
//! (refino futuro candidato Fase 5 NÃO-reservado per política P158).
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
#[derive(Debug, Clone, PartialEq)]
pub struct PlacedCell {
    /// Body original (clone do `GridCell.body` ou `Content` raw).
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
pub(crate) fn place_cells(
    cells: &[Content],
    num_cols: usize,
) -> SourceResult<Vec<PlacedCell>> {
    if num_cols == 0 {
        return Err(vec![SourceDiagnostic::error(
            Span::detached(),
            "grid_placement: num_cols deve ser >= 1".to_string(),
        )]);
    }

    // Occupancy grid: cresce dinamicamente. `occupied[row][col]` == true
    // se ocupado. `num_cols` colunas fixas; rows expandem conforme placement.
    let mut occupied: Vec<Vec<bool>> = Vec::new();
    let mut placed: Vec<PlacedCell> = Vec::new();

    // Separar cells em explicit (x ou y Some) vs auto (ambos None).
    let mut explicit: Vec<(usize, &Content)> = Vec::new();
    let mut auto:     Vec<(usize, &Content)> = Vec::new();
    for (idx, c) in cells.iter().enumerate() {
        match c {
            Content::GridCell { x, y, .. } if x.is_some() || y.is_some() => {
                explicit.push((idx, c));
            }
            _ => auto.push((idx, c)),
        }
    }

    // Pass 1 — explicit placement.
    for (_idx, cell) in &explicit {
        let (body, x, y, cs, rs) = extract_cell_fields(cell);
        let colspan = cs.unwrap_or(1).max(1);
        let rowspan = rs.unwrap_or(1).max(1);

        // Validar colspan cabe.
        let col_start = x.unwrap_or(0);
        if col_start + colspan > num_cols {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "grid placement: cell em x={} colspan={} excede num_cols={}",
                    col_start, colspan, num_cols
                ),
            )]);
        }

        // Row: explicit se y Some; auto se y None (próxima linha livre nessa coluna).
        let row_start = match y {
            Some(r) => r,
            None => find_next_free_row(&occupied, col_start, colspan),
        };

        // Expandir occupancy para rowspan.
        ensure_rows(&mut occupied, row_start + rowspan, num_cols);

        // Detectar conflito.
        for r in row_start..row_start + rowspan {
            for c in col_start..col_start + colspan {
                if occupied[r][c] {
                    return Err(vec![SourceDiagnostic::error(
                        Span::detached(),
                        format!(
                            "grid placement: conflito — célula explicit (x={}, y={}) ocupa posição já ocupada ({}, {})",
                            col_start, row_start, c, r
                        ),
                    )]);
                }
            }
        }

        // Marcar ocupação.
        for r in row_start..row_start + rowspan {
            for c in col_start..col_start + colspan {
                occupied[r][c] = true;
            }
        }

        placed.push(PlacedCell {
            body: body.clone(),
            row:  row_start,
            col:  col_start,
            colspan,
            rowspan,
        });
    }

    // Pass 2 — auto placement linear.
    let mut cursor_row: usize = 0;
    let mut cursor_col: usize = 0;
    for (_idx, cell) in &auto {
        let (body, _, _, cs, rs) = extract_cell_fields(cell);
        let colspan = cs.unwrap_or(1).max(1);
        let rowspan = rs.unwrap_or(1).max(1);

        // Validar colspan cabe.
        if colspan > num_cols {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!(
                    "grid placement: cell colspan={} excede num_cols={}",
                    colspan, num_cols
                ),
            )]);
        }

        // Avançar cursor até encontrar posição livre que acomoda colspan × rowspan.
        loop {
            // Wrap se colspan não cabe.
            if cursor_col + colspan > num_cols {
                cursor_col = 0;
                cursor_row += 1;
            }
            ensure_rows(&mut occupied, cursor_row + rowspan, num_cols);

            let mut fits = true;
            'outer: for r in cursor_row..cursor_row + rowspan {
                for c in cursor_col..cursor_col + colspan {
                    if occupied[r][c] {
                        fits = false;
                        break 'outer;
                    }
                }
            }
            if fits {
                break;
            }
            // Avançar 1 coluna; cursor_col wrap em próxima iteração.
            cursor_col += 1;
        }

        // Marcar ocupação.
        for r in cursor_row..cursor_row + rowspan {
            for c in cursor_col..cursor_col + colspan {
                occupied[r][c] = true;
            }
        }

        placed.push(PlacedCell {
            body: body.clone(),
            row:  cursor_row,
            col:  cursor_col,
            colspan,
            rowspan,
        });

        cursor_col += colspan;
    }

    Ok(placed)
}

/// Extrai fields de `Content::GridCell` ou trata `Content` raw como
/// (body=self, x=None, y=None, colspan=None, rowspan=None).
fn extract_cell_fields(c: &Content) -> (&Content, Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    match c {
        Content::GridCell { body, x, y, colspan, rowspan } => {
            (body.as_ref(), *x, *y, *colspan, *rowspan)
        }
        other => (other, None, None, None, None),
    }
}

/// Procura próxima linha livre na coluna `col_start` (com `colspan` colunas
/// contíguas). Usado pelo pass 1 quando `y: None` mas `x: Some`.
fn find_next_free_row(occupied: &[Vec<bool>], col_start: usize, colspan: usize) -> usize {
    for r in 0..=occupied.len() {
        if r == occupied.len() {
            return r;  // próxima linha não-ocupada.
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
        let cells = vec![
            Content::GridCell {
                body:    Box::new(Content::text("X")),
                x:       Some(1),
                y:       Some(1),
                colspan: None,
                rowspan: None,
            },
        ];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed.len(), 1);
        assert_eq!((placed[0].row, placed[0].col), (1, 1));
    }

    #[test]
    fn p224_placement_colspan_ocupa_adjacente() {
        // GridCell colspan=2 ocupa 2 colunas; cell auto seguinte vai p/ row 1.
        let cells = vec![
            Content::GridCell {
                body:    Box::new(Content::text("WIDE")),
                x:       None,
                y:       None,
                colspan: Some(2),
                rowspan: None,
            },
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
            Content::GridCell {
                body:    Box::new(Content::text("TALL")),
                x:       None,
                y:       None,
                colspan: None,
                rowspan: Some(2),
            },
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
            Content::GridCell {
                body:    Box::new(Content::text("X")),
                x:       Some(0),
                y:       Some(0),
                colspan: None,
                rowspan: None,
            },
            Content::GridCell {
                body:    Box::new(Content::text("Y")),
                x:       Some(0),
                y:       Some(0),
                colspan: None,
                rowspan: None,
            },
        ];
        let r = place_cells(&cells, 2);
        assert!(r.is_err(), "conflito 2-cells em (0,0) deve falhar");
    }

    #[test]
    fn p224_placement_colspan_excede_num_cols_rejeita() {
        let cells = vec![
            Content::GridCell {
                body:    Box::new(Content::text("HUGE")),
                x:       None,
                y:       None,
                colspan: Some(5),
                rowspan: None,
            },
        ];
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
            Content::GridCell {
                body:    Box::new(Content::text("B")),
                x:       Some(1),
                y:       Some(0),
                colspan: None,
                rowspan: None,
            },
            Content::text("C"),
        ];
        let placed = place_cells(&cells, 2).unwrap();
        assert_eq!(placed.len(), 3);
        // Pass 1: explicit cell B em (0,1).
        // Pass 2: auto A em (0,0), C em (1,0).
        let positions: Vec<(usize, usize)> = placed.iter()
            .map(|p| (p.row, p.col))
            .collect();
        // Order of placed depends on processing order: explicit first then auto.
        assert!(positions.contains(&(0, 1)), "B em (0,1)");
        assert!(positions.contains(&(0, 0)), "A em (0,0)");
        assert!(positions.contains(&(1, 0)), "C em (1,0)");
    }
}
