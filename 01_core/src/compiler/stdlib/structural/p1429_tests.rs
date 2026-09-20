//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/p1429-tests.md
//! @prompt-hash a7780874
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes da fronteira pública `math.accent` (P1429).

use super::*;
use crate::compiler::eval::{repr::repr_content, EvalContext};
use crate::contracts::world::World;
use crate::entities::args::{ArgOccurrence, Args};
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::font_book::FontBook;
use crate::entities::func::Func;
use crate::entities::layout_types::Length;
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
        file_id()
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

fn file_id() -> FileId {
    FileId::from_raw(NonZeroU16::new(1429).unwrap())
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

fn call(args: Args) -> SourceResult<Value> {
    let native = math_func("accent")
        .native_fn_addr()
        .expect("math.accent must be native");
    native(&mut EvalContext::new(), &args, &NullWorld::default(), file_id())
}

fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}

fn named(mut args: Args, name: &str, value: Value) -> Args {
    args.named.insert(name.into(), value);
    args
}

fn repr(args: Args) -> String {
    let Value::Content(content) = call(args).unwrap() else {
        panic!("math.accent must return content")
    };
    assert!(matches!(content, Content::MathAccent(_)), "wrong carrier: {content:?}");
    repr_content(&content)
}

fn diagnostic(args: Args) -> String {
    let errors = call(args).unwrap_err();
    assert_eq!(errors.len(), 1, "one primary diagnostic expected: {errors:?}");
    errors[0].message.clone()
}

