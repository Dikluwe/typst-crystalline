//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/elements/p1430-math-cases-tests.md
//! @prompt-hash f7651486
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
use crate::entities::layout_types::Length;
use crate::entities::rel::Rel;
use crate::entities::source::Source;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
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
    FileId::from_raw(NonZeroU16::new(1430).unwrap())
}

fn call(mut args: Args) -> Content {
    let Value::Module(module) = make_math_module() else { panic!("math module") };
    let Some(Value::Func(function)) = module.scope().get("cases") else {
        panic!("missing callable math.cases")
    };
    let native = function.native_fn_addr().expect("math.cases native");
    let Value::Content(content) =
        native(&mut EvalContext::new(), &args, &NullWorld::default(), id()).unwrap()
    else {
        panic!("math.cases must return content")
    };
    args.items.clear();
    assert!(matches!(content, Content::MathCases(_)));
    content
}

fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}

fn carrier(mask: u8) -> Content {
    let mut args = Args::positional(vec![cv("x")]);
    if mask & 1 != 0 {
        args.named.insert("delim".into(), Value::Str("{".into()));
    }
    if mask & 2 != 0 {
        args.named.insert("reverse".into(), Value::Bool(false));
    }
    if mask & 4 != 0 {
        args.named.insert("gap".into(), Value::Length(Length::em(0.2)));
    }
    call(args)
}

fn hash(content: &Content) -> u64 {
    let mut state = DefaultHasher::new();
    content.hash(&mut state);
    state.finish()
}

#[test]
fn p1430_carrier_discrimina_todos_os_bits_e_valores() {
    let absent = carrier(0);
    assert_eq!(repr_content(&absent), "cases(children: ([x],))");
    for mask in 1..8 {
        let changed = carrier(mask);
        assert_ne!(absent, changed, "presence mask {mask:03b} collapsed");
        assert_ne!(hash(&absent), hash(&changed), "mask {mask:03b} absent from hash");
    }
    assert_eq!(
        repr_content(&carrier(7)),
        "cases(delim: (\"{\", \"}\"), reverse: false, gap: 0% + 0.2em, children: ([x],))"
    );

    let mut asymmetric_args = Args::positional(vec![cv("x")]);
    asymmetric_args
        .named
        .insert("delim".into(), Value::Array(vec![Value::Str("[".into()), Value::None]));
    asymmetric_args.named.insert("reverse".into(), Value::Bool(true));
    asymmetric_args
        .named
        .insert("gap".into(), Value::Relative(Rel::<Length>::from_percent(150.0)));
    let asymmetric = call(asymmetric_args);
    assert_ne!(carrier(7), asymmetric);
    assert_eq!(
        repr_content(&asymmetric),
        "cases(delim: (\"[\", none), reverse: true, gap: 150% + 0pt, children: ([x],))"
    );
}

#[test]
fn p1430_children_ordem_alignpoint_e_walkers() {
    let aligned = Content::MathSequence(Arc::from(
        vec![Content::text("a"), Content::math_align_point(), Content::text("b")]
            .into_boxed_slice(),
    ));
    let mut args = Args::positional(vec![Value::Content(aligned), cv("c")]);
    args.named.insert("delim".into(), Value::None);
    args.named.insert("reverse".into(), Value::Bool(true));
    args.named.insert("gap".into(), Value::Length(Length::em(1.0)));
    let input = call(args);
    assert_eq!(input.clone(), input);
    assert_eq!(
        repr_content(&input),
        "cases(delim: (none, none), reverse: true, gap: 0% + 1em, children: (a & b, [c]))"
    );

    let Content::MathCases(elem) = &input else { unreachable!() };
    assert_eq!(elem.plain_text(), "ab, c");
    let mut transform = |child: &Content| -> SourceResult<Option<Content>> {
        Ok((child.plain_text() == "c").then(|| Content::text("z")))
    };
    let mapped = elem.map_content(&mut transform).unwrap();
    assert_eq!(
        repr_content(&mapped),
        "cases(delim: (none, none), reverse: true, gap: 0% + 1em, children: (a & b, [z]))"
    );
    let mut terminal = |_: &str| "mutated".to_string();
    assert_eq!(elem.map_text(&mut terminal), input);
}

#[test]
fn p1430_child_order_participa_de_igualdade_e_hash() {
    let xy = call(Args::positional(vec![cv("x"), cv("y")]));
    let yx = call(Args::positional(vec![cv("y"), cv("x")]));
    assert_ne!(xy, yx);
    assert_ne!(hash(&xy), hash(&yx));
    assert_eq!(repr_content(&xy), "cases(children: ([x], [y]))");
    assert_eq!(repr_content(&yx), "cases(children: ([y], [x]))");
}
