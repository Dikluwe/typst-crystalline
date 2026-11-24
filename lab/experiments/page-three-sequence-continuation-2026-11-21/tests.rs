
#[cfg(test)]
mod lab_sequence_boundary_tests {
    use super::*;

    #[test]
    fn lab_sequence_resume_lands_on_the_original_separator() {
        let parts = [Content::text("line"), Content::Parbreak, Content::text("next")];
        assert!(lab_sequence_boundary(&parts, 0, 1));
        assert!(!lab_sequence_boundary(&parts, 0, 2));
        assert!(!lab_sequence_boundary(&parts, 1, 1));
        assert!(!lab_sequence_boundary(&parts, 1, 3));
    }
}
