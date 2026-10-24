//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/grid/row_pagination.md
//! @prompt-hash 1ed7eb5c
//! @layer L1
//! @updated 2026-10-24
//!
//! Paginação conservadora de uma linha de grid.

use super::*;

impl<'a, M: FontMetrics, S: ImageSizer> super::super::Layouter<'a, M, S> {
    pub(super) fn grid_row_breaks(&self, cursor_y: f64, row_h: f64) -> bool {
        cursor_y + row_h > self.page_bottom_limit()
            && row_h <= self.regions.current.height - self.page_config.margin.vertical()
    }

    #[allow(clippy::too_many_arguments)]
    pub(super) fn paginate_grid_row(
        &mut self,
        row_h: f64,
        open_vsegments: &mut [Option<(f64, f64, Stroke)>],
        col_starts: &[f64],
        resolved_widths: &[f64],
        num_cols: usize,
    ) {
        // Quebra durante a emissão se a linha individual não cabe
        // (caso conservador: começar a linha no topo da página seguinte).
        // Se a linha for maior que uma página inteira, aceitar overflow.
        // P234 nota: cells com rowspan > 1 cruzando pagination =
        // out-of-scope (Categoria C.2 multi-region span futura).
        if self.grid_row_breaks(self.regions.current.cursor_y.0, row_h) {
            flush_all_vsegments(
                open_vsegments,
                col_starts,
                resolved_widths,
                num_cols,
                self,
            );
            self.new_page();
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
