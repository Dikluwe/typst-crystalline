//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/rules/stdlib/context.md
//! @prompt-hash 5139c5a0
//! @layer L1
//! @updated 2026-06-30
//!
//! `context { expr }` — delayed evaluation block. P506 — runtime state via
//! context.

use std::sync::Arc;

use crate::entities::args::Args;
use crate::entities::content::Content;
use crate::entities::elements::context_block::ContextBlockElem;
use crate::entities::file_id::FileId;
use crate::entities::source_result::SourceResult;
use crate::entities::value::Value;
use crate::rules::eval::EvalContext;

use super::err;

/// `context(body)` → `Content::ContextBlock`.
pub fn native_context(
    ctx: &mut EvalContext,
    args: &Args,
    _world: &dyn crate::contracts::world::World,
    _current_file: FileId,
) -> SourceResult<Value> {
    super::expect_no_named(&args.named)?;
    match args.items.as_slice() {
        [Value::Func(func)] => {
            let id = ctx.next_context_id();
            Ok(Value::Content(Content::ContextBlock(Arc::new(
                ContextBlockElem {
                    id,
                    closure: func.clone(),
                },
            ))))
        }
        [other] => err(format!(
            "context() requer função (corpo em bloco), recebeu {}",
            other.type_name()
        )),
        _ => err(format!(
            "context() requer 1 argumento, recebeu {}",
            args.items.len()
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::num::NonZeroU16;
    use crate::contracts::world::World;
    use crate::entities::font_book::FontBook;
    use crate::entities::func::Func;
    use crate::entities::source::Source;
    use crate::entities::world_types::{Bytes, Datetime, FileError, FileResult, Font, Library};

    struct NullWorld { library: Library, book: FontBook }
    impl World for NullWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self) -> &FontBook { &self.book }
        fn main(&self) -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }
        fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }
    fn null_world() -> NullWorld { NullWorld { library: Library::new(), book: FontBook::new() } }
    fn test_file_id() -> FileId { FileId::from_raw(NonZeroU16::new(1).unwrap()) }

    #[test]
    fn native_context_cria_context_block() {
        let closure = Value::Func(Func::native("x", |_, _, _, _| Ok(Value::None)));
        let v = native_context(
            &mut EvalContext::new(),
            &Args::positional(vec![closure]),
            &null_world(),
            test_file_id(),
        )
        .unwrap();
        assert!(matches!(v, Value::Content(Content::ContextBlock(_))));
    }
}
