"""Try bounded whole-line replay after actual regional and baseline admission.

This is an experimental source transform for a RAM checkout, not an L0 change.
"""
from pathlib import Path
import sys

root = Path(sys.argv[1]).resolve()
assert str(root).startswith("/dev/shm/")
paint = root / "01_core/src/compiler/layout/paragraph_paint.rs"
source = paint.read_text()

def edit(old, new):
    global source
    assert source.count(old) == 1, old
    source = source.replace(old, new, 1)

edit("""    line: usize,
    numeric: [u64; 4],
}""", """    line: usize,
    whole: bool,
    numeric: [u64; 4],
}""")
anchor = """    let baseline = l.regions.current.cursor_y;
    let numeric = ["""
assert source.count(anchor) == 1
source = source.replace(anchor, """    paint_with_close(l, identity, line, spacing, language, lend, false, emit)
}

pub(super) fn whole_with_close<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    identity: &Arc<()>,
    line: usize,
    spacing: [f64; 2],
    language: Option<Lang>,
    lend: bool,
    emit: impl FnOnce(&mut Layouter<M, S>),
) -> Option<CloseTail> {
    paint_with_close(l, identity, line, spacing, language, lend, true, emit)
}

fn paint_with_close<M: FontMetrics, S: ImageSizer>(
    l: &mut Layouter<M, S>,
    identity: &Arc<()>,
    line: usize,
    spacing: [f64; 2],
    language: Option<Lang>,
    lend: bool,
    whole: bool,
    emit: impl FnOnce(&mut Layouter<M, S>),
) -> Option<CloseTail> {
    let baseline = l.regions.current.cursor_y;
    let numeric = [""", 1)
edit("""        && super::html_frame::items_have_material(&l.regions.current.current_line);""", """        && if whole {
            !super::html_frame::items_have_material(&l.regions.current.current_line)
        } else {
            super::html_frame::items_have_material(&l.regions.current.current_line)
        };""")
edit("""        line,
        numeric: numeric.map(f64::to_bits),""", """        line,
        whole,
        numeric: numeric.map(f64::to_bits),""")
edit("""    let Some(session) = session.filter(|_| eligible) else {""", """    if whole && std::env::var_os("TEKT_LAB_WHOLE_TRACE").is_some() && !eligible {
        eprintln!("lab-whole-line refused initial={} block={} origin={} prefix={} escape={} bidi={} material={}",
            l.initial_baseline_pending, l.block_chain_active,
            super::paragraph_indent::origin_applied(l),
            l.pending_bib_prefix.is_some(), l.pending_bib_escape.is_some(),
            l.text_bidi.is_some(),
            super::html_frame::items_have_material(&l.regions.current.current_line));
    }
    let Some(session) = session.filter(|_| eligible) else {""")
edit("""        if product.is_some() {
            m.hits += 1;
        }""", """        if product.is_some() {
            m.hits += 1;
            if whole && std::env::var_os("TEKT_LAB_WHOLE_TRACE").is_some() {
                eprintln!("lab-whole-line hit");
            }
        }""")
edit("""    m.published += 1;
    m.order.push_back(key);""", """    m.published += 1;
    if whole && std::env::var_os("TEKT_LAB_WHOLE_TRACE").is_some() {
        eprintln!("lab-whole-line published");
    }
    m.order.push_back(key);""")
paint.write_text(source)

emit = root / "01_core/src/compiler/layout/paragraph_emit.rs"
source = emit.read_text()
anchor = """    let lab_page = l.pages.len() + 1;
    let lab_start = std::time::Instant::now();"""
if anchor in source:
    source = source.replace(anchor, """    if std::env::var_os("TEKT_LAB_WHOLE_TRACE").is_some() {
        eprintln!("lab-whole-input identity={} first={} page={}",
            identity.is_some(), line.start < line.end, l.pages.len() + 1);
    }
""" + anchor, 1)
start = source.index("    let next = if line.start < line.end {")
end = source.index("    if continued {\n        l.flush_line();", start)
old = source[start:end]
source = source[:start] + """    // Admit the first real fragment exactly where ordinary emission would.
    // The certificate rejects leading gaps and nonempty current lines.
    let whole = identity.filter(|_| line.start < line.end)
        .filter(|_| line.start > 0 || p.words[line.start].before.is_empty())
        .and_then(|_| p.words[line.start].fragments.first());
    let tail = if let (Some((id, index)), Some(first)) = (identity, whole) {
        let outer = std::mem::replace(&mut l.style, first.style.clone());
        l.ensure_text_baseline();
        super::paragraph_indent::admit(l);
        l.style = outer;
        let language = p.words[line.start..line.end].iter()
            .flat_map(|word| &word.fragments).find_map(|fragment| fragment.style.lang);
        super::paragraph_paint::whole_with_close(
            l, id, index, [proportional, extra], language, lend && !continued,
            |l| {
                for i in line.start..line.end {
                    emit_word(l, p, line, i, proportional, extra);
                }
                let trailing = if line.end < p.words.len() {
                    &p.words[line.end].before
                } else {
                    &p.trailing
                };
                for gap in trailing {
                    l.regions.current.cursor_x += Pt(p.tracking());
                    emit_gap(l, gap, gap.width);
                }
            },
        )
    } else {
""" + "".join("    " + line for line in old.splitlines(keepends=True)) + """        tail
    };
""" + source[end:]
emit.write_text(source)
