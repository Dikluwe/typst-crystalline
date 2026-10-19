//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1431-tests.md
//! @prompt-hash bccee650
//! @layer L1
#![cfg(test)]

use super::*;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::contracts::world::World;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::layout_types::{Align2D, HAlign, Length, VAlign};
use crate::entities::rel::Rel;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::span::Span;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use indexmap::IndexMap;
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
    FileId::from_raw(NonZeroU16::new(1431).unwrap())
}

fn mat_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("mat") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.mat must be callable, got {other:?}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    mat_func().native_fn_addr().expect("math.mat native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else { panic!("mat content") };
    assert!(matches!(content, Content::MathMatrix(_)));
    content
}

fn repr(args: Args) -> String {
    repr_content(&content(args))
}
fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}
fn row(values: Vec<Value>) -> Value {
    Value::Array(values)
}
fn named(mut args: Args, name: &str, value: Value) -> Args {
    args.named.insert(name.into(), value);
    args
}
fn diagnostic(args: Args) -> String {
    let errors = call(args).unwrap_err();
    assert_eq!(errors.len(), 1, "{errors:?}");
    errors[0].message.clone()
}
fn align(h: HAlign) -> Value {
    Value::Align(Align2D { h: Some(h), v: None })
}
fn occurrence(name: Option<&str>, value: Value) -> ArgOccurrence {
    ArgOccurrence {
        name: name.map(Into::into),
        value,
        span: Span::detached(),
        value_span: Span::detached(),
    }
}
fn duplicate(name: &str, a: Value, b: Value) -> Args {
    Args::from_occurrences(
        Span::detached(),
        vec![occurrence(Some(name), a), occurrence(Some(name), b)],
    )
}
fn dict(entries: Vec<(&str, Value)>) -> Value {
    let mut out = IndexMap::default();
    for (key, value) in entries {
        out.insert(key.into(), value);
    }
    Value::Dict(out)
}

#[test]
fn p1431_binding_rows_padding_e_repr_canonico() {
    assert_eq!(mat_func().name(), Some("mat"));
    assert_eq!(repr(Args::positional(vec![])), "mat(rows: ((),))");
    assert_eq!(
        repr(Args::positional(vec![cv("x"), cv("y"), cv("z")])),
        "mat(rows: (([x], [y], [z]),))"
    );
    assert_eq!(
        repr(Args::positional(vec![
            row(vec![Value::Int(1), Value::Int(2)]),
            row(vec![Value::Int(3)])
        ])),
        "mat(rows: (([1], [2]), ([3], [])))"
    );
    assert_eq!(
        repr(Args::positional(vec![row(vec![]), row(vec![cv("x"), cv("y")])])),
        "mat(rows: (([], []), ([x], [y])))"
    );
}

#[test]
fn p1431_delim_align_gaps_e_presenca_morfologica() {
    assert_eq!(
        repr(named(Args::positional(vec![]), "delim", Value::None)),
        "mat(delim: (none, none), rows: ((),))"
    );
    assert_eq!(
        repr(named(Args::positional(vec![]), "delim", Value::Str("[".into()))),
        "mat(delim: (\"[\", \"]\"), rows: ((),))"
    );
    assert_eq!(
        repr(named(
            Args::positional(vec![]),
            "delim",
            row(vec![Value::Str("[".into()), Value::None])
        )),
        "mat(delim: (\"[\", none), rows: ((),))"
    );
    for (value, spelling) in [
        (HAlign::Start, "start"),
        (HAlign::Left, "left"),
        (HAlign::Center, "center"),
        (HAlign::Right, "right"),
        (HAlign::End, "end"),
    ] {
        let observed = repr(named(Args::positional(vec![]), "align", align(value)));
        assert!(observed.contains(&format!("align: {spelling}")), "{observed}");
    }
    assert_eq!(
        repr(named(Args::positional(vec![]), "gap", Value::Length(Length::em(1.0)))),
        "mat(row-gap: 0% + 1em, column-gap: 0% + 1em, rows: ((),))"
    );
    let both_orders = [
        vec![
            ("gap", Value::Length(Length::em(1.0))),
            ("row-gap", Value::Length(Length::em(2.0))),
        ],
        vec![
            ("row-gap", Value::Length(Length::em(2.0))),
            ("gap", Value::Length(Length::em(1.0))),
        ],
    ];
    for entries in both_orders {
        let mut args = Args::positional(vec![]);
        for (name, value) in entries {
            args.named.insert(name.into(), value);
        }
        assert_eq!(
            repr(args),
            "mat(row-gap: 0% + 2em, column-gap: 0% + 1em, rows: ((),))"
        );
    }
    let relative = named(
        named(
            Args::positional(vec![]),
            "row-gap",
            Value::Relative(Rel::from_percent(150.0)),
        ),
        "column-gap",
        Value::Length(Length::em(2.0)),
    );
    assert_eq!(
        repr(relative),
        "mat(row-gap: 150% + 0pt, column-gap: 0% + 2em, rows: ((),))"
    );
}

