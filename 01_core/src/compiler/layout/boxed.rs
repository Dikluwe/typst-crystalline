//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/atomizacao_elementos.md
//! @prompt-hash 018a34a7
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
    e: &BoxedElem,
) {
    if layouter.regions.current.current_line.is_empty() {
        if layouter.prev_margin_is_parbreak {
            let font = layouter.style.size.val();
            // Paridade Vanilla P1119: transição Parágrafo -> Linha de Caixas Inline (Parbreak)
            // Salto exato de baseline = 28.81670pt para fonte base 11pt (ascender 8.4667 + leading 7.15 + spacing 13.2)
            let par_to_box_gap = 28.81670_f64 * (font / 11.0);
            layouter.regions.current.cursor_y = Pt(layouter.prev_line_baseline + par_to_box_gap);
            let top_edge_pt = 7.633997_f64 * (font / 11.0);
            layouter.line_assumed_ascent = top_edge_pt;
            layouter.prev_margin_is_parbreak = false;
            layouter.prev_block_below_pending = 0.0;
            layouter.block_chain_active = false;
            layouter.prev_block_equation_descent = 0.0;
        } else {
            layouter.ensure_initial_baseline();
        }
    }

    let (body, width, height, inset, baseline, outset, radius, clip, fill, stroke) = (
        &e.body,
        &e.width,
        &e.height,
        &e.inset,
        &e.baseline,
        &e.outset,
        &e.radius,
        &e.clip,
        &e.fill,
        &e.stroke,
    );
    let font = layouter.style.size.val();
    let inset_left = inset.left.resolve_pt(font);
    let inset_right = inset.right.resolve_pt(font);
    let inset_top = inset.top.resolve_pt(font);
    let inset_bottom = inset.bottom.resolve_pt(font);

    // P247 — outset paralelo Block (inline): margem externa
    // expande bounds Shape mas para Boxed o eixo Y inline
    // não avança cursor; usa line_height como proxy de
    // altura visual (paridade font-size; refino futuro
    // medir body altura real).
    let outset_left = outset.left.resolve_pt(font);
    let outset_right = outset.right.resolve_pt(font);
    let outset_top = outset.top.resolve_pt(font);
    let outset_bottom = outset.bottom.resolve_pt(font);
    let has_shape = fill.is_some() || stroke.is_some();
    let has_outset = outset_left != 0.0
        || outset_right != 0.0
        || outset_top != 0.0
        || outset_bottom != 0.0;

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

    let parent_line_len_before = layouter.regions.current.current_line.len();
    // **P1120** — as extensões inline acumuladas pelo body ficam isoladas:
    // quando a caixa tem `height` explícita, é a caixa (e não o body) que
    // define a extensão da linha.
    let saved_line_ascent = layouter.line_inline_ascent;
    let saved_line_descent = layouter.line_inline_descent;
    layouter.line_inline_ascent = 0.0;
    layouter.line_inline_descent = 0.0;
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
    layouter.regions.current.current_line.extend(body_items);

    // P273.7 — restore parent_bbox (LIFO). Shape emit do
    // próprio Boxed (linha ~1485) usa parent_bbox outer
    // (paridade Block P273.6 §2.3).
    layouter.parent_bbox = saved_parent_bbox;

    // P243 — restaurar width original.
    layouter.regions.current.width = saved_width;

    // **P1028** — forçar largura exterior quando `width` é especificado.
    let outer_w = if let Some(w) = width {
        outset_left + inset_left + w.resolve_pt(font) + inset_right + outset_right
    } else {
        layouter.regions.current.cursor_x.0 - start_x + inset_right + outset_right
    };
    layouter.regions.current.cursor_x = Pt(start_x + outer_w);

    // **P1120** — ascent do body: o vanilla coloca o body no topo da caixa e
    // a baseline da caixa é a do body, logo o ascent da caixa é o do body e o
    // descent é `h − ascent`. Sem medição do body (texto simples), o ascent é
    // a aresta superior do texto — o mesmo valor que o vanilla usa aí.
    let (text_top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
    let body_ascent = layouter.line_inline_ascent.max(text_top.0);
    let body_descent = layouter.line_inline_descent;
    layouter.line_inline_ascent = saved_line_ascent;
    layouter.line_inline_descent = saved_line_descent;
    if height.is_none() {
        // Sem altura explícita a caixa é do tamanho do body: as extensões
        // medidas pelo body são as da caixa.
        layouter.note_inline_extent(body_ascent, body_descent);
    }

    // P1119/P1120 / P248 — altura explícita: extensão da linha, anotação do
    // `Group` do body e clip quando activo.
    if let Some(h) = height {
        let h_pt = h.resolve_pt(font);
        let box_h = inset_top + h_pt + inset_bottom;
        let top_edge_pt = 7.633997_f64 * (font / 11.0);
        layouter.note_inline_extent(top_edge_pt, (box_h - top_edge_pt).max(0.0));
        let mut found_group = false;
        for item in &mut layouter.regions.current.current_line[parent_line_len_before..] {
            if let FrameItem::Group { inner_height, .. } = item {
                *inner_height = inner_height.max(h_pt);
                found_group = true;
            }
        }
        if !found_group && !layouter.regions.current.current_line[parent_line_len_before..].is_empty() {
            let pos_box = crate::entities::layout_types::Point {
                x: Pt(start_x + outset_left + inset_left),
                y: layouter.regions.current.cursor_y,
            };
            // **P1120** — os items do body foram produzidos em coordenadas
            // absolutas da linha; dentro do `Group` passam a ser locais à
            // matriz do grupo (o exportador soma `pos` outra vez).
            let mut body_items: Vec<FrameItem> = layouter
                .regions
                .current
                .current_line
                .drain(parent_line_len_before..)
                .collect();
            offset_items(&mut body_items, -pos_box.x.0, -pos_box.y.0);
            layouter.regions.current.current_line.push(FrameItem::Group {
                pos: pos_box,
                matrix: crate::entities::layout_types::TransformMatrix::identity(),
                clip_mask: None,
                inner_width: outer_w,
                inner_height: h_pt,
                items: body_items,
            });
        }
        if *clip {
            let avail_w_box = match width {
                Some(w) => w.resolve_pt(font),
                None => layouter.available_width(),
            };
            let (body_w_real, body_h_real) =
                layouter.measure_content_constrained(body, avail_w_box);
            if body_h_real > h_pt {
                let body_items: Vec<FrameItem> = layouter
                    .regions
                    .current
                    .current_line
                    .drain(parent_line_len_before..)
                    .collect();
                let pos_box = crate::entities::layout_types::Point {
                    x: Pt(start_x + outset_left + inset_left),
                    y: layouter.regions.current.cursor_y
                        - layouter
                            .metrics
                            .vertical_metrics(layouter.style.size, &layouter.style)
                            .1,
                };
                layouter.regions.current.current_line.push(FrameItem::Group {
                    pos: pos_box,
                    matrix: crate::entities::layout_types::TransformMatrix::identity(),
                    clip_mask: Some(crate::entities::geometry::ShapeKind::Rect),
                    inner_width: body_w_real,
                    inner_height: h_pt,
                    items: body_items,
                });
            }
        }
    }

    // P247 — emissão FrameItem::Shape inline (fill/stroke/outset).
    // Inline contexto: altura visual = line_height (proxy);
    // ajustada por outset.top + outset.bottom.
    if has_shape || has_outset {
        let (top_edge, bottom_edge) =
            layouter.metrics.text_edges(layouter.style.size, &layouter.style);
        let mut outer_w = outer_w;
        let inner_h = match height {
            Some(h) => h.resolve_pt(font),
            None => top_edge.0 + inset_top + bottom_edge.0.abs() + inset_bottom,
        };
        let mut outer_h = inner_h + outset_top + outset_bottom;
        let mut pos = crate::entities::layout_types::Point {
            x: Pt(start_x),
            // Top da caixa em relação à baseline real: baseline - top_edge - inset_top - outset_top
            y: layouter.regions.current.cursor_y - top_edge - Pt(inset_top + outset_top),
        };
        // P252 — stroke-overhang real activação Boxed
        // (paralelo Block; fecha último scope-out P156H
        // stroke-overhang; Boxed A.4 COMPLETO 6/6).
        // No Typst padrão, o stroke de box() é desenhado centrado na borda
        // sem expandir a geometria exterior do frame (paridade vanilla).
        let radius_is_zero_p247 = radius.top_left
            == crate::entities::layout_types::Length::ZERO
            && radius.top_right == crate::entities::layout_types::Length::ZERO
            && radius.bottom_right == crate::entities::layout_types::Length::ZERO
            && radius.bottom_left == crate::entities::layout_types::Length::ZERO;
        let shape_kind = if radius_is_zero_p247 {
            crate::entities::geometry::ShapeKind::Rect
        } else {
            crate::entities::geometry::ShapeKind::RoundedRect { radii: *radius }
        };
        layouter.regions.current.current_items.insert(
            items_before,
            FrameItem::Shape {
                pos,
                kind: shape_kind,
                width: outer_w,
                height: outer_h,
                fill: *fill,
                stroke: stroke.clone(),
                // P273.6 — Boxed's own shape; gradient relative=parent
                // resolve via outer layouter.parent_bbox (Boxed difere
                // save/restore P273.7).
                parent_bbox_at_emit: layouter.parent_bbox,
            },
        );
    }
}

/// **P1120** — desloca `items` por `(dx, dy)`. Usado ao mover items já
/// posicionados na linha para dentro de um `FrameItem::Group`, cuja matriz
/// `cm` volta a somar a origem do grupo no exportador.
fn offset_items(items: &mut [FrameItem], dx: f64, dy: f64) {
    for item in items {
        match item {
            FrameItem::Text { pos, .. }
            | FrameItem::TextShaped { pos, .. }
            | FrameItem::Glyph { pos, .. }
            | FrameItem::Shape { pos, .. }
            | FrameItem::Image { pos, .. }
            | FrameItem::Group { pos, .. } => {
                pos.x = Pt(pos.x.0 + dx);
                pos.y = Pt(pos.y.0 + dy);
            }
            FrameItem::Line { start, end, .. } => {
                start.x = Pt(start.x.0 + dx);
                start.y = Pt(start.y.0 + dy);
                end.x = Pt(end.x.0 + dx);
                end.y = Pt(end.y.0 + dy);
            }
            FrameItem::Link { .. } => {}
        }
    }
}
