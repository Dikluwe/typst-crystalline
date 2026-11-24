"""Instrument only a disposable RAM source tree for per-page line emission."""
from pathlib import Path
import sys

root = Path(sys.argv[1]).resolve()
assert str(root).startswith("/dev/shm/")
emit = root / "01_core/src/compiler/layout/paragraph_emit.rs"
source = emit.read_text()

def replace(old, new):
    global source
    assert source.count(old) == 1, old
    source = source.replace(old, new, 1)

replace("fn emit_gap<M: FontMetrics, S: ImageSizer>(", """thread_local! {
    static LAB_LINES: std::cell::RefCell<Vec<(usize, u64)>> = const {
        std::cell::RefCell::new(Vec::new())
    };
}
pub(super) fn lab_take_lines() -> Vec<(usize, u64)> {
    LAB_LINES.with(|lines| std::mem::take(&mut *lines.borrow_mut()))
}
fn emit_gap<M: FontMetrics, S: ImageSizer>(""")
replace("""    let width = super::paragraph_indent::current_width(l, width);
    let m = measure(p, line);""", """    let lab_page = l.pages.len() + 1;
    let lab_start = std::time::Instant::now();
    let width = super::paragraph_indent::current_width(l, width);
    let m = measure(p, line);""")
replace("""    tail
}

fn emit_word""", """    let lab_ns = lab_start.elapsed().as_nanos() as u64;
    LAB_LINES.with(|lines| lines.borrow_mut().push((lab_page, lab_ns)));
    tail
}

fn emit_word""")
emit.write_text(source)
root_file = root / "01_core/src/compiler/layout/page_running_requests.rs"
source = root_file.read_text()
old = """        self.layout_planned_cursor(cursor, body.content(), false);
    }

    pub(super) fn layout_planned_cursor"""
new = """        self.layout_planned_cursor(cursor, body.content(), false);
        if std::env::var_os("TEKT_LAB_EMISSION_CENSUS").is_some() {
            let lines = super::paragraph_emit::lab_take_lines();
            let mut pages = std::collections::BTreeMap::<usize, (usize, u64)>::new();
            for (page, ns) in lines {
                let entry = pages.entry(page).or_default();
                entry.0 += 1;
                entry.1 += ns;
            }
            eprintln!("lab-emission-pages {:?}", pages);
        }
    }

    pub(super) fn layout_planned_cursor"""
assert source.count(old) == 1
root_file.write_text(source.replace(old, new))
