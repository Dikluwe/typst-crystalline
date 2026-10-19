//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/query.md
//! @prompt-hash f37ba425
//! @layer L4
//! @updated 2026-10-06

use super::diagnostics::drain_to_stderr;
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
use typst_core::contracts::world::World;
use typst_infra::query_helpers::CompiledQueryError;
use typst_infra::world::{SystemWorld, SystemWorldError};
use typst_shell::cli::{self, QueryIntent};

pub(super) fn run_query(intent: QueryIntent) -> ExitCode {
    let (world, source) = if intent.input == Path::new("-") {
        let mut text = String::new();
        if let Err(error) = std::io::stdin().lock().read_to_string(&mut text) {
            eprintln!("error: failed to read source from stdin: {error}");
            return ExitCode::from(2);
        }
        let root = match std::env::current_dir() {
            Ok(root) => root,
            Err(error) => {
                eprintln!("error: failed to determine current directory: {error}");
                return ExitCode::from(2);
            }
        };
        let world = SystemWorld::for_stdin(root, text)
            .with_fonts_and_system(&[])
            .with_custom_ca(intent.cert_path.clone());
        let source = match world.source(world.main()) {
            Ok(source) => source,
            Err(error) => {
                eprintln!("error: failed to load source: {error:?}");
                return ExitCode::from(2);
            }
        };
        (world, source)
    } else {
        let root = intent
            .input
            .parent()
            .filter(|p| !p.as_os_str().is_empty())
            .unwrap_or(Path::new("."));
        let Some(main) = intent.input.file_name() else {
            eprintln!(
                "error: input path must have a file name: {}",
                intent.input.display()
            );
            return ExitCode::from(2);
        };
        let world = match SystemWorld::new(root, main) {
            Ok(world) => world
                .with_fonts_and_system(&[])
                .with_custom_ca(intent.cert_path.clone()),
            Err(SystemWorldError::MainNotFound(_)) => {
                eprintln!(
                    "error: input file not found (searched at {})",
                    intent.input.display()
                );
                return ExitCode::from(1);
            }
            Err(error) => {
                eprintln!("error: {error}");
                return ExitCode::from(2);
            }
        };
        let source = match world.source(world.main()) {
            Ok(source) => source,
            Err(error) => {
                eprintln!("error: failed to load source: {error:?}");
                return ExitCode::from(2);
            }
        };
        (world, source)
    };
    let (result, warnings) = typst_infra::query_helpers::query_compiled_elements(
        &world,
        &source,
        &intent.selector,
    );
    let elements = match result {
        Ok(elements) => elements,
        Err(CompiledQueryError::Compilation(errors)) => {
            drain_to_stderr(&world, &errors, &intent.input, intent.colored);
            drain_to_stderr(&world, &warnings, &intent.input, intent.colored);
            eprintln!(
                "warning: the `typst query` subcommand is deprecated\n = hint: {}\n",
                cli::query_deprecation_hint(&intent)
            );
            return ExitCode::from(1);
        }
        Err(CompiledQueryError::Retrieval(message)) => {
            eprintln!("error: {message}");
            return ExitCode::from(1);
        }
    };
    let bytes = match cli::serialize_query_with_format(
        &elements,
        intent.field.as_deref(),
        intent.one,
        intent.pretty,
        intent.format,
    ) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(1);
        }
    };
    if let Err(error) = std::io::stdout().lock().write_all(&bytes) {
        eprintln!("error: failed to write query output: {error}");
        return ExitCode::from(2);
    }
    drain_to_stderr(&world, &warnings, &intent.input, intent.colored);
    eprintln!(
        "warning: the `typst query` subcommand is deprecated\n = hint: {}\n",
        cli::query_deprecation_hint(&intent)
    );
    ExitCode::SUCCESS
}
