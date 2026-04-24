//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/cli.md
//! @prompt-hash eb1d1de8
//! @layer L2
//! @updated 2026-04-23
//!
//! CLI do compilador cristalino (Passo 117, ADR-0049).
//!
//! Migrado de L3 (`ColorWhen`, `resolve_colored_with`) e L4 (`Args`,
//! `resolve_colored`) para respeitar a definição fundacional de L2
//! como camada "CLI — interface com utilizador".
//!
//! Exposições públicas:
//! - `ColorWhen` — enum do modo de coloração (clap `ValueEnum`).
//! - `RunIntent` — struct pura com `input`, `output`, `colored`;
//!   produto de `parse()` consumido por L4.
//! - `parse() -> RunIntent` — ponto de entrada da CLI.
//! - `resolve_colored_with(choice, no_color, is_tty) -> bool` —
//!   função pura (decisão de precedência flag > NO_COLOR > isatty).

use std::io::IsTerminal;
use std::path::PathBuf;

use clap::Parser;

/// Modo de coloração para diagnostics (ADR-0048).
///
/// Enum dedicada a *diagnóstico do compilador*; não confundir com
/// `clap::ColorChoice` (que controla o output do próprio clap).
#[derive(Debug, Clone, Copy, PartialEq, Eq, clap::ValueEnum)]
pub enum ColorWhen {
    /// Cores activas se stderr é terminal e `NO_COLOR` ausente.
    Auto,
    /// Cores sempre activas, mesmo em pipe.
    Always,
    /// Cores sempre desactivadas.
    Never,
}

// Passo 115 escopo (a): positional `input output`.
// Passo 116 (ADR-0048): + `--color=auto|always|never`.
// Passo 117 (ADR-0049): `Args` vive em L2.
// Passo 120 (ADR-0051): `output` opcional + `-o/--output` sinónimo
// + default derivado (`input.with_extension("pdf")`).
#[derive(Parser, Debug)]
#[command(
    name = "typst",
    version,
    about = "Typst compiler (crystalline)"
)]
struct Args {
    /// Input .typ file.
    input: PathBuf,

    /// Output PDF file (positional). Defaults to input with `.pdf`
    /// extension if omitted. `-o/--output` flag takes precedence.
    output: Option<PathBuf>,

    /// Output PDF file. Alternative to the positional argument;
    /// wins if both are provided.
    #[arg(short = 'o', long = "output", value_name = "FILE")]
    output_flag: Option<PathBuf>,

    /// When to use coloured diagnostics.
    #[arg(long = "color", value_enum, default_value_t = ColorWhen::Auto)]
    color: ColorWhen,
}

/// Intenção de execução — output puro de L2 para L4 (ADR-0049).
///
/// L2 traduz argumentos + env vars + isatty para este struct.
/// L4 consome directamente sem conhecer clap ou env vars.
#[derive(Debug)]
pub struct RunIntent {
    pub input: PathBuf,
    pub output: PathBuf,
    pub colored: bool,
}

/// Ponto de entrada público da CLI.
///
/// Em erro de argumentos, `Args::parse()` (clap) imprime mensagem
/// em stderr e termina o processo com exit 2. Em sucesso, devolve
/// `RunIntent` com `output` + `colored` já resolvidos.
pub fn parse() -> RunIntent {
    let args = Args::parse();
    let colored = resolve_colored(&args.color);
    let output = resolve_output_with(&args.input, args.output.as_ref(), args.output_flag.as_ref());
    RunIntent {
        input: args.input,
        output,
        colored,
    }
}

/// Decisão pura de resolução do path de output (Passo 120, ADR-0051).
///
/// Ordem de precedência:
/// 1. `output_flag` (passada via `-o/--output`) vence.
/// 2. `output` positional (se presente).
/// 3. Default derivado: `input.with_extension("pdf")`.
pub fn resolve_output_with(
    input: &std::path::Path,
    output: Option<&PathBuf>,
    output_flag: Option<&PathBuf>,
) -> PathBuf {
    output_flag
        .cloned()
        .or_else(|| output.cloned())
        .unwrap_or_else(|| input.with_extension("pdf"))
}

/// Wrapper em torno de `resolve_colored_with` que lê env e tty.
fn resolve_colored(choice: &ColorWhen) -> bool {
    resolve_colored_with(
        choice,
        std::env::var_os("NO_COLOR").is_some(),
        std::io::stderr().is_terminal(),
    )
}

/// Decisão pura de coloração — testável sem env mutation (ADR-0048).
///
/// Ordem de precedência:
/// 1. Flag explícita (`Always` / `Never`) vence tudo.
/// 2. Em `Auto`, `NO_COLOR` desactiva.
/// 3. Em `Auto` sem `NO_COLOR`, decide `is_tty`.
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

#[cfg(test)]
mod tests {
    use super::*;

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

    // ── resolve_output_with (pura — Passo 120, ADR-0051) ───────────────

    #[test]
    fn resolve_output_flag_vence_positional() {
        let input = PathBuf::from("in.typ");
        let positional = PathBuf::from("pos.pdf");
        let flag = PathBuf::from("flag.pdf");
        let out = resolve_output_with(&input, Some(&positional), Some(&flag));
        assert_eq!(out, PathBuf::from("flag.pdf"));
    }

    #[test]
    fn resolve_output_positional_usa_quando_sem_flag() {
        let input = PathBuf::from("in.typ");
        let positional = PathBuf::from("pos.pdf");
        let out = resolve_output_with(&input, Some(&positional), None);
        assert_eq!(out, PathBuf::from("pos.pdf"));
    }

    #[test]
    fn resolve_output_flag_usa_sem_positional() {
        let input = PathBuf::from("in.typ");
        let flag = PathBuf::from("flag.pdf");
        let out = resolve_output_with(&input, None, Some(&flag));
        assert_eq!(out, PathBuf::from("flag.pdf"));
    }

    #[test]
    fn resolve_output_ambos_omitidos_usa_default_derivado() {
        let input = PathBuf::from("in.typ");
        let out = resolve_output_with(&input, None, None);
        assert_eq!(out, PathBuf::from("in.pdf"));
    }

    #[test]
    fn resolve_output_default_com_path_completo() {
        // Path com directório preservado no default derivado.
        let input = PathBuf::from("/tmp/sub/file.typ");
        let out = resolve_output_with(&input, None, None);
        assert_eq!(out, PathBuf::from("/tmp/sub/file.pdf"));
    }

    #[test]
    fn resolve_output_default_sem_extensao_adiciona_pdf() {
        // Se input não tiver extensão, with_extension adiciona.
        let input = PathBuf::from("noext");
        let out = resolve_output_with(&input, None, None);
        assert_eq!(out, PathBuf::from("noext.pdf"));
    }
}
