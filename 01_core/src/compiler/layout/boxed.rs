//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/boxed.md
//! @prompt-hash 68005e6e
//! @layer L1
//! @updated 2026-10-17
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
    if !super::html_frame::items_have_material(&layouter.regions.current.current_line) {
        if e.width_fraction.is_none() && layouter.block_chain_active {
            let gap = if layouter.prev_margin_is_parbreak {
                layouter
                    .prev_block_below_pending
                    .max(super::paragraph::spacing_for_size(
                        &layouter.chain,
                        layouter.style.size.0,
                    ))
            } else {
                layouter.prev_block_below_pending
            };
            layouter.regions.current.cursor_y = Pt(layouter.prev_line_baseline
                + layouter.prev_block_equation_descent
                + gap);
            // The sized frame installs its own ascent at this geometric origin.
            layouter.initial_baseline_pending = true;
            layouter.line_assumed_ascent = None;
            layouter.prev_margin_is_parbreak = false;
            layouter.prev_block_below_pending = 0.0;
            layouter.block_chain_active = false;
            layouter.prev_block_equation_descent = 0.0;
        } else if e.width_fraction.is_none() && at_fresh_root_origin(layouter) {
            layouter.regions.current.cursor_y = Pt(layouter.page_config.margin.top);
            layouter.initial_baseline_pending = true;
            layouter.line_assumed_ascent = None;
            layouter.prev_margin_is_parbreak = false;
        } else if e.width_fraction.is_some() && layouter.prev_margin_is_parbreak {
            let font = layouter.style.size.val();
            // Paridade Vanilla P1119: transição Parágrafo -> Linha de Caixas Inline (Parbreak)
            // Salto exato de baseline = 28.81670pt para fonte base 11pt (ascender 8.4667 + leading 7.15 + spacing 13.2)
            let par_to_box_gap = 28.81670_f64 * (font / 11.0);
            layouter.regions.current.cursor_y =
                Pt(layouter.prev_line_baseline + par_to_box_gap);
            // rationale: calibração P1119/P1120 normalizada da base medida de 11pt; owner `boxed.md`.
            let top_edge_pt = 7.633997_f64 * (font / 11.0);
            layouter.line_assumed_ascent = Some(top_edge_pt);
            layouter.prev_margin_is_parbreak = false;
            layouter.prev_block_below_pending = 0.0;
            layouter.block_chain_active = false;
            layouter.prev_block_equation_descent = 0.0;
        } else if e.width_fraction.is_some() || !layouter.initial_baseline_pending {
            layouter.ensure_initial_baseline();
        }
    }

    if e.width_fraction.is_some() {
        layout_fractional(layouter, e);
    } else {
        layout_sized(layouter, e);
    }
}

// A new root region may have a predicted font baseline from the preceding
// text flow. With no causal unit here, the first frame owns its geometric top.
fn at_fresh_root_origin<M: FontMetrics, S: ImageSizer>(
    layouter: &Layouter<M, S>,
) -> bool {
    !layouter.initial_baseline_pending
        && !layouter.is_sub_frame
        && layouter.page_config.height.is_finite()
        && layouter.regions.cell.is_none()
        && !layouter.column_mode
        && layouter.page_columns.is_none()
        && layouter.regions.current.height == layouter.page_config.height
        && !super::html_frame::items_have_causal_line(
            &layouter.regions.current.current_items,
        )
        && !super::html_frame::items_have_causal_line(
            &layouter.regions.current.current_line,
        )
        && layouter.flow_placement_end.is_none()
        && layouter.last_flush_advance == 0.0
        && layouter.floats_pending.is_empty()
        && layouter.cursor_y_top_reserve == 0.0
        && layouter.cursor_y_bottom_reserve == 0.0
        && layouter.decoration_lines_collector.is_none()
        && !layouter.flow_suffix_checkpoint_pending
        && !layouter.flow_suffix_transaction_active
        && layouter.flow_suffix_checkpoint.is_none()
}

