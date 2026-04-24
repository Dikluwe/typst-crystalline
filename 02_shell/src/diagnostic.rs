//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/diagnostic.md
//! @prompt-hash 8f708d2b
//! @layer L2
//! @updated 2026-04-23
//!
//! Formatter de diagnósticos para saída em terminal (Passo 119,
//! ADR-0050 — completa migração iniciada no Passo 117/ADR-0049).
//!
//! Migrado de `03_infra/src/diagnostic_format.rs`. Razão: decidir
//! o formato user-facing (palavras "warning:", cores ANSI,
//! indentação de hints) é concern de apresentação — pertence a
//! L2 (shell). L3 mantém-se I/O puro.
//!
//! Formato gcc/clang-compatível (ADR-0045):
//!
//! ```text
//! path:linha:coluna: severity: message
//!   hint: hint 1
//!   hint: hint 2
//! ```
//!
//! Parâmetro `colored: bool` (ADR-0048): quando `true`, aplica
//! escapes ANSI. Decisão de quando colorir é tomada pelo caller
//! em L2 via `cli::resolve_colored_with`.

use typst_core::entities::source::Source;
use typst_core::entities::source_result::{Severity, SourceDiagnostic};

// ── Paleta ANSI (Passo 116, ADR-0048; migrada Passo 119, ADR-0050) ──────

const ANSI_RED_BOLD:    &str = "\x1b[1;31m";
const ANSI_YELLOW_BOLD: &str = "\x1b[1;33m";
const ANSI_CYAN_BOLD:   &str = "\x1b[1;36m";
const ANSI_DIM:         &str = "\x1b[2m";
const ANSI_BOLD:        &str = "\x1b[1m";
const ANSI_RESET:       &str = "\x1b[0m";

/// Formata um `SourceDiagnostic` em texto gcc/clang-compatível.
///
/// Termina com `\n` final. Hints indentados com 2 espaços.
/// Spans detached ou cross-file caem em `<path>:<detached>:`.
///
/// `colored = false` produz output simples (formato Passo 111, ADR-0045).
/// `colored = true` aplica ANSI escapes (paleta ADR-0048).
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    source: &Source,
    source_path: &str,
    colored: bool,
) -> String {
    let (sev_color, sev_text) = match diag.severity {
        Severity::Error   => (ANSI_RED_BOLD,    "error"),
        Severity::Warning => (ANSI_YELLOW_BOLD, "warning"),
    };

    let location = match source.span_to_line_col(diag.span) {
        Some((line, col)) => format!("{}:{}:{}", source_path, line, col),
        None              => format!("{}:<detached>", source_path),
    };

    let mut out = if colored {
        format!(
            "{dim}{location}{reset}: {sev}{sev_text}{reset}: {bold}{msg}{reset}\n",
            dim = ANSI_DIM,
            reset = ANSI_RESET,
            sev = sev_color,
            bold = ANSI_BOLD,
            msg = diag.message,
        )
    } else {
        format!("{}: {}: {}\n", location, sev_text, diag.message)
    };

    for hint in &diag.hints {
        if colored {
            out.push_str(&format!(
                "  {cyan}hint{reset}: {hint}\n",
                cyan = ANSI_CYAN_BOLD,
                reset = ANSI_RESET,
            ));
        } else {
            out.push_str(&format!("  hint: {}\n", hint));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use typst_core::entities::source_result::SourceDiagnostic;
    use typst_core::entities::span::Span;

    // ── Sem cores (compatibilidade com Passo 111) ───────────────────────

    #[test]
    fn formato_warning_detached_sem_cores() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "msg");
        let out = format_diagnostic(&d, &src, "in.typ", false);
        assert_eq!(out, "in.typ:<detached>: warning: msg\n");
    }

    #[test]
    fn formato_error_uniforme_sem_cores() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::error(Span::detached(), "falha");
        let out = format_diagnostic(&d, &src, "in.typ", false);
        assert_eq!(out, "in.typ:<detached>: error: falha\n");
    }

    #[test]
    fn formato_com_hints_sem_cores() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "m")
            .with_hint("primeiro")
            .with_hint("segundo");
        let out = format_diagnostic(&d, &src, "in.typ", false);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[0], "in.typ:<detached>: warning: m");
        assert_eq!(lines[1], "  hint: primeiro");
        assert_eq!(lines[2], "  hint: segundo");
    }

    // ── Com cores (Passo 116, ADR-0048) ─────────────────────────────────

    #[test]
    fn formato_com_cores_contem_ansi_escapes() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "msg");
        let out = format_diagnostic(&d, &src, "in.typ", true);
        assert!(out.contains("\x1b["),
            "output com cores deve conter escapes ANSI; got: {:?}", out);
    }

    #[test]
    fn formato_com_cores_error_usa_vermelho_bold() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::error(Span::detached(), "falha");
        let out = format_diagnostic(&d, &src, "in.typ", true);
        assert!(out.contains(ANSI_RED_BOLD),
            "error deve usar vermelho bold; got: {:?}", out);
        assert!(!out.contains(ANSI_YELLOW_BOLD),
            "error não deve usar amarelo; got: {:?}", out);
    }

    #[test]
    fn formato_com_cores_warning_usa_amarelo_bold() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "aviso");
        let out = format_diagnostic(&d, &src, "in.typ", true);
        assert!(out.contains(ANSI_YELLOW_BOLD),
            "warning deve usar amarelo bold; got: {:?}", out);
        assert!(!out.contains(ANSI_RED_BOLD),
            "warning não deve usar vermelho; got: {:?}", out);
    }

    #[test]
    fn formato_com_cores_hint_usa_ciano_bold() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "m")
            .with_hint("pista");
        let out = format_diagnostic(&d, &src, "in.typ", true);
        assert!(out.contains(ANSI_CYAN_BOLD),
            "hint deve usar ciano bold; got: {:?}", out);
    }

    #[test]
    fn formato_com_cores_cada_span_fecha_com_reset() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "m")
            .with_hint("pista");
        let out = format_diagnostic(&d, &src, "in.typ", true);

        // Cada abertura ANSI (exceptuando RESET) deve ter pelo menos
        // um RESET subsequente.
        let opens = out.matches(ANSI_RED_BOLD).count()
            + out.matches(ANSI_YELLOW_BOLD).count()
            + out.matches(ANSI_CYAN_BOLD).count()
            + out.matches(ANSI_DIM).count()
            + out.matches(ANSI_BOLD).count();
        let resets = out.matches(ANSI_RESET).count();
        assert!(resets >= opens,
            "RESETS ({}) deve ser >= aberturas ({}); got: {:?}",
            resets, opens, out);
        assert!(resets > 0, "pelo menos 1 RESET esperado");
    }

    #[test]
    fn formato_com_cores_preserva_conteudo() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "aviso especifico");
        let out = format_diagnostic(&d, &src, "file.typ", true);
        // Texto semântico presente mesmo com cores:
        assert!(out.contains("warning"), "texto 'warning' presente; got: {:?}", out);
        assert!(out.contains("aviso especifico"),
            "mensagem preservada; got: {:?}", out);
        assert!(out.contains("file.typ"), "path presente; got: {:?}", out);
        assert!(out.contains("<detached>"), "detached presente; got: {:?}", out);
    }
}
