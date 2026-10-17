//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/transform.md
//! @prompt-hash e2acd328
//! @layer L1
//! @updated 2026-08-20
//!
//! Layout de transformações afins (`#rotate`, `#scale`, `#skew`, `#move`).
//! Passo 83 (ADR-0037, P431).
//!
//! Refactored in Passo 832 for nested arbitrary content support with sub-layout.
//! Pivô central e suporte a inline baseline corrigidos nos Passos 1118/1119;
//! pivô pela geometria do frame (sem tabela por matriz) em P1120 —
//! ver `compiler/layout.md` §P1120.

use super::{FontMetrics, ImageSizer, Layouter};
use crate::compiler::layout::helpers::measure_content;
use crate::compiler::layout::sub_frame::SubLayoutRegion;
use crate::entities::elements::transform::{
    TransformElem, TransformOperation, TransformScale,
};
use crate::entities::layout_types::{FrameItem, Point, Pt, TransformMatrix};

/// Layout de uma transformação afim (`move`/`rotate`/`scale`): layouta o
/// body num sub-frame isolado (cobre conteúdo arbitrariamente aninhado —
/// texto, shapes, sequências, outros transforms), mede a AABB dos items
/// produzidos, projecta os cantos pela matriz, compensa origem negativa e
/// emite um `FrameItem::Group` com a matriz final.
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &TransformElem,
) {
    if let TransformOperation::Matrix(matrix) = e.operation {
        layout_matrix(layouter, &e.body, &matrix);
    } else {
        layout_native(layouter, e);
    }
}

