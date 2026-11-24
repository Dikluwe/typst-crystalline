"""Admit the existing inert terminal comment shape for outline stabilization."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
runpy.run_path(str(here.parent / "references-500-outline-hybrid-2026-11-21" / "prototype.py"))
root = Path(sys.argv[1])
pdf = root / "03_infra/src/pipeline/pdf.rs"
running = root / "03_infra/src/pipeline/context_stabilization/page_running.rs"


def replace(path, old, new):
    source = path.read_text()
    assert source.count(old) == 1, (path, old)
    path.write_text(source.replace(old, new))


replace(pdf, """pub(super) fn lab_closed_outline_source(source: &Source) -> bool {
    if !inert_source(source) {""",
"""pub(super) fn lab_closed_outline_source(source: &Source) -> bool {
    if !inert_source_for_outline(source, true) {""")
replace(pdf, """fn inert_source(source: &Source) -> bool {
    let (prefix, tail) = watch_probe_tail(source);
    if !tail.iter().map(watch_probe_tail_signature).eq([(SyntaxKind::Space, true)])
        || prefix.last().is_none_or(|node| inert_loop(node).is_none())
    {""",
"""fn inert_source(source: &Source) -> bool {
    inert_source_for_outline(source, false)
}

fn inert_comment_tail(tail: &[SyntaxNode]) -> bool {
    tail.iter().map(watch_probe_tail_signature).eq([
        (SyntaxKind::Parbreak, false), (SyntaxKind::LineComment, false), (SyntaxKind::Space, true),
    ])
}

fn inert_source_for_outline(source: &Source, allow_comment_tail: bool) -> bool {
    let (prefix, tail) = watch_probe_tail(source);
    let admitted_tail = tail.iter().map(watch_probe_tail_signature).eq([(SyntaxKind::Space, true)])
        || (allow_comment_tail && inert_comment_tail(tail));
    if !admitted_tail || prefix.last().is_none_or(|node| inert_loop(node).is_none()) {""")
replace(pdf, """    tail.iter().map(watch_probe_tail_signature).eq([
        (SyntaxKind::Parbreak, false), (SyntaxKind::LineComment, false), (SyntaxKind::Space, true),
    ]) && prefix.len() == new_prefix.len()""", """    inert_comment_tail(tail) && prefix.len() == new_prefix.len()""")
replace(pdf, """fn inert_terminal_break(document: &PagedDocument) -> bool {""",
"""pub(super) fn lab_outline_terminal_boundary(source: &Source, document: &PagedDocument) -> bool {
    !inert_comment_tail(watch_probe_tail(source).1) || inert_terminal_break(document)
}

fn inert_terminal_break(document: &PagedDocument) -> bool {""")
replace(running, """            self.metrics.clone(), ImageSizeImageSizer, &sealed).map_err(|_| "retained layout")?;
        let mut step""",
"""            self.metrics.clone(), ImageSizeImageSizer, &sealed).map_err(|_| "retained layout")?;
        if !super::super::pdf::lab_outline_terminal_boundary(self.source, &retained.document) {
            return Err("unproved terminal paragraph boundary");
        }
        let mut step""")
