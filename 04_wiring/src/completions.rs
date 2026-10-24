//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/completions.md
//! @prompt-hash 71a49c3f
//! @layer L4
//! @updated 2026-09-26

use super::diagnostics::write_stderr;
use std::io::Write;
use std::process::ExitCode;
use typst_shell::cli::CompletionsIntent;

pub(super) fn run_completions(intent: CompletionsIntent) -> std::io::Result<ExitCode> {
    let output = typst_shell::completions::generate(intent.shell);
    if let Err(error) = std::io::stdout().lock().write_all(&output) {
        write_stderr(&format!("error: failed to write completions: {error}\n"))?;
        return Ok(ExitCode::from(2));
    }
    Ok(ExitCode::SUCCESS)
}
