//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1441-tests.md
//! @prompt-hash eb1a5ffe
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::{
    p1441_math_overline_observation_differs, repr::repr_content, EvalContext,
};
use crate::compiler::introspect::locatable::is_locatable;
use crate::compiler::math::layout::{p1441_boundary_classes, p1441_overline_geometry};
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_underline::MathOverlineElem;
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
    FileId::from_raw(NonZeroU16::new(1441).unwrap())
}

fn overline_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("overline") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.overline callable: {other:?}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    overline_func().native_fn_addr().expect("math.overline native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else { panic!("overline content") };
    assert!(matches!(content, Content::MathOverline(_)));
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
fn p1441_binding_cast_repr_e_nesting() {
    assert_eq!(overline_func().name(), Some("math.overline"));
    let plain = content(Args::positional(vec![cv("x")]));
    assert_eq!(repr_content(&plain), "overline(body: [x])");
    let none = content(Args::positional(vec![Value::None]));
    assert_eq!(repr_content(&none), "overline(body: [])");
    let string = content(Args::positional(vec![Value::Str("x".into())]));
    assert_eq!(repr_content(&string), "overline(body: [x])");
    let inner = content(Args::positional(vec![cv("x")]));
    assert_eq!(
        repr_content(&content(Args::positional(vec![Value::Content(inner)]))),
        "overline(body: overline(body: [x]))"
    );
}

#[test]
fn p1441_assinatura_falha_fechado() {
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
        diagnostic(named(Args::positional(vec![cv("x")]), "foo", Value::None)),
        "unexpected argument: foo"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(1)])),
        "expected content, found integer"
    );
}

#[test]
fn p1441_carrier_walker_observation_e_classe() {
    let elem = MathOverlineElem { body: Content::MathIdent("x".into()) };
    assert_eq!(elem.plain_text(), "x");
    let mut rewrite = |content: &Content| -> SourceResult<Option<Content>> {
        Ok(matches!(content, Content::MathIdent(_))
            .then(|| Content::MathIdent("y".into())))
    };
    let Content::MathOverline(mapped) = elem.map_content(&mut rewrite).unwrap() else {
        panic!("MathOverline")
    };
    assert_eq!(mapped.body, Content::MathIdent("y".into()));
    assert!(p1441_math_overline_observation_differs(
        &elem,
        &MathOverlineElem { body: Content::MathIdent("z".into()) },
    ));
    let wrapped = Content::MathOverline(std::sync::Arc::new(elem.clone()));
    assert_eq!(
        p1441_boundary_classes(&wrapped),
        (MathClass::Alphabetic, MathClass::Alphabetic)
    );
    assert!(!is_locatable(&wrapped));
}

#[test]
fn p1441_layout_cramped_overbar_largura_logica_sem_ajuste_italico() {
    let geometry = p1441_overline_geometry();
    assert!(geometry.body_cramped, "body de overline deve ser cramped");
    assert!(
        geometry.line_y < geometry.body_y,
        "regra deve ficar acima do body: {geometry:?}"
    );
    assert!((geometry.line_start_x - 0.0).abs() < 1e-9);
    assert!((geometry.line_end_x - geometry.width).abs() < 1e-9);
    assert!((geometry.line_thickness - 0.7).abs() < 1e-9);
    assert!(geometry.ascent > geometry.body_ascent);
    assert!((geometry.descent - geometry.body_descent).abs() < 1e-9);
}
