//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/layout/block.md
//! @prompt-hash deaf0d8a
//! @layer L1
//! @updated 2026-09-01
//!
//! Atomização (ADR-0109, P376): o layout de `Block` movido do monólito
//! `layout_content` para o arquivo da feature. Content-preserving — a lógica
//! é idêntica; só o endereço mudou. O `match` no núcleo delega numa linha.

use crate::entities::dir::Dir;
use crate::entities::elements::block::{BlockElem, BlockHeight};
use crate::entities::layout_types::{FrameItem, Length, Pt};
use crate::entities::rel::Rel;
use crate::entities::value::Value;

use super::{FontMetrics, ImageSizer, Layouter};

#[derive(Clone, Copy)]
pub(super) struct InlineRegion {
    offset: f64,
    width: f64,
}

/// Reinstall the active body's horizontal geometry in the current region.
pub(super) fn install_inline_region<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
) {
    let natural_width = if l.column_mode { l.column_width } else { l.page_config.width };
    let (offset, width) = l
        .block_inline_region
        .map(|r| (r.offset, r.width))
        .unwrap_or((0.0, natural_width - l.page_config.margin.horizontal()));
    let start = l.page_config.margin.left + offset;
    l.regions.current.line_start_x = Pt(start);
    l.regions.current.cursor_x = Pt(start);
    l.regions.current.width = start + width + l.page_config.margin.right;
}

/// Resolve altura relativa contra a região exterior, sem multiplicar
/// `infinity * 0` no caso absoluto de páginas com altura automática.
pub(super) fn resolve_height_pt(
    height: Rel<Length>,
    font: f64,
    outer_height: f64,
) -> Option<f64> {
    let relative = if height.rel == 0.0 { 0.0 } else { outer_height * height.rel };
    let resolved = relative + height.abs.resolve_pt(font);
    (resolved.is_finite() && resolved >= 0.0).then_some(resolved)
}

/// Resolve largura relativa uma única vez contra a região exterior útil.
pub(super) fn resolve_width_pt(
    width: Rel<Length>,
    font: f64,
    outer_width: f64,
) -> Option<f64> {
    let relative = if width.rel == 0.0 { 0.0 } else { outer_width * width.rel };
    let resolved = relative + width.abs.resolve_pt(font);
    (resolved.is_finite() && resolved >= 0.0).then_some(resolved)
}

/// Resolve causal margin priority before comparing their magnitudes.
pub(super) fn collapse_margin(
    previous: f64,
    previous_is_auto: bool,
    incoming: f64,
    incoming_is_auto: bool,
) -> f64 {
    if previous_is_auto == incoming_is_auto {
        previous.max(incoming)
    } else if previous_is_auto {
        incoming
    } else {
        previous
    }
}

