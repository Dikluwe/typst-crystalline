//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/grid/cell_body.md
//! @prompt-hash 98dd36de
//! @layer L1
//! @updated 2026-10-07
//!
//! Composição e emissão de uma célula de grid.

use super::*;

/// A default Cell is emitted only when its prepared snapshot survives show.
pub(super) fn prepared_default(
    cell: &Content,
) -> Option<&crate::entities::elements::table_cell::TableCellResolved> {
    fn snapshot(
        c: &Content,
    ) -> Option<&crate::entities::elements::table_cell::TableCellResolved> {
        match c.source_body() {
            Content::TableCell(c) => c.resolved.as_deref(),
            Content::Styled(s) => snapshot(&s.body),
            Content::Located(s) => snapshot(&s.body),
            Content::Realized(s) => snapshot(&s.visual),
            Content::Sequence(parts) => {
                fn zero(c: &Content) -> bool {
                    match c.source_body() {
                        Content::Empty | Content::Metadata(_) => true,
                        Content::Located(c) => zero(&c.body),
                        Content::Realized(c) => zero(&c.visual),
                        _ => false,
                    }
                }
                let mut material = parts.iter().filter(|c| !zero(c));
                let r = snapshot(material.next()?)?;
                material.next().is_none().then_some(r)
            }
            _ => None,
        }
    }
    let Content::TableCell(c) = cell else {
        return None;
    };
    snapshot(&c.resolved.as_ref()?.visual)
}

