//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/compiler/stdlib/primitives-constructors.md
//! @prompt-hash 5b6064d7
//! @layer L1
//! @updated 2026-08-23
//!
//! Hub estático dos construtores primitivos atomizados no Passo 1140.1-A.

mod decimal;
mod duration;
mod version;

pub use decimal::native_decimal;
pub use duration::native_duration;
pub use version::native_version;

#[cfg(test)]
mod test_support {
    use crate::compiler::eval::EvalContext;
    use crate::contracts::world::World;
    use crate::entities::args::Args;
    use crate::entities::file_id::FileId;
    use crate::entities::font_book::FontBook;
    use crate::entities::source::Source;
    use crate::entities::value::Value;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::collections::HashMap;
    use std::num::NonZeroU16;
    use std::sync::Arc;

    pub(super) fn p(items: Vec<Value>) -> Args {
        Args::positional(items)
    }

    pub(super) fn pn(items: Vec<Value>, name: &str, val: Value) -> Args {
        let mut a = Args::positional(items);
        a.named.insert(name.into(), val);
        a
    }

    pub(super) fn ctx() -> EvalContext {
        EvalContext::new()
    }

    pub(super) fn test_file_id() -> FileId {
        FileId::from_raw(NonZeroU16::new(1).unwrap())
    }

    #[derive(Default)]
    pub(super) struct NullWorld {
        library: Library,
        book: FontBook,
        files: HashMap<String, Arc<Vec<u8>>>,
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
        fn today(
            &self,
            _: Option<crate::entities::duration::Duration>,
        ) -> Option<Datetime> {
            None
        }
        fn read_bytes(
            &self,
            _current_file: FileId,
            path: &str,
        ) -> Result<Arc<Vec<u8>>, String> {
            self.files
                .get(path)
                .cloned()
                .ok_or_else(|| format!("ficheiro não encontrado: {}", path))
        }
    }

    pub(super) fn null_world() -> NullWorld {
        NullWorld::default()
    }
}
