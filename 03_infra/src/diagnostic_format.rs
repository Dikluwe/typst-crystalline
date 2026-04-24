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
//! Passo 116 (ADR-0048): ganho parâmetro `colored: bool`. Quando
//! `true`, aplica escapes ANSI (vermelho/amarelo bold para
//! severity, dim para path, bold para message, ciano bold para
//! hint). A decisão de quando colorir é delegada a L4
//! (`resolve_colored` com precedência flag > NO_COLOR > isatty).

use typst_core::entities::source::Source;
use typst_core::entities::source_result::{Severity, SourceDiagnostic};

// ── Paleta ANSI (Passo 116, ADR-0048) ────────────────────────────────────

const ANSI_RED_BOLD:    &str = "\x1b[1;31m";
const ANSI_YELLOW_BOLD: &str = "\x1b[1;33m";
const ANSI_CYAN_BOLD:   &str = "\x1b[1;36m";
const ANSI_DIM:         &str = "\x1b[2m";
const ANSI_BOLD:        &str = "\x1b[1m";
const ANSI_RESET:       &str = "\x1b[0m";

// ── Controlo de coloração (Passo 116, ADR-0048) ──────────────────────────

/// Modo de coloração para diagnostics. Exposto por L3 (e não por L4)
/// porque L4 compõe mas não cria tipos (regra V12 do linter); L4
/// importa directamente via `typst_infra::diagnostic_format::ColorWhen`.
///
/// `clap::ValueEnum` derive permite consumo directo com
/// `#[arg(value_enum, default_value_t = ColorWhen::Auto)]` em L4.
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorWhen {
    /// Cores activas se stderr é terminal e `NO_COLOR` ausente.
    Auto,
    /// Cores sempre activas, mesmo em pipe.
    Always,
    /// Cores sempre desactivadas.
    Never,
}

/// Decisão pura de coloração — testável sem env mutation (ADR-0048).
///
/// Ordem de precedência:
/// 1. Flag explícita (`Always` / `Never`) vence tudo.
/// 2. Em `Auto`, `NO_COLOR` desactiva.
/// 3. Em `Auto` sem `NO_COLOR`, decide `is_tty`.
///
/// L4 envolve esta função com a leitura real de env e tty — ver
/// `04_wiring/src/main.rs::resolve_colored`.
pub fn resolve_colored_with(
    choice: &ColorWhen,
    no_color_present: bool,
    is_tty: bool,
) -> bool {
    match choice {
        ColorWhen::Never  => false,
        ColorWhen::Always => true,
        ColorWhen::Auto   => !no_color_present && is_tty,
    }
}

/// Formata um `SourceDiagnostic` em texto simples gcc/clang-compatível.
///
/// Termina com `\n` final. Hints indentados com 2 espaços.
/// Spans detached ou cross-file caem em `<path>:<detached>:`.
///
/// `colored = false` produz output idêntico ao Passo 111 (ADR-0045).
/// `colored = true` aplica ANSI escapes da paleta (ADR-0048).
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

/// Dreno para stderr — cobre warnings e errors uniformemente
/// (ADR-0045). Nome reflecte a uniformidade.
///
/// `colored` propagado a cada `format_diagnostic` (ADR-0048).
pub fn drain_diagnostics_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
    colored: bool,
) {
    for diag in diagnostics {
        eprint!("{}", format_diagnostic(diag, source, source_path, colored));
    }
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
        // um RESET subsequente — avaliação qualitativa:
        // contar abrir vs fechar é difícil porque o ESC de RESET aparece
        // múltiplas vezes; confiar em que o número de RESETS iguala ou
        // excede o número de aberturas coloridas.
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

    // ── resolve_colored_with (pura — Passo 116, ADR-0048) ───────────────

    #[test]
    fn resolve_colored_never_e_false() {
        assert_eq!(resolve_colored_with(&ColorWhen::Never, false, false), false);
        assert_eq!(resolve_colored_with(&ColorWhen::Never, true,  false), false);
        assert_eq!(resolve_colored_with(&ColorWhen::Never, false, true),  false);
        assert_eq!(resolve_colored_with(&ColorWhen::Never, true,  true),  false);
    }

    #[test]
    fn resolve_colored_always_e_true() {
        assert_eq!(resolve_colored_with(&ColorWhen::Always, false, false), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true,  false), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, false, true),  true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true,  true),  true);
    }

    #[test]
    fn resolve_colored_auto_sem_tty_e_false() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, false, false), false);
    }

    #[test]
    fn resolve_colored_auto_com_tty_e_sem_no_color_e_true() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, false, true), true);
    }

    #[test]
    fn resolve_colored_auto_com_no_color_e_false() {
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, true, true),  false);
        assert_eq!(resolve_colored_with(&ColorWhen::Auto, true, false), false);
    }

    #[test]
    fn resolve_colored_always_vence_no_color() {
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true, false), true);
        assert_eq!(resolve_colored_with(&ColorWhen::Always, true, true),  true);
    }
}
