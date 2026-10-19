//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/p1427-tests.md
//! @prompt-hash 46b0d6fd
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes de reconstrução e layout `math.frac` P1427.

use super::*;
use crate::compiler::eval::repr::repr_content;
use crate::compiler::layout::{FixedMetrics, FontMetrics};
use crate::entities::elements::math_frac::{MathFracElem, MathFracStyle};
use crate::entities::location::Location;
use crate::entities::math_constants::MathConstants;
use crate::entities::math_style::MathStyleKind;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use std::sync::Arc;

fn style() -> TextStyle {
    TextStyle::regular(Pt(12.0))
}

fn frac(style: Option<MathFracStyle>, nb: bool, db: bool, line: bool) -> Content {
    Content::MathFrac(Arc::new(MathFracElem {
        num: Content::MathText("N".into()),
        den: Content::MathText("D".into()),
        style,
        num_deparenthesized: nb,
        denom_deparenthesized: db,
        line,
    }))
}

fn fields(content: &Content) -> (&Option<MathFracStyle>, bool, bool, bool) {
    let Content::MathFrac(elem) = content else { panic!("expected MathFrac") };
    (&elem.style, elem.num_deparenthesized, elem.denom_deparenthesized, elem.line)
}

fn projected(content: &Content) -> Vec<FrameItem> {
    let style = style();
    MathLayouter::new(&FixedMetrics, false, &style).layout_equation(content, &style)
}

fn projected_with_cascade(content: &Content, cascade: &str) -> Vec<FrameItem> {
    let style = style();
    let chain = StyleChain::default_chain()
        .push_custom("math.frac.style", Value::Str(cascade.into()));
    let callbacks = callbacks::MathCallbackPassState::new(None);
    MathLayouter::new_with_context(
        &FixedMetrics,
        false,
        &style,
        Location::from_raw(0x1427),
        Pt(200.0),
        &chain,
        &callbacks,
    )
    .layout_equation(content, &style)
}

#[derive(Clone)]
struct GapMetrics(MathConstants);

impl FontMetrics for GapMetrics {
    fn advance(&self, text: &str, size: Pt, style: &TextStyle) -> Pt {
        FixedMetrics.advance(text, size, style)
    }
    fn vertical_metrics(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        FixedMetrics.vertical_metrics(size, style)
    }
    fn cap_height(&self, size: Pt, style: &TextStyle) -> Pt {
        FixedMetrics.cap_height(size, style)
    }
    fn text_edges(&self, size: Pt, style: &TextStyle) -> (Pt, Pt) {
        FixedMetrics.text_edges(size, style)
    }
    fn math_constants(&self, _: &TextStyle) -> MathConstants {
        self.0.clone()
    }
}

fn skew_separation(constants: MathConstants) -> (f64, f64) {
    let metrics = GapMetrics(constants);
    let style = style();
    let items = MathLayouter::new(&metrics, false, &style)
        .layout_equation(&frac(Some(MathFracStyle::Skewed), false, false, true), &style);
    let text = texts(&items);
    let n = text.iter().find(|(value, _, _)| value == "N").unwrap();
    let d = text.iter().find(|(value, _, _)| value == "D").unwrap();
    (d.1 - n.1, d.2 - n.2)
}

fn texts(items: &[FrameItem]) -> Vec<(String, f64, f64)> {
    items
        .iter()
        .filter_map(|item| match item {
            #[allow(deprecated)]
            FrameItem::Text { pos, text, .. }
            | FrameItem::TextShaped { pos, text, .. } => {
                Some((text.to_string(), pos.x.val(), pos.y.val()))
            }
            _ => None,
        })
        .collect()
}

fn horizontal_lines(items: &[FrameItem]) -> usize {
    items.iter().filter(|item| matches!(item, FrameItem::Line { start, end, .. } if (start.y.val() - end.y.val()).abs() < 1e-9)).count()
}

#[test]
fn p1427_apply_math_default_preserva_frac_completa() {
    for input in [
        frac(None, false, false, true),
        frac(Some(MathFracStyle::Vertical), true, false, true),
        frac(Some(MathFracStyle::Skewed), false, true, true),
        frac(Some(MathFracStyle::Horizontal), true, true, false),
    ] {
        let expected = fields(&input);
        let output = apply_math_default(&input);
        assert_eq!(fields(&output), expected);
    }
}

#[test]
fn p1427_apply_math_style_preserva_frac_completa() {
    for input in [
        frac(None, false, false, true),
        frac(Some(MathFracStyle::Vertical), true, false, true),
        frac(Some(MathFracStyle::Skewed), false, true, true),
        frac(Some(MathFracStyle::Horizontal), true, true, false),
    ] {
        let expected = fields(&input);
        let output =
            apply_math_style(&input, Some(MathStyleKind::DoubleStruck), None, None);
        assert_eq!(fields(&output), expected);
    }
}

#[test]
fn p1427_vertical_empilha_e_desenha_barra() {
    let items = projected(&frac(Some(MathFracStyle::Vertical), false, false, true));
    let text = texts(&items);
    let n = text.iter().find(|(value, _, _)| value == "N").unwrap();
    let d = text.iter().find(|(value, _, _)| value == "D").unwrap();
    assert!(n.2 < d.2, "numerator must be above denominator: {items:?}");
    assert_eq!(
        horizontal_lines(&items),
        1,
        "vertical fraction needs one rule: {items:?}"
    );

    let omitted = projected(&frac(None, false, false, true));
    assert_eq!(
        format!("{omitted:?}"),
        format!("{items:?}"),
        "style=None without cascade must resolve exactly to Vertical"
    );
}

