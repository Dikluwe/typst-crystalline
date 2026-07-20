//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/engine/atomizacao_elementos.md
//! @prompt-hash a54abe5a
//! @layer L1
//! @updated 2026-06-18
//!
//! Atomização (ADR-0109, P376): o layout de `Boxed` (Box inline) movido do
//! monólito `layout_content` para o arquivo da feature. Content-preserving —
//! a lógica é idêntica; só o endereço mudou. O `match` no núcleo delega numa linha.

use crate::entities::elements::boxed::BoxedElem;
use crate::entities::layout_types::{FrameItem, Pt};

use super::{FontMetrics, ImageSizer, Layouter};

/// Layout do container INLINE `Boxed` (P156H/P231/P243/P247/P248/P252/P273.7).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e:        &BoxedElem,
) {
    let (body, width, height, inset, baseline, outset, radius, clip, fill, stroke) =
        (&e.body, &e.width, &e.height, &e.inset, &e.baseline, &e.outset, &e.radius, &e.clip, &e.fill, &e.stroke);
    let font = layouter.style.size.val();
    let inset_left  = inset.left.resolve_pt(font);
    let inset_right = inset.right.resolve_pt(font);

    // P247 — outset paralelo Block (inline): margem externa
    // expande bounds Shape mas para Boxed o eixo Y inline
    // não avança cursor; usa line_height como proxy de
    // altura visual (paridade font-size; refino futuro
    // medir body altura real).
    let outset_left   = outset.left.resolve_pt(font);
    let outset_right  = outset.right.resolve_pt(font);
    let outset_top    = outset.top.resolve_pt(font);
    let outset_bottom = outset.bottom.resolve_pt(font);
    let has_shape = fill.is_some() || stroke.is_some();
    let has_outset = outset_left != 0.0 || outset_right != 0.0
                     || outset_top != 0.0 || outset_bottom != 0.0;

    // P247 — snapshot items_before para inserir Shape antes
    // do body (Z-order paralelo Block).
    let items_before = layouter.regions.current.current_items.len();

    // P247 — outset.left avança cursor antes do inset.left.
    // start_x captura cursor ANTES do outset_left para que
    // outer_w = cursor_x_final - start_x cubra o intervalo
    // completo outset.l+inset.l+body+inset.r+outset.r.
    let start_x = layouter.regions.current.cursor_x.0;
    layouter.regions.current.cursor_x += Pt(outset_left);

    // Box é INLINE: avança cursor.x apenas (sem flush_line).
    // Aplica inset_left antes do body.
    layouter.regions.current.cursor_x += Pt(inset_left);

    // P243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)
    // — promoção real `Boxed.width`: quando `Some(w)`, clampa
    // `regions.current.width` ao valor user-provided durante
    // body layout via save/restore.
    let saved_width = layouter.regions.current.width;
    if let Some(w) = width {
        let w_pt = w.resolve_pt(font);
        let cursor_x_pt = layouter.regions.current.cursor_x.0;
        layouter.regions.current.width = (cursor_x_pt + w_pt).max(0.0);
    }

    let _ = baseline; // armazenado; refino futuro

    // P248 — Boxed.height overflow semantic real activação.
    // Quando `height: Some(h)` e body natural altura > h E
    // `clip: true` → wrap body items em FrameItem::Group com
    // `clip_mask: Rect` altura h (reuso mecanismo P242).
    // Quando `clip: false` → emit normal (overflow visível;
    // paridade vanilla). `height: None` → preservado P156H.
    let body_items_before = layouter.regions.current.current_items.len();

    // P273.7 — save/restore parent_bbox análogo Block P273.6
    // (Decisão 3γ.2.γ: popular apenas quando width+height
    // literais). Decisão 1 Fase A `3γ.2.γ-inline-baseline-y`:
    // bbox.y = cursor.y baseline-relative (coerente com P156H
    // limitação line_height inline).
    let saved_parent_bbox = layouter.parent_bbox;
    if let (Some(w), Some(h)) = (width, height) {
        let w_pt = w.resolve_pt(font);
        let h_pt = h.resolve_pt(font);
        layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y,
            w: Pt(w_pt),
            h: Pt(h_pt),
        });
    }

    // P625/P631 — layout inline do body isolando a linha do pai e aplicando
    // RTL apenas aos itens produzidos pelo body. O width do box ainda está
    // activo em `regions.current.width` (configurado acima).
    let (_, body_items) = layouter.layout_sub_frame_inline(
        body,
        super::sub_frame::SubLayoutRegion {
            origin_x: 0.0,
            width: 0.0,
            height: None,
            align_rtl: true,
            unconstrained_height: true,
        },
    );
    layouter.regions.current.current_items.extend(body_items);

    // P273.7 — restore parent_bbox (LIFO). Shape emit do
    // próprio Boxed (linha ~1485) usa parent_bbox outer
    // (paridade Block P273.6 §2.3).
    layouter.parent_bbox = saved_parent_bbox;

    // P243 — restaurar width original.
    layouter.regions.current.width = saved_width;

    // P248 — aplicar clip por overflow se height + clip activos.
    if let Some(h) = height {
        if *clip {
            let h_pt = h.resolve_pt(font);
            // Medir body real (inline): comparação com h_pt.
            let avail_w_box = match width {
                Some(w) => w.resolve_pt(font),
                None    => layouter.available_width(),
            };
            let (body_w_real, body_h_real) =
                layouter.measure_content_constrained(body, avail_w_box);
            if body_h_real > h_pt {
                // Body excede h E clip activo → wrap items
                // emitidos em Group com clip_mask Rect altura h.
                let body_items: Vec<FrameItem> =
                    layouter.regions.current.current_items
                        .drain(body_items_before..).collect();
                let pos_box = crate::entities::layout_types::Point {
                    x: Pt(start_x + outset_left + inset_left),
                    // Baseline-relative: top da caixa ~
                    // cursor_y - line_height (refino futuro).
                    y: layouter.regions.current.cursor_y
                       - layouter.metrics.vertical_metrics(layouter.style.size, &layouter.style).1,
                };
                layouter.regions.current.current_items.push(FrameItem::Group {
                    pos:          pos_box,
                    matrix:       crate::entities::layout_types::TransformMatrix::identity(),
                    clip_mask:    Some(crate::entities::geometry::ShapeKind::Rect),
                    inner_width:  body_w_real,
                    inner_height: h_pt,
                    items:        body_items,
                });
            }
        }
    }

    // Aplica inset_right após body.
    layouter.regions.current.cursor_x += Pt(inset_right);

    // P247 — outset.right após inset.right.
    layouter.regions.current.cursor_x += Pt(outset_right);

    // P247 — emissão FrameItem::Shape inline (fill/stroke/outset).
    // Inline contexto: altura visual = line_height (proxy);
    // ajustada por outset.top + outset.bottom. Width = avanço
    // horizontal total (cursor_x - start_x).
    if has_shape || has_outset {
        let (_, line_h) = layouter.metrics.vertical_metrics(layouter.style.size, &layouter.style);
        // outer_w cobre todo o intervalo (start_x captado
        // ANTES de outset_left).
        let mut outer_w = layouter.regions.current.cursor_x.0 - start_x;
        let inner_h = match height {
            Some(h) => h.resolve_pt(font),
            None    => line_h.0,
        };
        let mut outer_h = inner_h + outset_top + outset_bottom;
        let mut pos = crate::entities::layout_types::Point {
            x: Pt(start_x),
            // Para inline, top da caixa = cursor_y - line_h
            // (proxy; baseline-relative). Refino futuro.
            y: layouter.regions.current.cursor_y - line_h - Pt(outset_top),
        };
        // P252 — stroke-overhang real activação Boxed
        // (paralelo Block; fecha último scope-out P156H
        // stroke-overhang; Boxed A.4 COMPLETO 6/6).
        if let Some(ref s) = stroke {
            if s.overhang {
                let ov = s.thickness / 2.0;
                pos.x = pos.x - Pt(ov);
                pos.y = pos.y - Pt(ov);
                outer_w += 2.0 * ov;
                outer_h += 2.0 * ov;
            }
        }
        let radius_is_zero_p247 =
            radius.top_left == crate::entities::layout_types::Length::ZERO
         && radius.top_right == crate::entities::layout_types::Length::ZERO
         && radius.bottom_right == crate::entities::layout_types::Length::ZERO
         && radius.bottom_left == crate::entities::layout_types::Length::ZERO;
        let shape_kind = if radius_is_zero_p247 {
            crate::entities::geometry::ShapeKind::Rect
        } else {
            crate::entities::geometry::ShapeKind::RoundedRect {
                radii: *radius,
            }
        };
        layouter.regions.current.current_items.insert(items_before, FrameItem::Shape {
            pos,
            kind:   shape_kind,
            width:  outer_w,
            height: outer_h,
            fill:   *fill,
            stroke: stroke.clone(),
            // P273.6 — Boxed's own shape; gradient relative=parent
            // resolve via outer layouter.parent_bbox (Boxed difere
            // save/restore P273.7).
            parent_bbox_at_emit: layouter.parent_bbox,
        });
    }
}
