//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/source-transparency-tests.md
//! @prompt-hash 8d3967b8
//! @layer L1
//! @updated 2026-10-17
#![cfg(test)]

use super::MathLayouter;
use crate::{
    compiler::layout::{FixedMetrics, FontMetrics},
    entities::{
        content::{Content, SourceContent},
        elements::math_attach::MathAttachSlot,
        file_id::FileId,
        glyph_variants::{
            GlyphVariant, GlyphVariants, MathGlyphKern, MathKernRecord, MathKernTable,
        },
        layout_types::{FrameItem, Length, Point, Pt, TextSeparatorOrigin, TextStyle},
        math_class::MathClass,
        rel::Rel,
        span::Span,
    },
};
use std::{num::NonZeroU16, sync::Arc};

#[derive(Clone, Copy)]
struct Metrics;
impl FontMetrics for Metrics {
    fn advance(&self, t: &str, z: Pt, s: &TextStyle) -> Pt {
        FixedMetrics.advance(t, z, s)
    }
    fn vertical_metrics(&self, z: Pt, s: &TextStyle) -> (Pt, Pt) {
        FixedMetrics.vertical_metrics(z, s)
    }
    fn cap_height(&self, z: Pt, s: &TextStyle) -> Pt {
        FixedMetrics.cap_height(z, s)
    }
    fn text_edges(&self, z: Pt, s: &TextStyle) -> (Pt, Pt) {
        FixedMetrics.text_edges(z, s)
    }
    fn char_italics_correction(&self, _: char, z: Pt, _: &TextStyle) -> Pt {
        z * 0.08
    }
    fn top_accent_attach(&self, c: char, z: Pt, _: &TextStyle) -> Option<Pt> {
        Some(z * if c == 'x' || c == '𝑥' { 0.23 } else { 0.61 })
    }
    fn math_kern(&self, _: char, _: &TextStyle) -> MathGlyphKern {
        let t = MathKernTable {
            records: vec![MathKernRecord { correction_height: None, kern_value: 75.0 }],
        };
        MathGlyphKern {
            top_left: t.clone(),
            top_right: t.clone(),
            bottom_left: t.clone(),
            bottom_right: t,
        }
    }
    fn horizontal_glyph_variants(&self, c: char, _: &TextStyle) -> GlyphVariants {
        if c != '→' {
            return GlyphVariants::default();
        }
        GlyphVariants {
            variants: [600.0, 1200.0, 2400.0]
                .into_iter()
                .enumerate()
                .map(|(i, w)| GlyphVariant {
                    glyph_id: 200 + i as u16,
                    advance: w,
                    hor_advance: w,
                    top_accent_attach: None,
                })
                .collect(),
        }
    }
}

fn src(body: Content, on: bool) -> Content {
    if !on {
        return body;
    }
    Content::Source(Arc::new(SourceContent {
        body,
        span: Span::from_range(FileId::from_raw(NonZeroU16::new(1).unwrap()), 2..5),
    }))
}
fn text(t: &str) -> Content {
    Content::MathText(t.into())
}
fn seq(v: Vec<Content>) -> Content {
    Content::MathSequence(v.into())
}
fn attach(base: Content, script: Content) -> Content {
    Content::math_attach_scripts(
        base,
        MathAttachSlot::Omitted,
        MathAttachSlot::Omitted,
        MathAttachSlot::Omitted,
        MathAttachSlot::Present(script),
    )
}

