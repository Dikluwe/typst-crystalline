"""Apply the probe to a disposable copy of the current product sources."""
from pathlib import Path
import sys

here = Path(__file__).resolve().parent
root = Path(sys.argv[1])
pipeline = root / "03_infra/src/pipeline"


def replace(path, old, new):
    text = path.read_text()
    assert text.count(old) == 1, (path, old)
    path.write_text(text.replace(old, new))


guard = pipeline / "context_stabilization.rs"
replace(guard, "struct LegacyWorldGuard<'a> {", "pub(super) struct LegacyWorldGuard<'a> {\n    watch_probe: bool,")
replace(guard, "Self { inner, source: None,", "Self { watch_probe: false, inner, source: None,")
replace(guard, "    fn observed_source(&self)", """    pub(super) fn for_watch_probe(inner: &'a dyn World, source: &'a Source) -> Self {
        Self { watch_probe: true, ..Self::with_source(inner, source) }
    }
    fn observed_source(&self)""")
replace(guard, "    fn sealed(&self) -> bool", "    pub(super) fn sealed(&self) -> bool")
replace(guard, "Option<Source> { self.inner.immutable_source(id) }", """Option<Source> {
        if self.watch_probe && self.source.is_none_or(|source| source.id() != id) {
            self.poison();
        }
        self.inner.immutable_source(id)
    }""")
replace(guard, "SysInputs { self.poison(); self.inner.inputs() }", """SysInputs {
        if !self.watch_probe { self.poison(); }
        self.inner.inputs()
    }""")

pdf = pipeline / "pdf.rs"
replace(pdf, "    let (doc_result, mut warnings) = super::paged::compile_paged_in_session(\n        world, source, full_error, features, timings, session,\n    );\n    let doc_result = doc_result.map(|compiled| compiled.document);", """    let (doc_result, mut warnings) =
        watch_probe_paged(world, source, full_error, features, timings, session);""")
replace(pdf, "    // **P1468/P1469** — resolve `pages`", """    if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {
        let mut labels = doc.extracted_label_pages.iter().collect::<Vec<_>>();
        labels.sort_by(|(a, _), (b, _)| a.0.cmp(&b.0));
        let coordinates: Vec<_> = labels.iter().map(|(label, page)| {
            (label.0.as_str(), **page, doc.extracted_label_positions.get(label))
        }).collect();
        eprintln!("watch-positions {} {:?}", doc.extracted_positions.len(), coordinates);
    }
    // **P1468/P1469** — resolve `pages`""")
replace(pdf, "pub struct PdfCompileSession {", """pub struct PdfCompileSession {
    watch_probe: std::sync::Mutex<WatchProbeState>,""")
replace(pdf, "struct PdfCompileRequest<'a> {", (here / "watch_probe.inc.rs").read_text() + "\nstruct PdfCompileRequest<'a> {")

# Count memo outcomes and generations in L3 without adding I/O to L1.
running = pipeline / "context_stabilization/page_running.rs"
replace(running, "            let outcome = if !replay && self.bundle_html_targets.is_none() {", """            let trace_probe = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
            let before = trace_probe.then(|| self.running.physical.stats());
            let outcome = if !replay && self.bundle_html_targets.is_none() {""")
replace(running, "            match &outcome {", """            if let Some(before) = before {
                let after = self.running.physical.stats();
                eprintln!("watch-memo gen={} replay={} hits={} builds={} refusals={} {:?}",
                    self.running.generation, replay, after.0 - before.0,
                    after.1 - before.1, after.2 - before.2, self.running.physical.refusals());
            }
            match &outcome {""")

# Keep instrumentation in L3 and place all measured scopes outside one another.
export = root / "03_infra/src/export"
(export / "lab_pdf_cpu.rs").write_text((here / "pdf_cpu.rs").read_text())
p = export / "mod.rs"
replace(p, "mod builder;", "mod builder;\npub(crate) mod lab_pdf_cpu;")
m = "crate::export::lab_pdf_cpu"
replace(pdf, "    // **P1468/P1469** — resolve `pages`", f"""    let _pdf_cpu = {m}::Publication::begin();
    // **P1468/P1469** — resolve `pages`""")
replace(pdf, """    let font_combos = collect_fonts_from_doc(&doc, world);
    let resolved = bind_owned_pdf_fonts(&doc, resolve_fonts(&font_combos, world));""",
    f"""    let font_combos = {{
        let _span = {m}::scope({m}::Stage::FontDiscover);
        collect_fonts_from_doc(&doc, world)
    }};
    let resolved = {{
        let _span = {m}::scope({m}::Stage::FontResolveBind);
        bind_owned_pdf_fonts(&doc, resolve_fonts(&font_combos, world))
    }};""")

