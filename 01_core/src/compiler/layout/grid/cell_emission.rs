//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/grid/cell_emission.md
//! @prompt-hash c72dd9f1
//! @layer L1
//! @updated 2026-10-24
//!
//! Emissão de células de grid, paginação e segmentos de borda por linha.

use super::*;

pub(super) struct GridEmissionInput<'b> {
    pub(super) paint_borders: bool,
    pub(super) header_groups: &'b [crate::entities::elements::table::TableHeaderRange],
    pub(super) measured_cells: &'b mut measurement_cache::MeasuredCells,
    pub(super) placed_cells: &'b [PlacedCell],
    pub(super) cells_per_row: &'b [Vec<usize>],
    pub(super) num_rows_produced_final: usize,
    pub(super) num_cols: usize,
    pub(super) row_heights: &'b [f64],
    pub(super) col_starts: &'b [f64],
    pub(super) resolved_widths: &'b [f64],
    pub(super) row_tracks: &'b [TrackSizing],
    pub(super) inset: Sides<Length>,
    pub(super) table_alignment:
        Option<&'b crate::entities::elements::table::TableAlignment>,
    pub(super) stroke: Option<&'b Stroke>,
    pub(super) fill: Option<&'b Color>,
    pub(super) routes: Option<&'b [Option<super::super::page_running_requests::Cursor>]>,
}

