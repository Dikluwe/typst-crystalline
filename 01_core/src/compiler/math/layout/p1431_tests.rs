//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/p1431-tests.md
//! @prompt-hash fad95e1a
//! @layer L1
#![cfg(test)]

use super::*;
use crate::compiler::eval::{eval_for_test, repr::repr_content};
use crate::compiler::layout::{FixedMetrics, FontMetrics};
use crate::contracts::world::World;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::geometry::{LineCap, Stroke};
use crate::entities::location::Location;
use crate::entities::math_style::MathStyleKind;
use crate::entities::paint::Paint;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use indexmap::IndexMap;
use std::num::NonZeroU16;

struct TestWorld {
    library: Library,
    book: FontBook,
    source: Source,
}
impl TestWorld {
    fn new(text: &str) -> Self {
        Self {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(id(), text.into()),
        }
    }
}
impl World for TestWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        self.source.id()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Ok(self.source.clone())
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
    FileId::from_raw(NonZeroU16::new(1431).unwrap())
}
fn evaluate(text: &str) -> SourceResult<crate::entities::module::Module> {
    let world = TestWorld::new(text);
    eval_for_test(&world, &world.source)
}
fn find_mat(content: &Content) -> Option<&Content> {
    match content {
        Content::MathMatrix(_) => Some(content),
        Content::Equation(elem) => find_mat(&elem.body),
        Content::Styled(styled) => find_mat(&styled.body),
        Content::MathDelimited(elem) => find_mat(&elem.body),
        Content::Sequence(items) | Content::MathSequence(items) => {
            items.iter().find_map(find_mat)
        }
        Content::Source(_)
        | Content::ShowRecipe(_)
        | Content::Realized(_)
        | Content::Located(_)
        | Content::Empty
        | Content::Text(_)
        | Content::Space
        | Content::SpaceFromNewline
        | Content::Parbreak
        | Content::HtmlElem(_)
        | Content::HtmlFrame(_)
        | Content::Par { .. }
        | Content::Heading(_)
        | Content::Title(_)
        | Content::Strong(_)
        | Content::Emph(_)
        | Content::Raw(_)
        | Content::ListItem(_)
        | Content::EnumItem(_)
        | Content::Link(_)
        | Content::MathIdent(_)
        | Content::MathText(_)
        | Content::MathFrac(_)
        | Content::MathAttach(_)
        | Content::MathPrimes(_)
        | Content::MathRoot(_)
        | Content::MathStretch(_)
        | Content::MathLr(_)
        | Content::MathMid(_)
        | Content::MathOverbrace(_)
        | Content::MathUnderbrace(_)
        | Content::MathUnderbracket(_)
        | Content::MathUnderparen(_)
        | Content::MathUndershell(_)
        | Content::MathOverbracket(_)
        | Content::MathOverparen(_)
        | Content::MathOvershell(_)
        | Content::MathAlignPoint(_)
        | Content::Linebreak(_)
        | Content::MathCases(_)
        | Content::MathAccent(_)
        | Content::MathCancel(_)
        | Content::MathUnderline(_)
        | Content::MathOverline(_)
        | Content::MathVec(_)
        | Content::MathClassOverride(_)
        | Content::MathLimitsOverride(_)
        | Content::MathUnderover(_)
        | Content::MathOp(_)
        | Content::MathStyled(_)
        | Content::Label(_)
        | Content::Ref(_)
        | Content::CounterDisplay(_)
        | Content::CounterUpdate(_)
        | Content::Outline(_)
        | Content::OutlineEntry(_)
        | Content::Figure(_)
        | Content::FigureCaption(_)
        | Content::Image(_)
        | Content::Shape(_)
        | Content::Curve(_)
        | Content::Transform(_)
        | Content::Grid(_)
        | Content::GridHeader(_)
        | Content::GridFooter(_)
        | Content::GridCell(_)
        | Content::SetPage(_)
        | Content::PageRun(_)
        | Content::Align(_)
        | Content::Place(_)
        | Content::Flush(_)
        | Content::Divider(_)
        | Content::Terms(_)
        | Content::TermItem(_)
        | Content::Quote(_)
        | Content::Document { .. }
        | Content::Asset { .. }
        | Content::SmartQuote(_)
        | Content::PdfAttach(_)
        | Content::PdfArtifact(_)
        | Content::Underline(_)
        | Content::Strike(_)
        | Content::Overline(_)
        | Content::SmallCaps { .. }
        | Content::Pad(_)
        | Content::Hide(_)
        | Content::HSpace(_)
        | Content::VSpace(_)
        | Content::Pagebreak(_)
        | Content::Colbreak(_)
        | Content::Stack(_)
        | Content::Boxed(_)
        | Content::Block(_)
        | Content::TableCell(_)
        | Content::Bibliography(_)
        | Content::Cite(_)
        | Content::Footnote(_)
        | Content::FootnoteEntry(_)
        | Content::TableHeader(_)
        | Content::TableFooter(_)
        | Content::GridHLine(_)
        | Content::GridVLine(_)
        | Content::TableHLine(_)
        | Content::TableVLine(_)
        | Content::Table(_)
        | Content::Repeat(_)
        | Content::Columns(_)
        | Content::Metadata(_)
        | Content::State(_)
        | Content::StateUpdate(_)
        | Content::StateDisplay(_)
        | Content::CounterDisplayCallback(_)
        | Content::ContextBlock(_)
        | Content::Dynamic(_) => None,
    }
}
fn matrix(source: &str) -> Content {
    let module =
        evaluate(source).unwrap_or_else(|errors| panic!("{source:?}: {errors:?}"));
    let root = module.content().unwrap();
    find_mat(root)
        .unwrap_or_else(|| panic!("matrix missing: {root:?}"))
        .clone()
}
fn style() -> TextStyle {
    TextStyle::regular(Pt(12.0))
}
fn project(content: &Content) -> MathBox {
    project_with_chain(content, &StyleChain::default_chain())
}
fn project_with_chain(content: &Content, chain: &StyleChain) -> MathBox {
    let text_style = style();
    let callbacks = callbacks::MathCallbackPassState::new(None);
    MathLayouter::new_with_context(
        &FixedMetrics,
        false,
        &text_style,
        Location::from_raw(0x1431),
        Pt(200.0),
        chain,
        &callbacks,
    )
    .layout_node(content, &text_style)
}
fn augment_value(vline: i64, stroke: Option<Stroke>) -> Value {
    let mut dict = IndexMap::default();
    dict.insert("vline".into(), Value::Int(vline));
    if let Some(stroke) = stroke {
        dict.insert("stroke".into(), Value::from(stroke));
    }
    Value::Dict(dict)
}
fn texts(box_: &MathBox) -> Vec<(String, f64, f64, f64)> {
    box_.items
        .iter()
        .filter_map(|item| match item {
            #[allow(deprecated)]
            FrameItem::Text { pos, text, style, .. }
            | FrameItem::TextShaped { pos, text, style, .. } => {
                Some((text.to_string(), pos.x.val(), pos.y.val(), style.size.val()))
            }
            FrameItem::Semantic { .. }
            | FrameItem::Line { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. }
            | FrameItem::TextReplacement { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => None,
        })
        .collect()
}
fn text<'a>(
    items: &'a [(String, f64, f64, f64)],
    label: &str,
) -> &'a (String, f64, f64, f64) {
    items
        .iter()
        .find(|(value, _, _, _)| value == label)
        .unwrap_or_else(|| panic!("missing {label}: {items:?}"))
}
fn lines(box_: &MathBox) -> Vec<(f64, f64, f64, f64, f64)> {
    box_.items
        .iter()
        .filter_map(|item| match item {
            FrameItem::Line { start, end, thickness, .. } => {
                Some((start.x.val(), start.y.val(), end.x.val(), end.y.val(), *thickness))
            }
            FrameItem::Semantic { .. }
            | FrameItem::Group { .. }
            | FrameItem::Link { .. }
            | FrameItem::TextReplacement { .. }
            | FrameItem::TextSeparator { .. }
            | FrameItem::Text { .. }
            | FrameItem::TextShaped { .. }
            | FrameItem::Glyph { .. }
            | FrameItem::Image { .. }
            | FrameItem::Shape { .. } => None,
        })
        .collect()
}
fn line_debugs(box_: &MathBox) -> Vec<String> {
    box_.items
        .iter()
        .filter(|item| matches!(item, FrameItem::Line { .. }))
        .map(|item| format!("{item:?}"))
        .collect()
}

