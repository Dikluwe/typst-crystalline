//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/place.md
//! @prompt-hash 701d065b
//! @layer L1
//! @updated 2026-09-01
//!
//! Atomização (ADR-0109, P378): o layout de `Place` movido do monólito
//! `layout_content` para o arquivo da feature (forma B — free function na
//! camada de render). Content-preserving — a lógica é idêntica.

use crate::entities::elements::place::PlaceElem;
use crate::entities::layout_types::{Align2D, Pt, VAlign};

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
        // P1292 v12 — sem alinhamento vertical explícito, Place pertence à
        // unidade de flow corrente. Se a linha ainda está pendente, fornecer
        // sua baseline + top-edge semântico em vez do topo físico da página.
        // Fora de sub-frame, `layout_place` consome `margin.top`; dentro dele,
        // a origem interna é necessariamente zero, portanto traduzimos apenas
        // a cauda desta ocorrência e os metadados que podem reposicioná-la.
        // O ajuste local preserva dx/dy, alinhamento horizontal, rebase,
        // decoração e fixups do owner existente.
        let follows_pending_line =
            matches!(effective_alignment.v, None | Some(VAlign::Top))
                && !layouter.regions.current.current_line.is_empty();
        let semantic_top_edge = follows_pending_line
            .then(|| layouter.metrics.text_edges(layouter.style.size, &layouter.style).0);
        let in_sub_frame = layouter.is_sub_frame;
        let saved_margin_top = layouter.page_config.margin.top;
        if !in_sub_frame {
            if let Some(top_edge) = semantic_top_edge {
                layouter.page_config.margin.top =
                    layouter.regions.current.cursor_y.0 + top_edge.0;
            }
        }

        let item_start = layouter.regions.current.current_items.len();
        let align_y_start = layouter.pending_align_v_centering.len();
        let deco_start = layouter.decoration_lines_collector.as_ref().map_or(0, Vec::len);
        layouter.layout_place(effective_alignment, *dx, *dy, *scope, body);
        layouter.page_config.margin.top = saved_margin_top;

        if in_sub_frame {
            if let Some(top_edge) = semantic_top_edge {
                let delta = top_edge.0;
                for item in &mut layouter.regions.current.current_items[item_start..] {
                    super::helpers::shift_frame_item_y(item, delta);
                }
                for entry in &mut layouter.pending_align_v_centering[align_y_start..] {
                    entry.4 += delta;
                    entry.6 += delta;
                }
                if let Some(segments) = layouter.decoration_lines_collector.as_mut() {
                    for segment in &mut segments[deco_start..] {
                        segment.baseline_y += Pt(delta);
                    }
                }
            }
        }
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
    let region_has_flow = !layouter.regions.current.current_items.is_empty()
        || !layouter.regions.current.current_line.is_empty();
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
