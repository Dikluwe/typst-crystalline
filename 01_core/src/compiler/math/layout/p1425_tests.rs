//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/p1425-tests.md
//! @prompt-hash 66cd43a9
//! @layer L1
#![cfg(test)]
//!
//! Testes A/B independentes de preservação/layout P1425.

use super::*;
use crate::compiler::eval::repr::repr_content;
use crate::compiler::layout::FixedMetrics;
use crate::entities::elements::math_attach::MathAttachSlot;

fn style() -> TextStyle {
    TextStyle::regular(Pt(12.0))
}

fn attached(inline: bool, inline_explicit: bool) -> Content {
    Content::math_attach_scripts(
        Content::math_limits_override(
            Content::MathIdent("A".into()),
            true,
            inline,
            inline_explicit,
        ),
        MathAttachSlot::Omitted,
        MathAttachSlot::Omitted,
        MathAttachSlot::Present(Content::MathText("1".into())),
        MathAttachSlot::Present(Content::MathText("2".into())),
    )
}

fn x_of(items: &[FrameItem], needle: &str) -> f64 {
    items
        .iter()
        .find_map(|item| match item {
            FrameItem::Text { pos, text, .. } if text.as_str() == needle => {
                Some(pos.x.val())
            }
            _ => None,
        })
        .unwrap_or_else(|| panic!("missing {needle:?} in {items:?}"))
}

fn projection(block: bool, inline: bool, explicit: bool) -> (String, f64, f64) {
    let style = style();
    let items = MathLayouter::new(&FixedMetrics, block, &style)
        .layout_equation(&attached(inline, explicit), &style);
    let base_x = x_of(&items, "𝐴");
    let sup_x = x_of(&items, "2");
    (format!("{items:?}"), base_x, sup_x)
}

#[test]
fn p1425_apply_math_default_preserva_presenca_e_morfologia() {
    for (inline, explicit, expected) in [
        (true, false, "limits(body: [𝑥])"),
        (true, true, "limits(body: [𝑥], inline: true)"),
        (false, true, "limits(body: [𝑥], inline: false)"),
    ] {
        let input = Content::math_limits_override(
            Content::MathIdent("x".into()),
            true,
            inline,
            explicit,
        );
        let output = apply_math_default(&input);
        let Content::MathLimitsOverride(elem) = &output else {
            panic!("apply_math_default changed variant")
        };
        assert!(elem.limits);
        assert_eq!(elem.inline, inline);
        assert_eq!(elem.inline_explicit, explicit);
        assert_eq!(repr_content(&output), expected);
    }
}

#[test]
fn p1425_presenca_sozinha_nao_muda_geometria() {
    for block in [false, true] {
        let omitted = projection(block, true, false);
        let explicit_true = projection(block, true, true);
        assert_eq!(omitted.0, explicit_true.0);
    }
}

#[test]
fn p1425_inline_false_so_empilha_em_display() {
    let (_, base_inline, sup_inline) = projection(false, false, true);
    assert!(
        sup_inline > base_inline + 3.6,
        "inline:false must use lateral scripts inline: base={base_inline} sup={sup_inline}"
    );

    let (_, base_display, sup_display) = projection(true, false, true);
    assert!(
        (sup_display - base_display).abs() < 6.0,
        "inline:false must stack in display: base={base_display} sup={sup_display}"
    );
}

#[test]
fn p1425_scripts_controle_permanece_lateral_em_display() {
    let style = style();
    let attach = Content::math_attach_scripts(
        Content::math_limits_override(Content::MathText("∑".into()), false, true, false),
        MathAttachSlot::Omitted,
        MathAttachSlot::Omitted,
        MathAttachSlot::Present(Content::MathText("1".into())),
        MathAttachSlot::Present(Content::MathText("2".into())),
    );
    let items =
        MathLayouter::new(&FixedMetrics, true, &style).layout_equation(&attach, &style);
    let base_x = x_of(&items, "∑");
    let sup_x = x_of(&items, "2");
    assert!(sup_x > base_x + 3.6);
}