// Fractional widths are allocated at line closure (composition stage B).
fn layout_fractional<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &BoxedElem,
) {
    let (
        body,
        width,
        width_fraction,
        height,
        inset,
        baseline,
        outset,
        radius,
        clip,
        fill,
        stroke,
    ) = (
        &e.body,
        &e.width,
        &e.width_fraction,
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

    let baseline_shift = baseline.resolve_pt(font);
    let body_measure_width = width.map(|w| w.resolve_pt(font)).unwrap_or_else(|| {
        super::paragraph_indent::current_width(layouter, layouter.available_width())
    });
    let (measured_body_width, measured_body_height) =
        layouter.measure_content_constrained(body, body_measure_width);

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
    // P625/P631/P1491 — layout inline do body isolando a linha do pai e
    // aplicando RTL apenas aos itens produzidos pelo body. A separação
    // física da linha também impede um body de bloco (por exemplo, rect) de
    // descarregar os itens inline que precedem a caixa.
    let parent_line = std::mem::take(&mut layouter.regions.current.current_line);
    // P1502 — os pendentes pertencem à mesma linha que seus índices.
    let parent_pending_fr = std::mem::take(&mut layouter.regions.current.pending_fr);
    let parent_pending_fr_natural_widths =
        std::mem::take(&mut layouter.pending_fr_natural_widths);
    let parent_pending_repeats = std::mem::take(&mut layouter.pending_repeats);
    let body_origin_x = layouter.regions.current.cursor_x.0;
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
    // O body foi disposto com índices locais; rebasear antes de reunir
    // seu estado com o prefixo, sem expor pendentes do pai a flushes internos.
    let mut body_pending_fr = std::mem::take(&mut layouter.regions.current.pending_fr);
    let mut body_pending_fr_natural_widths =
        std::mem::take(&mut layouter.pending_fr_natural_widths);
    let mut body_pending_repeats = std::mem::take(&mut layouter.pending_repeats);
    for (index, _) in &mut body_pending_fr {
        *index += parent_line_len_before;
    }
    for repeat in &mut body_pending_repeats {
        repeat.start += parent_line_len_before;
        repeat.end += parent_line_len_before;
    }
    layouter.regions.current.current_line = parent_line;
    layouter.regions.current.pending_fr = parent_pending_fr;
    layouter.regions.current.pending_fr.append(&mut body_pending_fr);
    layouter.pending_fr_natural_widths = parent_pending_fr_natural_widths;
    layouter
        .pending_fr_natural_widths
        .append(&mut body_pending_fr_natural_widths);
    layouter.pending_repeats = parent_pending_repeats;
    layouter.pending_repeats.append(&mut body_pending_repeats);
    if body_items.is_empty() && measured_body_width > 0.0 {
        offset_items(
            &mut layouter.regions.current.current_items[body_items_before..],
            body_origin_x,
            0.0,
        );
        layouter.regions.current.cursor_x = Pt(body_origin_x + measured_body_width);
    }
    let body_has_text_baseline = frame_items_have_text_baseline(&body_items);
    layouter.regions.current.current_line.extend(body_items);
    layouter.text_tracking_state.interrupt();
    let fraction_insert_at =
        width_fraction.map(|_| layouter.regions.current.current_line.len());

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
    if let (Some(fr), Some(insert_at)) = (width_fraction, fraction_insert_at) {
        layouter.regions.current.pending_fr.push((insert_at, *fr));
        layouter.pending_fr_natural_widths.push(outer_w);
    }

    // **P1120** — ascent do body: o vanilla coloca o body no topo da caixa e
    // a baseline da caixa é a do body, logo o ascent da caixa é o do body e o
    // descent é `h − ascent`. Sem medição do body (texto simples), o ascent é
    // a aresta superior do texto — o mesmo valor que o vanilla usa aí.
    let (text_top, _) = layouter.metrics.text_edges(layouter.style.size, &layouter.style);
    let body_ascent = layouter.line_inline_ascent.max(text_top.0);
    let body_descent = layouter.line_inline_descent;
    layouter.line_inline_ascent = saved_line_ascent;
    layouter.line_inline_descent = saved_line_descent;

    // **P1491** — a baseline explícita desloca a baseline natural do frame,
    // sem alterar a sua altura. Corpos textuais herdam a baseline do texto;
    // corpos sem baseline natural (por exemplo, rect) alinham pela base.
    let natural_body_height = if body_has_text_baseline {
        body_ascent + body_descent
    } else {
        measured_body_height
    };
    let box_height = height.map_or(inset_top + natural_body_height + inset_bottom, |h| {
        inset_top + h.resolve_pt(font) + inset_bottom
    });
    let natural_baseline =
        if body_has_text_baseline { inset_top + body_ascent } else { box_height };
    let frame_baseline = natural_baseline - baseline_shift;
    layouter.note_inline_extent(
        frame_baseline.max(0.0),
        (box_height - frame_baseline).max(0.0),
    );

    // P1119/P1120 / P248 — altura explícita: extensão da linha, anotação do
    // `Group` do body e clip quando activo.
    if let Some(h) = height {
        let h_pt = h.resolve_pt(font);
        let mut found_group = false;
        for item in &mut layouter.regions.current.current_line[parent_line_len_before..] {
            if let FrameItem::Group { inner_height, .. } = item {
                *inner_height = inner_height.max(h_pt);
                found_group = true;
            }
        }
        if !found_group
            && !layouter.regions.current.current_line[parent_line_len_before..].is_empty()
        {
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
                paint_root: false,
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
                None => super::paragraph_indent::current_width(
                    layouter,
                    layouter.available_width(),
                ),
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
                    paint_root: false,
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
        let inner_h = match height {
            Some(h) => h.resolve_pt(font),
            None => top_edge.0 + inset_top + bottom_edge.0.abs() + inset_bottom,
        };
        let outer_h = inner_h + outset_top + outset_bottom;
        let pos = crate::entities::layout_types::Point {
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
            super::resolve_shape_kind(
                &crate::entities::geometry::ShapeKind::RoundedRect { radii: *radius },
                layouter.style.size,
            )
        };
        let prepared_stroke = super::tiling::resolve_stroke(layouter, stroke.as_ref());
        layouter.regions.current.current_items.insert(
            items_before,
            FrameItem::Shape {
                pos,
                kind: shape_kind,
                width: outer_w,
                height: outer_h,
                fill: fill.clone().map(crate::entities::frame_paint::FramePaint::Solid),
                stroke: prepared_stroke,
                // P273.6 — Boxed's own shape; gradient relative=parent
                // resolve via outer layouter.parent_bbox (Boxed difere
                // save/restore P273.7).
                fill_rule: Default::default(),
                parent_bbox_at_emit: layouter.parent_bbox,
            },
        );
    }
}

/// Compose the full body in a local region before placing this inline object.
fn layout_sized<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &BoxedElem,
) {
    use crate::entities::geometry::ShapeKind;
    use crate::entities::layout_types::{Length, Point, Rect, TransformMatrix};

    let font = layouter.style.size.0;
    let resolve = |s: crate::entities::sides::Sides<Length>| {
        crate::entities::sides::Sides::new(
            s.left.resolve_pt(font),
            s.top.resolve_pt(font),
            s.right.resolve_pt(font),
            s.bottom.resolve_pt(font),
        )
    };
    let inset = resolve(e.inset);
    let outset = resolve(e.outset);
    let explicit_w = e.width.map(|v| v.resolve_pt(font).max(0.0));
    let explicit_h = e.height.map(|v| v.resolve_pt(font).max(0.0));
    let inner_w = (explicit_w.unwrap_or_else(|| {
        super::paragraph_indent::current_width(layouter, layouter.available_width())
    }) - inset.left
        - inset.right)
        .max(0.0);
    let inner_h = explicit_h.map(|h| (h - inset.top - inset.bottom).max(0.0));
    let mut start = Point {
        x: layouter.regions.current.cursor_x,
        y: layouter.regions.current.cursor_y,
    };
    let saved_bbox = layouter.parent_bbox;
    if let (Some(w), Some(h)) = (explicit_w, explicit_h) {
        layouter.parent_bbox = Some(Rect {
            x: Pt(-inset.left),
            y: Pt(-inset.top),
            w: Pt(w),
            h: Pt(h),
        });
    }
    let (body_h, mut body, deco, align_x, align_y) = layouter
        .layout_sub_frame_at_geometric_origin(
            &e.body,
            super::sub_frame::SubLayoutRegion {
                origin_x: 0.0,
                width: inner_w,
                height: inner_h,
                align_rtl: true,
                unconstrained_height: inner_h.is_none(),
            },
        );
    layouter.parent_bbox = saved_bbox;
    let natural_w = super::helpers::line_content_right(&body, &layouter.metrics)
        .max(layouter.last_sub_frame_width);
    let box_w = explicit_w.unwrap_or((inset.left + natural_w + inset.right).max(0.0));
    let box_h = explicit_h.unwrap_or((inset.top + body_h + inset.bottom).max(0.0));
    let final_inner_w = (box_w - inset.left - inset.right).max(0.0);
    let final_inner_h = (box_h - inset.top - inset.bottom).max(0.0);

    // Deferred paths refer to this body, not to the parent's pending line.
    let saved_x = std::mem::replace(&mut layouter.pending_align_centering, align_x);
    layouter.apply_pending_align_fixups(
        &mut body,
        final_inner_w + layouter.page_config.margin.horizontal(),
    );
    layouter.pending_align_centering = saved_x;
    let saved_y = std::mem::replace(&mut layouter.pending_align_v_centering, align_y);
    layouter.apply_pending_align_v_fixups(
        &mut body,
        final_inner_h + layouter.page_config.margin.bottom,
    );
    layouter.pending_align_v_centering = saved_y;

    let natural_baseline = first_baseline(&body).map_or(box_h, |p| inset.top + p.y.0);
    let baseline = natural_baseline - e.baseline.resolve_pt(font);
    if layouter.initial_baseline_pending {
        // A first box installs its own ascent; zero is a valid installed value.
        let ascent = baseline.max(0.0);
        start.y += Pt(ascent);
        layouter.regions.current.cursor_y = start.y;
        layouter.line_assumed_ascent = Some(ascent);
        layouter.initial_baseline_pending = false;
    }
    let pos = Point { x: start.x, y: start.y - Pt(baseline) };
    let paint_w = (box_w + outset.left + outset.right).max(0.0);
    let paint_h = (box_h + outset.top + outset.bottom).max(0.0);
    let half_stroke = e.stroke.as_ref().map_or(0.0, |s| s.thickness.max(0.0) / 2.0);
    let kind = if e.radius == crate::entities::corners::Corners::uniform(Length::ZERO) {
        ShapeKind::Rect
    } else {
        super::resolve_shape_kind(
            &ShapeKind::RoundedRect { radii: e.radius },
            layouter.style.size,
        )
    };
    // The requested radius describes the outside of the uniform border.
    let kind = match kind {
        ShapeKind::RoundedRect { radii } => {
            let center = |r: Pt| {
                Pt((r.0.min(paint_w.min(paint_h) / 2.0 + half_stroke) - half_stroke)
                    .max(0.0))
            };
            ShapeKind::RoundedRect {
                radii: crate::entities::corners::Corners::new(
                    center(radii.top_left),
                    center(radii.top_right),
                    center(radii.bottom_right),
                    center(radii.bottom_left),
                ),
            }
        }
        other => other,
    };
    let mut contents = Vec::new();
    if e.fill.is_some() || e.stroke.is_some() {
        let prepared_stroke = super::tiling::resolve_stroke(layouter, e.stroke.as_ref());
        contents.push(FrameItem::Shape {
            pos: Point { x: Pt(-outset.left), y: Pt(-outset.top) },
            kind: kind.clone(),
            width: paint_w,
            height: paint_h,
            fill: e.fill.clone().map(crate::entities::frame_paint::FramePaint::Solid),
            stroke: prepared_stroke,
            fill_rule: Default::default(),
            parent_bbox_at_emit: saved_bbox.map(|r| Rect {
                x: r.x - pos.x,
                y: r.y - pos.y,
                ..r
            }),
        });
    }
    let body_start = contents.len();
    let body_pos = Point { x: Pt(inset.left), y: Pt(inset.top) };
    let body_frame = FrameItem::Group {
        paint_root: false,
        pos: body_pos,
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: final_inner_w,
        inner_height: final_inner_h,
        items: body,
    };
    if e.clip {
        // Clip at the inner edge so the body cannot paint over the border.
        let clip_kind = match kind {
            ShapeKind::RoundedRect { radii } => {
                let inner = |r: Pt| Pt((r.0 - half_stroke).max(0.0));
                ShapeKind::RoundedRect {
                    radii: crate::entities::corners::Corners::new(
                        inner(radii.top_left),
                        inner(radii.top_right),
                        inner(radii.bottom_right),
                        inner(radii.bottom_left),
                    ),
                }
            }
            other => other,
        };
        let mut child = body_frame;
        super::helpers::offset_frame_item(
            &mut child,
            outset.left - half_stroke,
            outset.top - half_stroke,
        );
        contents.push(FrameItem::Group {
            paint_root: false,
            pos: Point {
                x: Pt(half_stroke - outset.left),
                y: Pt(half_stroke - outset.top),
            },
            matrix: TransformMatrix::identity(),
            clip_mask: Some(clip_kind),
            inner_width: (paint_w - 2.0 * half_stroke).max(0.0),
            inner_height: (paint_h - 2.0 * half_stroke).max(0.0),
            items: vec![child],
        });
    } else {
        contents.push(body_frame);
    }
    // Only the descendants use the new root. The backing is painted in the
    // parent's reference space, before the container establishes its own.
    let body_items = contents.split_off(body_start);
    contents.push(FrameItem::Group {
        paint_root: true,
        pos: Point::ZERO,
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: box_w,
        inner_height: box_h,
        items: body_items,
    });
    if let Some(collector) = layouter.decoration_lines_collector.as_mut() {
        collector.extend(deco.into_iter().map(|seg| super::DecoSegment {
            start_x: seg.start_x + pos.x + Pt(inset.left),
            end_x: seg.end_x + pos.x + Pt(inset.left),
            baseline_y: seg.baseline_y + pos.y + Pt(inset.top),
        }));
    }
    layouter.regions.current.current_line.push(FrameItem::Group {
        paint_root: false,
        pos,
        matrix: TransformMatrix::identity(),
        clip_mask: None,
        inner_width: box_w,
        inner_height: box_h,
        items: contents,
    });
    layouter.regions.current.cursor_x = start.x + Pt(box_w);
    layouter.note_inline_extent(baseline.max(0.0), (box_h - baseline).max(0.0));
    layouter.text_tracking_state.interrupt();
}

/// First textual baseline in frame coordinates; wrappers do not erase it.
fn first_baseline(items: &[FrameItem]) -> Option<crate::entities::layout_types::Point> {
    use crate::entities::layout_types::Point;
    items.iter().find_map(|item| match item {
        FrameItem::Text { pos, .. }
        | FrameItem::TextShaped { pos, .. }
        | FrameItem::Glyph { pos, .. } => Some(*pos),
        FrameItem::Group { pos, matrix, items, .. } => first_baseline(items).map(|p| {
            let (x, y) = matrix.apply(p.x.0, p.y.0);
            Point { x: pos.x + Pt(x), y: pos.y + Pt(y) }
        }),
        FrameItem::Link { items, .. }
        | FrameItem::Semantic { items, .. }
        | FrameItem::TextReplacement { items, .. } => first_baseline(items),
        FrameItem::TextSeparator { .. }
        | FrameItem::Line { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. } => None,
    })
}

/// **P1120** — desloca `items` por `(dx, dy)`. Usado ao mover items já
/// posicionados na linha para dentro de um `FrameItem::Group`, cuja matriz
/// `cm` volta a somar a origem do grupo no exportador.
fn offset_items(items: &mut [FrameItem], dx: f64, dy: f64) {
    for item in items {
        match item {
            FrameItem::TextSeparator { pos, .. }
            | FrameItem::Text { pos, .. }
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
            FrameItem::Link { pos, items, .. } => {
                pos.x = Pt(pos.x.0 + dx);
                pos.y = Pt(pos.y.0 + dy);
                offset_items(items, dx, dy);
            }
            FrameItem::TextReplacement { items, .. }
            | FrameItem::Semantic { items, .. } => offset_items(items, dx, dy),
        }
    }
}

fn frame_items_have_text_baseline(items: &[FrameItem]) -> bool {
    items.iter().any(|item| match item {
        FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::Glyph { .. } => true,
        FrameItem::Group { items, .. }
        | FrameItem::TextReplacement { items, .. }
        | FrameItem::Semantic { items, .. } => frame_items_have_text_baseline(items),
        FrameItem::TextSeparator { .. }
        | FrameItem::Line { .. }
        | FrameItem::Image { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Link { .. } => false,
    })
}

#[cfg(test)]
mod composition_regressions {
    use super::*;
    use crate::entities::{
        content::Content,
        corners::Corners,
        elements::shape::ShapeElem,
        geometry::ShapeKind,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{
            Color, Length, PageMarginSpec, PageMargins, Point, TransformMatrix,
        },
        paint::Paint,
        sides::Sides,
        value::Value,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn box_element(body: Content) -> BoxedElem {
        BoxedElem {
            body,
            width: Some(Length::pt(20.0)),
            width_fraction: None,
            height: Some(Length::pt(14.0)),
            inset: Sides::uniform(Length::ZERO),
            baseline: Length::ZERO,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
        }
    }
    fn shape(width: f64, height: f64) -> Content {
        Content::Shape(Arc::new(ShapeElem {
            origin: crate::entities::elements::shape::ShapeOrigin::Geometry,
            kind: ShapeKind::Rect,
            width: Some(Box::new(Value::Length(Length::pt(width)))),
            height: Some(Box::new(Value::Length(Length::pt(height)))),
            fill: Some(Paint::Solid(Color::rgb(255, 0, 0))),
            stroke: None,
            fill_rule: Default::default(),
        }))
    }
    #[derive(Default, Debug)]
    struct Geometry {
        text: Vec<(String, Point)>,
        shapes: Vec<(Point, f64, f64)>,
        clips: Vec<(Point, f64, f64, ShapeKind<Pt>)>,
    }
    fn inspect(items: &[FrameItem], matrix: TransformMatrix, g: &mut Geometry) {
        let point = |p: Point| {
            let (x, y) = matrix.apply(p.x.0, p.y.0);
            Point { x: Pt(x), y: Pt(y) }
        };
        for item in items {
            match item {
                FrameItem::Text { text, pos, .. }
                | FrameItem::TextShaped { text, pos, .. } => {
                    g.text.push((text.to_string(), point(*pos)))
                }
                FrameItem::Shape { pos, width, height, .. } => {
                    g.shapes.push((point(*pos), *width, *height))
                }
                FrameItem::Group {
                    paint_root: _,
                    pos,
                    matrix: local,
                    clip_mask,
                    inner_width,
                    inner_height,
                    items,
                } => {
                    let m = matrix
                        .concat(&TransformMatrix::translate(pos.x.0, pos.y.0))
                        .concat(local);
                    if let Some(kind) = clip_mask {
                        let (x, y) = m.apply(0.0, 0.0);
                        g.clips.push((
                            Point { x: Pt(x), y: Pt(y) },
                            *inner_width,
                            *inner_height,
                            kind.clone(),
                        ));
                    }
                    inspect(items, m, g);
                }
                FrameItem::Semantic { items, .. }
                | FrameItem::TextReplacement { items, .. }
                | FrameItem::Link { items, .. } => inspect(items, matrix, g),
                FrameItem::TextSeparator { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }
    fn render(e: BoxedElem) -> Geometry {
        render_content(Content::sequence(vec![
            Content::text("P"),
            Content::Boxed(Arc::new(e)),
            Content::text("Q"),
        ]))
    }
    fn render_content(content: Content) -> Geometry {
        let doc = compose_document(content);
        assert_eq!(doc.pages.len(), 1);
        let mut g = Geometry::default();
        inspect(&doc.pages[0].items, TransformMatrix::identity(), &mut g);
        g
    }
    fn compose_document(
        content: Content,
    ) -> crate::entities::layout_types::PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 120.0;
        l.page_config.height = 100.0;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 120.0;
        l.regions.current.height = 100.0;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        l.layout_content(&content);
        l.finish()
    }
    fn point(g: &Geometry, text: &str) -> Point {
        g.text.iter().find(|(s, _)| s == text).unwrap().1
    }
    fn close(a: f64, b: f64) {
        assert!((a - b).abs() < 1e-8, "{a} != {b}");
    }

    #[test]
    fn explicit_dimensions_include_inset_and_exclude_outset() {
        let mut base = box_element(Content::Empty);
        base.fill = Some(Color::rgb(255, 0, 0));
        let plain = render(base.clone());
        base.inset = Sides::uniform(Length::pt(2.0));
        base.outset = Sides::uniform(Length::pt(3.0));
        let padded = render(base);
        assert_eq!(point(&padded, "Q"), point(&plain, "Q"));
        assert_eq!(point(&padded, "P"), point(&plain, "P"));
        assert_eq!(padded.shapes.len(), 1);
        close(padded.shapes[0].1, 26.0);
        close(padded.shapes[0].2, 20.0);
        close(padded.shapes[0].0.x.0, plain.shapes[0].0.x.0 - 3.0);
        close(padded.shapes[0].0.y.0, plain.shapes[0].0.y.0 - 3.0);
    }
    #[test]
    fn clipped_shape_is_local_and_preserves_the_outer_baseline() {
        let mut e = box_element(shape(40.0, 24.0));
        e.clip = true;
        let g = render(e);
        assert_eq!(g.shapes.len(), 1);
        assert_eq!(g.clips.len(), 1);
        close(point(&g, "P").y.0, point(&g, "Q").y.0);
        let (pos, w, h, _) = &g.clips[0];
        close(*w, 20.0);
        close(*h, 14.0);
        close(pos.x.0, 16.6);
        close(pos.y.0, 10.0);
        close(g.shapes[0].0.x.0, pos.x.0);
        close(g.shapes[0].0.y.0, pos.y.0);
    }
    #[test]
    fn clip_uses_horizontal_bounds_and_auto_height() {
        let mut e = box_element(shape(40.0, 8.0));
        e.clip = true;
        e.height = None;
        let g = render(e);
        assert_eq!(g.clips.len(), 1);
        close(g.clips[0].1, 20.0);
        close(g.clips[0].2, 8.0);
        close(point(&g, "P").y.0, point(&g, "Q").y.0);
    }
    #[test]
    fn inset_and_radius_apply_to_the_body_clip() {
        let mut e = box_element(shape(40.0, 24.0));
        e.clip = true;
        e.inset = Sides::uniform(Length::pt(2.0));
        e.radius = Corners::uniform(Length::pt(4.0));
        let g = render(e);
        assert_eq!(g.clips.len(), 1);
        assert!(matches!(g.clips[0].3, ShapeKind::RoundedRect { .. }));
        close(g.clips[0].1, 20.0);
        close(g.clips[0].2, 14.0);
        close(g.shapes[0].0.x.0, g.clips[0].0.x.0 + 2.0);
        close(g.shapes[0].0.y.0, g.clips[0].0.y.0 + 2.0);
    }
    #[test]
    fn no_clip_keeps_overflow_visible_and_inline() {
        let g = render(box_element(shape(40.0, 24.0)));
        assert!(g.clips.is_empty());
        assert_eq!(g.shapes.len(), 1);
        close(g.shapes[0].1, 40.0);
        close(g.shapes[0].2, 24.0);
        close(point(&g, "P").y.0, point(&g, "Q").y.0);
    }
    #[test]
    fn nested_box_has_no_phantom_font_descent() {
        let mut inner = box_element(shape(20.0, 30.0));
        inner.height = Some(Length::pt(30.0));
        let mut outer = box_element(Content::Boxed(Arc::new(inner)));
        outer.width = None;
        outer.height = None;
        outer.clip = true;
        let g = render(outer);
        assert_eq!(g.clips.len(), 1);
        close(g.clips[0].1, 20.0);
        close(g.clips[0].2, 30.0);
        close(g.shapes[0].0.x.0, g.clips[0].0.x.0);
        close(g.shapes[0].0.y.0, g.clips[0].0.y.0);
    }
    #[test]
    fn stroke_clip_preserves_the_inner_half_of_the_border() {
        for outset in [0.0, 3.0] {
            for radius in [0.0, 4.0] {
                let mut e = box_element(shape(40.0, 24.0));
                e.clip = true;
                e.outset = Sides::uniform(Length::pt(outset));
                e.radius = Corners::uniform(Length::pt(radius));
                e.stroke = Some(crate::entities::geometry::Stroke {
                    thickness: 2.0,
                    ..Default::default()
                });
                let g = render(e);
                assert_eq!(g.clips.len(), 1);
                close(g.clips[0].0.x.0, 16.6 - outset + 1.0);
                close(g.clips[0].0.y.0, 10.0 - outset + 1.0);
                close(g.clips[0].1, 18.0 + 2.0 * outset);
                close(g.clips[0].2, 12.0 + 2.0 * outset);
                if radius > 0.0 {
                    let ShapeKind::RoundedRect { radii } = &g.clips[0].3 else {
                        panic!("rounded clip")
                    };
                    assert_eq!(*radii, Corners::uniform(Pt(2.0)));
                }
                // The body origin stays at the box origin even as the clip moves.
                close(g.shapes[1].0.x.0, 16.6);
                close(g.shapes[1].0.y.0, 10.0);
            }
        }
    }

    #[test]
    fn constrained_measurement_preserves_explicit_box_dimensions() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        for body in [Content::Empty, shape(40.0, 24.0)] {
            let mut e = box_element(body);
            e.inset = Sides::uniform(Length::pt(2.0));
            e.outset = Sides::uniform(Length::pt(3.0));
            let content = Content::Boxed(Arc::new(e));
            for available in [100.0, 10.0] {
                assert_eq!(
                    l.measure_content_constrained(&content, available),
                    (20.0, 14.0)
                );
            }
        }
    }

    #[test]
    fn finite_subregion_owns_its_dimensions_even_inside_auto_page() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.margin = PageMargins::uniform(15.0);
        l.regions.current.line_start_x = Pt(3.0);
        l.regions.current.width = 58.0;
        l.regions.current.height = 30.0;
        l.is_sub_frame = true;
        for page in [100.0, f64::INFINITY] {
            l.page_config.width = page;
            l.page_config.height = page;
            close(l.available_width(), 40.0);
            close(l.available_height(), 30.0);
            close(l.page_bottom_limit(), 30.0);
        }
        l.is_sub_frame = false;
        l.page_config.width = 100.0;
        l.page_config.height = 100.0;
        close(l.available_width(), 28.0);
        close(l.available_height(), 0.0);
        close(l.page_bottom_limit(), 15.0);
    }
    #[test]
    fn initial_box_without_text_starts_at_the_region_top() {
        let mut e = box_element(shape(14.0, 10.0));
        e.height = Some(Length::pt(6.0));
        let g = render_content(Content::Boxed(Arc::new(e)));
        close(g.shapes[0].0.y.0, 10.0);
    }

    #[test]
    fn initial_box_followed_by_text_retains_the_text_baseline() {
        let control = render_content(Content::text("Q"));
        for height in [0.0, 6.0] {
            let mut e = box_element(shape(14.0, 10.0));
            e.height = Some(Length::pt(height));
            let g = render_content(Content::sequence(vec![
                Content::Boxed(Arc::new(e)),
                Content::text("Q"),
            ]));
            close(point(&g, "Q").y.0, point(&control, "Q").y.0);
            close(g.shapes[0].0.y.0, point(&g, "Q").y.0 - height);
        }
    }

    #[test]
    fn initial_boxes_share_their_own_maximum_ascent() {
        let mut first = box_element(shape(14.0, 10.0));
        first.height = Some(Length::pt(6.0));
        let mut second = first.clone();
        second.height = Some(Length::pt(4.0));
        let g = render_content(Content::sequence(vec![
            Content::Boxed(Arc::new(first)),
            Content::Boxed(Arc::new(second)),
        ]));
        close(g.shapes[0].0.y.0, 10.0);
        close(g.shapes[1].0.y.0, 12.0);
    }

    #[test]
    fn initial_negative_box_baseline_installs_zero_ascent() {
        let mut e = box_element(shape(14.0, 10.0));
        e.height = Some(Length::pt(6.0));
        e.baseline = Length::pt(10.0);
        let g = render_content(Content::Boxed(Arc::new(e.clone())));
        close(g.shapes[0].0.y.0, 14.0);
        let control = render_content(Content::text("Q"));
        let mixed = render_content(Content::sequence(vec![
            Content::Boxed(Arc::new(e)),
            Content::text("Q"),
        ]));
        close(point(&mixed, "Q").y.0, point(&control, "Q").y.0);
        close(mixed.shapes[0].0.y.0, point(&mixed, "Q").y.0 + 4.0);
    }

    #[test]
    fn initial_nested_box_has_no_extra_font_ascent() {
        let mut inner = box_element(shape(14.0, 10.0));
        inner.height = Some(Length::pt(6.0));
        let mut outer = box_element(Content::Boxed(Arc::new(inner)));
        outer.height = None;
        outer.clip = true;
        let g = render_content(Content::Boxed(Arc::new(outer)));
        close(g.shapes[0].0.y.0, 10.0);
        close(g.clips[0].0.y.0, 10.0);
        close(g.clips[0].2, 6.0);
    }

    #[test]
    fn initial_zero_box_and_text_inside_subframe_use_text_ascent() {
        let mut inner = box_element(shape(14.0, 10.0));
        inner.height = Some(Length::pt(0.0));
        let mut outer = box_element(Content::sequence(vec![
            Content::Boxed(Arc::new(inner)),
            Content::text("Q"),
        ]));
        outer.width = Some(Length::pt(40.0));
        outer.height = None;
        let g = render_content(Content::Boxed(Arc::new(outer)));
        let control = render_content(Content::text("Q"));
        close(point(&g, "Q").y.0, point(&control, "Q").y.0);
        close(g.shapes[0].0.y.0, point(&g, "Q").y.0);
    }
    #[test]
    fn initial_box_uses_the_actual_smaller_text_ascent() {
        use crate::entities::style::{Style, Styles};
        let small = Content::styled(
            Box::new(Content::text("Q")),
            Styles::from_iter([Style::Size(crate::entities::layout_types::Length::pt(
                10.0,
            ))]),
        );
        let control = render_content(small.clone());
        let mut e = box_element(small);
        e.height = None;
        let g = render_content(Content::Boxed(Arc::new(e)));
        close(point(&g, "Q").y.0, point(&control, "Q").y.0);
    }
    fn located(raw: u128, body: Content) -> Content {
        Content::Located(Arc::new(crate::entities::content::LocatedContent {
            location: crate::entities::location::Location::from_raw(raw),
            body,
        }))
    }
    fn published_point(
        doc: &crate::entities::layout_types::PagedDocument,
        raw: u128,
    ) -> Point {
        let pos = doc
            .extracted_positions
            .position_of(crate::entities::location::Location::from_raw(raw))
            .expect("captured Location must be published");
        assert_eq!(pos.page.get(), 1);
        pos.point
    }
    fn no_private_positions(items: &[FrameItem]) {
        for item in items {
            match item {
                FrameItem::Semantic { alt, items, .. } => {
                    assert!(!alt
                        .as_deref()
                        .is_some_and(|s| s.starts_with("\0tekt-position:")));
                    no_private_positions(items);
                }
                FrameItem::Group { items, .. }
                | FrameItem::Link { items, .. }
                | FrameItem::TextReplacement { items, .. } => no_private_positions(items),
                FrameItem::Text { .. }
                | FrameItem::TextShaped { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Line { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Image { .. } => {}
            }
        }
    }
    #[test]
    fn published_box_position_includes_inset_and_page_origin() {
        let mut e = box_element(located(9001, Content::text("A")));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_preserves_parent_and_suffix_identity() {
        let e = box_element(located(9002, Content::text("A")));
        let doc = compose_document(Content::sequence(vec![
            located(9001, Content::text("P")),
            Content::Boxed(Arc::new(e)),
            located(9003, Content::text("Q")),
        ]));
        assert_eq!(doc.extracted_positions.len(), 3);
        close(published_point(&doc, 9001).x.0, 10.0);
        close(published_point(&doc, 9002).x.0, 16.6);
        close(published_point(&doc, 9002).y.0, 10.0);
        close(published_point(&doc, 9003).x.0, 36.6);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_composes_nested_origins_once() {
        let mut inner = box_element(located(9001, Content::text("A")));
        inner.inset = Sides::uniform(Length::pt(2.0));
        let mut outer = box_element(Content::Boxed(Arc::new(inner)));
        outer.inset = Sides::uniform(Length::pt(3.0));
        let doc = compose_document(Content::Boxed(Arc::new(outer)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 15.0);
        close(p.y.0, 15.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_does_not_add_font_ascent_to_shape() {
        let mut e = box_element(located(9001, shape(14.0, 10.0)));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        let mut g = Geometry::default();
        inspect(&doc.pages[0].items, TransformMatrix::identity(), &mut g);
        assert_eq!(g.shapes.len(), 1);
        close(g.shapes[0].0.x.0, 12.0);
        close(g.shapes[0].0.y.0, 12.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_survives_empty_context_body() {
        let mut e = box_element(located(9001, Content::Empty));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let p = published_point(&doc, 9001);
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        no_private_positions(&doc.pages[0].items);
    }
    #[test]
    fn published_box_position_keeps_labels_in_the_same_frame() {
        let mut e =
            box_element(Content::label("anchor", located(9001, Content::text("A"))));
        e.inset = Sides::uniform(Length::pt(2.0));
        let doc = compose_document(Content::Boxed(Arc::new(e)));
        let label = crate::entities::label::Label("anchor".into());
        let p = doc.extracted_label_positions.get(&label).expect("label position");
        close(p.x.0, 12.0);
        close(p.y.0, 12.0);
        assert_eq!(doc.extracted_label_pages.get(&label), Some(&1));
        let context = published_point(&doc, 9001);
        close(context.x.0, p.x.0);
        close(context.y.0, p.y.0);
        no_private_positions(&doc.pages[0].items);
    }

    fn boundary_box(body: Content) -> BoxedElem {
        let mut e = box_element(body);
        e.width = Some(Length::pt(8.0));
        e.height = Some(Length::pt(7.0));
        e.fill = Some(Color::rgb(255, 0, 0));
        e
    }
    fn boundary_content(prefix: Vec<Content>, e: BoxedElem, suffix: Content) -> Content {
        Content::sequence(
            prefix
                .into_iter()
                .chain([Content::Boxed(Arc::new(e)), suffix])
                .collect(),
        )
    }
    fn finite_document(
        content: Content,
        height: f64,
    ) -> crate::entities::layout_types::PagedDocument {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 120.0;
        l.page_config.height = height;
        l.page_config.margin = PageMargins::uniform(10.0);
        l.page_config.margin_spec = PageMarginSpec::uniform(10.0);
        l.regions.current.width = 120.0;
        l.regions.current.height = height;
        l.regions.current.cursor_x = Pt(10.0);
        l.regions.current.line_start_x = Pt(10.0);
        l.regions.current.cursor_y = Pt(10.0);
        l.layout_content(&content);
        l.finish()
    }
    fn page_geometry(
        doc: &crate::entities::layout_types::PagedDocument,
        page: usize,
    ) -> Geometry {
        let mut g = Geometry::default();
        inspect(&doc.pages[page].items, TransformMatrix::identity(), &mut g);
        g
    }
    #[test]
    fn par_box_empty_prefix_uses_actual_box_ascent() {
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::Empty,
        ));
        close(g.shapes[0].0.y.0, 23.2);
    }
    #[test]
    fn par_box_suffix_text_settles_whole_line() {
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 30.9);
        close(g.shapes[0].0.y.0, 23.9);
    }
    #[test]
    fn par_box_previous_text_and_empty_par_have_two_collapsed_gaps() {
        let g = render_content(boundary_content(
            vec![Content::text("A"), Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "A").y.0, 17.7);
        close(point(&g, "Q").y.0, 51.8);
        close(g.shapes[0].0.y.0, 44.8);
    }
    #[test]
    fn par_box_explicit_spacing_survives_lexical_restore() {
        use crate::entities::style::Styles;
        let par = Content::styled(
            Content::par(Content::Empty),
            Styles::new().push_custom("par.spacing", Value::Length(Length::pt(30.0))),
        );
        let g = render_content(boundary_content(
            vec![Content::text("A"), par],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 85.4);
    }
    #[test]
    fn par_box_empty_par_preserves_scoped_font_spacing() {
        use crate::entities::style::{Style, Styles};
        for (size, baseline) in [(8.0, 51.8), (18.0, 68.6)] {
            let par = Content::styled(
                Content::par(Content::Empty),
                Styles::from_iter([Style::Size(
                    crate::entities::layout_types::Length::pt(size),
                )]),
            );
            let g = render_content(boundary_content(
                vec![Content::text("A"), par],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ));
            close(point(&g, "Q").y.0, baseline);
        }
    }
    #[test]
    fn par_box_strong_below_has_no_font_strut() {
        for below in [0.0, 20.0] {
            let mut block = Content::block(
                Content::Empty,
                None,
                Some(Length::pt(8.0)),
                Sides::uniform(Length::ZERO),
                false,
            );
            if let Content::Block(e) = &mut block {
                Arc::make_mut(e).below = Some(Length::pt(below));
            }
            let g = render_content(boundary_content(
                vec![block],
                boundary_box(Content::Empty),
                Content::Empty,
            ));
            close(g.shapes[0].0.y.0, 18.0 + below);
        }
    }
    #[test]
    fn par_box_shifted_baseline_keeps_frame_and_suffix_together() {
        let mut e = boundary_box(Content::Empty);
        e.baseline = Length::pt(10.0);
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            e,
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 30.9);
        close(g.shapes[0].0.y.0, 33.9);
    }
    #[test]
    fn par_box_two_empty_units_keep_internal_gap() {
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty), Content::par(Content::Empty)],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 44.1);
    }
    #[test]
    fn par_box_zero_height_installs_zero_before_text_settles() {
        let mut e = boundary_box(Content::Empty);
        e.height = Some(Length::ZERO);
        let g = render_content(boundary_content(
            vec![Content::par(Content::Empty)],
            e,
            Content::text("Q"),
        ));
        close(point(&g, "Q").y.0, 30.9);
    }
    #[test]
    fn par_box_finite_late_ascent_moves_entire_uncommitted_line() {
        let doc = finite_document(
            boundary_content(
                vec![Content::text("A"), Content::par(Content::Empty)],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ),
            61.5,
        );
        assert_eq!(doc.pages.len(), 2);
        let first = page_geometry(&doc, 0);
        let second = page_geometry(&doc, 1);
        assert_eq!(
            first
                .text
                .iter()
                .filter(|(text, _)| !text.is_empty())
                .map(|(text, _)| text.as_str())
                .collect::<Vec<_>>(),
            ["A"],
            "first={first:?}, second={second:?}"
        );
        assert!(first.shapes.is_empty());
        close(point(&second, "Q").y.0, 17.7);
        close(point(&second, "Q").x.0, 18.0);
        close(second.shapes[0].0.y.0, 10.7);
    }
    #[test]
    fn par_box_fitting_final_line_does_not_create_blank_page() {
        let doc = finite_document(
            boundary_content(
                vec![Content::text("A"), Content::par(Content::Empty)],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ),
            62.0,
        );
        assert_eq!(doc.pages.len(), 1);
        close(point(&page_geometry(&doc, 0), "Q").y.0, 51.8);
    }
    #[test]
    fn par_box_initial_empty_unit_can_precede_region_transition() {
        let doc = finite_document(
            boundary_content(
                vec![Content::par(Content::Empty)],
                boundary_box(Content::Empty),
                Content::text("Q"),
            ),
            30.0,
        );
        assert_eq!(doc.pages.len(), 2);
        assert!(page_geometry(&doc, 0).text.is_empty());
        close(point(&page_geometry(&doc, 1), "Q").y.0, 17.7);
    }
    #[test]
    fn par_box_first_overlong_frame_makes_progress_without_blank_page() {
        let mut e = boundary_box(Content::Empty);
        e.height = Some(Length::pt(45.0));
        let doc = finite_document(boundary_content(vec![], e, Content::text("Q")), 30.0);
        assert_eq!(
            doc.pages.len(),
            1,
            "pages={:?}",
            (0..doc.pages.len())
                .map(|p| page_geometry(&doc, p))
                .collect::<Vec<_>>()
        );
        close(page_geometry(&doc, 0).shapes[0].0.y.0, 10.0);
    }
    #[test]
    fn par_box_position_inside_migrated_frame_has_causal_page() {
        let doc = finite_document(
            boundary_content(
                vec![Content::text("A"), Content::par(Content::Empty)],
                boundary_box(located(9001, Content::Empty)),
                located(9002, Content::text("Q")),
            ),
            61.5,
        );
        assert_eq!(doc.pages.len(), 2);
        for (raw, x, y) in [(9001, 10.0, 10.7), (9002, 18.0, 17.7)] {
            let p = doc
                .extracted_positions
                .position_of(crate::entities::location::Location::from_raw(raw))
                .unwrap();
            assert_eq!(p.page.get(), 2);
            close(p.point.x.0, x);
            close(p.point.y.0, y);
        }
        for page in &doc.pages {
            no_private_positions(&page.items);
        }
    }
    #[test]
    fn par_box_inline_control_keeps_existing_baseline() {
        let g = render_content(boundary_content(
            vec![Content::text("A")],
            boundary_box(Content::Empty),
            Content::text("Q"),
        ));
        close(point(&g, "A").y.0, 17.7);
        close(point(&g, "Q").y.0, 17.7);
        close(g.shapes[0].0.y.0, 10.7);
    }

    #[test]
    fn par_box_previous_region_does_not_make_first_overlong_frame_migrate_again() {
        let mut e = boundary_box(Content::Empty);
        e.height = Some(Length::pt(45.0));
        let doc = finite_document(
            boundary_content(
                vec![Content::par(Content::Empty), Content::pagebreak(false, None)],
                e,
                Content::text("Q"),
            ),
            30.0,
        );
        assert_eq!(doc.pages.len(), 2);
        assert!(page_geometry(&doc, 0).shapes.is_empty());
        close(page_geometry(&doc, 1).shapes[0].0.y.0, 10.0);
    }

    #[test]
    fn par_box_new_region_small_box_uses_geometric_top() {
        for previous in [Content::par(Content::Empty), Content::text("A")] {
            let doc = finite_document(
                boundary_content(
                    vec![previous, Content::pagebreak(false, None)],
                    boundary_box(Content::Empty),
                    Content::Empty,
                ),
                60.0,
            );
            assert_eq!(doc.pages.len(), 2);
            close(page_geometry(&doc, 1).shapes[0].0.y.0, 10.0);
        }
    }
}