fn case(name: &str, on: bool) -> Content {
    match name {
        "limits-base" => attach(
            src(Content::math_limits_override(text("∑"), true, true, true), on),
            text("a"),
        ),
        "ordinary-base" => attach(src(text("x"), on), text("a")),
        "script" => attach(text("x"), src(text("a"), on)),
        "op-text" => {
            attach(Content::math_op(src(Content::text("lim"), on), false), text("a"))
        }
        "accent-base" => Content::math_accent(src(text("x"), on), '\u{0307}'),
        "accent-i" => Content::math_accent(src(text("i"), on), '\u{0307}'),
        "accent-nested" => Content::math_accent(
            src(Content::math_accent(text("x"), '\u{0307}'), on),
            '\u{0307}',
        ),
        "underover-base" => {
            Content::math_underover(src(text("x"), on), None, Some(text("⏞")))
        }
        "underover-piece" => {
            Content::math_underover(text("x"), None, Some(src(text("⏞"), on)))
        }
        "matrix-cell" => Content::math_matrix(
            vec![
                vec![src(
                    seq(vec![text("1"), Content::math_align_point(), text("2")]),
                    on,
                )],
                vec![seq(vec![text("333"), Content::math_align_point(), text("4")])],
            ],
            ('\0', '\0'),
        ),
        "matrix-align-point" => Content::math_matrix(
            vec![
                vec![seq(vec![
                    text("1"),
                    src(Content::math_align_point(), on),
                    text("2"),
                ])],
                vec![seq(vec![text("333"), Content::math_align_point(), text("4")])],
            ],
            ('\0', '\0'),
        ),
        "text-spacing" => seq(vec![src(Content::text("hello"), on), text("x")]),
        "pipe-spacing" => seq(vec![src(text("|"), on), text("x")]),
        "weak-space" => {
            seq(vec![src(Content::h_space(Length::em(0.2), true), on), text("x")])
        }
        "weak-between" => seq(vec![
            text("a"),
            src(Content::h_space(Length::em(0.2), true), on),
            text("x"),
        ]),
        "punctuation-tail" => {
            seq(vec![text("; "), src(Content::h_space(Length::em(0.2), true), on)])
        }
        "punctuation-source" => seq(vec![src(text("; "), on), Content::text("hello")]),
        "punctuation-next" => seq(vec![text("; "), src(Content::text("hello"), on)]),
        "sequence-body" => {
            seq(vec![text("a"), src(seq(vec![text("="), text("b")]), on), text("c")])
        }
        "tighten-text" => seq(vec![
            src(Content::text("hello"), on),
            Content::math_delimited('(', text("x"), ')'),
        ]),
        "tighten-next" => seq(vec![
            Content::text("hello"),
            src(Content::math_delimited('(', text("x"), ')'), on),
        ]),
        "class-body" => {
            Content::math_class_override(MathClass::Normal, src(text("f"), on))
        }
        "stretch-body" => Content::math_stretch(
            src(text("→"), on),
            Rel { rel: 1.5, abs: Length::ZERO },
            true,
        ),
        _ => unreachable!(),
    }
}

#[derive(Debug)]
struct Visual {
    text: String,
    x: f64,
    y: f64,
    size: f64,
}
#[derive(Debug)]
struct Observation {
    width: f64,
    ascent: f64,
    descent: f64,
    line_count: usize,
    first: f64,
    last: f64,
    visual: Vec<Visual>,
    replacements: Vec<String>,
}
fn visit(items: &[FrameItem], out: &mut Observation) {
    for item in items {
        match item {
            FrameItem::TextReplacement { text, items } => {
                out.replacements.push(text.to_string());
                visit(items, out);
            }
            FrameItem::Semantic { items, .. } => visit(items, out),
            FrameItem::Text { pos, text, style, .. }
            | FrameItem::TextShaped { pos, text, style, .. } => out.visual.push(Visual {
                text: text.to_string(),
                x: pos.x.val(),
                y: pos.y.val(),
                size: style.size.val(),
            }),
            FrameItem::Glyph { pos, base_char, size, .. } => out.visual.push(Visual {
                text: base_char.to_string(),
                x: pos.x.val(),
                y: pos.y.val(),
                size: size.val(),
            }),
            FrameItem::TextSeparator { .. }
            | FrameItem::Line { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. } => {
                panic!("Unknown math observation: {item:?}")
            }
        }
    }
}
fn observe(content: &Content, block: bool) -> Observation {
    let style = TextStyle::regular(Pt(12.0));
    let layouter = MathLayouter::new(&Metrics, block, &style);
    let (items, e) = layouter.layout_equation_measured(content, &style);
    let mut result = Observation {
        width: e.width,
        ascent: e.ascent,
        descent: e.descent,
        line_count: e.line_count,
        first: e.first_baseline,
        last: e.last_baseline,
        visual: vec![],
        replacements: vec![],
    };
    visit(&items, &mut result);
    result
}
fn close(left: f64, right: f64) {
    assert!((left - right).abs() < 1e-9, "geometria diferente: {left} versus {right}");
}
fn equivalent(plain: &Observation, source: &Observation) {
    close(plain.width, source.width);
    close(plain.ascent, source.ascent);
    close(plain.descent, source.descent);
    assert_eq!(plain.line_count, source.line_count);
    close(plain.first, source.first);
    close(plain.last, source.last);
    assert_eq!(plain.visual.len(), source.visual.len());
    for (a, z) in plain.visual.iter().zip(&source.visual) {
        assert_eq!(a.text, z.text);
        close(a.x, z.x);
        close(a.y, z.y);
        close(a.size, z.size);
    }
    assert_eq!(plain.replacements, source.replacements);
}
fn visual<'a>(observation: &'a Observation, text: &str) -> &'a Visual {
    let mut candidates = observation.visual.iter().filter(|v| v.text == text);
    let one = candidates.next().expect("observação atribuível ausente");
    assert!(candidates.next().is_none(), "observação ambígua");
    one
}
fn assert_case(name: &str) {
    let plain = observe(&case(name, false), false);
    let source = observe(&case(name, true), false);
    if name == "limits-base" {
        let scripts = observe(
            &attach(
                Content::math_limits_override(text("∑"), false, true, true),
                text("a"),
            ),
            false,
        );
        assert!(
            plain.ascent > scripts.ascent,
            "limits inline deve empilhar acima dos scripts"
        );
    }
    if name == "weak-between" {
        let gapless = observe(&seq(vec![text("a"), text("x")]), false);
        close(plain.width - gapless.width, 2.4);
    }
    if name == "stretch-body" {
        let natural = observe(&text("→"), false);
        assert!(plain.width >= natural.width * 1.5);
    }
    if name == "matrix-cell" || name == "matrix-align-point" {
        let one = visual(&plain, "1");
        let three = visual(&plain, "333");
        let style = TextStyle::regular(Pt(12.0));
        close(
            one.x + Metrics.advance("1", Pt(one.size), &style).val(),
            three.x + Metrics.advance("333", Pt(three.size), &style).val(),
        );
        close(visual(&plain, "2").x, visual(&plain, "4").x);
    }
    equivalent(&plain, &source);
}
macro_rules! source_case {
    ($test:ident, $case:literal) => {
        #[test]
        fn $test() {
            assert_case($case);
        }
    };
}

