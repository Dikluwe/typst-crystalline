//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/completions.md
//! @prompt-hash 8cf42e2c
//! @layer L4
//! @updated 2026-09-26

use std::io::Write;
use std::process::ExitCode;
use typst_shell::cli::CompletionsIntent;

pub(super) fn run_completions(intent: CompletionsIntent) -> ExitCode {
    let output = typst_shell::completions::generate(intent.shell);
    if let Err(error) = std::io::stdout().lock().write_all(&output) {
        eprintln!("error: failed to write completions: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}
