//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/compile.md
//! @prompt-hash 4ee89f4b
//! @layer L4
//! @updated 2026-10-08

use super::{diagnostics::drain_to_stderr, Published};
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::ExitCode;
use typst_core::compiler::eval::pdf_standards_conflict_diagnostic;
use typst_core::contracts::world::World;
use typst_core::entities::document_format_options::{
    PdfAccessibility, PdfArchival, PdfStandard, PdfStandards,
};
use typst_core::entities::source_result::SourceDiagnostic;
use typst_infra::pipeline::{
    compile_to_bundle_files,
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override,
    compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override,
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override,
    BundleCompileOptions, BundleFile,
};
use typst_infra::world::SystemWorld;
use typst_shell::cli::{CompileIntent, OutputFormat};

const BUNDLE_EXPERIMENTAL_WARNING: &str = concat!(
    "warning: bundle export is experimental\n",
    " = hint: its behaviour may change at any time\n",
    " = hint: do not rely on this feature for production use cases\n\n",
);

const HTML_EXPERIMENTAL_WARNING: &str = concat!(
    "warning: html export is under active development and incomplete\n",
    " = hint: its behaviour may change at any time\n",
    " = hint: do not rely on this feature for production use cases\n",
    " = hint: see https://github.com/typst/typst/issues/5512 for more information\n\n",
);

fn report_in_order(status: impl FnOnce(), diagnostics: impl FnOnce()) {
    status();
    diagnostics();
}

fn write_bundle_output(destination: &Path, files: &[BundleFile]) -> std::io::Result<()> {
    std::fs::create_dir_all(destination)?;
    let root = std::fs::canonicalize(destination)?;
    for file in files {
        let relative = Path::new(&file.path);
        if !relative
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid bundle file path",
            ));
        }
        let path = destination.join(relative);
        let parent = path.parent().ok_or_else(|| {
            std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "invalid bundle file parent",
            )
        })?;
        std::fs::create_dir_all(parent)?;
        if !std::fs::canonicalize(parent)?.starts_with(&root)
            || std::fs::symlink_metadata(&path)
                .is_ok_and(|metadata| metadata.file_type().is_symlink())
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "bundle file would escape output directory",
            ));
        }
        std::fs::write(path, &file.data)?;
    }
    Ok(())
}

pub(super) fn run_compile(intent: CompileIntent) -> ExitCode {
    let mut report_status = |_, _| {};
    run_compile_observed(intent, &mut report_status).0
}

pub(super) fn run_compile_observed(
    intent: CompileIntent,
    report_status: &mut impl FnMut(ExitCode, bool),
) -> (ExitCode, Vec<PathBuf>, bool, Option<Published>) {
    run_compile_observed_with_sessions(intent, report_status, None, None)
}

