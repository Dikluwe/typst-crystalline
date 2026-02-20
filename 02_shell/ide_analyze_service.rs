// 💎 Tipologia Tekt: L2 (Serviço de Superfície e Orquestração)
// Módulo de Origem: typst-ide/src/analyze.rs
// Consome o L1 (Lógica Pura) e o Contrato L0 (IIdeEnv) para orquestrar a análise.

use ecow::EcoVec;
use typst::foundations::{Styles, Value};
use typst::syntax::LinkedNode;

// Importa do contexto de compilação do irmão (L4 define o namespace)
use super::ide_env::IIdeEnv;
use super::ide_analyze_logic::{
    ExprAnalysis, ImportSourceKind,
    analyze_basic_expr, analyze_scope_fallback, classify_import_source,
};

/// Orquestrador L2: Consome a lógica pura do L1 e delega I/O para o Contrato IIdeEnv.
pub struct IdeAnalyzerOrchestrator<'a, E: IIdeEnv> {
    env: &'a E,
}

impl<'a, E: IIdeEnv> IdeAnalyzerOrchestrator<'a, E> {
    pub fn new(env: &'a E) -> Self {
        Self { env }
    }

    /// Pipeline de análise de expressão. Tenta resolver puramente no L1,
    /// faz fallback para tracing impuro via Contrato L0 apenas quando necessário.
    pub fn analyze_expr(&self, node: &LinkedNode) -> EcoVec<(Value, Option<Styles>)> {
        let mut current = node.clone();
        loop {
            match analyze_basic_expr(&current) {
                ExprAnalysis::Resolved(values) => return values,
                ExprAnalysis::Recurse(next) => {
                    current = next;
                    continue;
                }
                ExprAnalysis::NeedsTracing => {
                    return self.env.trace_expr(current.span());
                }
                ExprAnalysis::NotAnExpr => return EcoVec::new(),
            }
        }
    }

    /// Pipeline de análise com fallback para globals (dead code).
    pub fn analyze_expr_with_fallback(&self, node: &LinkedNode) -> Option<Value> {
        if let Some((value, _)) = self.analyze_expr(node).into_iter().next() {
            return Some(value);
        }

        let globals = self.env.resolve_globals(node);
        analyze_scope_fallback(node, globals)
    }

    /// Pipeline de import. Classifica puramente no L1 e delega I/O via Contrato.
    pub fn analyze_import(&self, source: &LinkedNode) -> Option<Value> {
        let source_span = source.span();
        let (source_val, _) = self.analyze_expr(source).into_iter().next()?;

        match classify_import_source(source_val) {
            ImportSourceKind::PreloadedModule(v) => Some(v),
            ImportSourceKind::Path(path) => {
                self.env.execute_import(&path, source_span)
            }
            ImportSourceKind::Invalid => None,
        }
    }
}
