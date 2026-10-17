//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/row_pagination.md
//! @prompt-hash 2aa14c2f
//! @layer L1
//! @updated 2026-09-30
//!
//! Paginação conservadora de uma linha de grid.

use super::*;

impl<'a, M: FontMetrics, S: ImageSizer> super::super::Layouter<'a, M, S> {
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
        if self.regions.current.cursor_y.0 + row_h > self.page_bottom_limit() {
            // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
            let page_usable_height =
                self.regions.current.height - self.page_config.margin.vertical();
            if row_h <= page_usable_height {
                // P888 — uma linha vertical não pode atravessar uma
                // quebra de página; descarregar tudo o que está aberto
                // antes de mudar de página (fica na página que está a
                // fechar), para a linha seguinte reabrir do zero.
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
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
