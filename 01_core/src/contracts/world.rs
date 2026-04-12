//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/contracts/world.md
//! @prompt-hash 55b3d0df
//! @layer L1
//! @updated 2026-03-22

use crate::entities::file_id::FileId;
use crate::entities::source::Source;
use crate::entities::font_book::FontBook;
use crate::entities::world_types::{
    Bytes, Datetime, FileResult, Font, Library,
};

/// Contrato entre o compilador Typst e o ambiente de execução.
///
/// Sem `comemo` — testável com qualquer implementação simples.
/// A separação `World` / `TrackedWorld` segue o padrão B3 (ADR-0005):
/// o contrato puro declara *o quê*; `TrackedWorld` adiciona *como*
/// o pipeline incremental rastreia as dependências.
pub trait World: Send + Sync {
    /// A biblioteca de funções e valores padrão do Typst.
    fn library(&self) -> &Library;

    /// O catálogo de fontes disponíveis.
    fn book(&self) -> &FontBook;

    /// O ficheiro principal a compilar.
    fn main(&self) -> FileId;

    /// Obter o source de um ficheiro pelo seu id.
    fn source(&self, id: FileId) -> FileResult<Source>;

    /// Obter o conteúdo binário de um ficheiro pelo seu id.
    fn file(&self, id: FileId) -> FileResult<Bytes>;

    /// Obter uma fonte pelo índice no `FontBook`.
    fn font(&self, index: usize) -> Option<Font>;

    /// A data actual com offset em horas UTC (None se indisponível).
    ///
    /// Usa `i64` em vez de `Duration` — o tipo `Duration` do Typst
    /// não existe em L1 neste passo.
    fn today(&self, offset: Option<i64>) -> Option<Datetime>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::file_id::FileId;
    use crate::entities::source::Source;
    use crate::entities::font_book::FontBook;
    use crate::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use std::num::NonZeroU16;

    struct MockWorld {
        library: Library,
        book:    FontBook,
        main_id: FileId,
    }

    impl World for MockWorld {
        fn library(&self) -> &Library  { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId    { self.main_id }
        fn source(&self, _: FileId) -> FileResult<Source> { Err(FileError::NotFound) }
        fn file(&self, _: FileId)   -> FileResult<Bytes>  { Err(FileError::NotFound) }
        fn font(&self, _: usize)    -> Option<Font>       { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn mock() -> MockWorld {
        MockWorld {
            library: Library::new(),
            book:    FontBook::new(),
            main_id: FileId::from_raw(NonZeroU16::new(1).unwrap()),
        }
    }

    #[test]
    fn world_main_returns_correct_id() {
        let w = mock();
        let expected = FileId::from_raw(NonZeroU16::new(1).unwrap());
        assert_eq!(World::main(&w), expected);
    }


    #[test]
    fn world_source_not_found() {
        let w = mock();
        let id = FileId::from_raw(NonZeroU16::new(2).unwrap());
        assert!(matches!(World::source(&w, id), Err(FileError::NotFound)));
    }

    #[test]
    fn world_file_not_found() {
        let w = mock();
        let id = FileId::from_raw(NonZeroU16::new(2).unwrap());
        assert!(matches!(World::file(&w, id), Err(FileError::NotFound)));
    }

    #[test]
    fn world_font_none() {
        assert!(World::font(&mock(), 0).is_none());
    }

    #[test]
    fn world_today_none() {
        let w = mock();
        assert!(World::today(&w, None).is_none());
        assert!(World::today(&w, Some(2)).is_none());
    }

    #[test]
    fn world_pure_no_comemo_import_needed() {
        // World pura compila sem importar comemo — testabilidade confirmada.
        // Chamadas explícitas via World trait para evitar ambiguidade com TrackedWorld.
        // Contrato correcto — teste adicionado para prevenir regressão.
        let w = mock();
        let _ = World::library(&w);
        let _ = World::book(&w);
    }
}
