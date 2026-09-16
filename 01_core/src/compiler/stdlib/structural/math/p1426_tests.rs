//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/p1426-tests.md
//! @prompt-hash 3f2491ac
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes do binding, carrier e morfologia `math.root` P1426.

use super::*;
use crate::contracts::world::World;
use crate::entities::elements::Element;
use crate::entities::font_book::FontBook;
use crate::entities::func::Func;
use crate::entities::source::Source;
use crate::entities::world_types::{
    Bytes, Datetime, FileError, FileResult, Font, Library,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
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
        test_file_id()
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

fn test_file_id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1).unwrap())
}

fn math_func(name: &str) -> Func {
    let Value::Module(module) = make_math_module() else {
        panic!("math must be a module")
    };
    match module.scope().get(name) {
        Some(Value::Func(func)) => func.clone(),
        Some(other) => panic!("math.{name} must be callable, got {other:?}"),
        None => panic!("missing callable math.{name}"),
    }
}

fn call_math(name: &str, args: Args) -> SourceResult<Value> {
    let call = math_func(name)
        .native_fn_addr()
        .unwrap_or_else(|| panic!("math.{name} must be native"));
    call(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
}

fn content(text: &str) -> Value {
    Value::Content(Content::text(text))
}

fn root(
    args: Args,
) -> std::sync::Arc<crate::entities::elements::math_root::MathRootElem> {
    let Value::Content(Content::MathRoot(elem)) = call_math("root", args).unwrap() else {
        panic!("math.root must produce MathRoot")
    };
    elem
}

fn diagnostic(args: Args) -> String {
    let errors = call_math("root", args).unwrap_err();
    assert_eq!(errors.len(), 1, "math.root must emit one primary diagnostic");
    errors.into_iter().next().unwrap().message
}

fn hash_of(value: &Content) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
}

#[test]
fn p1426_binding_e_assinatura_posicional() {
    assert_eq!(math_func("root").name(), Some("root"));

    assert_eq!(diagnostic(Args::positional(vec![])), "missing argument: radicand");
    assert_eq!(
        diagnostic(Args::positional(vec![content("x")])),
        "missing argument: radicand"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![content("3"), content("x"), content("y")])),
        "unexpected argument"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(3), content("x")])),
        "expected content, found integer"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![content("3"), Value::Int(1)])),
        "expected content, found integer"
    );

    let mut named_radicand = Args::positional(vec![content("3")]);
    named_radicand.named.insert("radicand".into(), content("x"));
    assert_eq!(diagnostic(named_radicand), "the argument `radicand` is positional");

    let mut unknown = Args::positional(vec![content("3"), content("x")]);
    unknown.named.insert("foo".into(), Value::Int(1));
    assert_eq!(diagnostic(unknown), "unexpected argument: foo");
}

#[test]
fn p1426_carrier_preserva_tres_estados_do_indice() {
    let sqrt = root(Args::positional(vec![Value::None, content("x")]));
    assert!(sqrt.index.is_none());
    assert!(sqrt.index_explicit, "qualified none is explicit");

    let indexed = root(Args::positional(vec![content("3"), content("x")]));
    assert_eq!(indexed.index.as_ref().unwrap().plain_text(), "3");
    assert!(indexed.index_explicit);

    let empty_radicand = root(Args::positional(vec![content("3"), Value::None]));
    assert!(empty_radicand.radicand.is_empty());

    let Value::Content(Content::MathRoot(square)) =
        call_math("sqrt", Args::positional(vec![content("x")])).unwrap()
    else {
        panic!("math.sqrt must produce MathRoot")
    };
    assert!(square.index.is_none());
    assert!(!square.index_explicit, "sqrt has absent index");
}

#[test]
fn p1426_repr_distingue_ausencia_none_e_indice() {
    use crate::compiler::eval::repr::repr_content;

    let Value::Content(square) =
        call_math("sqrt", Args::positional(vec![content("x")])).unwrap()
    else {
        unreachable!()
    };
    let explicit_none =
        Content::MathRoot(root(Args::positional(vec![Value::None, content("x")])));
    let indexed =
        Content::MathRoot(root(Args::positional(vec![content("3"), content("x")])));
    let none_radicand =
        Content::MathRoot(root(Args::positional(vec![content("3"), Value::None])));

    assert_eq!(repr_content(&square), "root(radicand: [x])");
    assert_eq!(repr_content(&explicit_none), "root(index: none, radicand: [x])");
    assert_eq!(repr_content(&indexed), "root(index: [3], radicand: [x])");
    assert_eq!(repr_content(&none_radicand), "root(index: [3], radicand: [])");
}

#[test]
fn p1426_walkers_igualdade_e_hash_preservam_presenca() {
    let absent = Content::math_root(None, Content::text("x"), false);
    let explicit_none = Content::math_root(None, Content::text("x"), true);
    assert_ne!(absent, explicit_none);
    assert_ne!(hash_of(&absent), hash_of(&explicit_none));
    assert_eq!(absent.clone(), absent, "clone must preserve the complete carrier");
    assert_eq!(
        absent.plain_text(),
        explicit_none.plain_text(),
        "presence alone is inert for value-based plain_text"
    );
    assert_eq!(
        absent.is_empty(),
        explicit_none.is_empty(),
        "presence alone is inert for emptiness"
    );

    let Content::MathRoot(elem) = explicit_none.clone() else { unreachable!() };
    let mut transform = |child: &Content| -> SourceResult<Option<Content>> {
        Ok(matches!(child, Content::Text(text) if text.as_str() == "x")
            .then(|| Content::text("z")))
    };
    let Content::MathRoot(mapped) = elem.map_content(&mut transform).unwrap() else {
        panic!("map_content changed variant")
    };
    assert!(mapped.index.is_none());
    assert!(mapped.index_explicit);
    assert_eq!(mapped.radicand.plain_text(), "z");

    let mut uppercase = |text: &str| text.to_uppercase();
    let Content::MathRoot(terminal) = elem.map_text(&mut uppercase) else {
        panic!("map_text changed variant")
    };
    assert_eq!(terminal.radicand.plain_text(), "x");
    assert!(terminal.index_explicit);
}

#[test]
fn p1426_math_sqrt_permanece_sem_indice_explicito() {
    use crate::compiler::eval::repr::repr_content;

    let Value::Content(content) =
        call_math("sqrt", Args::positional(vec![content("x")])).unwrap()
    else {
        unreachable!()
    };
    let Content::MathRoot(elem) = &content else { unreachable!() };
    assert!(elem.index.is_none());
    assert!(!elem.index_explicit);
    assert_eq!(repr_content(&content), "root(radicand: [x])");
}
