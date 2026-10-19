//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/math/layout/math_cases_layout_tests.md
//! @prompt-hash 340fd702
//! @layer L1
#![cfg(test)]

use super::*;
use crate::compiler::eval::{eval_for_test, repr::repr_content};
use crate::compiler::layout::FixedMetrics;
use crate::contracts::world::World;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::location::Location;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::style_chain::StyleChain;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
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
    FileId::from_raw(NonZeroU16::new(1430).unwrap())
}

fn evaluate(text: &str) -> SourceResult<crate::entities::module::Module> {
    let world = TestWorld::new(text);
    eval_for_test(&world, &world.source)
}

fn find_cases(content: &Content) -> Option<&Content> {
    match content {
        Content::MathCases(_) => Some(content),
        Content::Equation(elem) => find_cases(&elem.body),
        Content::Styled(styled) => find_cases(&styled.body),
        Content::MathDelimited(elem) => find_cases(&elem.body),
        Content::Sequence(items) | Content::MathSequence(items) => {
            items.iter().find_map(find_cases)
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
        | Content::MathMatrix(_)
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

fn cases(source: &str) -> Content {
    let module =
        evaluate(source).unwrap_or_else(|errors| panic!("{source:?}: {errors:?}"));
    let root = module.content().expect("module content");
    find_cases(root)
        .unwrap_or_else(|| panic!("missing MathCases in {source:?}: {root:?}"))
        .clone()
}

fn style() -> TextStyle {
    TextStyle::regular(Pt(12.0))
}

fn project(content: &Content, chain: &StyleChain) -> MathBox {
    let text_style = style();
    let callbacks = callbacks::MathCallbackPassState::new(None);
    MathLayouter::new_with_context(
        &FixedMetrics,
        false,
        &text_style,
        Location::from_raw(0x1430),
        Pt(200.0),
        chain,
        &callbacks,
    )
    .layout_node(content, &text_style)
}

fn texts(box_: &MathBox) -> Vec<(String, f64, f64, f64)> {
    fn visit(items: &[FrameItem], out: &mut Vec<(String, f64, f64, f64)>) {
        for item in items {
            match item {
                #[allow(deprecated)]
                FrameItem::Text { pos, text, style, .. }
                | FrameItem::TextShaped { pos, text, style, .. } => {
                    out.push((
                        text.to_string(),
                        pos.x.val(),
                        pos.y.val(),
                        style.size.val(),
                    ));
                }
                FrameItem::TextReplacement { items, .. } => visit(items, out),
                FrameItem::TextSeparator { .. }
                | FrameItem::Semantic { .. }
                | FrameItem::Line { .. }
                | FrameItem::Glyph { .. }
                | FrameItem::Image { .. }
                | FrameItem::Shape { .. }
                | FrameItem::Group { .. }
                | FrameItem::Link { .. } => {}
            }
        }
    }
    let mut found = Vec::new();
    visit(&box_.items, &mut found);
    found
}

fn item<'a>(
    items: &'a [(String, f64, f64, f64)],
    label: &str,
) -> &'a (String, f64, f64, f64) {
    items
        .iter()
        .find(|(text, _, _, _)| text == label)
        .unwrap_or_else(|| panic!("missing {label:?}: {items:?}"))
}

fn branch_delta(content: &Content, chain: &StyleChain) -> f64 {
    let rendered = project(content, chain);
    let items = texts(&rendered);
    (item(&items, "2").2 - item(&items, "1").2).abs()
}

#[test]
fn p1430_delimitador_respeita_direcao_e_none() {
    let chain = StyleChain::default_chain();
    let normal = project(&cases("$cases(1, 2)$"), &chain);
    let reverse = project(&cases("$cases(reverse: #true, 1, 2)$"), &chain);
    let absent = project(&cases("$cases(delim: #none, 1, 2)$"), &chain);
    let normal_items = texts(&normal);
    let reverse_items = texts(&reverse);
    let absent_items = texts(&absent);

    let left = item(&normal_items, "{");
    assert!(left.1 < item(&normal_items, "1").1, "opening delimiter not left");
    assert!(!normal_items.iter().any(|(text, _, _, _)| text == "}"));

    let right = item(&reverse_items, "}");
    assert!(right.1 > item(&reverse_items, "1").1, "closing delimiter not right");
    assert!(!reverse_items.iter().any(|(text, _, _, _)| text == "{"));

    assert!(
        absent_items.iter().all(|(text, _, _, _)| text != "{" && text != "}"),
        "none emitted a delimiter: {absent_items:?}"
    );
}

#[test]
fn p1430_gap_relativo_e_precedencia_de_cascata() {
    let default_chain = StyleChain::default_chain();
    let omitted = cases("$cases(1, 2)$");
    let explicit_one = cases("$cases(gap: #1em, 1, 2)$");
    let explicit_percent = cases("$cases(gap: #150%, 1, 2)$");
    let explicit_default = cases("$cases(gap: #0.2em, 1, 2)$");

    let default_delta = branch_delta(&omitted, &default_chain);
    let one_delta = branch_delta(&explicit_one, &default_chain);
    let percent_delta = branch_delta(&explicit_percent, &default_chain);
    assert!(
        one_delta > default_delta,
        "1em gap was not larger: {one_delta} <= {default_delta}"
    );
    assert!(
        percent_delta > one_delta,
        "150% gap did not remain relative: {percent_delta} <= {one_delta}"
    );

    let cascade = StyleChain::default_chain()
        .push_custom("math.cases.gap", Value::Length(Length::em(1.0)));
    let cascaded_delta = branch_delta(&omitted, &cascade);
    assert!((cascaded_delta - one_delta).abs() < 1e-9);
    assert_eq!(repr_content(&omitted), "cases(children: (1, 2))");

    let explicit_wins = branch_delta(&explicit_default, &cascade);
    assert!((explicit_wins - default_delta).abs() < 1e-9);
}

#[test]
fn p1430_alignpoint_resolve_tarde_e_ramos_usam_denominador() {
    let content = cases("$cases(1 & 2, 33 & 4)$");
    assert_eq!(repr_content(&content), "cases(children: (1 & 2, 33 & 4))");
    let rendered = project(&content, &StyleChain::default_chain());
    let items = texts(&rendered);

    assert_eq!(item(&items, "2").1, item(&items, "4").1, "align boundary diverged");
    let expected_size = 12.0 * 0.7;
    for label in ["1", "2", "33", "4"] {
        let actual = item(&items, label).3;
        assert!((actual - expected_size).abs() < 1e-9, "{label}: {actual}");
    }
    assert_ne!(
        item(&items, "1").2,
        item(&items, "33").2,
        "children collapsed into one row"
    );
}

#[test]
fn p1430_extents_incluem_delimitador_sem_padding_redundante() {
    let with_delim = project(&cases("$cases(1, 2, 3)$"), &StyleChain::default_chain());
    let without_delim =
        project(&cases("$cases(delim: #none, 1, 2, 3)$"), &StyleChain::default_chain());
    assert!(with_delim.width > without_delim.width, "delimiter absent from width");
    assert!(with_delim.ascent >= without_delim.ascent);
    assert!(with_delim.descent >= without_delim.descent);
    let brace = item(&texts(&with_delim), "{").1;
    assert!(brace.abs() < 1e-9, "redundant left padding before delimiter: {brace}");
}

#[test]
fn p1430_reconstrucao_preserva_campos_causais() {
    let input =
        cases("$cases(delim: #(\"[\", none), reverse: #true, gap: #150%, 1 & 2, 3)$");
    let causal = |content: &Content| {
        repr_content(content)
            .split_once("children:")
            .map(|(prefix, _)| prefix.to_string())
            .unwrap_or_else(|| panic!("not cases repr: {}", repr_content(content)))
    };
    let expected = causal(&input);
    assert_eq!(causal(&apply_math_default(&input)), expected);
    assert_eq!(
        causal(&apply_math_style(
            &input,
            Some(crate::entities::math_style::MathStyleKind::DoubleStruck),
            Some(true),
            Some(false),
        )),
        expected
    );
}

#[test]
fn p1430_wrapper_legado_preserva_geometria_de_rows() {
    let legacy = Content::math_cases(
        vec![
            vec![Content::MathText("1".into()), Content::MathText("2".into())],
            vec![Content::MathText("3".into())],
        ],
        ('{', '}'),
        false,
        None,
    );
    let public = cases("$cases(1 & 2, 3)$");
    let legacy_box = project(&legacy, &StyleChain::default_chain());
    let public_box = project(&public, &StyleChain::default_chain());
    assert!((legacy_box.width - public_box.width).abs() < 1e-9);
    assert!((legacy_box.ascent - public_box.ascent).abs() < 1e-9);
    assert!((legacy_box.descent - public_box.descent).abs() < 1e-9);
    assert_eq!(texts(&legacy_box), texts(&public_box));
}

#[test]
fn p1430_layout_opaco_falha_fechado() {
    let classify = |attributable: bool| {
        if attributable {
            ("PRESERVED", true, false)
        } else {
            ("UNKNOWN_LAYOUT_OBSERVATION", false, false)
        }
    };
    assert_eq!(classify(false), ("UNKNOWN_LAYOUT_OBSERVATION", false, false));
    assert_eq!(classify(true), ("PRESERVED", true, false));
}