fn layout_matrix<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    body: &crate::entities::content::Content,
    matrix: &TransformMatrix,
) {
    let available_w = layouter.available_width();

    // P832 — sub-layout real do body (save/restore completo do estado do
    // layouter; não afecta o cursor do frame pai).
    let (body_h, mut sub_items, deco_segments, orphaned_align_x, orphaned_align_y) =
        layouter.layout_sub_frame(
            body,
            SubLayoutRegion {
                origin_x: 0.0,
                width: available_w,
                height: None,
                align_rtl: true,
                unconstrained_height: true,
            },
        );

    // AABB do body: largura = limite direito real dos items; altura = altura
    // do sub-layout. Fallback à medição aproximada quando o body não produz
    // items mensuráveis (preserva o comportamento pré-P832 para esse caso).
    let measured_w = super::helpers::line_content_right(&sub_items, &layouter.metrics);
    let (orig_w, orig_h) = if measured_w > 0.0 || body_h > 0.0 {
        (measured_w, body_h)
    } else {
        measure_content(body, available_w)
    };

    // Projectar os quatro cantos da AABB original através da matriz.
    let corners = [
        matrix.apply(0.0, 0.0),
        matrix.apply(orig_w, 0.0),
        matrix.apply(0.0, orig_h),
        matrix.apply(orig_w, orig_h),
    ];
    let min_x = corners.iter().map(|(x, _)| *x).fold(f64::INFINITY, f64::min);
    let max_x = corners.iter().map(|(x, _)| *x).fold(f64::NEG_INFINITY, f64::max);
    let min_y = corners.iter().map(|(_, y)| *y).fold(f64::INFINITY, f64::min);
    let max_y = corners.iter().map(|(_, y)| *y).fold(f64::NEG_INFINITY, f64::max);

    let _new_w = max_x - min_x;
    let new_h = max_y - min_y;

    let has_orphaned = !orphaned_align_x.is_empty() || !orphaned_align_y.is_empty();

    let (final_matrix, pos) = if has_orphaned {
        if layouter.regions.current.cursor_y.0 + new_h
            > layouter.regions.current.height - layouter.page_config.margin.bottom
        {
            layouter.new_page();
        }
        layouter.flush_line();
        let pos = Point {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y,
        };
        let align = TransformMatrix::translate(-min_x, -min_y);
        (align.concat(matrix), pos)
    } else {
        let pos = Point {
            x: layouter.regions.current.cursor_x,
            y: layouter.regions.current.cursor_y,
        };
        // Pivô central do frame em relação à baseline (P1118/P1119):
        // Como sub_items são posicionados em relação à baseline (y=0),
        // o centro vertical fica no ponto médio entre -top_edge e bottom_edge.
        let (top_edge, bottom_edge) =
            layouter.metrics.text_edges(layouter.style.size, &layouter.style);
        let cx = orig_w / 2.0;
        let cy = -(top_edge.0 - bottom_edge.0.abs()) / 2.0;
        let t_center = TransformMatrix::translate(cx, cy);
        let t_uncenter = TransformMatrix::translate(-cx, -cy);
        (t_center.concat(matrix).concat(&t_uncenter), pos)
    };

    // P832 — os segmentos de decoração do sub-frame (só existem se o
    // transform estiver aninhado dentro de Underline/Strike/Overline) são
    // re-inseridos no collector ambiente traduzidos pela origem do grupo.
    if let Some(collector) = layouter.decoration_lines_collector.as_mut() {
        for seg in deco_segments {
            collector.push(super::DecoSegment {
                start_x: seg.start_x + pos.x,
                end_x: seg.end_x + pos.x,
                baseline_y: seg.baseline_y + pos.y,
            });
        }
    }

    let (ascender_local, _) = layouter
        .metrics
        .vertical_metrics(layouter.style.size, &layouter.style);

    if has_orphaned {
        let group_idx = layouter.regions.current.current_items.len();
        for (mut path, count, align, content_w, origin_x, applied_x) in orphaned_align_x {
            path.insert(0, group_idx);
            layouter.pending_align_centering.push((
                path,
                count,
                align,
                content_w,
                origin_x + pos.x.val(),
                applied_x + pos.x.val(),
            ));
        }
        for (mut path, count, align, content_h, origin_y, dy, applied_y) in
            orphaned_align_y
        {
            path.insert(0, group_idx);
            layouter.pending_align_v_centering.push((
                path,
                count,
                align,
                content_h,
                origin_y + pos.y.val(),
                dy,
                applied_y + pos.y.val() + ascender_local.val(),
            ));
        }
        layouter.regions.current.current_items.push(FrameItem::Group {
            paint_root: false,
            pos,
            matrix: final_matrix,
            clip_mask: None,
            inner_width: orig_w,
            inner_height: orig_h,
            items: sub_items,
        });
        layouter.regions.current.cursor_y += Pt(new_h);
    } else {
        let base_x = sub_items
            .iter()
            .find_map(|item| match item {
                FrameItem::TextSeparator { pos, .. }
                | FrameItem::Text { pos, .. }
                | FrameItem::TextShaped { pos, .. }
                | FrameItem::Glyph { pos, .. }
                | FrameItem::Group { pos, .. }
                | FrameItem::Shape { pos, .. }
                | FrameItem::Image { pos, .. } => Some(pos.x.0),
                FrameItem::Line { start, .. } => Some(start.x.0),
                FrameItem::TextReplacement { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::Link { .. } => None,
            })
            .unwrap_or(0.0);
        let base_y = sub_items
            .iter()
            .find_map(|item| match item {
                FrameItem::TextSeparator { pos, .. }
                | FrameItem::Text { pos, .. }
                | FrameItem::TextShaped { pos, .. }
                | FrameItem::Glyph { pos, .. }
                | FrameItem::Group { pos, .. }
                | FrameItem::Shape { pos, .. }
                | FrameItem::Image { pos, .. } => Some(pos.y.0),
                FrameItem::Line { start, .. } => Some(start.y.0),
                FrameItem::TextReplacement { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::Link { .. } => None,
            })
            .unwrap_or(ascender_local.0);

        for item in &mut sub_items {
            match item {
                FrameItem::TextSeparator { pos, .. }
                | FrameItem::Text { pos, .. }
                | FrameItem::TextShaped { pos, .. }
                | FrameItem::Glyph { pos, .. }
                | FrameItem::Shape { pos, .. }
                | FrameItem::Image { pos, .. }
                | FrameItem::Group { pos, .. } => {
                    pos.x = Pt(pos.x.0 - base_x);
                    pos.y = Pt(pos.y.0 - base_y);
                }
                FrameItem::Line { start, end, .. } => {
                    start.x = Pt(start.x.0 - base_x);
                    start.y = Pt(start.y.0 - base_y);
                    end.x = Pt(end.x.0 - base_x);
                    end.y = Pt(end.y.0 - base_y);
                }
                FrameItem::Link { .. } => {
                    crate::compiler::layout::helpers::offset_frame_item(
                        item, -base_x, -base_y,
                    );
                }
                FrameItem::TextReplacement { items, .. }
                | FrameItem::Semantic { items, .. } => {
                    for child in items {
                        crate::compiler::layout::helpers::offset_frame_item(
                            child, -base_x, -base_y,
                        );
                    }
                }
            }
        }

        let mut max_x = 0.0_f64;
        let mut min_y = 0.0_f64;
        let mut max_y = 0.0_f64;

        for item in &sub_items {
            if super::html_frame::is_context_marker(item)
                || matches!(item, FrameItem::TextSeparator { .. })
            {
                continue;
            }
            let (ix, iy) = super::helpers::item_pos(item);
            let w = super::helpers::item_width(item, &layouter.metrics);
            max_x = max_x.max(ix + w);

            match item {
                FrameItem::Glyph { size, .. } => {
                    let (top, bottom) =
                        layouter.metrics.text_edges(*size, &layouter.style);
                    min_y = min_y.min(iy - top.0);
                    max_y = max_y.max(iy - bottom.0);
                }
                FrameItem::Text { style, .. } | FrameItem::TextShaped { style, .. } => {
                    let (top, bottom) = layouter.metrics.text_edges(style.size, style);
                    min_y = min_y.min(iy - top.0);
                    max_y = max_y.max(iy - bottom.0);
                }
                FrameItem::Group { inner_height, .. } => {
                    min_y = min_y.min(iy);
                    max_y = max_y.max(iy + inner_height);
                }
                FrameItem::Shape { height, .. } => {
                    min_y = min_y.min(iy);
                    max_y = max_y.max(iy + height);
                }
                FrameItem::Image { height, .. } => {
                    min_y = min_y.min(iy);
                    max_y = max_y.max(iy + height.0);
                }
                FrameItem::TextReplacement { .. }
                | FrameItem::TextSeparator { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::Line { .. }
                | FrameItem::Link { .. } => {}
            }
        }

        let orig_w_exact = max_x.max(orig_w);

        // **P1120** — extensões do frame do body em relação à baseline. O
        // `base_y` (baseline do sub-frame, subtraído acima) é o ascent; o
        // descent vem do fundo medido pelo sub-frame quando existe (equação
        // de bloco: extensões da fórmula), com recurso às arestas de fonte
        // dos items quando não há medição.
        let (frame_ascent, frame_descent) = match layouter.last_sub_frame_bottom {
            Some(bottom) => (base_y, (bottom - base_y).max(0.0)),
            None => (-min_y, max_y),
        };

        // **P1120** — pivô = centro geométrico do frame do body (origem por
        // omissão de `rotate`/`scale`/`skew`: `center + horizon`).
        let cx = orig_w_exact / 2.0;
        let cy = (frame_descent - frame_ascent) / 2.0;
        let tx = matrix.tx + (1.0 - matrix.a) * cx - matrix.c * cy;
        let ty = matrix.ty - matrix.b * cx + (1.0 - matrix.d) * cy;
        let final_matrix_exact = TransformMatrix {
            a: matrix.a,
            b: matrix.b,
            c: matrix.c,
            d: matrix.d,
            tx,
            ty,
        };

        // **P1120** — o frame transformado é inline e alinha pela baseline
        // (`reflow: false` → a caixa de layout é a do body por transformar).
        // Só reporta a extensão à linha quando o sub-frame mediu de facto o
        // frame (equação de bloco): aí `base_y` é a baseline e o fundo é o do
        // frame. Para os outros bodies (shapes, texto solto) a baseline do
        // frame ainda não é medida — reportar as arestas de fonte daria uma
        // extensão errada, pelo que se mantém o comportamento anterior (a
        // linha não cresce). Gap registado em `layout.md` §P1120.
        if layouter.last_sub_frame_bottom.is_some() {
            layouter.note_inline_extent(frame_ascent, frame_descent);
        }

        layouter.regions.current.current_line.push(FrameItem::Group {
            paint_root: false,
            pos,
            matrix: final_matrix_exact,
            clip_mask: None,
            inner_width: orig_w_exact,
            inner_height: orig_h,
            items: sub_items,
        });
        layouter.regions.current.cursor_x += Pt(orig_w_exact);
    }
}

fn layout_native<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    e: &TransformElem,
) {
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};
    use crate::entities::source_result::SourceDiagnostic;
    use crate::entities::span::Span;
    use crate::entities::value::Value;
    let font = l.style.size.0;
    let had_line =
        super::html_frame::items_have_material(&l.regions.current.current_line);
    l.flush_line();
    let spacing = |field: &str| match l.chain.custom(&format!("block.{field}")) {
        Some(Value::Length(v)) => Some(v.resolve_pt(font)),
        _ => None,
    };
    let above = spacing("above")
        .or_else(|| spacing("spacing"))
        .unwrap_or(font * super::vanilla_defaults::BLOCK_SPACING);
    let below = spacing("below")
        .or_else(|| spacing("spacing"))
        .unwrap_or(font * super::vanilla_defaults::BLOCK_SPACING);
    let mut top = if had_line {
        l.flow_placement_end.map_or(l.prev_line_baseline, |(end, _)| end) + above
    } else if l.block_chain_active {
        l.prev_line_baseline
            + l.prev_block_equation_descent
            + l.prev_block_below_pending.max(above)
    } else {
        l.regions.current.cursor_y.0
    };
    let available_w = l.block_place_region.map_or_else(|| l.available_width(), |r| r.w.0);
    let outer_region = l.relative_region_size.unwrap_or((
        available_w,
        l.regions.current.height - l.page_config.margin.top - l.page_config.margin.bottom,
    ));
    let region_size = l.block_place_region.map_or(outer_region, |r| {
        (r.w.0, if r.h.0.is_finite() { r.h.0 } else { outer_region.1 })
    });
    let region_h = region_size.1;
    let name = match e.operation {
        TransformOperation::Matrix(_) => "transform",
        TransformOperation::Move { .. } => "move",
        TransformOperation::Rotate { .. } => "rotate",
        TransformOperation::Scale { .. } => "scale",
        TransformOperation::Skew { .. } => "skew",
    };
    let chain = l.chain.clone();
    let style = |field: &str| chain.custom(&format!("{name}.{field}"));
    let styled_angle = |field: &str| match style(field) {
        Some(Value::Angle(a)) => Some(*a),
        _ => None,
    };
    let styled_scale = |field: &str| match style(field) {
        Some(Value::Auto) => Some(TransformScale::Auto),
        Some(Value::Length(v)) => Some(TransformScale::Length(*v)),
        Some(Value::Ratio(v)) => Some(TransformScale::Ratio(*v)),
        _ => None,
    };
    let styled_relative = |field: &str| match style(field) {
        Some(Value::Relative(v)) => Some(*v),
        Some(Value::Length(v)) => Some(crate::entities::rel::Rel { rel: 0.0, abs: *v }),
        Some(Value::Ratio(v)) => Some(crate::entities::rel::Rel {
            rel: v.0,
            abs: crate::entities::layout_types::Length::ZERO,
        }),
        _ => None,
    };
    // Absolute scale is relative to the original logical frame, before reflow.
    let scale_basis = if let TransformOperation::Scale { x, y, .. } = e.operation {
        let x = x.or_else(|| styled_scale("x"));
        let y = y.or_else(|| styled_scale("y"));
        if matches!(x, Some(TransformScale::Length(_)))
            || matches!(y, Some(TransformScale::Length(_)))
        {
            let Some(size) = measure_native_body(l, &e.body, available_w, None) else {
                return;
            };
            size
        } else {
            (1.0, 1.0)
        }
    } else {
        (1.0, 1.0)
    };
    let mut preserve_baseline = true;
    let (mut matrix, origin, reflow) = match e.operation {
        TransformOperation::Matrix(_) => unreachable!("matrix dispatched separately"),
        TransformOperation::Move { dx, dy } => {
            let resolve = |value: Option<
                crate::entities::rel::Rel<crate::entities::layout_types::Length>,
            >,
                           extent| {
                value.map_or(0.0, |v| {
                    (if v.rel == 0.0 { 0.0 } else { v.rel * extent })
                        + v.abs.resolve_pt(font)
                })
            };
            (
                TransformMatrix::translate(
                    resolve(dx.or_else(|| styled_relative("dx")), region_size.0),
                    resolve(dy.or_else(|| styled_relative("dy")), region_h),
                ),
                None,
                Some(false),
            )
        }
        TransformOperation::Rotate { angle, origin, reflow } => {
            preserve_baseline = false;
            (
                TransformMatrix::rotate(
                    angle.or_else(|| styled_angle("angle")).map_or(0.0, |a| a.to_rad()),
                ),
                origin,
                reflow,
            )
        }
        TransformOperation::Skew { ax, ay, origin, reflow } => (
            TransformMatrix::skew(
                ax.or_else(|| styled_angle("ax")).map_or(0.0, |a| a.to_rad()),
                ay.or_else(|| styled_angle("ay")).map_or(0.0, |a| a.to_rad()),
            ),
            origin,
            reflow,
        ),
        TransformOperation::Scale { x, y, origin, reflow } => {
            let factor = |value: Option<TransformScale>, size| match value {
                None => Some(1.0),
                Some(TransformScale::Auto) => None,
                Some(TransformScale::Ratio(r)) => Some(r.0),
                Some(TransformScale::Length(v)) => Some(v.resolve_pt(font) / size),
            };
            let sx = factor(x.or_else(|| styled_scale("x")), scale_basis.0);
            let sy = factor(y.or_else(|| styled_scale("y")), scale_basis.1);
            let (sx, sy) = match (sx, sy) {
                (Some(x), Some(y)) => (x, y),
                (Some(x), None) => (x, x),
                (None, Some(y)) => (y, y),
                (None, None) => {
                    l.layout_errors.push(SourceDiagnostic::error(
                        Span::detached(),
                        "x and y cannot both be auto",
                    ));
                    return;
                }
            };
            preserve_baseline = sy >= 0.0;
            (TransformMatrix::scale(sx, sy), origin, reflow)
        }
    };
    let reflow = reflow.unwrap_or(matches!(style("reflow"), Some(Value::Bool(true))));
    let measured = if reflow {
        let size = match e.operation {
            TransformOperation::Scale { .. } => {
                let axis = |extent: f64, scale: f64| {
                    if extent.is_finite() {
                        (extent / scale).abs()
                    } else {
                        extent
                    }
                };
                (axis(region_size.0, matrix.a), axis(region_size.1, matrix.d))
            }
            TransformOperation::Rotate { .. } | TransformOperation::Skew { .. } => {
                if region_size.0.is_finite() && region_size.1.is_finite() {
                    // Rotation uses its inverse; shear uses the forward bbox.
                    // Both have the same absolute coefficients as the paint matrix.
                    (
                        matrix.a.abs() * region_size.0 + matrix.c.abs() * region_size.1,
                        matrix.b.abs() * region_size.0 + matrix.d.abs() * region_size.1,
                    )
                } else {
                    (f64::INFINITY, f64::INFINITY)
                }
            }
            TransformOperation::Move { .. } | TransformOperation::Matrix(_) => {
                region_size
            }
        };
        let Some(measured) = measure_native_body(l, &e.body, size.0, Some(size.1)) else {
            return;
        };
        Some(measured)
    } else {
        None
    };
    let body = compose_native_body(
        l,
        &e.body,
        measured.map_or(available_w, |size| size.0),
        measured.map(|size| size.1),
    );
    let (w, h) = measured.unwrap_or((body.width, body.height));
    let body_baseline = body.baseline;
    let NativeBody { mut items, deco, align_x, align_y, .. } = body;
    let saved_x = std::mem::replace(&mut l.pending_align_centering, align_x);
    l.apply_pending_align_fixups(&mut items, w + l.page_config.margin.horizontal());
    l.pending_align_centering = saved_x;
    let saved_y = std::mem::replace(&mut l.pending_align_v_centering, align_y);
    l.apply_pending_align_v_fixups(&mut items, h + l.page_config.margin.bottom);
    l.pending_align_v_centering = saved_y;
    let inherited = match style("origin") {
        Some(Value::Align(a)) => *a,
        _ => Align2D::default(),
    };
    let explicit = origin.unwrap_or_default();
    let rtl = l.style.dir == Some(crate::entities::dir::Dir::RTL);
    let px = match explicit.h.or(inherited.h).unwrap_or(HAlign::Center) {
        HAlign::Left => 0.0,
        HAlign::Center => w / 2.0,
        HAlign::Right => w,
        HAlign::Start => {
            if rtl {
                w
            } else {
                0.0
            }
        }
        HAlign::End => {
            if rtl {
                0.0
            } else {
                w
            }
        }
    };
    let py = match explicit.v.or(inherited.v).unwrap_or(VAlign::Horizon) {
        VAlign::Top => 0.0,
        VAlign::Horizon => h / 2.0,
        VAlign::Bottom => h,
    };
    matrix = TransformMatrix::translate(px, py)
        .concat(&matrix)
        .concat(&TransformMatrix::translate(-px, -py));
    let (frame_w, frame_h, baseline) = if reflow {
        let points = [
            matrix.apply(0.0, 0.0),
            matrix.apply(w, 0.0),
            matrix.apply(0.0, h),
            matrix.apply(w, h),
        ];
        let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
        let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
        matrix = TransformMatrix::translate(-min_x, -min_y).concat(&matrix);
        let height = max_y - min_y;
        (
            max_x - min_x,
            height,
            if preserve_baseline {
                matrix.apply(w / 2.0, body_baseline).1
            } else {
                height
            },
        )
    } else {
        (w, h, body_baseline)
    };
    if !l.is_sub_frame && top + frame_h > l.page_bottom_limit() {
        l.new_page();
        top = l.page_config.margin.top;
    }
    let pos = Point { x: l.regions.current.line_start_x, y: Pt(top) };
    if let Some(collector) = l.decoration_lines_collector.as_mut() {
        collector.extend(deco.into_iter().map(|seg| super::DecoSegment {
            start_x: seg.start_x + pos.x,
            end_x: seg.end_x + pos.x,
            baseline_y: seg.baseline_y + pos.y,
        }));
    }
    l.regions.current.current_items.push(FrameItem::Group {
        paint_root: false,
        pos,
        matrix,
        clip_mask: None,
        inner_width: w,
        inner_height: h,
        items,
    });
    let end = top + frame_h;
    l.regions.current.cursor_x = l.regions.current.line_start_x;
    l.regions.current.cursor_y = Pt(end + below);
    l.initial_baseline_pending = false;
    l.prev_line_baseline = end;
    l.prev_block_equation_descent = 0.0;
    l.prev_block_below_pending = below;
    l.prev_margin_is_parbreak = false;
    l.block_chain_active = true;
    l.last_block_descent_y = Some(end);
    l.record_flow_placement_end(end);
    l.record_flow_width_at(pos.x.0, frame_w);
    // The frame's baseline remains meaningful for a containing inline box.
    l.last_sub_frame_inline_extents = (baseline, frame_h - baseline);
}

struct NativeBody {
    width: f64,
    height: f64,
    baseline: f64,
    items: Vec<FrameItem>,
    deco: Vec<super::DecoSegment>,
    align_x: Vec<super::PendingAlignXEntry>,
    align_y: Vec<super::PendingAlignYEntry>,
}

fn compose_native_body<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    content: &crate::entities::content::Content,
    width: f64,
    height: Option<f64>,
) -> NativeBody {
    let (body_h, items, deco, align_x, align_y) = l.layout_sub_frame_at_geometric_origin(
        content,
        SubLayoutRegion {
            origin_x: 0.0,
            width,
            height,
            align_rtl: false,
            unconstrained_height: height.is_none(),
        },
    );
    let width = l.last_sub_frame_flow_width.min(width);
    let height = body_h.max(l.last_sub_frame_bottom.unwrap_or(0.0));
    let baseline = if l.last_sub_frame_inline_extents.0 > 0.0 {
        l.last_sub_frame_inline_extents.0
    } else {
        first_baseline(&items).unwrap_or(height)
    };
    NativeBody {
        width,
        height,
        baseline,
        items,
        deco,
        align_x,
        align_y,
    }
}

