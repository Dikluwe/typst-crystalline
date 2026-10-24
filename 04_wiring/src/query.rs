//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/query.md
//! @prompt-hash 52a287c0
//! @layer L4
//! @updated 2026-10-06

use super::diagnostics::prepare;
use super::diagnostics::write_stderr;
use std::io::{Read, Write};
use std::path::Path;
use std::process::ExitCode;
use typst_core::contracts::world::World;
use typst_infra::query_helpers::CompiledQueryError;
use typst_infra::world::{SystemWorld, SystemWorldError};
use typst_shell::cli::{self, QueryIntent};

pub(super) fn run_query(intent: QueryIntent) -> std::io::Result<ExitCode> {
    let (world, source) = if intent.input == Path::new("-") {
        let mut text = String::new();
        if let Err(error) = std::io::stdin().lock().read_to_string(&mut text) {
            write_stderr(&format!("error: failed to read source from stdin: {error}\n"))?;
            return Ok(ExitCode::from(2));
        }
        let root = match std::env::current_dir() {
            Ok(root) => root,
            Err(error) => {
                write_stderr(&format!(
                    "error: failed to determine current directory: {error}\n"
                ))?;
                return Ok(ExitCode::from(2));
            }
        };
        let world = SystemWorld::for_stdin(root, text)
            .with_fonts_and_system(&[])
            .with_custom_ca(intent.cert_path.clone());
        let source = match world.source(world.main()) {
            Ok(source) => source,
            Err(error) => {
                write_stderr(&format!("error: failed to load source: {error:?}\n"))?;
                return Ok(ExitCode::from(2));
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
            write_stderr(&format!(
                "error: input path must have a file name: {}\n",
                intent.input.display()
            ))?;
            return Ok(ExitCode::from(2));
        };
        let world = match SystemWorld::new(root, main) {
            Ok(world) => world
                .with_fonts_and_system(&[])
                .with_custom_ca(intent.cert_path.clone()),
            Err(SystemWorldError::MainNotFound(_)) => {
                write_stderr(&format!(
                    "error: input file not found (searched at {})\n",
                    intent.input.display()
                ))?;
                return Ok(ExitCode::from(1));
            }
            Err(error) => {
                write_stderr(&format!("error: {error}\n"))?;
                return Ok(ExitCode::from(2));
            }
        };
        let source = match world.source(world.main()) {
            Ok(source) => source,
            Err(error) => {
                write_stderr(&format!("error: failed to load source: {error:?}\n"))?;
                return Ok(ExitCode::from(2));
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
            let (error_text, mut metadata_errors) =
                prepare(&world, &errors, &intent.input, intent.colored);
            let (warning_text, warning_errors) =
                prepare(&world, &warnings, &intent.input, intent.colored);
            metadata_errors.extend(warning_errors);
            write_stderr(&format!("{error_text}{warning_text}"))?;
            write_stderr(&format!(
                "warning: the `typst query` subcommand is deprecated\n = hint: {}\n\n",
                cli::query_deprecation_hint(&intent)
            ))?;
            if !metadata_errors.is_empty() {
                for error in metadata_errors {
                    write_stderr(&format!(
                        "error: failed to acquire diagnostic metadata: {error}\n"
                    ))?;
                }
                return Ok(ExitCode::from(2));
            }
            return Ok(ExitCode::from(1));
        }
        Err(CompiledQueryError::Retrieval(message)) => {
            write_stderr(&format!("error: {message}\n"))?;
            return Ok(ExitCode::from(1));
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
            write_stderr(&format!("error: {error}\n"))?;
            return Ok(ExitCode::from(1));
        }
    };
    let (warning_text, metadata_errors) =
        prepare(&world, &warnings, &intent.input, intent.colored);
    if !metadata_errors.is_empty() {
        write_stderr(&warning_text)?;
        write_stderr(&format!(
            "warning: the `typst query` subcommand is deprecated\n = hint: {}\n\n",
            cli::query_deprecation_hint(&intent)
        ))?;
        for error in metadata_errors {
            write_stderr(&format!(
                "error: failed to acquire diagnostic metadata: {error}\n"
            ))?;
        }
        return Ok(ExitCode::from(2));
    }
    if let Err(error) = std::io::stdout().lock().write_all(&bytes) {
        write_stderr(&format!("error: failed to write query output: {error}\n"))?;
        return Ok(ExitCode::from(2));
    }
    write_stderr(&warning_text)?;
    write_stderr(&format!(
        "warning: the `typst query` subcommand is deprecated\n = hint: {}\n\n",
        cli::query_deprecation_hint(&intent)
    ))?;
    Ok(ExitCode::SUCCESS)
}
