//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1447-tests.md
//! @prompt-hash 4a56aab7
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::contracts::world::World;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
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
    FileId::from_raw(NonZeroU16::new(1447).unwrap())
}

fn abs_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("abs") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.abs callable: {other:?}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    abs_func().native_fn_addr().expect("math.abs native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else { panic!("abs content") };
    assert!(matches!(content, Content::MathLr(_)));
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
fn p1447_binding_casts_morfologia_e_repr() {
    assert_eq!(abs_func().name(), Some("abs"));
    assert_eq!(
        repr_content(&content(Args::positional(vec![cv("x")]))),
        "lr(body: sequence([|], [x], [|]))"
    );
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::None]))),
        "lr(body: sequence([|], [], [|]))"
    );
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Str("x".into())]))),
        "lr(body: sequence([|], [x], [|]))"
    );
}

#[test]
fn p1447_size_presenca_carrier_e_nesting() {
    let omitted = content(Args::positional(vec![cv("x")]));
    let explicit = content(named(
        Args::positional(vec![cv("x")]),
        "size",
        Value::Relative(Rel::from_percent(100.0)),
    ));
    assert_ne!(omitted, explicit);
    assert_eq!(
        repr_content(&explicit),
        "lr(size: 100% + 0pt, body: sequence([|], [x], [|]))"
    );
    assert!(!matches!(explicit, Content::MathDelimited(_)));

    let inner = content(Args::positional(vec![cv("x")]));
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Content(inner)]))),
        "lr(body: sequence([|], lr(body: sequence([|], [x], [|])), [|]))"
    );
}

#[test]
fn p1447_assinatura_e_diagnosticos_falham_fechado() {
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
        diagnostic(named(Args::positional(vec![cv("x")]), "nope", Value::None)),
        "unexpected argument: nope"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(1)])),
        "expected content, found integer"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("x")]), "size", Value::Bool(true))),
        "expected relative length, found boolean"
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
fn p1447_barras_sao_folhas_do_body_math_lr() {
    let Content::MathLr(abs) = content(Args::positional(vec![cv("x")])) else {
        panic!("MathLr")
    };
    let Content::MathSequence(nodes) = &abs.body else { panic!("sequence") };
    assert_eq!(nodes.len(), 3);
    assert_eq!(nodes[0].plain_text(), "|");
    assert_eq!(nodes[1], Content::text("x"));
    assert_eq!(nodes[2].plain_text(), "|");
    assert_eq!(abs.size, Rel::from_percent(100.0));
    assert!(!abs.size_explicit);
}

#[test]
fn p1447_campos_do_carrier_preservam_presenca_causal() {
    let omitted_content = content(Args::positional(vec![cv("x")]));
    let Content::MathLr(omitted) = &omitted_content else { panic!("MathLr") };
    assert_eq!(
        omitted_content.get_field("body"),
        Some(Value::Content(omitted.body.clone()))
    );
    assert_eq!(omitted_content.get_field("size"), None);

    let explicit_content = content(named(
        Args::positional(vec![cv("x")]),
        "size",
        Value::Relative(Rel::from_percent(150.0)),
    ));
    let Content::MathLr(_) = &explicit_content else { panic!("MathLr") };
    assert_eq!(
        explicit_content.get_field("size"),
        Some(Value::Relative(Rel::from_percent(150.0)))
    );
}
