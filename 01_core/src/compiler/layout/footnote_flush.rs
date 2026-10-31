//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/footnote_flush.md
//! @prompt-hash 1e4f74ab
//! @layer L1
//! @updated 2026-10-24
//!
//! Flush dos footnote bodies pendentes (`flush_pending_footnote_bodies`):
//! greedy measure-then-fit, defer cross-page, aviso quando o body excede
//! a página/coluna. Extraído de `layout/cursor.rs` no P847 (um ficheiro,
//! um prompt).
#![allow(deprecated)] // P483 — FrameItem::Text fallback path legítimo
use crate::entities::{
    image_sizer::ImageSizer,
    layout_types::{FrameItem, Point, Pt},
};

use super::helpers::{item_pos, translate_frame_item};
use super::metrics::FontMetrics;

type EntryFrame = (
    f64,
    Vec<FrameItem>,
    Vec<super::DecoSegment>,
    Vec<super::PendingAlignXEntry>,
    Vec<super::PendingAlignYEntry>,
);

impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter<'a, M, S> {
    fn layout_entry_separator(
        &mut self,
        entry: &super::footnote::EntrySettings,
        left_x: f64,
        width: f64,
    ) -> EntryFrame {
        let Some(content) = &entry.separator else {
            return (
                0.0,
                vec![FrameItem::Line {
                    start: Point { x: Pt(left_x), y: Pt(0.0) },
                    end: Point { x: Pt(left_x + width * 0.3), y: Pt(0.0) },
                    thickness: entry.style.size.0 * 0.05,
                    color: Some(crate::entities::color::Color::rgb(0, 0, 0)),
                    cap: crate::entities::geometry::LineCap::Butt,
                }],
                Vec::new(),
                Vec::new(),
                Vec::new(),
            );
        };
        let style = std::mem::replace(&mut self.style, entry.style.clone());
        let chain = std::mem::replace(&mut self.chain, entry.chain.clone());
        let frame = self.layout_sub_frame_at_geometric_origin(
            content,
            super::sub_frame::SubLayoutRegion {
                origin_x: left_x,
                width,
                height: None,
                align_rtl: true,
                unconstrained_height: true,
            },
        );
        self.style = style;
        self.chain = chain;
        frame
    }

    fn place_entry_separator(&mut self, frame: EntryFrame, y: f64) -> f64 {
        let (height, items, deco, orphaned_x, orphaned_y) = frame;
        let base = self.regions.current.current_items.len();
        for (mut path, count, align, content_w, origin_x, applied_x) in orphaned_x {
            path[0] += base;
            self.pending_align_centering
                .push((path, count, align, content_w, origin_x, applied_x));
        }
        for (mut path, count, align, content_h, origin_y, dy, applied_y) in orphaned_y {
            path[0] += base;
            self.pending_align_v_centering.push((
                path,
                count,
                align,
                content_h,
                origin_y + y,
                dy,
                applied_y + y,
            ));
        }
        for mut item in items {
            super::helpers::offset_frame_item(&mut item, 0.0, y);
            self.regions.current.current_items.push(item);
        }
        if let Some(collector) = &mut self.decoration_lines_collector {
            for mut segment in deco {
                segment.baseline_y += Pt(y);
                collector.push(segment);
            }
        }
        height
    }

    pub(super) fn drain_pending_footnotes(
        &mut self,
        bottom_y: Option<f64>,
        certified_only: bool,
    ) {
        let mut stalled = false;
        while !self.pending_footnote_bodies.is_empty()
            && (!certified_only
                || self
                    .pending_footnote_frames
                    .first()
                    .and_then(Option::as_ref)
                    .is_some_and(|frame| frame.source_lines))
        {
            let before =
                super::footnote_fragments::pending_work(&self.pending_footnote_frames);
            self.new_page();
            self.flush_footnote_region(bottom_y, stalled);
            let after =
                super::footnote_fragments::pending_work(&self.pending_footnote_frames);
            stalled = after >= before;
        }
    }

    /// **P304 (P295.1)** — flush dos footnote bodies pendentes no
    /// rodapé da página actual. Cada body é layoutado num sub-frame
    /// (`layout_sub_frame`) e posicionado em Y absoluto
    /// bottom-up: a primeira footnote fica imediatamente acima do
    /// limite inferior `page_h - margin`, a segunda abaixo dela, etc.
    /// O marcador formatado da chamada prepende cada body para identificação.
    ///
    /// **P305 (P295.2)** — overflow multi-página: bodies que não
    /// cabem no espaço disponível ficam no buffer; próximo
    /// `new_page()` (ou loop em `finish()`) faz flush deles na
    /// próxima página. Bug latente P304 (overlap silencioso quando
    /// `total_h > available_h`) fixado via greedy fit + defer.
    /// Fallback defensivo: se body único > página inteira, emite
    /// na mesma para evitar loop infinito (paralelo P251 `forwarded_count`
    /// limit).
    ///
    /// Subpadrão "DeferredX buffer + flush em new_page" N=3 cumulativo
    /// (P245 floats + P251 cell tails + P304 footnotes; P305 estende
    /// com cross-page partial drain).
    /// Flush das footnotes pendentes.
    ///
    /// `bottom_y` permite forçar a coordenada Y inferior da área de
    /// footnotes (usado por `columns.rs` quando as notas de um contentor
    /// `#columns()` devem ser empilhadas abaixo do conteúdo, não no fundo
    /// da página).
    pub(super) fn flush_pending_footnote_bodies(&mut self, bottom_y: Option<f64>) {
        self.flush_footnote_region(bottom_y, false);
    }

    pub(super) fn advance_footnote_region(&mut self, force: bool) {
        if force
            && self.current_page_is_empty()
            && !self.pending_footnote_bodies.is_empty()
        {
            self.flush_footnote_region(None, true);
        }
        self.new_page();
    }

    fn flush_footnote_region(&mut self, bottom_y: Option<f64>, terminal_force: bool) {
        use crate::entities::content::Content;
        if self.pending_footnote_bodies.is_empty() {
            return;
        }
        let saved_link_scopes = self.pause_link_items();
        let bodies: Vec<(u32, Box<Content>, Content)> =
            std::mem::take(&mut self.pending_footnote_bodies);
        let mut plans = std::mem::take(&mut self.pending_footnote_plans);
        plans.resize(bodies.len(), None);
        let mut frames = std::mem::take(&mut self.pending_footnote_frames);
        frames.resize(bodies.len(), None);
        let margin = self.page_config.margin;
        let page_w = self.regions.current.width;
        let page_h = self.regions.current.height;
        // P537 — em modo coluna as notas são posicionadas em coordenadas
        // "locais da coluna" (origem no canto superior-esquerdo útil da
        // coluna, i.e. `margin` de offset). O arquivo `columns.rs` depois
        // translada todos os items da coluna para `column_origin_x`.
        let (avail_w, left_x) = if self.column_mode {
            // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
            (self.column_width - margin.horizontal(), margin.left)
        } else {
            // rationale: PageConfig::margin é escalar único (f64) — left=right=top=bottom por definição do tipo (entities/layout_types.rs). 2.0 * margin é verdade algébrica estrutural. P1066.
            (page_w - margin.horizontal(), margin.left)
        };
        let area_bot = bottom_y.unwrap_or(page_h - margin.bottom);

        // P305 — compute top boundary safe: max Y of current_items
        // (above which bodies would overlap main content). Fallback
        // a `margin` se página vazia.
        // **P595** — incluir cursor_y e current_line no cálculo, porque
        // o texto ainda não flushado para current_items também ocupa
        // espaço; ignorá-lo faz a nota sobrepor o conteúdo principal
        // (especialmente em colunas).
        let item_top_y = |it: &FrameItem| {
            if super::html_frame::is_context_marker_only_tree(it) {
                return 0.0;
            }
            match it {
                FrameItem::TextSeparator { .. } => 0.0,
                FrameItem::Text { pos, .. } => pos.y.0,
                FrameItem::TextShaped { pos, .. } => pos.y.0,
                FrameItem::Line { start, .. } => start.y.0,
                FrameItem::Glyph { pos, .. } => pos.y.0,
                FrameItem::Image { pos, .. } => pos.y.0,
                FrameItem::Shape { pos, .. } => pos.y.0,
                FrameItem::Group { pos, .. } => pos.y.0,
                FrameItem::Link { .. } => 0.0,
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. } => items
                    .iter()
                    .map(|child| crate::compiler::layout::helpers::item_pos(child).1)
                    .fold(0.0, f64::max),
            }
        };
        // A prepared baseline on a fresh physical page occupies no note area.
        // Keep explicit spacing and causal blank lines distinct from that state.
        let unused_physical_baseline = !self.is_sub_frame
            && !super::html_frame::items_have_causal_line(
                &self.regions.current.current_items,
            )
            && !super::html_frame::items_have_causal_line(
                &self.regions.current.current_line,
            )
            && self.flow_placement_end.is_none()
            && self.last_flush_advance == 0.0
            && self.regions.current.cursor_y.0
                == margin.top + self.line_assumed_ascent.unwrap_or(0.0);
        let flow_bottom = if frames
            .first()
            .and_then(Option::as_ref)
            .is_some_and(|frame| frame.source_lines)
            && !super::html_frame::items_have_causal_line(
                &self.regions.current.current_line,
            ) {
            self.flow_placement_end
                .map(|(end, _)| end)
                .unwrap_or(self.regions.current.cursor_y.0)
        } else {
            self.regions.current.cursor_y.0
        };
        let top_safe = if unused_physical_baseline {
            margin.top
        } else {
            self.regions
                .current
                .current_items
                .iter()
                .map(item_top_y)
                .chain(self.regions.current.current_line.iter().map(item_top_y))
                .fold(margin.top, f64::max)
                .max(flow_bottom)
        };
        let available_h = (area_bot - top_safe).max(0.0);

        // P305 — greedy measure-then-fit. Bodies que cabem ficam
        // measured (place na pass 2); restantes diferidos para
        // próxima página via `remainder`. Fallback defensivo
        // refinado: emite primeiro body apenas se for maior que
        // a área de conteúdo completa (i.e., não cabe em nenhuma
        // página) — evita loop infinito sem forçar overlap em
        // partial-page overflow normal. Paralelo P251 `forwarded_count`
        // limit.
        let full_avail = (page_h - margin.vertical()).max(0.0);
        #[allow(clippy::type_complexity)]
        let mut measured: Vec<(
            f64,
            Vec<FrameItem>,
            Vec<super::DecoSegment>,
            Vec<super::PendingAlignXEntry>,
            Vec<super::PendingAlignYEntry>,
        )> = Vec::new();
        let mut acc_h = 0.0_f64;
        let mut separator_size = None;
        let mut rendered_separator: Option<EntryFrame> = None;
        let mut continuation_anchor = false;
        let mut entry_gaps = Vec::new();
        let mut remainder: Vec<(u32, Box<Content>, Content)> = Vec::new();
        let mut overflow = false;
        let mut remainder_plans = Vec::new();
        let mut remainder_frames = Vec::new();
        let region =
            (self.pages.len(), self.current_column, self.segmented_par_line_column);
        for (((n, body, prefix), planned), captured) in
            bodies.into_iter().zip(plans).zip(frames)
        {
            let blocked = captured
                .as_ref()
                .is_some_and(|frame| frame.deferred_region == Some(region));
            if overflow || blocked {
                overflow = true;
                remainder.push((n, body, prefix));
                remainder_plans.push(planned);
                remainder_frames.push(captured);
                continue;
            }
            let pass_before = self.page_running_pass.clone();
            let captures_before = self.position_captures.len();
            let nested_bodies_before = self.pending_footnote_bodies.len();
            let nested_plans_before = self.pending_footnote_plans.len();
            // P772g (encerra achado B de P772f) — origin_x real (`left_x`,
            // não 0.0): esta emissão é um "consumidor absoluto" de
            // `layout_sub_frame` (ver 00_nucleo/prompts/core/compiler/layout/mod.md
            // §"Contrato de composição de coordenadas"). Um `Content::Place`
            // aninhado no body da nota emite coordenadas absolutas via
            // `regions.cell`/`cell_origin_x`; se este sub-frame usasse
            // origin_x=0.0 e depois somasse `target_x=left_x` inteiro a
            // todos os itens (como acontecia antes), a origem seria somada
            // duas vezes para o `Place` aninhado.
            let already_captured = captured.is_some();
            let mut frame = captured.unwrap_or_else(|| {
                self.capture_footnote_frame(&prefix, &body, planned.clone())
            });
            let separator_height = match frame.entry.separator_height {
                Some(height) => height,
                None => {
                    let separator = rendered_separator.get_or_insert_with(|| {
                        self.layout_entry_separator(&frame.entry, left_x, avail_w)
                    });
                    frame.entry.separator_height = Some(separator.0);
                    separator.0
                }
            };
            let clearance = if measured.is_empty() {
                frame.entry.clearance + separator_height
            } else {
                0.0
            };
            let gap = frame.entry.gap;
            let overhead = clearance + gap;
            let continuation =
                frame.source_lines && frame.line_continuation && unused_physical_baseline;
            let remaining = if continuation {
                (full_avail - acc_h).max(0.0)
            } else {
                (available_h - acc_h - overhead).max(0.0)
            };
            if frame.height <= remaining {
                continuation_anchor |= continuation;
                if measured.is_empty() {
                    separator_size = Some(frame.entry.clone());
                }
                entry_gaps.push(gap);
                acc_h += overhead + frame.height;
                measured.push((
                    frame.height,
                    frame.items,
                    frame.deco,
                    frame.orphaned_x,
                    frame.orphaned_y,
                ));
                continue;
            }
            let frame = if already_captured {
                match super::footnote_fragments::split_frame(frame, remaining, full_avail)
                {
                    Ok((head, mut tail)) => {
                        continuation_anchor |= continuation;
                        tail.deferred_region = Some(region);
                        if measured.is_empty() {
                            separator_size = Some(head.entry.clone());
                        }
                        entry_gaps.push(gap);
                        acc_h += overhead + head.height;
                        measured.push((
                            head.height,
                            head.items,
                            head.deco,
                            head.orphaned_x,
                            head.orphaned_y,
                        ));
                        overflow = true;
                        remainder.push((n, body, prefix));
                        remainder_plans.push(planned);
                        remainder_frames.push(Some(tail));
                        continue;
                    }
                    Err(frame) => *frame,
                }
            } else {
                frame
            };
            if frame.source_lines
                && !frame.line_continuation
                && unused_physical_baseline
                && measured.is_empty()
            {
                let mut frame = frame;
                if self.page_column_notes
                    && self
                        .page_columns
                        .is_some_and(|count| self.current_column + 1 < count)
                {
                    frame.deferred_region = Some(region);
                    overflow = true;
                    remainder.push((n, body, prefix));
                    remainder_plans.push(planned);
                    remainder_frames.push(Some(frame));
                    continue;
                }
                frame.line_continuation = true;
                frame.deferred_region = Some(region);
                separator_size = Some(frame.entry.clone());
                acc_h += overhead;
                overflow = true;
                remainder.push((n, body, prefix));
                remainder_plans.push(planned);
                remainder_frames.push(Some(frame));
                continue;
            }
            let force_emit = measured.is_empty()
                && ((!frame.source_lines && frame.height > full_avail) || terminal_force);
            if force_emit {
                self.layout_warnings.push(format!(
                    "footnote body [{}] exceeds available column/page height and may be truncated or overlap content",
                    n
                ));
                if measured.is_empty() {
                    separator_size = Some(frame.entry.clone());
                }
                entry_gaps.push(gap);
                acc_h += overhead + frame.height;
                measured.push((
                    frame.height,
                    frame.items,
                    frame.deco,
                    frame.orphaned_x,
                    frame.orphaned_y,
                ));
            } else {
                if !already_captured {
                    self.page_running_pass = pass_before;
                    self.position_captures.truncate(captures_before);
                    self.pending_footnote_bodies.truncate(nested_bodies_before);
                    self.pending_footnote_plans.truncate(nested_plans_before);
                    self.pending_footnote_frames.truncate(nested_plans_before);
                }
                overflow = true;
                remainder.push((n, body, prefix));
                remainder_plans.push(planned);
                remainder_frames.push(already_captured.then_some(frame));
            }
        }

        // Pass 2 — place top-down a partir de `area_bot - acc_h`.
        // Primeira footnote no topo da zona; última no fundo.
        // `layout_sub_frame` posicionou items com ascender
        // offset (cursor_y inicial = ascender). Para alinhar ao
        // target_y absoluto exacto (não baseline), subtrair ascender
        // do offset de translação — paridade pattern `emit_deferred_float`
        // (P245) + `layout_place` (placement.rs).

        // P305 — clamp Y inicial ao top_safe para evitar overlap em
        // defensive emit (body > full_avail). Se acc_h ≤ available_h,
        // clamp é no-op (area_bot - acc_h ≥ top_safe por construção).
        let mut y_cursor = if continuation_anchor {
            area_bot - acc_h
        } else {
            (area_bot - acc_h).max(top_safe)
        };
        if let Some(entry) = separator_size {
            y_cursor += entry.clearance;
            let frame = rendered_separator
                .unwrap_or_else(|| self.layout_entry_separator(&entry, left_x, avail_w));
            y_cursor += self.place_entry_separator(frame, y_cursor);
        }
        for ((h, items, deco, orphaned_x, orphaned_y), gap) in
            measured.into_iter().zip(entry_gaps)
        {
            y_cursor += gap;
            // **P908** — `target_y_logical` (pré-ascender) rebaseia
            // `origin_y`/`applied_y` das entradas órfãs (quantidades sem
            // ascender embutido — mesma distinção física-vs-lógica de
            // `layout_align`/`layout_place`/`emit_deferred_float`,
            // `00_nucleo/prompts/core/compiler/layout/mod.md` §P908); `target_y`
            // (pós-subtracção) só é correcto para os `FrameItem`s reais
            // abaixo, cujo `pos.y` já é baseline.
            let target_y_logical = y_cursor;
            let target_y = y_cursor;
            // P772g — sub-frame já iniciado em `left_x` (não 0.0, ver acima),
            // logo os itens "normais" já vêm absolutos em x; footnotes não
            // têm alinhamento horizontal próprio (sempre ancoradas a
            // `left_x`), logo o deslocamento incremental em x é 0 — mantém-se
            // a variável (agora delta, não valor absoluto) para minimizar o
            // diff no resto do bloco de tradução abaixo.
            let target_x = 0.0;
            // **P908** — rebasear as entradas órfãs devolvidas pelo
            // `layout_sub_frame` deste body (`Content::Align`/
            // `Content::Place` aninhado sob `width`/`height: auto`) com a
            // MESMA translação `(target_x, target_y)` aplicada aos items
            // reais abaixo. `path[0]` passa a apontar para a posição real
            // em `current_items` (soma o comprimento actual, capturado
            // ANTES do merge). Ver
            // `00_nucleo/prompts/core/compiler/layout/mod.md` §P908.
            let insertion_base = self.regions.current.current_items.len();
            for (mut path, count, align, content_w, origin_x, applied_x) in orphaned_x {
                path[0] += insertion_base;
                self.pending_align_centering.push((
                    path,
                    count,
                    align,
                    content_w,
                    origin_x + target_x,
                    applied_x + target_x,
                ));
            }
            for (mut path, count, align, content_h, origin_y, dy, applied_y) in orphaned_y
            {
                path[0] += insertion_base;
                self.pending_align_v_centering.push((
                    path,
                    count,
                    align,
                    content_h,
                    origin_y + target_y_logical,
                    dy,
                    applied_y + target_y_logical,
                ));
            }
            for item in items {
                let translated = match item {
                    FrameItem::TextSeparator { pos, advance, style, origin, .. } => {
                        FrameItem::TextSeparator {
                            pos: Point {
                                x: pos.x + Pt(target_x),
                                y: pos.y + Pt(target_y),
                            },
                            advance,
                            style,
                            origin,
                            bidi: None,
                        }
                    }
                    FrameItem::Text { pos, text, style, .. } => FrameItem::Text {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        text,
                        style,
                        bidi: None,
                    },
                    FrameItem::TextShaped {
                        pos,
                        glyphs,
                        style,
                        text,
                        units_per_em,
                        font,
                        ..
                    } => FrameItem::TextShaped {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        glyphs,
                        style,
                        text,
                        units_per_em,
                        bidi: None,
                        font,
                    },
                    FrameItem::Shape {
                        pos,
                        kind,
                        width,
                        height,
                        fill,
                        stroke,
                        fill_rule,
                        parent_bbox_at_emit,
                    } => FrameItem::Shape {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        kind,
                        width,
                        height,
                        fill,
                        stroke,
                        fill_rule,
                        parent_bbox_at_emit,
                    },
                    FrameItem::Group {
                        paint_root,
                        pos,
                        matrix,
                        clip_mask,
                        inner_width,
                        inner_height,
                        items,
                    } => FrameItem::Group {
                        paint_root,
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        matrix,
                        clip_mask,
                        inner_width,
                        inner_height,
                        items,
                    },
                    FrameItem::Line { start, end, thickness, color, cap } => {
                        FrameItem::Line {
                            start: Point {
                                x: start.x + Pt(target_x),
                                y: start.y + Pt(target_y),
                            },
                            end: Point {
                                x: end.x + Pt(target_x),
                                y: end.y + Pt(target_y),
                            },
                            thickness,
                            color,
                            cap,
                        }
                    }
                    FrameItem::Glyph {
                        pos,
                        glyph_id,
                        x_advance,
                        size,
                        style,
                        base_char,
                    } => FrameItem::Glyph {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        glyph_id,
                        x_advance,
                        size,
                        style,
                        base_char,
                    },
                    FrameItem::Image {
                        pos,
                        data,
                        alt,
                        width,
                        height,
                        intrinsic_width,
                        intrinsic_height,
                        orientation,
                        ..
                    } => FrameItem::Image {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        alt,
                        data,
                        width,
                        height,
                        intrinsic_width,
                        intrinsic_height,
                        clip_rect: None,
                        orientation,
                    },
                    FrameItem::Link { target, items, pos, size } => FrameItem::Link {
                        target,
                        items: items
                            .into_iter()
                            .map(|child| {
                                let (ix, iy) = item_pos(&child);
                                translate_frame_item(
                                    child,
                                    Pt(target_x + ix),
                                    Pt(target_y + iy),
                                )
                            })
                            .collect(),
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        size,
                    },
                    FrameItem::TextReplacement { text, mut items } => {
                        for child in &mut items {
                            crate::compiler::layout::helpers::offset_frame_item(
                                child, target_x, target_y,
                            );
                        }
                        FrameItem::TextReplacement { text, items }
                    }
                    FrameItem::Semantic { kind, placement, alt, mut items } => {
                        for child in &mut items {
                            crate::compiler::layout::helpers::offset_frame_item(
                                child, target_x, target_y,
                            );
                        }
                        FrameItem::Semantic { kind, placement, alt, items }
                    }
                };
                self.regions.current.current_items.push(translated);
            }
            // **P772x** — mesma translação (target_x=0.0, target_y desta
            // iteração) para segmentos de decoração do body da footnote.
            if let Some(coll) = self.decoration_lines_collector.as_mut() {
                for seg in &deco {
                    coll.push(super::DecoSegment {
                        start_x: Pt(target_x + seg.start_x.val()),
                        end_x: Pt(target_x + seg.end_x.val()),
                        baseline_y: Pt(target_y + seg.baseline_y.val()),
                        piece_line: seg.piece_line,
                    });
                }
            }
            y_cursor += h;
        }

        // P305 — re-inserir remainder no buffer para flush na
        // próxima página. `new_page()` chama este método; iteração
        // automática até buffer vazio. `finish()` itera explicitamente
        // via loop com new_page() se buffer não-vazio pós-flush final.
        remainder.append(&mut self.pending_footnote_bodies);
        remainder_plans.append(&mut self.pending_footnote_plans);
        remainder_frames.append(&mut self.pending_footnote_frames);
        self.pending_footnote_bodies = remainder;
        self.pending_footnote_plans = remainder_plans;
        self.pending_footnote_frames = remainder_frames;
        self.footnote_reserved_height = 0.0;
        self.resume_link_items(saved_link_scopes);
    }
}

