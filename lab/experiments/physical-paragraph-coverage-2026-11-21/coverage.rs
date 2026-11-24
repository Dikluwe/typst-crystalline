
struct ParagraphAscii {
    owner: std::sync::Weak<Vec<u8>>,
    mask: u128,
}

impl ShapeSession {
    /// Immutable cmap result for the retained bytes of face zero.
    pub(crate) fn paragraph_ascii(
        &self,
        owner: &Arc<Vec<u8>>,
        measure: impl FnOnce() -> u128,
    ) -> u128 {
        let mut entries = self.paragraph_ascii.lock().unwrap();
        if let Some(entry) = entries
            .iter()
            .find(|entry| entry.owner.as_ptr() == Arc::as_ptr(owner))
        {
            return entry.mask;
        }
        let mask = measure();
        if entries.len() == MAX_OWNERS {
            entries.pop_front();
        }
        entries.push_back(ParagraphAscii { owner: Arc::downgrade(owner), mask });
        mask
    }
}
