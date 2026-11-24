"""Retain one material item per line after all native break decisions."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
layout = Path(sys.argv[1]) / "01_core/src/compiler/layout"
emit = layout / "paragraph_emit.rs"
line = layout / "line_commit.rs"
original_emit = emit.read_bytes()
original_line = line.read_text()
runpy.run_path(str(here / "prototype-first.py"))
emit.write_bytes(original_emit)
anchor = """        for item in self.regions.current.current_line.drain(..) {
            self.regions.current.current_items.push(item);
        }"""
assert original_line.count(anchor) == 1
line.write_text(original_line.replace(anchor, """        let compact = self.lab_geometry_only && !self.is_sub_frame
            && self.link_capture.active.is_empty()
            && self.regions.current.current_line.iter()
                .all(|item| !matches!(item, FrameItem::Link { .. }))
            && self.regions.current.current_line.iter()
                .any(|item| matches!(item, FrameItem::Text { text, style, .. }
                                    if !text.is_empty() && style.heading_level.is_none()));
        let mut material = false;
        let mut reduced = false;
        for item in self.regions.current.current_line.drain(..) {
            if compact {
                match &item {
                    FrameItem::Text { text, style, .. }
                        if !text.is_empty() && style.heading_level.is_none() => {
                        if material {
                            reduced = true;
                            continue;
                        }
                        material = true;
                    }
                    FrameItem::TextSeparator { .. } => {
                        reduced = true;
                        continue;
                    }
                    _ => {}
                }
            }
            self.regions.current.current_items.push(item);
        }
        self.lab_geometry_lines += usize::from(reduced);"""))
module = layout / "mod.rs"
source = module.read_text()
source = source.replace("    pub(super) lab_line_right: Option<f64>,\n", "")
source = source.replace("            lab_line_right: None,\n", "")
module.write_text(source)
