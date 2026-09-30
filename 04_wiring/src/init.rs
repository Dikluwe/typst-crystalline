//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/init.md
//! @prompt-hash 5eb1d938
//! @layer L4
//! @updated 2026-09-26

use std::process::ExitCode;
use typst_shell::cli::InitIntent;

pub(super) fn run_init(intent: InitIntent) -> ExitCode {
    match typst_infra::project_init::initialize(
        &intent.template,
        intent.directory,
        intent.cert_path,
    ) {
        Ok(result) => {
            println!("Successfully created project in {}", result.destination.display());
            println!("Entrypoint: {}", result.entrypoint.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}
