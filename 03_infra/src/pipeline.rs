//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/pipeline.md
//! @prompt-hash 169fbacd
//! @layer L3
//! @updated 2026-04-23
//!
//! Pipeline de compilação — orquestra eval → introspect → layout
//! → export_pdf, gere o boilerplate `comemo` (Route, Sink, Traced,
//! Routines) e drena warnings.
//!
//! Materializado no Passo 113 (ADR-0046) a partir de helpers
//! test-only em `integration_tests.rs`. API pública consumível
//! pelo 04_wiring (CLI) e por testes.

use comemo::Track;

use typst_core::contracts::world::World;
use typst_core::entities::module::Module;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::{SourceDiagnostic, SourceResult};
use typst_core::entities::world_types::{Route, Routines, Sink, Traced};
use typst_core::rules::eval::eval;
use typst_core::rules::introspect::introspect;
use typst_core::rules::layout::layout;

use crate::export::export_pdf;

/// Avalia `source` contra `world` e devolve `(Module, warnings)`.
///
/// Boilerplate `comemo` (Routines, Traced, Sink, Route) gerido
/// internamente. Warnings drenados do `Sink` após retorno.
///
/// Para compilar directamente a PDF, usar `compile_to_pdf_bytes`.
pub fn eval_to_module_with_sink(
    world: &dyn World,
    source: &Source,
) -> (SourceResult<Module>, Vec<SourceDiagnostic>) {
    let routines = Routines::new();
    let traced   = Traced::default();
    let mut sink = Sink::new();
    let route    = Route::root();
    let result = eval(
        &routines,
        world,
        traced.track(),
        sink.track_mut(),
        route.track(),
        source,
    );
    let warnings = sink.into_diagnostics();
    (result, warnings)
}

/// Pipeline completo `Source` → bytes PDF.
///
/// Retorna `(Ok(pdf_bytes), warnings)` em sucesso ou
/// `(Err(errors), warnings)` em falha. Warnings são sempre
/// devolvidos, mesmo em erro — caller decide se os imprime.
///
/// Uses `export_pdf` (Helvetica Type1, sem fonte custom). Para
/// output com fonte real, usar `eval_to_module_with_sink` +
/// `export_pdf_with_font` manualmente.
pub fn compile_to_pdf_bytes(
    world: &dyn World,
    source: &Source,
) -> (Result<Vec<u8>, Vec<SourceDiagnostic>>, Vec<SourceDiagnostic>) {
    let (eval_result, warnings) = eval_to_module_with_sink(world, source);
    let module = match eval_result {
        Ok(m) => m,
        Err(errors) => return (Err(errors), warnings),
    };
    let content = match module.content() {
        Some(c) => c,
        None => return (Ok(Vec::new()), warnings),
    };
    let state = introspect(content);
    let doc   = layout(content, state);
    let pdf   = export_pdf(&doc);
    (Ok(pdf), warnings)
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::world_types::{
        Bytes, Datetime, FileError, FileResult, Font, Library,
    };
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::font_book::FontBook;
    use std::num::NonZeroU16;

    // MockWorld mínimo para smoke test — source única.
    struct MockWorld {
        library: Library,
        book:    FontBook,
        source:  Source,
    }
    impl World for MockWorld {
        fn library(&self) -> &Library { &self.library }
        fn book(&self)    -> &FontBook { &self.book }
        fn main(&self)    -> FileId { self.source.id() }
        fn source(&self, id: FileId) -> FileResult<Source> {
            if id == self.source.id() {
                Ok(self.source.clone())
            } else {
                Err(FileError::NotFound)
            }
        }
        fn file(&self, _: FileId) -> FileResult<Bytes> { Err(FileError::NotFound) }
        fn font(&self, _: usize) -> Option<Font> { None }
        fn today(&self, _: Option<i64>) -> Option<Datetime> { None }
    }

    fn mock_world(src: &str) -> MockWorld {
        let id = FileId::from_raw(NonZeroU16::new(1).unwrap());
        let source = Source::new(id, src.to_string());
        MockWorld {
            library: Library::new(),
            book:    FontBook::new(),
            source,
        }
    }

    #[test]
    fn eval_to_module_retorna_modulo_e_sem_warnings() {
        let w = mock_world("Olá");
        let source = w.source.clone();
        let (result, warnings) = eval_to_module_with_sink(&w, &source);
        assert!(result.is_ok());
        assert!(warnings.is_empty());
    }

    #[test]
    fn eval_to_module_ficheiro_vazio_emite_warning() {
        let w = mock_world("");
        let source = w.source.clone();
        let (result, warnings) = eval_to_module_with_sink(&w, &source);
        assert!(result.is_ok());
        assert_eq!(warnings.len(), 1,
            "pilot do Passo 106 emite warning para ficheiro vazio");
    }

    #[test]
    fn compile_to_pdf_bytes_produz_pdf_valido() {
        let w = mock_world("Texto de teste");
        let source = w.source.clone();
        let (result, _warnings) = compile_to_pdf_bytes(&w, &source);
        let pdf = result.expect("compilação deve ter sucesso");
        assert!(!pdf.is_empty(), "bytes PDF devem existir");
        assert_eq!(&pdf[..5], b"%PDF-", "header PDF esperado");
    }
}