#[test]
fn p1431_alinhamento_horizontal_move_celula_sem_mudar_coluna() {
    let render = |align: &str| {
        project(&matrix(&format!(
            "$mat(delim: #none, align: #{align}, (x, q), (\"long\", z))$"
        )))
    };
    let left = texts(&render("left"));
    let center = texts(&render("center"));
    let right = texts(&render("right"));
    assert!(text(&left, "x").1 < text(&center, "x").1);
    assert!(text(&center, "x").1 < text(&right, "x").1);
    assert_eq!(text(&left, "q").2, text(&right, "q").2);
    let start = texts(&render("start"));
    let end = texts(&render("end"));
    assert_eq!(text(&start, "x").1, text(&left, "x").1);
    assert_eq!(text(&end, "x").1, text(&right, "x").1);
}

#[test]
fn p1431_gaps_relativos_afetam_somente_o_eixo_causal() {
    let default = texts(&project(&matrix("$mat(delim: #none, (a, b), (c, d))$")));
    let rows =
        texts(&project(&matrix("$mat(delim: #none, row-gap: #150%, (a, b), (c, d))$")));
    let cols =
        texts(&project(&matrix("$mat(delim: #none, column-gap: #2em, (a, b), (c, d))$")));
    let dx = |items: &[(String, f64, f64, f64)]| {
        (text(items, "b").1 - text(items, "a").1).abs()
    };
    let dy = |items: &[(String, f64, f64, f64)]| {
        (text(items, "c").2 - text(items, "a").2).abs()
    };
    assert!(dy(&rows) > dy(&default));
    assert!((dx(&rows) - dx(&default)).abs() < 1e-9);
    assert!(dx(&cols) > dx(&default));
    assert!((dy(&cols) - dy(&default)).abs() < 1e-9);

    let one_em = texts(&project(&matrix(
        "$mat(delim: #none, row-gap: #1em, column-gap: #1em, (a, b), (c, d))$",
    )));
    let percent = texts(&project(&matrix(
        "$mat(delim: #none, row-gap: #150%, column-gap: #150%, (a, b), (c, d))$",
    )));
    assert!(dx(&one_em) > dx(&default) && dy(&one_em) > dy(&default));
    assert!(dx(&percent) > dx(&default) && dy(&percent) > dy(&default));
    assert_ne!(
        dx(&one_em),
        dx(&percent),
        "P27 relative and absolute column gaps collapsed"
    );
    assert_ne!(dy(&one_em), dy(&percent), "P27 relative and absolute row gaps collapsed");
}

