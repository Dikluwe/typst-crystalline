from pathlib import Path
import gzip,json,datetime,subprocess,hashlib
out=Path('00_nucleo/diagnosticos/tiling-cobertura-2026-10-14')
files=[Path('01_core/src/compiler/layout/tiling.rs'),Path('00_nucleo/prompts/compiler/layout/tiling.md')]
r=dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={str(p):dict(text=p.read_text(),sha256=hashlib.sha256(p.read_bytes()).hexdigest()) for p in files})
with gzip.open(out/'red-state.json.gz','wt') as f:json.dump(r,f)
p=files[0];s=p.read_text();s=s.replace('''    if !layout_width.is_finite() && explicit.is_some() || layout_width <= 0.0 {
''','''    if explicit.is_some_and(|size| {
        !size.width.0.is_finite()
            || !size.height.0.is_finite()
            || size.width.0 <= 0.0
            || size.height.0 <= 0.0
    }) {
''');s=s.replace('layouter.layout_sub_frame(', 'layouter.layout_sub_frame_at_geometric_origin(')
s=s.replace('    let body_width = layouter.last_sub_frame_width;', '''    let body_width = layouter.last_sub_frame_width.max(super::helpers::line_content_right(
        &body_items,
        &layouter.metrics,
    ));''')
s=s.replace('''    if body_items.is_empty()
        || !cell_width.is_finite()''','''    if !cell_width.is_finite()''')
start=s.index('    let radius = libm::hypot(');end=s.index('    let mut cells = ',start)
s=s[:start]+'''    if !pos.x.0.is_finite() || !pos.y.0.is_finite() {
        return None;
    }
    if body_items.is_empty() {
        return Some(FrameItem::Group {
            pos,
            matrix: TransformMatrix::identity(),
            clip_mask: Some(clip_kind),
            inner_width: width,
            inner_height: height,
            items: Vec::new(),
        });
    }

    // Whole pitches change cell indices, never the visible pattern. Reduce
    // both phases before combining them to avoid huge translations and
    // saturating integer casts for otherwise finite user offsets.
    let phase_pos = Point {
        x: Pt(pos.x.0.rem_euclid(pitch_x)),
        y: Pt(pos.y.0.rem_euclid(pitch_y)),
    };
    let placement = placement_transform(
        tiling.relative,
        phase_pos,
        offset_x.rem_euclid(pitch_x),
        offset_y.rem_euclid(pitch_y),
        angle,
    );
    let determinant = placement.a * placement.d - placement.b * placement.c;
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    for (x, y) in [(0.0, 0.0), (width, 0.0), (0.0, height), (width, height)] {
        let dx = x - placement.tx;
        let dy = y - placement.ty;
        let local_x = (placement.d * dx - placement.c * dy) / determinant;
        let local_y = (placement.a * dy - placement.b * dx) / determinant;
        if !local_x.is_finite() || !local_y.is_finite() {
            return None;
        }
        min_x = min_x.min(local_x);
        max_x = max_x.max(local_x);
        min_y = min_y.min(local_y);
        max_y = max_y.max(local_y);
    }
    let (min_x, max_x) = lattice_axis(min_x, max_x, cell_width, pitch_x)?;
    let (min_y, max_y) = lattice_axis(min_y, max_y, cell_height, pitch_y)?;
    let span_x = i64::from(max_x) - i64::from(min_x) + 1;
    let span_y = i64::from(max_y) - i64::from(min_y) + 1;
    let count = span_x.saturating_mul(span_y);
    if count <= 0 || count > 10_000 {
        return None;
    }

'''+s[end:]
s=s.replace('    let placement = placement_transform(tiling.relative, pos, offset_x, offset_y, angle);\n','')
start=s.index('\nfn placement_transform(')
s=s[:start]+'''
// Include boundary cells and overlapping tiles; only the enclosing clip
// determines painted pixels. Check bounds before any integer conversion.
fn lattice_axis(min: f64, max: f64, cell: f64, pitch: f64) -> Option<(i32, i32)> {
    let first = ((min - cell) / pitch).floor();
    let last = (max / pitch).ceil();
    if !first.is_finite()
        || !last.is_finite()
        || first < f64::from(i32::MIN)
        || last > f64::from(i32::MAX)
        || first > last
    {
        return None;
    }
    Some((first as i32, last as i32))
}
''' +s[start:]
p.write_text(s)
