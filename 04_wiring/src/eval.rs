//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/eval.md
//! @prompt-hash af10cc21
//! @layer L4
//! @updated 2026-10-06

use super::diagnostics::drain_to_stderr_with_primary;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use typst_core::contracts::world::World;
use typst_infra::world::SystemWorld;
use typst_shell::cli::{self, EvalIntent};

pub(super) fn run_eval(intent: EvalIntent) -> ExitCode {
    let (world, contextual_source) = match intent.input.as_deref() {
        None => (
            SystemWorld::for_eval(PathBuf::from("."))
                .with_fonts_and_system(&[])
                .with_custom_ca(intent.cert_path.clone()),
            None,
        ),
        Some(path) if path == Path::new("-") => {
            let mut text = String::new();
            if let Err(error) = std::io::stdin().lock().read_to_string(&mut text) {
                eprintln!("error: failed to read source from stdin: {error}");
                return ExitCode::from(2);
            }
            let root = match std::env::current_dir() {
                Ok(root) => root,
                Err(error) => {
                    eprintln!("error: failed to acquire current directory: {error}");
                    return ExitCode::from(2);
                }
            };
            let world = SystemWorld::for_stdin(root, text)
                .with_fonts_and_system(&[])
                .with_custom_ca(intent.cert_path.clone());
            let source = world.source(world.main()).expect("preloaded stdin Source");
            (world, Some(source))
        }
        Some(path) => {
            let root = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let Some(main) = path.file_name() else {
                eprintln!("error: input path must have a file name: {}", path.display());
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
            (world, Some(source))
        }
    };
    let eval_source = match world.source_for_expression(intent.expression) {
        Ok(source) => source,
        Err(error) => {
            eprintln!("error: failed to acquire expression source: {error}");
            return ExitCode::from(2);
        }
    };
    let (result, warnings) = match contextual_source.as_ref() {
        Some(source) => typst_infra::pipeline::eval_source_with_document_features(
            &world,
            source,
            &eval_source,
            intent.features,
        ),
        None => typst_infra::pipeline::eval_source_with_sink_features(
            &world,
            &eval_source,
            intent.features,
        ),
    };
    if let Err(errors) = &result {
        drain_to_stderr_with_primary(
            &world,
            errors,
            Path::new("<input-expression>"),
            intent.colored,
            Some(&eval_source),
        );
    }
    drain_to_stderr_with_primary(
        &world,
        &warnings,
        Path::new("<input-expression>"),
        intent.colored,
        Some(&eval_source),
    );
    let value = match result {
        Ok(value) => value,
        Err(_) => return ExitCode::from(1),
    };
    let bytes = match cli::serialize_eval(&value, intent.format, intent.pretty) {
        Ok(bytes) => bytes,
        Err(error) => {
            eprintln!("error: {error}");
            return ExitCode::from(1);
        }
    };
    if let Err(error) = std::io::stdout().lock().write_all(&bytes) {
        eprintln!("error: failed to write eval output: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}