#[test]
fn p1431_augment_inteiro_dict_none_e_ordem_canonica() {
    assert_eq!(
        repr(named(Args::positional(vec![]), "augment", Value::Int(2))),
        "mat(augment: 2, rows: ((),))"
    );
    assert_eq!(
        repr(named(Args::positional(vec![]), "augment", Value::Int(-1))),
        "mat(augment: -1, rows: ((),))"
    );
    assert_eq!(
        repr(named(Args::positional(vec![]), "augment", Value::None)),
        "mat(augment: none, rows: ((),))"
    );
    let observed = repr(named(
        named(
            named(
                named(
                    named(
                        Args::positional(vec![]),
                        "column-gap",
                        Value::Length(Length::em(2.0)),
                    ),
                    "augment",
                    dict(vec![
                        ("hline", row(vec![Value::Int(1), Value::Int(-1)])),
                        ("vline", row(vec![Value::Int(1), Value::Int(2)])),
                    ]),
                ),
                "align",
                align(HAlign::Right),
            ),
            "delim",
            Value::None,
        ),
        "row-gap",
        Value::Relative(Rel::from_percent(150.0)),
    ));
    let positions = ["delim:", "align:", "augment:", "row-gap:", "column-gap:", "rows:"]
        .map(|field| {
            observed
                .find(field)
                .unwrap_or_else(|| panic!("missing {field}: {observed}"))
        });
    assert!(positions.windows(2).all(|pair| pair[0] < pair[1]), "{observed}");
    assert!(observed.contains("hline") && observed.contains("vline"), "{observed}");
}

#[test]
fn p1431_augment_ignora_chaves_desconhecidas_do_dicionario() {
    let only_unknown = repr(named(
        Args::positional(vec![]),
        "augment",
        dict(vec![("foo", Value::Int(1))]),
    ));
    assert_eq!(
        only_unknown,
        "mat(augment: (hline: (), vline: (), stroke: auto), rows: ((),))"
    );

    let known_and_unknown = repr(named(
        Args::positional(vec![]),
        "augment",
        dict(vec![
            ("hline", Value::Int(-1)),
            ("vline", Value::Array(vec![Value::Int(1), Value::Int(2)])),
            ("foo", Value::Int(7)),
        ]),
    ));
    assert!(known_and_unknown.contains("hline"), "{known_and_unknown}");
    assert!(known_and_unknown.contains("vline"), "{known_and_unknown}");
    assert!(!known_and_unknown.contains("foo"), "{known_and_unknown}");
}

