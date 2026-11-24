
#[cfg(test)]
mod lab_page_three_fixture_tests {
    use super::*;

    #[test]
    fn literal_watch_history_is_admitted() {
        assert!(lab_page_three_fixture(include_str!("../references-500-watch-physical-2026-11-20/census/retain/startup.typ")));
        assert!(lab_page_three_fixture(include_str!("../references-500-watch-physical-2026-11-20/census/retain/visible.typ")));
    }

    #[test]
    fn mutations_of_pagination_reference_or_suffix_refuse() {
        let sample = include_str!("../references-500-watch-physical-2026-11-20/census/retain/visible.typ");
        for (from,to) in [
            ("height:600pt","height:440pt"),
            ("See @s0.","See @s13."),
            ("See @s19.","See @s0."),
            ("= Section 1 <s1>","= Extra <s1>"),
            ("// next comment","Content instead of a comment"),
            ("same visible source","same visible source\n#pagebreak()"),
        ] {
            assert!(!lab_page_three_fixture(&sample.replace(from,to)));
        }
    }
}
