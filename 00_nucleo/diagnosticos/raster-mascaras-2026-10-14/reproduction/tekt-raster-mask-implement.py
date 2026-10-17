from pathlib import Path
p=Path('03_infra/src/export/render.rs');s=p.read_text()
s=s.replace('use typst_core::entities::geometry::{LineCap, PathItem, ShapeKind, Stroke};','use typst_core::entities::geometry::{\n    DashLength, FillRule, LineCap, LineJoin, PathItem, ShapeKind, Stroke,\n};')
s=s.replace('struct State {\n    transform: sk::Transform,\n}',"struct State<'a> {\n    transform: sk::Transform,\n    mask: Option<&'a sk::Mask>,\n}")
s=s.replace('impl State {',"impl State<'_> {")
s=s.replace('transform: sk::Transform::from_scale(pixel_per_pt, pixel_per_pt),','transform: sk::Transform::from_scale(pixel_per_pt, pixel_per_pt),\n            mask: None,')
s=s.replace('Self { transform: self.transform.pre_translate(x, y) }','Self { transform: self.transform.pre_translate(x, y), ..self }').replace('Self { transform: self.transform.pre_concat(other) }','Self { transform: self.transform.pre_concat(other), ..self }')
s=s.replace('state: State,',"state: State<'_>,")
old='''        FrameItem::Image { pos, data, width, height, orientation, .. } => {
            render_image(canvas, state, pos, data, *width, *height, *orientation);
        }'''
new='''        FrameItem::Image { pos, data, width, height, orientation, clip_rect, .. } => {
            let mask = if let Some(clip) = clip_rect {
                let Some(rect) = sk::Rect::from_xywh(
                    clip.x.0 as f32, clip.y.0 as f32, clip.w.0 as f32, clip.h.0 as f32,
                ) else { return };
                let path = sk::PathBuilder::from_rect(rect);
                let Some(mask) = intersect_clip(canvas, state, &path, state.transform)
                else { return };
                Some(mask)
            } else { None };
            let state = State { mask: mask.as_ref().or(state.mask), ..state };
            render_image(canvas, state, pos, data, *width, *height, *orientation);
        }'''
assert old in s;s=s.replace(old,new)
s=s.replace('FrameItem::Shape { pos, kind, width, height, fill, stroke, .. } => {','FrameItem::Shape { pos, kind, width, height, fill, stroke, fill_rule, .. } => {',1)
s=s.replace('                stroke.as_ref(),\n            );','                stroke.as_ref(),\n                *fill_rule,\n            );',1)
old='''        FrameItem::Group { pos, matrix, clip_mask, items, .. } => {
            render_group(canvas, state, pos, matrix, clip_mask.as_ref(), items, fonts);
        }'''
new='''        FrameItem::Group { pos, matrix, clip_mask, inner_width, inner_height, items, .. } => {
            render_group(canvas, state, pos, matrix,
                clip_mask.as_ref().map(|clip| (clip, *inner_width, *inner_height)),
                items, fonts);
        }'''
assert old in s;s=s.replace(old,new)
s=s.replace('            let cov = bitmap.coverage[ai];','''            let pi = (py * cw + px) as usize;
            let cov = bitmap.coverage[ai];
            let cov = state.mask.map_or(cov, |mask| {
                ((u16::from(cov) * u16::from(mask.data()[pi]) + 127) / 255) as u8
            });''',1)
s=s.replace('            let pi = (py * cw + px) as usize;\n            pixels[pi]', '            pixels[pi]',1)
s=s.replace('canvas.stroke_path(&path, &paint, &stroke, state.transform, None);','canvas.stroke_path(&path, &paint, &stroke, state.transform, state.mask);')
s=s.replace('canvas.fill_rect(rect, &paint, ts, None);','canvas.fill_rect(rect, &paint, ts, state.mask);')
s=s.replace('    stroke: Option<&Stroke>,\n) {','    stroke: Option<&Stroke>,\n    fill_rule: FillRule,\n) {',1)
s=s.replace('canvas.fill_path(&path, &paint, sk::FillRule::Winding, ts, None);','''canvas.fill_path(&path, &paint, match fill_rule {
            FillRule::NonZero => sk::FillRule::Winding,
            FillRule::EvenOdd => sk::FillRule::EvenOdd,
        }, ts, state.mask);''',1)
s=s.replace('''            line_cap: sk::LineCap::Butt,
            line_join: sk::LineJoin::Miter,
            ..Default::default()''','''            line_cap: match stroke.cap {
                LineCap::Butt => sk::LineCap::Butt,
                LineCap::Round => sk::LineCap::Round,
                LineCap::Square => sk::LineCap::Square,
            },
            line_join: match stroke.join {
                LineJoin::Miter => sk::LineJoin::Miter,
                LineJoin::Round => sk::LineJoin::Round,
                LineJoin::Bevel => sk::LineJoin::Bevel,
            },
            miter_limit: stroke.miter_limit as f32,
            dash: stroke.dash.as_ref().and_then(|dash| {
                let len = dash.array.len();
                let count = if len % 2 == 0 { len } else { len * 2 };
                let array = dash.array.iter().cycle().take(count).map(|length| {
                    match length {
                        DashLength::Length(length) => *length,
                        DashLength::LineWidth => stroke.thickness,
                    } as f32
                }).collect();
                sk::StrokeDash::new(array, dash.phase as f32)
            }),''',1)
s=s.replace('canvas.stroke_path(&path, &paint, &sk_stroke, ts, None);','canvas.stroke_path(&path, &paint, &sk_stroke, ts, state.mask);')
s=s.replace('''        ShapeKind::Ellipse => {
            let rx''','''        ShapeKind::Ellipse => {
            sk::Rect::from_xywh(0.0, 0.0, width as f32, height as f32)?;
            let rx''',1)
s=s.replace('''    _clip_mask: Option<&ShapeKind<typst_core::entities::layout_types::Pt>>,''','''    clip_mask: Option<(&ShapeKind<Pt>, f64, f64)>,''',1)
s=s.replace('''    for item in items {
        render_item(canvas, new_state, item, fonts);
    }
}

fn color_to_sk''','''    let mask = if let Some((kind, width, height)) = clip_mask {
        let Some(path) = shape_to_path(kind, width, height) else { return };
        let Some(mask) = intersect_clip(canvas, new_state, &path, new_state.transform)
        else { return };
        Some(mask)
    } else { None };
    let new_state = State { mask: mask.as_ref().or(new_state.mask), ..new_state };
    for item in items {
        render_item(canvas, new_state, item, fonts);
    }
}

fn intersect_clip(
    canvas: &sk::Pixmap,
    state: State<'_>,
    path: &sk::Path,
    transform: sk::Transform,
) -> Option<sk::Mask> {
    if let Some(parent) = state.mask {
        let mut mask = parent.clone();
        mask.intersect_path(path, sk::FillRule::Winding, true, transform);
        Some(mask)
    } else {
        let mut mask = sk::Mask::new(canvas.width(), canvas.height())?;
        mask.fill_path(path, sk::FillRule::Winding, true, transform);
        Some(mask)
    }
}

fn color_to_sk''',1)
s=s.replace('''    let data = canvas.data();
    let rgba = image::RgbaImage::from_raw(width, height, data.to_vec())''','''    let data = canvas.pixels().iter().flat_map(|pixel| {
        let pixel = pixel.demultiply();
        [pixel.red(), pixel.green(), pixel.blue(), pixel.alpha()]
    }).collect();
    let rgba = image::RgbaImage::from_raw(width, height, data)''',1)
p.write_text(s)
