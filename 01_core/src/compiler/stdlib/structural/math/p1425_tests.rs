//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/p1425-tests.md
//! @prompt-hash f6e7db64
//! @layer L1
#![cfg(test)]
//!
//! Testes A/B independentes do contrato `math.limits` P1425.

use super::*;
use crate::contracts::world::World;
use crate::entities::elements::Element;
use crate::entities::font_book::FontBook;
use crate::entities::func::Func;
use crate::entities::source::Source;
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
        .unwrap_or_else(|| panic!("math.{name} must be a native callable"));
    call(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
}

fn body() -> Value {
    Value::Content(Content::text("x"))
}

fn diagnostic(args: Args) -> String {
    let errors = call_math("limits", args).unwrap_err();
    assert_eq!(errors.len(), 1, "math.limits must emit one primary diagnostic");
    errors.into_iter().next().unwrap().message
}

fn limits(
    args: Args,
) -> std::sync::Arc<crate::entities::elements::math_limits_override::MathLimitsOverrideElem>
{
    let Value::Content(Content::MathLimitsOverride(elem)) =
        call_math("limits", args).unwrap()
    else {
        panic!("math.limits must produce MathLimitsOverride")
    };
    elem
}

#[test]
fn p1425_binding_e_tres_estados_de_presenca() {
    let func = math_func("limits");
    assert_eq!(func.name(), Some("limits"));

    let omitted = limits(Args::positional(vec![body()]));
    assert!(omitted.limits);
    assert!(omitted.inline);
    assert!(!omitted.inline_explicit);

    let mut explicit_true = Args::positional(vec![body()]);
    explicit_true.named.insert("inline".into(), Value::Bool(true));
    let explicit_true = limits(explicit_true);
    assert!(explicit_true.limits);
    assert!(explicit_true.inline);
    assert!(explicit_true.inline_explicit);

    let mut explicit_false = Args::positional(vec![body()]);
    explicit_false.named.insert("inline".into(), Value::Bool(false));
    let explicit_false = limits(explicit_false);
    assert!(explicit_false.limits);
    assert!(!explicit_false.inline);
    assert!(explicit_false.inline_explicit);
}

#[test]
fn p1425_repr_distingue_omissao_de_default_explicito() {
    use crate::compiler::eval::repr::repr_content;

    let omitted = limits(Args::positional(vec![body()]));
    let mut true_args = Args::positional(vec![body()]);
    true_args.named.insert("inline".into(), Value::Bool(true));
    let explicit_true = limits(true_args);
    let mut false_args = Args::positional(vec![body()]);
    false_args.named.insert("inline".into(), Value::Bool(false));
    let explicit_false = limits(false_args);

    assert_eq!(repr_content(&Content::MathLimitsOverride(omitted)), "limits(body: [x])");
    assert_eq!(
        repr_content(&Content::MathLimitsOverride(explicit_true)),
        "limits(body: [x], inline: true)"
    );
    assert_eq!(
        repr_content(&Content::MathLimitsOverride(explicit_false)),
        "limits(body: [x], inline: false)"
    );
}

#[test]
fn p1425_assinatura_publica_rejeita_cinco_classes_invalidas() {
    assert_eq!(diagnostic(Args::positional(vec![])), "missing argument: body");

    let mut wrong_type = Args::positional(vec![body()]);
    wrong_type.named.insert("inline".into(), Value::Int(1));
    assert_eq!(diagnostic(wrong_type), "expected boolean, found integer");

    let mut unknown = Args::positional(vec![body()]);
    unknown.named.insert("nope".into(), Value::Bool(true));
    assert_eq!(diagnostic(unknown), "unexpected argument: nope");

    assert_eq!(diagnostic(Args::positional(vec![body(), body()])), "unexpected argument");

    let mut named_body = Args::positional(vec![]);
    named_body.named.insert("body".into(), body());
    assert_eq!(diagnostic(named_body), "the argument `body` is positional");
}

#[test]
fn p1425_walkers_e_igualdade_preservam_presenca() {
    let omitted = Content::math_limits_override(Content::text("x"), true, true, false);
    let explicit = Content::math_limits_override(Content::text("x"), true, true, true);
    assert_ne!(omitted, explicit, "explicit default must not equal omission");

    let Content::MathLimitsOverride(elem) = explicit else { unreachable!() };
    assert_eq!(elem.plain_text(), "x");
    assert!(!elem.is_empty());

    let mut transform = |content: &Content| -> SourceResult<Option<Content>> {
        Ok(matches!(content, Content::Text(text) if text.as_str() == "x")
            .then(|| Content::text("z")))
    };
    let Content::MathLimitsOverride(mapped) = elem.map_content(&mut transform).unwrap()
    else {
        panic!("map_content changed variant")
    };
    assert_eq!(mapped.plain_text(), "z");
    assert!(mapped.limits && mapped.inline && mapped.inline_explicit);

    let mut uppercase = |text: &str| text.to_uppercase();
    let Content::MathLimitsOverride(terminal) = elem.map_text(&mut uppercase) else {
        panic!("map_text changed variant")
    };
    assert_eq!(terminal.plain_text(), "x");
    assert!(terminal.limits && terminal.inline && terminal.inline_explicit);

    let empty = Content::math_limits_override(Content::Empty, true, true, false);
    assert!(empty.is_empty());
}

#[test]
fn p1425_scripts_permanece_controle_sem_presenca_inline() {
    use crate::compiler::eval::repr::repr_content;

    let Value::Content(Content::MathLimitsOverride(elem)) =
        call_math("scripts", Args::positional(vec![body()])).unwrap()
    else {
        panic!("math.scripts must produce MathLimitsOverride")
    };
    assert!(!elem.limits);
    assert!(elem.inline);
    assert!(!elem.inline_explicit);
    assert_eq!(repr_content(&Content::MathLimitsOverride(elem)), "scripts(body: [x])");
}
