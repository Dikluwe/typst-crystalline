//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash a78b0adc
//! @layer L1
//! @updated 2026-04-23
//!
//! Braço `Content::Grid` do `layout_content`. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    content::Content,
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Point, Pt, TrackSizing},
};

use super::metrics::FontMetrics;
use super::{item_pos, translate_frame_item};

impl<M: FontMetrics, S: ImageSizer> super::Layouter<M, S> {
    /// Layout de `Content::Grid` — algoritmo de tracks (Passo 80, 83, 84.2,
    /// 84.6). Extraído no Passo 96.7.
    pub(super) fn layout_grid(
        &mut self,
        columns: &[TrackSizing],
        rows:    &[TrackSizing],
        cells:   &[Content],
    ) {
        let available_width = self.available_width();

        // Guarda Passo 83 — colunas vazias caem em [Auto].
        let cols: Vec<TrackSizing> = if columns.is_empty() {
            vec![TrackSizing::Auto]
        } else {
            columns.to_vec()
        };
        let num_cols = cols.len();

        // Guarda Passo 83 — rows vazias caem em [Auto] para evitar
        // panic por divisão por zero em N % rows.len() quando o AST
        // é construído manualmente (testes que ignoram a stdlib).
        let row_tracks: Vec<TrackSizing> = if rows.is_empty() {
            vec![TrackSizing::Auto]
        } else {
            rows.to_vec()
        };

        // ── Resolução de larguras (Passo 80, inalterado) ──────
        let mut cols_cells: Vec<Vec<usize>> = vec![vec![]; num_cols];
        for (idx, _) in cells.iter().enumerate() {
            cols_cells[idx % num_cols].push(idx);
        }

        let mut resolved_widths = vec![0.0_f64; num_cols];
        let mut total_fixed_w   = 0.0_f64;
        let mut total_fr_w      = 0.0_f64;

        for (i, sizing) in cols.iter().enumerate() {
            match sizing {
                TrackSizing::Fixed(w) => {
                    resolved_widths[i] = *w;
                    total_fixed_w     += *w;
                }
                TrackSizing::Auto => {
                    let safe = (available_width - total_fixed_w).max(0.0);
                    let mut max_w = 0.0_f64;
                    for &ci in &cols_cells[i] {
                        let (w, _) = self.measure_content_constrained(&cells[ci], safe);
                        max_w = max_w.max(w);
                    }
                    resolved_widths[i] = max_w;
                    total_fixed_w     += max_w;
                }
                TrackSizing::Fraction(fr) => {
                    total_fr_w += fr;
                }
            }
        }

        let remaining_w = (available_width - total_fixed_w).max(0.0);
        if total_fr_w > 0.0 {
            let per_fr = remaining_w / total_fr_w;
            for (i, sizing) in cols.iter().enumerate() {
                if let TrackSizing::Fraction(fr) = sizing {
                    resolved_widths[i] = fr * per_fr;
                }
            }
        }

        // X de cada coluna.
        let mut col_starts = vec![0.0_f64; num_cols];
        {
            let mut x = self.page_config.margin;
            for i in 0..num_cols {
                col_starts[i] = x;
                x += resolved_widths[i];
            }
        }

        // ── Particionar items em linhas ──────────────────────
        let rows_of_items: Vec<&[Content]> = cells.chunks(num_cols).collect();
        let num_rows_produced = rows_of_items.len();

        // ── Resolução de alturas (Passo 83): 3 passagens ─────
        // Fase 1 — Fixed e Auto numa travessia. Auto mede via
        // layout_sub_frame_with_width.
        //
        // Cache local Passo 84.2 (encerra DEBT-38):
        // a fase Auto guarda (sub_h, sub_items) por célula; a fase de
        // emissão consome (via remove) em vez de relayoutar. Chave:
        // row_idx * num_cols + col_idx. Sai de escopo no fim do braço,
        // sem invalidação manual; cada Grid (incluindo aninhados) tem
        // o seu próprio cache.
        let mut row_heights: Vec<f64> = vec![0.0; num_rows_produced];
        let mut total_fixed_and_auto: f64 = 0.0;
        let mut fraction_indices: Vec<(usize, f64)> = Vec::new();
        let mut cell_cache: std::collections::HashMap<usize, (f64, Vec<FrameItem>)> =
            std::collections::HashMap::new();

        for (row_idx, row_items) in rows_of_items.iter().enumerate() {
            let track = &row_tracks[row_idx % row_tracks.len()];
            match track {
                TrackSizing::Fixed(pt) => {
                    row_heights[row_idx] = *pt;
                    total_fixed_and_auto += *pt;
                }
                TrackSizing::Auto => {
                    let mut max_h = 0.0_f64;
                    for (col_idx, item) in row_items.iter().enumerate() {
                        if col_idx >= num_cols { break; }
                        let cell_w = resolved_widths[col_idx];
                        let cell_x = col_starts[col_idx];
                        let (sub_h, sub_items) =
                            self.layout_sub_frame_with_width(item, cell_x, cell_w);
                        // Passo 84.2 (DEBT-38): guardar para a fase
                        // de emissão. Chave estável dentro do braço.
                        let cell_idx = row_idx * num_cols + col_idx;
                        cell_cache.insert(cell_idx, (sub_h, sub_items));
                        if sub_h > max_h {
                            max_h = sub_h;
                        }
                    }
                    row_heights[row_idx] = max_h;
                    total_fixed_and_auto += max_h;
                }
                TrackSizing::Fraction(fr) => {
                    fraction_indices.push((row_idx, *fr));
                }
            }
        }

        // Garantir linha limpa antes do Grid.
        self.flush_line();

        // Fase 1.5 — paginação ANTES da fase 2 de Fraction.
        // Se Fixed+Auto não cabe no resto da página actual mas cabe
        // numa página vazia, quebrar agora — assim cursor_y fica
        // estabilizado e a fase 2 calcula `fr` com o available_below
        // correcto. Se o Grid é maior que uma página inteira, aceita
        // overflow (não chama new_page() em loop).
        let space_left = f64::max(0.0, self.page_bottom_limit() - self.cursor_y.0);
        if total_fixed_and_auto > space_left {
            let page_usable_height = self.page_config.height - 2.0 * self.page_config.margin;
            if total_fixed_and_auto <= page_usable_height {
                self.new_page();
            }
        }

        // Fase 2 — resolver Fraction com cursor_y estabilizado.
        if !fraction_indices.is_empty() {
            let grid_top_y = self.cursor_y.0;
            let available_below = f64::max(0.0, self.page_bottom_limit() - grid_top_y);
            if total_fixed_and_auto > available_below {
                // Caso patológico residual (Grid > página inteira):
                // não distribuir espaço negativo, atribuir 0pt aos fr.
                for (row_idx, _fr) in &fraction_indices {
                    row_heights[*row_idx] = 0.0;
                }
            } else {
                let remaining_v = available_below - total_fixed_and_auto;
                let total_fr: f64 = fraction_indices.iter().map(|(_, fr)| fr).sum();
                if total_fr > 0.0 {
                    for (row_idx, fr) in &fraction_indices {
                        row_heights[*row_idx] = remaining_v * (fr / total_fr);
                    }
                }
            }
        }

        // ── Emissão de células linha a linha ──────────────────
        let local_start_y = {
            let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
            ascender.0
        };

        for (row_idx, row_items) in rows_of_items.iter().enumerate() {
            let row_h = row_heights[row_idx];

            // Quebra durante a emissão se a linha individual não cabe
            // (caso conservador: começar a linha no topo da página seguinte).
            // Se a linha for maior que uma página inteira, aceitar overflow.
            if self.cursor_y.0 + row_h > self.page_bottom_limit() {
                let page_usable_height = self.page_config.height - 2.0 * self.page_config.margin;
                if row_h <= page_usable_height {
                    self.new_page();
                }
            }

            let row_start_y = self.cursor_y.0;

            for (col_idx, cell) in row_items.iter().enumerate() {
                if col_idx >= num_cols { break; }
                let cell_w = resolved_widths[col_idx];
                let cell_x = col_starts[col_idx];

                // Definir o contexto de altura/origem da célula para
                // Content::Align (Passo 83) e Content::Place (Passo 84.6).
                let saved_cell_h  = self.cell_available_h;
                let saved_cell_ox = self.cell_origin_x;
                let saved_cell_oy = self.cell_origin_y;
                let saved_cell_ow = self.cell_origin_w;
                self.cell_available_h = Some(row_h);
                self.cell_origin_x    = Some(cell_x);
                self.cell_origin_y    = Some(row_start_y);
                self.cell_origin_w    = Some(cell_w);

                // Passo 84.2 (DEBT-38): consumir resultado da fase
                // Auto se já foi medido. `remove` em vez de `get`
                // transfere o Vec sem clonar — cada célula é emitida
                // exactamente uma vez. Cache miss em linhas Fixed/
                // Fraction cai silenciosamente para a chamada original.
                let cell_idx = row_idx * num_cols + col_idx;
                let saved_cursor_x = self.cursor_x;
                let saved_cursor_y = self.cursor_y;
                let (_cell_h, cell_items) = match cell_cache.remove(&cell_idx) {
                    Some(cached) => cached,
                    None => self.layout_sub_frame_with_width(cell, cell_x, cell_w),
                };
                self.cursor_x = saved_cursor_x;
                self.cursor_y = saved_cursor_y;

                self.cell_available_h = saved_cell_h;
                self.cell_origin_x    = saved_cell_ox;
                self.cell_origin_y    = saved_cell_oy;
                self.cell_origin_w    = saved_cell_ow;

                // Transferir items com posições absolutas (Y rebaseado
                // a row_start_y, compensando o ascender_local).
                for item in cell_items {
                    let (lx, ly) = item_pos(&item);
                    let abs_pos = Point {
                        x: Pt(lx),
                        y: Pt(row_start_y + (ly - local_start_y)),
                    };
                    let translated = translate_frame_item(item, abs_pos.x, abs_pos.y);
                    self.current_items.push(translated);
                }
            }

            // Avançar cursor para o fim da linha (altura conhecida).
            self.cursor_y = Pt(row_start_y + row_h);
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }
}
