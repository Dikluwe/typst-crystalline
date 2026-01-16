use crate::core::diagnostics::{CrystalResult, VoidSignal};
use crate::shell::diagnostics::Shield;
use typst::diag::SourceDiagnostic;
use typst::syntax::Span;

/// A Bridge é a Composição (L4) que executa a "Solda" entre camadas.
pub struct DiagnosticBridge;

impl DiagnosticBridge {
    /// Resolve um colapso geométrico do Core em um diagnóstico pronto para o Shell.
    ///
    /// Esta função atua como o terminal da Law IV, pareando o sinal puramente lógico
    /// com a coordenada geográfica (Span) que o Core é proibido de conhecer.
    pub fn resolve<T>(
        result: CrystalResult<T>,
        geography: Span,
    ) -> Result<T, SourceDiagnostic> {
        result.map_err(|signal| {
            // Injeção de Dependência Social: O Shield (L2) é invocado aqui.
            // O Core (L1) nunca soube da existência do Shield.
            Shield::project(signal.as_ref(), geography)
        })
    }

    /// Executa uma operação lógica e converte qualquer falha imediatamente.
    pub fn perform<F, T>(logic: F, span: Span) -> Result<T, SourceDiagnostic>
    where
    F: FnOnce() -> CrystalResult<T>,
    {
        let outcome = logic();
        Self::resolve(outcome, span)
    }
}