fn duplicate(name: &str, first: Value, second: Value) -> Args {
    Args::from_occurrences(
        Span::detached(),
        vec![
            ArgOccurrence {
                name: None,
                value: cv("x"),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: None,
                value: Value::Str("^".into()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: Some(name.into()),
                value: first,
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: Some(name.into()),
                value: second,
                span: Span::detached(),
                value_span: Span::detached(),
            },
        ],
    )
}

#[test]
fn p1429_binding_normalizacao_casts_e_presenca() {
    assert_eq!(math_func("accent").name(), Some("accent"));
    assert_eq!(
        repr(Args::positional(vec![cv("x"), Value::Str("^".into())])),
        "accent(base: [x], accent: \"\\u{302}\")"
    );
    assert_eq!(
        repr(Args::positional(vec![Value::Str("x".into()), Value::Str("~".into())])),
        "accent(base: [x], accent: \"\\u{303}\")"
    );
    assert_eq!(
        repr(Args::positional(vec![Value::None, Value::Str(".".into())])),
        "accent(base: [], accent: \"\\u{307}\")"
    );
    assert_eq!(
        repr(Args::positional(vec![cv("x"), Value::Str("→".into())])),
        "accent(base: [x], accent: \"\\u{20d7}\")"
    );
    assert_eq!(
        repr(Args::positional(vec![
            cv("x"),
            Value::Content(Content::MathText("^".into())),
        ])),
        "accent(base: [x], accent: \"\\u{302}\")"
    );

    let explicit_size = named(
        Args::positional(vec![cv("x"), Value::Str("^".into())]),
        "size",
        Value::Relative(Rel::<Length>::from_percent(150.0)),
    );
    assert_eq!(
        repr(explicit_size),
        "accent(base: [x], accent: \"\\u{302}\", size: 150% + 0pt)"
    );
    let explicit_length = named(
        Args::positional(vec![cv("x"), Value::Str("^".into())]),
        "size",
        Value::Length(Length::em(2.0)),
    );
    assert_eq!(
        repr(explicit_length),
        "accent(base: [x], accent: \"\\u{302}\", size: 0% + 2em)"
    );
    let explicit_dotless = named(
        Args::positional(vec![cv("i"), Value::Str("^".into())]),
        "dotless",
        Value::Bool(false),
    );
    assert_eq!(
        repr(explicit_dotless),
        "accent(base: [i], accent: \"\\u{302}\", dotless: false)"
    );

    let both = Args::from_occurrences(
        Span::detached(),
        vec![
            ArgOccurrence {
                name: Some("size".into()),
                value: Value::Relative(Rel::<Length>::from_percent(100.0)),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: Some("dotless".into()),
                value: Value::Bool(true),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: None,
                value: cv("x"),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: None,
                value: Value::Str("^".into()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        ],
    );
    assert_eq!(
        repr(both),
        "accent(base: [x], accent: \"\\u{302}\", size: 100% + 0pt, dotless: true)"
    );
}

#[test]
fn p1429_assinatura_fechada_rejeita_casos_atribuiveis() {
    assert_eq!(diagnostic(Args::positional(vec![])), "missing argument: base");
    assert_eq!(diagnostic(Args::positional(vec![cv("x")])), "missing argument: accent");
    assert_eq!(
        diagnostic(Args::positional(vec![cv("x"), Value::Str("^".into()), cv("extra")])),
        "unexpected argument"
    );
    assert_eq!(
        diagnostic(named(
            Args::positional(vec![Value::Str("^".into())]),
            "base",
            cv("x")
        )),
        "the argument `base` is positional"
    );
    assert_eq!(
        diagnostic(named(
            Args::positional(vec![cv("x")]),
            "accent",
            Value::Str("^".into())
        )),
        "the argument `accent` is positional"
    );
    assert_eq!(
        diagnostic(named(
            Args::positional(vec![cv("x"), Value::Str("^".into())]),
            "foo",
            Value::Int(1)
        )),
        "unexpected argument: foo"
    );
    assert_eq!(
        diagnostic(duplicate(
            "size",
            Value::Relative(Rel::<Length>::from_percent(100.0)),
            Value::Relative(Rel::<Length>::from_percent(150.0)),
        )),
        "duplicate argument: size"
    );
    assert_eq!(
        diagnostic(duplicate("dotless", Value::Bool(true), Value::Bool(false))),
        "duplicate argument: dotless"
    );
    assert!(diagnostic(Args::positional(vec![cv("x"), Value::Str("ab".into())]))
        .contains("expected exactly one character"));
    assert!(diagnostic(Args::positional(vec![cv("x"), cv("ab")]))
        .contains("expected a single-codepoint symbol"));
    assert!(diagnostic(Args::positional(vec![cv("x"), Value::None]))
        .contains("expected a single-codepoint symbol"));
    assert!(diagnostic(named(
        Args::positional(vec![cv("x"), Value::Str("^".into())]),
        "size",
        Value::Int(1),
    ))
    .contains("expected relative length, found integer"));
    assert!(diagnostic(named(
        Args::positional(vec![cv("x"), Value::Str("^".into())]),
        "dotless",
        Value::Int(1),
    ))
    .contains("expected boolean, found integer"));
}

struct EvalWorld {
    library: Library,
    book: FontBook,
    source: Source,
}

impl EvalWorld {
    fn new(text: &str) -> Self {
        Self {
            library: Library::new(),
            book: FontBook::new(),
            source: Source::new(file_id(), text.into()),
        }
    }
}

impl World for EvalWorld {
    fn library(&self) -> &Library {
        &self.library
    }
    fn book(&self) -> &FontBook {
        &self.book
    }
    fn main(&self) -> FileId {
        self.source.id()
    }
    fn source(&self, _: FileId) -> FileResult<Source> {
        Ok(self.source.clone())
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

fn eval(text: &str) -> SourceResult<crate::entities::module::Module> {
    let world = EvalWorld::new(text);
    crate::compiler::eval::eval_for_test(&world, &world.source)
}

fn find_accent(content: &Content) -> Option<&Content> {
    match content {
        Content::MathAccent(_) => Some(content),
        Content::Equation(e) => find_accent(&e.body),
        Content::Styled(inner, _) => find_accent(inner),
        Content::MathDelimited(elem) => find_accent(&elem.body),
        Content::Sequence(items) | Content::MathSequence(items) => {
            items.iter().find_map(find_accent)
        }
        _ => None,
    }
}

fn eval_repr(text: &str) -> String {
    let module = eval(text).unwrap_or_else(|e| panic!("fixture {text:?}: {e:?}"));
    let content = module.content().expect("module content");
    repr_content(
        find_accent(content)
            .unwrap_or_else(|| panic!("missing MathAccent in {text:?}: {content:?}")),
    )
}

#[test]
fn p1429_rotas_bare_qualificada_e_wrappers_convergem() {
    let expected =
        "accent(base: x, accent: \"\\u{302}\", size: 150% + 0pt, dotless: false)";
    assert_eq!(eval_repr("$accent(x, \"^\", size: #150%, dotless: #false)$"), expected);
    assert_eq!(eval_repr("$hat(size: #150%, dotless: #false, x)$"), expected);

    for (source, accent) in [
        ("$tilde(x)$", "\\u{303}"),
        ("$dot(x)$", "\\u{307}"),
        ("$dot.double(x)$", "\\u{308}"),
    ] {
        let actual = eval_repr(source);
        assert!(actual.contains(accent), "{source}: {actual}");
        assert!(!actual.contains("size:"), "default size leaked: {actual}");
        assert!(!actual.contains("dotless:"), "default dotless leaked: {actual}");
    }

    for (source, fragment) in [
        ("$hat(x, y)$", "unexpected argument"),
        ("$hat(foo: #1, x)$", "unexpected argument: foo"),
        ("#set math.accent(foo: 1); $hat(x)$", "unknown property"),
        ("#set math.accent(size: 1); $hat(x)$", "relative length"),
        ("#set math.accent(dotless: 1); $hat(x)$", "boolean"),
    ] {
        let errors = eval(source).unwrap_err();
        assert!(
            errors.iter().any(|e| e.message.contains(fragment)),
            "{source}: {errors:?}"
        );
    }
}
