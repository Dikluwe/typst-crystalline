//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/diagnostic.md
//! @prompt-hash a861c08e
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

const ANSI_RED_BOLD: &str = "\x1b[1;31m";
const ANSI_YELLOW_BOLD: &str = "\x1b[1;33m";
const ANSI_CYAN_BOLD: &str = "\x1b[1;36m";
const ANSI_DIM: &str = "\x1b[2m";
const ANSI_BOLD: &str = "\x1b[1m";
const ANSI_RESET: &str = "\x1b[0m";

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
        Severity::Error => (ANSI_RED_BOLD, "error"),
        Severity::Warning => (ANSI_YELLOW_BOLD, "warning"),
    };

    let location = match source.span_to_line_col(diag.span) {
        Some((line, col)) => format!("{}:{}:{}", source_path, line, col),
        None => format!("{}:<detached>", source_path),
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

    // P846 (#57) — call trace, formato medido no vanilla
    // (`typst-kit/src/diagnostics.rs:105-146`): por nível, uma linha
    // `  <tracepoint> at <path>:<linha>:<col>` (2 espaços) seguida de 4
    // espaços e o texto fonte do span do tracepoint (multi-linha: primeira
    // linha + `…` + último char não-whitespace). Tracepoints cujo span não
    // resolve no ficheiro são omitidos (comportamento do `emit_trace`).
    for point in &diag.trace {
        let Some((line, col)) = source.span_to_line_col(point.span) else {
            continue;
        };
        let kind = match &point.v {
            typst_core::entities::source_result::Tracepoint::Call(Some(name)) => {
                format!("while calling `{name}`")
            }
            typst_core::entities::source_result::Tracepoint::Call(None) => {
                "while calling function".to_string()
            }
            typst_core::entities::source_result::Tracepoint::Show(name) => {
                format!("while showing {name} element")
            }
            typst_core::entities::source_result::Tracepoint::Import(name) => {
                format!("while importing `{name}`")
            }
            typst_core::entities::source_result::Tracepoint::Include(name) => {
                format!("while including `{name}`")
            }
        };
        if colored {
            out.push_str(&format!(
                "  {kind} at {dim}{source_path}:{line}:{col}{reset}\n",
                dim = ANSI_DIM,
                reset = ANSI_RESET,
            ));
        } else {
            out.push_str(&format!("  {kind} at {source_path}:{line}:{col}\n"));
        }
        let Some(range) = source.span_byte_range(point.span) else {
            continue;
        };
        let Some(text) = source.text().get(range) else {
            continue;
        };
        let mut lines = text.lines();
        let first = lines.next().unwrap_or("");
        let mut snippet = first.to_string();
        if let Some(last) = lines.next_back() {
            if let Some(last_char) = last.chars().next_back() {
                if !last_char.is_whitespace() {
                    snippet = format!("{first}…{last_char}");
                }
            }
        }
        if colored {
            out.push_str(&format!(
                "    {dim}{snippet}{reset}\n",
                dim = ANSI_DIM,
                reset = ANSI_RESET,
            ));
        } else {
            out.push_str(&format!("    {snippet}\n"));
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
        assert!(
            out.contains("\x1b["),
            "output com cores deve conter escapes ANSI; got: {:?}",
            out
        );
    }

    #[test]
    fn formato_com_cores_error_usa_vermelho_bold() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::error(Span::detached(), "falha");
        let out = format_diagnostic(&d, &src, "in.typ", true);
        assert!(
            out.contains(ANSI_RED_BOLD),
            "error deve usar vermelho bold; got: {:?}",
            out
        );
        assert!(
            !out.contains(ANSI_YELLOW_BOLD),
            "error não deve usar amarelo; got: {:?}",
            out
        );
    }

    #[test]
    fn formato_com_cores_warning_usa_amarelo_bold() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "aviso");
        let out = format_diagnostic(&d, &src, "in.typ", true);
        assert!(
            out.contains(ANSI_YELLOW_BOLD),
            "warning deve usar amarelo bold; got: {:?}",
            out
        );
        assert!(
            !out.contains(ANSI_RED_BOLD),
            "warning não deve usar vermelho; got: {:?}",
            out
        );
    }

    #[test]
    fn formato_com_cores_hint_usa_ciano_bold() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "m").with_hint("pista");
        let out = format_diagnostic(&d, &src, "in.typ", true);
        assert!(
            out.contains(ANSI_CYAN_BOLD),
            "hint deve usar ciano bold; got: {:?}",
            out
        );
    }

    #[test]
    fn formato_com_cores_cada_span_fecha_com_reset() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "m").with_hint("pista");
        let out = format_diagnostic(&d, &src, "in.typ", true);

        // Cada abertura ANSI (exceptuando RESET) deve ter pelo menos
        // um RESET subsequente.
        let opens = out.matches(ANSI_RED_BOLD).count()
            + out.matches(ANSI_YELLOW_BOLD).count()
            + out.matches(ANSI_CYAN_BOLD).count()
            + out.matches(ANSI_DIM).count()
            + out.matches(ANSI_BOLD).count();
        let resets = out.matches(ANSI_RESET).count();
        assert!(
            resets >= opens,
            "RESETS ({}) deve ser >= aberturas ({}); got: {:?}",
            resets,
            opens,
            out
        );
        assert!(resets > 0, "pelo menos 1 RESET esperado");
    }

    #[test]
    fn formato_com_cores_preserva_conteudo() {
        let src = Source::detached("x");
        let d = SourceDiagnostic::warning(Span::detached(), "aviso especifico");
        let out = format_diagnostic(&d, &src, "file.typ", true);
        // Texto semântico presente mesmo com cores:
        assert!(out.contains("warning"), "texto 'warning' presente; got: {:?}", out);
        assert!(out.contains("aviso especifico"), "mensagem preservada; got: {:?}", out);
        assert!(out.contains("file.typ"), "path presente; got: {:?}", out);
        assert!(out.contains("<detached>"), "detached presente; got: {:?}", out);
    }

    // ── P846 (#57) — call trace ─────────────────────────────────────────────

    #[test]
    fn formato_com_call_trace_sem_cores() {
        // Formato medido no vanilla (`typst-kit/src/diagnostics.rs:105-146`):
        // `  while calling \`name\` at path:linha:col` (2 espaços) seguido de
        // linha com 4 espaços e o texto fonte do span do tracepoint.
        // `Call(None)` → `while calling function` (sem backticks).
        use typst_core::entities::source_result::Tracepoint;
        use typst_core::entities::span::Spanned;
        let src = Source::detached("#let b() = { c() }\n#b()\n");
        let mut d =
            SourceDiagnostic::error(Span::detached(), "cannot add integer and string");
        // `c()` na linha 1, col 13 (0-indexed); `b()` na linha 2, col 1.
        d.trace
            .push(Spanned::new(Tracepoint::Call(Some("b".into())), Span::from_range(src.id(), 13..16)));
        d.trace
            .push(Spanned::new(Tracepoint::Call(None), Span::from_range(src.id(), 20..23)));
        let out = format_diagnostic(&d, &src, "in.typ", false);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(
            lines,
            vec![
                "in.typ:<detached>: error: cannot add integer and string",
                "  while calling `b` at in.typ:1:13",
                "    c()",
                "  while calling function at in.typ:2:1",
                "    b()",
            ]
        );
    }

    #[test]
    fn formato_trace_snippet_multilinha_com_ellipsis() {
        // Vanilla (`emit_trace`): span multi-linha mostra a primeira linha
        // seguida de `…` e do último char (se não whitespace).
        use typst_core::entities::source_result::Tracepoint;
        use typst_core::entities::span::Spanned;
        let src = Source::detached("#f(\n  1\n)\n");
        let mut d = SourceDiagnostic::error(Span::detached(), "boom");
        // Span da chamada `f(\n  1\n)` (linha 1, col 1).
        d.trace
            .push(Spanned::new(Tracepoint::Call(Some("f".into())), Span::from_range(src.id(), 1..9)));
        let out = format_diagnostic(&d, &src, "in.typ", false);
        let lines: Vec<&str> = out.lines().collect();
        assert_eq!(lines[1], "  while calling `f` at in.typ:1:1");
        assert_eq!(lines[2], "    f(…)");
    }

    #[test]
    fn formato_trace_span_nao_resolvido_e_omitido() {
        // Vanilla (`emit_trace`): tracepoint cujo span não resolve no ficheiro
        // não produz output nenhum.
        use typst_core::entities::source_result::Tracepoint;
        use typst_core::entities::span::Spanned;
        let src = Source::detached("x");
        let mut d = SourceDiagnostic::error(Span::detached(), "boom");
        d.trace.push(Spanned::detached(Tracepoint::Call(Some("f".into()))));
        let out = format_diagnostic(&d, &src, "in.typ", false);
        assert_eq!(out, "in.typ:<detached>: error: boom\n");
    }
}
