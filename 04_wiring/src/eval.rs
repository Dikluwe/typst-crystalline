//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/eval.md
//! @prompt-hash 2d7e60a4
//! @layer L4
//! @updated 2026-09-26

use super::diagnostics::drain_to_stderr_with_primary;
use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;
use typst_core::contracts::world::World;
use typst_core::entities::source::Source;
use typst_infra::world::SystemWorld;
use typst_shell::cli::{self, EvalIntent};

pub(super) fn run_eval(intent: EvalIntent) -> ExitCode {
    let (world, contextual_source, input_path) = match intent.input.as_deref() {
        None => (
            SystemWorld::for_eval(PathBuf::from("."))
                .with_fonts_and_system(&[])
                .with_custom_ca(intent.cert_path.clone()),
            None,
            PathBuf::from("<input-expression>"),
        ),
        Some(path) if path == Path::new("-") => {
            let mut text = String::new();
            if let Err(error) = std::io::stdin().lock().read_to_string(&mut text) {
                eprintln!("error: failed to read source from stdin: {error}");
                return ExitCode::from(2);
            }
            let world = SystemWorld::for_eval(PathBuf::from("."))
                .with_fonts_and_system(&[])
                .with_custom_ca(intent.cert_path.clone());
            let source = Source::new(world.main(), text);
            (world, Some(source), PathBuf::from("-"))
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
            (world, Some(source), path.to_path_buf())
        }
    };
    let eval_source = Source::new_with_parser(
        world.main(),
        intent.expression.clone(),
        typst_core::compiler::parse::parse_code,
    );
    let (result, warnings) = match contextual_source.as_ref() {
        Some(source) => typst_infra::pipeline::eval_expression_with_document_features(
            &world,
            source,
            &intent.expression,
            intent.features,
        ),
        None => typst_infra::pipeline::eval_expression_with_sink_features(
            &world,
            &intent.expression,
            intent.features,
        ),
    };
    if let Err(errors) = &result {
        drain_to_stderr_with_primary(
            &world,
            errors,
            &input_path,
            intent.colored,
            Some(&eval_source),
        );
    }
    drain_to_stderr_with_primary(
        &world,
        &warnings,
        &input_path,
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
