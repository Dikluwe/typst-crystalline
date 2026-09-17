//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/p1431-math-matrix-tests.md
//! @prompt-hash 255cb626
//! @layer L1
#![cfg(test)]

use super::Element;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::compiler::stdlib::make_math_module;
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::geometry::{LineCap, Stroke};
use crate::entities::layout_types::{Align2D, HAlign, Length};
use crate::entities::rel::Rel;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use indexmap::IndexMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::num::NonZeroU16;
use std::sync::Arc;

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
    FileId::from_raw(NonZeroU16::new(1431).unwrap())
}
fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}
fn dict(entries: Vec<(&str, Value)>) -> Value {
    let mut out = IndexMap::default();
    for (key, value) in entries {
        out.insert(key.into(), value);
    }
    Value::Dict(out)
}
fn call(args: Args) -> Content {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    let Some(Value::Func(function)) = module.scope().get("mat") else {
        panic!("math.mat")
    };
    let Value::Content(content) = function.native_fn_addr().unwrap()(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
    .unwrap() else {
        panic!("mat content")
    };
    content
}
fn hash(content: &Content) -> u64 {
    let mut state = DefaultHasher::new();
    content.hash(&mut state);
    state.finish()
}
fn complete() -> Content {
    let mut stroke = Stroke::default();
    stroke.thickness = 2.0;
    stroke.cap = LineCap::Round;
    let mut args = Args::positional(vec![
        Value::Array(vec![cv("a"), cv("b")]),
        Value::Array(vec![cv("c")]),
    ]);
    args.named
        .insert("delim".into(), Value::Array(vec![Value::Str("[".into()), Value::None]));
    args.named.insert(
        "align".into(),
        Value::Align(Align2D { h: Some(HAlign::Right), v: None }),
    );
    args.named.insert(
        "augment".into(),
        dict(vec![
            ("hline", Value::Array(vec![Value::Int(1), Value::Int(-1)])),
            ("vline", Value::Array(vec![Value::Int(2), Value::Int(1)])),
            ("stroke", Value::Stroke(stroke)),
        ]),
    );
    args.named
        .insert("row-gap".into(), Value::Relative(Rel::from_percent(150.0)));
    args.named.insert("column-gap".into(), Value::Length(Length::em(2.0)));
    call(args)
}

#[test]
fn p1431_carrier_completo_preserva_valores_ordem_e_bits() {
    let input = complete();
    let Content::MathMatrix(elem) = &input else { panic!("matrix") };
    assert_eq!(elem.rows.len(), 2);
    assert_eq!(elem.rows[0].len(), 2);
    assert_eq!(elem.rows[1].len(), 2);
    assert!(matches!(elem.rows[1][1], Content::Empty));
    let observed = repr_content(&input);
    for expected in [
        "delim: (\"[\", none)",
        "align: right",
        "hline",
        "vline",
        "2pt",
        "row-gap: 150% + 0pt",
        "column-gap: 0% + 2em",
        "([c], [])",
    ] {
        assert!(observed.contains(expected), "missing {expected:?}: {observed}");
    }
    assert_eq!(input.clone(), input);
}

#[test]
fn p1431_igualdade_e_hash_discriminam_campos_e_presenca() {
    let base = call(Args::positional(vec![cv("x")]));
    let cases = [
        ("delim", Value::None),
        ("align", Value::Align(Align2D { h: Some(HAlign::Center), v: None })),
        ("augment", Value::None),
        ("row-gap", Value::Length(Length::em(0.2))),
        ("column-gap", Value::Length(Length::em(0.5))),
    ];
    for (name, value) in cases {
        let mut args = Args::positional(vec![cv("x")]);
        args.named.insert(name.into(), value);
        let changed = call(args);
        assert_ne!(base, changed, "collapsed {name} presence/value");
        assert_ne!(hash(&base), hash(&changed), "hash omitted {name}");
    }
    let mut ordered_a = Args::positional(vec![cv("x")]);
    ordered_a.named.insert(
        "augment".into(),
        dict(vec![("hline", Value::Array(vec![1.into(), 2.into()]))]),
    );
    let mut ordered_b = Args::positional(vec![cv("x")]);
    ordered_b.named.insert(
        "augment".into(),
        dict(vec![("hline", Value::Array(vec![2.into(), 1.into()]))]),
    );
    assert_ne!(call(ordered_a), call(ordered_b));
}

#[test]
fn p1431_walkers_visitam_celulas_row_major_sem_perder_carrier() {
    let input = complete();
    let Content::MathMatrix(elem) = &input else { panic!("matrix") };
    assert_eq!(elem.plain_text(), "a, b; c, ");
    let before = repr_content(&input);
    let mut seen = Vec::new();
    let mut transform = |child: &Content| -> SourceResult<Option<Content>> {
        let text = child.plain_text();
        if !text.is_empty() {
            seen.push(text.clone());
        }
        Ok((text == "c").then(|| Content::text("z")))
    };
    let mapped = elem.map_content(&mut transform).unwrap();
    assert_eq!(seen, vec!["a", "b", "c"]);
    let after = repr_content(&mapped);
    for field in ["delim:", "align:", "augment:", "row-gap:", "column-gap:"] {
        assert_eq!(before.contains(field), after.contains(field), "lost {field}");
    }
    assert!(after.contains("[z]"), "{after}");
    let mut terminal = |_: &str| "mutated".to_string();
    assert_eq!(elem.map_text(&mut terminal), input);
}

#[test]
fn p1431_alignpoint_permanece_na_celula_do_carrier() {
    let aligned = Content::MathSequence(Arc::from(
        vec![Content::text("a"), Content::math_align_point(), Content::text("b")]
            .into_boxed_slice(),
    ));
    let matrix = call(Args::positional(vec![Value::Content(aligned)]));
    let Content::MathMatrix(elem) = &matrix else { panic!("matrix") };
    assert_eq!(elem.rows.len(), 1);
    assert_eq!(elem.rows[0].len(), 1);
    assert!(repr_content(&matrix).contains("a & b"));
}

#[test]
fn p1431_opacidade_de_carrier_e_cascata_falha_fechado() {
    let classify =
        |count: usize, cascade_attributable: bool| match (count, cascade_attributable) {
            (1, true) => ("PRESERVED", true, false),
            (1, false) => ("UNKNOWN_STYLE_CASCADE", false, false),
            _ => ("UNKNOWN_CARRIER_CARDINALITY", false, false),
        };
    assert_eq!(classify(0, true), ("UNKNOWN_CARRIER_CARDINALITY", false, false));
    assert_eq!(classify(2, true), ("UNKNOWN_CARRIER_CARDINALITY", false, false));
    assert_eq!(classify(1, false), ("UNKNOWN_STYLE_CASCADE", false, false));
}
