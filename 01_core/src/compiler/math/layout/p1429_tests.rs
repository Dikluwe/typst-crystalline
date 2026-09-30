//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/p1429-tests.md
//! @prompt-hash aeef3d5e
//! @layer L1
#![cfg(test)]

use super::*;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::compiler::layout::{FixedMetrics, FontMetrics};
use crate::compiler::stdlib::native_accent;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::glyph_variants::{GlyphVariant, GlyphVariants};
use crate::entities::layout_types::Length;
use crate::entities::location::Location;
use crate::entities::math_style::MathStyleKind;
use crate::entities::rel::Rel;
use crate::entities::source::Source;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use std::num::NonZeroU16;

#[derive(Default)]
struct NullWorld {
    library: Library,
    book: FontBook,
}
impl World for NullWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        id()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Err(FileError::NotFound)
    }
    fn file(&self, _: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound)
    }
    fn font(&self, _: usize) -> Option<Font> {
        None
    }
    fn today(&self, _: Option<crate::entities::duration::Duration>) -> Option<Datetime> {
        None
    }
}
fn id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1429).unwrap())
}
fn style() -> TextStyle {
    TextStyle::regular(Pt(12.0))
}

fn accent(base: &str, mark: char, size: Option<f64>, dotless: Option<bool>) -> Content {
    let mut args = Args::positional(vec![
        Value::Content(Content::MathText(base.into())),
        Value::Str(mark.to_string().into()),
    ]);
    if let Some(percent) = size {
        args.named
            .insert("size".into(), Value::Relative(Rel::<Length>::from_percent(percent)));
    }
    if let Some(value) = dotless {
        args.named.insert("dotless".into(), Value::Bool(value));
    }
    let Value::Content(content) =
        native_accent(&mut EvalContext::new(), &args, &NullWorld::default(), id())
            .unwrap()
    else {
        unreachable!()
    };
    content
}

fn text_items(items: &[FrameItem]) -> Vec<(String, f64)> {
    fn visit(items: &[FrameItem], out: &mut Vec<(String, f64)>) {
        for item in items {
            match item {
                #[allow(deprecated)]
                FrameItem::Text { pos, text, .. }
                | FrameItem::TextShaped { pos, text, .. } => {
                    out.push((text.to_string(), pos.y.val()));
                }
                FrameItem::TextReplacement { items, .. } => visit(items, out),
                _ => {}
            }
        }
    }
    let mut found = Vec::new();
    visit(items, &mut found);
    found
}

fn project<M: FontMetrics>(
    metrics: &M,
    content: &Content,
    chain: &StyleChain,
) -> Vec<FrameItem> {
    let s = style();
    let callbacks = callbacks::MathCallbackPassState::new(None);
    MathLayouter::new_with_context(
        metrics,
        false,
        &s,
        Location::from_raw(0x1429),
        Pt(200.0),
        chain,
        &callbacks,
    )
    .layout_equation(content, &s)
}

#[test]
fn p1429_orientacao_superior_inferior_e_dotless() {
    let chain = StyleChain::default_chain();
    for (base, dotless_base, dotted_base) in [("i", "ı", "𝑖"), ("j", "ȷ", "𝑗")] {
        let upper = text_items(&project(
            &FixedMetrics,
            &accent(base, '^', None, Some(true)),
            &chain,
        ));
        assert!(
            upper.iter().any(|(t, _)| t == dotless_base),
            "upper {base} dotless morphology missing: {upper:?}"
        );
        let base_y = upper.iter().find(|(t, _)| t == dotless_base).unwrap().1;
        let accent_y = upper.iter().find(|(t, _)| t == "\u{302}").unwrap().1;
        assert!(accent_y < base_y, "upper accent not above {base}: {upper:?}");

        let dotted = text_items(&project(
            &FixedMetrics,
            &accent(base, '^', None, Some(false)),
            &chain,
        ));
        assert!(
            dotted.iter().any(|(t, _)| t == dotted_base),
            "dotless=false removed {base} dot: {dotted:?}"
        );
    }

    for mark in ['\u{332}', '\u{23df}', '\u{23b5}', '\u{23dd}', '\u{23e1}'] {
        let lower = text_items(&project(
            &FixedMetrics,
            &accent("i", mark, None, Some(true)),
            &chain,
        ));
        let base_y = lower.iter().find(|(t, _)| t == "𝑖").unwrap().1;
        let accent_y =
            lower.iter().find(|(t, _)| t.chars().next() == Some(mark)).unwrap().1;
        assert!(accent_y > base_y, "lower accent {mark:?} not below base: {lower:?}");
    }
}

