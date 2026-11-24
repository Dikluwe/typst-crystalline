// The previous successful subset supplies a contiguous CID-ordered prefix.
pub(crate) fn cid_prefix(remap: &HashMap<u16, u16>) -> Option<Vec<u16>> {
    if remap.is_empty() || remap.len() > 2048 || remap.get(&0) != Some(&0) {
        return None;
    }
    let mut ordered = vec![None; remap.len()];
    for (&gid, &cid) in remap {
        if ordered.get_mut(usize::from(cid))?.replace(gid).is_some() {
            return None;
        }
    }
    ordered.into_iter().collect()
}

impl OperatorMaps {
    pub(crate) fn same_page_cids(&self, next: &Self, page: &PageFontScan) -> bool {
        page.faces.len() == self.remap.len()
            && next.remap.len() == self.remap.len()
            && page.faces.iter().enumerate().all(|(fi, face)| {
                !self.remap[fi].is_empty() && !next.remap[fi].is_empty()
                    && face.glyph_ids.iter().chain(page.used[fi].iter()).all(|gid| {
                        self.remap[fi].get(gid) == next.remap[fi].get(gid)
                            && self.nominal[fi].get(gid) == next.nominal[fi].get(gid)
                    })
                    && face.chars.iter().all(|ch| {
                        self.character[fi].get(ch) == next.character[fi].get(ch)
                    })
            })
    }
}

impl Parts {
    pub(crate) fn store_stream(&mut self, index: usize, pages: usize, stream: Arc<[u8]>) {
        if self.mode.operators && stream.len() <= MAX_BYTES - self.stream_bytes {
            self.streams.resize_with(pages, || None);
            self.stream_bytes += stream.len();
            self.streams[index] = Some(stream);
        }
    }
}
