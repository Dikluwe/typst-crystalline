from pathlib import Path
p=Path('01_core/src/compiler/layout/transform.rs');s=p.read_text().replace('use crate::entities::elements::transform::TransformElem;','use crate::entities::elements::transform::{TransformElem, TransformOperation, TransformScale};')
s=s.replace('''    let matrix = &e.matrix;
    let body = &e.body;''','''    if let TransformOperation::Matrix(matrix) = e.operation {
        layout_matrix(layouter, &e.body, &matrix);
    } else {
        layout_native(layouter, e);
    }
}

fn layout_matrix<M: FontMetrics, S: ImageSizer>(
    layouter: &mut Layouter<M, S>, body: &crate::entities::content::Content, matrix: &TransformMatrix,
) {''')
s+='''

fn layout_native<M: FontMetrics, S: ImageSizer>(l: &mut Layouter<M, S>, e: &TransformElem) {
    use crate::entities::layout_types::{Align2D, HAlign, VAlign};
    use crate::entities::value::Value;
    use crate::entities::source_result::SourceDiagnostic;
    use crate::entities::span::Span;
    let font = l.style.size.0;
    let available_w = l.available_width();
    let region_h = l.regions.current.height - l.page_config.margin.top - l.page_config.margin.bottom;
    let (body_h, mut items, deco, align_x, align_y) = l.layout_sub_frame_at_geometric_origin(
        &e.body, SubLayoutRegion { origin_x: 0.0, width: available_w, height: None,
            align_rtl: false, unconstrained_height: true });
    let w = l.last_sub_frame_width.max(l.last_sub_frame_flow_width)
        .max(super::helpers::line_content_right(&items, &l.metrics));
    let h = body_h.max(l.last_sub_frame_bottom.unwrap_or(0.0));
    let body_baseline = if l.last_sub_frame_inline_extents.0 > 0.0 {
        l.last_sub_frame_inline_extents.0
    } else { first_baseline(&items).unwrap_or(h) };
    // Resolve alignment against the measured frame, before the outer transform.
    let saved_x = std::mem::replace(&mut l.pending_align_centering, align_x);
    l.apply_pending_align_fixups(&mut items, w + l.page_config.margin.horizontal());
    l.pending_align_centering = saved_x;
    let saved_y = std::mem::replace(&mut l.pending_align_v_centering, align_y);
    l.apply_pending_align_v_fixups(&mut items, h + l.page_config.margin.bottom);
    l.pending_align_v_centering = saved_y;
    let name = crate::entities::content::Content::Transform(std::sync::Arc::new(e.clone())).elem_name();
    let style = |field: &str| l.chain.custom(&format!("{name}.{field}"));
    let styled_angle = |field: &str| match style(field) { Some(Value::Angle(a)) => Some(*a), _ => None };
    let styled_scale = |field: &str| match style(field) {
        Some(Value::Auto) => Some(TransformScale::Auto),
        Some(Value::Length(v)) => Some(TransformScale::Length(*v)),
        Some(Value::Ratio(v)) => Some(TransformScale::Ratio(*v)), _ => None,
    };
    let styled_relative = |field: &str| match style(field) {
        Some(Value::Relative(v)) => Some(*v),
        Some(Value::Length(v)) => Some(crate::entities::rel::Rel { rel: 0.0, abs: *v }),
        Some(Value::Ratio(v)) => Some(crate::entities::rel::Rel { rel: v.0, abs: crate::entities::layout_types::Length::ZERO }), _ => None,
    };
    let mut preserve_baseline = true;
    let (mut matrix, origin, reflow) = match e.operation {
        TransformOperation::Matrix(_) => unreachable!("matrix dispatched separately"),
        TransformOperation::Move { dx, dy } => {
            let resolve = |value: Option<crate::entities::rel::Rel<crate::entities::layout_types::Length>>, extent| value.map_or(0.0, |v| v.rel * extent + v.abs.resolve_pt(font));
            (TransformMatrix::translate(resolve(dx.or_else(|| styled_relative("dx")), available_w), resolve(dy.or_else(|| styled_relative("dy")), region_h)), None, Some(false))
        }
        TransformOperation::Rotate { angle, origin, reflow } => {
            preserve_baseline = false;
            (TransformMatrix::rotate(angle.or_else(|| styled_angle("angle")).map_or(0.0, |a| a.to_rad())), origin, reflow)
        }
        TransformOperation::Skew { ax, ay, origin, reflow } => (
            TransformMatrix::skew(ax.or_else(|| styled_angle("ax")).map_or(0.0, |a| a.to_rad()), ay.or_else(|| styled_angle("ay")).map_or(0.0, |a| a.to_rad())), origin, reflow),
        TransformOperation::Scale { x, y, origin, reflow } => {
            let factor = |value: Option<TransformScale>, size| match value {
                None => Some(1.0), Some(TransformScale::Auto) => None,
                Some(TransformScale::Ratio(r)) => Some(r.0),
                Some(TransformScale::Length(v)) => Some(v.resolve_pt(font) / size),
            };
            let sx = factor(x.or_else(|| styled_scale("x")), w);
            let sy = factor(y.or_else(|| styled_scale("y")), h);
            let (sx, sy) = match (sx, sy) {
                (Some(x), Some(y)) => (x, y), (Some(x), None) => (x, x), (None, Some(y)) => (y, y),
                (None, None) => { l.layout_errors.push(SourceDiagnostic::error(Span::detached(), "x and y cannot both be auto")); return; }
            };
            preserve_baseline = sy >= 0.0;
            (TransformMatrix::scale(sx, sy), origin, reflow)
        }
    };
    let inherited = match style("origin") { Some(Value::Align(a)) => *a, _ => Align2D::default() };
    let explicit = origin.unwrap_or_default();
    let rtl = l.style.dir == crate::entities::dir::Dir::Rtl;
    let px = match explicit.h.or(inherited.h).unwrap_or(HAlign::Center) {
        HAlign::Left => 0.0, HAlign::Center => w / 2.0, HAlign::Right => w,
        HAlign::Start => if rtl { w } else { 0.0 }, HAlign::End => if rtl { 0.0 } else { w },
    };
    let py = match explicit.v.or(inherited.v).unwrap_or(VAlign::Horizon) {
        VAlign::Top => 0.0, VAlign::Horizon => h / 2.0, VAlign::Bottom => h,
    };
    matrix = TransformMatrix::translate(px, py).concat(&matrix).concat(&TransformMatrix::translate(-px, -py));
    let reflow = reflow.unwrap_or(matches!(style("reflow"), Some(Value::Bool(true))));
    let (frame_w, frame_h, baseline) = if reflow {
        let points = [matrix.apply(0.0, 0.0), matrix.apply(w, 0.0), matrix.apply(0.0, h), matrix.apply(w, h)];
        let min_x = points.iter().map(|p| p.0).fold(f64::INFINITY, f64::min);
        let min_y = points.iter().map(|p| p.1).fold(f64::INFINITY, f64::min);
        let max_x = points.iter().map(|p| p.0).fold(f64::NEG_INFINITY, f64::max);
        let max_y = points.iter().map(|p| p.1).fold(f64::NEG_INFINITY, f64::max);
        matrix = TransformMatrix::translate(-min_x, -min_y).concat(&matrix);
        let height = max_y - min_y;
        (max_x - min_x, height, if preserve_baseline { matrix.apply(w / 2.0, body_baseline).1 } else { height })
    } else { (w, h, body_baseline) };
    if l.initial_baseline_pending {
        let ascent = baseline.max(0.0);
        l.regions.current.cursor_y += Pt(ascent);
        l.line_assumed_ascent = Some(ascent);
        l.initial_baseline_pending = false;
    } else {
        l.ensure_initial_baseline();
    }
    let pos = Point { x: l.regions.current.cursor_x, y: l.regions.current.cursor_y - Pt(baseline) };
    if let Some(collector) = l.decoration_lines_collector.as_mut() {
        collector.extend(deco.into_iter().map(|seg| super::DecoSegment {
            start_x: seg.start_x + pos.x, end_x: seg.end_x + pos.x, baseline_y: seg.baseline_y + pos.y,
        }));
    }
    l.regions.current.current_line.push(FrameItem::Group {
        paint_root: false, pos, matrix, clip_mask: None, inner_width: w, inner_height: h, items,
    });
    l.note_inline_extent(baseline.max(0.0), (frame_h - baseline).max(0.0));
    l.regions.current.cursor_x += Pt(frame_w);
}

fn first_baseline(items: &[FrameItem]) -> Option<f64> {
    items.iter().find_map(|item| match item {
        FrameItem::Text { pos, .. } | FrameItem::TextShaped { pos, .. } | FrameItem::Glyph { pos, .. } => Some(pos.y.0),
        FrameItem::TextReplacement { items, .. } | FrameItem::Semantic { items, .. } | FrameItem::Link { items, .. } => first_baseline(items),
        FrameItem::Group { pos, matrix, items, .. } => first_baseline(items).map(|y| pos.y.0 + matrix.apply(0.0, y).1),
        _ => None,
    })
}
''';p.write_text(s)
