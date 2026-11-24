pub(crate) struct PageFontScan {
    pub faces: Vec<FaceScan>,
    pub used: Vec<BTreeSet<u16>>,
}

// ToUnicode takes the first cluster for a remapped GID. The space fixup also
// needs to know if a later cluster consisted solely of word joiners.
pub(crate) fn compact_clusters(clusters: impl IntoIterator<Item = (u16, String)>)
    -> Vec<(u16, String)>
{
    let mut first = BTreeSet::new();
    let mut joiners = BTreeSet::new();
    let mut out = Vec::new();
    for (gid, hex) in clusters {
        let first_cluster = first.insert(gid);
        let joiner = !hex.is_empty() && hex.len() % 4 == 0
            && hex.as_bytes().chunks_exact(4).all(|part| part == b"2060");
        if first_cluster || joiner && !joiners.contains(&gid) {
            out.push((gid, hex));
        }
        if joiner {
            joiners.insert(gid);
        }
    }
    out
}

pub(crate) fn merge_page_fonts(pages: &[Arc<PageFontScan>], fonts: usize) -> FontScans {
    let mut used = Vec::with_capacity(fonts);
    let mut faces = Vec::with_capacity(fonts);
    for fi in 0..fonts {
        let mut used_glyphs = BTreeSet::new();
        let mut all_chars = BTreeSet::new();
        let mut text_chars = BTreeSet::new();
        let mut glyph_ids = BTreeSet::new();
        let mut shaped_mappings = BTreeMap::new();
        for page in pages {
            let face = &page.faces[fi];
            used_glyphs.extend(&page.used[fi]);
            all_chars.extend(face.chars.iter().copied());
            text_chars.extend(face.text_chars.iter().copied());
            glyph_ids.extend(&face.glyph_ids);
            for (&gid, &ch) in &face.shaped_mappings {
                shaped_mappings.entry(gid).or_insert(ch);
            }
        }
        let chars = all_chars.into_iter().chain(text_chars.iter().copied()).collect();
        let clusters = pages.iter().flat_map(|page| page.faces[fi].shaped_cluster_texts.iter().cloned());
        faces.push(FaceScan {
            chars,
            text_chars: text_chars.into_iter().collect(),
            glyph_ids,
            shaped_mappings,
            shaped_cluster_texts: compact_clusters(clusters),
        });
        used.push(used_glyphs);
    }
    FontScans { views: Vec::new(), faces, used }
}