impl<'a, M: FontMetrics, S: ImageSizer> super::super::Layouter<'a, M, S> {
    pub(super) fn emit_grid_cells_planned(
        &mut self,
        input: GridEmissionInput<'_>,
    ) -> Vec<f64> {
        let GridEmissionInput {
            paint_borders,
            header_groups,
            measured_cells,
            placed_cells,
            cells_per_row,
            num_rows_produced_final,
            num_cols,
            row_heights,
            col_starts,
            resolved_widths,
            row_tracks,
            inset,
            stroke,
            fill,
            routes,
            table_alignment,
        } = input;
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

        let uniform_stroke = if paint_borders {
            uniform_table_stroke(
                placed_cells,
                &grid_owner,
                col_starts,
                resolved_widths,
                row_heights,
                stroke,
                self.style.size.0,
            )
        } else {
            None
        };
        let mut border_boundaries = Vec::new();

        if header_groups.iter().enumerate().any(|(i, g)| {
            g.level == 0
                || g.start >= g.end
                || g.end > num_rows_produced_final
                || (i > 0 && header_groups[i - 1].end > g.start)
        }) {
            self.layout_errors.push(
                crate::entities::source_result::SourceDiagnostic::error(
                    self.current_source_span,
                    "invalid prepared table header ranges",
                ),
            );
            return emitted_row_starts;
        }
        let mut active: Vec<usize> = Vec::new();
        let mut capture: Option<(usize, usize)> = None;
        for row_idx in 0..num_rows_produced_final {
            let row_h = row_heights[row_idx];

            if self.grid_row_breaks(self.regions.current.cursor_y.0, row_h) {
                if let Some(stroke) = &uniform_stroke {
                    emit_uniform_table_borders(
                        self,
                        &mut border_boundaries,
                        col_starts,
                        resolved_widths,
                        stroke,
                    );
                }
            }

            let starting = header_groups.iter().position(|g| g.start == row_idx);
            if let Some(index) = starting {
                active
                    .retain(|old| header_groups[*old].level < header_groups[index].level);
                flush_all_vsegments(
                    &mut open_vsegments,
                    col_starts,
                    resolved_widths,
                    num_cols,
                    self,
                );
            }
            let header_height: f64 = starting.map_or(row_h, |i| {
                row_heights[header_groups[i].start..header_groups[i].end].iter().sum()
            });
            let required = starting.map_or(row_h, |i| {
                header_height
                    + row_heights.get(header_groups[i].end).copied().unwrap_or(0.0)
            });
            let capacity =
                self.regions.current.height - self.page_config.margin.vertical();
            if starting.is_some()
                && (!header_height.is_finite() || header_height > capacity)
            {
                self.layout_errors.push(
                    crate::entities::source_result::SourceDiagnostic::error(
                        self.current_source_span,
                        "table header group does not fit the available region",
                    ),
                );
                return emitted_row_starts;
            }
            if !header_groups.is_empty()
                && self.grid_row_breaks(self.regions.current.cursor_y.0, required)
            {
                flush_all_vsegments(
                    &mut open_vsegments,
                    col_starts,
                    resolved_widths,
                    num_cols,
                    self,
                );
                if let Some(stroke) = &uniform_stroke {
                    emit_uniform_table_borders(
                        self,
                        &mut border_boundaries,
                        col_starts,
                        resolved_widths,
                        stroke,
                    );
                }
                self.new_page();
                let repeat_height: f64 = active
                    .iter()
                    .map(|i| {
                        let group = &header_groups[*i];
                        row_heights[group.start..group.end].iter().sum::<f64>()
                    })
                    .sum();
                if self.regions.current.cursor_y.0 + repeat_height + required
                    > self.page_bottom_limit() + 1e-8
                {
                    self.layout_errors.push(
                        crate::entities::source_result::SourceDiagnostic::error(
                            self.current_source_span,
                            "repeating table headers leave no room for the next row",
                        ),
                    );
                    return emitted_row_starts;
                }
                for index in &active {
                    let group = &header_groups[*index];
                    let source_indices: Vec<_> = placed_cells
                        .iter()
                        .enumerate()
                        .filter(|(_, p)| group.start <= p.row && p.row < group.end)
                        .map(|(i, _)| i)
                        .collect();
                    let repeated: Vec<_> = source_indices
                        .iter()
                        .map(|i| {
                            let mut cell = placed_cells[*i].clone();
                            cell.row -= group.start;
                            cell
                        })
                        .collect();
                    let count = group.end - group.start;
                    let repeated_tracks: Vec<_> = (group.start..group.end)
                        .map(|row| row_tracks[row % row_tracks.len()].clone())
                        .collect();
                    let mut by_row = vec![Vec::new(); count];
                    for (i, cell) in repeated.iter().enumerate() {
                        by_row[cell.row].push(i);
                    }
                    let repeated_routes = routes.map(|routes| {
                        source_indices
                            .iter()
                            .map(|i| routes[*i].clone())
                            .collect::<Vec<_>>()
                    });
                    let starts = self.emit_grid_cells_planned(GridEmissionInput {
                        paint_borders: uniform_stroke.is_none(),
                        header_groups: &[],
                        measured_cells: &mut *measured_cells,
                        placed_cells: &repeated,
                        cells_per_row: &by_row,
                        num_rows_produced_final: count,
                        num_cols,
                        row_heights: &row_heights[group.start..group.end],
                        col_starts,
                        resolved_widths,
                        row_tracks: &repeated_tracks,
                        inset,
                        table_alignment,
                        stroke,
                        fill,
                        routes: repeated_routes.as_deref(),
                    });
                    if uniform_stroke.is_some() {
                        for (offset, start) in starts.iter().enumerate() {
                            if border_boundaries.is_empty() {
                                border_boundaries.push(*start);
                            }
                            border_boundaries
                                .push(start + row_heights[group.start + offset]);
                        }
                    }
                }
            } else {
                self.paginate_grid_row(
                    row_h,
                    &mut open_vsegments,
                    col_starts,
                    resolved_widths,
                    num_cols,
                );
            }
            if let Some(index) = starting {
                capture = Some((index, self.pages.len()));
            }

            let row_start_y = self.regions.current.cursor_y.0;
            emitted_row_starts[row_idx] = row_start_y;

            for &placed_idx in &cells_per_row[row_idx] {
                let placed = &placed_cells[placed_idx];
                let saved_alignment = self.cell_align;
                if let Some(alignment) = table_alignment {
                    self.cell_align = crate::compiler::stdlib::table_alignment_at(
                        alignment, placed.col,
                    );
                }
                if let Some(cursor) = routes
                    .and_then(|routes| routes.get(placed_idx))
                    .and_then(Option::as_ref)
                {
                    self.emit_grid_cell_body_planned(cell_body::CellBodyInput {
                        measured_cells: &mut *measured_cells,
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
                        cursor: Some(cursor.clone()),
                    });
                } else {
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
                self.cell_align = saved_alignment;
            }

            if uniform_stroke.is_some() {
                if border_boundaries.is_empty() {
                    border_boundaries.push(row_start_y);
                }
                border_boundaries.push(row_start_y + row_h);
            } else if paint_borders {
                let next_on_same_page =
                    !header_groups.iter().any(|g| g.end == row_idx + 1)
                        && row_heights.get(row_idx + 1).is_some_and(|height| {
                            let required = header_groups
                                .iter()
                                .find(|g| g.start == row_idx + 1)
                                .map_or(*height, |g| {
                                    row_heights[g.start..g.end].iter().sum::<f64>()
                                        + row_heights.get(g.end).copied().unwrap_or(0.0)
                                });
                            !self.grid_row_breaks(row_start_y + row_h, required)
                        });
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
                    next_on_same_page,
                );
            }

            if capture
                .as_ref()
                .is_some_and(|(index, _)| header_groups[*index].end == row_idx + 1)
            {
                let (index, page) = capture.take().unwrap();
                if page != self.pages.len() {
                    self.layout_errors.push(
                        crate::entities::source_result::SourceDiagnostic::error(
                            self.current_source_span,
                            "table header group unexpectedly fragmented",
                        ),
                    );
                    return emitted_row_starts;
                }
                flush_all_vsegments(
                    &mut open_vsegments,
                    col_starts,
                    resolved_widths,
                    num_cols,
                    self,
                );
                if header_groups[index].repeat {
                    active.push(index);
                }
            }

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

        if let Some(stroke) = &uniform_stroke {
            emit_uniform_table_borders(
                self,
                &mut border_boundaries,
                col_starts,
                resolved_widths,
                stroke,
            );
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

#[cfg(test)]
mod intersection_tests {
    use super::*;
    use crate::compiler::layout::{FixedMetrics, Layouter};
    use crate::entities::{
        geometry::PathItem,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
    };
    use comemo::Track;

    fn contains(kind: &ShapeKind<Pt>, x: f64, y: f64, thickness: f64) -> bool {
        match kind {
            ShapeKind::Line { dx, dy } if *dy == 0.0 => {
                x >= 0.0 && x < *dx && y >= -thickness / 2.0 && y < thickness / 2.0
            }
            ShapeKind::Line { dx, dy } if *dx == 0.0 => {
                y >= 0.0 && y < *dy && x >= -thickness / 2.0 && x < thickness / 2.0
            }
            ShapeKind::Path(path) => {
                let mut winding = 0;
                let mut first = Point::ZERO;
                let mut prev = Point::ZERO;
                for item in path {
                    let next = match item {
                        PathItem::MoveTo(p) => {
                            first = *p;
                            prev = *p;
                            continue;
                        }
                        PathItem::LineTo(p) => *p,
                        PathItem::ClosePath => first,
                        PathItem::CubicTo(..) => panic!("unexpected curved table border"),
                    };
                    let cross = (next.x.0 - prev.x.0) * (y - prev.y.0)
                        - (x - prev.x.0) * (next.y.0 - prev.y.0);
                    if prev.y.0 <= y && next.y.0 > y && cross > 0.0 {
                        winding += 1;
                    }
                    if prev.y.0 > y && next.y.0 <= y && cross < 0.0 {
                        winding -= 1;
                    }
                    prev = next;
                }
                winding != 0
            }
            _ => panic!("unexpected table border geometry"),
        }
    }

    // Rasterize each operation separately, then composite its fractional
    // coverage. This observes repeated antialiasing without prescribing paths.
    fn coverage(items: &[FrameItem], x: f64, y: f64) -> f64 {
        let mut result = 0.0;
        for item in items {
            if let FrameItem::Shape { pos, kind, stroke, .. } = item {
                let thickness = stroke.as_ref().map_or(0.0, |s| s.thickness);
                let mut hits = 0;
                for ix in 0..32 {
                    for iy in 0..32 {
                        if contains(
                            kind,
                            x + (ix as f64 + 0.5) / 32.0 - 0.5 - pos.x.0,
                            y + (iy as f64 + 0.5) / 32.0 - 0.5 - pos.y.0,
                            thickness,
                        ) {
                            hits += 1;
                        }
                    }
                }
                let alpha = hits as f64 / 1024.0;
                result = alpha + result * (1.0 - alpha);
            }
        }
        result
    }

    #[test]
    fn issue_8777_outer_edge_does_not_darken_at_intersections() {
        for thickness in [0.5, 1.0, 3.0] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            use crate::entities::elements::{
                table::TableGridResolved,
                table_cell::{TableCellResolved, TableCellStage},
            };
            use std::sync::Arc;
            let stroke = Stroke {
                thickness: crate::entities::layout_types::Length::pt(thickness),
                ..Stroke::default()
            };
            let cells: Vec<_> = (0..6)
                .map(|i| {
                    let body = Content::text("cell");
                    let mut cell =
                        Content::table_cell(body.clone(), None, None, None, None);
                    let Content::TableCell(c) = &mut cell else { unreachable!() };
                    Arc::make_mut(c).resolved = Some(Arc::new(TableCellResolved {
                        x: i % 2,
                        y: i / 2,
                        colspan: 1,
                        rowspan: 1,
                        fill: None,
                        stroke: Some(stroke.clone()),
                        align: None,
                        inset: Sides::uniform(Length::pt(5.0)),
                        breakable: false,
                        visual: body,
                        stage: TableCellStage::Complete,
                        guards: vec![],
                    }));
                    cell
                })
                .collect();
            let mut table = Content::table(
                vec![TrackSizing::Fixed(30.0); 2],
                vec![TrackSizing::Fixed(20.0); 3],
                cells.clone(),
            );
            let Content::Table(t) = &mut table else { unreachable!() };
            let t = Arc::make_mut(t);
            t.stroke = Some(stroke);
            t.resolved = Some(Arc::new(TableGridResolved {
                header_groups: Vec::new(),
                cells,
                num_cols: 2,
                num_rows: 3,
                header_rows: 0,
                footer_start: 3,
                complete: true,
            }));
            l.layout_content(&table);
            let items = &l.regions.current.current_items;
            let shapes: Vec<_> = items
                .iter()
                .filter_map(|i| match i {
                    FrameItem::Shape { pos, .. } => Some(*pos),
                    _ => None,
                })
                .collect();
            let x = shapes.iter().map(|p| p.x.0).fold(f64::INFINITY, f64::min);
            let y = shapes.iter().map(|p| p.y.0).fold(f64::INFINITY, f64::min)
                + thickness / 2.0;
            // Left ink edge: same fractional coverage at the T and between rows.
            let straight = coverage(items, x, y + 10.0);
            let junction = coverage(items, x, y + 20.0);
            assert!(
                (junction - straight).abs() < 0.001,
                "stroke {thickness}: edge {straight}, intersection {junction}"
            );
            assert!((straight - 0.5).abs() < 0.001, "outer edge must stay put");
        }
    }
    #[test]
    fn issue_8954_header_keeps_horizontal_borders_and_coverage() {
        for thickness in [0.5, 1.0, 3.0] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            use crate::entities::elements::{
                table::TableGridResolved,
                table_cell::{TableCellResolved, TableCellStage},
            };
            use std::sync::Arc;
            let stroke = Stroke {
                thickness: crate::entities::layout_types::Length::pt(thickness),
                ..Stroke::default()
            };
            let cells: Vec<_> = (0..6)
                .map(|i| {
                    let body = Content::text("cell");
                    let mut cell =
                        Content::table_cell(body.clone(), None, None, None, None);
                    let Content::TableCell(c) = &mut cell else { unreachable!() };
                    Arc::make_mut(c).resolved = Some(Arc::new(TableCellResolved {
                        x: i % 2,
                        y: i / 2,
                        colspan: 1,
                        rowspan: 1,
                        fill: None,
                        stroke: Some(stroke.clone()),
                        align: None,
                        inset: Sides::uniform(Length::pt(5.0)),
                        breakable: false,
                        visual: body,
                        stage: TableCellStage::Complete,
                        guards: vec![],
                    }));
                    cell
                })
                .collect();
            let mut table = Content::table(
                vec![TrackSizing::Fixed(30.0); 2],
                vec![TrackSizing::Fixed(20.0); 3],
                cells.clone(),
            );
            let Content::Table(t) = &mut table else { unreachable!() };
            let t = Arc::make_mut(t);
            t.stroke = Some(stroke);
            t.resolved = Some(Arc::new(TableGridResolved {
                header_groups: vec![crate::entities::elements::table::TableHeaderRange {
                    start: 0,
                    end: 1,
                    level: 1,
                    repeat: true,
                }],
                cells,
                num_cols: 2,
                num_rows: 3,
                header_rows: 0,
                footer_start: 3,
                complete: true,
            }));
            l.layout_content(&table);
            let items = &l.regions.current.current_items;
            let shapes: Vec<_> = items
                .iter()
                .filter_map(|i| match i {
                    FrameItem::Shape { pos, .. } => Some(*pos),
                    _ => None,
                })
                .collect();
            let x = shapes.iter().map(|p| p.x.0).fold(f64::INFINITY, f64::min);
            let y = shapes.iter().map(|p| p.y.0).fold(f64::INFINITY, f64::min)
                + thickness / 2.0;
            // Left ink edge: same fractional coverage at the T and between rows.
            let straight = coverage(items, x, y + 10.0);
            let junction = coverage(items, x, y + 20.0);
            assert!(
                (junction - straight).abs() < 0.001,
                "stroke {thickness}: edge {straight}, intersection {junction}"
            );
            assert!((straight - 0.5).abs() < 0.001, "outer edge must stay put");
            assert!(
                coverage(items, x + 15.0, y + 60.0) > 0.4,
                "the bottom horizontal border must remain visible"
            );
        }
    }
}
