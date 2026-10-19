//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/math_frac_tests.md
//! @prompt-hash 222394aa
//! @layer L1
#![cfg(test)]
//!
//! Testes independentes de binding, assinatura e carrier `math.frac` P1427.

use super::*;
use crate::contracts::world::World;
use crate::entities::elements::math_frac::{MathFracElem, MathFracStyle};
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

fn call(args: Args) -> SourceResult<Value> {
    let native = math_func("frac").native_fn_addr().expect("math.frac must be native");
    native(&mut EvalContext::new(), &args, &NullWorld::default(), test_file_id())
}

fn cv(text: &str) -> Value {
    Value::Content(Content::text(text))
}

fn named(mut args: Args, name: &str, value: Value) -> Args {
    args.named.insert(name.into(), value);
    args
}

fn frac(args: Args) -> std::sync::Arc<MathFracElem> {
    let Value::Content(Content::MathFrac(elem)) = call(args).unwrap() else {
        panic!("math.frac must construct MathFrac")
    };
    elem
}

fn diagnostic(args: Args) -> String {
    let errors = call(args).unwrap_err();
    assert_eq!(errors.len(), 1);
    errors[0].message.clone()
}

fn hash_of(value: &MathFracElem) -> u64 {
    let mut state = DefaultHasher::new();
    value.hash(&mut state);
    state.finish()
}

fn complete(
    style: Option<MathFracStyle>,
    nb: bool,
    db: bool,
    line: bool,
) -> MathFracElem {
    MathFracElem {
        num: Content::text("a"),
        den: Content::text("b"),
        style,
        num_deparenthesized: nb,
        denom_deparenthesized: db,
        line,
    }
}

#[test]
fn p1427_binding_assinatura_casts_e_diagnosticos() {
    let _callable = math_func("frac");
    assert_eq!(diagnostic(Args::positional(vec![])), "missing argument: num");
    assert_eq!(diagnostic(Args::positional(vec![cv("a")])), "missing argument: denom");
    assert_eq!(
        diagnostic(Args::positional(vec![cv("a"), cv("b"), cv("c")])),
        "unexpected argument"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("b")]), "num", cv("a"))),
        "the argument `num` is positional"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("a")]), "denom", cv("b"))),
        "the argument `denom` is positional"
    );
    assert_eq!(
        diagnostic(named(Args::positional(vec![cv("a"), cv("b")]), "foo", Value::Int(1))),
        "unexpected argument: foo"
    );
    assert!(diagnostic(named(
        Args::positional(vec![cv("a"), cv("b")]),
        "style",
        Value::Str("diagonal".into())
    ))
    .contains("vertical"));
    assert!(diagnostic(named(
        Args::positional(vec![cv("a"), cv("b")]),
        "style",
        Value::Int(1)
    ))
    .contains("integer"));
    assert_eq!(
        diagnostic(Args::positional(vec![Value::Int(1), cv("b")])),
        "expected content, found integer"
    );
    assert_eq!(
        diagnostic(Args::positional(vec![cv("a"), Value::Int(1)])),
        "expected content, found integer"
    );

    let none = frac(Args::positional(vec![Value::None, Value::None]));
    assert!(none.num.is_empty() && none.den.is_empty());
    let strings =
        frac(Args::positional(vec![Value::Str("a".into()), Value::Str("b".into())]));
    assert_eq!(
        (strings.num.plain_text(), strings.den.plain_text()),
        ("a".into(), "b".into())
    );

    let occurrences = vec![
        ArgOccurrence {
            name: None,
            value: cv("a"),
            span: Span::detached(),
            value_span: Span::detached(),
        },
        ArgOccurrence {
            name: None,
            value: cv("b"),
            span: Span::detached(),
            value_span: Span::detached(),
        },
        ArgOccurrence {
            name: Some("style".into()),
            value: Value::Str("vertical".into()),
            span: Span::detached(),
            value_span: Span::detached(),
        },
        ArgOccurrence {
            name: Some("style".into()),
            value: Value::Str("skewed".into()),
            span: Span::detached(),
            value_span: Span::detached(),
        },
    ];
    assert!(diagnostic(Args::from_occurrences(Span::detached(), occurrences))
        .contains("duplicate"));

    let style_first = Args::from_occurrences(
        Span::detached(),
        vec![
            ArgOccurrence {
                name: Some("style".into()),
                value: Value::Str("skewed".into()),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: None,
                value: cv("a"),
                span: Span::detached(),
                value_span: Span::detached(),
            },
            ArgOccurrence {
                name: None,
                value: cv("b"),
                span: Span::detached(),
                value_span: Span::detached(),
            },
        ],
    );
    assert_eq!(frac(style_first).style, Some(MathFracStyle::Skewed));
}

#[test]
fn p1427_style_explicito_e_repr_preservam_causalidade() {
    use crate::compiler::eval::repr::repr_content;
    let omitted = frac(Args::positional(vec![cv("a"), cv("b")]));
    assert_eq!(omitted.style, None);
    assert_eq!(repr_content(&Content::MathFrac(omitted)), "frac(num: [a], denom: [b])");

    for (raw, style) in [
        ("vertical", MathFracStyle::Vertical),
        ("skewed", MathFracStyle::Skewed),
        ("horizontal", MathFracStyle::Horizontal),
    ] {
        let elem = frac(named(
            Args::positional(vec![cv("a"), cv("b")]),
            "style",
            Value::Str(raw.into()),
        ));
        assert_eq!(elem.style, Some(style));
        let repr = repr_content(&Content::MathFrac(elem));
        assert_eq!(repr, format!("frac(num: [a], denom: [b], style: \"{raw}\")"));
        assert!(!repr.contains("deparenthesized"));
    }

    let causal = Content::MathFrac(std::sync::Arc::new(complete(
        Some(MathFracStyle::Horizontal),
        true,
        true,
        true,
    )));
    let repr = repr_content(&causal);
    assert_eq!(repr, "frac(num: [a], denom: [b], style: \"horizontal\")");
    assert!(!repr.contains("deparenthesized"));
}

#[test]
fn p1427_carrier_walkers_igualdade_e_hash_preservam_campos() {
    let original = complete(Some(MathFracStyle::Horizontal), true, false, true);
    assert_eq!(original.clone(), original);
    for changed in [
        complete(None, true, false, true),
        complete(Some(MathFracStyle::Horizontal), false, false, true),
        complete(Some(MathFracStyle::Horizontal), true, true, true),
        complete(Some(MathFracStyle::Horizontal), true, false, false),
    ] {
        assert_ne!(original, changed);
        assert_ne!(hash_of(&original), hash_of(&changed));
    }

    let mut transform = |child: &Content| -> SourceResult<Option<Content>> {
        Ok((child.plain_text() == "b").then(|| Content::text("z")))
    };
    let Content::MathFrac(mapped) = original.map_content(&mut transform).unwrap() else {
        panic!()
    };
    assert_eq!(mapped.den.plain_text(), "z");
    assert_eq!(mapped.style, original.style);
    assert_eq!(
        (mapped.num_deparenthesized, mapped.denom_deparenthesized, mapped.line),
        (true, false, true)
    );

    let mut uppercase = |text: &str| text.to_uppercase();
    let Content::MathFrac(terminal) = original.map_text(&mut uppercase) else { panic!() };
    assert_eq!(terminal, std::sync::Arc::new(original));
}