p = export / "builder/fonts.rs"
replace(p, "        let bound_views = crate::export::font_binding::partition(doc, fonts);",
    f"""        let bound_views = {{
            let _span = {m}::scope({m}::Stage::FontPartition);
            crate::export::font_binding::partition(doc, fonts)
        }};""")
replace(p, """            let view = &bound_views[per_font_mappings.len()];
            let mut chars = collect_codepoints(view);""", f"""            let view = &bound_views[per_font_mappings.len()];
            let font_scan = {m}::scope({m}::Stage::FontScan);
            let mut chars = collect_codepoints(view);""")
replace(p, "            let shaped_cluster_texts = collect_shaped_cluster_texts(view);",
    """            let shaped_cluster_texts = collect_shaped_cluster_texts(view);
            drop(font_scan);""")
replace(p, "            let face_glyph_ids: BTreeSet<u16> =\n                collect_glyph_ids(&bound_views[font_index])\n                    .into_iter()\n                    .filter(|&gid| gid < n_glyphs)\n                    .collect();",
    f"""            let face_glyph_ids: BTreeSet<u16> = {{
                let _span = {m}::scope({m}::Stage::FontScan);
                collect_glyph_ids(&bound_views[font_index])
                    .into_iter()
                    .filter(|&gid| gid < n_glyphs)
                    .collect()
            }};""")
s = p.read_text()
# Shared across font paths, including bitmap eligibility; this document
# takes the multifont branch.
s = s.replace("let per_font_used = per_font_used_glyphs(doc, fonts, faces);",
    f"""let per_font_used = {{
            let _span = {m}::scope({m}::Stage::FontScan);
            per_font_used_glyphs(doc, fonts, faces)
        }};""")
s = s.replace("let subset = crate::export::subset::subset_font_with_variations(",
              f"let subset_cpu = {m}::scope({m}::Stage::FontSubset);\n            let subset = crate::export::subset::subset_font_with_variations(")
s = s.replace("            self.subset_ms += duration_ms(t0.elapsed());\n            let (embed_data, glyph_mapping, instancing_applied)", "            drop(subset_cpu);\n            self.subset_ms += duration_ms(t0.elapsed());\n            let (embed_data, glyph_mapping, instancing_applied)")
# All the target document streams go through these scopes in the same
# implementation path; the helpers themselves need no changes in semantics.
p.write_text(s)

p = export / "stream/page.rs"
replace(p, "    let mut ops = String::new();",
        f"    let _span = {m}::scope({m}::Stage::PageOperators);\n    let mut ops = String::new();")
p = export / "builder/streams.rs"
replace(p, "        match self.serialization {",
        f"        let _span = {m}::scope({m}::Stage::PageCompression);\n        match self.serialization {{")
p = export / "stream.rs"
replace(p, "    pub(crate) fn with_pdf_tags(mut self, tags: PdfTags, page: &Page, ua1: bool) -> Self {",
    f"""    pub(crate) fn with_pdf_tags(mut self, tags: PdfTags, page: &Page, ua1: bool) -> Self {{
        let _span = {m}::scope({m}::Stage::TagMcids);""")
p = export / "builder/metadata.rs"
replace(p, "    pub(super) fn emit_tag_structure(&mut self, doc: &PagedDocument) {",
    f"""    pub(super) fn emit_tag_structure(&mut self, doc: &PagedDocument) {{
        let _span = {m}::scope({m}::Stage::TagTree);""")
p = export / "builder/serialization.rs"
replace(p, "    pub(super) fn serialize(self) -> Vec<u8> {",
    f"    pub(super) fn serialize(self) -> Vec<u8> {{\n        let _span = {m}::scope({m}::Stage::Serialize);")
for pathname, header in (
    ("builder/fonts.rs", "fn build_font_stream(stream_subtype: &str, font_stream_data: &[u8]) -> Vec<u8> {"),
    ("builder/fonts.rs", "fn build_cidset_stream(cids: &HashSet<u16>) -> Vec<u8> {"),
    ("fonts.rs", "pub(super) fn to_unicode_cmap(mappings: &[(u16, String)]) -> Vec<u8> {"),
    ("images/icc.rs", "pub(in crate::export) fn build_icc_profile_stream(icc_data: &[u8]) -> Vec<u8> {"),
):
    replace(export / pathname, header, header + f"\n    let _span = {m}::scope({m}::Stage::ResourceStreams);")
