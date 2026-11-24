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

export = root / "03_infra/src/export"
(export / "lab_pdf_reuse.rs").write_text((here / "pdf_reuse.rs").read_text())
m = "crate::export::lab_pdf_reuse"
def replace_after(path, anchor, old, new):
    text = path.read_text()
    index = text.index(anchor)
    prefix, body = text[:index], text[index:]
    assert old in body, (path, old)
    path.write_text(prefix + body.replace(old,new,1))

p = export / "mod.rs"
replace(p, "mod builder;", "mod builder;\npub(crate) mod lab_pdf_reuse;")
replace_after(p, "pub fn export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(", """    let bound_fonts = font_binding::bind_document(doc, fonts);
    let fonts = bound_fonts.as_ref();
    if fonts.is_empty() {""",
"""    lab_multifont(doc, fonts, document_id, stream_mode, pdf_tags, serialization, pages, standard, None)
}

pub(crate) fn lab_multifont(
    doc: &PagedDocument,
    fonts: &[FontKey],
    document_id: Option<[u8; 16]>,
    stream_mode: StreamMode,
    pdf_tags: PdfTags,
    serialization: PdfSerializationMode,
    pages: Option<PageRanges>,
    standard: PdfStandards,
    mut parts: Option<&mut lab_pdf_reuse::Parts>,
) -> PdfExport<(Vec<u8>, f64)> {
    let saved = parts.as_ref().and_then(|parts| parts.bound.clone());
    let bound_fonts = if let Some(saved) = &saved {
        std::borrow::Cow::Borrowed(saved.as_slice())
    } else {
        font_binding::bind_document(doc, fonts)
    };
    let fonts = bound_fonts.as_ref();
    if let Some(parts) = &mut parts {
        parts.bind_basis(fonts);
        if parts.mode.fonts && parts.bound.is_none() {
            parts.bound = Some(std::sync::Arc::new(fonts.to_vec()));
        }
    }
    if fonts.is_empty() {""")
replace(p, """    builder.build_multifont(doc, fonts, &faces)
}

/// **P980**""", """    builder.build_multifont_with_reuse(doc, fonts, &faces, parts)
}

/// **P980**""")

replace(pdf, "    watch_probe: std::sync::Mutex<WatchProbeState>,",
        f"    watch_probe: std::sync::Mutex<WatchProbeState>,\n    export_probe: std::sync::Mutex<{m}::Cache>,")
replace(pdf, """    let font_combos = collect_fonts_from_doc(&doc, world);
    let resolved = bind_owned_pdf_fonts(&doc, resolve_fonts(&font_combos, world));""",
f"""    let mode = {m}::Mode::from_env();
    let mut watch_export = session.filter(|_| mode.fonts || mode.operators)
        .map(|session| session.export_probe.lock().unwrap());
    let epoch = world.font_query_epoch();
    let mut parts = watch_export.as_mut().and_then(|cache| {{
        if oracle || pages.is_some() || pdf_standards != PdfStandards::default() {{
            cache.clear();
            return None;
        }}
        cache.select(&doc, epoch.clone()?, {m}::Key(stream_mode, pdf_tags, serialization, pdf_standards), mode)
    }});
    let saved = parts.as_ref().and_then(|parts| parts.resolved.clone());
    let had_saved = saved.is_some();
    let resolve = || {{
        let font_combos = collect_fonts_from_doc(&doc, world);
        std::sync::Arc::new(bind_owned_pdf_fonts(&doc, resolve_fonts(&font_combos, world)))
    }};
    let mut resolved = saved.unwrap_or_else(&resolve);
    let stable_fonts = epoch.as_ref().is_some_and(|epoch|
        world.font_query_epoch().is_some_and(|now| std::sync::Arc::ptr_eq(epoch, &now)));
    if !stable_fonts || resolved.len() < 2 {{
        parts = None;
        if let Some(cache) = watch_export.as_mut() {{ cache.clear(); }}
        if !stable_fonts && had_saved {{ resolved = resolve(); }}
    }} else if let Some(parts) = &mut parts {{
        if parts.mode.fonts && resolved.len() <= 16
            && resolved.iter().map(|(_, bytes)| bytes.len()).sum::<usize>() <= {m}::MAX_BYTES
        {{
            parts.resolved = Some(resolved.clone());
        }}
    }}""")
