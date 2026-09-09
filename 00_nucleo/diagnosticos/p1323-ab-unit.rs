// Snippet A/B independente; integrar dentro do módulo #[cfg(test)] de main.rs.
// Derivado de wiring.md §P1323 e do vanilla ratificado, sem leitura de main.rs.
#[test]
fn p1323_html_experimental_warning_matches_ratified_envelope() {
    let expected = concat!(
        "warning: html export is under active development and incomplete\n",
        " = hint: its behaviour may change at any time\n",
        " = hint: do not rely on this feature for production use cases\n",
        " = hint: see https://github.com/typst/typst/issues/5512 for more information\n",
        "\n",
    );
    assert_eq!(super::HTML_EXPERIMENTAL_WARNING.as_bytes(), expected.as_bytes());
}
