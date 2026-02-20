// 💎 Tipologia Tekt: L0 (Contrato de I/O)
// Contrato para abstrair as dependências privadas do módulo `typst-ide::utils`.
// Permite que o L2 Orquestrador acesse globals e engine sem depender de módulos privados.

use typst::foundations::{Scope, Value};
use typst::syntax::{LinkedNode, Span};

/// Abstrai as capacidades de I/O do IDE que o legado esconde em `mod utils;`.
pub trait IIdeEnv {
    /// Resolve o escopo de globais (Math vs Global) para o nó dado.
    /// Delegação da lógica legada `utils::globals(world, leaf)`.
    fn resolve_globals<'a>(&'a self, leaf: &LinkedNode) -> &'a Scope;

    /// Executa uma operação de import usando uma Engine efêmera.
    /// Delegação da lógica legada `utils::with_engine + typst_eval::import`.
    fn execute_import(&self, path: &str, source_span: Span) -> Option<Value>;

    /// Dispara o tracing do compilador num span (fallback pesado de análise).
    /// Delegação da lógica legada `typst::trace::<PagedDocument>(world, span)`.
    fn trace_expr(&self, span: Span) -> ecow::EcoVec<(Value, Option<typst::foundations::Styles>)>;
}
