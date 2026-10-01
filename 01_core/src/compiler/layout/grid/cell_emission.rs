//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/cell_emission.md
//! @prompt-hash 09570062
//! @layer L1
//! @updated 2026-09-30
//!
//! Emissão de células de grid, paginação e segmentos de borda por linha.

use super::*;

impl<'a, M: FontMetrics, S: ImageSizer> super::super::Layouter<'a, M, S> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn emit_grid_cells(
        &mut self,
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
            for r in p.row..(p.row + p.rowspan).min(num_rows_produced_final) {
                for c in p.col..(p.col + p.colspan).min(num_cols) {
                    grid_owner[r][c] = Some(i);
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
                        &mut open_vsegments,
                        &col_starts,
                        &resolved_widths,
                        num_cols,
                        &mut self.regions.current.current_items,
                    );
                    self.new_page();
                }
            }

            let row_start_y = self.regions.current.cursor_y.0;
            emitted_row_starts[row_idx] = row_start_y;

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
                let (cell_stroke, cell_fill, cell_align, cell_inset, _cell_breakable) =
                    match cell {
                        // Modelo D (Lote 12 P327): arm `|`-combinado GridCell+TableCell
                        // dividido — TableCell já é `Arc<…Elem>`, GridCell ainda struct
                        // (migra a seguir). Bindings com tipos distintos não partilham
                        // or-pattern; split permanente.
                        Content::TableCell(e) => (
                            e.stroke.as_ref(),
                            e.fill.as_ref(),
                            e.align.as_ref().copied(),
                            e.inset.as_ref(),
                            e.breakable.as_ref().copied(),
                        ),
                        Content::GridCell(e) => (
                            e.stroke.as_ref(),
                            e.fill.as_ref(),
                            e.align.as_ref().copied(),
                            e.inset.as_ref(),
                            e.breakable.as_ref().copied(),
                        ),
                        _ => (None, None, None, None, None), // neutro: N16[β] — Content não-cell retorna tupla de Nones para stroke/fill/align/inset/breakable
                    };

                // Precedência `.or()` uniforme P230 + P232 + P235.
                let effective_stroke: Option<&Stroke> = cell_stroke.or(stroke);
                let effective_fill: Option<&Color> = cell_fill.or(fill);
                // P772j — fold por eixo (não `.or()` do Align2D inteiro):
                // confirmado contra o vanilla (`show_cell`/`resolve_cell` em
                // rules.rs/resolve.rs) e por medição directa
                // (`grid(align: horizon, grid.cell(align: left)[..])` — o
                // vanilla preserva o V herdado do grid quando a célula só
                // especifica H). `.or()` do Align2D inteiro descartava o
                // eixo do grid sempre que a célula especificava qualquer
                // eixo — divergência real, não só mecânica. Ver
                // 00_nucleo/prompts/compiler/layout.md §"Alinhamento efectivo
                // per-célula".
                let effective_align = match (cell_align, self.cell_align) {
                    (None, None) => None,
                    (Some(c), None) => Some(c),
                    (None, Some(g)) => Some(g),
                    (Some(c), Some(g)) => {
                        Some(Align2D { h: c.h.or(g.h), v: c.v.or(g.v) })
                    }
                };
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
                let saved_cell_region = self
                    .regions
                    .enter_cell(crate::entities::region::Region::new(body_w, body_h));
                self.cell_origin_x = Some(body_x);
                self.cell_origin_y = Some(body_y);

                // P273.9 — save/restore parent_bbox paralelo a cell_origin_*
                // (Decisão 2α: bbox exacto cell body). Defaults rigorosos:
                // popular apenas se body_w/h > 0. Pattern DEBT-37 reused N=4.
                let saved_parent_bbox_p273_9 = self.parent_bbox;
                if body_w > 0.0 && body_h > 0.0 {
                    self.parent_bbox = Some(crate::entities::layout_types::Rect {
                        x: Pt(body_x),
                        y: Pt(body_y),
                        w: Pt(body_w),
                        h: Pt(body_h),
                    });
                }

                // P772j — alinhamento efectivo aplicado envolvendo o corpo
                // em `Content::Align` (nunca `Content::Place` — mecanismo
                // confirmado contra o vanilla, `show_cell` em
                // typst-layout/src/engine.rs, que faz `body.aligned(align)`
                // quando `align` é `Smart::Custom`, nunca `body.placed(..)`).
                // Reutiliza a disciplina de "consumidor absoluto" de P772g:
                // `layout_align`, invocado a partir daqui, lê
                // `self.regions.current.line_start_x` (=body_x, já definido
                // pela chamada de `layout_sub_frame` abaixo) e
                // `self.regions.cell` (Some, já definido por `enter_cell`
                // acima) — nenhuma alteração adicional a `placement.rs`.
                let cell_to_layout: Content = match effective_align {
                    Some(alignment) => Content::Align(std::sync::Arc::new(
                        crate::entities::elements::align::AlignElem {
                            alignment,
                            body: cell.clone(),
                        },
                    )),
                    None => cell.clone(),
                };

                // P234 — sem cache; re-medir cell (custo perf ~2× aceitável).
                // P235 — layout em body_x/body_w reduzidos por inset.
                let saved_cursor_x = self.regions.current.cursor_x;
                let saved_cursor_y = self.regions.current.cursor_y;
                let (
                    cell_h_measured,
                    cell_items,
                    cell_deco,
                    orphaned_align_x,
                    orphaned_align_y,
                ) = self.layout_sub_frame(
                    &cell_to_layout,
                    super::super::sub_frame::SubLayoutRegion {
                        origin_x: body_x,
                        width: body_w,
                        height: None,
                        align_rtl: true,
                        unconstrained_height: true,
                    },
                );
                self.regions.current.cursor_x = saved_cursor_x;
                self.regions.current.cursor_y = saved_cursor_y;
                // **P772x** — traduzir segmentos de decoração da célula com a
                // MESMA translação aplicada a `cell_items` abaixo (x já
                // absoluto — `origin_x: body_x` — só y é rebaseado) e
                // reinserir no collector ambiente, se activo.
                if let Some(coll) = self.decoration_lines_collector.as_mut() {
                    for seg in &cell_deco {
                        coll.push(super::super::DecoSegment {
                            start_x: seg.start_x,
                            end_x: seg.end_x,
                            baseline_y: Pt(body_y + seg.baseline_y.val()),
                        });
                    }
                }

                // P273.9 — restore parent_bbox (LIFO).
                self.parent_bbox = saved_parent_bbox_p273_9;

                // P246 — sair célula; restaurar legacy fields.
                self.regions.exit_cell(saved_cell_region);
                self.cell_origin_x = saved_cell_ox;
                self.cell_origin_y = saved_cell_oy;
                self.cell_align = saved_cell_align_inner;

                // P248 — TableCell overflow clip implícito: se cell body
                // ultrapassa o limite da célula (`body_h` populado via
                // `regions.cell.height` P246), emite items dentro de
                // `FrameItem::Group` com `clip_mask: Rect` (paridade
                // mecanismo P242). Row break real diferido per
                // Decisão 3 (refino futuro; DEBT-34e preservado aberto).
                let cell_overflow = cell_h_measured > body_h + 1e-3;

                // P228 + P230 + P234 — Z-order step 1: fill efectivo
                // emite primeiro (atrás do conteúdo cell + stroke).
                // Bounds reais cell_w/cell_h (cobrem colspan/rowspan).
                if let Some(c) = effective_fill {
                    self.regions.current.current_items.push(FrameItem::Shape {
                        pos: Point { x: Pt(cell_x), y: Pt(cell_y) },
                        kind: ShapeKind::Rect,
                        width: cell_w,
                        height: cell_h,
                        fill: Some(crate::entities::paint::Paint::Solid(c.clone())),
                        stroke: None,
                        fill_rule: Default::default(),
                        parent_bbox_at_emit: None,
                    });
                }

                // Z-order step 2: conteúdo cell (existing P82-84.6 lógica).
                // Transferir items com posições absolutas (Y rebaseado
                // a body_y reduzido por inset P235, compensando o ascender_local).
                let (top_edge, _) = self.metrics.text_edges(self.style.size, &self.style);
                let translated_items: Vec<FrameItem> = cell_items
                    .into_iter()
                    .map(|item| {
                        let (lx, ly) = item_pos(&item);
                        let is_text = matches!(
                            &item,
                            FrameItem::Text { .. } | FrameItem::TextShaped { .. }
                        );
                        let abs_pos = if is_text {
                            Point {
                                x: Pt(lx),
                                y: Pt(body_y + (ly - local_start_y) + top_edge.0),
                            }
                        } else {
                            Point { x: Pt(lx), y: Pt(body_y + (ly - local_start_y)) }
                        };
                        translate_frame_item(item, abs_pos.x, abs_pos.y)
                    })
                    .collect();
                // **P908** — rebasear as entradas órfãs devolvidas pelo
                // `layout_sub_frame` da célula (`Content::Align`/
                // `Content::Place` aninhado sob `width`/`height: auto`)
                // para ficarem relativas a `translated_items` (mesma
                // ordem/contagem de `cell_items` — só a posição de cada
                // item mudou, não o índice). X já vem absoluto
                // (`origin_x: body_x` na chamada acima) — sem deslocamento
                // extra. **Distinção física vs lógica (eixo Y)** — mesma
                // nota de `layout_align`: `origin_y`/`applied_y` são
                // quantidades LÓGICAS (sem ascender embutido), ao
                // contrário dos `FrameItem`s reais (cujo `ly` já é
                // baseline, por isso a translação real usa `body_y + (ly
                // - local_start_y)`); a translação lógica usa `body_y`
                // directamente — NÃO `body_y - local_start_y`. Ver
                // `00_nucleo/prompts/compiler/layout.md` §P908.
                let orphaned_align_y: Vec<_> = orphaned_align_y
                    .into_iter()
                    .map(|(path, count, align, content_h, origin_y, dy, applied_y)| {
                        (
                            path,
                            count,
                            align,
                            content_h,
                            origin_y + body_y,
                            dy,
                            applied_y + body_y,
                        )
                    })
                    .collect();
                if cell_overflow {
                    // P251 — row break TableCell cell-level γ-Items
                    // (slice items por body_h threshold + push tail
                    // ao buffer pending_cell_tails; flush no topo da
                    // próxima página via new_page).
                    //
                    // Preservação P248 clip implícito para rows
                    // TrackSizing::Fixed (paridade vanilla "Fixed
                    // rows clip overflow"); só Auto/Fraction usam
                    // row break.
                    let row_track = &row_tracks[placed.row % row_tracks.len()];
                    let is_fixed_row = matches!(row_track, TrackSizing::Fixed(_));
                    if is_fixed_row {
                        // **P908** — família "envolvimento em `Group`":
                        // `translated_items` (já absolutos) tornam-se
                        // `Group.items` sem alteração adicional — só
                        // `path` desce um nível (índice do `Group` na
                        // lista onde vai ser inserido, capturado ANTES do
                        // `push` abaixo).
                        let group_idx = self.regions.current.current_items.len();
                        for (mut path, count, align, content_w, origin_x, applied_x) in
                            orphaned_align_x
                        {
                            path.insert(0, group_idx);
                            self.pending_align_centering.push((
                                path, count, align, content_w, origin_x, applied_x,
                            ));
                        }
                        for (
                            mut path,
                            count,
                            align,
                            content_h,
                            origin_y,
                            dy,
                            applied_y,
                        ) in orphaned_align_y
                        {
                            path.insert(0, group_idx);
                            self.pending_align_v_centering.push((
                                path, count, align, content_h, origin_y, dy, applied_y,
                            ));
                        }
                        // P248 preservado para Fixed rows.
                        self.regions.current.current_items.push(FrameItem::Group {
                            pos:          Point { x: Pt(body_x), y: Pt(body_y) },
                            matrix:       crate::entities::layout_types::TransformMatrix::identity(),
                            clip_mask:    Some(ShapeKind::Rect),
                            inner_width:  body_w,
                            inner_height: body_h,
                            items:        translated_items,
                        });
                    } else {
                        // P251 — γ-Items slice por threshold = body_y + body_h
                        // (em coordenadas absolutas; items já translated_).
                        let threshold = body_y + body_h;
                        // **P908** — família "diferido-cruzado": calcular a
                        // MESMA decisão head/tail de
                        // `slice_frame_items_at_height` por índice
                        // ORIGINAL, antes de consumir `translated_items`,
                        // para poder remapear os índices das entradas
                        // órfãs para a lista compactada correspondente
                        // (head OU tail — nunca as duas, cada partição tem
                        // a sua própria indexação a partir de 0).
                        let is_tail: Vec<bool> = translated_items
                            .iter()
                            .map(|item| {
                                super::super::slicing::item_y_start(item) >= threshold
                            })
                            .collect();
                        let compact_idx = |orig_idx: usize, want_tail: bool| -> usize {
                            is_tail[..orig_idx]
                                .iter()
                                .filter(|&&t| t == want_tail)
                                .count()
                        };
                        // Entrada cujo intervalo `[start, start+count)`
                        // atravessa a fronteira head/tail é descartada
                        // (degradação graciosa, caso raro — ver doc de
                        // `DeferredCellTail::orphaned_align_x`).
                        let split_orphaned = |entries: Vec<(
                            Vec<usize>,
                            usize,
                            crate::entities::layout_types::Align2D,
                            f64,
                            f64,
                            f64,
                        )>| {
                            let mut to_head = Vec::new();
                            let mut to_tail = Vec::new();
                            for (mut path, count, align, dim, origin, applied) in entries
                            {
                                let start = path[0];
                                let range = start..start + count;
                                if range.end > is_tail.len() {
                                    continue;
                                }
                                let all_tail = is_tail[range.clone()].iter().all(|&t| t);
                                let all_head = is_tail[range].iter().all(|&t| !t);
                                if all_head {
                                    path[0] = compact_idx(start, false);
                                    to_head
                                        .push((path, count, align, dim, origin, applied));
                                } else if all_tail {
                                    path[0] = compact_idx(start, true);
                                    to_tail.push((
                                        path,
                                        count,
                                        align,
                                        dim,
                                        origin - threshold,
                                        applied - threshold,
                                    ));
                                }
                                // misto: descartado.
                            }
                            (to_head, to_tail)
                        };
                        let (head_align_x, tail_align_x) =
                            split_orphaned(orphaned_align_x);
                        // eixo Y tem um campo extra (`dy`) — mesma lógica,
                        // duplicada por causa da forma da tupla.
                        let (head_align_y, tail_align_y): (
                            Vec<PendingAlignYEntry>,
                            Vec<PendingAlignYEntry>,
                        ) = {
                            let mut to_head = Vec::new();
                            let mut to_tail = Vec::new();
                            for (
                                mut path,
                                count,
                                align,
                                content_h,
                                origin_y,
                                dy,
                                applied_y,
                            ) in orphaned_align_y
                            {
                                let start = path[0];
                                let range = start..start + count;
                                if range.end > is_tail.len() {
                                    continue;
                                }
                                let all_tail = is_tail[range.clone()].iter().all(|&t| t);
                                let all_head = is_tail[range].iter().all(|&t| !t);
                                if all_head {
                                    path[0] = compact_idx(start, false);
                                    to_head.push((
                                        path, count, align, content_h, origin_y, dy,
                                        applied_y,
                                    ));
                                } else if all_tail {
                                    path[0] = compact_idx(start, true);
                                    to_tail.push((
                                        path,
                                        count,
                                        align,
                                        content_h,
                                        origin_y - threshold,
                                        dy,
                                        applied_y - threshold,
                                    ));
                                }
                            }
                            (to_head, to_tail)
                        };

                        let (head_items, tail_items) =
                            crate::compiler::layout::slicing::slice_frame_items_at_height(
                                translated_items,
                                threshold,
                            );
                        let insertion_base = self.regions.current.current_items.len();
                        for (mut path, count, align, content_w, origin_x, applied_x) in
                            head_align_x
                        {
                            path[0] += insertion_base;
                            self.pending_align_centering.push((
                                path, count, align, content_w, origin_x, applied_x,
                            ));
                        }
                        for (
                            mut path,
                            count,
                            align,
                            content_h,
                            origin_y,
                            dy,
                            applied_y,
                        ) in head_align_y
                        {
                            path[0] += insertion_base;
                            self.pending_align_v_centering.push((
                                path, count, align, content_h, origin_y, dy, applied_y,
                            ));
                        }
                        for item in head_items {
                            self.regions.current.current_items.push(item);
                        }
                        if !tail_items.is_empty() {
                            self.pending_cell_tails.push(
                                crate::compiler::layout::DeferredCellTail {
                                    items: tail_items,
                                    origin_x: body_x,
                                    width: body_w,
                                    fill: effective_fill.cloned(),
                                    stroke: effective_stroke.cloned(),
                                    forwarded_count: 0,
                                    orphaned_align_x: tail_align_x,
                                    orphaned_align_y: tail_align_y,
                                },
                            );
                        }
                    }
                } else {
                    let insertion_base = self.regions.current.current_items.len();
                    for (mut path, count, align, content_w, origin_x, applied_x) in
                        orphaned_align_x
                    {
                        path[0] += insertion_base;
                        self.pending_align_centering
                            .push((path, count, align, content_w, origin_x, applied_x));
                    }
                    for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                        orphaned_align_y
                    {
                        path[0] += insertion_base;
                        self.pending_align_v_centering.push((
                            path, count, align, content_h, origin_y, dy, applied_y,
                        ));
                    }
                    for item in translated_items {
                        self.regions.current.current_items.push(item);
                    }
                }
            }

            // P888 (achado 3 de P885, secção 7 de P887) — substitui a
            // antiga "Opção β simplificada" (4 bordas por célula, sempre)
            // por fusão de segmentos partilhados: horizontal fundido
            // dentro da linha, vertical fundido entre linhas via
            // `open_vsegments`. Ver `emit_row_borders` (definição acima,
            // fora do `impl`) para a lógica completa e a nota de segurança
            // sobre `stroke` divergente por célula.
            {
                let mut row_cells_sorted: Vec<usize> = cells_per_row[row_idx].clone();
                row_cells_sorted.sort_by_key(|&idx| placed_cells[idx].col);
                emit_row_borders(
                    &mut self.regions.current.current_items,
                    row_idx,
                    row_start_y,
                    row_h,
                    &row_cells_sorted,
                    &placed_cells,
                    &grid_owner,
                    &mut open_vsegments,
                    &col_starts,
                    &resolved_widths,
                    num_cols,
                    stroke,
                );
            }

            // Avançar cursor para o fim da linha (altura conhecida).
            self.regions.current.cursor_y = Pt(row_start_y + row_h);
        }

        // P888 — descarregar quaisquer segmentos verticais ainda abertos
        // no fim da última linha (nenhuma quebra de página os fechou).
        flush_all_vsegments(
            &mut open_vsegments,
            &col_starts,
            &resolved_widths,
            num_cols,
            &mut self.regions.current.current_items,
        );

        emitted_row_starts
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn modulo_compila_e_carrega() {}
}
