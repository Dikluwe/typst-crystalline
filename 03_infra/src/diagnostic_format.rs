//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/infra/diagnostic_format.md
//! @prompt-hash 38db9eb0
//! @layer L3
//! @updated 2026-04-23
//!
//! Formato rico de diagnósticos (ADR-0045, Passo 111).
//!
//! Produz saída gcc/clang-compatível:
//!
//! ```text
//! path:linha:coluna: severity: message
//!   hint: hint 1
//!   hint: hint 2
//! ```
//!
//! Promovido de test-only (`integration_tests.rs`) a API pública
//! no Passo 113 (ADR-0046) para consumo pelo 04_wiring (CLI).

use typst_core::entities::source::Source;
use typst_core::entities::source_result::{Severity, SourceDiagnostic};

/// Formata um `SourceDiagnostic` em texto simples gcc/clang-compatível.
///
/// Termina com `\n` final. Hints indentados com 2 espaços.
/// Spans detached ou cross-file caem em `<path>:<detached>:`.
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
) -> String {
    let severity = match diag.severity {
        Severity::Error   => "error",
        Severity::Warning => "warning",
    };

    let location = match source.span_to_line_col(diag.span) {
        Some((line, col)) => format!("{}:{}:{}", source_path, line, col),
        None              => format!("{}:<detached>", source_path),
    };

    let mut out = format!("{}: {}: {}\n", location, severity, diag.message);
    for hint in &diag.hints {
        out.push_str(&format!("  hint: {}\n", hint));
    }
    out
}

/// Dreno para stderr — cobre warnings e errors uniformemente
/// (ADR-0045). Nome reflecte a uniformidade.
pub fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
) {
    for diag in diagnostics {
        eprint!("{}", format_diagnostic(diag, source, source_path));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::source_result::SourceDiagnostic;
    use typst_core::entities::span::Span;

    #[test]
    fn formato_warning_detached() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "msg");
        let out = format_diagnostic(&d, &src, "in.typ");
        assert_eq!(out, "in.typ:<detached>: warning: msg\n");
    }

    #[test]
    fn formato_error_uniforme() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::error(Span::detached(), "falha");
        let out = format_diagnostic(&d, &src, "in.typ");
        assert_eq!(out, "in.typ:<detached>: error: falha\n");
    }

    #[test]
    fn formato_com_hints() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "m")
            .with_hint("primeiro")
            .with_hint("segundo");
        let out = format_diagnostic(&d, &src, "in.typ");
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "in.typ:<detached>: warning: m");
        assert_eq!(lines[1], "  hint: primeiro");
        assert_eq!(lines[2], "  hint: segundo");
    }
}