#[test]
fn p1431_rejeita_modos_tipos_named_e_duplicados() {
    let vertical = Value::Align(Align2D { h: None, v: Some(VAlign::Top) });
    for (args, fragment) in [
        (Args::positional(vec![row(vec![cv("x")]), cv("y")]), "expected array"),
        (
            named(Args::positional(vec![]), "delim", Value::Int(1)),
            "expected array, none, symbol, or string",
        ),
        (
            named(Args::positional(vec![]), "delim", row(vec![Value::Str("[".into())])),
            "exactly two",
        ),
        (
            named(
                Args::positional(vec![]),
                "delim",
                row(vec![Value::Str("[".into()), Value::Str("]".into()), Value::None]),
            ),
            "exactly two",
        ),
        (named(Args::positional(vec![]), "delim", Value::Str("[[".into())), "single"),
        (
            named(Args::positional(vec![]), "delim", Value::Str("a".into())),
            "invalid delimiter",
        ),
        (named(Args::positional(vec![]), "align", vertical), "start"),
        (
            named(Args::positional(vec![]), "gap", Value::Int(1)),
            "expected relative length",
        ),
        (
            named(Args::positional(vec![]), "row-gap", Value::Int(1)),
            "expected relative length",
        ),
        (
            named(Args::positional(vec![]), "column-gap", Value::Int(1)),
            "expected relative length",
        ),
        (
            named(Args::positional(vec![]), "augment", Value::Str("x".into())),
            "expected integer, dictionary, or none",
        ),
        (
            named(
                Args::positional(vec![]),
                "augment",
                dict(vec![("hline", Value::Str("x".into()))]),
            ),
            "integer",
        ),
        (
            named(
                Args::positional(vec![]),
                "augment",
                dict(vec![("hline", Value::Float(1.5))]),
            ),
            "integer",
        ),
        (
            named(
                Args::positional(vec![]),
                "augment",
                dict(vec![(
                    "hline",
                    Value::Array(vec![Value::Int(1), Value::Bool(true)]),
                )]),
            ),
            "integer",
        ),
        (
            named(
                Args::positional(vec![]),
                "augment",
                dict(vec![("vline", Value::Str("x".into()))]),
            ),
            "integer",
        ),
        (
            named(
                Args::positional(vec![]),
                "augment",
                dict(vec![("vline", Value::Float(1.5))]),
            ),
            "integer",
        ),
        (
            named(
                Args::positional(vec![]),
                "augment",
                dict(vec![(
                    "vline",
                    Value::Array(vec![Value::Int(1), Value::Bool(true)]),
                )]),
            ),
            "integer",
        ),
        (
            named(
                Args::positional(vec![]),
                "augment",
                dict(vec![("stroke", Value::Bool(true))]),
            ),
            "stroke",
        ),
        (
            named(Args::positional(vec![]), "foo", Value::Int(1)),
            "unexpected argument: foo",
        ),
        (
            named(Args::positional(vec![]), "rows", row(vec![])),
            "unexpected argument: rows",
        ),
    ] {
        let observed = diagnostic(args);
        assert!(observed.contains(fragment), "expected {fragment:?}: {observed}");
    }
    for (name, a, b) in [
        ("delim", Value::None, Value::Str("[".into())),
        ("align", align(HAlign::Left), align(HAlign::Right)),
        ("augment", Value::None, Value::Int(1)),
        ("gap", Value::Length(Length::em(1.0)), Value::Length(Length::em(2.0))),
        ("row-gap", Value::Length(Length::em(1.0)), Value::Length(Length::em(2.0))),
        ("column-gap", Value::Length(Length::em(1.0)), Value::Length(Length::em(2.0))),
    ] {
        assert_eq!(
            diagnostic(duplicate(name, a, b)),
            format!("duplicate argument: {name}")
        );
    }
}

#[test]
fn p1431_carrier_default_e_explicito_default_nao_colapsam() {
    let omitted = content(Args::positional(vec![]));
    let explicit = content(named(
        named(Args::positional(vec![]), "align", align(HAlign::Center)),
        "row-gap",
        Value::Length(Length::em(0.2)),
    ));
    assert_ne!(omitted, explicit);
    assert_eq!(repr_content(&omitted), "mat(rows: ((),))");
    assert!(repr_content(&explicit).contains("align: center"));
}
