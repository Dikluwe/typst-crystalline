//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math_primes_tests.md
//! @prompt-hash 8763d653
//! @layer L1
#![cfg(test)]

use super::math::make_math_module;
use crate::compiler::eval::{
    p1444_math_primes_observation_differs, repr::repr_content, EvalContext,
};
use crate::compiler::introspect::locatable::is_locatable;
use crate::compiler::math::layout::{p1444_boundary_classes, p1444_primes_recipe};
use crate::contracts::world::World;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::math_attach::MathPrimesElem;
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
    FileId::from_raw(NonZeroU16::new(1444).unwrap())
}

fn primes_func() -> crate::entities::func::Func {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    match module.scope().get("primes") {
        Some(Value::Func(func)) => func.clone(),
        other => panic!("math.primes callable: {other:?}"),
    }
}

fn call(args: Args) -> SourceResult<Value> {
    primes_func().native_fn_addr().expect("math.primes native")(
        &mut EvalContext::new(),
        &args,
        &NullWorld::default(),
        id(),
    )
}

fn content(args: Args) -> Content {
    let Value::Content(content) = call(args).unwrap() else { panic!("primes content") };
    assert!(matches!(content, Content::MathPrimes(_)));
    content
}

fn named(mut args: Args, name: &str, value: Value) -> Args {
    args.named.insert(name.into(), value);
    args
}

fn diagnostic(args: Args) -> String {
    call(args).unwrap_err()[0].message.clone()
}

#[test]
fn p1444_binding_count_field_repr_e_identidade() {
    assert_eq!(primes_func().name(), Some("primes"));
    for count in [0, 1, 2, 3, 4, 5, 8] {
        let got = content(Args::positional(vec![Value::Int(count)]));
        assert_eq!(repr_content(&got), format!("primes(count: {count})"));
        assert_eq!(got.get_field("count"), Some(Value::Int(count)));
        assert_eq!(got.elem_name(), "primes");
    }
    assert_eq!(
        content(Args::positional(vec![Value::Int(3)])),
        content(Args::positional(vec![Value::Int(3)])),
    );
    assert_ne!(
        content(Args::positional(vec![Value::Int(3)])),
        content(Args::positional(vec![Value::Int(4)])),
    );
}

#[test]
fn p1444_assinatura_falha_fechado() {
    assert_eq!(diagnostic(Args::positional(vec![])), "missing argument: count");
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(-1)])),
        "number must be at least zero"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Float(1.0)])),
        "expected integer, found float"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Str("3".into())])),
        "expected integer, found string"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![]), "count", Value::Int(3))),
        "the argument `count` is positional"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![Value::Int(3)]), "foo", Value::Int(1))),
        "unexpected argument: foo"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(3), Value::Int(4)])),
        "unexpected argument"
    );
}

#[test]
fn p1444_carrier_walker_observation_classe_e_receita() {
    let elem = MathPrimesElem { count: 5 };
    assert_eq!(elem.plain_text(), "′′′′′");
    assert_eq!(MathPrimesElem { count: 0 }.plain_text(), "");
    let mapped = elem.map_content(&mut |_| Ok(Some(Content::text("changed")))).unwrap();
    let Content::MathPrimes(mapped) = mapped else { panic!("MathPrimes") };
    assert_eq!(mapped.count, 5);
    assert!(p1444_math_primes_observation_differs(&elem, &MathPrimesElem { count: 4 }));
    let content = Content::MathPrimes(std::sync::Arc::new(elem));
    assert_eq!(p1444_boundary_classes(&content), (MathClass::Normal, MathClass::Normal));
    assert!(!is_locatable(&content));

    for (count, glyph) in [(1, '′'), (2, '″'), (3, '‴'), (4, '⁗')] {
        let recipe = p1444_primes_recipe(count);
        assert_eq!(recipe.glyphs, vec![glyph]);
        assert_eq!(recipe.half_width_extent, 2);
        assert_eq!(recipe.half_width_offsets, vec![0]);
    }
    let zero = p1444_primes_recipe(0);
    assert!(zero.glyphs.is_empty());
    assert_eq!(zero.half_width_extent, 1);
    assert!(zero.half_width_offsets.is_empty());
    let five = p1444_primes_recipe(5);
    assert_eq!(five.glyphs, vec!['′'; 5]);
    assert_eq!(five.half_width_extent, 6);
    assert_eq!(five.half_width_offsets, vec![0, 1, 2, 3, 4]);
}
