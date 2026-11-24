
#[cfg(test)]
impl ShapeSession {
    pub(crate) fn paragraph_ascii(&self, _: &Arc<Vec<u8>>, measure: impl FnOnce() -> u128) -> u128 {
        measure()
    }
}