source_case!(v16_math_source_limits_base, "limits-base");
source_case!(v16_math_source_ordinary_base, "ordinary-base");
source_case!(v16_math_source_script, "script");
source_case!(v16_math_source_op_text, "op-text");
source_case!(v16_math_source_accent_base, "accent-base");
source_case!(v16_math_source_accent_i, "accent-i");
source_case!(v16_math_source_accent_nested, "accent-nested");
source_case!(v16_math_source_underover_base, "underover-base");
source_case!(v16_math_source_underover_piece, "underover-piece");
source_case!(v16_math_source_matrix_cell, "matrix-cell");
source_case!(v16_math_source_matrix_align_point, "matrix-align-point");
source_case!(v16_math_source_text_spacing, "text-spacing");
source_case!(v16_math_source_pipe_spacing, "pipe-spacing");
source_case!(v16_math_source_weak_space, "weak-space");
source_case!(v16_math_source_weak_between, "weak-between");
source_case!(v16_math_source_punctuation_tail, "punctuation-tail");
source_case!(v16_math_source_punctuation_source, "punctuation-source");
source_case!(v16_math_source_punctuation_next, "punctuation-next");
source_case!(v16_math_source_stretch_body, "stretch-body");
source_case!(v16_math_source_sequence_body, "sequence-body");
source_case!(v16_math_source_tighten_text, "tighten-text");
source_case!(v16_math_source_tighten_next, "tighten-next");
source_case!(v16_math_source_class_body, "class-body");

#[test]
fn v16_math_source_display_lines_keep_full_extent_metadata() {
    let body = seq(vec![text("a"), Content::linebreak(), text("b")]);
    let plain = observe(&body, true);
    let source = observe(&src(body, true), true);
    assert_eq!(plain.line_count, 2);
    assert!(plain.last > plain.first);
    equivalent(&plain, &source);
}
#[test]
#[should_panic(expected = "Unknown math observation")]
fn v16_math_source_observer_rejects_unknown_separator() {
    let style = TextStyle::regular(Pt(12.0));
    let mut observation = observe(&text("a"), false);
    visit(
        &[FrameItem::TextSeparator {
            pos: Point { x: Pt(0.0), y: Pt(0.0) },
            advance: Pt(0.0),
            style,
            origin: TextSeparatorOrigin::Markup,
            bidi: None,
        }],
        &mut observation,
    );
}
#[test]
fn v16_math_source_flatten_retains_first_causal_origins() {
    let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
    let first = Span::from_range(id, 10..11);
    let parent = Span::from_range(id, 20..21);
    let child = text("x").with_source_span(first);
    let container = seq(vec![child, text("y")]).with_source_span(parent);
    let mut flat = vec![];
    super::flatten_math_sequence_nodes(std::slice::from_ref(&container), &mut flat);
    assert_eq!(flat.len(), 2);
    assert_eq!(flat[0].source_span(), first);
    assert_eq!(flat[1].source_span(), parent);
    assert_eq!(container.source_span(), parent);
}

#[test]
fn v16_math_source_linebreak_marker_keeps_two_rows() {
    let plain = seq(vec![text("a"), Content::linebreak(), text("b")]);
    let source = seq(vec![text("a"), src(Content::linebreak(), true), text("b")]);
    let expected = observe(&plain, true);
    assert_eq!(expected.line_count, 2);
    equivalent(&expected, &observe(&source, true));
}
#[test]
fn v16_math_source_align_marker_keeps_two_aligned_columns() {
    let make = |on| {
        seq(vec![
            text("1"),
            src(Content::math_align_point(), on),
            text("2"),
            Content::linebreak(),
            text("333"),
            src(Content::math_align_point(), on),
            text("4"),
        ])
    };
    let expected = observe(&make(false), true);
    assert_eq!(expected.line_count, 2);
    equivalent(&expected, &observe(&make(true), true));
}
