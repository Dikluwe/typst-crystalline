from pathlib import Path
import json,subprocess,gzip,hashlib,datetime
out=Path('00_nucleo/diagnosticos/block-auto-width-2026-10-14');owners=json.loads((out/'owners.json').read_text())
def edit(name, pairs):
 p=Path('01_core/src/compiler/layout')/(name+'.rs');s=p.read_text()
 for a,b in pairs:
  assert a in s,(name,a[:100]);s=s.replace(a,b,1)
 p.write_text(s)
edit('mod', [('    pub(super) flow_placement_end: Option<(f64, f64)>,','''    pub(super) flow_placement_end: Option<(f64, f64)>,
    /// Logical origin and occupied width of the active flow scope.
    pub(super) flow_frame_extent: (f64, f64),
    /// Causal width returned by the most recent isolated subframe.
    pub(super) last_sub_frame_flow_width: f64,'''),('            flow_placement_end: None,','''            flow_placement_end: None,
            flow_frame_extent: (0.0, 0.0),
            last_sub_frame_flow_width: 0.0,''')])
edit('checkpoint',[('    flow_placement_end: Option<(f64, f64)>,','''    flow_placement_end: Option<(f64, f64)>,
    flow_frame_extent: (f64, f64),
    last_sub_frame_flow_width: f64,'''),('            flow_placement_end: self.flow_placement_end,','''            flow_placement_end: self.flow_placement_end,
            flow_frame_extent: self.flow_frame_extent,
            last_sub_frame_flow_width: self.last_sub_frame_flow_width,'''),('        self.flow_placement_end = checkpoint.flow_placement_end;','''        self.flow_placement_end = checkpoint.flow_placement_end;
        self.flow_frame_extent = checkpoint.flow_frame_extent;
        self.last_sub_frame_flow_width = checkpoint.last_sub_frame_flow_width;''')])
edit('sub_frame',[('        let saved_flow_placement_end = self.flow_placement_end.take();','''        let saved_flow_placement_end = self.flow_placement_end.take();
        let saved_flow_frame_extent =
            std::mem::replace(&mut self.flow_frame_extent, (region.origin_x, 0.0));'''),('        self.last_sub_frame_width = (self.regions.current.cursor_x.0','''        self.last_sub_frame_flow_width = self.flow_frame_extent.1;
        self.last_sub_frame_width = (self.regions.current.cursor_x.0'''),('        self.flow_placement_end = saved_flow_placement_end;','''        self.flow_placement_end = saved_flow_placement_end;
        self.flow_frame_extent = saved_flow_frame_extent;''')])
edit('block',[('    layouter.layout_content(body);\n    layouter.flush_line();','''    let saved_flow_extent = std::mem::replace(
        &mut layouter.flow_frame_extent,
        (layouter.regions.current.line_start_x.0, 0.0),
    );
    layouter.layout_content(body);
    layouter.flush_line();
    let body_width = layouter.flow_frame_extent.1;
    layouter.flow_frame_extent = saved_flow_extent;'''),('    let root_height = same_region.then(|| {','''    let allocated_w = resolved_width.unwrap_or_else(|| {
        if same_region {
            (inset_left + body_width + inset_right).max(0.0)
        } else {
            outer_w
        }
    });
    let root_height = same_region.then(|| {'''),('                inner_width: outer_w,','                inner_width: allocated_w,'),('    // Auto-sized textual frames are positioned as a unit.','''    let mut root_origin_x = block_origin_x;
    // Auto-sized textual frames are positioned as a unit.'''),('        let delta = saved_width - layouter.page_config.margin.right - right;','''        let delta = saved_width - layouter.page_config.margin.right - right;
        root_origin_x += Pt(delta);'''),('''    if let Some(allocated_h) = root_height {
        let pos = crate::entities::layout_types::Point {
            x: block_origin_x,''','''    if let Some(allocated_h) = root_height {
        let pos = crate::entities::layout_types::Point {
            x: root_origin_x,'''),('            inner_width: outer_w,','            inner_width: allocated_w,'),('''        let block_outer_w = match resolved_width {
            Some(w_pt) => w_pt,
            None => saved_width - saved_line_start.0,
        };''','''        let block_outer_w = allocated_w;'''),('    layouter.record_flow_placement_end(end);','''    layouter.record_flow_placement_end(end);
    layouter.record_flow_width_at(block_origin_x.0, allocated_w);''')])
