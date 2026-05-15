//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/layout.md
//! @prompt-hash 089621fc
//! @layer L1
//! @updated 2026-04-23
//!
//! Braço `Content::Grid` do `layout_content`. Extraído de `layout/mod.rs`
//! no Passo 96.7 conforme ADR-0037.

use crate::entities::{
    content::Content,
    geometry::{ShapeKind, Stroke},
    image_sizer::ImageSizer,
    layout_types::{Align2D, Color, FrameItem, Length, Point, Pt, TrackSizing},
    sides::Sides,
};

use super::grid_placement::{place_cells, PlacedCell};
use super::metrics::FontMetrics;
use super::{item_pos, translate_frame_item};

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    /// Layout de `Content::Grid` — algoritmo de tracks (Passo 80, 83, 84.2,
    /// 84.6). Extraído no Passo 96.7. **P224 refino** — signature expandida
    /// com 5 fields (`gutter`/`align`/`inset`/`header`/`footer`); semantic
    /// real adiada para gutter/align/inset (paridade ADR-0054 graded;
    /// algoritmo placement via `grid_placement::place_cells` em sub-fase
    /// futura quando integração for substantiva). `_align`/`_inset`
    /// armazenados mas ignorados nesta versão — `gutter` aplicado
    /// horizontalmente como soma a col_starts (graded).
    pub(super) fn layout_grid(
        &mut self,
        columns: &[TrackSizing],
        rows:    &[TrackSizing],
        cells:   &[Content],
        _gutter: Option<Length>,
        align:   Option<Align2D>,  // P232 — Grid-level align disponível para Place herdar
        inset:   Sides<Length>,    // P235 — Grid-level inset (default per-cell)
        _header: Option<&Content>,
        _footer: Option<&Content>,
        stroke:  Option<&Stroke>,  // P227 — borders cell render Opção β
        fill:    Option<&Color>,    // P228 — fill cell render Z-order correcto
    ) {
        // P232 — save/restore cell_align Grid-level para Place
        // herdar via `.or()` per eixo no arm Content::Place. Paridade
        // pattern cell_origin_* P84.6 mas com scope Grid-level (não
        // per-cell — align uniforme aplica-se a todas as cells do Grid).
        let saved_cell_align = self.cell_align;
        self.cell_align = align;
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
                    // P233 — DEBT-34d fix: capar `safe` quando há fr
                    // tracks presentes para Auto NÃO consumir todo o
                    // remaining (deixando 0pt para fr). Sem fr presente,
                    // comportamento baseline preservado (P80).
                    //
                    // Estratégia subset minimal: dividir `safe_total`
                    // proporcionalmente entre auto + fr (split igualitário
                    // simples). Auto que precisar mais que `safe_capped`
                    // será truncado; fr recebe pelo menos `safe_total /
                    // (num_auto + num_fr) * num_fr_remaining`.
                    //
                    // Two-pass measure→place inaugurado P233 (pattern N=1).
                    // Resolução completa min-content/max-content
                    // negotiation continua DEBT-34d-rest se necessária
                    // (atomização ADR-0036).
                    let has_fr = cols.iter().any(|t| matches!(t, TrackSizing::Fraction(_)));
                    let safe = if has_fr {
                        let num_auto_cols = cols.iter().filter(|t| matches!(t, TrackSizing::Auto)).count();
                        let num_fr_cols   = cols.iter().filter(|t| matches!(t, TrackSizing::Fraction(_))).count();
                        let safe_total = (available_width - total_fixed_w).max(0.0);
                        let total_tracks_concorrentes = (num_auto_cols + num_fr_cols).max(1) as f64;
                        safe_total / total_tracks_concorrentes
                    } else {
                        (available_width - total_fixed_w).max(0.0)
                    };
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
        // P234 (B.2 consumer geometric): cache `cell_cache` removido
        // porque emissão pós-P234 itera `placed_cells` (não
        // `rows_of_items` direct) — placed cells reordenam input
        // (explicit Pass 1 + auto Pass 2). Cells re-medidas durante
        // emissão (custo perf ~2× aceitável MVP; cache reintegrável
        // futuro candidato indexado por input_idx).
        let mut row_heights: Vec<f64> = vec![0.0; num_rows_produced];
        let mut total_fixed_and_auto: f64 = 0.0;
        let mut fraction_indices: Vec<(usize, f64)> = Vec::new();

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
                        let (sub_h, _sub_items) =
                            self.layout_sub_frame_with_width(item, cell_x, cell_w);
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
        let space_left = f64::max(0.0, self.page_bottom_limit() - self.regions.current.cursor_y.0);
        if total_fixed_and_auto > space_left {
            let page_usable_height = self.regions.current.height - 2.0 * self.page_config.margin;
            if total_fixed_and_auto <= page_usable_height {
                self.new_page();
            }
        }

        // Fase 2 — resolver Fraction com cursor_y estabilizado.
        if !fraction_indices.is_empty() {
            let grid_top_y = self.regions.current.cursor_y.0;
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

        // ── P234 — Placement algorítmico via place_cells (B.2). ───
        // `layout_grid` consumer geometric integration: itera
        // `Vec<PlacedCell>` em vez de `rows_of_items` chunks direct.
        // Cells com `colspan: None / rowspan: None` resolved como
        // `colspan: 1, rowspan: 1` em `PlacedCell` → comportamento
        // sequencial preservado paridade pré-P234. Cells com
        // colspan/rowspan > 1 ocupam bounds reais.
        //
        // Error path: place_cells retorna Err em conflict explicit/
        // explicit ou colspan excede num_cols; MVP fallback vec vazio
        // (sem render — comportamento "no-op" para grid inválido).
        // Sink reporting candidato futuro.
        let placed_cells: Vec<PlacedCell> = place_cells(cells, num_cols).unwrap_or_default();

        // Derive num_rows_produced_final do placed (pode estender
        // além de rows_of_items.len() para cells explicit com y > N).
        let num_rows_from_placed = placed_cells.iter()
            .map(|p| p.row + p.rowspan)
            .max()
            .unwrap_or(0);
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

        // ── Emissão linha a linha (P234 — placed cells iteration) ──
        let local_start_y = {
            let (ascender, _) = self.metrics.vertical_metrics(self.font_size_pt);
            ascender.0
        };

        for row_idx in 0..num_rows_produced_final {
            let row_h = row_heights[row_idx];

            // Quebra durante a emissão se a linha individual não cabe
            // (caso conservador: começar a linha no topo da página seguinte).
            // Se a linha for maior que uma página inteira, aceitar overflow.
            // P234 nota: cells com rowspan > 1 cruzando pagination =
            // out-of-scope (Categoria C.2 multi-region span futura).
            if self.regions.current.cursor_y.0 + row_h > self.page_bottom_limit() {
                let page_usable_height = self.regions.current.height - 2.0 * self.page_config.margin;
                if row_h <= page_usable_height {
                    self.new_page();
                }
            }

            let row_start_y = self.regions.current.cursor_y.0;

            for &placed_idx in &cells_per_row[row_idx] {
                let placed = &placed_cells[placed_idx];
                let cell = &placed.body;

                // P234 — bounds reais usando placed.col/colspan ×
                // resolved_widths + placed.row/rowspan × row_heights.
                let (cell_x, cell_y, cell_w, cell_h) = cell_bounds(
                    placed,
                    &col_starts,
                    &resolved_widths,
                    &row_heights,
                    row_start_y,
                );

                // P230 + P235 — extrair per-cell 5 fields (stroke + fill
                // cosméticos P230; align + inset + breakable algorítmicos
                // P235). Match em `placed.body` preserva GridCell wrapper
                // P234.
                let (cell_stroke, cell_fill, cell_align, cell_inset, _cell_breakable) = match cell {
                    Content::GridCell { stroke, fill, align, inset, breakable, .. } |
                    Content::TableCell { stroke, fill, align, inset, breakable, .. } => (
                        stroke.as_ref(),
                        fill.as_ref(),
                        align.as_ref().copied(),
                        inset.as_ref(),
                        breakable.as_ref().copied(),
                    ),
                    _ => (None, None, None, None, None),
                };

                // Precedência `.or()` uniforme P230 + P232 + P235.
                let effective_stroke: Option<&Stroke> = cell_stroke.or(stroke);
                let effective_fill:   Option<&Color>  = cell_fill.or(fill);
                // P235 — align per-cell override Grid-level (self.cell_align
                // ainda contém Grid.align porque save/restore Grid-level
                // P232 cobre todo o emit loop).
                let effective_align = cell_align.or(self.cell_align);
                // P235 — inset per-cell override Grid-level; default Grid inset.
                let effective_inset: Sides<Length> = cell_inset.cloned().unwrap_or(inset);
                // P235 — breakable per-cell semantic adiada graded
                // (pattern "Field armazenado semantic adiada" N=7 → 8).

                // P235 — Cell-level cell_align save/restore (extensão P232
                // per-cell granularidade). Pattern emergente N=1 inaugurado.
                let saved_cell_align_inner = self.cell_align;
                self.cell_align = effective_align;

                // P235 — Inset bounds reduction: layout body em área
                // reduzida (left/top/right/bottom). Clamp a 0 para evitar
                // bounds negativos.
                let inset_l = effective_inset.left.abs.to_pt();
                let inset_t = effective_inset.top.abs.to_pt();
                let inset_r = effective_inset.right.abs.to_pt();
                let inset_b = effective_inset.bottom.abs.to_pt();
                let body_x = cell_x + inset_l;
                let body_y = cell_y + inset_t;
                let body_w = (cell_w - inset_l - inset_r).max(0.0);
                let body_h = (cell_h - inset_t - inset_b).max(0.0);

                // Definir o contexto de altura/origem da célula para
                // Content::Align (Passo 83) e Content::Place (Passo 84.6).
                // P235 — set ao body bounds reduzidos por inset.
                // P246 — cell_available_h + cell_origin_w migrados a
                // regions.cell (entity-side); cell_origin_x/y preservados
                // como Layouter fields legacy.
                let saved_cell_ox = self.cell_origin_x;
                let saved_cell_oy = self.cell_origin_y;
                let saved_cell_region = self.regions.enter_cell(
                    crate::entities::region::Region::new(body_w, body_h),
                );
                self.cell_origin_x = Some(body_x);
                self.cell_origin_y = Some(body_y);

                // P234 — sem cache; re-medir cell (custo perf ~2× aceitável).
                // P235 — layout em body_x/body_w reduzidos por inset.
                let saved_cursor_x = self.regions.current.cursor_x;
                let saved_cursor_y = self.regions.current.cursor_y;
                let (cell_h_measured, cell_items) =
                    self.layout_sub_frame_with_width(cell, body_x, body_w);
                self.regions.current.cursor_x = saved_cursor_x;
                self.regions.current.cursor_y = saved_cursor_y;

                // P246 — sair célula; restaurar legacy fields.
                self.regions.exit_cell(saved_cell_region);
                self.cell_origin_x = saved_cell_ox;
                self.cell_origin_y = saved_cell_oy;
                self.cell_align    = saved_cell_align_inner;

                // P248 — TableCell overflow clip implícito: se cell body
                // ultrapassa o limite da célula (`body_h` populado via
                // `regions.cell.height` P246), emite items dentro de
                // `FrameItem::Group` com `clip_mask: Rect` (paridade
                // mecanismo P242). Row break real diferido per
                // Decisão 3 (refino futuro; DEBT-34e preservado aberto).
                let cell_overflow = cell_h_measured > body_h;

                // P228 + P230 + P234 — Z-order step 1: fill efectivo
                // emite primeiro (atrás do conteúdo cell + stroke).
                // Bounds reais cell_w/cell_h (cobrem colspan/rowspan).
                if let Some(c) = effective_fill {
                    self.regions.current.current_items.push(FrameItem::Shape {
                        pos:    Point { x: Pt(cell_x), y: Pt(cell_y) },
                        kind:   ShapeKind::Rect,
                        width:  cell_w,
                        height: cell_h,
                        fill:   Some(*c),
                        stroke: None,
                    });
                }

                // Z-order step 2: conteúdo cell (existing P82-84.6 lógica).
                // Transferir items com posições absolutas (Y rebaseado
                // a body_y reduzido por inset P235, compensando o ascender_local).
                let translated_items: Vec<FrameItem> = cell_items
                    .into_iter()
                    .map(|item| {
                        let (lx, ly) = item_pos(&item);
                        let abs_pos = Point {
                            x: Pt(lx),
                            y: Pt(body_y + (ly - local_start_y)),
                        };
                        translate_frame_item(item, abs_pos.x, abs_pos.y)
                    })
                    .collect();
                if cell_overflow {
                    // P248 — wrap em Group com clip_mask Rect bounds
                    // body_w × body_h (paridade vanilla clip implícito).
                    self.regions.current.current_items.push(FrameItem::Group {
                        pos:          Point { x: Pt(body_x), y: Pt(body_y) },
                        matrix:       crate::entities::layout_types::TransformMatrix::identity(),
                        clip_mask:    Some(ShapeKind::Rect),
                        inner_width:  body_w,
                        inner_height: body_h,
                        items:        translated_items,
                    });
                } else {
                    for item in translated_items {
                        self.regions.current.current_items.push(item);
                    }
                }

                // P227 + P230 + P234 — Renderização Opção β simplificada:
                // emite 4 FrameItem::Shape::Line per cell border (top +
                // bottom + left + right). Stroke efectivo. Bounds reais
                // cobrem colspan/rowspan.
                if let Some(s) = effective_stroke {
                    let stroke_clone = s.clone();
                    // Top edge.
                    self.regions.current.current_items.push(FrameItem::Shape {
                        pos:    Point { x: Pt(cell_x), y: Pt(cell_y) },
                        kind:   ShapeKind::Line { dx: cell_w, dy: 0.0 },
                        width:  0.0,
                        height: 0.0,
                        fill:   None,
                        stroke: Some(stroke_clone.clone()),
                    });
                    // Bottom edge.
                    self.regions.current.current_items.push(FrameItem::Shape {
                        pos:    Point { x: Pt(cell_x), y: Pt(cell_y + cell_h) },
                        kind:   ShapeKind::Line { dx: cell_w, dy: 0.0 },
                        width:  0.0,
                        height: 0.0,
                        fill:   None,
                        stroke: Some(stroke_clone.clone()),
                    });
                    // Left edge.
                    self.regions.current.current_items.push(FrameItem::Shape {
                        pos:    Point { x: Pt(cell_x), y: Pt(cell_y) },
                        kind:   ShapeKind::Line { dx: 0.0, dy: cell_h },
                        width:  0.0,
                        height: 0.0,
                        fill:   None,
                        stroke: Some(stroke_clone.clone()),
                    });
                    // Right edge.
                    self.regions.current.current_items.push(FrameItem::Shape {
                        pos:    Point { x: Pt(cell_x + cell_w), y: Pt(cell_y) },
                        kind:   ShapeKind::Line { dx: 0.0, dy: cell_h },
                        width:  0.0,
                        height: 0.0,
                        fill:   None,
                        stroke: Some(stroke_clone),
                    });
                }
            }

            // Avançar cursor para o fim da linha (altura conhecida).
            self.regions.current.cursor_y = Pt(row_start_y + row_h);
        }

        // P232 — restore cell_align ao sair de Grid context (paridade
        // cell_origin_* save/restore pattern P84.6).
        self.cell_align = saved_cell_align;
    }
}

/// P234 — Bounds reais per `PlacedCell` × tracks resolved.
///
/// `current_row_start_y` é o cursor_y no início da emissão da row
/// `placed.row` (já considerou pagination). Para cells com
/// `rowspan > 1`, height = sum(row_heights[row..row+rowspan])
/// estende para baixo a partir de current_row_start_y.
fn cell_bounds(
    placed: &PlacedCell,
    col_starts: &[f64],
    resolved_widths: &[f64],
    row_heights: &[f64],
    current_row_start_y: f64,
) -> (f64, f64, f64, f64) {
    let x0 = col_starts.get(placed.col).copied().unwrap_or(0.0);
    let y0 = current_row_start_y;
    let cell_w: f64 = (placed.col..placed.col + placed.colspan)
        .map(|i| resolved_widths.get(i).copied().unwrap_or(0.0))
        .sum();
    let cell_h: f64 = (placed.row..placed.row + placed.rowspan)
        .map(|i| row_heights.get(i).copied().unwrap_or(0.0))
        .sum();
    (x0, y0, cell_w, cell_h)
}

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — submódulo extraído no Passo 96.7 (ADR-0037).
        // A cobertura funcional vive em `layout/tests.rs`.
    }
}
