//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1437-tests.md
//! @prompt-hash 14a9d99b
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::{
    p1437_math_mid_observation_differs, repr::repr_content, EvalContext,
};
use crate::compiler::introspect::locatable::is_locatable;
use crate::compiler::math::layout::{
    p1437_boundary_classes, p1437_lr_mid_height, p1437_mid_target_pt,
};
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_delimited::MathMidElem;
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
    FileId::from_raw(NonZeroU16::new(1437).unwrap())
}
fn mid_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("mid") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.mid callable: {other:?}"),
    }
}
fn call(args: Args) -> SourceResult<Value> {
    mid_func().native_fn_addr().expect("math.mid native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}
fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else { panic!("mid content") };
    assert!(matches!(content, Content::MathMid(_)));
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
fn p1437_binding_cast_repr_e_nesting() {
    assert_eq!(mid_func().name(), Some("mid"));
    assert_eq!(repr_content(&content(Args::positional(vec![cv("|")]))), "mid(body: [|])");
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::None]))),
        "mid(body: [])"
    );
    let inner = content(Args::positional(vec![cv("|")]));
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Content(inner)]))),
        "mid(body: mid(body: [|]))"
    );
}

#[test]
fn p1437_aridade_named_e_tipo_falham_fechado() {
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
}

#[test]
fn p1437_carrier_walker_observation_e_classe() {
    let elem = MathMidElem { body: Content::text("x") };
    assert_eq!(elem.plain_text(), "x");
    let mut rewrite = |content: &Content| -> SourceResult<Option<Content>> {
        Ok(matches!(content, Content::Text(_)).then(|| Content::text("y")))
    };
    let Content::MathMid(mapped) = elem.map_content(&mut rewrite).unwrap() else {
        panic!("MathMid")
    };
    assert_eq!(mapped.body, Content::text("y"));
    assert!(p1437_math_mid_observation_differs(
        &elem,
        &MathMidElem { body: Content::text("z") },
    ));
    assert_eq!(
        p1437_boundary_classes(&Content::MathMid(std::sync::Arc::new(elem))),
        (MathClass::Relation, MathClass::Relation)
    );
    assert!(!is_locatable(&Content::math_mid(Content::text("x"))));
}

#[test]
fn p1437_meta_geometrica_usa_shortfall_do_lr() {
    assert!((p1437_mid_target_pt(20.0, 10.0, 1.0) - 19.0).abs() < 1e-9);
    assert!((p1437_mid_target_pt(20.0, 10.0, 1.5) - 29.0).abs() < 1e-9);
    assert!(p1437_lr_mid_height(3.0, None) > p1437_lr_mid_height(1.0, None));
    assert_eq!(
        p1437_lr_mid_height(1.0, Some(1.0)),
        p1437_lr_mid_height(3.0, Some(1.0)),
        "o lr interior deve bloquear a meta do lr exterior"
    );
}