fn measure_native_body<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    content: &crate::entities::content::Content,
    width: f64,
    height: Option<f64>,
) -> Option<(f64, f64)> {
    let checkpoint = l.capture_flow_suffix_checkpoint();
    let errors_start = l.layout_errors.len();
    let body = compose_native_body(l, content, width, height);
    let errors = l.layout_errors[errors_start..].to_vec();
    l.restore_flow_suffix_checkpoint(checkpoint);
    if errors.is_empty() {
        Some((body.width, body.height))
    } else {
        l.layout_errors.extend(errors);
        None
    }
}

fn first_baseline(items: &[FrameItem]) -> Option<f64> {
    items.iter().find_map(|item| match item {
        FrameItem::Text { pos, .. }
        | FrameItem::TextShaped { pos, .. }
        | FrameItem::Glyph { pos, .. } => Some(pos.y.0),
        FrameItem::TextReplacement { items, .. }
        | FrameItem::Semantic { items, .. }
        | FrameItem::Link { items, .. } => first_baseline(items),
        FrameItem::Group { pos, matrix, items, .. } => {
            first_baseline(items).map(|y| pos.y.0 + matrix.apply(0.0, y).1)
        }
        FrameItem::TextSeparator { .. }
        | FrameItem::Line { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Image { .. } => None,
    })
}

pub(super) fn terminal_native(content: &crate::entities::content::Content) -> bool {
    use crate::entities::content::Content;
    match content {
        Content::Transform(e) => !matches!(e.operation, TransformOperation::Matrix(_)),
        Content::Source(e) => terminal_native(&e.body),
        Content::Styled(e) => terminal_native(&e.body),
        Content::ShowRecipe(e) => terminal_native(&e.body),
        Content::Located(e) => terminal_native(&e.body),
        Content::Realized(e) => terminal_native(&e.visual),
        Content::Sequence(parts) => parts
            .iter()
            .rev()
            .find(|c| {
                !matches!(
                    c,
                    Content::Space
                        | Content::SpaceFromNewline
                        | Content::Empty
                        | Content::Parbreak
                )
            })
            .is_some_and(terminal_native),
        Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::Equation(_)
        | Content::MathSequence(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathDelimited(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathMatrix(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => false,
    }
}
