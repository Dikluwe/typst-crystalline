"""Apply the font-page probe only to the disposable compiler sources."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
runpy.run_path(str(here.parent / "references-500-visible-page-operators-2026-11-21" / "prototype.py"))
root = Path(sys.argv[1]) / "03_infra/src/export"
cache, binding, builder = (root/name for name in ("lab_pdf_reuse.rs", "font_binding.rs", "builder/fonts.rs"))


def edit(path, before, after):
    text = path.read_text()
    assert text.count(before) == 1, (path, before)
    path.write_text(text.replace(before, after))


edit(cache, "pub(crate) struct FaceScan {", "#[derive(PartialEq, Eq)]\npub(crate) struct FaceScan {")
edit(cache, "    pub scans: Option<Arc<FontScans>>,", """    pub scans: Option<Arc<FontScans>>,
    pub page_fonts: Vec<Arc<PageFontScan>>,""")
edit(cache, "            self.scans = None;", "            self.scans = None;\n            self.page_fonts.clear();")
edit(cache, "                parts.basis = old.parts.basis;", """                parts.basis = old.parts.basis;
                parts.page_fonts = old.parts.page_fonts;""")
with cache.open("a") as f:
    f.write("\n" + (here / "page_fonts.rs").read_text())

edit(binding, """pub(super) fn partition(doc: &PagedDocument, fonts: &[FontKey]) -> Vec<PagedDocument> {
    let Some(first) = doc.pages.first() else {""", """pub(super) fn partition(doc: &PagedDocument, fonts: &[FontKey]) -> Vec<PagedDocument> {
    partition_pages(&doc.pages, fonts)
}

pub(super) fn partition_pages(
    pages: &[typst_core::entities::layout_types::Page],
    fonts: &[FontKey],
) -> Vec<PagedDocument> {
    let Some(first) = pages.first() else {""")
edit(binding, """    for page in &doc.pages {
        walk_frame_items(&mut visitor, &page.background);
        walk_frame_items(&mut visitor, &page.items);
        walk_frame_items(&mut visitor, &page.foreground);
    }
    views""", """    for page in pages {
        walk_frame_items(&mut visitor, &page.background);
        walk_frame_items(&mut visitor, &page.items);
        walk_frame_items(&mut visitor, &page.foreground);
    }
    views""")

edit(builder, "use typst_core::entities::layout_types::FrameItem;", """use typst_core::entities::layout_types::{FrameItem, Page};
use crate::export::lab_pdf_reuse::{self as reuse, FaceScan, FontScans, PageFontScan};
use std::sync::Arc;""")
edit(builder, """fn per_font_used_glyphs(
    doc: &PagedDocument,""", """fn per_font_used_glyphs(
    pages: &[Page],""")
edit(builder, """    for page in &doc.pages {
        for items in [&page.background, &page.items, &page.foreground] {""", """    for page in pages {
        for items in [&page.background, &page.items, &page.foreground] {""")
edit(builder, "fn subset_font_name(", (here / "page_fonts_builder.rs").read_text() + "\nfn subset_font_name(")
text = builder.read_text()
start = text.index("        let saved_scans = ")
end = text.index("        let bound_views = &scans.views;", start)
builder.write_text(text[:start] + (here / "scans.inc.rs").read_text() + text[end:])
edit(builder, """                previous_document.as_ref().is_none_or(|old| old.pages.get(i).is_some_and(|before|
                    crate::export::lab_pdf_reuse::same_page(before, page)))""", """                same_pages.as_ref().map_or_else(
                    || previous_document.as_ref().is_none_or(|old| old.pages.get(i).is_some_and(|before|
                        reuse::same_page(before, page))),
                    |same| same[i],
                )""")
