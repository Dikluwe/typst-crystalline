from pathlib import Path
import json,gzip,hashlib
out=Path('00_nucleo/diagnosticos/block-regiao-2026-10-14')
with gzip.open(out/'red-state.json.gz','rt') as f:pre=json.load(f)
assert all(hashlib.sha256(Path(n).read_bytes()).hexdigest()==v['sha256'] for n,v in pre['files'].items())
# L0 refinement precedes private metadata implementation.
for n in ['mod','checkpoint','sub_frame','region_progress','block','placement']:
 owner='00_nucleo/prompts/compiler/layout.md' if n=='mod' else f'00_nucleo/prompts/compiler/layout/{n}.md'
 p=Path(owner);p.write_text(p.read_text()+ '\nB2a conserva também uma lista privada de faixas de decoração associadas ao índice do fixup Y de Place auto. O hub armazena; checkpoint clona/restaura; subframe e Block isolam/restauram; transição limpa; placement registra somente os segmentos emitidos pela ocorrência; Block aplica a mesma delta da geometria antes de encerrar o escopo.\n')
def edit(n,fn):
 p=Path(f'01_core/src/compiler/layout/{n}.rs');s=p.read_text();t=fn(s);assert s!=t,n;p.write_text(t)
edit('mod',lambda s:s.replace('    pub(super) flow_placement_end: Option<(f64, f64)>,','    pub(super) flow_placement_end: Option<(f64, f64)>,\n    /// Inner Block region; independent of the page and cell references.\n    pub(super) block_place_region: Option<crate::entities::layout_types::Rect>,\n    /// Deferred Y entry index and its local decoration segment range.\n    pub(super) block_place_decorations: Vec<(usize, usize, usize)>,').replace('            flow_placement_end: None,','            flow_placement_end: None,\n            block_place_region: None,\n            block_place_decorations: Vec::new(),').replace('        self.flow_placement_end = None;','        self.flow_placement_end = None;\n        self.block_place_region = None;\n        self.block_place_decorations.clear();'))
edit('checkpoint',lambda s:s.replace('    flow_placement_end: Option<(f64, f64)>,','    flow_placement_end: Option<(f64, f64)>,\n    block_place_region: Option<crate::entities::layout_types::Rect>,\n    block_place_decorations: Vec<(usize, usize, usize)>,').replace('            flow_placement_end: self.flow_placement_end,','            flow_placement_end: self.flow_placement_end,\n            block_place_region: self.block_place_region,\n            block_place_decorations: self.block_place_decorations.clone(),').replace('        self.flow_placement_end = checkpoint.flow_placement_end;','        self.flow_placement_end = checkpoint.flow_placement_end;\n        self.block_place_region = checkpoint.block_place_region;\n        self.block_place_decorations = checkpoint.block_place_decorations;'))
edit('sub_frame',lambda s:s.replace('        let saved_flow_placement_end = self.flow_placement_end.take();','        let saved_flow_placement_end = self.flow_placement_end.take();\n        let saved_block_place_region = self.block_place_region.take();\n        let saved_block_place_decorations = std::mem::take(&mut self.block_place_decorations);').replace('        self.flow_placement_end = saved_flow_placement_end;','        self.flow_placement_end = saved_flow_placement_end;\n        self.block_place_region = saved_block_place_region;\n        self.block_place_decorations = saved_block_place_decorations;'))
edit('region_progress',lambda s:s.replace('    layouter.flow_placement_end = None;','    layouter.flow_placement_end = None;\n    layouter.block_place_region = None;\n    layouter.block_place_decorations.clear();'))
def place(s):
 a=s.index('        let in_sub_frame = layouter.is_sub_frame;');b=s.index('\n    }\n}',a)
 return s[:a]+'        layouter.layout_place(effective_alignment, *dx, *dy, *scope, body, flow_y);'+s[b:]
edit('place',place)
def placement(s):
 a=s.index('    pub(super) fn layout_place(');pre=s[:a];s=s[a:]
 s=s.replace('        body: &Content,\n','        body: &Content,\n        flow_y: Option<f64>,\n',1)
 s=s.replace('        let avail_w_page = self.available_width();','        let block_region = self.block_place_region;\n        let avail_w_page = block_region.map_or_else(|| self.available_width(), |r| r.w.0);',1)
 s=s.replace('        let avail_h_page = if self.is_height_unconstrained {','        let avail_h_page = if let Some(region) = block_region {\n            region.h.0\n        } else if self.is_height_unconstrained {',1)
 s=s.replace('        let (origin_x, origin_y, avail_w, avail_h) = match scope {','        let (origin_x, origin_y, avail_w, avail_h) = if let Some(region) = block_region {\n            (region.x.0, region.y.0, region.w.0, region.h.0)\n        } else { match scope {',1)
 s=s.replace('        };\n\n        // **P897**', '        }};\n        let origin_y = if alignment.v.is_none() { flow_y.unwrap_or(origin_y) } else { origin_y };\n\n        // **P897**',1)
 s=s.replace('        if !avail_h_is_finite {','        let deferred_y_index = self.pending_align_v_centering.len();\n        if !avail_h_is_finite {',1)
 s=s.replace('        if let Some(coll) = self.decoration_lines_collector.as_mut() {\n            for seg in &sub_deco {','        if let Some(coll) = self.decoration_lines_collector.as_mut() {\n            let start = coll.len();\n            for seg in &sub_deco {',1)
 # locate end of decoration block before next function docs
 pos=s.index('            let start = coll.len();');end=s.index('\n        }\n    }',pos)
 s=s[:end]+'''\n            if block_region.is_some() && !avail_h_is_finite && start < coll.len() {
                self.block_place_decorations.push((deferred_y_index, start, coll.len()));
            }'''+s[end:]
 return pre+s
