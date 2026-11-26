//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/eval.md
//! @prompt-hash 50b60705
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
    let html = intent.target == typst_shell::cli::EvalTarget::Html;
    // ref: lab/typst-original/crates/typst-cli/src/eval.rs
    if html
        && !intent.features.contains(typst_core::entities::compiler_features::Feature::Html)
    {
        write_stderr(
            "error: html export is only available when `--features html` is passed\n = hint: html export is under active development and incomplete\n = hint: see https://github.com/typst/typst/issues/5512 for more information\n\n",
        )?;
        return Ok(ExitCode::from(1));
    }
    if html && intent.input.is_some() {
        write_stderr("error: eval with `--in` supports only the paged target\n")?;
        return Ok(ExitCode::from(2));
    }
    if let Some(timestamp) = intent.creation_timestamp {
        std::env::set_var("SOURCE_DATE_EPOCH", timestamp.to_string());
    }
    let short = intent.diagnostic_format == typst_shell::cli::DiagnosticFormat::Short;
    let configure = |world: SystemWorld| {
        world
            .with_font_selection(
                &intent.font_paths,
                intent.include_system_fonts,
                intent.include_embedded_fonts,
            )
            .with_inputs(intent.inputs.clone())
            .with_custom_ca(intent.cert_path.clone())
    };
    let (world, contextual_source) = match intent.input.as_deref() {
        None => (
            configure(SystemWorld::for_eval(
                intent.root.clone().unwrap_or_else(|| PathBuf::from(".")),
            )),
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
            let root = match intent.root.clone().map_or_else(std::env::current_dir, Ok) {
                Ok(root) => root,
                Err(error) => {
                    write_stderr(&format!(
                        "error: failed to acquire current directory: {error}\n"
                    ))?;
                    return Ok(ExitCode::from(2));
                }
            };
            let world = configure(SystemWorld::for_stdin(root, text));
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
            let parent = path
                .parent()
                .filter(|parent| !parent.as_os_str().is_empty())
                .unwrap_or(Path::new("."));
            let Some(name) = path.file_name() else {
                write_stderr(&format!(
                    "error: input path must have a file name: {}\n",
                    path.display()
                ))?;
                return Ok(ExitCode::from(2));
            };
            // With an explicit root the input keeps its own path inside it.
            let (root, main) = match intent.root.clone() {
                Some(root) => (root, path.to_path_buf()),
                None => (parent.to_path_buf(), PathBuf::from(name)),
            };
            let main = if intent.root.is_some() && main.is_relative() {
                match std::env::current_dir() {
                    Ok(cwd) => cwd.join(main),
                    Err(_) => main,
                }
            } else {
                main
            };
            let world = match SystemWorld::new(&root, &main) {
                Ok(world) => configure(world),
                Err(error) => {
                    let (code, message) = match &error {
                        typst_infra::world::SystemWorldError::MainNotFound(_) => (
                            1,
                            format!("input file not found (searched at {})", path.display()),
                        ),
                        typst_infra::world::SystemWorldError::RootNotFound(_)
                        | typst_infra::world::SystemWorldError::OutsideRoot => {
                            (1, error.to_string())
                        }
                        typst_infra::world::SystemWorldError::Io(_) => (2, error.to_string()),
                    };
                    write_stderr(&format!("error: {message}\n"))?;
                    return Ok(ExitCode::from(code));
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
        None => typst_infra::pipeline::eval_source_with_sink_target_features(
            &world,
            &eval_source,
            intent.features,
            html,
        ),
    };
    let (error_text, mut metadata_errors) = match &result {
        Err(errors) => prepare_with_primary(
            &world,
            errors,
            Path::new("<input-expression>"),
            intent.colored,
            short,
            Some(&eval_source),
        ),
        Ok(_) => (String::new(), Vec::new()),
    };
    let (warning_text, warning_errors) = prepare_with_primary(
        &world,
        &warnings,
        Path::new("<input-expression>"),
        intent.colored,
        short,
        Some(&eval_source),
    );
    metadata_errors.extend(warning_errors);
    write_stderr(&format!("{error_text}{warning_text}"))?;
    if html {
        write_stderr(super::compile::HTML_EXPERIMENTAL_WARNING)?;
    }
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