#[test]
fn p1431_alignpoint_resolve_tarde_e_estilo_denominador_permanece() {
    let input = matrix("$mat(delim: #none, (1 & 2,), (33 & 4,))$");
    assert!(repr_content(&input).contains("1 & 2"));
    let items = texts(&project(&input));
    let one = text(&items, "1");
    let thirty_three = text(&items, "33");
    let one_style = TextStyle::regular(Pt(one.3));
    let thirty_three_style = TextStyle::regular(Pt(thirty_three.3));
    let one_right = one.1 + FixedMetrics.text_width("1", Pt(one.3), &one_style).val();
    let thirty_three_right = thirty_three.1
        + FixedMetrics
            .text_width("33", Pt(thirty_three.3), &thirty_three_style)
            .val();
    assert!(
        (one_right - thirty_three_right).abs() < 1e-9,
        "P26 left subcolumn must right-align: {items:?}"
    );
    assert_eq!(text(&items, "2").1, text(&items, "4").1);
    for label in ["1", "2", "33", "4"] {
        assert!((text(&items, label).3 - 8.4).abs() < 1e-9, "{label}: {items:?}");
    }

    let uneven = texts(&project(&matrix("$mat(delim: #none, (1 & + & 3,), (44 & 5,))$")));
    for label in ["1", "+", "3", "44", "5"] {
        let _ = text(&uneven, label);
    }
    assert!(text(&uneven, "+").1 <= text(&uneven, "5").1);
    assert!(text(&uneven, "3").1 > text(&uneven, "+").1);

    for (kind, expected_size) in [
        (MathStyleKind::Display, 8.4),
        (MathStyleKind::Inline, 8.4),
        (MathStyleKind::Script, 6.0),
        (MathStyleKind::SScript, 6.0),
    ] {
        let styled = Content::math_styled(
            Some(kind),
            None,
            None,
            matrix("$mat(delim: #none, x)$"),
            None,
        );
        let observed = texts(&project(&styled));
        assert!(
            (text(&observed, "x").3 - expected_size).abs() < 1e-9,
            "P31 {kind:?}: {observed:?}"
        );
    }
    let cramped = Content::math_styled(
        Some(MathStyleKind::Display),
        None,
        None,
        matrix("$mat(delim: #none, x)$"),
        Some(true),
    );
    let cramped_items = texts(&project(&cramped));
    assert!((text(&cramped_items, "x").3 - 8.4).abs() < 1e-9);
}

