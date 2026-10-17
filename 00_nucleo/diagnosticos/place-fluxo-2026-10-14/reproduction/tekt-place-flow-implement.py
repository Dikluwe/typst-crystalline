from pathlib import Path
import json,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/place-fluxo-2026-10-14');owners=json.loads((out/'owners.json').read_text());paths=list(owners)+list(owners.values())+['00_nucleo/prompts/_nuclei/place-flow-boundary.toml']
with gzip.open(out/'red-state.json.gz','wt') as f:json.dump(dict(utc=datetime.datetime.now(datetime.timezone.utc).isoformat(),files={n:dict(text=Path(n).read_text(),sha256=hashlib.sha256(Path(n).read_bytes()).hexdigest()) for n in paths}),f)
base='01_core/src/compiler/layout/'
def edit(name,old,new,count=1):
 p=Path(base+name+'.rs');s=p.read_text();assert s.count(old)==count,(name,old,s.count(old));p.write_text(s.replace(old,new))
edit('mod','    pub(super) sub_frame_origin_y: f64,','    pub(super) sub_frame_origin_y: f64,\n    /// Physical flow end and the prepared cursor that certifies it.\n    pub(super) flow_placement_end: Option<(f64, f64)>,')
edit('mod','            sub_frame_origin_y: 0.0,','            sub_frame_origin_y: 0.0,\n            flow_placement_end: None,')
edit('mod','        self.initial_baseline_pending = true;','        self.initial_baseline_pending = true;\n        self.flow_placement_end = None;')
edit('checkpoint','    sub_frame_origin_y: f64,','    sub_frame_origin_y: f64,\n    flow_placement_end: Option<(f64, f64)>,')
edit('checkpoint','            sub_frame_origin_y: self.sub_frame_origin_y,','            sub_frame_origin_y: self.sub_frame_origin_y,\n            flow_placement_end: self.flow_placement_end,')
edit('checkpoint','        self.sub_frame_origin_y = checkpoint.sub_frame_origin_y;','        self.sub_frame_origin_y = checkpoint.sub_frame_origin_y;\n        self.flow_placement_end = checkpoint.flow_placement_end;')
edit('sub_frame','        let saved_sub_frame_origin_y = self.sub_frame_origin_y;','        let saved_sub_frame_origin_y = self.sub_frame_origin_y;\n        let saved_flow_placement_end = self.flow_placement_end.take();')
edit('sub_frame','        self.sub_frame_origin_y = saved_sub_frame_origin_y;','        self.sub_frame_origin_y = saved_sub_frame_origin_y;\n        self.flow_placement_end = saved_flow_placement_end;')
p=Path(base+'place.rs');s=p.read_text();a=s.index('        // Omitted vertical alignment');b=s.index('        let item_start',a);s=s[:a]+'''        // Pending lines own their baseline; a committed producer supplies a
        // certified physical end, excluding the prepared next-line advance.
        let flow_y = if effective_alignment.v.is_none() {
            if super::html_frame::items_have_material(
                &layouter.regions.current.current_line,
            ) {
                Some(layouter.regions.current.cursor_y.0)
            } else {
                layouter.flow_placement_y()
            }
        } else {
            None
        };
        let in_sub_frame = layouter.is_sub_frame;
        let pending_line_delta = flow_y.map(|y| y - layouter.sub_frame_origin_y);
        let saved_margin_top = layouter.page_config.margin.top;
        if !in_sub_frame {
            if let Some(y) = flow_y {
                layouter.page_config.margin.top = y;
            }
        }

'''+s[b:];i=s.index('/// P1292-D v10 — admite');s=s[:i]+'''impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    pub(super) fn record_flow_placement_end(&mut self, end: f64) {
        self.flow_placement_end = Some((end, self.regions.current.cursor_y.0));
    }

    pub(super) fn flow_placement_y(&self) -> Option<f64> {
        self.flow_placement_end
            .filter(|&(_, prepared)| prepared == self.regions.current.cursor_y.0)
            .map(|(end, _)| end)
    }
}

'''+s[i:];p.write_text(s)
edit('line_commit','            self.regions.current.cursor_y += advance;','            self.regions.current.cursor_y += advance;\n            self.record_flow_placement_end(baseline_y + line_descent);')
edit('shape','        layouter.regions.current.cursor_y += Pt(resolved_h);\n    }\n}','        layouter.regions.current.cursor_y += Pt(resolved_h);\n    }\n    layouter.record_flow_placement_end(shape_base.0 + resolved_h);\n}')
edit('placement','        if emitted_textual_material && matches!(effective_v, None | Some(VAlign::Top)) {','        self.record_flow_placement_end(target_y + sub_h);\n        if emitted_textual_material && matches!(effective_v, None | Some(VAlign::Top)) {')
edit('grid/cell_emission','            self.regions.current.cursor_y = Pt(row_start_y + row_h);','            self.regions.current.cursor_y = Pt(row_start_y + row_h);\n            self.record_flow_placement_end(row_start_y + row_h);')
edit('raw','        l.initial_baseline_pending = true;\n    }\n}\n\n/// Terminal Raw','        l.initial_baseline_pending = true;\n    }\n    l.record_flow_placement_end(end);\n}\n\n/// Terminal Raw')
edit('v_space','    layouter.regions.current.cursor_y += Pt(pt);','    let end = layouter.flow_placement_y().unwrap_or(layouter.regions.current.cursor_y.0) + pt;\n    layouter.regions.current.cursor_y += Pt(pt);\n    layouter.record_flow_placement_end(end);')
edit('parbreak','        layouter.flush_line();','        layouter.flush_line();\n        let flow_end = layouter.flow_placement_y();')
edit('parbreak','        layouter.prev_margin_is_parbreak = true;','        layouter.prev_margin_is_parbreak = true;\n        if let Some(end) = flow_end {\n            layouter.record_flow_placement_end(end);\n        }')
edit('region_progress','    layouter.last_flush_advance = 0.0;','    layouter.last_flush_advance = 0.0;\n    layouter.flow_placement_end = None;')
edit('block','    let start_y = layouter.regions.current.cursor_y.0;','    let start_y = layouter.regions.current.cursor_y.0;\n    layouter.flow_placement_end = None;')
edit('block','    // 6. Aplica inset bottom.','    let body_flow_end = layouter.flow_placement_y();\n\n    // 6. Aplica inset bottom.')
edit('block','    layouter.prev_margin_is_parbreak = false;\n}','''    layouter.prev_margin_is_parbreak = false;
    let same_region = layouter.pages.len() == page_before
        && layouter.current_column == column_before;
    let end = if same_region {
        resolved_height.map_or_else(
            || body_flow_end.unwrap_or(start_y + outset_top + inset_top)
                + inset_bottom + outset_bottom,
            |height| start_y + outset_top + height + outset_bottom,
        )
    } else {
        body_flow_end.unwrap_or(layouter.regions.current.cursor_y.0)
    };
    layouter.record_flow_placement_end(end);
}''')
