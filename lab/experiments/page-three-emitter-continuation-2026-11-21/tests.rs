
#[cfg(test)]
mod lab_emitter_boundary_tests {
    use super::*;

    #[test]
    fn lab_emitter_detaches_only_the_terminal_line_admitted_on_page_three() {
        assert!(lab_suspend_emitter(true, 1, 2, 0, 1));
        assert!(lab_suspend_emitter(true, 1, 2, 2, 3));
        assert!(!lab_suspend_emitter(false, 1, 2, 0, 1));
        assert!(!lab_suspend_emitter(true, 1, 2, 0, 2));
        assert!(!lab_suspend_emitter(true, 2, 2, 0, 1));
        assert!(!lab_suspend_emitter(true, 0, 1, 0, 1));
    }
}
