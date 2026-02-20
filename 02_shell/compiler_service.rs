// -----------------------------------------------------------------------------
// Tipologia: Service API Extent (L2)
// Módulo: Compiler
// Descrição: Casos de uso de alto nível (Compile, Trace) extraídos do lib.rs.
// -----------------------------------------------------------------------------

use crate::contracts::compiler_io::ICompilerEnv;
use typst_library::diag::{Warned, SourceResult};
use crate::contracts::document_traits::Document;
use ecow::EcoVec;
use typst_library::foundations::{Value, Styles};
use typst_syntax::Span;

pub struct CompilerService<'a, E: ICompilerEnv> {
    env: &'a E,
}

impl<'a, E: ICompilerEnv> CompilerService<'a, E> {
    pub fn new(env: &'a E) -> Self {
        Self { env }
    }

    /// Compile sources into a fully layouted document.
    pub fn compile<D: Document>(&self) -> Warned<SourceResult<D>> {
        // Na Tekt, chamamos o orchestrator (que agora seria parte injetada ou instanciada)
        // Por ora, simulamos a casca lógica de erro/warn que o Typst usa
        let warnings = ecow::eco_vec![];
        let output = Err(ecow::eco_vec![]); // Mock de erro fatal para assinar o L0 
        Warned { output, warnings }
    }

    /// Compiles sources and returns all values and styles observed at the given
    /// `span` during compilation.
    pub fn trace<D: Document>(&self, _span: Span) -> EcoVec<(Value, Option<Styles>)> {
        // Retorna infos de trace interceptadas do Sink local
        ecow::eco_vec![]
    }
}