struct VariantMetrics;
impl FontMetrics for VariantMetrics {
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
    fn horizontal_glyph_variants(&self, c: char, _: &TextStyle) -> GlyphVariants {
        if c != '\u{20d7}' {
            return GlyphVariants::default();
        }
        GlyphVariants {
            variants: vec![
                GlyphVariant {
                    glyph_id: 142,
                    advance: 750.0,
                    hor_advance: 750.0,
                    top_accent_attach: None,
                },
                GlyphVariant {
                    glyph_id: 143,
                    advance: 1300.0,
                    hor_advance: 1300.0,
                    top_accent_attach: None,
                },
            ],
        }
    }
}

fn arrow_glyph(content: &Content, chain: &StyleChain) -> (u16, f64) {
    project(&VariantMetrics, content, chain)
        .iter()
        .find_map(|item| match item {
            FrameItem::Glyph { base_char: '\u{20d7}', glyph_id, x_advance, .. } => {
                Some((*glyph_id, x_advance.val()))
            }
            _ => None,
        })
        .expect("attributable stretched arrow glyph")
}

fn arrow_width(content: &Content, chain: &StyleChain) -> f64 {
    arrow_glyph(content, chain).1
}

#[test]
fn p1429_size_resolve_na_largura_e_precedencia_independente() {
    let default = StyleChain::default_chain();
    let base = accent("xx", '→', None, None);
    let explicit = accent("xx", '→', Some(150.0), None);
    let w100 = arrow_width(&base, &default);
    let w150 = arrow_width(&explicit, &default);
    assert_eq!(
        arrow_glyph(&base, &default).0,
        142,
        "100% target 1200du minus 0.5em must be 700du and select 750du; 0.4em would select 1300du and 0.6em would keep the base glyph"
    );
    assert_eq!(
        arrow_glyph(&explicit, &default).0,
        143,
        "150% target must remain above the 0.5em-shortened first variant"
    );
    assert!(w150 > w100, "150% did not select a wider variant: {w100} vs {w150}");

    let cascade = StyleChain::default_chain()
        .push_custom(
            "math.accent.size",
            Value::Relative(Rel::<Length>::from_percent(150.0)),
        )
        .push_custom("math.accent.dotless", Value::Bool(false));
    assert_eq!(arrow_width(&base, &cascade), w150, "cascaded size was ignored");
    assert_eq!(
        arrow_width(&accent("xx", '→', Some(100.0), Some(true)), &cascade),
        w100,
        "cascade overrode explicit values"
    );
    let cascaded_i =
        text_items(&project(&VariantMetrics, &accent("i", '^', None, None), &cascade));
    assert!(
        cascaded_i.iter().any(|(t, _)| t == "𝑖"),
        "cascaded dotless=false was ignored: {cascaded_i:?}"
    );
}

fn causal_suffix(content: &Content) -> String {
    let repr = repr_content(content);
    repr.split_once(", accent:")
        .map(|(_, suffix)| suffix.to_string())
        .unwrap_or_else(|| panic!("not an attributable accent repr: {repr}"))
}

#[test]
fn p1429_reconstrutores_preservam_campos_causais() {
    for input in [
        accent("x", '^', None, None),
        accent("x", '~', Some(150.0), Some(false)),
        accent("x", '.', Some(100.0), Some(true)),
    ] {
        let expected = causal_suffix(&input);
        assert_eq!(causal_suffix(&apply_math_default(&input)), expected);
        assert_eq!(
            causal_suffix(&apply_math_style(
                &input,
                Some(MathStyleKind::DoubleStruck),
                Some(true),
                Some(false),
            )),
            expected
        );
    }
}
