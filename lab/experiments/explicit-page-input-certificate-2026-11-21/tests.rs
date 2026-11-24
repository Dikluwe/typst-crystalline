
#[cfg(test)]
mod explicit_page_inputs_tests {
    use super::*;

    const HEADER: &str = "#set page(width: 400pt, height: 600pt, margin: 20pt)\n#set text(font: \"Libertinus Serif\")\n";
    const FIRST: &str = "First paragraph on the page.\n\n";
    const SECOND: &str = "Second independent paragraph.\n\n";

    #[test]
    fn lab_page_inputs_accept_literal_boundary_and_refuse_external_reads() {
        let source = format!("{HEADER}{FIRST}#pagebreak()\n{SECOND}");
        assert_eq!(lab_page_inputs(&source), Some((HEADER, vec![FIRST, SECOND])));
        for extra in [
            "#context counter(page).get()\n\n",
            "#include \"leaf.typ\"\n\n",
            "<label>\n\n",
            "#pagebreak(weak: true)\n\n",
            "#image(\"test.png\")\n\n",
            "Arabic العربية\n\n",
        ] {
            assert!(lab_page_inputs(&format!("{source}{extra}")).is_none(), "{extra}");
        }
        assert!(lab_page_inputs(&source.replace("margin: 20pt", "margin: (inside: 30pt, outside: 20pt)")).is_none());
    }
}