p=Path('01_core/src/compiler/layout/block.rs');s=p.read_text();pos=s.index('\n#[cfg(test)]');s=s[:pos]+'''
impl<M: FontMetrics, S: ImageSizer> Layouter<'_, M, S> {
    /// Record allocated horizontal flow, excluding visual-only overlays.
    pub(super) fn record_flow_width_at(&mut self, origin: f64, width: f64) {
        let extent = origin - self.flow_frame_extent.0 + width;
        if extent.is_finite() {
            self.flow_frame_extent.1 = self.flow_frame_extent.1.max(extent);
        }
    }
}
'''+s[pos:];p.write_text(s)
edit('line_commit',[('''        // Determinar o tamanho máximo de fonte presente nos items da linha e''','''        // Capture intrinsic occupancy before alignment or marginal numbering.
        if had_items {
            let right = super::helpers::line_content_right(
                &self.regions.current.current_line, &self.metrics,
            ).max(literal_line_right(&self.regions.current.current_line));
            self.record_flow_width_at(right, 0.0);
        }

        // Determinar o tamanho máximo de fonte presente nos items da linha e''')])
p=Path('01_core/src/compiler/layout/line_commit.rs');s=p.read_text();pos=s.index('\nimpl<');s=s[:pos]+'''
fn literal_line_right(items: &[FrameItem]) -> f64 {
    items.iter().map(|item| match item {
        FrameItem::TextSeparator { pos, advance, origin, .. }
            if *origin == crate::entities::layout_types::TextSeparatorOrigin::Literal => {
                pos.x.0 + advance.0
            }
        FrameItem::Link { items, .. } => literal_line_right(items),
        FrameItem::Semantic { items, .. } => literal_line_right(items),
        FrameItem::TextReplacement { items, .. } => literal_line_right(items),
        // Other carriers have no terminal literal-space advance.
        _ => 0.0,
    }).fold(0.0_f64, f64::max)
}
'''+s[pos:];p.write_text(s)
edit('shape',[('    layouter.record_flow_placement_end(shape_base.0 + resolved_h);','''    layouter.record_flow_placement_end(shape_base.0 + resolved_h);
    layouter.record_flow_width_at(pos.x.0, resolved_w);''')])
edit('image',[('    let pos = Point {\n        // rationale: P1064 Classe 1A','''    layouter.record_flow_width_at(layouter.regions.current.cursor_x.0, target_w);
    let pos = Point {
        // rationale: P1064 Classe 1A''')])
edit('placement',[('        self.explicit_line_alignment = saved_alignment;','''        let flow_width = self.last_sub_frame_flow_width;
        self.explicit_line_alignment = saved_alignment;'''),('        self.record_flow_placement_end(target_y + sub_h);','''        self.record_flow_placement_end(target_y + sub_h);
        self.record_flow_width_at(origin_x_abs, flow_width);''')])
edit('grid/cell_emission',[('            self.record_flow_placement_end(row_start_y + row_h);','''            self.record_flow_placement_end(row_start_y + row_h);
            let right = col_starts.iter().zip(resolved_widths)
                .map(|(start, width)| start + width)
                .fold(self.flow_frame_extent.0, f64::max);
            self.record_flow_width_at(right, 0.0);''')])
edit('columns',[('    let usable_width = page_width - margins.horizontal();','''    let usable_width = page_width - margins.horizontal();
    layouter.record_flow_width_at(layouter.regions.current.line_start_x.0, usable_width);''')])
edit('stack',[('''            let (item_ascent, max_descent, ink_ascent) =''','''            layouter.record_flow_width_at(base_x, layouter.last_sub_frame_flow_width);
            let (item_ascent, max_descent, ink_ascent) ='''),('        let mut cur_x = base_x;','''        let mut cur_x = base_x;
        let mut flow_width = 0.0;'''),('''            fn first_baseline(items: &[FrameItem]) -> Option<f64> {''','''            if i > 0 && space_pt > 0.0 {
                flow_width += space_pt;
            }
            flow_width += layouter.last_sub_frame_flow_width;
            fn first_baseline(items: &[FrameItem]) -> Option<f64> {'''),('''        layouter.regions.current.cursor_y = Pt(base_y);''','''        layouter.record_flow_width_at(base_x, flow_width);
        layouter.regions.current.cursor_y = Pt(base_y);''')])
subprocess.run(['rustfmt','--edition','2021','--config','skip_children=true']+list(owners),check=True)
