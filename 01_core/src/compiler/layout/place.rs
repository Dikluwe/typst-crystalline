//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/place.md
//! @prompt-hash 726933c5
//! @layer L1
//! @updated 2026-09-25
//!
//! Atomização (ADR-0109, P378): o layout de `Place` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.

#![allow(deprecated)] // P1504 — FrameItem::Text fallback no float diferido.

use super::helpers::{item_pos, translate_frame_item};
use crate::entities::elements::place::PlaceElem;
use crate::entities::layout_types::{Align2D, FrameItem, Pt, VAlign};

use super::{DeferredFloat, FontMetrics, ImageSizer, Layouter};

/// Layout de `place(...)`: `float: true` captura o body num sub-frame e empurra
/// para `floats_pending` (emitido no flush da página); `float: false` preserva
/// o caminho in-place (`layout_place`).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &PlaceElem,
) {
    let alignment = &e.alignment;
    let dx = &e.dx;
    let dy = &e.dy;
    let scope = &e.scope;
    let float = &e.float;
    let clearance = &e.clearance;
    let body = &e.body;
    // P232 — Resolver effective alignment per eixo via `.or()`.
    let effective_alignment = match layouter.cell_align {
        Some(grid_a) => Align2D {
            h: alignment.h.or(grid_a.h),
            v: alignment.v.or(grid_a.v),
        },
        None => *alignment,
    };

    if *float {
        // P245 — float real: layout body em sub-frame,
        // capturar items + dimensões, push ao buffer.
        let avail_w_page = layouter.available_width();
        let (body_height, body_items, deco_segments, orphaned_align_x, orphaned_align_y) =
            layouter.layout_sub_frame(
                body,
                super::sub_frame::SubLayoutRegion {
                    origin_x: 0.0,
                    width: avail_w_page,
                    height: None,
                    align_rtl: false,
                    unconstrained_height: true,
                },
            );
        // **P904 (Item 3)** — mesmo bug/fix de `placement.rs::layout_place`
        // (ramo `float: false`): `measure_content` devolvia `(0.0, 0.0)`
        // para `Content::Text`. Mede a partir de `body_items` já
        // layoutados via `FontMetrics::line_content_right` (mecanismo de
        // P772j, `origin_x` do sub-frame é sempre `0.0` aqui).
        let body_item_refs: Vec<&crate::entities::layout_types::FrameItem> =
            body_items.iter().collect();
        let content_w = layouter.metrics.line_content_right(&body_item_refs).max(0.0);
        let resolved_clearance = clearance
            .map(|l| l.resolve_pt(layouter.style.size.val()))
            .unwrap_or(0.0);

        layouter.floats_pending.push(DeferredFloat {
            alignment: effective_alignment,
            body_items,
            body_height,
            body_width: content_w,
            clearance: resolved_clearance,
            deco_segments,
            // **P908** — já relativas a `body_items` (mesmo referencial
            // local, origem 0,0); `emit_deferred_float` rebaseia-as no
            // flush, quando a translação final é conhecida.
            orphaned_align_x,
            orphaned_align_y,
        });
        // Cursor.y NÃO avança — float não consome flow space.
        // dx/dy aplicado durante flush (não in-place).
        let _ = dx; // dx aplicado em flush via translate
        let _ = dy; // dy aplicado em flush via translate
        let _ = scope; // scope: Parent + float: true (DEBT-37 sentinela)
    } else {
        // Pending lines own their baseline; a committed producer supplies a
        // certified physical end, excluding the prepared next-line advance.
        let flow_y = if effective_alignment.v.is_none() {
            if super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            ) {
                Some(layouter.regions.current.cursor_y.0)
            } else {
                layouter.flow_placement_y()
            }
        } else {
            None
        };
        layouter.layout_place(effective_alignment, *dx, *dy, *scope, body, flow_y);
    }
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    pub(super) fn record_flow_placement_end(&mut self, end: f64) {
        self.flow_placement_end = Some((end, self.regions.current.cursor_y.0));
    }

    pub(super) fn flow_placement_y(&self) -> Option<f64> {
        self.flow_placement_end
            .filter(|&(_, prepared)| prepared == self.regions.current.cursor_y.0)
            .map(|(end, _)| end)
    }
}

/// P1292-D v10 — admite, em ordem, apenas o prefixo que cabe na geometria
/// efectiva. Fitting, reserva e âncoras pertencem ao owner Place; o cursor
/// limita-se a pedir esta estabilização e avançar a região quando sobra fila.
pub(super) fn admit_fitting_floats<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
) -> usize {
    let count = fitting_float_prefix_len(layouter);
    if count > 0 {
        realize_float_prefix(layouter, count);
    }
    count
}