edit('placement',placement)
def block(s):
 s=s.replace('    // inset.right é scope-out (mesma razão que Pad.right\n    // em P156C — refino com refactor multi-region).','    let inset_right = inset.right.resolve_pt(font);')
 s=s.replace('        let outer_width = w_pt + inset_left;','        let outer_width = w_pt;')
 a=s.index('    if let Some(w_pt) = resolved_width {\n        let line_start_pt');b=s.index('    // P1292 v12',a)
 s=s[:a]+'''    let outer_w = resolved_width.unwrap_or(saved_width - saved_line_start.0 - layouter.page_config.margin.right);
    let inner_w = (outer_w - inset_left - inset_right).max(0.0);
    layouter.regions.current.width = layouter.regions.current.line_start_x.0
        + inner_w + layouter.page_config.margin.right;
'''+s[b:]
 a=s.index('    if *clip {\n        // P242');b=s.index('    // Auto-sized textual frames',a)
 s=s[:a]+'''    let inner_origin_y = layouter.regions.current.cursor_y.0;
    let saved_block_region = layouter.block_place_region.replace(crate::entities::layout_types::Rect {
        x: layouter.regions.current.line_start_x,
        y: Pt(inner_origin_y),
        w: Pt(inner_w),
        h: Pt(resolved_height.map_or(f64::INFINITY, |h| (h - inset_top - inset_bottom).max(0.0))),
    });
    let saved_block_decorations = std::mem::take(&mut layouter.block_place_decorations);
    let pending_y_before = layouter.pending_align_v_centering.len();
    let pending_x_before = layouter.pending_align_centering.len();
    let saved_parent_bbox = layouter.parent_bbox;
    if let (Some(w_pt), Some(h_pt)) = (resolved_width, resolved_height) {
        layouter.parent_bbox = Some(crate::entities::layout_types::Rect {
            x: block_origin_x, y: Pt(start_y + outset_top), w: Pt(w_pt), h: Pt(h_pt),
        });
    }
    layouter.layout_content(body);
    layouter.flush_line();
    layouter.parent_bbox = saved_parent_bbox;
    let same_region = layouter.pages.len() == page_before && layouter.current_column == column_before;
    if same_region {
        let causal_h = (layouter.flow_placement_y().unwrap_or(inner_origin_y) - inner_origin_y).max(0.0);
        let allocated_h = resolved_height.unwrap_or(causal_h + inset_top + inset_bottom);
        let inner_h = (allocated_h - inset_top - inset_bottom).max(0.0);
        let pending = layouter.pending_align_v_centering.split_off(pending_y_before);
        for (index, (path, count, align, content_h, origin_y, dy, applied_y)) in pending.into_iter().enumerate() {
            let available = (inner_origin_y + inner_h - origin_y).max(0.0);
            let (_, target) = layouter.resolve_alignment(align, 0.0, content_h, 0.0, available, 0.0, origin_y);
            let delta = target + dy - applied_y;
            if let Some(items) = super::helpers::resolve_path_slice(&mut layouter.regions.current.current_items, &path, count) {
                for item in items { super::helpers::shift_frame_item_y(item, delta); }
            }
            if let Some(segments) = layouter.decoration_lines_collector.as_mut() {
                for &(entry, first, end) in &layouter.block_place_decorations {
                    if entry == pending_y_before + index {
                        for segment in &mut segments[first..end] { segment.baseline_y += Pt(delta); }
                    }
                }
            }
        }
        if *clip {
            let pos = crate::entities::layout_types::Point { x: block_origin_x, y: Pt(start_y + outset_top) };
            let mut body_items: Vec<FrameItem> = layouter.regions.current.current_items.drain(items_before..).collect();
            for item in &mut body_items {
                super::helpers::shift_frame_item_x(item, -pos.x.0);
                super::helpers::shift_frame_item_y(item, -pos.y.0);
            }
            for entry in &mut layouter.pending_align_centering[pending_x_before..] {
                entry.0[0] -= items_before;
                entry.0.insert(0, items_before);
            }
            let clip_shape = if radius_is_zero { crate::entities::geometry::ShapeKind::Rect } else {
                super::resolve_shape_kind(&crate::entities::geometry::ShapeKind::RoundedRect { radii: *radius }, layouter.style.size)
            };
            layouter.regions.current.current_items.push(FrameItem::Group {
                pos, matrix: crate::entities::layout_types::TransformMatrix::identity(),
                clip_mask: Some(clip_shape), inner_width: outer_w, inner_height: allocated_h, items: body_items,
            });
        }
        layouter.block_place_region = saved_block_region;
        layouter.block_place_decorations = saved_block_decorations;
    }

'''+s[b:]
 s=s.replace('            Some(w_pt) => w_pt + inset_left,','            Some(w_pt) => w_pt,')
 # backing insertion shifts newly generated horizontal paths
 needle='        layouter.regions.current.current_items.insert(\n            items_before,'
 s=s.replace(needle,'''        for entry in &mut layouter.pending_align_centering[pending_x_before..] {
            entry.0[0] += 1;
        }
'''+needle)
 return s
edit('block',block)
print('Implemented eight private consumers after L0 and three RED regressions')
