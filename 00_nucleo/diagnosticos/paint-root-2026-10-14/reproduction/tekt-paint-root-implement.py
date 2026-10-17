from pathlib import Path
import json,gzip,hashlib,datetime,subprocess
out=Path('00_nucleo/diagnosticos/paint-root-2026-10-14');owners=json.loads((out/'owners.json').read_text());paths=list(owners)+list(owners.values())+['benches/stream_bench.rs']
with gzip.open(out/'red-behavior-state.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),head=subprocess.check_output(['git','rev-parse','HEAD'],text=True),status=subprocess.check_output(['git','status','--porcelain=v1'],text=True),diffstat=subprocess.check_output(['git','diff','HEAD','--stat'],text=True),files={p:dict(text=Path(p).read_text(),sha256=hashlib.sha256(Path(p).read_bytes()).hexdigest()) for p in paths}),f)
p=Path('01_core/src/compiler/layout/boxed.rs');s=p.read_text();needle='    let body_pos = Point { x: Pt(inset.left), y: Pt(inset.top) };';assert s.count(needle)==1;s=s.replace(needle,'    let body_start = contents.len();\n'+needle)
needle='    if let Some(collector) = layouter.decoration_lines_collector.as_mut() {\n        collector.extend(deco.into_iter()';assert needle in s
s=s.replace(needle,'''    // Only the descendants use the new root. The backing is painted in the
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
'''+needle);p.write_text(s)
p=Path('01_core/src/compiler/layout/block.rs');s=p.read_text();old='''    if same_region {
        let causal_h = (layouter.flow_placement_y().unwrap_or(inner_origin_y)
            - inner_origin_y)
            .max(0.0);
        let allocated_h = resolved_height.unwrap_or(causal_h + inset_top + inset_bottom);
        let inner_h = (allocated_h - inset_top - inset_bottom).max(0.0);''';new='''    let root_height = same_region.then(|| {
        let causal_h = (layouter.flow_placement_y().unwrap_or(inner_origin_y)
            - inner_origin_y)
            .max(0.0);
        resolved_height.unwrap_or(causal_h + inset_top + inset_bottom)
    });
    if let Some(allocated_h) = root_height {
        let inner_h = (allocated_h - inset_top - inset_bottom).max(0.0);''';assert old in s;s=s.replace(old,new)
needle='    // Descendentes do Block não podem fazer a região explícita vazar para o\n';assert s.count(needle)==1
s=s.replace(needle,'''    // Root geometry is the allocated container, including inset. Its own
    // backing is inserted later as a preceding sibling, outside this scope.
    if let Some(allocated_h) = root_height {
        let pos = crate::entities::layout_types::Point {
            x: block_origin_x,
            y: Pt(start_y + outset_top),
        };
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
            inner_width: outer_w,
            inner_height: allocated_h,
            items,
        });
    }

'''+needle);p.write_text(s)
p=Path('01_core/src/compiler/layout/tiling.rs');s=p.read_text();needle='''            cells.push(FrameItem::Group {
                paint_root: false,''';assert needle in s;s=s.replace(needle,needle.replace('false','true'),1);p.write_text(s)
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true','01_core/src/compiler/layout/boxed.rs','01_core/src/compiler/layout/block.rs','01_core/src/compiler/layout/tiling.rs'],check=True)