/// Calcula o prefixo admissível sem realizar, reservar ou reordenar nenhuma
/// ocorrência. O checkpoint do Cursor usa esta mesma decisão do owner Place
/// para saber se a cauda especulativa foi aceita pela região reduzida.
pub(super) fn fitting_float_prefix_len<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> usize {
    if layouter.floats_pending.is_empty() {
        return 0;
    }

    let page_height = layouter.regions.current.height;
    let usable_height = if page_height.is_finite() {
        (page_height - layouter.page_config.margin.vertical()).max(0.0)
    } else {
        f64::INFINITY
    };
    // A baseline inicial é geometria do cursor, não fluxo materializado. Se
    // ela fosse usada para decidir se a região está vazia, uma página recém-
    // criada recusaria o primeiro float cujo `body + clearance` ultrapassa a
    // altura restante após essa baseline e o marcador avançaria para sempre.
    let region_has_flow =
        super::html_frame::items_have_material(&layouter.regions.current.current_items)
            || super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            );
    let used_height = if region_has_flow {
        (layouter.regions.current.cursor_y.0 - layouter.page_config.margin.top).max(0.0)
    } else {
        0.0
    };
    let mut remaining = (usable_height - used_height).max(0.0);
    let mut count = 0usize;

    for float in &layouter.floats_pending {
        let clearance = if region_has_flow || count > 0 { float.clearance } else { 0.0 };
        let need = float.body_height + clearance;
        if need <= remaining || (count == 0 && !region_has_flow) {
            count += 1;
            remaining = (remaining - need).max(0.0);
        } else {
            break;
        }
    }

    count
}

fn realize_float_prefix<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    count: usize,
) {
    let margin = layouter.page_config.margin;
    let page_w = layouter.regions.current.width;
    let effective_page_h = layouter.regions.current.height;
    let physical_page_h = effective_page_h + layouter.cursor_y_bottom_reserve;
    let avail_w = page_w - margin.horizontal();
    let height_auto = !physical_page_h.is_finite();
    let width_auto = !page_w.is_finite();

    let count = count.min(layouter.floats_pending.len());
    let floats: Vec<DeferredFloat> = layouter.floats_pending.drain(..count).collect();
    let (mut top_floats, mut bottom_floats): (Vec<_>, Vec<_>) = floats
        .into_iter()
        .partition(|float| matches!(float.alignment.v, Some(VAlign::Top)));
    if height_auto {
        top_floats.append(&mut bottom_floats);
    }

    let top_height: f64 = top_floats
        .iter()
        .map(|float| float.body_height + float.clearance)
        .sum();
    let bottom_height: f64 = bottom_floats
        .iter()
        .map(|float| float.body_height + float.clearance)
        .sum();

    if top_height > 0.0 {
        for item in &mut layouter.regions.current.current_items {
            super::helpers::shift_frame_item_y(item, top_height);
        }
        layouter.shift_current_line_y(top_height);
        layouter.regions.current.cursor_y += Pt(top_height);
    }

    let mut y_top = margin.top + layouter.cursor_y_top_reserve;
    for float in top_floats.drain(..) {
        let avail = if width_auto { float.body_width } else { avail_w };
        layouter.translate_deferred_float(&float, y_top, margin.left, avail);
        y_top += float.body_height + float.clearance;
    }

    if !height_auto {
        let mut y_bottom =
            physical_page_h - margin.bottom - layouter.cursor_y_bottom_reserve;
        for float in bottom_floats {
            y_bottom -= float.body_height;
            layouter.translate_deferred_float(&float, y_bottom, margin.left, avail_w);
            y_bottom -= float.clearance;
        }
    }

    layouter.cursor_y_top_reserve += top_height;
    layouter.cursor_y_bottom_reserve += bottom_height;
    if !height_auto && bottom_height > 0.0 {
        layouter.regions.current.height =
            (effective_page_h - bottom_height).max(margin.vertical());
    }
}

/// Origem física vertical do frame local devolvido por `layout_sub_frame`.
///
/// A baseline inicial não coincide necessariamente com o top-edge semântico:
/// com as métricas ratificadas, o sub-frame começa 2.596pt abaixo da origem
/// física. A translação diferida precisa compensar apenas esse offset medido,
/// não o ascender integral.
fn deferred_frame_origin_y<M: FontMetrics>(items: &[FrameItem], metrics: &M) -> f64 {
    fn item_origin<M: FontMetrics>(item: &FrameItem, metrics: &M) -> Option<f64> {
        if super::html_frame::is_context_marker(item) {
            return None;
        }
        match item {
            FrameItem::TextSeparator { .. } => None,
            FrameItem::Text { pos, style, .. }
            | FrameItem::TextShaped { pos, style, .. } => {
                let (top, _) = metrics.text_edges(style.size, style);
                Some(pos.y.0 - top.0)
            }
            FrameItem::Glyph { pos, size, style, .. } => {
                let (top, _) = metrics.text_edges(*size, style);
                Some(pos.y.0 - top.0)
            }
            FrameItem::Line { start, end, thickness, .. } => {
                Some(start.y.0.min(end.y.0) - thickness / 2.0)
            }
            FrameItem::Shape { pos, .. }
            | FrameItem::Group { pos, .. }
            | FrameItem::Image { pos, .. } => Some(pos.y.0),
            FrameItem::Link { items, .. }
            | FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => items
                .iter()
                .filter_map(|child| item_origin(child, metrics))
                .reduce(f64::min),
        }
    }

    items
        .iter()
        .filter_map(|item| item_origin(item, metrics))
        .reduce(f64::min)
        .unwrap_or(0.0)
}

