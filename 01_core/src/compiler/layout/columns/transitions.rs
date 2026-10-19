//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/columns/transitions.md
//! @prompt-hash 1076fade
//! @layer L1
//! @updated 2026-09-30

use super::*;

/// Translada um `FrameItem` horizontalmente por `dx` pontos,
/// preservando a coordenada Y original.
pub(super) fn translate_item_x(item: FrameItem, dx: f64) -> FrameItem {
    use crate::compiler::layout::helpers::{item_pos, translate_frame_item};
    let (x, y) = item_pos(&item);
    translate_frame_item(item, Pt(x + dx), Pt(y))
}

// P1504 — operações das colunas físicas, antes coordenadas pelo Cursor.
impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    /// **P538c** — fecha a coluna actual em fluxo contínuo: flush das
    /// notas de rodapé pendentes, translada os items da coluna para a
    /// posição horizontal absoluta correcta e guarda-os no buffer de
    /// colunas da página.
    pub(in crate::compiler::layout) fn close_current_column(&mut self) {
        // Flush das footnotes no fundo da coluna actual.
        let prev_column_mode = self.column_mode;
        self.column_mode = true;
        self.flush_pending_footnote_bodies(None);
        self.column_mode = prev_column_mode;

        // P1418 — o fluxo contínuo fecha a região física aqui. Patterns são
        // consolidados antes de retirar/transladar os items, com um estilo
        // compartilhado capturado somente na primeira coluna numerável.
        if self.defer_segmented_par_line_patterns && self.page_columns.is_some() {
            let start = self.page_par_line_pattern_column_start;
            if self.page_par_line_pattern_style.is_none() {
                self.page_par_line_pattern_style = Some(
                    self.pending_segmented_par_line_patterns
                        .get(start)
                        .map(|candidate| TextStyle::from(&candidate.styles))
                        .unwrap_or_else(|| self.style.clone()),
                );
            }
            if let Some(number_style) = self.page_par_line_pattern_style.clone() {
                let force_end_margin = self
                    .page_columns
                    .is_some_and(|count| self.current_column + 1 == count);
                let numbers = super::realize_segmented_par_line_patterns(
                    self,
                    start,
                    &number_style,
                    self.regions.current.width,
                    self.regions.current.line_start_x.0,
                    force_end_margin,
                );
                self.regions.current.current_items.extend(numbers);
            }
            self.page_par_line_pattern_column_start =
                self.pending_segmented_par_line_patterns.len();
        }

        // Transladar items da coluna actual de coordenadas locais (origem
        // na margem) para coordenadas absolutas na página.
        let dx = self.column_origin_x - self.page_config.margin.left;
        self.close_link_items();
        let mut items = std::mem::take(&mut self.regions.current.current_items);
        self.apply_pending_par_line_fixups(&mut items);
        let translated = items
            .into_iter()
            .map(|item| {
                let (x, y) = item_pos(&item);
                translate_frame_item(item, Pt(x + dx), Pt(y))
            })
            .collect();
        self.column_page_items.push(translated);
    }

    /// **P538c** — funde os items acumulados de todas as colunas da
    /// página actual no `current_items` da região actual, preparando a
    /// criação de uma nova página física.
    pub(in crate::compiler::layout) fn merge_column_items(&mut self) {
        let mut merged: Vec<FrameItem> = Vec::new();
        for column in std::mem::take(&mut self.column_page_items) {
            merged.extend(column);
        }
        // A coluna actual já foi fechada e está em column_page_items;
        // limpar current_items por segurança e repor merged.
        self.regions.current.current_items = merged;
        self.open_link_items();
    }

    /// **P538c** — fecha a coluna actual, funde as colunas da página e
    /// restaura o estado do Layouter para layout de página normal.
    /// Chamado por `columns::layout` no fim do modo fluxo contínuo.
    pub(in crate::compiler::layout) fn finish_columns(&mut self) {
        if self.page_columns.is_none() {
            return;
        }
        self.close_current_column();
        self.merge_column_items();
        // Restaurar estado de página normal.
        self.page_columns = None;
        self.current_column = 0;
        self.column_page_items.clear();
        self.column_x_offsets.clear();
        self.column_mode = false;
        self.column_origin_x = 0.0;
        self.column_width = self.regions.current.width; // já é a largura da página
        self.regions.current.width = self.page_config.width;
        self.regions.current.cursor_x = Pt(self.page_config.margin.left);
        self.regions.current.line_start_x = Pt(self.page_config.margin.left);
    }

    /// **P538c** — avança o cursor para a coluna seguinte da mesma página.
    pub(in crate::compiler::layout) fn start_next_column(&mut self) {
        self.current_column += 1;
        self.start_column(self.current_column);
    }

    /// **P538c** — configura a região actual para a coluna `idx`.
    pub(in crate::compiler::layout) fn start_column(&mut self, idx: usize) {
        let count = self.page_columns.unwrap_or(1);
        if idx >= count {
            return;
        }
        self.column_origin_x = self.column_x_offsets[idx];
        self.regions.current.width = self.column_width;
        self.regions.current.cursor_x = Pt(self.page_config.margin.left);
        self.regions.current.line_start_x = Pt(self.page_config.margin.left);
        // **P761/P762** — quando a baseline inicial ainda está pendente,
        // `ensure_initial_baseline()` adicionará o offset do `top-edge`.
        self.regions.current.cursor_y = if self.initial_baseline_pending {
            Pt(self.page_config.margin.top)
        } else {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            Pt(self.page_config.margin.top) + top
        };
        self.regions.current.current_line.clear();
        self.open_link_line();
        if self.regions.current.current_items.is_empty() {
            self.open_link_items();
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