/// Layout do container `Block` (P156G/P242/P243/P247/P248/P250/P252/P273.6).
pub(super) fn layout<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>,
    e: &BlockElem,
) {
    let (
        body,
        width,
        height,
        inset,
        breakable,
        outset,
        radius,
        clip,
        fill,
        stroke,
        spacing,
        above,
        below,
    ) = (
        &e.body,
        &e.width,
        &e.height,
        &e.inset,
        &e.breakable,
        &e.outset,
        &e.radius,
        &e.clip,
        &e.fill,
        &e.stroke,
        &e.spacing,
        &e.above,
        &e.below,
    );
    let font = layouter.style.size.val();
    let outer_width = layouter
        .block_place_region
        .map_or_else(|| layouter.available_width(), |r| r.w.0);
    let outer_height = layouter
        .block_place_region
        .filter(|r| r.h.0.is_finite())
        .map(|r| r.h.0)
        .or(layouter.relative_region_size.map(|size| size.1))
        .unwrap_or_else(|| layouter.available_height());
    let resolved_width =
        (*width).and_then(|value| resolve_width_pt(value, font, outer_width));
    let resolved_height = match *height {
        Some(BlockHeight::Relative(value)) => {
            resolve_height_pt(value, font, outer_height)
        }
        Some(BlockHeight::Fractional(_)) => layouter.fractional_block_height,
        None => None,
    };
    let inset_left = inset.left.resolve_pt(font);
    let inset_top = inset.top.resolve_pt(font);
    let inset_bottom = inset.bottom.resolve_pt(font);
    let inset_right = inset.right.resolve_pt(font);

    // P247 — outset semantic real activação (cenário A audit:
    // outset zero-uso pré-P247). Margem externa visual que
    // expande bounds Shape mas NÃO afecta cursor interno do
    // body (paralelo a margin CSS).
    let outset_left = outset.left.resolve_pt(font);
    let outset_right = outset.right.resolve_pt(font);
    let outset_top = outset.top.resolve_pt(font);
    let outset_bottom = outset.bottom.resolve_pt(font);
    let has_shape = fill.is_some() || stroke.is_some();
    let has_outset = outset_left != 0.0
        || outset_right != 0.0
        || outset_top != 0.0
        || outset_bottom != 0.0;

    let had_line =
        super::html_frame::items_have_causal_line(&layouter.regions.current.current_line);
    if had_line
        || layouter.regions.current.cursor_x.0 > layouter.regions.current.line_start_x.0
    {
        layouter.flush_line();
    }

    // P250 — spacing/above collapse (paridade vanilla CSS margin
    // collapse `max(prev.below, curr.above)` entre Blocks
    // consecutivos; `above` suprimido no primeiro Block dum
    // Sequence — sinalizado via `block_chain_active == false`).
    let paragraph_spacing = match layouter.chain.custom("par.spacing") {
        Some(Value::Length(length)) => length.resolve_pt(font),
        _ => font * super::vanilla_defaults::PAR_SPACING,
    };
    let has_custom_above = above.is_some() || spacing.is_some();
    let above_pt = above
        .or(*spacing)
        .map(|l| l.resolve_pt(font))
        .unwrap_or(paragraph_spacing);
    if had_line {
        let end = layouter.flow_placement_y().unwrap_or(layouter.prev_line_baseline);
        layouter.regions.current.cursor_y = Pt(end + above_pt);
    } else if layouter.block_chain_active {
        let gap = collapse_margin(
            layouter.prev_block_below_pending,
            layouter.prev_margin_is_parbreak,
            above_pt,
            !has_custom_above,
        );
        let prev_descent = layouter.prev_block_equation_descent;
        layouter.regions.current.cursor_y =
            Pt(layouter.prev_line_baseline + prev_descent + gap);
        layouter.initial_baseline_pending = true;
    } else if !layouter.initial_baseline_pending {
        // A closed line still owns its causal end. Otherwise a region or
        // fractional prefix has already prepared the next text baseline;
        // recover the geometric origin before the body installs its own.
        if let Some(end) = layouter
            .flow_placement_y()
            .filter(|_| layouter.fractional_block_height.is_none())
        {
            layouter.regions.current.cursor_y = Pt(end + above_pt);
        } else if !layouter.is_sub_frame || layouter.fractional_block_height.is_some() {
            let (top, _) =
                layouter.metrics.text_edges(layouter.style.size, &layouter.style);
            layouter.regions.current.cursor_y = layouter.regions.current.cursor_y - top;
        }
    }
    layouter.prev_block_below_pending = 0.0;

    // P248 (M9d / M7+5; ADR-0079 Categoria A.4 cumulativa) —
    // Block.breakable semantic real activação: medição
    // antecipada via `measure_content_constrained` (puro;
    // §2.4 audit). Quando `breakable: false` E o bloco não
    // cabe no espaço restante mas cabe numa página inteira,
    // `new_page()` antecipa o break. Quando o bloco excede a
    // página inteira, emit normal (paridade vanilla
    // "overlong atómico"). Default `breakable: true` preserva
    // comportamento P156G literal.
    if !*breakable {
        let avail_w =
            (resolved_width.unwrap_or(outer_width) - inset_left - inset_right).max(0.0);
        let (_, body_h) = layouter.measure_content_constrained(body, avail_w);
        let block_total_h = resolved_height.unwrap_or(inset_top + body_h + inset_bottom);
        let page_usable_h = layouter.available_height();
        let remaining_h =
            layouter.page_bottom_limit() - layouter.regions.current.cursor_y.0;
        if block_total_h <= page_usable_h && block_total_h > remaining_h {
            // Cabe em página nova mas não na actual → break antecipado.
            layouter.new_page();
            if !layouter.initial_baseline_pending {
                let (top, _) =
                    layouter.metrics.text_edges(layouter.style.size, &layouter.style);
                layouter.regions.current.cursor_y =
                    layouter.regions.current.cursor_y - top;
            }
        }
        // else: cabe na actual OU overlong (emit normal — paridade vanilla).
    }

    // P247 — snapshot items_before para inserir Shape ANTES
    // do body (Z-order: fill+stroke atrás do conteúdo).
    let items_before = layouter.regions.current.current_items.len();
    let page_before = layouter.pages.len();
    let column_before = layouter.current_column;
    let deco_before = layouter.decoration_lines_collector.as_ref().map_or(0, Vec::len);

    // Capture the physical frame origin before laying out its body.
    let start_y = layouter.regions.current.cursor_y.0;
    layouter.flow_placement_end = None;
    // The body owns a fresh flow origin, independent of the parent's gap.
    layouter.block_chain_active = false;
    layouter.prev_margin_is_parbreak = false;
    layouter.prev_block_equation_descent = 0.0;
    layouter.last_equation_descent = 0.0;
    layouter.last_block_descent_y = None;
    layouter.last_flush_advance = 0.0;
    layouter.initial_baseline_pending = true;

    // 3. Aplica inset top.
    layouter.regions.current.cursor_y += Pt(inset_top);

    // 4. Aplica inset left (e width se especificado).
    let saved_line_start = layouter.regions.current.line_start_x;
    let saved_width = layouter.regions.current.width;
    let saved_height = layouter.regions.current.height;
    let is_rtl = layouter.chain.custom("text.dir").and_then(|value| match value {
        Value::Dir(dir) => Some(*dir),
        _ => None,
    }) == Some(Dir::RTL);
    let container_alignment = layouter.explicit_line_alignment.unwrap_or(if is_rtl {
        crate::entities::layout_types::HAlign::Right
    } else {
        crate::entities::layout_types::HAlign::Left
    });
    let block_origin_x = if let Some(w_pt) = resolved_width {
        let outer_width = w_pt;
        match container_alignment {
            crate::entities::layout_types::HAlign::Right => {
                Pt(saved_width - layouter.page_config.margin.right - outer_width)
            }
            crate::entities::layout_types::HAlign::End => {
                Pt(saved_width - layouter.page_config.margin.right - outer_width)
            }
            crate::entities::layout_types::HAlign::Center => {
                saved_line_start + Pt((layouter.available_width() - outer_width) / 2.0)
            }
            crate::entities::layout_types::HAlign::Left => saved_line_start,
            crate::entities::layout_types::HAlign::Start => saved_line_start,
        }
    } else {
        saved_line_start
    };
    layouter.regions.current.line_start_x = block_origin_x + Pt(inset_left);
    layouter.regions.current.cursor_x = layouter.regions.current.line_start_x;

    // P243 (M9d / M7+3 fase (a); ADR-0081 IMPLEMENTADO parcial 4/5)
    // — promoção real `Block.width`: quando `Some(w)`,
    // clampa `regions.current.width` ao valor user-provided
    // durante body layout via save/restore. `width: None`
    // preserva largura herdada.
    let outer_w = resolved_width
        .unwrap_or(saved_width - saved_line_start.0 - layouter.page_config.margin.right);
    let inner_w = (outer_w - inset_left - inset_right).max(0.0);
    layouter.regions.current.width = layouter.regions.current.line_start_x.0
        + inner_w
        + layouter.page_config.margin.right;
    // P1292 v12 — uma altura explícita também é a região efetiva dos
    // descendentes durante o body. Como o Block preserva coordenadas
    // absolutas no Layouter, o limite local é expresso a partir de `start_y`;
    // o outset superior fica fora da altura interna declarada. O limite
    // externo é restaurado logo após o body, como já ocorre com a largura.
    if let Some(h_pt) = resolved_height {
        // Region limits include the physical bottom margin, which consumers
        // subtract when finding the usable bottom. Do not subtract it twice.
        layouter.regions.current.height =
            saved_height.min(start_y + h_pt + layouter.page_config.margin.bottom);
    }

    // P242 (M9d / M7+5) — clip=true emite FrameItem::Group com
    // clip_mask `RoundedRect{radii}` (radius non-zero) ou `Rect`
    // (radius zero; paridade DEBT-30 P79). Layouter restringe
    // body output em mask. Quando clip=false, comportamento
    // inline original preservado (semantic adiada graded mas
    // estrutura inline mantida).
    let radius_is_zero = radius.top_left == crate::entities::layout_types::Length::ZERO
        && radius.top_right == crate::entities::layout_types::Length::ZERO
        && radius.bottom_right == crate::entities::layout_types::Length::ZERO
        && radius.bottom_left == crate::entities::layout_types::Length::ZERO;

    let inner_origin_y = layouter.regions.current.cursor_y.0;
    let saved_block_region =
        layouter
            .block_place_region
            .replace(crate::entities::layout_types::Rect {
                x: layouter.regions.current.line_start_x,
                y: Pt(inner_origin_y),
                w: Pt(inner_w),
                h: Pt(resolved_height
                    .map_or(f64::INFINITY, |h| (h - inset_top - inset_bottom).max(0.0))),
            });
    let saved_block_decorations = std::mem::take(&mut layouter.block_place_decorations);
    let pending_y_before = layouter.pending_align_v_centering.len();
    let pending_x_before = layouter.pending_align_centering.len();
    let saved_parent_bbox = layouter.parent_bbox;
    if let (Some(w_pt), Some(h_pt)) = (resolved_width, resolved_height) {
        layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
            x: block_origin_x,
            y: Pt(start_y),
            w: Pt(w_pt),
            h: Pt(h_pt),
        });
    }
    let saved_flow_extent = std::mem::replace(
        &mut layouter.flow_frame_extent,
        (layouter.regions.current.line_start_x.0, 0.0),
    );
    let saved_block_auto_width =
        std::mem::replace(&mut layouter.block_auto_width, resolved_width.is_none());
    let saved_relative_region = layouter.relative_region_size.replace((
        inner_w,
        (resolved_height.unwrap_or(outer_height) - inset_top - inset_bottom).max(0.0),
    ));
    let previous_inline_region = layouter.block_inline_region.replace(InlineRegion {
        offset: layouter.regions.current.line_start_x.0
            - layouter.page_config.margin.left,
        width: inner_w,
    });
    let previous_indent = super::paragraph_indent::enter_fragment(layouter, body);
    let previous_planning =
        std::mem::replace(&mut layouter.paragraph_planning_suppressed, false);
    layouter.layout_content(body);
    layouter.flush_line();
    layouter.paragraph_planning_suppressed = previous_planning;
    layouter.paragraph_indent = previous_indent;
    layouter.block_inline_region = previous_inline_region;
    layouter.relative_region_size = saved_relative_region;
    layouter.block_auto_width = saved_block_auto_width;
    let body_width = layouter.flow_frame_extent.1;
    layouter.flow_frame_extent = saved_flow_extent;
    layouter.parent_bbox = saved_parent_bbox;
    let same_region =
        layouter.pages.len() == page_before && layouter.current_column == column_before;
    let allocated_w = resolved_width.unwrap_or_else(|| {
        if same_region {
            (inset_left + body_width + inset_right).max(0.0)
        } else {
            outer_w
        }
    });
    let root_height = same_region.then(|| {
        let causal_h = (layouter.flow_placement_y().unwrap_or(inner_origin_y)
            - inner_origin_y)
            .max(0.0);
        resolved_height.unwrap_or(causal_h + inset_top + inset_bottom)
    });
    if let Some(allocated_h) = root_height {
        // Resolve local alignment before introducing clipping/root coordinates.
        // The body was laid out once under the available wrapping constraint.
        let inner_right = block_origin_x.0 + allocated_w - inset_right;
        let pending = layouter.pending_align_centering.split_off(pending_x_before);
        for (path, count, align, content_w, origin_x, applied_x) in pending {
            let available = (inner_right - origin_x).max(0.0);
            let (target, _) = layouter
                .resolve_alignment(align, content_w, 0.0, available, 0.0, origin_x, 0.0);
            if let Some(items) = super::helpers::resolve_path_slice(
                &mut layouter.regions.current.current_items,
                &path,
                count,
            ) {
                for item in items {
                    super::helpers::shift_frame_item_x(item, target - applied_x);
                }
            }
        }
        let inner_h = (allocated_h - inset_top - inset_bottom).max(0.0);
        let pending = layouter.pending_align_v_centering.split_off(pending_y_before);
        for (index, (path, count, align, content_h, origin_y, dy, applied_y)) in
            pending.into_iter().enumerate()
        {
            let available = (inner_origin_y + inner_h - origin_y).max(0.0);
            let (_, target) = layouter
                .resolve_alignment(align, 0.0, content_h, 0.0, available, 0.0, origin_y);
            let delta = target + dy - applied_y;
            if let Some(items) = super::helpers::resolve_path_slice(
                &mut layouter.regions.current.current_items,
                &path,
                count,
            ) {
                for item in items {
                    super::helpers::shift_frame_item_y(item, delta);
                }
            }
            if let Some(segments) = layouter.decoration_lines_collector.as_mut() {
                for &(entry, first, end) in &layouter.block_place_decorations {
                    if entry == pending_y_before + index {
                        for segment in &mut segments[first..end] {
                            segment.baseline_y += Pt(delta);
                        }
                    }
                }
            }
        }
        if *clip {
            let pos = crate::entities::layout_types::Point {
                x: block_origin_x - Pt(outset_left),
                y: Pt(start_y - outset_top),
            };
            let mut body_items: Vec<FrameItem> =
                layouter.regions.current.current_items.drain(items_before..).collect();
            for item in &mut body_items {
                super::helpers::shift_frame_item_x(item, -pos.x.0);
                super::helpers::shift_frame_item_y(item, -pos.y.0);
            }
            for entry in &mut layouter.pending_align_centering[pending_x_before..] {
                entry.0[0] -= items_before;
                entry.0.insert(0, items_before);
            }
            let clip_shape = if radius_is_zero {
                crate::entities::geometry::ShapeKind::Rect
            } else {
                super::resolve_shape_kind(
                    &crate::entities::geometry::ShapeKind::RoundedRect { radii: *radius },
                    layouter.style.size,
                )
            };
            layouter.regions.current.current_items.push(FrameItem::Group {
                paint_root: false,
                pos,
                matrix: crate::entities::layout_types::TransformMatrix::identity(),
                clip_mask: Some(clip_shape),
                inner_width: allocated_w + outset_left + outset_right,
                inner_height: allocated_h + outset_top + outset_bottom,
                items: body_items,
            });
        }
        layouter.block_place_region = saved_block_region;
        layouter.block_place_decorations = saved_block_decorations;
    }

    let mut root_origin_x = block_origin_x;
    // Auto-sized textual frames are positioned as a unit. Do not change
    // paragraph direction to obtain the container's outer alignment.
    if resolved_width.is_none()
        && is_rtl
        && layouter.explicit_line_alignment.is_none()
        && !has_shape
        && !has_outset
        && !*clip
        && *inset == crate::entities::sides::Sides::uniform(Length::ZERO)
        && layouter.pages.len() == page_before
        && saved_width.is_finite()
        && super::placement::text_alignment_body(body)
        && items_before < layouter.regions.current.current_items.len()
    {
        let refs: Vec<_> = layouter.regions.current.current_items[items_before..]
            .iter()
            .collect();
        let visual_right = layouter.metrics.line_content_right(&refs);
        let literal_right = refs
            .iter()
            .filter_map(|item| match item {
                FrameItem::TextSeparator {
                    pos,
                    advance,
                    origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                    ..
                } => Some(pos.x.0 + advance.0),
                _ => None,
            })
            .fold(0.0_f64, f64::max);
        let right = visual_right.max(literal_right);
        let delta = saved_width - layouter.page_config.margin.right - right;
        root_origin_x += Pt(delta);
        for item in &mut layouter.regions.current.current_items[items_before..] {
            super::helpers::shift_frame_item_x(item, delta);
        }
        if let Some(segments) = layouter.decoration_lines_collector.as_mut() {
            for segment in &mut segments[deco_before..] {
                segment.start_x += Pt(delta);
                segment.end_x += Pt(delta);
            }
        }
    }

    // Root geometry is the allocated container, including inset. Its own
    // backing is inserted later as a preceding sibling, outside this scope.
    if let Some(allocated_h) = root_height {
        let pos =
            crate::entities::layout_types::Point { x: root_origin_x, y: Pt(start_y) };
        let mut items: Vec<FrameItem> =
            layouter.regions.current.current_items.drain(items_before..).collect();
        for item in &mut items {
            super::helpers::shift_frame_item_x(item, -pos.x.0);
            super::helpers::shift_frame_item_y(item, -pos.y.0);
        }
        for entry in &mut layouter.pending_align_centering[pending_x_before..] {
            entry.0[0] -= items_before;
            entry.0.insert(0, items_before);
        }
        layouter.regions.current.current_items.push(FrameItem::Group {
            paint_root: true,
            pos,
            matrix: crate::entities::layout_types::TransformMatrix::identity(),
            clip_mask: None,
            inner_width: allocated_w,
            inner_height: allocated_h,
            items,
        });
    }

    // Descendentes do Block não podem fazer a região explícita vazar para o
    // flow exterior.
    if layouter.pages.len() == page_before && layouter.current_column == column_before {
        layouter.regions.current.height = saved_height;
    }

    let body_flow_end = layouter.flow_placement_y();

    if let Some(allocated_h) = root_height {
        layouter.regions.current.cursor_y = Pt(start_y + allocated_h);
    } else {
        // Fragmentation still uses the surviving region's existing protocol.
        layouter.regions.current.cursor_y += Pt(inset_bottom);
        if let Some(h_pt) = resolved_height {
            let consumed_inner = layouter.regions.current.cursor_y.0 - start_y;
            if consumed_inner < h_pt {
                layouter.regions.current.cursor_y += Pt(h_pt - consumed_inner);
            }
        }
    }

    // Paint expands the allocated frame by outset, without consuming flow.
    // Insert it before the body to preserve the backing's paint order.
    if has_shape || has_outset {
        let block_outer_w = allocated_w;
        let mut outer_w = block_outer_w + outset_left + outset_right;
        let mut outer_h = root_height
            .unwrap_or(layouter.regions.current.cursor_y.0 - start_y)
            + outset_top
            + outset_bottom;
        let mut pos = crate::entities::layout_types::Point {
            x: block_origin_x - Pt(outset_left),
            y: Pt(start_y - outset_top),
        };
        // P252 — stroke-overhang real activação Block.
        // Bounds Shape expandidos por thickness/2 em todos
        // os lados quando `stroke.overhang == true` (paridade
        // vanilla). Default Rust construtor `overhang: false`
        // preserva bounds literais (backward compat literal
        // pré-P252; sentinela p252).
        if let Some(ref s) = stroke {
            if s.overhang {
                // rationale: P1064 Classe 1B — semi-espessura de traço (thickness / 2.0)
                let ov = s.thickness / 2.0;
                pos.x = pos.x - Pt(ov);
                pos.y = pos.y - Pt(ov);
                outer_w += 2.0 * ov;
                outer_h += 2.0 * ov;
            }
        }
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
        for entry in &mut layouter.pending_align_centering[pending_x_before..] {
            entry.0[0] += 1;
        }
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
                // P273.6 — Block's own shape; gradient relative=parent
                // resolve para contentor outer (saved_parent_bbox foi
                // restaurado em parent_bbox antes desta emissão).
                fill_rule: Default::default(),
                parent_bbox_at_emit: layouter.parent_bbox,
            },
        );
    }

    // 8. Restaura line_start_x e width (P243).
    if same_region {
        layouter.regions.current.line_start_x = saved_line_start;
        layouter.regions.current.cursor_x = saved_line_start;
        layouter.regions.current.width = saved_width;
    } else {
        install_inline_region(layouter);
    }

    // P250/P1061 — below cursor.y advance + state update para
    // collapse com próximo Block/Parágrafo consecutivo.
    let is_geometric_container = height.is_some() || fill.is_some() || stroke.is_some();
    let below_pt = below
        .or(*spacing)
        .map(|length| length.resolve_pt(font))
        .unwrap_or(paragraph_spacing);

    if let Some(allocated_h) = root_height {
        let end = start_y + allocated_h;
        layouter.prev_line_baseline = end;
        layouter.last_block_descent_y = Some(end);
        layouter.prev_block_equation_descent = 0.0;
        layouter.prev_block_below_pending = below_pt;
        layouter.block_chain_active = true;
        layouter.initial_baseline_pending = false;
    } else if is_geometric_container {
        // P1292 v12 — o fim físico do frame é a fronteira lógica para a
        // unidade seguinte. O gap permanece pendente e colapsa pela mesma
        // regra de `above`/`below`; não é somado pelo Cursor nem pelo replay.
        let frame_end = layouter.regions.current.cursor_y.0;
        layouter.prev_line_baseline = frame_end;
        layouter.prev_block_below_pending = below_pt;
        layouter.block_chain_active = true;
    } else {
        let leading_pt = layouter
            .style
            .leading
            .map(|l| l.resolve_pt(font))
            .unwrap_or(font * super::vanilla_defaults::PAR_LEADING);
        let extra_below = below_pt - leading_pt;
        layouter.regions.current.cursor_y =
            Pt((layouter.regions.current.cursor_y.0 + extra_below).max(0.0));
        layouter.prev_block_below_pending = below_pt;
        layouter.block_chain_active = true;
    }
    layouter.prev_margin_is_parbreak = below.is_none() && spacing.is_none();
    let end = if let Some(allocated_h) = root_height {
        start_y + allocated_h
    } else {
        body_flow_end.unwrap_or(layouter.regions.current.cursor_y.0)
    };
    layouter.record_flow_placement_end(end);
    layouter.record_flow_width_at(block_origin_x.0, allocated_w);
}

impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    /// Record allocated horizontal flow, excluding visual-only overlays.
    pub(super) fn record_flow_width_at(&mut self, origin: f64, width: f64) {
        let extent = origin - self.flow_frame_extent.0 + width;
        if extent.is_finite() {
            self.flow_frame_extent.1 = self.flow_frame_extent.1.max(extent);
        }
    }
}

#[cfg(test)]
mod p1373_tests {
    use super::*;

    #[test]
    fn block_does_not_restore_a_previous_regions_float_reduced_height() {
        use crate::entities::{
            content::Content,
            image_sizer::NullImageSizer,
            introspector::{Introspector, TagIntrospector},
            layout_types::PageMargins,
            sides::Sides,
        };
        use comemo::Track;
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            11.0,
            (&intr as &dyn Introspector).track(),
        );
        l.page_config.width = 100.0;
        l.page_config.height = 100.0;
        l.page_config.margin = PageMargins::uniform(0.0);
        l.page_config.margin_spec =
            crate::entities::layout_types::PageMarginSpec::uniform(0.0);
        l.regions.current.height = 20.0;
        l.cursor_y_bottom_reserve = 80.0;
        let body = Content::sequence(vec![
            Content::pagebreak(false, None),
            Content::text("new region"),
        ]);
        let block = Content::block(body, None, None, Sides::uniform(Length::ZERO), true);
        l.layout_content(&block);
        assert_eq!(l.pages.len(), 1);
        assert_eq!(
            l.regions.current.height, 100.0,
            "the old float reservation must not leak into the new page"
        );
        assert_eq!(l.cursor_y_bottom_reserve, 0.0);
    }

    #[test]
    fn p1373_height_100_percent_resolves_against_usable_region() {
        let height = Rel::<Length>::from_percent(100.0);
        let resolved = resolve_height_pt(height, 11.0, 220.0)
            .expect("100% de região finita deve resolver");
        assert!((resolved - 220.0).abs() < 1e-9, "resolved={resolved}");
    }

    #[test]
    fn p1373_mixed_height_preserves_absolute_offset() {
        let height = Rel::<Length>::from_percent(50.0) + Length::pt(10.0);
        let resolved = resolve_height_pt(height, 11.0, 220.0)
            .expect("altura relativa mista válida deve resolver");
        assert!((resolved - 120.0).abs() < 1e-9, "resolved={resolved}");
    }
}

