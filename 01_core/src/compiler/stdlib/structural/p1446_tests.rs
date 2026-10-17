//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1446-tests.md
//! @prompt-hash cc609bcd
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::contracts::world::World;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::content::Content;
use crate::entities::elements::Element;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::span::Span;
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
    FileId::from_raw(NonZeroU16::new(1446).unwrap())
}

fn stretch_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("stretch") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.stretch callable: {other:?}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    stretch_func().native_fn_addr().expect("math.stretch native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else { panic!("stretch content") };
    assert!(matches!(content, Content::MathStretch(_)));
    content
}

fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}
fn named(mut args: Args, name: &str, value: Value) -> Args {
    args.named.insert(name.into(), value);
    args
}
fn occurrence(name: Option<&str>, value: Value) -> ArgOccurrence {
    ArgOccurrence {
        name: name.map(Into::into),
        value,
        span: Span::detached(),
        value_span: Span::detached(),
    }
}
fn diagnostic(args: Args) -> String {
    call(args).unwrap_err()[0].message.clone()
}

#[test]
fn p1446_binding_carrier_casts_repr_e_nesting() {
    assert_eq!(stretch_func().name(), Some("stretch"));
    assert_eq!(
        repr_content(&content(Args::positional(vec![cv("x")]))),
        "stretch(body: [x])"
    );
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::None]))),
        "stretch(body: [])"
    );
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Str("x".into())]))),
        "stretch(body: [x])"
    );
    let inner = content(Args::positional(vec![cv("x")]));
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Content(inner)]))),
        "stretch(body: stretch(body: [x]))"
    );
}

#[test]
fn p1446_size_presenca_fields_e_mapeamento() {
    let omitted = content(Args::positional(vec![cv("x")]));
    let explicit = content(named(
        Args::positional(vec![cv("x")]),
        "size",
        Value::Relative(Rel::from_percent(100.0)),
    ));
    assert_ne!(omitted, explicit);
    assert_eq!(repr_content(&explicit), "stretch(body: [x], size: 100% + 0pt)");
    let Content::MathStretch(elem) = explicit else { unreachable!() };
    assert_eq!(elem.get_field("body"), Some(cv("x")));
    assert_eq!(elem.get_field("size"), Some(Value::Relative(Rel::from_percent(100.0))));
    let mixed = Rel { rel: 1.5, abs: Length::pt(2.0) };
    let Content::MathStretch(mixed_elem) =
        content(named(Args::positional(vec![cv("x")]), "size", Value::Relative(mixed)))
    else {
        unreachable!()
    };
    assert_eq!(mixed_elem.size, mixed);
    let mut rewrite = |c: &Content| -> SourceResult<Option<Content>> {
        Ok((c == &Content::text("x")).then(|| Content::text("z")))
    };
    assert_eq!(
        repr_content(&mixed_elem.map_content(&mut rewrite).unwrap()),
        "stretch(body: [z], size: 150% + 2pt)"
    );
}

#[test]
fn p1446_assinatura_falha_fechada() {
    assert_eq!(diagnostic(Args::positional(vec![])), "missing argument: body");
    assert_eq!(
        diagnostic(Args::positional(vec![cv("x"), cv("y")])),
        "unexpected argument"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![]), "body", cv("x"))),
        "the argument `body` is positional"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("x")]), "foo", Value::Int(1))),
        "unexpected argument: foo"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(1)])),
        "expected content, found integer"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("x")]), "size", Value::Int(1))),
        "expected relative length, found integer"
    );
    let duplicate = Args::from_occurrences(
        Span::detached(),
        vec![
            occurrence(Some("size"), Value::Relative(Rel::from_percent(100.0))),
            occurrence(Some("size"), Value::Relative(Rel::from_percent(150.0))),
            occurrence(None, cv("x")),
        ],
    );
    assert_eq!(diagnostic(duplicate), "duplicate argument: size");
}

#[test]
fn p1446_absoluto_e_default_sao_preservados() {
    let Content::MathStretch(default) = content(Args::positional(vec![cv("x")])) else {
        unreachable!()
    };
    assert_eq!(default.size, Rel::from_percent(100.0));
    assert!(!default.size_explicit);
    assert_eq!(default.get_field("size"), None);
    let Content::MathStretch(absolute) = content(named(
        Args::positional(vec![cv("x")]),
        "size",
        Value::Length(Length::pt(12.0)),
    )) else {
        unreachable!()
    };
    assert_eq!(absolute.size, Rel { rel: 0.0, abs: Length::pt(12.0) });
    assert!(absolute.size_explicit);
}
