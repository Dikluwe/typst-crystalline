//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout.md
//! @prompt-hash 0450974a
//! @layer L1
//! @updated 2026-05-13
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
    let mut auto: Vec<(usize, &Content)> = Vec::new();
    for (idx, c) in cells.iter().enumerate() {
        match c {
            Content::GridCell(e) if e.x.is_some() || e.y.is_some() => {
                explicit.push((idx, c));
            }
            _ => auto.push((idx, c)),
        }
    }

    // Pass 1 — explicit placement.
    for (_idx, cell) in &explicit {
        let (_body, x, y, cs, rs) = extract_cell_fields(cell);
        let colspan = cs.unwrap_or(1).max(1);
        let rowspan = rs.unwrap_or(1).max(1);

        let col_start = x.unwrap_or(0);
        // P822 — coluna explícita fora da grid é erro dedicado (paridade
        // vanilla `resolve.rs:2221-2225`, sem hint), verificada ANTES da
        // validação de colspan — o vanilla distingue "invalid column" de
        // "colspan exceed" (o cristalino confundia os dois, achado #9(b)
        // de P810).
        if x.is_some() && col_start >= num_cols {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                format!("cell could not be placed at invalid column {col_start}"),
            )]);
        }

        // Validar colspan cabe (paridade vanilla `resolve.rs:1413-1419`
        // — mensagem + hint verbatim, P822).
        if col_start + colspan > num_cols {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cell's colspan would cause it to exceed the available column(s)"
                    .to_string(),
            )
            .with_hint(
                "try placing the cell in another position or reducing its colspan",
            )]);
        }

        // Row: explicit se y Some; auto se y None (próxima linha livre nessa coluna).
        let row_start = match y {
            Some(r) => r,
            None => find_next_free_row(&occupied, col_start, colspan),
        };

        // Expandir occupancy para rowspan.
        ensure_rows(&mut occupied, row_start + rowspan, num_cols);

        // Detectar conflito. P822 — mensagens em paridade verbatim com o
        // vanilla: conflito na posição de origem da célula → "attempted to
        // place a second cell" (resolve.rs:1500-1504); conflito numa
        // posição spanned → "cell would span a previously placed cell"
        // (resolve.rs:1522-1530). Hints incluídos (observável ao nível da
        // língua — ADR-0107).
        for r in row_start..row_start + rowspan {
            for c in col_start..col_start + colspan {
                if occupied[r][c] {
                    let diag = if r == row_start && c == col_start {
                        SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "attempted to place a second cell at column {c}, row {r}"
                            ),
                        )
                        .with_hint("try specifying your cells in a different order")
                    } else {
                        SourceDiagnostic::error(
                            Span::detached(),
                            format!(
                                "cell would span a previously placed cell at column {c}, row {r}"
                            ),
                        )
                        .with_hint(
                            "try specifying your cells in a different order or reducing the cell's rowspan or colspan",
                        )
                    };
                    return Err(vec![diag]);
                }
            }
        }

        // Marcar ocupação.
        for r in row_start..row_start + rowspan {
            for c in col_start..col_start + colspan {
                occupied[r][c] = true;
            }
        }

        // P234 — preserva outer cell (GridCell wrapper inteiro) em
        // PlacedCell.body para consumer extrair per-cell stroke/fill
        // via match preservando precedência P230.
        placed.push(PlacedCell {
            body: (*cell).clone(),
            row: row_start,
            col: col_start,
            colspan,
            rowspan,
        });
    }

    // Pass 2 — auto placement linear.
    let mut cursor_row: usize = 0;
    let mut cursor_col: usize = 0;
    for (_idx, cell) in &auto {
        let (_body, _, _, cs, rs) = extract_cell_fields(cell);
        let colspan = cs.unwrap_or(1).max(1);
        let rowspan = rs.unwrap_or(1).max(1);

        // Validar colspan cabe (paridade vanilla `resolve.rs:1413-1419`
        // — mensagem + hint verbatim, P822).
        if colspan > num_cols {
            return Err(vec![SourceDiagnostic::error(
                Span::detached(),
                "cell's colspan would cause it to exceed the available column(s)"
                    .to_string(),
            )
            .with_hint(
                "try placing the cell in another position or reducing its colspan",
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

        // P234 — preserva outer cell (GridCell wrapper inteiro).
        placed.push(PlacedCell {
            body: (*cell).clone(),
            row: cursor_row,
            col: cursor_col,
            colspan,
            rowspan,
        });

        cursor_col += colspan;
    }

    Ok(placed)
}

/// Extrai fields de `Content::GridCell` ou trata `Content` raw como
/// (body=self, x=None, y=None, colspan=None, rowspan=None).
/// P230 — stroke/fill GridCell ignorados aqui (consumidos pelo
/// `layout_grid` consumer via `extract_cell_cosmetics`; placement
/// algorítmico ortogonal a render cosmético).
fn extract_cell_fields(
    c: &Content,
) -> (&Content, Option<usize>, Option<usize>, Option<usize>, Option<usize>) {
    match c {
        Content::GridCell(e) => (&e.body, e.x, e.y, e.colspan, e.rowspan),
        other => (other, None, None, None, None),
    }
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
