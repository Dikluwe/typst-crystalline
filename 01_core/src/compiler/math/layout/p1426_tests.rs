//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/p1426-tests.md
//! @prompt-hash 75596a40
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes de reconstrução e inércia geométrica P1426.

use super::*;
use crate::compiler::eval::repr::repr_content;
use crate::compiler::layout::FixedMetrics;
use crate::entities::math_style::MathStyleKind;

fn style() -> TextStyle {
    TextStyle::regular(Pt(12.0))
}

fn root(index: Option<Content>, explicit: bool) -> Content {
    Content::math_root(index, Content::MathIdent("x".into()), explicit)
}

fn presence(content: &Content) -> bool {
    let Content::MathRoot(elem) = content else { panic!("expected MathRoot") };
    elem.index_explicit
}

fn projection(content: &Content) -> String {
    let style = style();
    let items =
        MathLayouter::new(&FixedMetrics, false, &style).layout_equation(content, &style);
    format!("{items:?}")
}

#[test]
fn p1426_apply_math_default_preserva_presenca() {
    for (index, explicit, expected_index_fragment) in [
        (None, false, None),
        (None, true, Some("index: none")),
        (Some(Content::MathText("3".into())), true, Some("index:")),
    ] {
        let output = apply_math_default(&root(index, explicit));
        assert_eq!(presence(&output), explicit);
        let repr = repr_content(&output);
        assert!(repr.starts_with("root("));
        assert!(repr.contains("radicand:"));
        match expected_index_fragment {
            Some(fragment) => assert!(repr.contains(fragment), "{repr}"),
            None => assert!(!repr.contains("index:"), "{repr}"),
        }
    }
}

#[test]
fn p1426_apply_math_style_preserva_presenca() {
    for (index, explicit) in
        [(None, false), (None, true), (Some(Content::MathText("3".into())), true)]
    {
        let output = apply_math_style(
            &root(index, explicit),
            Some(MathStyleKind::DoubleStruck),
            None,
            None,
        );
        assert_eq!(presence(&output), explicit);
        assert!(output.plain_text().contains('𝕩'));
    }
}

#[test]
fn p1426_presenca_sozinha_nao_muda_layout() {
    let absent = root(None, false);
    let explicit_none = root(None, true);
    assert_eq!(projection(&absent), projection(&explicit_none));
}

#[test]
fn p1426_sqrt_controle_mantem_morfologia_e_layout() {
    let square = root(None, false);
    assert_eq!(repr_content(&square), "root(radicand: x)");
    let projected = projection(&square);
    assert!(projected.contains('√'), "missing radical: {projected}");
}