replace(pdf, """            many => export_pdf_multifont_and_timings_and_document_id_and_tags_and_serialization_and_pages_and_standard(
                &doc,""", """            many => crate::export::lab_multifont(
                &doc,""")
replace(pdf, """                pages.clone(),
                pdf_standards,
            ),
        }
    };
    warnings.extend(export.warnings);""",
"""                pages.clone(),
                pdf_standards,
                parts,
            ),
        }
    };
    warnings.extend(export.warnings);""")

p = export / "builder/fonts.rs"
replace(p, """    pub(in crate::export) fn build_multifont(
        mut self,
        doc: &PagedDocument,
        fonts: &[FontKey],
        faces: &[Face<'_>],
    ) -> PdfExport<(Vec<u8>, f64)> {""",
f"""    pub(in crate::export) fn build_multifont(
        self,
        doc: &PagedDocument,
        fonts: &[FontKey],
        faces: &[Face<'_>],
    ) -> PdfExport<(Vec<u8>, f64)> {{
        self.build_multifont_with_reuse(doc, fonts, faces, None)
    }}

    pub(in crate::export) fn build_multifont_with_reuse(
        mut self,
        doc: &PagedDocument,
        fonts: &[FontKey],
        faces: &[Face<'_>],
        mut parts: Option<&mut {m}::Parts>,
    ) -> PdfExport<(Vec<u8>, f64)> {{""")
replace(p, """        let bound_views = crate::export::font_binding::partition(doc, fonts);
        for face in faces {
            let view = &bound_views[per_font_mappings.len()];
            let mut chars = collect_codepoints(view);
            let text_chars = collect_text_codepoints(view);
            chars.extend(text_chars.iter().copied());
            let glyph_ids = collect_glyph_ids(view);
            let shaped_mappings = collect_shaped_glyph_mappings(view);
            let shaped_cluster_texts = collect_shaped_cluster_texts(view);
            let mut mappings = map_chars_to_glyphs(face, &chars);""",
f"""        let saved_scans = parts.as_ref().and_then(|parts| parts.scans.clone());
        let font_hit = saved_scans.is_some();
        let scans = saved_scans.unwrap_or_else(|| {{
            let views = crate::export::font_binding::partition(doc, fonts);
            let scans = views.iter().map(|view| {{
                let mut chars = collect_codepoints(view);
                let text_chars = collect_text_codepoints(view);
                chars.extend(text_chars.iter().copied());
                {m}::FaceScan {{
                    chars, text_chars, glyph_ids: collect_glyph_ids(view),
                    shaped_mappings: collect_shaped_glyph_mappings(view),
                    shaped_cluster_texts: collect_shaped_cluster_texts(view),
                }}
            }}).collect();
            std::sync::Arc::new({m}::FontScans {{
                views, faces: scans, used: per_font_used_glyphs(doc, fonts, faces),
            }})
        }});
        if let Some(parts) = &mut parts {{
            if parts.mode.fonts {{
                parts.scans = Some(scans.clone());
            }}
        }}
        let bound_views = &scans.views;
        for (face, scan) in faces.iter().zip(&scans.faces) {{
            let text_chars = &scan.text_chars;
            let glyph_ids = &scan.glyph_ids;
            let shaped_mappings = &scan.shaped_mappings;
            let shaped_cluster_texts = &scan.shaped_cluster_texts;
            let mut mappings = map_chars_to_glyphs(face, &scan.chars);""")
