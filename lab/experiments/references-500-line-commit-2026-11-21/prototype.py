"""Retain a uniform tail edge summary under the existing paint witness."""
from pathlib import Path
import sys

root = Path(sys.argv[1]).resolve()
assert str(root).startswith("/dev/shm/")
layout = root / "01_core/src/compiler/layout"

def edit(file, old, new):
    path = layout / file
    source = path.read_text()
    assert source.count(old) == 1, (file, old)
    path.write_text(source.replace(old, new, 1))

paint = "paragraph_paint.rs"
edit(paint, "struct Paint {", """#[derive(Clone, Copy)]
pub(super) struct LabEdges {
    pub(super) first_item: usize,
    pub(super) size: Pt,
    pub(super) count: usize,
    pub(super) top: f64,
    pub(super) bottom: f64,
    pub(super) all_superscript: bool,
}

struct Paint {""")
edit(paint, "    width: Mutex<Option<(super::MetricEpoch, f64)>>,",
     """    width: Mutex<Option<(super::MetricEpoch, f64)>>,
    lab_edges: Mutex<Option<(super::MetricEpoch, LabEdges)>>,""")
edit(paint, "            .checked_add(anchors.len().checked_mul(8)?)?;",
     """            .checked_add(std::mem::size_of::<Mutex<Option<(super::MetricEpoch, LabEdges)>>>())?
            .checked_add(anchors.len().checked_mul(8)?)?;""")
edit(paint, "        width: Mutex::new(None),", """        width: Mutex::new(None),
        lab_edges: Mutex::new(None),""")
edit(paint, "impl CloseTail {", """impl CloseTail {
    pub(super) fn lab_edges(
        &self,
        items: &[FrameItem],
        end_x: Pt,
        metrics: &dyn FontMetrics,
    ) -> Option<(usize, LabEdges, bool)> {
        use crate::entities::layout_types::{SemanticKind, TextSeparatorOrigin};
        let end = self.first.checked_add(self.product.items.len())?;
        if items.len() != end + 1 || self.product.end.0.to_bits() != end_x.0.to_bits() {
            return None;
        }
        let FrameItem::Semantic { kind: SemanticKind::ParbreakBoundary, .. } = &items[end] else {
            return None;
        };
        let epoch = metrics.physical_epoch()?;
        let mut slot = self.product.lab_edges.lock().unwrap();
        if let Some((old, value)) = slot.as_ref().filter(|(old, _)| old.same(&epoch)) {
            if metrics.physical_epoch().is_some_and(|current| old.same(&current)) {
                return Some((self.first, *value, true));
            }
            return None;
        }
        let mut first: Option<(usize, Pt)> = None;
        let mut count = 0;
        let mut top = 0.0_f64;
        let mut bottom = 0.0_f64;
        let mut all_superscript = true;
        // Paint's immutable snapshot is the same ordered range. Refuse mixed
        // sizes and wrappers. Width, geometry, leading and effects stay live.
        for (index, item) in self.product.items.iter().enumerate() {
            let style = match item {
                FrameItem::Text { style, bidi: None, .. } |
                FrameItem::TextSeparator { style, origin: TextSeparatorOrigin::Literal, bidi: None, .. } => style,
                FrameItem::TextSeparator { bidi: None, .. } => continue,
                _ => return None,
            };
            if !style.size.0.is_finite() || style.size.0 <= 0.0 ||
                first.is_some_and(|(_, old)| old.0.to_bits() != style.size.0.to_bits()) {
                return None;
            }
            first.get_or_insert((index, style.size));
            count += 1;
            all_superscript &= style.superscript && !style.subscript;
            let (t, b) = metrics.text_edges(style.size, style);
            if !t.0.is_finite() || !b.0.is_finite() || t.0 < 0.0 || b.0 > 0.0 {
                return None;
            }
            top = top.max(t.0);
            bottom = bottom.min(b.0);
        }
        let (first_item, size) = first?;
        let value = LabEdges { first_item, size, count, top, bottom, all_superscript };
        if !metrics.physical_epoch().is_some_and(|current| epoch.same(&current)) {
            return None;
        }
        *slot = Some((epoch, value));
        Some((self.first, value, false))
    }
""")
commit = "line_commit.rs"
edit(commit, "        let tail_right = tail.filter", "        let tail_right = tail.as_ref().filter")
edit(commit, """        visit_text_styles(
            &self.regions.current.current_line,
            has_inline_formula(&self.regions.current.current_line),""", """        let line = &self.regions.current.current_line;
        let formula = has_inline_formula(line);
        let summary = if had_items && !formula && lab_line_commit::reuse_enabled() {
            tail.as_ref().and_then(|t| t.lab_edges(line, self.regions.current.cursor_x, &self.metrics))
        } else {
            None
        };
        let live = summary.as_ref().map_or(line.as_slice(), |(prefix, _, _)| &line[..*prefix]);
        visit_text_styles(
            live,
            formula,""")
edit(commit, """        let forced_smaller_singleton = force_outer_singleton""", """        if let Some((prefix, summary, hit)) = summary {
            lab_line_commit::reused(self.pages.len()+1, hit, summary.count);
            let size = summary.size;
            mixed_text_sizes |= first_text_size.is_some_and(|first| first != size.0)
                || ((text_count > 0 || summary.count > 1) && size.0 != outer_font_size.0);
            first_text_size = first_text_size.or(Some(size.0));
            if max_text.as_ref().is_none_or(|(max, _)| size.0 > max.0) {
                let style = match &line[prefix + summary.first_item] {
                    FrameItem::Text { style, .. } | FrameItem::TextSeparator { style, .. } => style,
                    _ => unreachable!("paint witness certifies the first text style"),
                };
                max_text = Some((size, style.clone()));
            }
            text_count += summary.count;
            all_superscript &= summary.all_superscript;
            actual_text_top = actual_text_top.max(summary.top);
            actual_text_bottom = actual_text_bottom.min(summary.bottom);
        }
        let forced_smaller_singleton = force_outer_singleton""")