#[test]
fn p1427_explicito_vertical_vence_cascade_horizontal() {
    let cascaded = projected_with_cascade(&frac(None, false, false, true), "horizontal");
    assert_eq!(horizontal_lines(&cascaded), 0, "cascade was ignored: {cascaded:?}");

    let explicit = projected_with_cascade(
        &frac(Some(MathFracStyle::Vertical), false, false, true),
        "horizontal",
    );
    assert_eq!(
        horizontal_lines(&explicit),
        1,
        "cascade overrode explicit Vertical: {explicit:?}"
    );
}

#[test]
fn p1427_skewed_usa_slash_e_gaps_independentes() {
    let items = projected(&frac(Some(MathFracStyle::Skewed), false, false, true));
    let text = texts(&items);
    let n = text.iter().find(|(value, _, _)| value == "N").unwrap();
    let d = text.iter().find(|(value, _, _)| value == "D").unwrap();
    assert!(n.1 < d.1 && n.2 < d.2, "skewed ordering mismatch: {items:?}");
    assert!(
        text.iter()
            .any(|(value, _, _)| value.contains('/') || value.contains('⁄')),
        "missing slash: {items:?}"
    );
    assert_eq!(
        horizontal_lines(&items),
        0,
        "skewed must not draw a horizontal rule: {items:?}"
    );

    let mut base = MathConstants::fallback();
    let base_sep = skew_separation(base.clone());
    base.skewed_fraction_vertical_gap += 100.0;
    let vertical = skew_separation(base.clone());
    assert!(
        (vertical.1 - base_sep.1).abs() > 0.5,
        "vertical gap is unused: base={base_sep:?} varied={vertical:?}"
    );
    assert!(
        (vertical.0 - base_sep.0).abs() < 1e-9,
        "vertical gap leaked into horizontal separation"
    );

    base.skewed_fraction_vertical_gap -= 100.0;
    base.skewed_fraction_horizontal_gap += 100.0;
    let horizontal = skew_separation(base.clone());
    assert!(
        (horizontal.0 - base_sep.0).abs() > 0.5,
        "horizontal gap is unused: base={base_sep:?} varied={horizontal:?}"
    );
    assert!(
        (horizontal.1 - base_sep.1).abs() < 1e-9,
        "horizontal gap leaked into vertical separation"
    );

    let mut swapped = MathConstants::fallback();
    std::mem::swap(
        &mut swapped.skewed_fraction_vertical_gap,
        &mut swapped.skewed_fraction_horizontal_gap,
    );
    assert_ne!(skew_separation(swapped), base_sep, "96/350 swap survived");
}

#[test]
fn p1427_horizontal_restaura_protecao_causal() {
    for (nb, db, left, right) in [
        (false, false, false, false),
        (true, false, true, false),
        (false, true, false, true),
        (true, true, true, true),
    ] {
        let items = projected(&frac(Some(MathFracStyle::Horizontal), nb, db, true));
        let flat = texts(&items).into_iter().map(|(text, _, _)| text).collect::<String>();
        assert_eq!(flat.contains("(N)"), left, "numerator protection: {flat:?}");
        assert_eq!(flat.contains("(D)"), right, "denominator protection: {flat:?}");
        let n = flat.find('N').unwrap();
        let slash = flat.find('/').or_else(|| flat.find('⁄')).unwrap();
        let d = flat.find('D').unwrap();
        assert!(n < slash && slash < d, "horizontal order: {flat:?}");
        assert_eq!(horizontal_lines(&items), 0);
    }
}

#[test]
fn p1427_fallback_skewed_gaps_e_96_350() {
    let constants = MathConstants::fallback();
    assert_eq!(constants.skewed_fraction_vertical_gap, 96.0);
    assert_eq!(constants.skewed_fraction_horizontal_gap, 350.0);
    assert_ne!(
        constants.skewed_fraction_vertical_gap,
        constants.skewed_fraction_horizontal_gap
    );
    assert_ne!(constants.skewed_fraction_vertical_gap, constants.axis_height);
}

#[test]
fn p1427_binom_permanece_vertical_sem_barra() {
    let binom = Content::math_delimited(
        '(',
        frac(Some(MathFracStyle::Horizontal), true, true, false),
        ')',
    );
    let items = projected(&binom);
    let flat = texts(&items).into_iter().map(|(text, _, _)| text).collect::<String>();
    assert!(flat.contains('(') && flat.contains(')'));
    assert!(!flat.contains('/') && !flat.contains('⁄'), "binom gained slash: {items:?}");
    assert_eq!(horizontal_lines(&items), 0, "binom gained fraction bar: {items:?}");
    assert!(repr_content(&binom).starts_with("binom("));

    let cascaded = projected_with_cascade(&frac(None, false, false, false), "horizontal");
    let text = texts(&cascaded);
    let n = text.iter().find(|(value, _, _)| value == "N").unwrap();
    let d = text.iter().find(|(value, _, _)| value == "D").unwrap();
    assert!(n.2 < d.2, "line=false inherited horizontal cascade: {cascaded:?}");
    assert_eq!(horizontal_lines(&cascaded), 0, "line=false gained a bar");
    assert!(
        !text
            .iter()
            .any(|(value, _, _)| value.contains('/') || value.contains('⁄')),
        "line=false gained a slash: {cascaded:?}"
    );
}
