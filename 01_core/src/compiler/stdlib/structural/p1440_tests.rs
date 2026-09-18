//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1440-tests.md
//! @prompt-hash c9c8cf1d
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::{
    p1440_math_overbracket_observation_differs, repr::repr_content, EvalContext,
};
use crate::compiler::introspect::locatable::is_locatable;
use crate::compiler::math::layout::{p1440_boundary_classes, p1440_overbracket_lowering};
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_underover::MathOverbracketElem;
use crate::entities::elements::Element;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::math_class::MathClass;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
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
    FileId::from_raw(NonZeroU16::new(1440).unwrap())
}

fn overbracket_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("overbracket") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.overbracket callable: {other:?}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    overbracket_func().native_fn_addr().expect("math.overbracket native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else {
        panic!("overbracket content")
    };
    assert!(matches!(content, Content::MathOverbracket(_)));
    content
}

fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}

fn named(mut args: Args, name: &str, value: Value) -> Args {
    args.named.insert(name.into(), value);
    args
}

fn diagnostic(args: Args) -> String {
    call(args).unwrap_err()[0].message.clone()
}

#[test]
fn p1440_binding_cast_repr_presenca_e_nesting() {
    assert_eq!(overbracket_func().name(), Some("overbracket"));
    let omitted = content(Args::positional(vec![cv("x")]));
    assert_eq!(repr_content(&omitted), "overbracket(body: [x])");
    let annotated = content(Args::positional(vec![cv("x"), cv("n")]));
    assert_eq!(repr_content(&annotated), "overbracket(body: [x], annotation: [n])");
    let explicit_none = content(Args::positional(vec![cv("x"), Value::None]));
    assert_eq!(repr_content(&explicit_none), "overbracket(body: [x], annotation: none)");
    assert_ne!(omitted, explicit_none);
    let inner = content(Args::positional(vec![cv("x")]));
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Content(inner)]))),
        "overbracket(body: overbracket(body: [x]))"
    );
}

#[test]
fn p1440_assinatura_falha_fechado() {
    assert_eq!(diagnostic(Args::positional(vec![])), "missing argument: body");
    assert_eq!(
        diagnostic(Args::positional(vec![cv("x"), cv("n"), cv("z")])),
        "unexpected argument"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![]), "body", cv("x"))),
        "the argument `body` is positional"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("x")]), "annotation", cv("n"))),
        "unexpected argument: annotation"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("x")]), "foo", Value::None)),
        "unexpected argument: foo"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(1)])),
        "expected content, found integer"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![cv("x"), Value::Int(1)])),
        "unexpected argument"
    );
}

#[test]
fn p1440_carrier_walker_observation_classe_e_lowering() {
    let elem = MathOverbracketElem {
        body: Content::text("x"),
        annotation: Some(Content::text("n")),
        annotation_explicit: true,
    };
    assert_eq!(elem.plain_text(), "n⎴x");
    let mut rewrite = |content: &Content| -> SourceResult<Option<Content>> {
        Ok(matches!(content, Content::Text(_)).then(|| Content::text("y")))
    };
    let Content::MathOverbracket(mapped) = elem.map_content(&mut rewrite).unwrap() else {
        panic!("MathOverbracket")
    };
    assert_eq!(mapped.body, Content::text("y"));
    assert_eq!(mapped.annotation, Some(Content::text("y")));
    assert!(mapped.annotation_explicit);
    assert!(p1440_math_overbracket_observation_differs(
        &elem,
        &MathOverbracketElem { annotation_explicit: false, ..elem.clone() },
    ));
    assert_eq!(
        p1440_boundary_classes(&Content::MathOverbracket(std::sync::Arc::new(
            elem.clone()
        ))),
        (MathClass::Alphabetic, MathClass::Alphabetic)
    );
    assert!(!is_locatable(&Content::MathOverbracket(std::sync::Arc::new(elem.clone()))));

    let lowered = p1440_overbracket_lowering(&elem);
    let Content::MathUnderover(outer) = lowered else { panic!("annotation superior") };
    assert_eq!(outer.over, elem.annotation);
    let Content::MathUnderover(inner) = &outer.base else { panic!("barra superior") };
    assert_eq!(inner.base, elem.body);
    assert_eq!(inner.under, None);
    assert_eq!(inner.over.as_ref().map(Content::plain_text).as_deref(), Some("⎴"));
}
