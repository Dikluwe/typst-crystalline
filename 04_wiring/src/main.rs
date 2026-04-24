//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash daf27284
//! @layer L4
//! @updated 2026-04-23
//!
//! CLI mínima do compilador cristalino (Passo 113, ADR-0046;
//! argparsing migrado para `clap` no Passo 115, ADR-0047).
//!
//! Uso:
//! ```text
//! typst <INPUT> <OUTPUT>
//! typst --help
//! typst --version
//! ```
//!
//! Pipeline: `SystemWorld::new` → `eval_to_module_with_sink` →
//! `introspect` → `layout` → `export_pdf` → escrita de bytes.
//! Warnings e errors formatados via ADR-0045
//! (`path:linha:coluna: severity: mensagem`) e emitidos em stderr.
//!
//! Exit codes:
//! - 0: sucesso (PDF escrito).
//! - 1: erro de compilação (eval falhou ou gerou errors).
//! - 2: erro de argumentos (clap) ou I/O.

use std::path::{Path, PathBuf};
use std::process::ExitCode;

use clap::Parser;

use typst_core::contracts::world::World;
use typst_infra::diagnostic_format::drain_diagnostics_to_stderr;
use typst_infra::pipeline::compile_to_pdf_bytes;
use typst_infra::world::SystemWorld;

// Escopo (a) do Passo 115 — mínimo: positional `input output`.
// `--help` e `--version` vêm gratuitos do derive.
#[derive(Parser, Debug)]
#[command(
    name = "typst",
    version,
    about = "Typst compiler (crystalline)"
)]
struct Args {
    /// Input .typ file.
    input: PathBuf,
    /// Output PDF file.
    output: PathBuf,
}

fn main() -> ExitCode {
    let args = Args::parse();
    let input = args.input;
    let output = args.output;

    // Raiz do projecto: directório do input (ou "." se input é bare).
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

    // Path do source para mensagens de diagnóstico (ADR-0045).
    let source_path = input.display().to_string();

    // Pipeline: eval → introspect → layout → export_pdf.
    let (result, warnings) = compile_to_pdf_bytes(&world, &source);

    // Warnings primeiro — consistência com convenção gcc/clang.
    drain_diagnostics_to_stderr(&warnings, &source, &source_path);

    match result {
        Ok(pdf_bytes) => {
            if let Err(e) = std::fs::write(&output, &pdf_bytes) {
                eprintln!("error: failed to write {}: {}", output.display(), e);
                return ExitCode::from(2);
            }
            ExitCode::SUCCESS
        }
        Err(errors) => {
            drain_diagnostics_to_stderr(&errors, &source, &source_path);
            ExitCode::from(1)
        }
    }
}
