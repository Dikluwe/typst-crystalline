from pathlib import Path
base=Path('01_core/src/compiler/layout')
def edit(name,old,new,count=1):
 p=base/(name+'.rs');s=p.read_text();assert s.count(old)==count,(name,old,s.count(old));p.write_text(s.replace(old,new))
edit('mod','    pub(super) block_place_region: Option<crate::entities::layout_types::Rect>,','    pub(super) block_place_region: Option<crate::entities::layout_types::Rect>,\n    /// Available width is a constraint; the current Block allocation is intrinsic.\n    pub(super) block_auto_width: bool,')
edit('mod','            block_place_region: None,','            block_place_region: None,\n            block_auto_width: false,')
edit('mod','        self.block_place_region = None;','        self.block_place_region = None;\n        self.block_auto_width = false;')
edit('checkpoint','    block_place_region: Option<crate::entities::layout_types::Rect>,','    block_place_region: Option<crate::entities::layout_types::Rect>,\n    block_auto_width: bool,')
edit('checkpoint','            block_place_region: self.block_place_region,','            block_place_region: self.block_place_region,\n            block_auto_width: self.block_auto_width,')
edit('checkpoint','        self.block_place_region = checkpoint.block_place_region;','        self.block_place_region = checkpoint.block_place_region;\n        self.block_auto_width = checkpoint.block_auto_width;')
edit('sub_frame','        let saved_block_place_region = self.block_place_region.take();','        let saved_block_place_region = self.block_place_region.take();\n        let saved_block_auto_width = std::mem::take(&mut self.block_auto_width);')
edit('sub_frame','        self.block_place_region = saved_block_place_region;','        self.block_place_region = saved_block_place_region;\n        self.block_auto_width = saved_block_auto_width;')
edit('block','    layouter.layout_content(body);\n    layouter.flush_line();','    let saved_block_auto_width =\n        std::mem::replace(&mut layouter.block_auto_width, resolved_width.is_none());\n    layouter.layout_content(body);\n    layouter.flush_line();\n    layouter.block_auto_width = saved_block_auto_width;')
edit('placement','        let avail_w_is_finite = avail_w.is_finite();','        let defer_block_x = self.block_auto_width && !flow_aligned;\n        let avail_w_is_finite = avail_w.is_finite() && !defer_block_x;')
old='    if let Some(allocated_h) = root_height {\n        let inner_h'
new='''    if let Some(allocated_h) = root_height {
        // Resolve local alignment before introducing clipping/root coordinates.
        // The body was laid out once under the available wrapping constraint.
        let inner_right = block_origin_x.0 + allocated_w - inset_right;
        let pending = layouter.pending_align_centering.split_off(pending_x_before);
        for (path, count, align, content_w, origin_x, applied_x) in pending {
            let available = (inner_right - origin_x).max(0.0);
            let (target, _) = layouter.resolve_alignment(
                align, content_w, 0.0, available, 0.0, origin_x, 0.0,
            );
            if let Some(items) = super::helpers::resolve_path_slice(
                &mut layouter.regions.current.current_items, &path, count,
            ) {
                for item in items {
                    super::helpers::shift_frame_item_x(item, target - applied_x);
                }
            }
        }
        let inner_h'''
edit('block',old,new)