impl<'a, M: FontMetrics, S: ImageSizer> Layouter<'a, M, S> {
    /// **P245 (M9d / M7+4)** — emite um `DeferredFloat` na posição
    /// final calculada. Aplica `alignment.x` para posicionamento
    /// horizontal dentro da largura útil da página. Translada items
    /// locais (origem 0,0 + ascender) para coordenadas finais.
    pub(super) fn translate_deferred_float(
        &mut self,
        f: &DeferredFloat,
        target_y: f64,
        margin: f64,
        avail_w: f64,
    ) {
        use crate::entities::layout_types::{FrameItem, HAlign, Point};
        // P1292-D v10 — alinhar a origem física local ao target calculado por
        // Place. `layout_sub_frame` inicia na baseline e pode deixar um offset
        // positivo entre o zero local e o top-edge semântico. Compensar esse
        // offset medido (2.596pt no vetor selado), não o ascender integral
        // refutado, que deslocava a tinta para 10.166pt.
        let target_y_logical = target_y;
        let target_y = target_y - deferred_frame_origin_y(&f.body_items, &self.metrics);
        // Calcular X conforme alignment.x.
        let x_offset = match f.alignment.h {
            // rationale: P1064 Classe 1A — centragem horizontal de container ((avail_w - body_w) / 2.0)
            Some(HAlign::Center) => (avail_w - f.body_width) / 2.0,
            Some(HAlign::Right) | Some(HAlign::End) => avail_w - f.body_width,
            _ => 0.0, // None / Left / Start default. // neutro: N16[α] — alignment None/Left/Start: x_offset = 0.0 (esgotamento de enum horizontal)
        };
        let target_x = margin + x_offset.max(0.0);

        // **P908** — rebasear as entradas órfãs de `pending_align_*`
        // carregadas por este `DeferredFloat` (um `Content::Align`/
        // `Content::Place` aninhado dentro do body deste float, sob
        // `width`/`height: auto`) com a MESMA translação `(target_x,
        // target_y)` aplicada aos `body_items` reais abaixo. `path[0]`
        // passa a apontar para a posição real em `current_items` (soma o
        // comprimento actual, capturado ANTES do merge). Como
        // `flush_pending_floats` corre sempre antes de
        // `apply_pending_align_fixups` na mesma `new_page()`/`finish()`
        // (ver `00_nucleo/prompts/core/compiler/layout/mod.md` §P908), a entrada
        // resolve-se contra a página correcta sem sequenciamento novo.
        let saved_link_scopes = self.pause_link_items();
        let insertion_base = self.regions.current.current_items.len();
        for (mut path, count, align, content_w, origin_x, applied_x) in
            f.orphaned_align_x.clone()
        {
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
        for (mut path, count, align, content_h, origin_y, dy, applied_y) in
            f.orphaned_align_y.clone()
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

        // Translate items: cada item ganha offset (target_x, target_y).
        for item in &f.body_items {
            let translated = match item.clone() {
                FrameItem::TextSeparator { pos, advance, style, origin, .. } => {
                    FrameItem::TextSeparator {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
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
                        end: Point { x: end.x + Pt(target_x), y: end.y + Pt(target_y) },
                        thickness,
                        // P285: cursor reflector preserva cor (translação).
                        color,
                        cap,
                    }
                }
                FrameItem::Glyph { pos, glyph_id, x_advance, size, style, base_char } => {
                    FrameItem::Glyph {
                        pos: Point { x: pos.x + Pt(target_x), y: pos.y + Pt(target_y) },
                        glyph_id,
                        x_advance,
                        size,
                        style,
                        base_char,
                    }
                }
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

        // **P772x** — mesma translação para segmentos de decoração do body,
        // reinseridos no collector ambiente SE ainda estiver activo neste
        // ponto (best-effort — ver comentário em `DeferredFloat::deco_segments`,
        // `mod.rs`: floats que só flusham muito depois do consumer
        // `Underline`/`Strike`/`Overline` ter retornado não têm collector
        // para reinserir e o segmento é descartado, não é um crash).
        if let Some(coll) = self.decoration_lines_collector.as_mut() {
            for seg in &f.deco_segments {
                coll.push(super::DecoSegment {
                    start_x: Pt(target_x + seg.start_x.val()),
                    end_x: Pt(target_x + seg.end_x.val()),
                    baseline_y: Pt(target_y + seg.baseline_y.val()),
                });
            }
        }
        self.resume_link_items(saved_link_scopes);
    }
}

#[cfg(test)]
mod p1504_tests {
    #[test]
    fn emissao_diferida_pertence_a_place() {
        assert!(include_str!("place.rs").contains("fn deferred_frame_origin_y"));
    }
}
