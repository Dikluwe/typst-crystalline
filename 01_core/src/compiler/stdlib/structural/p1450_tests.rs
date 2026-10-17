//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1450-tests.md
//! @prompt-hash dd69b0bb
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::{
    p1450_math_underparen_observation_differs, repr::repr_content, EvalContext,
};
use crate::compiler::introspect::locatable::is_locatable;
use crate::compiler::math::layout::{p1450_boundary_classes, p1450_underparen_lowering};
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_underover::MathUnderparenElem;
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
    FileId::from_raw(NonZeroU16::new(1450).unwrap())
}

fn underparen_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("underparen") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.underparen callable: {other:?}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    underparen_func().native_fn_addr().expect("math.underparen native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else {
        panic!("underparen content")
    };
    assert!(matches!(content, Content::MathUnderparen(_)));
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
fn p1450_binding_cast_repr_presenca_e_nesting() {
    assert_eq!(underparen_func().name(), Some("underparen"));
    let omitted = content(Args::positional(vec![cv("x")]));
    assert_eq!(repr_content(&omitted), "underparen(body: [x])");
    let annotated = content(Args::positional(vec![cv("x"), cv("n")]));
    assert_eq!(repr_content(&annotated), "underparen(body: [x], annotation: [n])");
    let explicit_none = content(Args::positional(vec![cv("x"), Value::None]));
    assert_eq!(repr_content(&explicit_none), "underparen(body: [x], annotation: none)");
    assert_ne!(omitted, explicit_none);
    assert_eq!(omitted.get_field("body"), Some(cv("x")));
    assert_eq!(omitted.get_field("annotation"), None);
    assert_eq!(explicit_none.get_field("annotation"), Some(Value::None));
    let inner = content(Args::positional(vec![cv("x")]));
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Content(inner)]))),
        "underparen(body: underparen(body: [x]))"
    );
}

#[test]
fn p1450_assinatura_falha_fechado() {
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
fn p1450_carrier_walker_observation_classe_e_lowering() {
    let elem = MathUnderparenElem {
        body: Content::text("x"),
        annotation: Some(Content::text("n")),
        annotation_explicit: true,
    };
    assert_eq!(elem.plain_text(), "x⏝n");
    let mut rewrite = |content: &Content| -> SourceResult<Option<Content>> {
        Ok(matches!(content, Content::Text(_)).then(|| Content::text("y")))
    };
    let Content::MathUnderparen(mapped) = elem.map_content(&mut rewrite).unwrap() else {
        panic!("MathUnderparen")
    };
    assert_eq!(mapped.body, Content::text("y"));
    assert_eq!(mapped.annotation, Some(Content::text("y")));
    assert!(mapped.annotation_explicit);
    assert!(p1450_math_underparen_observation_differs(
        &elem,
        &MathUnderparenElem { annotation_explicit: false, ..elem.clone() },
    ));
    assert_eq!(
        p1450_boundary_classes(&Content::MathUnderparen(std::sync::Arc::new(
            elem.clone()
        ))),
        (MathClass::Alphabetic, MathClass::Alphabetic)
    );
    assert!(!is_locatable(&Content::MathUnderparen(std::sync::Arc::new(elem.clone()))));

    let lowered = p1450_underparen_lowering(&elem);
    let Content::MathUnderover(outer) = lowered else { panic!("annotation inferior") };
    assert_eq!(outer.under, elem.annotation);
    let Content::MathUnderover(inner) = &outer.base else { panic!("paren inferior") };
    assert_eq!(inner.base, elem.body);
    assert_eq!(inner.over, None);
    assert_eq!(inner.under.as_ref().map(Content::plain_text).as_deref(), Some("⏝"));
}