#[cfg(test)]
mod p1374_tests {
    use super::*;

    #[test]
    fn p1374_width_60_percent_resolves_against_usable_region() {
        let width = Rel::<Length>::from_percent(60.0);
        let resolved = resolve_width_pt(width, 11.0, 180.0)
            .expect("60% de região finita deve resolver");
        assert!((resolved - 108.0).abs() < 1e-9, "resolved={resolved}");
    }
}

#[cfg(test)]
mod literal_extent_regression {
    use super::*;
    use crate::entities::{
        content::Content,
        corners::Corners,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::TextStyle,
        sides::Sides,
        style::Styles,
    };
    use comemo::Track;
    #[test]
    fn rtl_block_places_literal_terminal_space_inside_its_frame() {
        for spaces in [0, 1, 2] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                11.0,
                (&intr as &dyn Introspector).track(),
            );
            l.chain = l.chain.push_custom("text.dir", Value::Dir(Dir::RTL));
            l.style = TextStyle::from(&l.chain);
            let right = l.regions.current.width - l.page_config.margin.right;
            let text = format!("ABC{}", " ".repeat(spaces));
            let expected = l.metrics.text_width(&text, l.style.size, &l.style).0;
            let b = BlockElem {
                body: Content::styled(
                    Content::text(text),
                    Styles::new().push_custom("text.dir", Value::Dir(Dir::LTR)),
                ),
                width: None,
                height: None,
                inset: Sides::uniform(Length::ZERO),
                breakable: true,
                outset: Sides::uniform(Length::ZERO),
                radius: Corners::uniform(Length::ZERO),
                clip: false,
                fill: None,
                stroke: None,
                spacing: None,
                above: None,
                below: None,
                sticky: false,
            };
            layout(&mut l, &b);
            let doc = l.finish();
            let x = super::super::tests::frame_items_recursive(&doc.pages[0].items)
                .into_iter()
                .find_map(|it| {
                    if let FrameItem::Text { pos, .. } = it {
                        Some(pos.x.0)
                    } else {
                        None
                    }
                })
                .unwrap();
            assert!(
                (x + expected - right).abs() < 1e-6,
                "spaces={spaces} x={x} expected={expected} right={right}"
            );
        }
    }
}

