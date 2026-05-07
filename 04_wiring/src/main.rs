//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash 63faaf24
//! @layer L4
//! @updated 2026-04-23
//!
//! CLI mínima do compilador cristalino — composição thin.
//!
//! L4 apenas orquestra: `cli::parse()` em L2 → pipeline em L3 →
//! escrita de output. Toda a lógica de args/cores/diagnostics vive
//! em L2 (`typst_shell::{cli,diagnostic}`); toda a lógica de
//! compilação vive em L3 (`typst_infra::pipeline`).
//!
//! Passos relevantes:
//! - Passo 113 (ADR-0046): CLI mínima.
//! - Passo 115 (ADR-0047): `clap` argparsing.
//! - Passo 116 (ADR-0048): cores ANSI.
//! - Passo 117 (ADR-0049): CLI movida para L2; L4 é composição pura.
//! - Passo 119 (ADR-0050): formatter completamente em L2; drain
//!   inline em L4 (helper local `drain_to_stderr`).
//! - Passo 121 (ADR-0051): `--root` resolvido em L2; L4 apenas consome
//!   `intent.root` — sem cálculo local de parent.
//! - Passo 122 (ADR-0051): `--font-path` (repetível) resolvido em L2;
//!   L4 invoca `discover_fonts` + `.with_fonts(...)`.
//!
//! Exit codes:
//! - 0: sucesso.
//! - 1: erro de compilação (eval).
//! - 2: erro de I/O ou argumentos (clap, via L2).

// P204E (M8): wrapper `crystalline_evict` sobre `comemo::evict`
// per ADR-0073. Reservado para integração CLI / watch mode
// futura (não exercido em P204E — apenas exposto).
mod eviction;

// P204G (M8): measurements internos (cache stats + counts de
// invocação Introspector) vivem em L3
// (`typst_infra::measurements`) per ADR-0073. L4 apenas dispara
// dump opt-in quando `CRYSTALLINE_MEASUREMENTS=1`. V12 OK:
// L4 não cria tipos, apenas consome `cache_stats()` e
// `introspector_call_counts()`.

use std::path::PathBuf;
use std::process::ExitCode;

use typst_core::contracts::world::World;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_infra::fonts::discover_fonts;
use typst_infra::pipeline::compile_to_pdf_bytes;
use typst_infra::world::SystemWorld;
use typst_shell::cli::{self, RunIntent};
use typst_shell::diagnostic::format_diagnostic;

fn main() -> ExitCode {
    let RunIntent { input, output, root, font_paths, colored } = cli::parse();

    let main_path = match input.file_name() {
        Some(name) => PathBuf::from(name),
        None => {
            eprintln!("error: input path must have a file name: {}", input.display());
            return ExitCode::from(2);
        }
    };

    let font_slots = discover_fonts(&font_paths);

    let world = match SystemWorld::new(&root, &main_path) {
        Ok(w) => w.with_fonts(font_slots),
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
    drain_to_stderr(&warnings, &source, &source_path, colored);

    let exit_code = match result {
        Ok(pdf_bytes) => {
            if let Err(e) = std::fs::write(&output, &pdf_bytes) {
                eprintln!("error: failed to write {}: {}", output.display(), e);
                return ExitCode::from(2);
            }
            ExitCode::SUCCESS
        }
        Err(errors) => {
            drain_to_stderr(&errors, &source, &source_path, colored);
            ExitCode::from(1)
        }
    };

    // P204G (M8): logging opt-in. `CRYSTALLINE_MEASUREMENTS=1`
    // dump cache_stats + introspector_call_counts no fim do
    // pipeline. Default silencioso. Não muda valores em tests
    // (env var não setada por defeito).
    if std::env::var("CRYSTALLINE_MEASUREMENTS").as_deref() == Ok("1") {
        let stats = typst_infra::measurements::cache_stats();
        let counts = typst_infra::measurements::introspector_call_counts();
        eprintln!(
            "[crystalline] cache_stats: evict_calls={} last_max_age={}",
            stats.evict_calls, stats.last_max_age,
        );
        eprintln!(
            "[crystalline] introspector_call_counts: total={}",
            counts.total,
        );
        for (method, count) in &counts.per_method {
            if *count > 0 {
                eprintln!("[crystalline]   {}: {}", method, count);
            }
        }
    }

    exit_code
}

/// Helper local: drena diagnostics para stderr via formatter de L2.
///
/// Substitui `typst_infra::diagnostic_format::drain_diagnostics_to_stderr`
/// eliminado no Passo 119 (ADR-0050). L4 faz I/O trivial (`eprint!`)
/// sem criar tipos (V12 OK) — composição pura.
fn drain_to_stderr(
    diagnostics: &[SourceDiagnostic],
    source: &Source,
    source_path: &str,
    colored: bool,
) {
    for diag in diagnostics {
        eprint!("{}", format_diagnostic(diag, source, source_path, colored));
    }
}