#[test]
fn p1431_augment_emite_todas_as_linhas_bordas_e_negativos() {
    let positive = project(&matrix(
        "$mat(delim: #none, augment: #(hline: (0, 1, 2), vline: (0, 1, 2)), (a, b), (c, d))$",
    ));
    let negative = project(&matrix(
        "$mat(delim: #none, augment: #(hline: (-2, -1), vline: (-2, -1)), (a, b), (c, d))$",
    ));
    let all = lines(&positive);
    assert_eq!(all.len(), 6, "one line per ordered offset: {all:?}");
    assert_eq!(all.iter().filter(|(x1, _, x2, _, _)| (x1 - x2).abs() < 1e-9).count(), 3);
    assert_eq!(all.iter().filter(|(_, y1, _, y2, _)| (y1 - y2).abs() < 1e-9).count(), 3);
    let neg = lines(&negative);
    assert_eq!(neg.len(), 4);
    for line in neg {
        assert!(
            all.iter().any(|candidate| (candidate.0 - line.0).abs() < 1e-9
                && (candidate.1 - line.1).abs() < 1e-9
                && (candidate.2 - line.2).abs() < 1e-9
                && (candidate.3 - line.3).abs() < 1e-9),
            "negative offset did not coincide: {line:?} vs {all:?}"
        );
    }
    assert!(all.iter().all(|line| (line.4 - 0.6).abs() < 1e-9));

    let unordered = project(&matrix(
        "$mat(delim: #none, augment: #(hline: (2, 0, 2, 1), vline: (1, 2, 1, 0)), (a, b), (c, d))$",
    ));
    let unordered_lines = lines(&unordered);
    assert_eq!(unordered_lines.len(), 8, "M23 duplicate offsets were collapsed");
    let hcoords: Vec<_> = unordered_lines
        .iter()
        .filter(|(_, y1, _, y2, _)| (y1 - y2).abs() < 1e-9)
        .map(|line| line.1)
        .collect();
    let vcoords: Vec<_> = unordered_lines
        .iter()
        .filter(|(x1, _, x2, _, _)| (x1 - x2).abs() < 1e-9)
        .map(|line| line.0)
        .collect();
    assert_eq!(hcoords.len(), 4);
    assert_eq!(vcoords.len(), 4);
    assert!((hcoords[0] - hcoords[2]).abs() < 1e-9 && hcoords[0] != hcoords[1]);
    assert!((vcoords[0] - vcoords[2]).abs() < 1e-9 && vcoords[0] != vcoords[1]);

    let middle = project(&matrix(
        "$mat(delim: #none, augment: #(hline: 1, vline: 1), (a, b), (c, d))$",
    ));
    let middle_text = texts(&middle);
    let cell = project(&Content::text("a"));
    let expected_h = ((text(&middle_text, "a").2 + cell.descent)
        + (text(&middle_text, "c").2 - cell.ascent))
        / 2.0;
    let expected_v =
        ((text(&middle_text, "a").1 + cell.width) + text(&middle_text, "b").1) / 2.0;
    let middle_lines = lines(&middle);
    let observed_h = middle_lines
        .iter()
        .find(|(_, y1, _, y2, _)| (y1 - y2).abs() < 1e-9)
        .expect("one horizontal middle-gap line")
        .1;
    let observed_v = middle_lines
        .iter()
        .find(|(x1, _, x2, _, _)| (x1 - x2).abs() < 1e-9)
        .expect("one vertical middle-gap line")
        .0;
    assert!(
        (observed_h - expected_h).abs() < 1e-9,
        "M23 hline edge/midpoint: {observed_h} vs {expected_h}"
    );
    assert!(
        (observed_v - expected_v).abs() < 1e-9,
        "M23 vline edge/midpoint: {observed_v} vs {expected_v}"
    );

    let horizontal: Vec<_> = all
        .iter()
        .filter(|(_, y1, _, y2, _)| (y1 - y2).abs() < 1e-9)
        .map(|line| line.1)
        .collect();
    let vertical: Vec<_> = all
        .iter()
        .filter(|(x1, _, x2, _, _)| (x1 - x2).abs() < 1e-9)
        .map(|line| line.0)
        .collect();
    assert!(horizontal.iter().any(|y| y.abs() < 1e-9));
    assert!(horizontal
        .iter()
        .any(|y| (*y - (positive.ascent + positive.descent)).abs() < 1e-9));
    assert!(vertical.iter().any(|x| x.abs() < 1e-9));
    assert!(vertical.iter().any(|x| (*x - positive.width).abs() < 1e-9));
}

