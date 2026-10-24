//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/eval.md
//! @prompt-hash 301d58c5
//! @layer L4
//! @updated 2026-10-06

use super::diagnostics::prepare_with_primary;
use super::diagnostics::write_stderr;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use typst_core::contracts::world::World;
use typst_infra::world::SystemWorld;
use typst_shell::cli::{self, EvalIntent};

pub(super) fn run_eval(intent: EvalIntent) -> std::io::Result<ExitCode> {
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
                write_stderr(&format!(
                    "error: failed to read source from stdin: {error}\n"
                ))?;
                return Ok(ExitCode::from(2));
            }
            let root = match std::env::current_dir() {
                Ok(root) => root,
                Err(error) => {
                    write_stderr(&format!(
                        "error: failed to acquire current directory: {error}\n"
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
            (world, Some(source))
        }
        Some(path) => {
            let root = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let Some(main) = path.file_name() else {
                write_stderr(&format!(
                    "error: input path must have a file name: {}\n",
                    path.display()
                ))?;
                return Ok(ExitCode::from(2));
            };
            let world = match SystemWorld::new(root, main) {
                Ok(world) => world
                    .with_fonts_and_system(&[])
                    .with_custom_ca(intent.cert_path.clone()),
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
            (world, Some(source))
        }
    };
    let eval_source = match world.source_for_expression(intent.expression) {
        Ok(source) => source,
        Err(error) => {
            write_stderr(&format!(
                "error: failed to acquire expression source: {error}\n"
            ))?;
            return Ok(ExitCode::from(2));
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
    let (error_text, mut metadata_errors) = match &result {
        Err(errors) => prepare_with_primary(
            &world,
            errors,
            Path::new("<input-expression>"),
            intent.colored,
            Some(&eval_source),
        ),
        Ok(_) => (String::new(), Vec::new()),
    };
    let (warning_text, warning_errors) = prepare_with_primary(
        &world,
        &warnings,
        Path::new("<input-expression>"),
        intent.colored,
        Some(&eval_source),
    );
    metadata_errors.extend(warning_errors);
    write_stderr(&format!("{error_text}{warning_text}"))?;
    if !metadata_errors.is_empty() {
        for error in metadata_errors {
            write_stderr(&format!(
                "error: failed to acquire diagnostic metadata: {error}\n"
            ))?;
        }
        return Ok(ExitCode::from(2));
    }
    let value = match result {
        Ok(value) => value,
        Err(_) => return Ok(ExitCode::from(1)),
    };
    let bytes = match cli::serialize_eval(&value, intent.format, intent.pretty) {
        Ok(bytes) => bytes,
        Err(error) => {
            write_stderr(&format!("error: {error}\n"))?;
            return Ok(ExitCode::from(1));
        }
    };
    if let Err(error) = std::io::stdout().lock().write_all(&bytes) {
        write_stderr(&format!("error: failed to write eval output: {error}\n"))?;
        return Ok(ExitCode::from(2));
    }
    Ok(ExitCode::SUCCESS)
}
