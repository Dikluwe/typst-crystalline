//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/entities/source-result.md
//! @prompt-hash 69b93803
//! @layer L1
//! @updated 2026-03-26

use crate::entities::span::{Span, Spanned};

/// Severidade de um diagnóstico do compilador.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Severity {
    /// Erro fatal.
    Error,
    /// Aviso não-fatal.
    Warning,
}

/// Ponto de rastreio para stack traces de erros de compilação.
///
/// Variantes baseadas em `typst-library/src/diag.rs` — EcoString → String (ADR-0015).
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum Tracepoint {
    /// Uma chamada de função com nome opcional.
    Call(Option<String>),
    /// Aplicação de uma show rule.
    Show(String),
    /// Import de um módulo.
    Import(String),
    /// Include de um módulo.
    Include(String),
}

/// Diagnóstico emitido pelo compilador Typst.
///
/// Migrado de `typst-library/src/diag.rs`. Substituições aplicadas:
/// - `EcoString` → `String` (ADR-0015)
/// - `EcoVec` → `Vec` (ADR-0015)
/// - Hints simplificados para `Vec<String>` sem Spanned (span adicionado quando eval() migrar)
#[derive(Debug, Clone)]
pub struct SourceDiagnostic {
    /// Severidade do diagnóstico.
    pub severity: Severity,
    /// Span do nó relevante no código fonte.
    pub span: Span,
    /// Mensagem descritiva do problema.
    pub message: String,
    /// Hints adicionais para o utilizador.
    pub hints: Vec<String>,
    /// Stack trace de chamadas que levaram ao problema.
    pub trace: Vec<Spanned<Tracepoint>>,
}

impl SourceDiagnostic {
    /// Cria um novo erro.
    pub fn error(span: Span, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Error,
            span,
            message: message.into(),
            hints: vec![],
            trace: vec![],
        }
    }

    /// Cria um novo aviso.
    pub fn warning(span: Span, message: impl Into<String>) -> Self {
        Self {
            severity: Severity::Warning,
            span,
            message: message.into(),
            hints: vec![],
            trace: vec![],
        }
    }

    /// Adiciona um hint ao diagnóstico (builder pattern).
    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hints.push(hint.into());
        self
    }
}

/// Resultado de uma operação do compilador Typst.
///
/// `EcoVec<SourceDiagnostic>` substituído por `Vec<SourceDiagnostic>` (ADR-0015).
pub type SourceResult<T> = Result<T, Vec<SourceDiagnostic>>;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entities::span::Span;

    #[test]
    fn error_construtor() {
        let d = SourceDiagnostic::error(Span::detached(), "msg");
        assert_eq!(d.severity, Severity::Error);
        assert_eq!(d.message, "msg");
        assert!(d.hints.is_empty());
        assert!(d.trace.is_empty());
    }

    #[test]
    fn warning_construtor() {
        let d = SourceDiagnostic::warning(Span::detached(), "warn");
        assert_eq!(d.severity, Severity::Warning);
        assert_eq!(d.message, "warn");
    }

    #[test]
    fn with_hint() {
        let d = SourceDiagnostic::error(Span::detached(), "e")
            .with_hint("try this");
        assert_eq!(d.hints, vec!["try this"]);
    }

    #[test]
    fn with_multiple_hints() {
        let d = SourceDiagnostic::error(Span::detached(), "e")
            .with_hint("hint 1")
            .with_hint("hint 2");
        assert_eq!(d.hints.len(), 2);
        assert_eq!(d.hints[0], "hint 1");
        assert_eq!(d.hints[1], "hint 2");
    }

    #[test]
    fn source_result_ok() {
        let r: SourceResult<u32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }

    #[test]
    fn source_result_err() {
        let r: SourceResult<u32> = Err(vec![
            SourceDiagnostic::error(Span::detached(), "e")
        ]);
        assert!(r.is_err());
        assert_eq!(r.unwrap_err().len(), 1);
    }

    #[test]
    fn tracepoint_variants_exist() {
        // Contrato correcto — todas as variantes reais de Tracepoint existem
        let _ = Tracepoint::Call(None);
        let _ = Tracepoint::Call(Some("foo".into()));
        let _ = Tracepoint::Show("bar".into());
        let _ = Tracepoint::Import("mod".into());
        let _ = Tracepoint::Include("file".into());
    }
}