#[test]
fn p1431_augment_stroke_custom_chega_a_todas_as_linhas() {
    let rendered = project(&matrix(
        "$mat(delim: #none, augment: #(hline: 1, vline: 1, stroke: 2pt + red), (a, b), (c, d))$",
    ));
    let observed_lines = lines(&rendered);
    assert_eq!(observed_lines.len(), 2);
    assert!(observed_lines.iter().all(|line| (line.4 - 2.0).abs() < 1e-9));
    assert!(rendered
        .items
        .iter()
        .filter(|item| matches!(item, FrameItem::Line { .. }))
        .all(|item| { matches!(item, FrameItem::Line { color: Some(_), .. }) }));

    let resolved = Stroke {
        thickness: 3.0,
        paint: Paint::Solid(Color::rgb(255, 0, 0)),
        cap: LineCap::Round,
        ..Stroke::default()
    };
    let chain = StyleChain::default_chain()
        .push_custom("math.mat.augment", augment_value(1, Some(resolved)));
    let custom = project_with_chain(
        &matrix("$mat(delim: #none, augment: #1, (a, b), (c, d))$"),
        &chain,
    );
    assert!(lines(&custom).iter().all(|line| (line.4 - 3.0).abs() < 1e-9));
    let debug = line_debugs(&custom).join(" | ");
    assert!(
        custom
            .items
            .iter()
            .filter_map(|item| match item {
                FrameItem::Line { color, .. } => color.clone(),
                _ => None,
            })
            .all(|color| color == Color::rgb(255, 0, 0)),
        "P29 color: {debug}"
    );
    assert!(debug.contains("Round"), "P29/P34 custom cap: {debug}");

    let butt_stroke = Stroke {
        thickness: 3.0,
        cap: LineCap::Butt,
        ..Stroke::default()
    };
    let butt_chain = StyleChain::default_chain()
        .push_custom("math.mat.augment", augment_value(1, Some(butt_stroke)));
    let butt = project_with_chain(
        &matrix("$mat(delim: #none, augment: #1, (a, b), (c, d))$"),
        &butt_chain,
    );
    assert!(line_debugs(&butt).join(" | ").contains("Butt"), "P34 custom Butt");

    let auto = project(&matrix(
        "$mat(delim: #none, augment: #(hline: 1, vline: 1), (a, b), (c, d))$",
    ));
    let auto_debug = line_debugs(&auto).join(" | ");
    assert!(auto_debug.contains("Square"), "P29/P34 auto cap: {auto_debug}");
}