#[cfg(test)]
mod a2_inline_region_tests {
    use super::*;
    use crate::entities::sides::Sides;
    use crate::entities::{
        content::Content,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        layout_types::{FrameItem, PageMarginSpec, PageMargins},
    };
    use comemo::Track;

    #[test]
    fn a2_structure_block_insets_survive_page_progress_and_restore_parent() {
        for strategy in ["simple", "optimized"] {
            let intr = TagIntrospector::empty();
            let mut l = Layouter::new(
                super::super::FixedMetrics,
                NullImageSizer,
                12.0,
                (&intr as &dyn Introspector).track(),
            );
            l.page_config.width = 200.0;
            l.page_config.height = 80.0;
            l.page_config.margin = PageMargins::uniform(0.0);
            l.page_config.margin_spec = PageMarginSpec::uniform(0.0);
            l.regions.current.width = 200.0;
            l.regions.current.height = 80.0;
            l.regions.current.line_start_x = Pt(0.0);
            l.regions.current.cursor_x = Pt(0.0);
            l.regions.current.cursor_y = Pt(0.0);
            l.chain = l.chain.push_custom("par.linebreaks", Value::Str(strategy.into()));
            let body = Content::text("alpha beta gamma ".repeat(30));
            l.layout_content(&Content::block(
                body,
                None,
                None,
                Sides {
                    left: Length::pt(10.0),
                    right: Length::pt(20.0),
                    top: Length::ZERO,
                    bottom: Length::ZERO,
                },
                true,
            ));
            l.layout_content(&Content::text("OUTSIDE"));
            let doc = l.finish();
            assert!(doc.pages.len() > 1);
            let mut rows = vec![];
            for (page, frame) in doc.pages.iter().enumerate() {
                for item in
                    crate::compiler::layout::tests::frame_items_recursive(&frame.items)
                {
                    if let FrameItem::Text { text, pos, .. } = item {
                        rows.push((page, text.to_string(), pos.x.0));
                    }
                }
            }
            assert!(
                rows.iter()
                    .filter(|(_, text, _)| ["alpha", "beta", "gamma"]
                        .contains(&text.as_str()))
                    .all(|(_, _, x)| *x >= 10.0 - 1e-8),
                "{strategy}: {rows:?}"
            );
            assert_eq!(
                rows.iter().find(|(_, text, _)| text == "OUTSIDE").unwrap().2,
                0.0,
                "{strategy}"
            );
        }
    }
    #[test]
    fn a2_structure_block_owns_preparation_after_outer_flow_rejection() {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        let body = Content::sequence(vec![Content::text("xx 2020"), Content::text(".")]);
        let block = Content::block(
            body,
            Some(Length::pt(52.0)),
            None,
            Sides::uniform(Length::ZERO),
            true,
        );
        l.layout_content(&Content::sequence(vec![block, Content::text("OUTSIDE")]));
        let doc = l.finish();
        let rows: Vec<_> = doc
            .pages
            .iter()
            .flat_map(|page| {
                crate::compiler::layout::tests::frame_items_recursive(&page.items)
            })
            .filter_map(|item| match item {
                FrameItem::Text { text, pos, .. } => Some((text.to_string(), pos.y.0)),
                _ => None,
            })
            .collect();
        let year = rows.iter().find(|(text, _)| text.contains("2020")).unwrap();
        let period = rows.iter().find(|(text, _)| text.ends_with('.')).unwrap();
        assert_eq!(year.1, period.1, "{rows:?}");
    }
}

