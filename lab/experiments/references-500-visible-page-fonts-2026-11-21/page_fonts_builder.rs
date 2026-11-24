fn collect_face_scan(view: &PagedDocument) -> FaceScan {
    let mut chars = collect_codepoints(view);
    let text_chars = collect_text_codepoints(view);
    chars.extend(text_chars.iter().copied());
    FaceScan {
        chars,
        text_chars,
        glyph_ids: collect_glyph_ids(view),
        shaped_mappings: collect_shaped_glyph_mappings(view),
        shaped_cluster_texts: collect_shaped_cluster_texts(view),
    }
}

fn collect_document_fonts(doc: &PagedDocument, fonts: &[FontKey], faces: &[Face<'_>]) -> FontScans {
    let views = crate::export::font_binding::partition(doc, fonts);
    let scans = views.iter().map(collect_face_scan).collect();
    FontScans { views, faces: scans, used: per_font_used_glyphs(&doc.pages, fonts, faces) }
}

fn collect_page_fonts(page: &Page, fonts: &[FontKey], faces: &[Face<'_>]) -> Arc<PageFontScan> {
    let pages = std::slice::from_ref(page);
    let views = crate::export::font_binding::partition_pages(pages, fonts);
    let scans = views.iter().map(|view| {
        let mut scan = collect_face_scan(view);
        scan.shaped_cluster_texts = reuse::compact_clusters(scan.shaped_cluster_texts);
        scan
    }).collect();
    Arc::new(PageFontScan { faces: scans, used: per_font_used_glyphs(pages, fonts, faces) })
}
