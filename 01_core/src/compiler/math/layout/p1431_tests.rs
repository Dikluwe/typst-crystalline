//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/math/layout/p1431-tests.md
//! @prompt-hash 088accba
//! @layer L1
#![cfg(test)]

use super::*;
use crate::compiler::eval::{eval_for_test, repr::repr_content};
use crate::compiler::layout::FixedMetrics;
use crate::contracts::world::World;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::geometry::Stroke;
use crate::entities::location::Location;
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
        Content::Styled(inner, _) => find_mat(inner),
        Content::MathDelimited(elem) => find_mat(&elem.body),
        Content::Sequence(items) | Content::MathSequence(items) => {
            items.iter().find_map(find_mat)
        }
        _ => None,
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
        dict.insert("stroke".into(), Value::Stroke(stroke));
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
            _ => None,
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
            _ => None,
        })
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
}

#[test]
fn p1431_alignpoint_resolve_tarde_e_estilo_denominador_permanece() {
    let input = matrix("$mat(delim: #none, (1 & 2,), (33 & 4,))$");
    assert!(repr_content(&input).contains("1 & 2"));
    let items = texts(&project(&input));
    assert_eq!(text(&items, "2").1, text(&items, "4").1);
    for label in ["1", "2", "33", "4"] {
        assert!((text(&items, label).3 - 8.4).abs() < 1e-9, "{label}: {items:?}");
    }
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
}

#[test]
fn p1431_augment_stroke_custom_chega_a_todas_as_linhas() {
    let rendered = project(&matrix(
        "$mat(delim: #none, augment: #(hline: 1, vline: 1, stroke: 2pt + red), (a, b), (c, d))$",
    ));
    let lines = lines(&rendered);
    assert_eq!(lines.len(), 2);
    assert!(lines.iter().all(|line| (line.4 - 2.0).abs() < 1e-9));
    assert!(rendered
        .items
        .iter()
        .filter(|item| matches!(item, FrameItem::Line { .. }))
        .all(|item| { matches!(item, FrameItem::Line { color: Some(_), .. }) }));
}

#[test]
fn p1431_augment_cascata_dobra_stroke_sem_substituir_offsets_e_none_limpa() {
    let mut outer_stroke = Stroke::default();
    outer_stroke.thickness = 2.0;
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
    for rendered in [&omitted, &inner_auto] {
        let observed = lines(rendered);
        assert_eq!(observed.len(), 1);
        assert!((observed[0].4 - 2.0).abs() < 1e-9);
    }
    assert!(lines(&inner_none).is_empty());
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
}

#[test]
fn p1431_offsets_fora_da_dimensao_sao_diagnosticos() {
    for (source, offset, dimension) in [
        ("$mat(augment: #(hline: 3), (a, b), (c, d))$", "3", "2"),
        ("$mat(augment: #(hline: -3), (a, b), (c, d))$", "-3", "2"),
        ("$mat(augment: #(vline: 3), (a, b), (c, d))$", "3", "2"),
        ("$mat(augment: #(vline: -3), (a, b), (c, d))$", "-3", "2"),
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
}

#[test]
fn p1431_delimitadores_extents_e_centro_unico() {
    let with = project(&matrix("$mat((1,), (2,), (3,))$"));
    let without = project(&matrix("$mat(delim: #none, (1,), (2,), (3,))$"));
    assert!(with.width > without.width);
    assert!(with.ascent >= without.ascent && with.descent >= without.descent);
    let items = texts(&with);
    assert!(text(&items, "(").1.abs() < 1e-9);
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