pub(super) fn run_compile_observed_with_sessions(
    intent: CompileIntent,
    report_status: &mut impl FnMut(ExitCode, bool),
    font_session: Option<std::sync::Arc<typst_infra::world::FontCatalogSession>>,
    source_session: Option<std::sync::Arc<typst_infra::world::SourceSession>>,
) -> (ExitCode, Vec<PathBuf>, bool, Option<Published>) {
    // P428 (DEBT-59): `full_error` é fiado de RunIntent até L1 pelo caminho
    // interno de L3. O campo mantém default `false` quando a flag não é usada.
    let CompileIntent {
        features,
        input,
        output,
        output_format,
        html_serialization,
        root,
        font_paths,
        colored,
        full_error,
        timings_json,
        document_id,
        inputs,
        compact,
        pretty,
        ppi,
        pdf_tagged,
        no_pdf_tags,
        oracle_pdf,
        cert_path,
        pages,
        pdf_standard,
    } = intent;

    if output_format == OutputFormat::Bundle {
        if output == Path::new("-") {
            eprintln!("error: cannot write bundle to standard output");
            return (ExitCode::from(2), vec![input], false, None);
        }
        if !features.contains(typst_core::entities::compiler_features::Feature::Bundle) {
            eprintln!("error: bundle export is only available when `--features bundle` is passed\n = hint: bundle export is experimental");
            return (ExitCode::from(1), vec![input], false, None);
        }
    }

    // **P1468** — gate CLI-only do braço PDF: `--pages` + `--pdf-tagged=true`
    // explícito é erro antes de compilar (não envolve `PagedDocument` nem
    // pipeline — checagem puramente entre duas flags já parseadas,
    // `wiring.md` P1468). Verificado aqui, antes de `SystemWorld`/`Source`
    // (construídos mais abaixo para todos os formatos), para não tentar I/O
    // de compilação — mesmo comportamento medido no vanilla
    // (`typst-cli/src/compile.rs:162-165`). Neutro para PNG/SVG/HTML.
    // Distinta do erro documental de `infra/pipeline.md` P1468, que só
    // dispara quando é o documento a pedir `tagged: true`.
    if matches!(output_format, OutputFormat::Pdf)
        && pages.is_some()
        && pdf_tagged == Some(true)
    {
        report_in_order(
            || report_status(ExitCode::from(1), false),
            || eprintln!("error: cannot enable PDF tags when exporting a page range"),
        );
        return (ExitCode::from(1), vec![input], false, None);
    }

    // **P1469** (escopo reduzido — contrato de linguagem) — gate CLI-only
    // do braço PDF: `--pdf-standard` com um dos quatro perfis acessíveis
    // (`a-1a`/`a-2a`/`a-3a`/`ua-1`) combinado com `tagged` resolvido
    // `false` só pela CLI (explícito, legado `--no-pdf-tags`, ou colapsado
    // por `--pages` sem decisão explícita) é erro antes de compilar
    // (`wiring.md` P1469, análogo ao gate `pages`+`tagged` de P1468).
    // Distinta da checagem documental completa de `infra/pipeline.md`
    // P1469, que só dispara depois de CLI+documento combinados.
    let standard_override = if matches!(output_format, OutputFormat::Pdf) {
        let tagged_resolved_cli_only: Option<bool> = match pdf_tagged {
            Some(explicit) => Some(explicit),
            None if no_pdf_tags => Some(false),
            None if pages.is_some() => Some(false),
            None => None,
        };

        if tagged_resolved_cli_only == Some(false) {
            #[expect(
                clippy::type_complexity,
                reason = "ACCESSIBLE: composição estática de standards e labels existentes"
            )]
            const ACCESSIBLE: [(PdfStandard, fn() -> &'static str); 4] = [
                (PdfStandard::A1a, || PdfArchival::A1a.label()),
                (PdfStandard::A2a, || PdfArchival::A2a.label()),
                (PdfStandard::A3a, || PdfArchival::A3a.label()),
                (PdfStandard::Ua1, || PdfAccessibility::Ua1.label()),
            ];
            let accessible_name = ACCESSIBLE.iter().find_map(|(standard, label)| {
                pdf_standard.contains(standard).then(label)
            });
            if let Some(name) = accessible_name {
                // Hint só quando a causa raiz do `false` foi o colapso por
                // `--pages` (sem decisão explícita de `--pdf-tagged`/
                // `--no-pdf-tags`) — mesma regra medida em `shell/cli.md`
                // P1469.
                let with_hint = !no_pdf_tags && pages.is_some();
                report_in_order(
                    || report_status(ExitCode::from(1), false),
                    || {
                        if with_hint {
                            eprintln!(
                                "error: cannot disable PDF tags when exporting a {name} document\n = hint: using --pages implies --pdf-tagged=false"
                            );
                        } else {
                            eprintln!(
                                "error: cannot disable PDF tags when exporting a {name} document"
                            );
                        }
                    },
                );
                return (ExitCode::from(1), vec![input], false, None);
            }
        }

        // Gate não disparou — validar conflitos entre os nomes da própria
        // lista `--pdf-standard` (mesma função de `compiler/eval/rules.rs`
        // P1469, reaproveitada para o caso CLI). Lista vazia produz `None`
        // (delega ao documento/default).
        if pdf_standard.is_empty() {
            None
        } else {
            match PdfStandards::new(pdf_standard.iter().copied()) {
                Ok(resolved) => Some(resolved),
                Err(conflict) => {
                    let diag = pdf_standards_conflict_diagnostic(conflict);
                    report_in_order(
                        || report_status(ExitCode::from(1), false),
                        || {
                            let mut message = format!("error: {}", diag.message);
                            for hint in &diag.hints {
                                message.push_str(&format!("\n = hint: {hint}"));
                            }
                            eprintln!("{message}");
                        },
                    );
                    return (ExitCode::from(1), vec![input], false, None);
                }
            }
        }
    } else {
        None
    };

    let from_stdin = input == Path::new("-");
    let world = if from_stdin {
        let stdin_root = match root.canonicalize() {
            Ok(root) => root,
            Err(error) => {
                report_in_order(
                    || report_status(ExitCode::from(1), false),
                    || {
                        if error.kind() == std::io::ErrorKind::NotFound {
                            eprintln!(
                                "error: root directory not found (searched at {})",
                                root.display()
                            );
                        } else {
                            eprintln!("error: {error}");
                        }
                    },
                );
                return (ExitCode::from(1), vec![], false, None);
            }
        };
        let mut bytes = Vec::new();
        if let Err(error) = std::io::stdin().lock().read_to_end(&mut bytes) {
            if error.kind() != std::io::ErrorKind::BrokenPipe {
                report_in_order(
                    || report_status(ExitCode::from(2), false),
                    || eprintln!("error: failed to read source from stdin: {error}"),
                );
                return (ExitCode::from(2), vec![], false, None);
            }
        }
        let text = match String::from_utf8(bytes) {
            Ok(text) => text,
            Err(_) => {
                report_in_order(
                    || report_status(ExitCode::from(1), false),
                    || {
                        eprintln!("error: file is not valid UTF-8\n = hint: a file without an extension is not usually a Typst file\n")
                    },
                );
                return (ExitCode::from(1), vec![], false, None);
            }
        };
        SystemWorld::for_stdin(stdin_root, text)
    } else {
        let main_path = match input.file_name() {
            Some(name) => PathBuf::from(name),
            None => {
                report_in_order(
                    || report_status(ExitCode::from(2), false),
                    || {
                        eprintln!(
                            "error: input path must have a file name: {}",
                            input.display()
                        )
                    },
                );
                return (ExitCode::from(2), vec![input], false, None);
            }
        };
        let created = match source_session {
            Some(session) => {
                SystemWorld::new_with_source_session(&root, &main_path, session)
            }
            None => SystemWorld::new(&root, &main_path),
        };
        match created {
            Ok(world) => world,
            Err(error) => {
                report_in_order(
                    || report_status(ExitCode::from(2), false),
                    || eprintln!("error: {error}"),
                );
                return (ExitCode::from(2), vec![input], false, None);
            }
        }
    };
    let world = match font_session {
        Some(session) => world.with_font_catalog_session(session, &font_paths),
        None => world.with_fonts_and_system(&font_paths),
    }
    .with_inputs(inputs)
    .with_custom_ca(cert_path)
    .with_plugin_host(std::sync::Arc::new(
        typst_infra::plugin_host::WasmiPluginHost::new(),
    ));

    let source = match world.source(world.main()) {
        Ok(s) => s,
        Err(e) => {
            report_in_order(
                || report_status(ExitCode::from(2), false),
                || eprintln!("error: failed to load source: {:?}", e),
            );
            return (ExitCode::from(2), world.dependencies(), false, None);
        }
    };

    // P870 — dispatch por formato de saída: PDF, PNG (primeira página) ou SVG.
    // **P956** — a flag `--compact` (bool cru de L2) é traduzida aqui para
    // `StreamMode` (L3) e passada apenas às chamadas PDF; PNG/SVG não têm
    // content stream PDF e não recebem modo.
    let stream_mode = if compact {
        typst_infra::export::StreamMode::Compact
    } else {
        typst_infra::export::StreamMode::Verbose
    };
    let pretty_override = pretty;
    // **P1467** — preserva a ausência de decisão CLI (`None`) até a pipeline
    // consultar `#set format.pdf(tagged: ...)` no documento (Núcleo
    // `format-option-precedence`); só colapsa a precedência **entre as duas
    // flags de CLI** (`--pdf-tagged` vence o legado `--no-pdf-tags`).
    let pdf_tagged_override: Option<bool> = match pdf_tagged {
        Some(explicit) => Some(explicit),
        None if no_pdf_tags => Some(false),
        None => None,
    };
    // **P980** — a flag `--oracle-pdf` (bool cru de L2) selecciona a
    // entrada do oráculo de paridade de operador (só afecta PDF).
    let oracle_pdf = oracle_pdf && matches!(output_format, OutputFormat::Pdf);
    let emit_html_experimental_warning = matches!(&output_format, OutputFormat::Html)
        && features.contains(typst_core::entities::compiler_features::Feature::Html);
    let mut bundle_files: Option<Vec<BundleFile>> = None;
    let mut published: Option<Published> = None;
    let (result, warnings, timings): (
        Result<Vec<u8>, Vec<SourceDiagnostic>>,
        Vec<SourceDiagnostic>,
        typst_infra::pipeline::Timings,
    ) = match output_format {
        OutputFormat::Bundle => {
            let html_mode = match html_serialization {
                typst_shell::cli::HtmlSerialization::Crystalline => {
                    typst_infra::export::HtmlSerializationMode::Crystalline
                }
                typst_shell::cli::HtmlSerialization::Vanilla => {
                    typst_infra::export::HtmlSerializationMode::Vanilla
                }
            };
            let (result, warnings, timings) = compile_to_bundle_files(
                &world,
                &source,
                BundleCompileOptions {
                    full_error,
                    features,
                    document_id,
                    pdf_stream_mode: stream_mode,
                    pdf_serialization: typst_infra::export::PdfSerializationMode::Compact,
                    pdf_tagged_override,
                    pdf_pages_override: pages.clone(),
                    pdf_standard_override: standard_override,
                    png_ppi_override: ppi,
                    html_serialization: html_mode,
                    pretty_override,
                },
            );
            match result {
                Ok(files) => {
                    bundle_files = Some(files);
                    (Ok(Vec::new()), warnings, timings)
                }
                Err(errors) => (Err(errors), warnings, timings),
            }
        }
        OutputFormat::Pdf => {
            if oracle_pdf {
                let (r, w) =
                    typst_infra::pipeline::compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tagged_override,
                        features,
                        pretty_override,
                        pages.clone(),
                        standard_override,
                    );
                (r, w, typst_infra::pipeline::Timings::default())
            } else if timings_json.is_some() {
                let (r, w, t) =
                    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tagged_override,
                        features,
                        pretty_override,
                        pages.clone(),
                        standard_override,
                    );
                (r, w, t)
            } else {
                let (r, w) =
                    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_and_standard_override(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tagged_override,
                        features,
                        pretty_override,
                        pages.clone(),
                        standard_override,
                    );
                (r, w, typst_infra::pipeline::Timings::default())
            }
        }
        OutputFormat::Png => {
            if timings_json.is_some() {
                let (r, w, t) =
                    compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
                        &world, &source, full_error, features, ppi,
                    );
                (r, w, t)
            } else {
                let (r, w, _) =
                    compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override(
                        &world, &source, full_error, features, ppi,
                    );
                (r, w, typst_infra::pipeline::Timings::default())
            }
        }
        OutputFormat::Svg => {
            if timings_json.is_some() {
                let (r, w, t) =
                    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
                        &world,
                        &source,
                        full_error,
                        features,
                        pretty_override,
                    );
                (r.map(|s| s.into_bytes()), w, t)
            } else {
                let (r, w, _) =
                    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override(
                        &world,
                        &source,
                        full_error,
                        features,
                        pretty_override,
                    );
                (r.map(|s| s.into_bytes()), w, typst_infra::pipeline::Timings::default())
            }
        }
        OutputFormat::Html => {
            let html_serialization = match html_serialization {
                typst_shell::cli::HtmlSerialization::Crystalline => {
                    typst_infra::export::HtmlSerializationMode::Crystalline
                }
                typst_shell::cli::HtmlSerialization::Vanilla => {
                    typst_infra::export::HtmlSerializationMode::Vanilla
                }
            };
            let (r, w) =
                typst_infra::pipeline::compile_to_html_string_with_features_and_serialization_and_pretty_override(
                    &world,
                    &source,
                    features,
                    html_serialization,
                    pretty_override,
                );
            (r.map(String::into_bytes), w, typst_infra::pipeline::Timings::default())
        }
    };
    let has_warnings = !warnings.is_empty();
    let dependencies = world.dependencies();
    let mut errors = None;
    let mut operational_error = None;

    let exit_code = match result {
        Ok(output_bytes) => {
            let write_result = if let Some(files) = bundle_files.as_ref() {
                write_bundle_output(&output, files)
            } else if output == Path::new("-") {
                std::io::stdout().lock().write_all(&output_bytes)
            } else {
                std::fs::write(&output, &output_bytes)
            };
            if let Err(e) = write_result {
                operational_error =
                    Some(format!("error: failed to write {}: {}", output.display(), e));
            } else if let Some(path) = timings_json.as_ref() {
                if let Err(e) = std::fs::write(path, timings.to_json()) {
                    operational_error = Some(format!(
                        "error: failed to write timings {}: {}",
                        path.display(),
                        e
                    ));
                }
            }
            if operational_error.is_some() {
                ExitCode::from(2)
            } else {
                if let Some(files) = bundle_files.take() {
                    published = Some(Published::Bundle(files));
                } else if output_format == OutputFormat::Html {
                    published = Some(Published::Html(
                        String::from_utf8_lossy(&output_bytes).into_owned(),
                    ));
                }
                ExitCode::SUCCESS
            }
        }
        Err(compilation_errors) => {
            errors = Some(compilation_errors);
            if let Some(path) = timings_json.as_ref() {
                if let Err(e) = std::fs::write(path, timings.to_json()) {
                    operational_error = Some(format!(
                        "error: failed to write timings {}: {}",
                        path.display(),
                        e
                    ));
                }
            }
            if operational_error.is_some() {
                ExitCode::from(2)
            } else {
                ExitCode::from(1)
            }
        }
    };

    report_in_order(
        || report_status(exit_code, has_warnings),
        || {
            if let Some(errors) = errors.as_deref() {
                drain_to_stderr(&world, errors, &input, colored);
            }
            if emit_html_experimental_warning {
                eprint!("{HTML_EXPERIMENTAL_WARNING}");
            }
            if output_format == OutputFormat::Bundle {
                eprint!("{BUNDLE_EXPERIMENTAL_WARNING}");
            }
            drain_to_stderr(&world, &warnings, &input, colored);
            if let Some(error) = operational_error {
                eprintln!("{error}");
            }
        },
    );

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
        eprintln!("[crystalline] introspector_call_counts: total={}", counts.total,);
        for (method, count) in &counts.per_method {
            if *count > 0 {
                eprintln!("[crystalline]   {}: {}", method, count);
            }
        }
    }

    (exit_code, dependencies, has_warnings, published)
}