impl<'a, M: FontMetrics, S: ImageSizer> super::super::Layouter<'a, M, S> {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn emit_grid_cell_body(
        &mut self,
        measured_cells: &mut measurement_cache::MeasuredCells,
        placed: &PlacedCell,
        row_start_y: f64,
        local_start_y: f64,
        col_starts: &[f64],
        resolved_widths: &[f64],
        row_heights: &[f64],
        row_tracks: &[TrackSizing],
        inset: Sides<Length>,
        stroke: Option<&Stroke>,
        fill: Option<&Color>,
    ) {
        let cell = &placed.body;

        // P234 — bounds reais usando placed.col/colspan ×
        // resolved_widths + placed.row/rowspan × row_heights.
        let (cell_x, cell_y, cell_w, cell_h) =
            cell_bounds(placed, col_starts, resolved_widths, row_heights, row_start_y);

        // P230 + P235 — extrair per-cell 5 fields (stroke + fill
        // cosméticos P230; align + inset + breakable algorítmicos
        // P235). Match em `placed.body` preserva GridCell wrapper
        // P234.
        let (cell_stroke, cell_fill, cell_align, cell_inset, _cell_breakable) = match cell
        {
            // Modelo D (Lote 12 P327): arm `|`-combinado GridCell+TableCell
            // dividido — TableCell já é `Arc<…Elem>`, GridCell ainda struct
            // (migra a seguir). Bindings com tipos distintos não partilham
            // or-pattern; split permanente.
            Content::TableCell(e) if e.resolved.is_some() => {
                let plan = e.resolved.as_ref().unwrap();
                let defaults = prepared_default(cell);
                (
                    plan.stroke.as_ref(),
                    plan.fill.as_ref(),
                    defaults.and_then(|r| r.align),
                    defaults.map(|r| &r.inset),
                    Some(plan.breakable),
                )
            }
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
        let prepared = matches!(cell,Content::TableCell(c) if c.resolved.is_some());
        let effective_stroke: Option<&Stroke> =
            if prepared { cell_stroke } else { cell_stroke.or(stroke) };
        let effective_fill: Option<&Color> =
            if prepared { cell_fill } else { cell_fill.or(fill) };
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
        let effective_align = if prepared {
            cell_align
        } else {
            match (cell_align, self.cell_align) {
                (None, None) => None,
                (Some(c), None) => Some(c),
                (None, Some(g)) => Some(g),
                (Some(c), Some(g)) => Some(Align2D { h: c.h.or(g.h), v: c.v.or(g.v) }),
            }
        };
        // P235 — inset per-cell override Grid-level; default Grid inset.
        let effective_inset: Sides<Length> = cell_inset.cloned().unwrap_or_else(|| {
            if prepared {
                Sides::uniform(Length::pt(0.0))
            } else {
                inset
            }
        });
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
        let measured_product = measured_cells.take(self, placed, body_x, body_w);
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

        // Use the measured product only under the cache owner's proof;
        // otherwise retain the existing cell-region layout.
        // P235 — layout em body_x/body_w reduzidos por inset.
        let saved_cursor_x = self.regions.current.cursor_x;
        let saved_cursor_y = self.regions.current.cursor_y;
        let saved_cell_bidi_allowed = self.cell_bidi_allowed;
        self.cell_bidi_allowed = col_starts.len() == 1 && body_w.is_finite();
        let (cell_h_measured, cell_items, cell_deco, orphaned_align_x, orphaned_align_y) =
            measured_product.unwrap_or_else(|| {
                self.layout_sub_frame(
                    &cell_to_layout,
                    super::super::sub_frame::SubLayoutRegion {
                        origin_x: body_x,
                        width: body_w,
                        height: None,
                        align_rtl: true,
                        unconstrained_height: true,
                    },
                )
            });
        self.cell_bidi_allowed = saved_cell_bidi_allowed;
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
                fill: Some(crate::entities::frame_paint::FramePaint::Solid(c.clone())),
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
                    FrameItem::Text { .. }
                        | FrameItem::TextShaped { .. }
                        | FrameItem::TextSeparator { .. }
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
                (path, count, align, content_h, origin_y + body_y, dy, applied_y + body_y)
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
                    self.pending_align_centering
                        .push((path, count, align, content_w, origin_x, applied_x));
                }
                for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                    orphaned_align_y
                {
                    path.insert(0, group_idx);
                    self.pending_align_v_centering
                        .push((path, count, align, content_h, origin_y, dy, applied_y));
                }
                // P248 preservado para Fixed rows.
                self.regions.current.current_items.push(FrameItem::Group {
                    paint_root: false,
                    pos: Point { x: Pt(body_x), y: Pt(body_y) },
                    matrix: crate::entities::layout_types::TransformMatrix::identity(),
                    clip_mask: Some(ShapeKind::Rect),
                    inner_width: body_w,
                    inner_height: body_h,
                    items: translated_items,
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
                    .map(|item| super::super::slicing::item_y_start(item) >= threshold)
                    .collect();
                let compact_idx = |orig_idx: usize, want_tail: bool| -> usize {
                    is_tail[..orig_idx].iter().filter(|&&t| t == want_tail).count()
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
                    for (mut path, count, align, dim, origin, applied) in entries {
                        let start = path[0];
                        let range = start..start + count;
                        if range.end > is_tail.len() {
                            continue;
                        }
                        let all_tail = is_tail[range.clone()].iter().all(|&t| t);
                        let all_head = is_tail[range].iter().all(|&t| !t);
                        if all_head {
                            path[0] = compact_idx(start, false);
                            to_head.push((path, count, align, dim, origin, applied));
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
                let (head_align_x, tail_align_x) = split_orphaned(orphaned_align_x);
                // eixo Y tem um campo extra (`dy`) — mesma lógica,
                // duplicada por causa da forma da tupla.
                let (head_align_y, tail_align_y): (
                    Vec<PendingAlignYEntry>,
                    Vec<PendingAlignYEntry>,
                ) = {
                    let mut to_head = Vec::new();
                    let mut to_tail = Vec::new();
                    for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                        orphaned_align_y
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
                                path, count, align, content_h, origin_y, dy, applied_y,
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
                    self.pending_align_centering
                        .push((path, count, align, content_w, origin_x, applied_x));
                }
                for (mut path, count, align, content_h, origin_y, dy, applied_y) in
                    head_align_y
                {
                    path[0] += insertion_base;
                    self.pending_align_v_centering
                        .push((path, count, align, content_h, origin_y, dy, applied_y));
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
                self.pending_align_v_centering
                    .push((path, count, align, content_h, origin_y, dy, applied_y));
            }
            for item in translated_items {
                self.regions.current.current_items.push(item);
            }
        }
    }
}

#[cfg(test)]
mod smoke {
    #[test]
    fn text_and_separator_share_a_certified_cell_baseline() {
        use super::*;
        use crate::entities::{
            dir::Dir,
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::TextStyle,
            style::Styles,
            value::Value,
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let mut l = super::super::super::Layouter::new(
            super::super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom("text.dir", Value::Dir(Dir::RTL));
        l.style = TextStyle::from(&l.chain);
        let body = Content::sequence(vec![
            Content::styled(
                Content::text("ABC"),
                Styles::new().push_custom("text.dir", Value::Dir(Dir::LTR)),
            ),
            Content::Space,
            Content::text("مرحبا"),
        ]);
        let mut measured =
            measurement_cache::MeasuredCells::new(&l, std::slice::from_ref(&body), 1);
        let placed = PlacedCell { body, col: 0, row: 0, colspan: 1, rowspan: 1 };
        let local_y = l.metrics.vertical_metrics(l.style.size, &l.style).0 .0;
        l.emit_grid_cell_body(
            &mut measured,
            &placed,
            20.0,
            local_y,
            &[10.0],
            &[100.0],
            &[40.0],
            &[TrackSizing::Auto],
            Sides::uniform(Length::ZERO),
            None,
            None,
        );
        let text_y = l
            .regions
            .current
            .current_items
            .iter()
            .find_map(|it| {
                if let FrameItem::Text { pos, .. } = it {
                    Some(pos.y.0)
                } else {
                    None
                }
            })
            .unwrap();
        let space_y = l
            .regions
            .current
            .current_items
            .iter()
            .find_map(|it| {
                if let FrameItem::TextSeparator { pos, .. } = it {
                    Some(pos.y.0)
                } else {
                    None
                }
            })
            .unwrap();
        assert!((text_y - space_y).abs() < 1e-6, "text={text_y} separator={space_y}");
        assert!(l
            .regions
            .current
            .current_items
            .iter()
            .filter(|it| matches!(
                it,
                FrameItem::Text { .. } | FrameItem::TextSeparator { .. }
            ))
            .all(|it| match it {
                FrameItem::Text { bidi, .. } | FrameItem::TextSeparator { bidi, .. } =>
                    bidi.is_some(),
                _ => false,
            }));
        assert!(l.regions.cell.is_none());
    }

    #[test]
    fn d08b3c_plan_fill_survives_replaced_cell_product() {
        use super::*;
        use crate::entities::{
            elements::table_cell::{TableCellResolved, TableCellStage},
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
        };
        use comemo::Track;
        use std::sync::Arc;
        let intr = TagIntrospector::empty();
        let mut l = super::super::super::Layouter::new(
            super::super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        let Content::TableCell(c) =
            Content::table_cell(Content::text("OLD"), None, None, None, None)
        else {
            unreachable!()
        };
        let red = Color::rgb(255, 0, 0);
        let mut c = c.as_ref().clone();
        c.resolved = Some(Arc::new(TableCellResolved {
            x: 0,
            y: 0,
            colspan: 1,
            rowspan: 1,
            fill: Some(red.clone()),
            stroke: None,
            align: None,
            inset: Sides::uniform(Length::pt(5.0)),
            breakable: true,
            visual: Content::text("HIDDEN"),
            stage: TableCellStage::Complete,
            guards: vec![],
        }));
        let body = Content::TableCell(Arc::new(c));
        let mut measured =
            measurement_cache::MeasuredCells::new(&l, std::slice::from_ref(&body), 1);
        let placed = PlacedCell { body, col: 0, row: 0, colspan: 1, rowspan: 1 };
        l.emit_grid_cell_body(
            &mut measured,
            &placed,
            20.0,
            0.0,
            &[10.0],
            &[100.0],
            &[40.0],
            &[TrackSizing::Auto],
            Sides::uniform(Length::ZERO),
            None,
            None,
        );
        assert!(l.regions.current.current_items.iter().any(|it|matches!(it,FrameItem::Shape {fill:Some(crate::entities::frame_paint::FramePaint::Solid(color)),..} if color==&red)));
    }
    #[test]
    fn modulo_compila_e_carrega() {}
}
