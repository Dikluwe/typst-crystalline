from pathlib import Path
f=Path('01_core/src/compiler/layout/line_commit.rs');s=f.read_text();idx=s.index("impl<'a, M: FontMetrics, S: ImageSizer> super::Layouter")
s=s[:idx]+'''/// Transparent visual envelopes share line coordinates; transformed groups do not.
fn visit_text_styles(items: &[FrameItem], visit: &mut impl FnMut(&TextStyle)) {
    for item in items {
        if super::html_frame::is_context_marker(item) {
            continue;
        }
        match item {
            FrameItem::Text { style, .. }
            | FrameItem::TextShaped { style, .. }
            | FrameItem::TextSeparator {
                style,
                origin: crate::entities::layout_types::TextSeparatorOrigin::Literal,
                ..
            } => visit(style),
            FrameItem::Link { items, .. }
            | FrameItem::Semantic { items, .. }
            | FrameItem::TextReplacement { items, .. } => visit_text_styles(items, visit),
            FrameItem::TextSeparator { .. }
            | FrameItem::Group { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Line { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => {}
        }
    }
}

''' + s[idx:]
a=s.index('        #[allow(deprecated)]\n        let (max_text, first_text_size');b=s.index('        let forced_smaller_singleton',a)
s=s[:a]+'''        let mut max_text = None::<(Pt, TextStyle)>;
        let mut first_text_size = None::<f64>;
        let mut mixed_text_sizes = false;
        let mut text_count = 0usize;
        let mut actual_text_top = 0.0_f64;
        let mut actual_text_bottom = 0.0_f64;
        visit_text_styles(&self.regions.current.current_line, &mut |style| {
            let size = style.size;
            mixed_text_sizes |= (text_count > 0 && size.0 != outer_font_size.0)
                || first_text_size.is_some_and(|first| size.0 != first);
            first_text_size = first_text_size.or(Some(size.0));
            if max_text.as_ref().is_none_or(|(max, _)| size.0 > max.0) {
                max_text = Some((size, style.clone()));
            }
            text_count += 1;
            let (top, bottom) = self.metrics.text_edges(size, style);
            actual_text_top = actual_text_top.max(top.0);
            actual_text_bottom = actual_text_bottom.min(bottom.0);
        });
''' + s[b:]
a=s.index('        let (current_text_top, current_text_bottom) = if had_items');b=s.index('        let next_text_top',a)
s=s[:a]+'''        let legacy_edge_fallback = !self.geometric_text_lines
            && (forced_smaller_singleton || self.column_mode);
        let (current_text_top, current_text_bottom) = if !had_items {
            (Pt(0.0), Pt(0.0))
        } else if text_count == 0 {
            self.metrics.text_edges(max_font_size, &max_style)
        } else if legacy_edge_fallback {
            let (top, bottom) = self.metrics.text_edges(max_font_size, &max_style);
            (Pt(actual_text_top.max(top.0)), Pt(actual_text_bottom.min(bottom.0)))
        } else {
            (Pt(actual_text_top), Pt(actual_text_bottom))
        };
''' + s[b:]
a=s.index('        let line_extra_ascent = if self.geometric_text_lines');b=s.index('\n        // P1292-D v10',a)
s=s[:a]+'''        let reconcile_text_top = self.geometric_text_lines
            || (text_count > 0 && !self.column_mode && !force_outer_singleton);
        let line_extra_ascent = if reconcile_text_top {
            inline_ascent.max(actual_text_top) - assumed_ascent
        } else if mixed_text_sizes {
            inline_ascent.max(current_text_top.0) - assumed_ascent
        } else {
            inline_ascent.max(actual_text_top) - assumed_ascent
        };
        let mut line_extra_ascent = if reconcile_text_top {
            line_extra_ascent
        } else {
            line_extra_ascent.max(0.0)
        };
''' + s[b:];f.write_text(s)