#[cfg(test)]
mod tests {
    // Testes históricos P1323/P1366, conservados na atomização do wiring.
    #[test]
    fn p1323_html_experimental_warning_matches_ratified_envelope() {
        let expected = concat!(
            "warning: html export is under active development and incomplete\n",
            " = hint: its behaviour may change at any time\n",
            " = hint: do not rely on this feature for production use cases\n",
            " = hint: see https://github.com/typst/typst/issues/5512 for more information\n",
            "\n",
        );
        assert_eq!(super::HTML_EXPERIMENTAL_WARNING.as_bytes(), expected.as_bytes());
    }

    #[test]
    fn p1366_report_in_order_runs_status_before_diagnostics_once() {
        use std::cell::{Cell, RefCell};

        let order = RefCell::new(Vec::new());
        let status_calls = Cell::new(0);
        let diagnostics_calls = Cell::new(0);

        super::report_in_order(
            || {
                order.borrow_mut().push("status");
                status_calls.set(status_calls.get() + 1);
            },
            || {
                order.borrow_mut().push("diagnostics");
                diagnostics_calls.set(diagnostics_calls.get() + 1);
            },
        );

        assert_eq!(&*order.borrow(), &["status", "diagnostics"]);
        assert_eq!(status_calls.get(), 1);
        assert_eq!(diagnostics_calls.get(), 1);
    }
}
