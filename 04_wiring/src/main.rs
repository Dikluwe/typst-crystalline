//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash a2b00a19
//! @layer L4
//! @updated 2026-04-23
//!
//! CLI mínima do compilador cristalino — composição thin.
//!
//! L4 apenas orquestra: `cli::parse()` em L2 → pipeline em L3 →
//! escrita de output. Toda a lógica de args/cores/flags vive em L2
//! (`typst_shell::cli`); toda a lógica de compilação vive em L3
//! (`typst_infra::pipeline`).
//!
//! Passos relevantes:
//! - Passo 113 (ADR-0046): CLI mínima.
//! - Passo 115 (ADR-0047): `clap` argparsing.
//! - Passo 116 (ADR-0048): cores ANSI.
//! - Passo 117 (ADR-0049): CLI movida para L2; L4 é composição pura.
//!
//! Exit codes:
//! - 0: sucesso.
//! - 1: erro de compilação (eval).
//! - 2: erro de I/O ou argumentos (clap, via L2).

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use typst_core::contracts::world::World;
use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;
use typst_infra::pipeline::compile_to_pdf_bytes;
use typst_infra::world::SystemWorld;
use typst_shell::cli::{self, RunIntent};

fn main() -> ExitCode {
    let RunIntent { input, output, colored } = cli::parse();

    let root = input
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .map(Path::to_path_buf)
        .unwrap_or_else(|| PathBuf::from("."));

    let main_path = match input.file_name() {
        Some(name) => PathBuf::from(name),
        None => {
            eprintln!("error: input path must have a file name: {}", input.display());
            return ExitCode::from(2);
        }
    };

    let world = match SystemWorld::new(&root, &main_path) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::from(2);
        }
    };

    let source = match world.source(world.main()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to load source: {:?}", e);
            return ExitCode::from(2);
        }
    };

    let source_path = input.display().to_string();

    let (result, warnings) = compile_to_pdf_bytes(&world, &source);
    drain_diagnostics_to_stderr(&warnings, &source, &source_path, colored);

    match result {
        Ok(pdf_bytes) => {
            if let Err(e) = std::fs::write(&output, &pdf_bytes) {
                eprintln!("error: failed to write {}: {}", output.display(), e);
                return ExitCode::from(2);
            }
            ExitCode::SUCCESS
        }
        Err(errors) => {
            drain_diagnostics_to_stderr(&errors, &source, &source_path, colored);
            ExitCode::from(1)
        }
    }
}
