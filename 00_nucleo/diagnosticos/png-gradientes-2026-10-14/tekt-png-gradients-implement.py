from pathlib import Path
p=Path('03_infra/src/export/render.rs');s=p.read_text()
s=s.replace('    local_to_root: TransformMatrix,','    local_to_root: TransformMatrix,\n    root_size: (f32, f32),\n    sample_scale: (f32, f32),',1)
s=s.replace('            local_to_root: TransformMatrix::identity(),','            local_to_root: TransformMatrix::identity(),\n            root_size: (0.0, 0.0),\n            sample_scale: (pixel_per_pt, pixel_per_pt),',1)
s=s.replace('    match &page.fill {','    let mut state = State { root_size: (width as f32, height as f32), ..State::new(pixel_per_pt) };\n    match &page.fill {',1)
s=s.replace('            canvas.fill(color_to_sk(paint.to_color()));','''            match paint {
                typst_core::entities::paint::Paint::Gradient(g) => render_shape(
                    &mut canvas, state, &Point::ZERO, &ShapeKind::Rect, width, height,
                    Some(&FramePaint::Gradient(g.clone())), None, FillRule::NonZero, fonts,
                ),
                typst_core::entities::paint::Paint::Solid(_) | typst_core::entities::paint::Paint::Tiling(_) => canvas.fill(color_to_sk(paint.to_color())),
            }''',1)
s=s.replace('    let mut state = State::new(pixel_per_pt);','    state.root_size = (page.width as f32, page.height as f32);',1)
s=s.replace('            let state = State {\n                local_to_root: if *paint_root {','            let state = State {\n                root_size: if *paint_root { (*inner_width as f32, *inner_height as f32) } else { state.root_size },\n                local_to_root: if *paint_root {',1)
s=s.replace('''        FramePaint::Gradient(g) => sk::Shader::SolidColor(color_to_sk(
            typst_core::entities::paint::Paint::Gradient(g.clone()).to_color(),
        )),''','''        FramePaint::Gradient(_) => {
            let (pixmap, matrix) = texture?;
            sk::Pattern::new(pixmap.as_ref(), sk::SpreadMode::Pad, sk::FilterQuality::Nearest, 1.0, *matrix)
        },''',1)
s=s.replace('    Some(sk::Paint { shader, anti_alias: true, ..Default::default() })','''    let anti_alias = match paint {
        FramePaint::Gradient(g) => match g {
            typst_core::entities::gradient::Gradient::Linear(g) => g.anti_alias,
            typst_core::entities::gradient::Gradient::Radial(g) => g.anti_alias,
            typst_core::entities::gradient::Gradient::Conic(g) => g.anti_alias,
        },
        FramePaint::Solid(_) | FramePaint::Tiling(_) => true,
    };
    Some(sk::Paint { shader, anti_alias, ..Default::default() })''',1)
s=s.replace('''    origin: Point,
    fonts: Option<&super::font_binding::PreparedFonts>,
) -> Option<(sk::Pixmap, sk::Transform)> {
    let FramePaint::Tiling(cell) = paint else {
        return None;
    };''','''    origin: Point,
    geometry: PaintGeometry,
    fonts: Option<&super::font_binding::PreparedFonts>,
) -> Option<(sk::Pixmap, sk::Transform)> {
    let cell = match paint {
        FramePaint::Solid(_) => return None,
        FramePaint::Gradient(g) => return gradient_texture(g, state, origin, geometry),
        FramePaint::Tiling(cell) => cell,
    };''',1)
s=s.replace('''        local_to_root: TransformMatrix::identity(),
        mask: Some(&mask),''','''        local_to_root: TransformMatrix::identity(),
        root_size: (width as f32, height as f32),
        sample_scale: (sx as f32, sy as f32),
        mask: Some(&mask),''',1)
# Lift stroke conversion intact so geometry and rendering consume one resolved style.
a=s.index('        let sk_stroke = sk::Stroke {',s.index('fn render_shape('));b=s.index('        canvas.stroke_path',a)
stroke=s[a:b].replace('        let sk_stroke = sk::Stroke {','    sk::Stroke {',1).rstrip();stroke=stroke[:-1] # remove final semicolon
s=s[:a]+s[b:]
s=s.replace('''    if let Some(fill) = fill {
        let texture = prepare_texture(fill, state, *pos, fonts);
        let Some(paint) = frame_paint(fill, texture.as_ref()) else {
            return;
        };
        canvas.fill_path(''','''    let mirror = (matches!(kind, ShapeKind::Rect) && width < 0.0, matches!(kind, ShapeKind::Rect) && height < 0.0);
    if let Some(fill) = fill {
        let Some(bounds) = path.compute_tight_bounds() else { return; };
        let texture = prepare_texture(fill, state, *pos, PaintGeometry { bounds, mirror }, fonts);
        if let Some(mut paint) = frame_paint(fill, texture.as_ref()) {
        if matches!(kind, ShapeKind::Rect) && matches!(fill, FramePaint::Gradient(_)) { paint.anti_alias = false; }
        canvas.fill_path(''',1)
