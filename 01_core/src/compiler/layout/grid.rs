//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid.md
//! @prompt-hash 726ac543
//! @layer L1
//! @updated 2026-07-24
//!
//! Braço `Content::Grid` do `layout_content`. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037.

mod borders;
mod cell_body;
mod cell_emission;
mod row_groups;
mod row_pagination;
mod rules;
mod tracks;
use self::borders::*;

use super::PendingAlignYEntry;
use crate::entities::elements::grid_hline::GridHLineElem;
use crate::entities::elements::grid_vline::GridVLineElem;
use crate::entities::elements::table_hline::TableHLineElem;
use crate::entities::elements::table_vline::TableVLineElem;
use crate::entities::{
    content::Content,
    elements::grid::GridElem,
    geometry::{ShapeKind, Stroke},
    image_sizer::ImageSizer,
    layout_types::{Align2D, Color, FrameItem, Length, Point, Pt, TrackSizing},
    sides::Sides,
};

use super::grid_placement::{place_cells, PlacedCell};
use super::metrics::FontMetrics;
use super::{item_pos, translate_frame_item};

/// Trait privado para desenhar hlines tanto de grid como de table com o
/// mesmo algoritmo (`layout_grid`).
pub(super) trait LayoutHLine {
    fn start(&self) -> usize;
    fn end(&self) -> Option<usize>;
    fn row(&self) -> usize;
    /// **P739A** — `None` = `stroke: none` (linha não desenhada; paridade
    /// vanilla, medido).
    fn stroke(&self) -> Option<&Stroke>;
    fn position(&self) -> &str;
}

