"""Instrument only the RAM copy. Keep allocation and clock probes out of the nominal run."""
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
root = Path(sys.argv[1]).resolve()
assert str(root).startswith("/dev/shm/")
layout = root / "01_core/src/compiler/layout"

def replace(file, old, new):
    path = layout / file
    source = path.read_text()
    assert source.count(old) == 1, (file, old)
    path.write_text(source.replace(old, new))

replace("line_commit.rs", "\nuse crate::entities::{",
        (here/"census.rs").read_text()+"\npub(super) use lab_line_commit::dump as lab_dump;\n\nuse crate::entities::{")
anchor = "        let tail = tail.filter(|_| {"
replace("line_commit.rs", anchor, """        let lab_probe = lab_line_commit::enter();
        let lab_page = self.pages.len() + 1;
        self.lab_flush(force_outer_singleton, continued, tail);
        if let Some(probe) = lab_probe {
            probe.finish(lab_page);
        }
    }

    fn lab_flush(
        &mut self,
        force_outer_singleton: bool,
        continued: bool,
        tail: Option<super::paragraph_paint::CloseTail>,
    ) {
""" + anchor)
anchor = "        self.layout_planned_cursor(cursor, body.content(), false);"
replace("page_running_requests.rs", anchor, anchor + "\n        super::line_commit::lab_dump(\"root\");")

anchor = "        let (doc, running) = l.finish_with_running();"
replace("convergence.rs", anchor, anchor + "\n        super::line_commit::lab_dump(\"finish\");")
