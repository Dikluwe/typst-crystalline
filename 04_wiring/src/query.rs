//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/query.md
//! @prompt-hash ee6113c3
//! @layer L4
//! @updated 2026-09-26

use super::diagnostics::drain_to_stderr;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use typst_core::contracts::world::World;
use typst_core::entities::source::Source;
use typst_infra::world::SystemWorld;
use typst_shell::cli::{self, QueryIntent};

pub(super) fn run_query(intent: QueryIntent) -> ExitCode {
    eprintln!("warning: the `typst query` subcommand is deprecated\n = hint: use `typst eval 'query(...)' --in ...` instead\n");
    let (world, source) = if intent.input == Path::new("-") {
        let mut text = String::new();
        if let Err(error) = std::io::stdin().lock().read_to_string(&mut text) {
            eprintln!("error: failed to read source from stdin: {error}");
            return ExitCode::from(2);
        }
        let world = SystemWorld::for_eval(PathBuf::from("."))
            .with_fonts_and_system(&[])
            .with_custom_ca(intent.cert_path.clone());
        let source = Source::new(world.main(), text);
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
    let (result, warnings) =
        typst_infra::query_helpers::query_elements(&world, &source, &intent.selector);
    drain_to_stderr(&world, &warnings, &intent.input, intent.colored);
    let elements = match result {
        Ok(elements) => elements,
        Err(errors) => {
            drain_to_stderr(&world, &errors, &intent.input, intent.colored);
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
    ExitCode::SUCCESS
}