impl LayoutHLine for GridHLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn row(&self) -> usize {
        self.row
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

impl LayoutHLine for TableHLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn row(&self) -> usize {
        self.row
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

/// Trait privado para desenhar vlines tanto de grid como de table com o
/// mesmo algoritmo (`layout_grid`).
pub(super) trait LayoutVLine {
    fn start(&self) -> usize;
    fn end(&self) -> Option<usize>;
    fn col(&self) -> usize;
    fn stroke(&self) -> Option<&Stroke>;
    fn position(&self) -> &str;
}

impl LayoutVLine for GridVLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn col(&self) -> usize {
        self.col
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

impl LayoutVLine for TableVLineElem {
    fn start(&self) -> usize {
        self.start
    }
    fn end(&self) -> Option<usize> {
        self.end
    }
    fn col(&self) -> usize {
        self.col
    }
    fn stroke(&self) -> Option<&Stroke> {
        self.stroke.as_ref()
    }
    fn position(&self) -> &str {
        self.position.as_str()
    }
}

/// Layout de `grid(...)` (atomização ADR-0109 P380): delega ao motor
/// `layout_grid` (cluster Grid+Table). Content-preserving — era inline no
/// `layout_content`.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut super::Layouter<M, S>,
    e: &GridElem,
) {
    layouter.layout_grid(
        &e.columns,
        &e.rows,
        &e.cells,
        &e.hlines,
        &e.vlines,
        e.gutter,
        e.align,
        e.inset,
        e.header.as_ref(),
        e.footer.as_ref(),
        e.stroke.as_ref(),
        e.fill.as_ref(),
    );
}

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Grid` — algoritmo de tracks (Passo 80, 83, 84.2,
    /// 84.6). Extraído no Passo 96.7. **P224 refino** — signature expandida
    /// com 5 fields (`gutter`/`align`/`inset`/`header`/`footer`); semantic
    /// real adiada para gutter/align/inset (paridade ADR-0054 graded;
    /// algoritmo placement via `grid_placement::place_cells` em sub-fase
    /// futura quando integração for substantiva). `_align`/`_inset`
    /// armazenados mas ignorados nesta versão — `gutter` aplicado
    /// horizontalmente como soma a col_starts (graded).
    pub(super) fn layout_grid<H: LayoutHLine, V: LayoutVLine>(
        &mut self,
        columns: &[TrackSizing],
        rows: &[TrackSizing],
        cells: &[Content],
        hlines: &[H],
        vlines: &[V],
        _gutter: Option<Length>,
        align: Option<Align2D>, // P232 — Grid-level align disponível para Place herdar
        inset: Sides<Length>,   // P235 — Grid-level inset (default per-cell)
        header: Option<&Content>,
        footer: Option<&Content>,
        stroke: Option<&Stroke>, // P227 — borders cell render Opção β
        fill: Option<&Color>,    // P228 — fill cell render Z-order correcto
    ) {
        // P232 — save/restore cell_align Grid-level para Place
        // herdar via `.or()` per eixo no arm Content::Place. Paridade
        // pattern cell_origin_* P84.6 mas com scope Grid-level (não
        // per-cell — align uniforme aplica-se a todas as cells do Grid).
        let saved_cell_align = self.cell_align;
        self.cell_align = align;
        let available_width = self.available_width();

        // Guarda Passo 83 — colunas vazias caem em [Auto].
        let cols: Vec<TrackSizing> =
            if columns.is_empty() { vec![TrackSizing::Auto] } else { columns.to_vec() };
        let num_cols = cols.len();

        let combined_cells =
            match row_groups::combine_cells(cells, header, footer, num_cols) {
                Ok(cells) => cells,
                Err(row) => {
                    self.layout_errors.push(
                        crate::entities::source_result::SourceDiagnostic::error(
                            crate::entities::span::Span::detached(),
                            format!(
                                "cell would conflict with header also spanning row {row}"
                            ),
                        )
                        .with_hint("try moving the cell or the header"),
                    );
                    self.cell_align = saved_cell_align;
                    return;
                }
            };
        let cells: &[Content] = &combined_cells;

        // Guarda Passo 83 — rows vazias caem em [Auto] para evitar
        // panic por divisão por zero em N % rows.len() quando o AST
        // é construído manualmente (testes que ignoram a stdlib).
        let row_tracks: Vec<TrackSizing> =
            if rows.is_empty() { vec![TrackSizing::Auto] } else { rows.to_vec() };

        let (resolved_widths, col_starts) =
            self.resolve_grid_columns(&cols, cells, inset, available_width);
        let (mut row_heights, num_rows_produced) = self.resolve_grid_rows(
            &row_tracks,
            cells,
            num_cols,
            &resolved_widths,
            &col_starts,
            inset,
        );

        // ── P234 — Placement algorítmico via place_cells (B.2). ───
        // `layout_grid` consumer geometric integration: itera
        // `Vec<PlacedCell>` em vez de `rows_of_items` chunks direct.
        // Cells com `colspan: None / rowspan: None` resolved como
        // `colspan: 1, rowspan: 1` em `PlacedCell` → comportamento
        // sequencial preservado paridade pré-P234. Cells com
        // colspan/rowspan > 1 ocupam bounds reais.
        //
        // Error path: place_cells retorna Err em conflito explicit/
        // explicit, colspan a exceder as colunas disponíveis ou coluna
        // inválida (mensagens vanilla, P822). P647: propagar como erro
        // de layout em vez de renderizar grid vazia em silêncio.
        let placed_cells: Vec<PlacedCell> = match place_cells(cells, num_cols) {
            Ok(placed) => placed,
            Err(diagnostics) => {
                self.layout_errors.extend(diagnostics);
                self.cell_align = saved_cell_align;
                return;
            }
        };

        // Derive num_rows_produced_final do placed (pode estender
        // além de rows_of_items.len() para cells explicit com y > N).
        let num_rows_from_placed =
            placed_cells.iter().map(|p| p.row + p.rowspan).max().unwrap_or(0);
        let num_rows_produced_final = num_rows_from_placed.max(num_rows_produced).max(1);

        // Pad row_heights se placed estende além de chunks-derived
        // (cells explicit com y maior ou cells com rowspan estendem
        // para rows não-covered por chunks). Extra rows: Fixed
        // resolved literal de row_tracks; Auto/Fraction = 0pt
        // (sem cells a medir → refino futuro candidato).
        while row_heights.len() < num_rows_produced_final {
            let row_idx = row_heights.len();
            let track = &row_tracks[row_idx % row_tracks.len()];
            let h = match track {
                TrackSizing::Fixed(pt) => *pt,
                _ => 0.0,
            };
            row_heights.push(h);
        }

        // Group placed cells por start row para per-row pagination
        // preservada (cells starting in row r emitidas conjunto
        // após pagination check de row r).
        let mut cells_per_row: Vec<Vec<usize>> = vec![vec![]; num_rows_produced_final];
        for (i, p) in placed_cells.iter().enumerate() {
            if p.row < num_rows_produced_final {
                cells_per_row[p.row].push(i);
            }
        }

        let emitted_row_starts = self.emit_grid_cells(
            &placed_cells,
            &cells_per_row,
            num_rows_produced_final,
            num_cols,
            &row_heights,
            &col_starts,
            &resolved_widths,
            &row_tracks,
            inset,
            stroke,
            fill,
        );

        rules::emit_explicit_rules(
            hlines,
            vlines,
            num_rows_produced_final,
            num_cols,
            &emitted_row_starts,
            &row_heights,
            &col_starts,
            &resolved_widths,
            &mut self.regions.current.current_items,
        );

        // P232 — restore cell_align ao sair de Grid context (paridade
        // cell_origin_* save/restore pattern P84.6).
        self.cell_align = saved_cell_align;
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }

    use super::*;
    use crate::entities::layout_types::Length;

    #[test]
    fn p1043_grid_stroke_top_coalesce_isolada() {
        // C1=T, C2=T: rs == s && contiguous -> top_run funde as células
        let stroke = Stroke {
            paint: Color::luma(0.0).into(),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        };
        let cells = vec![
            PlacedCell {
                body: Content::Empty,
                col: 0,
                row: 0,
                colspan: 1,
                rowspan: 1,
            },
            PlacedCell {
                body: Content::Empty,
                col: 1,
                row: 0,
                colspan: 1,
                rowspan: 1,
            },
        ];
        let mut items = Vec::new();
        let mut open_v = vec![None; 3];
        let grid_owner = vec![vec![Some(0), Some(1)]];
        emit_row_borders(
            &mut items,
            0,
            0.0,
            20.0,
            &[0, 1],
            &cells,
            &grid_owner,
            &mut open_v,
            &[0.0, 50.0],
            &[50.0, 50.0],
            2,
            Some(&stroke),
        );
        let h_lines: Vec<_> = items
            .iter()
            .filter(|i| {
                matches!(i, FrameItem::Shape { kind: ShapeKind::Line { .. }, .. })
            })
            .collect();
        assert!(h_lines.len() >= 2);
    }

    #[test]
    fn p1043_grid_stroke_top_non_contiguous_isolada() {
        // C1=T, C2=F: rs == s && !contiguous (gap) -> flushes run e inicia novo
        let stroke = Stroke {
            paint: Color::luma(0.0).into(),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        };
        let cells = vec![
            PlacedCell {
                body: Content::Empty,
                col: 0,
                row: 0,
                colspan: 1,
                rowspan: 1,
            },
            PlacedCell {
                body: Content::Empty,
                col: 1,
                row: 0,
                colspan: 1,
                rowspan: 1,
            },
        ];
        let mut items = Vec::new();
        let mut open_v = vec![None; 3];
        let grid_owner = vec![vec![Some(0), Some(1)]];
        // Gap: col 0 ends at 50, col 1 starts at 60
        emit_row_borders(
            &mut items,
            0,
            0.0,
            20.0,
            &[0, 1],
            &cells,
            &grid_owner,
            &mut open_v,
            &[0.0, 60.0],
            &[50.0, 50.0],
            2,
            Some(&stroke),
        );
        let h_lines: Vec<_> = items
            .iter()
            .filter(|i| {
                matches!(i, FrameItem::Shape { kind: ShapeKind::Line { .. }, .. })
            })
            .collect();
        assert!(h_lines.len() >= 4);
    }

    #[test]
    fn p1043_grid_stroke_top_diff_stroke_isolada() {
        // C1=F, C2=_: rs != s -> flushes run e inicia novo
        let stroke1 = Stroke {
            paint: Color::luma(0.0).into(),
            thickness: 1.0,
            overhang: false,
            ..Stroke::default()
        };
        let stroke2 = Stroke {
            paint: Color::luma(0.0).into(),
            thickness: 2.0,
            overhang: false,
            ..Stroke::default()
        };
        use crate::entities::elements::grid_cell::GridCellElem;
        use std::sync::Arc;
        let cell1 = Content::GridCell(Arc::new(GridCellElem {
            body: Content::Empty,
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: Some(stroke1),
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        }));
        let cell2 = Content::GridCell(Arc::new(GridCellElem {
            body: Content::Empty,
            x: None,
            y: None,
            colspan: None,
            rowspan: None,
            stroke: Some(stroke2),
            fill: None,
            align: None,
            inset: None,
            breakable: None,
        }));
        let cells = vec![
            PlacedCell {
                body: cell1,
                col: 0,
                row: 0,
                colspan: 1,
                rowspan: 1,
            },
            PlacedCell {
                body: cell2,
                col: 1,
                row: 0,
                colspan: 1,
                rowspan: 1,
            },
        ];
        let mut items = Vec::new();
        let mut open_v = vec![None; 3];
        let grid_owner = vec![vec![Some(0), Some(1)]];
        emit_row_borders(
            &mut items,
            0,
            0.0,
            20.0,
            &[0, 1],
            &cells,
            &grid_owner,
            &mut open_v,
            &[0.0, 50.0],
            &[50.0, 50.0],
            2,
            None,
        );
        let h_lines: Vec<_> = items
            .iter()
            .filter(|i| {
                matches!(i, FrameItem::Shape { kind: ShapeKind::Line { .. }, .. })
            })
            .collect();
        assert!(h_lines.len() >= 4);
    }
}