// ── Testes ─────────────────────────────────────────────────────────────────

#[cfg(test)]
mod smoke {
    #[test]
    fn module_compila_e_carrega() {
        // V2 smoke test — módulo extraído de `cursor.rs` em P847 (um
        // ficheiro, um prompt: `compiler/footnote_overflow_columns.md`).
        // A cobertura funcional de footnote vive nos testes de layout e
        // nos goldens de colunas (P595+).
    }
}

#[cfg(test)]
mod r12_separator_origin_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::TextStyle,
        value::Value,
    };
    use comemo::Track;
    struct Edges;
    impl FontMetrics for Edges {
        fn advance(&self, text: &str, size: Pt, _: &TextStyle) -> Pt {
            Pt(text.chars().count() as f64 * size.0 * 0.5)
        }
        fn vertical_metrics(&self, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (Pt(size.0 * 0.9), Pt(size.0 * 1.2))
        }
        fn cap_height(&self, size: Pt, _: &TextStyle) -> Pt {
            Pt(size.0 * 0.7)
        }
        fn text_edges(&self, size: Pt, _: &TextStyle) -> (Pt, Pt) {
            (Pt(size.0 * 0.7), Pt(-size.0 * 0.2))
        }
    }
    #[test]
    fn r12_text_separator_owns_its_full_geometric_source_box() {
        let intr = TagIntrospector::empty();
        let mut l = super::super::Layouter::new(
            Edges,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom(
            "footnote.entry.separator",
            Value::Content(Content::text("SEP")),
        );
        let entry = super::super::footnote::entry_settings(&l);
        let (height, items, ..) = l.layout_entry_separator(&entry, 20.0, 260.0);
        assert!(
            (height - 9.9).abs() < 1e-8,
            "height={height}; source edges are7.7+2.2pt"
        );
        let baseline = items
            .iter()
            .find_map(|item| match item {
                FrameItem::Text { pos, .. } | FrameItem::TextShaped { pos, .. } => {
                    Some(pos.y.0)
                }
                FrameItem::TextSeparator { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Group { .. }
                | FrameItem::Link { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::TextReplacement { .. } => None,
            })
            .expect("the original separator text must be present");
        assert!(
            (baseline - 7.7).abs() < 1e-8,
            "baseline={baseline}; geometric cap edge7.7pt"
        );
    }
}
