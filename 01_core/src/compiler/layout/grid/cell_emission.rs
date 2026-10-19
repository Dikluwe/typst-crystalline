//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/cell_emission.md
//! @prompt-hash f5827603
//! @layer L1
//! @updated 2026-10-07
//!
//! Emissão de células de grid, paginação e segmentos de borda por linha.

use super::*;

impl<'a, M: FontMetrics, S: ImageSizer> super::super::Layouter<'a, M, S> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn emit_grid_cells(
        &mut self,
        measured_cells: &mut measurement_cache::MeasuredCells,
        placed_cells: &[PlacedCell],
        cells_per_row: &[Vec<usize>],
        num_rows_produced_final: usize,
        num_cols: usize,
        row_heights: &[f64],
        col_starts: &[f64],
        resolved_widths: &[f64],
        row_tracks: &[TrackSizing],
        inset: Sides<Length>,
        stroke: Option<&Stroke>,
        fill: Option<&Color>,
    ) -> Vec<f64> {
        // ── Emissão linha a linha (P234 — placed cells iteration) ──
        let local_start_y = {
            let (ascender, _) =
                self.metrics.vertical_metrics(self.style.size, &self.style);
            ascender.0
        };

        // P512 — guarda o Y real de cada linha emitida (afectado por
        // paginação) para posicionar hlines/vlines depois das células.
        let mut emitted_row_starts = vec![0.0_f64; num_rows_produced_final];

        // P888 (achado 3 de P885, secção 7 de P887) — mapa (linha, coluna)
        // → índice em `placed_cells`, cobrindo colspan/rowspan (uma célula
        // ocupa todas as posições no seu range, não só a de início).
        // Permite resolver, para qualquer fronteira de linha vertical
        // (mesmo em linhas cobertas por rowspan, que não aparecem em
        // `cells_per_row`), qual célula está de cada lado — necessário
        // para a fusão de segmentos de borda em `emit_row_borders`.
        let mut grid_owner: Vec<Vec<Option<usize>>> =
            vec![vec![None; num_cols]; num_rows_produced_final];
        for (i, p) in placed_cells.iter().enumerate() {
            for row in grid_owner
                .iter_mut()
                .take((p.row + p.rowspan).min(num_rows_produced_final))
                .skip(p.row)
            {
                for owner in
                    row.iter_mut().take((p.col + p.colspan).min(num_cols)).skip(p.col)
                {
                    *owner = Some(i);
                }
            }
        }

        // P888 — segmentos verticais "abertos" (a acumular enquanto o
        // stroke resolvido numa fronteira de coluna se mantém igual ao
        // longo de linhas consecutivas). `None` = fronteira sem stroke
        // nesta linha ou ainda por iniciar. Descarregado (emitido) quando
        // o stroke muda, há quebra de página, ou no fim da função —
        // ver `emit_row_borders`/`flush_all_vsegments`.
        let mut open_vsegments: Vec<Option<(f64, f64, Stroke)>> =
            vec![None; num_cols + 1];

        for row_idx in 0..num_rows_produced_final {
            let row_h = row_heights[row_idx];

            self.paginate_grid_row(
                row_h,
                &mut open_vsegments,
                col_starts,
                resolved_widths,
                num_cols,
            );

            let row_start_y = self.regions.current.cursor_y.0;
            emitted_row_starts[row_idx] = row_start_y;

            for &placed_idx in &cells_per_row[row_idx] {
                let placed = &placed_cells[placed_idx];
                self.emit_grid_cell_body(
                    measured_cells,
                    placed,
                    row_start_y,
                    local_start_y,
                    col_starts,
                    resolved_widths,
                    row_heights,
                    row_tracks,
                    inset,
                    stroke,
                    fill,
                );
            }

            emit_placed_row_borders(
                self,
                row_idx,
                row_start_y,
                row_h,
                &cells_per_row[row_idx],
                placed_cells,
                &grid_owner,
                &mut open_vsegments,
                col_starts,
                resolved_widths,
                num_cols,
                stroke,
            );

            // Avançar cursor para o fim da linha (altura conhecida).
            self.regions.current.cursor_y = Pt(row_start_y + row_h);
            self.record_flow_placement_end(row_start_y + row_h);
            let right = col_starts
                .iter()
                .zip(resolved_widths)
                .map(|(start, width)| start + width)
                .fold(self.flow_frame_extent.0, f64::max);
            self.record_flow_width_at(right, 0.0);
        }

        // P888 — descarregar quaisquer segmentos verticais ainda abertos
        // no fim da última linha (nenhuma quebra de página os fechou).
        flush_all_vsegments(
            &mut open_vsegments,
            col_starts,
            resolved_widths,
            num_cols,
            self,
        );

        emitted_row_starts
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
