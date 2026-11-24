
#[cfg(test)]
mod physical_paragraph_ascii_tests {
    use super::*;

    #[test]
    fn paragraph_ascii_reuses_proof_across_session_clones_only_for_same_owner() {
        let session = ShapeSession::new();
        let font = Arc::new(vec![1, 2, 3]);
        assert_eq!(session.paragraph_ascii(&font, || 37), 37);
        let clone = session.clone();
        assert_eq!(
            clone.paragraph_ascii(&font, || panic!("repeated cmap work")),
            37,
        );
        let replacement = Arc::new(font.as_ref().clone());
        assert_eq!(clone.paragraph_ascii(&replacement, || 22), 22);
        assert_eq!(clone.paragraph_ascii(&font, || 0), 37);
        assert_eq!(ShapeSession::new().paragraph_ascii(&font, || 42), 42);
    }
}
