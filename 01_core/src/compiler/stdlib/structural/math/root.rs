//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/core/compiler/stdlib/structural/math/root.md
//! @prompt-hash be706aee
//! @layer L1
//! @updated 2026-09-30
//!
//! Cast e construção semântica de `math.sqrt` e `math.root`.

use crate::compiler::eval::EvalContext;
use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::file_id::FileId;
use crate::entities::source_result::{SourceDiagnostic, SourceResult};
use crate::entities::value::Value;

/// Construtor puro compartilhado pela função pública e pelo modo math.
pub(crate) fn sqrt_content(radicand: Content) -> Content {
    Content::math_root(None, radicand, false)
}

/// `math.sqrt(radicand)` — raiz quadrada estrutural.
pub fn native_math_sqrt(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    crate::compiler::stdlib::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [radicand] => Ok(Value::Content(sqrt_content(
            super::casts::math_lr_content_value(radicand, args.span)?,
        ))),
        _ => Err(vec![SourceDiagnostic::error(
            args.span,
            "unexpected argument".to_string(),
        )]),
    }
}

/// `math.root(index, radicand)` — raiz estrutural com índice causalmente presente.
pub fn native_math_root(
    _ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    if args.named.contains_key("radicand") {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            "the argument `radicand` is positional".to_string(),
        )]);
    }
    if let Some((name, _)) = args.named.iter().next() {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            format!("unexpected argument: {name}"),
        )]);
    }

    let [index, radicand] = args.items.as_slice() else {
        return Err(vec![SourceDiagnostic::error(
            args.span,
            if args.items.len() < 2 {
                "missing argument: radicand".to_string()
            } else {
                "unexpected argument".to_string()
            },
        )]);
    };

    let index = match index {
        Value::None => None,
        other => Some(super::casts::math_lr_content_value(other, args.span)?),
    };
    let radicand = super::casts::math_lr_content_value(radicand, args.span)?;

    Ok(Value::Content(Content::math_root(index, radicand, true)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
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
            FileId::from_raw(NonZeroU16::new(1).unwrap())
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
    }

    #[test]
    fn square_and_qualified_none_keep_distinct_index_presence() {
        let Content::MathRoot(square) = sqrt_content(Content::text("x")) else {
            panic!("sqrt must construct MathRoot")
        };
        assert!(square.index.is_none());
        assert!(!square.index_explicit);

        let args =
            Args::positional(vec![Value::None, Value::Content(Content::text("x"))]);
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let Value::Content(Content::MathRoot(root)) =
            native_math_root(&mut EvalContext::new(), &args, &NullWorld::default(), file)
                .unwrap()
        else {
            panic!("root must construct MathRoot")
        };
        assert!(root.index.is_none());
        assert!(root.index_explicit);
    }

    fn r4_call(name: &str, value: Value) -> SourceResult<Value> {
        let Value::Module(module) = super::super::make_math_module() else {
            panic!("math module")
        };
        let Some(Value::Func(func)) = module.scope().get(name) else {
            panic!("math.{name} binding")
        };
        let items = match name {
            "frac" => vec![value.clone(), value],
            "root" => vec![Value::Content(Content::text("3")), value],
            "class" => vec![Value::Str("relation".into()), value],
            "accent" => vec![value, Value::Str("^".into())],
            _ => vec![value],
        };
        func.native_fn_addr().expect("native constructor")(
            &mut EvalContext::new(),
            &Args::positional(items),
            &NullWorld::default(),
            FileId::from_raw(NonZeroU16::new(1).unwrap()),
        )
    }

    #[test]
    fn r4_qualified_symbol_conversion_preserves_native_content() {
        let mut failures = Vec::new();
        for name in [
            "frac",
            "sqrt",
            "root",
            "cancel",
            "underline",
            "accent",
            "limits",
            "scripts",
            "class",
            "vec",
            "attach",
            "equation",
        ] {
            let actual = r4_call(
                name,
                Value::Symbol(crate::entities::symbol::Symbol::new("α", "alpha")),
            );
            let expected = r4_call(name, Value::Content(Content::Symbol("α".into())));
            if actual != expected || actual.is_err() {
                failures.push((name, actual, expected));
            }
        }
        assert!(failures.is_empty(), "{failures:?}");
    }

    #[test]
    fn r4_qualified_string_none_and_rejected_types() {
        let mut failures = Vec::new();
        for name in [
            "frac",
            "sqrt",
            "root",
            "cancel",
            "underline",
            "accent",
            "limits",
            "scripts",
            "class",
            "vec",
            "attach",
            "equation",
        ] {
            for (value, content, attach_content) in [
                (
                    Value::Str("α".into()),
                    Content::text("α"),
                    Content::MathText("α".into()),
                ),
                (Value::None, Content::Empty, Content::Empty),
            ] {
                let content = if name == "attach" { attach_content } else { content };
                let actual = r4_call(name, value);
                let expected = r4_call(name, Value::Content(content));
                if actual != expected || actual.is_err() {
                    failures.push(format!("{name}: {actual:?} != {expected:?}"));
                }
            }
            for (value, kind) in
                [(Value::Int(12), "integer"), (Value::Bool(true), "boolean")]
            {
                let result = r4_call(name, value);
                if !matches!(&result, Err(e) if e[0].message == format!("expected content, found {kind}"))
                {
                    failures.push(format!("{name}: invalid {kind}: {result:?}"));
                }
            }
        }
        assert!(failures.is_empty(), "{failures:?}");
    }

    #[test]
    fn r4_root_index_and_binom_lower_display() {
        let world = NullWorld::default();
        let file = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let symbol = Value::Symbol(crate::entities::symbol::Symbol::new("α", "alpha"));
        let body = Value::Content(Content::text("x"));
        let actual = native_math_root(
            &mut EvalContext::new(),
            &Args::positional(vec![symbol, body.clone()]),
            &world,
            file,
        );
        let expected = native_math_root(
            &mut EvalContext::new(),
            &Args::positional(vec![
                Value::Content(Content::Symbol("α".into())),
                body.clone(),
            ]),
            &world,
            file,
        );
        assert_eq!(actual, expected);
        let Value::Module(module) = super::super::make_math_module() else {
            panic!("math module")
        };
        let Some(Value::Func(func)) = module.scope().get("binom") else {
            panic!("binom binding")
        };
        let native = func.native_fn_addr().unwrap();
        for (lower, display) in [
            (Value::None, Content::Empty),
            (Value::Int(12), Content::text("12")),
            (Value::Bool(true), Content::raw("true", Some("typc".into()), false)),
        ] {
            let actual = native(
                &mut EvalContext::new(),
                &Args::positional(vec![body.clone(), lower]),
                &world,
                file,
            )
            .unwrap();
            let expected = native(
                &mut EvalContext::new(),
                &Args::positional(vec![body.clone(), Value::Content(display)]),
                &world,
                file,
            )
            .unwrap();
            assert_eq!(actual, expected);
        }
    }
}