#[cfg(test)]
mod a2_margin_priority_tests {
    use super::*;
    use crate::entities::{
        content::Content,
        corners::Corners,
        image_sizer::NullImageSizer,
        introspector::{Introspector, TagIntrospector},
        sides::Sides,
    };
    use comemo::Track;
    use std::sync::Arc;

    fn block(text: &str) -> BlockElem {
        BlockElem {
            body: Content::text(text),
            width: None,
            height: None,
            inset: Sides::uniform(Length::ZERO),
            breakable: true,
            outset: Sides::uniform(Length::ZERO),
            radius: Corners::uniform(Length::ZERO),
            clip: false,
            fill: None,
            stroke: None,
            spacing: None,
            above: None,
            below: None,
            sticky: false,
        }
    }

    fn positions(
        below: Option<f64>,
        above: Option<f64>,
        ambient: f64,
        spacing: Option<f64>,
    ) -> (f64, f64) {
        let intr = TagIntrospector::empty();
        let mut l = Layouter::new(
            super::super::FixedMetrics,
            NullImageSizer,
            12.0,
            (&intr as &dyn Introspector).track(),
        );
        l.chain = l.chain.push_custom("par.spacing", Value::Length(Length::pt(ambient)));
        let mut first = block("BEFORE");
        first.below = below.map(Length::pt);
        first.spacing = spacing.map(Length::pt);
        let mut second = block("BODY");
        second.above = above.map(Length::pt);
        l.layout_content(&Content::Block(Arc::new(first)));
        l.layout_content(&Content::Block(Arc::new(second)));
        l.layout_content(&Content::text("OUTSIDE"));
        let doc = l.finish();
        let y = |word| {
            doc.pages
                .iter()
                .flat_map(|page| {
                    crate::compiler::layout::tests::frame_items_recursive(&page.items)
                })
                .find_map(|item| match item {
                    FrameItem::Text { text, pos, .. } if text.as_str() == word => {
                        Some(pos.y.0)
                    }
                    _ => None,
                })
                .expect("marker survives")
        };
        (y("BODY"), y("OUTSIDE"))
    }