#[test]
fn p1431_augment_cascata_dobra_stroke_sem_substituir_offsets_e_none_limpa() {
    let mut outer_stroke = Stroke {
        thickness: 2.0,
        paint: Paint::Solid(Color::rgb(255, 0, 0)),
        cap: LineCap::Round,
        ..Stroke::default()
    };
    outer_stroke.specified.paint = true;
    outer_stroke.specified.thickness = true;
    outer_stroke.specified.cap = true;
    let chain = StyleChain::default_chain()
        .push_custom("math.mat.augment", augment_value(1, Some(outer_stroke)));
    let omitted =
        project_with_chain(&matrix("$mat(delim: #none, (a, b), (c, d))$"), &chain);
    let inner_auto = project_with_chain(
        &matrix("$mat(delim: #none, augment: #1, (a, b), (c, d))$"),
        &chain,
    );
    let inner_none = project_with_chain(
        &matrix("$mat(delim: #none, augment: #none, (a, b), (c, d))$"),
        &chain,
    );
    let inner_custom = project_with_chain(
        &matrix(
            "$mat(delim: #none, augment: #(hline: 1, stroke: 3pt + blue), (a, b), (c, d))$",
        ),
        &chain,
    );
    for rendered in [&omitted, &inner_auto] {
        let observed = lines(rendered);
        assert_eq!(observed.len(), 1);
        assert!((observed[0].4 - 2.0).abs() < 1e-9);
    }
    assert!(lines(&inner_none).is_empty());
    let folded = lines(&inner_custom);
    assert_eq!(folded.len(), 1);
    assert!((folded[0].4 - 3.0).abs() < 1e-9, "inner thickness must win: {folded:?}");
    let folded_debug = line_debugs(&inner_custom).join(" | ");
    assert!(folded_debug.contains("Round"), "outer cap must fold: {folded_debug}");
    assert!(
        folded_debug.contains("255") || folded_debug.contains("blue"),
        "inner paint: {folded_debug}"
    );
}

#[test]
fn p1431_reconstrucoes_preservam_todos_os_campos_causais() {
    let input = matrix(
        "$mat(delim: #(\"[\", none), align: #right, augment: #(hline: (1, -1), vline: 1), row-gap: #150%, column-gap: #2em, (1 & 2, 3), (4, 5))$",
    );
    let expected = repr_content(&input);
    for field in ["delim:", "align:", "augment:", "row-gap:", "column-gap:", "rows:"] {
        assert!(expected.contains(field), "missing {field}: {expected}");
    }
    assert_eq!(repr_content(&apply_math_default(&input)), expected);
    assert_eq!(
        repr_content(&apply_math_style(
            &input,
            Some(crate::entities::math_style::MathStyleKind::DoubleStruck),
            Some(true),
            Some(false),
        )),
        expected
    );

    let legacy = Content::math_matrix(
        vec![
            vec![Content::text("a"), Content::text("b")],
            vec![Content::text("c"), Content::text("d")],
        ],
        ('(', ')'),
    );
    let legacy_repr = repr_content(&legacy);
    assert!(
        legacy_repr.starts_with("mat("),
        "M26 wrapper carrier identity: {legacy_repr}"
    );
    for witness in ["rows:", "a", "b", "c", "d"] {
        assert!(
            legacy_repr.contains(witness),
            "M26 wrapper missing {witness}: {legacy_repr}"
        );
    }
    let common = matrix("$mat((a, b), (c, d))$");
    let legacy_box = project(&legacy);
    let common_box = project(&common);
    assert!((legacy_box.width - common_box.width).abs() < 1e-9);
    assert!((legacy_box.ascent - common_box.ascent).abs() < 1e-9);
    assert!((legacy_box.descent - common_box.descent).abs() < 1e-9);
    assert_eq!(texts(&legacy_box), texts(&common_box), "M26 legacy wrapper geometry");
}

