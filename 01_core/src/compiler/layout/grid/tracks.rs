//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/tracks.md
//! @prompt-hash 4c6b2127
//! @layer L1
//! @updated 2026-10-07

use super::*;

impl<'a, M: FontMetrics, S: ImageSizer> super::super::Layouter<'a, M, S> {
    /// Resolve larguras e origens das colunas antes de medir linhas.
    pub(super) fn resolve_grid_columns(
        &mut self,
        cols: &[TrackSizing],
        cells: &[Content],
        inset: Sides<Length>,
        available_width: f64,
    ) -> (Vec<f64>, Vec<f64>) {
        let num_cols = cols.len();
        // ── Resolução de larguras (Passo 80, inalterado) ──────
        let mut cols_cells: Vec<Vec<usize>> = vec![vec![]; num_cols];
        for (idx, cell) in cells.iter().enumerate() {
            let col = match cell {
                Content::TableCell(e) if e.resolved.is_some() => {
                    e.resolved.as_ref().unwrap().x
                }
                _ => idx % num_cols,
            };
            cols_cells[col].push(idx);
        }

        let mut resolved_widths = vec![0.0_f64; num_cols];
        let mut total_fixed_w = 0.0_f64;
        let mut total_fr_w = 0.0_f64;

        for (i, sizing) in cols.iter().enumerate() {
            match sizing {
                TrackSizing::Fixed(w) => {
                    resolved_widths[i] = *w;
                    total_fixed_w += *w;
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
                    let has_fr =
                        cols.iter().any(|t| matches!(t, TrackSizing::Fraction(_)));
                    let safe = if has_fr {
                        let num_auto_cols = cols
                            .iter()
                            .filter(|t| matches!(t, TrackSizing::Auto))
                            .count();
                        let num_fr_cols = cols
                            .iter()
                            .filter(|t| matches!(t, TrackSizing::Fraction(_)))
                            .count();
                        let safe_total = (available_width - total_fixed_w).max(0.0);
                        let total_tracks_concorrentes =
                            (num_auto_cols + num_fr_cols).max(1) as f64;
                        safe_total / total_tracks_concorrentes
                    } else {
                        (available_width - total_fixed_w).max(0.0)
                    };
                    let mut max_w = 0.0_f64;
                    for &ci in &cols_cells[i] {
                        let cell_inset = match &cells[ci] {
                            Content::TableCell(e) => {
                                if e.resolved.is_some() {
                                    cell_body::prepared_default(&cells[ci])
                                        .map(|r| &r.inset)
                                } else {
                                    e.inset.as_ref()
                                }
                            }
                            Content::GridCell(e) => e.inset.as_ref(),
                            _ => None,
                        };
                        let eff_inset = cell_inset.cloned().unwrap_or_else(||if matches!(&cells[ci],Content::TableCell(e) if e.resolved.is_some()){Sides::uniform(Length::pt(0.0))}else{inset});
                        let inset_l = eff_inset.left.abs.to_pt();
                        let inset_r = eff_inset.right.abs.to_pt();
                        let visual = match &cells[ci] {
                            Content::TableCell(e) if e.resolved.is_some() => {
                                measurement_visual(&e.resolved.as_ref().unwrap().visual)
                            }
                            c => c.clone(),
                        };
                        let mut nested_table = false;
                        let _ = visual.map_content_pre(&mut |c| {
                            if matches!(c, Content::Table(_) | Content::Raw(_)) {
                                nested_table = true;
                                return Ok(Some(c.clone()));
                            }
                            Ok(None)
                        });
                        let (w, _) = if nested_table || composed_visual(&visual) {
                            let product = self.layout_sub_frame(
                                &visual,
                                super::super::sub_frame::SubLayoutRegion {
                                    origin_x: 0.0,
                                    width: safe,
                                    height: None,
                                    align_rtl: true,
                                    unconstrained_height: true,
                                },
                            );
                            let refs = product.1.iter().collect::<Vec<_>>();
                            (
                                self.metrics
                                    .line_content_right(&refs)
                                    .max(self.last_sub_frame_width),
                                product.0,
                            )
                        } else {
                            self.measure_content_constrained(&visual, safe)
                        };
                        max_w = max_w.max(w + inset_l + inset_r);
                    }
                    if max_w.is_finite()
                        && max_w > 0.0
                        && cells.iter().all(
                            |c| matches!(c,Content::TableCell(e) if e.resolved.is_some()),
                        )
                    {
                        max_w += f64::EPSILON
                            * (available_width.abs()
                                + self.regions.current.line_start_x.0.abs()
                                + max_w.abs());
                    }
                    resolved_widths[i] = max_w;
                    total_fixed_w += max_w;
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
            let offset = if (num_cols == 1 || self.explicit_line_alignment.is_some())
                && available_width.is_finite()
            {
                use crate::entities::{dir::Dir, layout_types::HAlign, value::Value};
                let rtl = self.chain.custom("text.dir") == Some(&Value::Dir(Dir::RTL));
                let free =
                    (available_width - resolved_widths.iter().sum::<f64>()).max(0.0);
                match self.explicit_line_alignment.unwrap_or(HAlign::Start) {
                    HAlign::Left => 0.0,
                    HAlign::Right => free,
                    HAlign::Center => free / 2.0,
                    HAlign::Start => {
                        if rtl {
                            free
                        } else {
                            0.0
                        }
                    }
                    HAlign::End => {
                        if rtl {
                            0.0
                        } else {
                            free
                        }
                    }
                }
            } else {
                0.0
            };
            let mut x = self.regions.current.line_start_x.0 + offset;
            for i in 0..num_cols {
                col_starts[i] = x;
                x += resolved_widths[i];
            }
        }

        (resolved_widths, col_starts)
    }

    /// Mede linhas e fixa a página antes de distribuir frações verticais.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn resolve_grid_rows(
        &mut self,
        row_tracks: &[TrackSizing],
        cells: &[Content],
        num_cols: usize,
        resolved_widths: &[f64],
        col_starts: &[f64],
        inset: Sides<Length>,
    ) -> (Vec<f64>, usize, measurement_cache::MeasuredCells) {
        // ── Particionar items em linhas ──────────────────────
        let num_rows_produced = cells
            .iter()
            .enumerate()
            .map(|(i, c)| match c {
                Content::TableCell(e) if e.resolved.is_some() => {
                    let r = e.resolved.as_ref().unwrap();
                    r.y + r.rowspan
                }
                _ => i / num_cols + 1,
            })
            .max()
            .unwrap_or(0);
        #[derive(Clone, Copy)]
        struct RowCell<'a> {
            cell_index: usize,
            content: &'a Content,
            column: usize,
            colspan: usize,
            rowspan: usize,
        }
        let mut rows_of_items: Vec<Vec<RowCell<'_>>> = vec![vec![]; num_rows_produced];
        for (index, c) in cells.iter().enumerate() {
            let (row, col, cs, rs) = match c {
                Content::TableCell(e) if e.resolved.is_some() => {
                    let r = e.resolved.as_ref().unwrap();
                    (r.y, r.x, r.colspan, r.rowspan)
                }
                _ => (index / num_cols, index % num_cols, 1, 1),
            };
            rows_of_items[row].push(RowCell {
                cell_index: index,
                content: c,
                column: col,
                colspan: cs,
                rowspan: rs,
            });
        }

        // ── Resolução de alturas (Passo 83): 3 passagens ─────
        // Fase 1 — Fixed e Auto numa travessia. Auto mede via
        // layout_sub_frame.
        //
        // Keep eligible measured products in this grid invocation only.
        // The cache owner proves placement, body and environment equivalence.
        let mut measured_cells =
            measurement_cache::MeasuredCells::new(self, cells, num_cols);
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
                    for &RowCell {
                        cell_index,
                        content: item,
                        column: col_idx,
                        colspan,
                        rowspan,
                    } in row_items
                    {
                        if col_idx >= num_cols {
                            break;
                        }
                        let cell_w = resolved_widths[col_idx..col_idx + colspan]
                            .iter()
                            .sum::<f64>();
                        let cell_x = col_starts[col_idx];

                        // P1050 — considerar inset no cálculo da altura de linha da célula
                        let cell_inset = match item {
                            Content::TableCell(e) => {
                                if e.resolved.is_some() {
                                    cell_body::prepared_default(item).map(|r| &r.inset)
                                } else {
                                    e.inset.as_ref()
                                }
                            }
                            Content::GridCell(e) => e.inset.as_ref(),
                            _ => None,
                        };
                        let eff_inset: crate::entities::sides::Sides<
                            crate::entities::layout_types::Length,
                        > = cell_inset.cloned().unwrap_or_else(||if matches!(item,Content::TableCell(e) if e.resolved.is_some()){Sides::uniform(Length::pt(0.0))}else{inset});
                        let inset_l = eff_inset.left.abs.to_pt();
                        let inset_t = eff_inset.top.abs.to_pt();
                        let inset_r = eff_inset.right.abs.to_pt();
                        let inset_b = eff_inset.bottom.abs.to_pt();
                        let body_w = (cell_w - inset_l - inset_r).max(0.0);

                        let product = self.layout_sub_frame(
                            item,
                            super::super::sub_frame::SubLayoutRegion {
                                origin_x: cell_x + inset_l,
                                width: body_w,
                                height: None,
                                align_rtl: true,
                                unconstrained_height: true,
                            },
                        );
                        let cell_eff_h = (product.0 + inset_t + inset_b) / rowspan as f64;
                        measured_cells.capture(
                            self,
                            cell_index,
                            item,
                            cell_x + inset_l,
                            body_w,
                            product,
                        );
                        if cell_eff_h > max_h {
                            max_h = cell_eff_h;
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

        // P750/P761/P762 — quando a baseline inicial ainda está pendente,
        // `cursor_y` representa o topo da área disponível (margem) e
        // `ensure_initial_baseline()` ainda vai adicionar o offset do top-edge.
        // Nesse caso o Grid alinha-se directamente com esse topo. Quando
        // a baseline já foi fixada, `cursor_y` representa a baseline e o
        // topo do Grid é baseline − top-edge.
        if !self.initial_baseline_pending {
            let (top, _) = self.metrics.text_edges(self.style.size, &self.style);
            self.regions.current.cursor_y = Pt(self.regions.current.cursor_y.0 - top.0);
        }

        // Fase 1.5 — paginação ANTES da fase 2 de Fraction.
        // Se Fixed+Auto não cabe no resto da página actual mas cabe
        // numa página vazia, quebrar agora — assim cursor_y fica
        // estabilizado e a fase 2 calcula `fr` com o available_below
        // correcto. Se o Grid é maior que uma página inteira, aceita
        // overflow (não chama new_page() em loop).
        let space_left =
            f64::max(0.0, self.page_bottom_limit() - self.regions.current.cursor_y.0);
        if total_fixed_and_auto > space_left {
            // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
            let page_usable_height =
                self.regions.current.height - self.page_config.margin.vertical();
            if total_fixed_and_auto <= page_usable_height {
                self.new_page();
            }
        }

        // Fase 2 — resolver Fraction com cursor_y estabilizado.
        if !fraction_indices.is_empty() {
            let grid_top_y = self.regions.current.cursor_y.0;
            // **P904 (Item 1)** — sob `height: auto`, `page_bottom_limit()`
            // é `f64::INFINITY` (mesmo sentinel de P867 usado por outros
            // achados desta frente, P895-903); sem esta guarda,
            // `available_below`/`remaining_v` ficavam infinitos e cada
            // linha `fr` recebia `f64::INFINITY` de altura — não só
            // "diverge do vanilla", produzia uma `MediaBox` malformada
            // (`0 0 W inf`), mesma classe do crash original de P894/895.
            // Paridade vanilla confirmada por compilação directa
            // (`lab/typst-original`): `fr` numa página `auto` degenera a 0pt
            // — não há espaço "restante" para distribuir quando a altura é
            // indefinida. `f64::MAX` no lugar de `INFINITY` faz o ramo
            // `total_fixed_and_auto > available_below` abaixo nunca
            // disparar por engano (comparação com infinito real seria
            // sempre falsa para qualquer largura finita, o que já bastava,
            // mas usar um tecto finito explícito é mais robusto a somas
            // subsequentes que poderiam produzir `NaN` a partir de
            // `INFINITY - INFINITY` em casos futuros).
            let page_bottom = self.page_bottom_limit();
            let available_below = if page_bottom.is_finite() {
                f64::max(0.0, page_bottom - grid_top_y)
            } else {
                total_fixed_and_auto
            };
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

        (row_heights, num_rows_produced, measured_cells)
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}

#[cfg(test)]
mod rtl_anchor_regression {
    use super::*;
    use comemo::Track;
    #[test]
    fn one_column_rtl_uses_the_outer_right_anchor() {
        use crate::entities::{
            dir::Dir,
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::TextStyle,
            value::Value,
        };
        let intr = TagIntrospector::empty();
        let mut l = super::super::super::Layouter::new(
            super::super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom("text.dir", Value::Dir(Dir::RTL));
        l.style = TextStyle::from(&l.chain);
        let available = l.available_width();
        let (widths, starts) = l.resolve_grid_columns(
            &[TrackSizing::Fixed(50.0)],
            &[Content::text("ABC")],
            Sides::uniform(Length::ZERO),
            available,
        );
        assert!(
            (starts[0] + widths[0] - (l.page_config.margin.left + available)).abs()
                < 1e-6
        );
    }
}

fn measurement_visual(c: &Content) -> Content {
    c.map_content_pre(&mut |node| {
        Ok(match node {
            Content::Table(_) => Some(node.clone()),
            Content::TableCell(e) if e.stage().is_some() => {
                Some(measurement_visual(e.visual()))
            }
            Content::ShowRecipe(e) => Some(measurement_visual(&e.body)),
            _ => None,
        })
    })
    .expect("pure measurement visual projection")
}

fn composed_visual(c: &Content) -> bool {
    fn zero(c: &Content) -> bool {
        match c.source_body() {
            Content::Empty | Content::Metadata(_) => true,
            Content::Located(e) => zero(&e.body),
            Content::Realized(e) => zero(&e.visual),
            _ => false,
        }
    }
    match c.source_body() {
        Content::Sequence(parts) => parts.iter().filter(|c| !zero(c)).take(2).count() > 1,
        Content::Styled(s) => composed_visual(&s.body),
        Content::Located(s) => composed_visual(&s.body),
        Content::Realized(s) => composed_visual(&s.visual),
        _ => false,
    }
}