    #[test]
    fn a2_margin_explicit_below_beats_automatic_above() {
        for ambient in [0.0, 5.0, 24.0] {
            for below in [0.0, 3.0, 30.0] {
                assert_eq!(
                    positions(Some(below), None, ambient, None),
                    positions(Some(below), Some(0.0), ambient, None),
                    "ambient={ambient}, explicit below={below}"
                );
            }
        }
    }

    #[test]
    fn a2_margin_explicit_above_beats_automatic_below() {
        for ambient in [0.0, 5.0, 24.0] {
            for above in [0.0, 3.0, 30.0] {
                assert_eq!(
                    positions(None, Some(above), ambient, None),
                    positions(Some(0.0), Some(above), ambient, None),
                    "ambient={ambient}, explicit above={above}"
                );
            }
        }
    }

    #[test]
    fn a2_margin_equal_priorities_keep_the_larger_gap() {
        for ambient in [0.0, 5.0, 24.0] {
            assert_eq!(
                positions(None, None, ambient, None),
                positions(Some(ambient), Some(ambient), ambient, None)
            );
            for (below, above) in [(3.0, 30.0), (30.0, 3.0), (0.0, 0.0)] {
                assert_eq!(
                    positions(Some(below), Some(above), ambient, None),
                    positions(Some(f64::max(below, above)), Some(0.0), ambient, None)
                );
            }
        }
    }

    #[test]
    fn a2_margin_explicit_spacing_is_not_automatic() {
        for ambient in [5.0, 24.0] {
            for spacing in [0.0, 3.0, 30.0] {
                assert_eq!(
                    positions(None, None, ambient, Some(spacing)),
                    positions(Some(spacing), Some(0.0), ambient, Some(spacing))
                );
                assert_eq!(
                    positions(Some(0.0), None, ambient, Some(spacing)),
                    positions(Some(0.0), Some(0.0), ambient, Some(spacing))
                );
            }
        }
    }
}