s=p.read_text()
idx=s.index("    pub(in crate::export) fn build_multifont_with_reuse")
before, after=s[:idx],s[idx:]
for old,new in (
    ("let mut extended_glyph_ids = glyph_ids.clone();", "let mut extended_glyph_ids = (*glyph_ids).clone();"),
    ("for &c in &text_chars {", "for &c in text_chars {"),
    ("for &gid in &glyph_ids {", "for &gid in glyph_ids {"),
    ("for (&old_gid, &ch) in &shaped_mappings {", "for (&old_gid, &ch) in shaped_mappings {"),
    ("for &(old_gid, ref hex) in &shaped_cluster_texts {", "for &(old_gid, ref hex) in shaped_cluster_texts {"),
    ("collect_glyph_ids(&bound_views[font_index])\n                    .into_iter()", """{
                    if parts.as_ref().is_some_and(|parts| parts.mode.fonts) {
                        glyph_ids.clone()
                    } else {
                        collect_glyph_ids(&bound_views[font_index])
                    }
                }.into_iter()"""),
    ("let per_font_used = per_font_used_glyphs(doc, fonts, faces);", "let per_font_used = &scans.used;"),
):
    assert old in after, old
    after=after.replace(old,new,1)
after=after.replace("collect_glyph_ids(&bound_views[fi])\n                    .into_iter()", "scans.faces[fi].glyph_ids.iter().copied()",1)
pos=after.index("        for (i, page) in doc.pages.iter().enumerate() {")
after=after[:pos]+"""        let may_reuse_operators = parts.as_ref().is_some_and(|parts| parts.mode.operators)
            && img_refs.is_empty() && pat_refs.is_empty() && bitmap_res_entries.is_empty()
            && tilings.names.iter().all(HashMap::is_empty);
        if may_reuse_operators {
            if let Some(parts) = &mut parts {
                let unchanged = parts.operator_maps.as_ref().is_some_and(|maps|
                    maps.character == per_font_char_to_gid && maps.remap == per_font_glyph_mapping
                        && maps.nominal == per_font_glyph_to_nominal && maps.reverse == per_font_glyph_reverse);
                if !unchanged {
                    parts.operators.clear();
                    parts.operator_bytes = 0;
                    parts.operator_maps = Some(crate::export::lab_pdf_reuse::OperatorMaps {
                        character: per_font_char_to_gid.clone(), remap: per_font_glyph_mapping.clone(),
                        nominal: per_font_glyph_to_nominal.clone(), reverse: per_font_glyph_reverse.clone(),
                    });
                }
            }
        }
        let mut operator_hits = 0;
"""+after[pos:]
pos=after.index("            let mut ctx = PageContext::with_font_scenario(")
after=after[:pos]+"""            if may_reuse_operators {
                if let Some(saved) = parts.as_ref().and_then(|parts| parts.operators.get(i))
                    .and_then(Option::as_ref)
                {
                    let stream_object = self.serialize_page_stream(saved);
                    self.add_bytes(stream_id, stream_object);
                    operator_hits += 1;
                    continue;
                }
            }
"""+after[pos:]
old="""            self.add_bytes(stream_id, stream_object);
        }

        // Emit objectos por font"""
new=f"""            self.add_bytes(stream_id, stream_object);
            if may_reuse_operators {{
                if let Some(parts) = &mut parts {{
                    if parts.mode.operators && stream_bytes.len() <= {m}::MAX_BYTES - parts.operator_bytes {{
                        parts.operators.resize_with(doc.pages.len(), || None);
                        parts.operator_bytes += stream_bytes.len();
                        parts.operators[i] = Some(stream_bytes.into());
                    }} else {{
                        parts.mode.operators = false;
                        parts.operators.clear();
                        parts.operator_bytes = 0;
                    }}
                }}
            }}
        }}
        if std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some() {{
            eprintln!("pdf-reuse fonts={{font_hit}} operators={{operator_hits}}/{{n_pages}} eligible={{}}",
                parts.is_some());
        }}

        // Emit objectos por font"""
assert old in after
after=after.replace(old,new,1)
p.write_text(before+after)