s=s.replace('''            state.mask,
        );
    }

    if let Some(stroke) = stroke {
        let texture = prepare_texture(&stroke.paint, state, *pos, fonts);
        let Some(paint) = frame_paint(&stroke.paint, texture.as_ref()) else {
            return;
        };''','''            state.mask,
        );
        }
    }

    if let Some(stroke) = stroke {
        let sk_stroke = raster_stroke(stroke);
        let bounds = if matches!(kind, ShapeKind::Rect) {
            let half = stroke.thickness as f32 / 2.0;
            sk::Rect::from_xywh((width as f32).min(0.0)-half, (height as f32).min(0.0)-half, width.abs() as f32+2.0*half, height.abs() as f32+2.0*half)
        } else {
            let dashed = sk_stroke.dash.as_ref().and_then(|dash| path.dash(dash, 1.0));
            dashed.as_ref().unwrap_or(&path).stroke(&sk_stroke, 1.0).and_then(|p| p.compute_tight_bounds())
        };
        let Some(bounds) = bounds else { return; };
        let texture = prepare_texture(&stroke.paint, state, *pos, PaintGeometry { bounds, mirror }, fonts);
        let Some(paint) = frame_paint(&stroke.paint, texture.as_ref()) else { return; };''',1)
s=s.replace('fn shape_to_path(', 'fn raster_stroke(stroke: &Stroke) -> sk::Stroke {\n'+stroke+'\n}\n\nfn shape_to_path(',1)
s=s.replace('''let rect = sk::Rect::from_xywh(0.0, 0.0, width as f32, height as f32)?;
            Some(sk::PathBuilder::from_rect(rect))''','''let rect = sk::Rect::from_xywh(width.min(0.0) as f32, height.min(0.0) as f32, width.abs() as f32, height.abs() as f32)?;
            Some(sk::PathBuilder::from_rect(rect))''',1)
helpers='''
#[derive(Clone, Copy)]
struct PaintGeometry {
    bounds: sk::Rect,
    mirror: (bool, bool),
}

fn gradient_texture(
    gradient: &typst_core::entities::gradient::Gradient,
    state: State<'_>,
    origin: Point,
    geometry: PaintGeometry,
) -> Option<(sk::Pixmap, sk::Transform)> {
    use typst_core::entities::gradient::{Gradient, RelativeTo};
    let relative = match gradient {
        Gradient::Linear(g) => g.relative,
        Gradient::Radial(g) => g.relative,
        Gradient::Conic(g) => g.relative,
    }.unwrap_or_default();
    let (size, base, mirror) = match relative {
        RelativeTo::Self_ => ((geometry.bounds.width(), geometry.bounds.height()), sk::Transform::from_translate(geometry.bounds.left(), geometry.bounds.top()), geometry.mirror),
        RelativeTo::Parent => (state.root_size, sk_transform(TransformMatrix::translate(-origin.x.0, -origin.y.0).concat(&super::tiling::inverse(state.local_to_root)?)), (false, false)),
    };
    let (sx, sy) = state.sample_scale;
    let width = (size.0.abs() * sx).ceil().max(sx.ceil()).max(1.0) as u32;
    let height = (size.1.abs() * sy).ceil().max(sy.ceil()).max(1.0) as u32;
    let mut texture = sk::Pixmap::new(width, height)?;
    for (i, pixel) in texture.pixels_mut().iter_mut().enumerate() {
        let x = (i % width as usize) as f32;
        let y = (i / width as usize) as f32;
        let x = if mirror.0 { width as f32-x } else { x };
        let y = if mirror.1 { height as f32-y } else { y };
        let color = gradient_sample(gradient, x / width as f32, y / height as f32, width as f32 / height as f32);
        let (r,g,b,a) = color.to_rgba_f32();
        *pixel = sk::Color::from_rgba(r.clamp(0.0,1.0),g.clamp(0.0,1.0),b.clamp(0.0,1.0),a.clamp(0.0,1.0))?.premultiply().to_color_u8();
    }
    Some((texture, base.pre_scale(1.0/sx, 1.0/sy)))
}

fn gradient_sample(gradient: &typst_core::entities::gradient::Gradient, x: f32, y: f32, aspect: f32) -> Color {
    use typst_core::entities::gradient::Gradient;
    let (x,y,aspect) = (f64::from(x),f64::from(y),f64::from(aspect));
    match gradient {
        Gradient::Linear(g) => {
            let angle = libm::atan2(libm::sin(g.angle.to_rad()) / aspect.abs(), libm::cos(g.angle.to_rad()));
            let (sin,cos) = (libm::sin(angle),libm::cos(angle));
            let x = if cos < 0.0 {1.0-x} else {x};
            let y = if sin < 0.0 {1.0-y} else {y};
            g.sample(((x*cos.abs()+y*sin.abs())/(cos.abs()+sin.abs())) as f32)
        }
        Gradient::Radial(g) => {
            let (dx,dy) = (x-g.focal_center.x.0,y-g.focal_center.y.0);
            let distance = libm::hypot(dx,dy);
            let t = if distance <= g.focal_radius.0 || distance == 0.0 { 0.0 }
            else if libm::hypot(x-g.center.x.0,y-g.center.y.0) > g.radius.0 { 1.0 }
            else {
                let (qx,qy) = (g.focal_center.x.0-g.center.x.0,g.focal_center.y.0-g.center.y.0);
                let a = (qx*dx+qy*dy)/distance;
                let rho = g.radius.0*g.radius.0-qx*qx-qy*qy;
                let boundary = libm::sqrt((a*a+rho).max(0.0))-a;
                (distance-g.focal_radius.0)/(boundary-g.focal_radius.0)
            };
            g.sample(t.clamp(0.0,1.0) as f32)
        }
        Gradient::Conic(g) => {
            let angle = libm::atan2((y-g.center.y.0)/aspect.abs(),x-g.center.x.0);
            let t = (std::f64::consts::PI+angle-g.angle.to_rad()).rem_euclid(std::f64::consts::TAU)/std::f64::consts::TAU;
            g.sample(t as f32)
        }
    }
}

'''
s=s.replace('fn raster_period(',helpers+'fn raster_period(',1)
p.write_text(s)
print('Implemented raster gradient consumer; no public API changes.')
