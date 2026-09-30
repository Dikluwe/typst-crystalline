//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/structural/math/root.md
//! @prompt-hash dc3452de
//! @layer L1
//! @updated 2026-09-30
//!
//! Cast e construção semântica de `math.sqrt` e `math.root`.

use super::vanilla_type_name_class;
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
        [Value::Content(radicand)] => Ok(Value::Content(sqrt_content(radicand.clone()))),
        [other] => Err(vec![SourceDiagnostic::error(
            args.span,
            format!("expected content, found {}", other.type_name()),
        )]),
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
        Value::Content(index) => Some(index.clone()),
        Value::None => None,
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
    };
    let radicand = match radicand {
        Value::Content(radicand) => radicand.clone(),
        Value::None => Content::Empty,
        other => {
            return Err(vec![SourceDiagnostic::error(
                args.span,
                format!("expected content, found {}", vanilla_type_name_class(other)),
            )])
        }
    };

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
}
