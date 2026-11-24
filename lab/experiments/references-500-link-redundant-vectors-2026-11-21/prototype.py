"""Change only remove_redundant in the disposable laboratory source."""
from pathlib import Path
import sys

path, mode = Path(sys.argv[1]), sys.argv[2]
source = path.read_text()
start = source.index("fn remove_redundant(")
end = source.index("fn wrap<", start)
old = source[start:end]
new = """fn clean_link_child(item: &mut FrameItem, target: &LinkTarget) -> bool {
    match item {
        FrameItem::Link { target: inner, .. } => same_target(inner, target),
        FrameItem::Group { items, .. }
        | FrameItem::Semantic { items, .. }
        | FrameItem::TextReplacement { items, .. } => {
            *items = remove_redundant(std::mem::take(items), target);
            false
        }
        FrameItem::Text { .. }
        | FrameItem::TextShaped { .. }
        | FrameItem::TextSeparator { .. }
        | FrameItem::Glyph { .. }
        | FrameItem::Line { .. }
        | FrameItem::Shape { .. }
        | FrameItem::Image { .. } => false,
    }
}

fn promote_link(item: FrameItem, target: &LinkTarget) -> Vec<FrameItem> {
    let FrameItem::Link { items, .. } = item else { unreachable!() };
    remove_redundant(items, target)
}

fn remove_redundant(mut items: Vec<FrameItem>, target: &LinkTarget) -> Vec<FrameItem> {
    let Some(first) = items.iter_mut().position(|item| clean_link_child(item, target)) else {
        return items;
    };

    let mut out = Vec::with_capacity(items.len());
    let mut items = items.into_iter();
    out.extend(items.by_ref().take(first));
    out.extend(promote_link(items.next().expect("first redundant link"), target));
    for mut item in items {
        if clean_link_child(&mut item, target) {
            out.extend(promote_link(item, target));
        } else {
            out.push(item);
        }
    }
    out
}
"""

if mode == "candidate":
    replacement = new
elif mode == "census":
    # The counters belong to a single reduction and never survive the call.
    # The last two slots count allocator requests by Vec capacity transitions.
    stats = """type LinkVectorCounts = [u64; 6];

fn vector_growth(counts: &mut LinkVectorCounts, before: usize, after: usize) {
    if before != after {
        counts[if before == 0 { 4 } else { 5 }] += 1;
    }
}

#[inline(never)]
fn lab_link_vector_counts(counts: &[LinkVectorCounts; 2]) {
    // The reducer only copies an opaque local record. A laboratory preload
    // observes this particular memory copy outside the core.
    let mut record = [0u64; 14];
    record[0] = 0xb27c_e642_54ba_881f;
    record[1] = 0x2710_a95e_bc07_f361;
    record[2..8].copy_from_slice(&counts[0]);
    record[8..].copy_from_slice(&counts[1]);
    let mut sink = [0u64; 14];
    let source = std::hint::black_box(record.as_slice());
    let destination = std::hint::black_box(sink.as_mut_slice());
    destination.copy_from_slice(source);
    std::hint::black_box(sink);
}

fn remove_redundant(items: Vec<FrameItem>, target: &LinkTarget) -> Vec<FrameItem> {
    let mut counts = [[0; 6]; 2];
    // The comparison clone is outside both reducers and their counters.
    let old = original_redundant(items.clone(), target, &mut counts[0]);
    let current = retained_redundant(items, target, &mut counts[1]);
    assert_eq!(old.len(), current.len());
    std::hint::black_box(&old);
    lab_link_vector_counts(&counts);
    current
}

"""
    old = old.replace("remove_redundant(", "original_redundant(")
    old = old.replace("target: &LinkTarget)", "target: &LinkTarget, counts: &mut LinkVectorCounts)")
    old = old.replace("    let mut out = vec![];", "    counts[0] += 1;\n    let mut out = vec![];")
    old = old.replace("    for mut item in items {", "    for mut item in items {\n        counts[1] += 1;")
    old = old.replace("                out.extend(original_redundant(items, target));",
                      """                counts[2] += 1;
                let before = out.capacity();
                out.extend(original_redundant(items, target, counts));
                vector_growth(counts, before, out.capacity());""")
    old = old.replace("std::mem::take(items), target)", "std::mem::take(items), target, counts)")
    old = old.replace("        out.push(item);",
                      """        let before = out.capacity();
        out.push(item);
        vector_growth(counts, before, out.capacity());""")

    new = new.replace("remove_redundant(", "retained_redundant(")
    new = new.replace("target: &LinkTarget)", "target: &LinkTarget, counts: &mut LinkVectorCounts)")
    new = new.replace("std::mem::take(items), target)", "std::mem::take(items), target, counts)")
    new = new.replace("    match item {", "    counts[1] += 1;\n    match item {", 1)
    new = new.replace("    let FrameItem::Link { items, .. }", "    counts[2] += 1;\n    let FrameItem::Link { items, .. }")
    new = new.replace("retained_redundant(items, target)", "retained_redundant(items, target, counts)")
    new = new.replace("    let Some(first)", "    counts[0] += 1;\n    let Some(first)")
    new = new.replace("clean_link_child(item, target)", "clean_link_child(item, target, counts)")
    new = new.replace("        return items;", "        counts[3] += u64::from(items.capacity() != 0);\n        return items;", 1)
    new = new.replace("    let mut out = Vec::with_capacity(items.len());",
                      "    let mut out = Vec::with_capacity(items.len());\n    vector_growth(counts, 0, out.capacity());")
    new = new.replace("    out.extend(promote_link(items.next().expect(\"first redundant link\"), target));",
                      """    let before = out.capacity();
    out.extend(promote_link(items.next().expect("first redundant link"), target, counts));
    vector_growth(counts, before, out.capacity());""")
    new = new.replace("        if clean_link_child(&mut item, target) {",
                      "        let before = out.capacity();\n        if clean_link_child(&mut item, target, counts) {")
    new = new.replace("out.extend(promote_link(item, target));", "out.extend(promote_link(item, target, counts));")
    new = new.replace("            out.push(item);\n        }", "            out.push(item);\n        }\n        vector_growth(counts, before, out.capacity());")
    replacement = stats + old + new
else:
    raise ValueError(mode)
path.write_text(source[:start] + replacement + source[end:])
