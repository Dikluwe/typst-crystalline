"""Apply stable CIDs and an encoded-stream slot to a disposable lab checkout."""
from pathlib import Path
import runpy
import sys

here = Path(__file__).resolve().parent
runpy.run_path(str(here.parent / "references-500-visible-page-fonts-2026-11-21" / "prototype.py"))
root = Path(sys.argv[1]) / "03_infra/src/export"
cache, subset, builder = (root/name for name in ("lab_pdf_reuse.rs", "subset.rs", "builder/fonts.rs"))


def edit(path, before, after):
    text = path.read_text()
    assert text.count(before) == 1, (path, before)
    path.write_text(text.replace(before, after))


edit(subset, "additional_gids, None)", "additional_gids, None, &[])")
edit(subset, "additional_gids, Some(variations))", "additional_gids, Some(variations), &[])")
edit(subset, "fn subset_font_inner(", """pub(super) fn subset_font_with_prefix(
    font_data: &[u8],
    char_to_old_gid: &BTreeMap<char, u16>,
    additional_gids: &BTreeSet<u16>,
    variations: &[(ttf_parser::Tag, f32)],
    prefix: &[u16],
) -> Option<FontSubset> {
    subset_font_inner(font_data, char_to_old_gid, additional_gids, Some(variations), prefix)
}

fn subset_font_inner(""")
edit(subset, "    variations: Option<&[(ttf_parser::Tag, f32)]>,", """    variations: Option<&[(ttf_parser::Tag, f32)]>,
    prefix: &[u16],""")
edit(subset, "    remapper.remap(0);", """    remapper.remap(0);
    for &gid in prefix {
        remapper.remap(gid);
    }""")
edit(subset, """    Some(FontSubset { data: subset_data, mapping })""", """    for &gid in prefix {
        mapping.insert(gid, remapper.get(gid)?);
    }
    Some(FontSubset { data: subset_data, mapping })""")

edit(cache, "    pub operator_bytes: usize,", """    pub operator_bytes: usize,
    pub streams: Vec<Option<Arc<[u8]>>>,
    pub stream_bytes: usize,""")
edit(cache, "            self.operator_bytes = 0;\n            self.basis", """            self.operator_bytes = 0;
            self.streams.clear();
            self.stream_bytes = 0;
            self.basis""")
edit(cache, "                parts.operator_bytes = old.parts.operator_bytes;", """                parts.operator_bytes = old.parts.operator_bytes;
                parts.streams = old.parts.streams;
                parts.stream_bytes = old.parts.stream_bytes;""")
with cache.open("a") as output:
    output.write("\n" + (here / "stable_cids.rs").read_text())

edit(builder, "        let bound_views = &scans.views;", """        let bound_views = &scans.views;
        let mut seeded_faces = 0;
        let trace_cids = std::env::var_os("TEKT_LAB_WATCH_TRACE").is_some();
        let cid_reference = std::env::var_os("TEKT_LAB_CID_REEMIT").is_some();""")
edit(builder, """            let subset = crate::export::subset::subset_font_with_variations(
                font_bytes,
                &char_to_old_gid,
                &face_glyph_ids,
                &axis_tuples,
            );""", """            let prefix = parts.as_ref().filter(|parts| parts.mode.operators)
                .and_then(|parts| parts.operator_maps.as_ref())
                .and_then(|maps| maps.remap.get(font_index))
                .and_then(reuse::cid_prefix);
            let seeded = prefix.and_then(|prefix| {
                crate::export::subset::subset_font_with_prefix(
                    font_bytes, &char_to_old_gid, &face_glyph_ids, &axis_tuples, &prefix,
                ).filter(|subset| Face::parse(&subset.data, 0).is_ok())
            });
            if seeded.is_some() { seeded_faces += 1; }
            let subset = seeded.or_else(|| crate::export::subset::subset_font_with_variations(
                font_bytes, &char_to_old_gid, &face_glyph_ids, &axis_tuples,
            ));""")
edit(builder, """        let mut saved_operators = Vec::new();""", """        let mut saved_operators = Vec::new();
        let mut saved_streams = Vec::new();
        let mut stable_pages = Vec::new();
        let mut stream_hits = 0;
        let mut serialized_pages = 0;""")
edit(builder, """            parts.operator_bytes = 0;
            if may_reuse_operators {""", """            parts.operator_bytes = 0;
            let prior_streams = std::mem::take(&mut parts.streams);
            parts.stream_bytes = 0;
            if may_reuse_operators {""")
edit(builder, """                if let Some(old) = parts.operator_maps.take() {
                    if old == maps {""", """                if let Some(old) = parts.operator_maps.take() {
                    if !cid_reference {
                        stable_pages = if old == maps {
                            vec![true; doc.pages.len()]
                        } else {
                            parts.page_fonts.iter().map(|page| old.same_page_cids(&maps, page)).collect()
                        };
                        saved_streams = prior_streams;
                    }
                    if old == maps {""")
edit(builder, """                previous_document = previous;
                parts.operator_maps = Some(maps);""", """                if cid_reference { saved_operators.clear(); }
                previous_document = previous;
                parts.operator_maps = Some(maps);""")
edit(builder, """            let saved = saved_operators.get_mut(i).and_then(Option::take).filter(|_| {
                same_pages.as_ref().map_or_else(""", """            let unchanged = || {
                same_pages.as_ref().map_or_else(""")
edit(builder, """                    |same| same[i],
                )
            }).and_then(|cached| match &translation {""", """                    |same| same[i],
                )
            };
            if stable_pages.get(i) == Some(&true) {
                if let Some(encoded) = saved_streams.get_mut(i).and_then(Option::take)
                    .filter(|_| unchanged())
                {
                    self.add_bytes(stream_id, encoded.to_vec());
                    stream_hits += 1;
                    operator_hits += 1;
                    if let Some(parts) = &mut parts {
                        parts.store_stream(i, doc.pages.len(), encoded);
                        if let Some(raw) = saved_operators.get_mut(i).and_then(Option::take) {
                            parts.store_operators(i, doc.pages.len(), raw);
                        }
                    }
                    continue;
                }
            }
            let saved = saved_operators.get_mut(i).and_then(Option::take)
                .filter(|_| unchanged()).and_then(|cached| match &translation {""")
edit(builder, """                let stream_object = self.serialize_page_stream(&saved);
                self.add_bytes(stream_id, stream_object);""", """                let stream_object = self.serialize_page_stream(&saved);
                serialized_pages += 1;
                if let Some(parts) = &mut parts {
                    parts.store_stream(i, doc.pages.len(), Arc::from(stream_object.as_slice()));
                }
                self.add_bytes(stream_id, stream_object);""")
edit(builder, """            let stream_object = self.serialize_page_stream(&stream_bytes);
            self.add_bytes(stream_id, stream_object);
            if may_reuse_operators {""", """            let stream_object = self.serialize_page_stream(&stream_bytes);
            serialized_pages += 1;
            if may_reuse_operators {
                if let Some(parts) = &mut parts {
                    parts.store_stream(i, doc.pages.len(), Arc::from(stream_object.as_slice()));
                }
            }
            self.add_bytes(stream_id, stream_object);
            if may_reuse_operators {""")
edit(builder, """        if trace {
            eprintln!("pdf-page-operators""", """        if trace_cids {
            eprintln!("pdf-stable-cids seeded_faces={seeded_faces} streams_reused={stream_hits} serialized={serialized_pages}");
        }
        if trace {
            eprintln!("pdf-page-operators""")