#[test]
fn p1431_offsets_fora_da_dimensao_sao_diagnosticos() {
    for (source, offset, dimension) in [
        ("$mat(augment: #(hline: 3), (a, b), (c, d))$", "3", "2"),
        ("$mat(augment: #(hline: -3), (a, b), (c, d))$", "-3", "2"),
        ("$mat(augment: #(vline: 3), (a, b), (c, d))$", "3", "2"),
        ("$mat(augment: #(vline: -3), (a, b), (c, d))$", "-3", "2"),
        ("$mat(augment: #(hline: 3), (a, b, c), (d, e, f))$", "3", "2"),
        ("$mat(augment: #(vline: 4), (a, b, c), (d, e, f))$", "4", "3"),
    ] {
        let errors = evaluate(source).unwrap_err();
        let message = errors
            .iter()
            .map(|e| e.message.as_str())
            .collect::<Vec<_>>()
            .join(" | ");
        assert!(
            message.contains(offset) && message.contains(dimension),
            "{source}: {message}"
        );
    }

    let wide = project(&matrix(
        "$mat(delim: #none, augment: #(hline: 2, vline: 3), (a, b, c), (d, e, f))$",
    ));
    assert_eq!(
        lines(&wide).len(),
        2,
        "2x3 bounds must use rows for hline and columns for vline"
    );
    let tall = project(&matrix(
        "$mat(delim: #none, augment: #(hline: 3, vline: 2), (a, b), (c, d), (e, f))$",
    ));
    assert_eq!(
        lines(&tall).len(),
        2,
        "3x2 bounds must use rows for hline and columns for vline"
    );
}

#[test]
fn p1431_delimitadores_extents_e_centro_unico() {
    let with = project(&matrix("$mat((1,), (2,), (3,))$"));
    let without = project(&matrix("$mat(delim: #none, (1,), (2,), (3,))$"));
    assert!(with.width > without.width);
    assert!(with.ascent >= without.ascent && with.descent >= without.descent);
    let items = texts(&with);
    assert!(text(&items, "(").1.abs() < 1e-9);

    let left_only = project(&matrix("$mat(delim: #(\"[\", none), (1,), (2,), (3,))$"));
    let right_only = project(&matrix("$mat(delim: #(none, \"]\"), (1,), (2,), (3,))$"));
    assert!(left_only.width > without.width && right_only.width > without.width);
    assert!(left_only.ascent >= without.ascent && left_only.descent >= without.descent);
    assert!(right_only.ascent >= without.ascent && right_only.descent >= without.descent);
    assert!(texts(&left_only).iter().any(|item| item.0 == "["));
    assert!(texts(&right_only).iter().any(|item| item.0 == "]"));
}

#[test]
fn p1431_layout_e_stroke_opacos_falham_fechado() {
    let classify = |layout: bool, stroke: bool| match (layout, stroke) {
        (true, true) => ("PRESERVED", true, false),
        (false, _) => ("UNKNOWN_LAYOUT_OBSERVATION", false, false),
        (_, false) => ("UNKNOWN_STROKE_OBSERVATION", false, false),
    };
    assert_eq!(classify(false, true), ("UNKNOWN_LAYOUT_OBSERVATION", false, false));
    assert_eq!(classify(true, false), ("UNKNOWN_STROKE_OBSERVATION", false, false));
}
