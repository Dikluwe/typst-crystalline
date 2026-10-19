from pathlib import Path
p=Path('03_infra/src/export/render.rs');s=p.read_text()
s=s.replace('PathItem, ShapeKind, Stroke,','PathItem, ShapeKind,')
s=s.replace('use tiny_skia as sk;', 'use tiny_skia as sk;\nuse typst_core::entities::frame_paint::{FramePaint, FrameStroke as Stroke, TilingFrame};')
s=s.replace('    transform: sk::Transform,','    transform: sk::Transform,\n    local_to_root: TransformMatrix,',1)
s=s.replace('            mask: None,','            mask: None,\n            local_to_root: TransformMatrix::identity(),',1)
s=s.replace('fill.as_ref().map(|paint| paint.to_color()),','fill.as_ref(),').replace('                *fill_rule,\n            );','                *fill_rule,\n                fonts,\n            );',1)
s=s.replace('        FrameItem::Group {\n            pos,','        FrameItem::Group {\n            paint_root,\n            pos,',1)
s=s.replace('                matrix,\n                clip_mask.as_ref()', '                matrix,\n                *paint_root,\n                clip_mask.as_ref()',1)
s=s.replace('    fill: Option<Color>,','    fill: Option<&FramePaint>,',1).replace('    fill_rule: FillRule,\n) {','    fill_rule: FillRule,\n    fonts: Option<&super::font_binding::PreparedFonts>,\n) {',1)
s=s.replace('''        let paint = sk::Paint {
            shader: sk::Shader::SolidColor(color_to_sk(fill)),
            anti_alias: true,
            ..Default::default()
        };''','''        let texture = prepare_texture(fill, state, *pos, fonts);
        let Some(paint) = frame_paint(fill, texture.as_ref()) else { return; };''',1)
s=s.replace('''        let paint = sk::Paint {
            shader: sk::Shader::SolidColor(color_to_sk(stroke.paint.to_color())),
            anti_alias: true,
            ..Default::default()
        };''','''        let texture = prepare_texture(&stroke.paint, state, *pos, fonts);
        let Some(paint) = frame_paint(&stroke.paint, texture.as_ref()) else { return; };''',1)
s=s.replace('    matrix: &TransformMatrix,\n    clip_mask:', '    matrix: &TransformMatrix,\n    paint_root: bool,\n    clip_mask:',1)
s=s.replace('''    let new_state = state
        .pre_translate(pos.x.0 as f32, pos.y.0 as f32)
        .pre_concat(sk_matrix);''','''    let mut new_state = state
        .pre_translate(pos.x.0 as f32, pos.y.0 as f32)
        .pre_concat(sk_matrix);
    new_state.local_to_root = if paint_root { TransformMatrix::identity() }
        else { state.local_to_root.concat(&TransformMatrix::translate(pos.x.0, pos.y.0)).concat(matrix) };''',1)
pos=s.index('\nfn render_shape(')
s=s[:pos]+'''
fn sk_transform(m: TransformMatrix) -> sk::Transform {
    sk::Transform::from_row(m.a as f32, m.b as f32, m.c as f32, m.d as f32, m.tx as f32, m.ty as f32)
}

fn frame_paint<'a>(paint: &FramePaint, texture: Option<&'a (sk::Pixmap, sk::Transform)>) -> Option<sk::Paint<'a>> {
    let shader = match paint {
        FramePaint::Solid(color) => sk::Shader::SolidColor(color_to_sk(color.clone())),
        FramePaint::Gradient(g) => sk::Shader::SolidColor(color_to_sk(typst_core::entities::paint::Paint::Gradient(g.clone()).to_color())),
        FramePaint::Tiling(_) => {
            let (pixmap, matrix) = texture?;
            sk::Pattern::new(pixmap.as_ref(), sk::SpreadMode::Repeat, sk::FilterQuality::Bilinear, 1.0, *matrix)
        }
    };
    Some(sk::Paint { shader, anti_alias: true, ..Default::default() })
}

fn prepare_texture(paint: &FramePaint, state: State<'_>, origin: Point, fonts: Option<&super::font_binding::PreparedFonts>) -> Option<(sk::Pixmap, sk::Transform)> {
    let FramePaint::Tiling(cell) = paint else { return None; };
    let placement = super::tiling::placement(cell, origin, state.local_to_root)?;
    let device = state.transform.pre_concat(sk_transform(placement));
    let pitch_x = cell.frame.size.width.0 + cell.spacing.width.0;
    let pitch_y = cell.frame.size.height.0 + cell.spacing.height.0;
    let px = (pitch_x * f64::from(device.sx.hypot(device.ky))).ceil().max(1.0) as u32;
    let py = (pitch_y * f64::from(device.kx.hypot(device.sy))).ceil().max(1.0) as u32;
    // Supersampling resolves coverage even when an entire period is subpixel.
    let high_x = px.max(16); let high_y = py.max(16);
    let period = raster_period(cell, high_x, high_y, fonts)?;
    let texture = if (px,py) == (high_x,high_y) { period } else {
        let source = image::RgbaImage::from_raw(high_x, high_y, period.data().to_vec())?;
        let reduced = image::imageops::resize(&source, px, py, image::imageops::FilterType::Triangle);
        let mut reduced_texture = sk::Pixmap::new(px,py)?;
        reduced_texture.data_mut().copy_from_slice(reduced.as_raw());
        reduced_texture
    };
    let matrix = TransformMatrix::translate(-origin.x.0,-origin.y.0).concat(&placement)
        .concat(&TransformMatrix::scale(pitch_x/f64::from(px),pitch_y/f64::from(py)));
    Some((texture, sk_transform(matrix)))
}

fn raster_period(cell: &TilingFrame, px: u32, py: u32, fonts: Option<&super::font_binding::PreparedFonts>) -> Option<sk::Pixmap> {
    let width = cell.frame.size.width.0; let height = cell.frame.size.height.0;
    let pitch_x = width + cell.spacing.width.0; let pitch_y = height + cell.spacing.height.0;
    let sx = f64::from(px)/pitch_x; let sy = f64::from(py)/pitch_y;
    let cw = (width*sx).ceil().max(1.0) as u32; let ch = (height*sy).ceil().max(1.0) as u32;
    let mut body = sk::Pixmap::new(cw,ch)?;
    let transform = sk::Transform::from_scale(sx as f32, sy as f32);
    let mut mask = sk::Mask::new(cw,ch)?;
    let rect = sk::Rect::from_xywh(0.0,0.0,width as f32,height as f32)?;
    mask.fill_path(&sk::PathBuilder::from_rect(rect), sk::FillRule::Winding, true, transform);
    let state = State { transform, local_to_root: TransformMatrix::identity(), mask: Some(&mask) };
    for item in &cell.frame.items { render_item(&mut body,state,item,fonts); }
    let mut period = sk::Pixmap::new(px,py)?;
    // Only overlap contributors to one period; never enumerate the target area.
    let nx = (width/pitch_x).ceil() as i64; let ny = (height/pitch_y).ceil() as i64;
    for y in (1-ny)..=0 {
        for x in (1-nx)..=0 {
            let tx = x.checked_mul(i64::from(px))?; let ty = y.checked_mul(i64::from(py))?;
            period.draw_pixmap(i32::try_from(tx).ok()?, i32::try_from(ty).ok()?, body.as_ref(), &sk::PixmapPaint::default(), sk::Transform::identity(), None);
        }
    }
    Some(period)
}
''' +s[pos:]
p.write_text(s)
