//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1436-tests.md
//! @prompt-hash 8261ee2b
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::p1436_math_lr_observation_differs;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::compiler::math::layout::{
    p1436_boundary_classes, p1436_effective_size, p1436_target_pt,
};
use crate::contracts::world::World;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::content::Content;
use crate::entities::elements::math_delimited::MathLrElem;
use crate::entities::elements::Element;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::layout_types::Length;
use crate::entities::math_class::MathClass;
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
    FileId::from_raw(NonZeroU16::new(1436).unwrap())
}
fn lr_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("lr") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.lr callable: {other:?}"),
    }
}
fn call(args: Args) -> SourceResult<Value> {
    lr_func().native_fn_addr().expect("math.lr native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}
fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else { panic!("lr content") };
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
fn p1436_binding_zero_um_muitos_e_repr() {
    assert_eq!(lr_func().name(), Some("lr"));
    assert_eq!(repr_content(&content(Args::positional(vec![]))), "lr(body: [])");
    assert_eq!(repr_content(&content(Args::positional(vec![cv("x")]))), "lr(body: [x])");
    assert_eq!(
        repr_content(&content(Args::positional(vec![cv("x"), cv("y")]))),
        "lr(body: sequence([x], [,], [y]))"
    );
}

#[test]
fn p1436_size_presenca_e_carrier_distinto() {
    let omitted = content(Args::positional(vec![cv("x")]));
    let explicit = content(named(
        Args::positional(vec![cv("x")]),
        "size",
        Value::Relative(Rel::from_percent(100.0)),
    ));
    assert_ne!(omitted, explicit);
    assert_eq!(repr_content(&omitted), "lr(body: [x])");
    assert_eq!(repr_content(&explicit), "lr(size: 100% + 0pt, body: [x])");
    assert!(!matches!(explicit, Content::MathDelimited(_)));
}

#[test]
fn p1436_diagnosticos_fechados() {
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(1)])),
        "expected content, found integer"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![]), "size", Value::Int(1))),
        "expected relative length, found integer"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![]), "body", cv("x"))),
        "unexpected argument: body"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![]), "nope", Value::None)),
        "unexpected argument: nope"
    );
    let duplicate = Args::from_occurrences(
        Span::detached(),
        vec![
            occurrence(Some("size"), Value::Relative(Rel::from_percent(100.0))),
            occurrence(Some("size"), Value::Relative(Rel::from_percent(150.0))),
        ],
    );
    assert_eq!(diagnostic(duplicate), "duplicate argument: size");
}

#[test]
fn p1436_walker_preserva_size_e_presenca() {
    let elem = MathLrElem {
        body: Content::text("x"),
        size: Rel::from_percent(150.0),
        size_explicit: true,
    };
    let mut rewrite = |content: &Content| -> SourceResult<Option<Content>> {
        Ok(matches!(content, Content::Text(_)).then(|| Content::text("y")))
    };
    let Content::MathLr(mapped) = elem.map_content(&mut rewrite).unwrap() else {
        panic!("MathLr")
    };
    assert_eq!(mapped.body, Content::text("y"));
    assert_eq!(mapped.size, Rel::from_percent(150.0));
    assert!(mapped.size_explicit);
}

#[test]
fn p1436_nested_repr_observation_spacing_cascade_e_shortfall() {
    let inner = content(Args::positional(vec![cv("x")]));
    let outer = content(Args::positional(vec![Value::Content(inner)]));
    assert_eq!(repr_content(&outer), "lr(body: lr(body: [x]))");

    let a = MathLrElem {
        body: Content::MathText("+".into()),
        size: Rel::from_percent(150.0),
        size_explicit: true,
    };
    let b = MathLrElem { size: Rel::from_percent(175.0), ..a.clone() };
    assert!(p1436_math_lr_observation_differs(&a, &b));
    assert_eq!(
        p1436_boundary_classes(&Content::MathLr(std::sync::Arc::new(a.clone()))),
        (MathClass::Vary, MathClass::Vary)
    );
    assert_eq!(
        p1436_effective_size(&a, Some(Rel::from_percent(200.0))),
        Rel::from_percent(150.0)
    );
    let omitted = MathLrElem { size_explicit: false, ..a };
    assert_eq!(
        p1436_effective_size(&omitted, Some(Rel::from_percent(200.0))),
        Rel::from_percent(200.0)
    );
    assert_eq!(p1436_effective_size(&omitted, None), Rel::from_percent(100.0));
    assert!((p1436_target_pt(20.0, 10.0, Rel::from_percent(150.0)) - 29.0).abs() < 1e-9);
    assert!(
        (p1436_target_pt(20.0, 10.0, Rel { rel: 0.0, abs: Length::em(2.0) }) - 19.0)
            .abs()
            < 1e-9
    );
}
