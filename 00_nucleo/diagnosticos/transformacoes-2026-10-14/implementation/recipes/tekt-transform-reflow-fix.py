from pathlib import Path
p=Path('01_core/src/compiler/layout/transform.rs');s=p.read_text()
a=s.index('    let (body_h, mut items, deco, align_x, align_y) = l\n',s.index('fn layout_native'))
b=s.index('    let name = match e.operation',a)
s=s[:a]+s[b:]
s=s.replace('    let style = |field: &str| l.chain.custom(&format!("{name}.{field}"));','    let chain = l.chain.clone();\n    let style = |field: &str| chain.custom(&format!("{name}.{field}"));')
s=s.replace('    let mut preserve_baseline = true;','''    // Absolute scale is relative to the original logical frame, before reflow.
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
    let mut preserve_baseline = true;''')
s=s.replace('let sx = factor(x.or_else(|| styled_scale("x")), w);','let sx = factor(x.or_else(|| styled_scale("x")), scale_basis.0);')
s=s.replace('let sy = factor(y.or_else(|| styled_scale("y")), h);','let sy = factor(y.or_else(|| styled_scale("y")), scale_basis.1);')
a=s.index('    let inherited = match style("origin")',s.index('fn layout_native'))
s=s[:a]+'''    let reflow = reflow.unwrap_or(matches!(style("reflow"), Some(Value::Bool(true))));
    let measured = if reflow {
        let size = match e.operation {
            TransformOperation::Scale { .. } => {
                let axis = |extent: f64, scale: f64| {
                    if extent.is_finite() { (extent / scale).abs() } else { extent }
                };
                (axis(region_size.0, matrix.a), axis(region_size.1, matrix.d))
            }
            TransformOperation::Rotate { .. } | TransformOperation::Skew { .. } => {
                if region_size.0.is_finite() && region_size.1.is_finite() {
                    // Rotation uses its inverse; shear uses the forward bbox.
                    // Both have the same absolute coefficients as the paint matrix.
                    (matrix.a.abs() * region_size.0 + matrix.c.abs() * region_size.1,
                     matrix.b.abs() * region_size.0 + matrix.d.abs() * region_size.1)
                } else {
                    (f64::INFINITY, f64::INFINITY)
                }
            }
            TransformOperation::Move { .. } | TransformOperation::Matrix(_) => region_size,
        };
        let Some(measured) = measure_native_body(l, &e.body, size.0, Some(size.1)) else {
            return;
        };
        Some(measured)
    } else {
        None
    };
    let body = compose_native_body(
        l, &e.body,
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
'''+s[a:]
# The second identical binding was at the old position after pivot resolution.
needle='    let reflow = reflow.unwrap_or(matches!(style("reflow"), Some(Value::Bool(true))));\n'
a=s.index(needle);b=s.index(needle,a+len(needle));s=s[:b]+s[b+len(needle):]
a=s.index('\nfn first_baseline(')
s=s[:a]+'''
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
        SubLayoutRegion { origin_x: 0.0, width, height, align_rtl: false,
            unconstrained_height: height.is_none() },
    );
    let width = l.last_sub_frame_width.max(l.last_sub_frame_flow_width)
        .max(super::helpers::line_content_right(&items, &l.metrics));
    let height = body_h.max(l.last_sub_frame_bottom.unwrap_or(0.0));
    let baseline = if l.last_sub_frame_inline_extents.0 > 0.0 {
        l.last_sub_frame_inline_extents.0
    } else {
        first_baseline(&items).unwrap_or(height)
    };
    NativeBody { width, height, baseline, items, deco, align_x, align_y }
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
''' +s[a:]
p.write_text(s)
