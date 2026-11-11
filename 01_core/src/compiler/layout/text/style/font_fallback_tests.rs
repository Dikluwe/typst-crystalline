//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/layout/text/style/font_fallback_tests.md
//! @prompt-hash d82e518e
//! @layer L1
//! @updated 2026-11-08
#![cfg(test)]
use super::*;
use crate::entities::{
    dir::Dir,
    font_list::{FontFamily, FontList, FontNamePattern},
    font_variations::FontVariations,
    regex::Regex,
    style_chain::{StyleChain, StyleDelta},
};

fn rich_list() -> FontList {
    FontList::new(vec![
        FontFamily::new_named(
            FontNamePattern::Literal("Typed Family".into()),
            vec!["Regular".into(), "Bold".into()],
            Some("Book".into()),
            Some("450".into()),
            Some("oblique".into()),
        ),
        FontFamily::new_named(
            FontNamePattern::Regex(Regex::new("Second.*").unwrap()),
            vec!["Italic".into()],
            Some("Display".into()),
            Some("700".into()),
            Some("italic".into()),
        ),
        FontFamily::new("Last Fallback".into()),
    ])
    .unwrap()
}
fn rich_value() -> Value {
    Value::Array(vec![
        Value::Int(99),
        Value::Dict(
            [
                ("name".into(), Value::Str("Custom Literal".into())),
                (
                    "variants".into(),
                    Value::Array(vec![
                        Value::Str("Regular".into()),
                        Value::Int(9),
                        Value::Str("Bold".into()),
                    ]),
                ),
                ("variant".into(), Value::Str("Book".into())),
                ("weight".into(), Value::Str("450".into())),
                ("style".into(), Value::Str("oblique".into())),
            ]
            .into_iter()
            .collect(),
        ),
        Value::Dict(
            [
                ("name".into(), Value::Regex(Regex::new("Custom.*").unwrap())),
                ("variants".into(), Value::Array(vec![Value::Str("Italic".into())])),
                ("variant".into(), Value::Str("Display".into())),
                ("weight".into(), Value::Str("700".into())),
                ("style".into(), Value::Str("italic".into())),
            ]
            .into_iter()
            .collect(),
        ),
        Value::Str("LAST CUSTOM".into()),
    ])
}
fn custom_expected() -> FontList {
    FontList::new(vec![
        FontFamily::new_named(
            FontNamePattern::Literal("Custom Literal".into()),
            vec!["Regular".into(), "Bold".into()],
            Some("Book".into()),
            Some("450".into()),
            Some("oblique".into()),
        ),
        FontFamily::new_named(
            FontNamePattern::Regex(Regex::new("Custom.*").unwrap()),
            vec!["Italic".into()],
            Some("Display".into()),
            Some("700".into()),
            Some("italic".into()),
        ),
        FontFamily::new("last custom".into()),
    ])
    .unwrap()
}
#[test]
fn nearest_and_same_node_typed_precedence() {
    let typed = rich_list();
    let custom = rich_value();
    let outer_custom =
        StyleChain::default_chain().push_custom("text.font", custom.clone());
    let inner_typed = outer_custom
        .push(StyleDelta { font: Some(typed.clone()), ..StyleDelta::empty() });
    let active = TextStyle::from(&inner_typed);
    assert_eq!(from_parts(&inner_typed, &active).font, Some(typed.clone()));
    let outer_typed = StyleChain::default_chain()
        .push(StyleDelta { font: Some(typed.clone()), ..StyleDelta::empty() });
    let inner_custom = outer_typed.push_custom("text.font", custom.clone());
    assert_eq!(
        from_parts(&inner_custom, &TextStyle::from(&inner_custom)).font,
        Some(custom_expected())
    );
    let same = outer_typed.push(StyleDelta {
        font: Some(typed.clone()),
        custom: vec![("text.font".into(), custom)],
        ..StyleDelta::empty()
    });
    assert_eq!(from_parts(&same, &TextStyle::from(&same)).font, Some(typed));
}
#[test]
fn rich_custom_preserves_every_representable_identity_field() {
    let chain = StyleChain::default_chain().push_custom("text.font", rich_value());
    let mut active = TextStyle::from(&chain);
    active.font = Some(rich_list());
    let before = active.clone();
    let declaration = chain.custom("text.font").cloned();
    let got = from_parts(&chain, &active);
    assert_eq!(got.font, Some(custom_expected()));
    assert!(got
        .font
        .as_ref()
        .unwrap()
        .as_slice()
        .iter()
        .all(|f| f.covers.is_none()));
    assert_eq!(active, before);
    assert_eq!(chain.custom("text.font").cloned(), declaration);
    let again = from_parts(&chain, &active);
    assert_eq!(got.font, again.font);
    assert_ne!(
        got.font.as_ref().unwrap().as_slice().as_ptr(),
        again.font.as_ref().unwrap().as_slice().as_ptr()
    );
}
#[test]
fn invalid_empty_none_nonarray_fallback_is_complete_and_owned() {
    let invalids = vec![
        Value::Array(vec![]),
        Value::Array(vec![Value::Int(3), Value::None]),
        Value::Array(vec![Value::Dict(
            [("name".into(), Value::Str("Missing variants".into()))]
                .into_iter()
                .collect(),
        )]),
        Value::None,
        Value::Str("nonarray".into()),
        Value::Int(7),
    ];
    for invalid in invalids {
        let chain = StyleChain::default_chain().push_custom("text.font", invalid.clone());
        for fallback in [None, Some(rich_list())] {
            let mut active = TextStyle::from(&chain);
            active.font = fallback;
            let before = active.clone();
            let got = from_parts(&chain, &active);
            assert_eq!(got.font, before.font, "invalid={invalid:?}");
            assert_eq!(active, before);
            assert_eq!(chain.custom("text.font"), Some(&invalid));
            if let (Some(a), Some(b)) = (&active.font, &got.font) {
                assert_ne!(a.as_slice().as_ptr(), b.as_slice().as_ptr());
            }
        }
    }
}
#[test]
fn absent_custom_and_none_active_remain_distinct_from_decoded_success() {
    let chain = StyleChain::default_chain();
    let mut active = TextStyle::from(&chain);
    assert_eq!(from_parts(&chain, &active).font, active.font);
    active.font = None;
    assert_eq!(from_parts(&chain, &active).font, None);
    active.font = Some(rich_list());
    let got = from_parts(&chain, &active);
    assert_eq!(got.font, active.font);
    assert_ne!(
        got.font.as_ref().unwrap().as_slice().as_ptr(),
        active.font.as_ref().unwrap().as_slice().as_ptr()
    );
    let custom = chain.push_custom("text.font", rich_value());
    active.font = None;
    assert_eq!(from_parts(&custom, &active).font, Some(custom_expected()));
}
#[test]
fn other_render_and_math_script_fields_are_preserved() {
    let chain = StyleChain::default_chain().push_custom("text.font", rich_value());
    let mut active = TextStyle::from(&chain);
    active.font = Some(rich_list());
    active.size = Pt(23.5);
    active.bold = true;
    active.italic = true;
    active.semantic_strong = true;
    active.semantic_emph = true;
    active.math = true;
    active.math_text_item = true;
    active.math_script = true;
    active.cramped = true;
    active.dir = Some(Dir::RTL);
    active.lang = Some("ar".parse().unwrap());
    active.variations = Some(FontVariations(vec![(*b"wght", 450.0), (*b"ital", 1.0)]));
    active.superscript = true;
    active.superscript_native = Some(false);
    active.baseline_offset = Length::pt(2.0);
    let before = active.clone();
    let mut expected = active.clone();
    expected.font = Some(custom_expected());
    // The actual-script request preserves outer size before script metrics.
    assert_eq!(resolve_parts(&chain, &active, false), expected);
    assert_eq!(active, before);
    active.superscript = false;
    active.lang = None;
    expected = active.clone();
    expected.font = Some(custom_expected());
    assert_eq!(from_parts(&chain, &active), expected);
}
#[test]
fn active_affine_size_and_direction_rules_survive_font_choice() {
    let chain = StyleChain::default_chain()
        .push_custom("text.size", Value::Length(Length::em(2.0)))
        .push_custom("text.dir", Value::Dir(Dir::RTL))
        .push_custom("text.font", rich_value());
    let mut active = TextStyle::from(&chain);
    assert_eq!(active.size, Pt(22.0));
    active.dir = Some(Dir::LTR);
    let got = from_parts(&chain, &active);
    assert_eq!(got.size, Pt(22.0));
    assert_eq!(got.dir, Some(Dir::RTL));
    assert_eq!(got.lang, None);
    active.math = true;
    assert_eq!(from_parts(&chain, &active).dir, Some(Dir::RTL));
    let plain = StyleChain::default_chain();
    active.dir = Some(Dir::RTL);
    assert_eq!(from_parts(&plain, &active).dir, Some(Dir::RTL));
}
