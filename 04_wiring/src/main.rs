//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash 934a8298
//! @layer L4
//! @updated 2026-06-17
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
//!   L4 invoca `.with_fonts_and_system(...)` (P517).
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

use std::io::{Read, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use typst_core::contracts::world::World;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_infra::pipeline::{
    compile_to_pdf_bytes_full_error_and_document_id_with_features,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features,
    compile_to_png_bytes_with_timings_full_error_and_features,
    compile_to_svg_string_with_timings_full_error_and_features,
};
use typst_infra::world::SystemWorld;
use typst_shell::cli::{
    self, CompileIntent, CompletionsIntent, EvalIntent, FontsIntent, InfoFormat,
    InfoIntent, InitIntent, OutputFormat, QueryIntent, RunIntent, WatchIntent,
};
use typst_shell::diagnostic::{format_diagnostic, DiagnosticSource};

fn main() -> ExitCode {
    match cli::parse() {
        RunIntent::Compile(intent) => run_compile(intent),
        RunIntent::Watch(intent) => run_watch(intent),
        RunIntent::Eval(intent) => run_eval(intent),
        RunIntent::Query(intent) => run_query(intent),
        RunIntent::Fonts(intent) => run_fonts(intent),
        RunIntent::Completions(intent) => run_completions(intent),
        RunIntent::Info(intent) => run_info(intent),
        RunIntent::Init(intent) => run_init(intent),
    }
}

fn run_watch(intent: WatchIntent) -> ExitCode {
    if intent.compile.output == Path::new("-") {
        eprintln!("error: watch requires a file output; stdout is not supported");
        return ExitCode::from(2);
    }

    let destination = intent.compile.output.clone();
    let file_name = destination
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("output");
    let staging = destination.with_file_name(format!(
        ".{file_name}.crystalline-watch-{}.tmp",
        std::process::id()
    ));

    loop {
        let mut compile = intent.compile.clone();
        compile.output = staging.clone();
        let (exit_code, dependencies) = run_compile_observed(compile);
        if exit_code == ExitCode::SUCCESS {
            if let Err(error) = typst_infra::watch::commit_output(&staging, &destination)
            {
                typst_infra::watch::discard_output(&staging);
                eprintln!(
                    "error: failed to replace {} atomically: {}",
                    destination.display(),
                    error
                );
                return ExitCode::from(2);
            }
        } else {
            typst_infra::watch::discard_output(&staging);
        }

        let dependencies = if dependencies.is_empty() {
            vec![intent.compile.input.clone()]
        } else {
            dependencies
        };
        eviction::crystalline_evict(10);
        typst_infra::watch::wait_for_change(
            &dependencies,
            std::time::Duration::from_millis(100),
        );
    }
}

fn run_init(intent: InitIntent) -> ExitCode {
    match typst_infra::project_init::initialize(
        &intent.template,
        intent.directory,
        intent.cert_path,
    ) {
        Ok(result) => {
            println!("Successfully created project in {}", result.destination.display());
            println!("Entrypoint: {}", result.entrypoint.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::from(1)
        }
    }
}

fn run_info(intent: InfoIntent) -> ExitCode {
    use std::collections::BTreeMap;
    use typst_core::entities::version::PARITY_VERSION;
    use typst_shell::info::{
        BuildInfo, FeatureInfo, FontInfo, InfoData, PackageInfo, PlatformInfo,
    };

    let snapshot = typst_infra::runtime_info::snapshot();
    let info = InfoData {
        version: format!(
            "{}.{}.{}",
            PARITY_VERSION.0, PARITY_VERSION.1, PARITY_VERSION.2
        ),
        build: BuildInfo {
            commit: cli::build_commit().map(str::to_owned),
            platform: PlatformInfo {
                os: std::env::consts::OS.to_owned(),
                arch: std::env::consts::ARCH.to_owned(),
            },
        },
        features: FeatureInfo { html: false, a11y_extras: false, bundle: false },
        fonts: FontInfo {
            system: true,
            font_paths: snapshot
                .font_paths
                .iter()
                .map(|path| path.display().to_string())
                .collect(),
        },
        packages: PackageInfo {
            data_path: snapshot.package_data_path.map(|path| path.display().to_string()),
            cache_path: snapshot
                .package_cache_path
                .map(|path| path.display().to_string()),
            custom_ca_configured: intent.cert_path.is_some()
                || snapshot.custom_cert_configured,
        },
        env: snapshot.env.into_iter().collect::<BTreeMap<_, _>>(),
    };
    let output = match intent.format {
        Some(InfoFormat::Json) => {
            match typst_shell::info::format_json(&info, intent.pretty) {
                Ok(output) => output,
                Err(error) => {
                    eprintln!("error: failed to serialize info: {error}");
                    return ExitCode::from(2);
                }
            }
        }
        None => typst_shell::info::format_human(&info),
    };
    if let Err(error) = std::io::stdout().lock().write_all(&output) {
        eprintln!("error: failed to write info: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}

fn run_completions(intent: CompletionsIntent) -> ExitCode {
    let output = typst_shell::completions::generate(intent.shell);
    if let Err(error) = std::io::stdout().lock().write_all(&output) {
        eprintln!("error: failed to write completions: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}

fn run_fonts(intent: FontsIntent) -> ExitCode {
    let entries =
        typst_infra::fonts::inventory_fonts(&intent.font_paths, intent.include_system)
            .into_iter()
            .map(|entry| typst_shell::fonts::FontDisplayEntry {
                family: entry.family,
                path: entry.path,
                index: entry.index,
                style: entry.style,
                weight: entry.weight,
                stretch: entry.stretch,
                embedded: entry.embedded,
            })
            .collect::<Vec<_>>();
    let output = typst_shell::fonts::format_fonts(&entries, intent.variants);
    if let Err(error) = std::io::stdout().lock().write_all(output.as_bytes()) {
        eprintln!("error: failed to write font list: {error}");
        return ExitCode::from(2);
    }
    ExitCode::SUCCESS
}

fn run_compile(intent: CompileIntent) -> ExitCode {
    run_compile_observed(intent).0
}

fn run_compile_observed(intent: CompileIntent) -> (ExitCode, Vec<PathBuf>) {
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
        no_pdf_tags,
        oracle_pdf,
        cert_path,
    } = intent;

    let main_path = match input.file_name() {
        Some(name) => PathBuf::from(name),
        None => {
            eprintln!("error: input path must have a file name: {}", input.display());
            return (ExitCode::from(2), vec![input]);
        }
    };

    // P517 — fontes do sistema activas por defeito; `--font-path` adiciona
    // fontes de projecto às fontes do sistema.
    // P699 — instala o host de plugins WASM (L3) para que `plugin(...)` na
    // linguagem devolva um `Module` real (em vez de "plugins não suportados").
    let world = match SystemWorld::new(&root, &main_path) {
        Ok(w) => w
            .with_fonts_and_system(&font_paths)
            .with_inputs(inputs)
            .with_custom_ca(cert_path)
            .with_plugin_host(std::sync::Arc::new(
                typst_infra::plugin_host::WasmiPluginHost::new(),
            )),
        Err(e) => {
            eprintln!("error: {}", e);
            return (ExitCode::from(2), vec![input]);
        }
    };

    let source = match world.source(world.main()) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error: failed to load source: {:?}", e);
            return (ExitCode::from(2), world.dependencies());
        }
    };

    // P927/P938 — scan condicional do source bruto. Se encontrar carateres
    // fora da cobertura das fontes embutidas, dispara a extração lazy de
    // coverage das fontes do sistema antes do layout.
    world.preload_coverage_if_needed(&source);

    // P870 — dispatch por formato de saída: PDF, PNG (primeira página) ou SVG.
    // **P956** — a flag `--compact` (bool cru de L2) é traduzida aqui para
    // `StreamMode` (L3) e passada apenas às chamadas PDF; PNG/SVG não têm
    // content stream PDF e não recebem modo.
    let stream_mode = if compact {
        typst_infra::export::StreamMode::Compact
    } else {
        typst_infra::export::StreamMode::Verbose
    };
    let pdf_tags = if no_pdf_tags {
        typst_infra::export::PdfTags::Disabled
    } else {
        typst_infra::export::PdfTags::Enabled
    };
    // **P980** — a flag `--oracle-pdf` (bool cru de L2) selecciona a
    // entrada do oráculo de paridade de operador (só afecta PDF).
    let oracle_pdf = oracle_pdf && matches!(output_format, OutputFormat::Pdf);
    let (result, warnings, timings): (
        Result<Vec<u8>, Vec<SourceDiagnostic>>,
        Vec<SourceDiagnostic>,
        typst_infra::pipeline::Timings,
    ) = match output_format {
        OutputFormat::Pdf => {
            if oracle_pdf {
                let (r, w) =
                    typst_infra::pipeline::compile_to_pdf_bytes_oracle_with_features(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tags,
                        features,
                    );
                (r, w, typst_infra::pipeline::Timings::default())
            } else if timings_json.is_some() {
                let (r, w, t) =
                    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tags,
                        features,
                    );
                (r, w, t)
            } else {
                let (r, w) =
                    compile_to_pdf_bytes_full_error_and_document_id_with_features(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tags,
                        features,
                    );
                (r, w, typst_infra::pipeline::Timings::default())
            }
        }
        OutputFormat::Png => {
            if timings_json.is_some() {
                let (r, w, t) = compile_to_png_bytes_with_timings_full_error_and_features(
                    &world, &source, full_error, features,
                );
                (r, w, t)
            } else {
                let (r, w, _) = compile_to_png_bytes_with_timings_full_error_and_features(
                    &world, &source, full_error, features,
                );
                (r, w, typst_infra::pipeline::Timings::default())
            }
        }
        OutputFormat::Svg => {
            if timings_json.is_some() {
                let (r, w, t) =
                    compile_to_svg_string_with_timings_full_error_and_features(
                        &world, &source, full_error, features,
                    );
                (r.map(|s| s.into_bytes()), w, t)
            } else {
                let (r, w, _) =
                    compile_to_svg_string_with_timings_full_error_and_features(
                        &world, &source, full_error, features,
                    );
                (r.map(|s| s.into_bytes()), w, typst_infra::pipeline::Timings::default())
            }
        }
        OutputFormat::Html => {
            if features.contains(typst_core::entities::compiler_features::Feature::Html) {
                eprintln!(
                    "warning: html export is under active development and incomplete"
                );
            }
            let html_serialization = match html_serialization {
                typst_shell::cli::HtmlSerialization::Crystalline => {
                    typst_infra::export::HtmlSerializationMode::Crystalline
                }
                typst_shell::cli::HtmlSerialization::Vanilla => {
                    typst_infra::export::HtmlSerializationMode::Vanilla
                }
            };
            let (r, w) =
                typst_infra::pipeline::compile_to_html_string_with_features_and_serialization(
                    &world,
                    &source,
                    features,
                    html_serialization,
                );
            (r.map(String::into_bytes), w, typst_infra::pipeline::Timings::default())
        }
    };
    drain_to_stderr(&world, &warnings, &input, colored);
    let dependencies = world.dependencies();

    let exit_code = match result {
        Ok(output_bytes) => {
            if let Err(e) = std::fs::write(&output, &output_bytes) {
                eprintln!("error: failed to write {}: {}", output.display(), e);
                return (ExitCode::from(2), dependencies);
            }
            if let Some(path) = timings_json {
                if let Err(e) = std::fs::write(&path, timings.to_json()) {
                    eprintln!("error: failed to write timings {}: {}", path.display(), e);
                    return (ExitCode::from(2), dependencies);
                }
            }
            ExitCode::SUCCESS
        }
        Err(errors) => {
            drain_to_stderr(&world, &errors, &input, colored);
            if let Some(path) = timings_json {
                if let Err(e) = std::fs::write(&path, timings.to_json()) {
                    eprintln!("error: failed to write timings {}: {}", path.display(), e);
                    return (ExitCode::from(2), dependencies);
                }
            }
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
        eprintln!("[crystalline] introspector_call_counts: total={}", counts.total,);
        for (method, count) in &counts.per_method {
            if *count > 0 {
                eprintln!("[crystalline]   {}: {}", method, count);
            }
        }
    }

    (exit_code, dependencies)
}

fn run_eval(intent: EvalIntent) -> ExitCode {
    let root = PathBuf::from(".");
    let world = SystemWorld::for_eval(root)
        .with_fonts_and_system(&[])
        .with_custom_ca(intent.cert_path.clone());
    let eval_source = Source::new_with_parser(
        world.main(),
        intent.expression.clone(),
        typst_core::compiler::parse::parse_code,
    );
    let (result, warnings) = typst_infra::pipeline::eval_expression_with_sink_features(
        &world,
        &intent.expression,
        intent.features,
    );
    drain_to_stderr_with_primary(
        &world,
        &warnings,
        Path::new("<input-expression>"),
        intent.colored,
        Some(&eval_source),
    );
    let value = match result {
        Ok(value) => value,
        Err(errors) => {
            drain_to_stderr_with_primary(
                &world,
                &errors,
                Path::new("<input-expression>"),
                intent.colored,
                Some(&eval_source),
            );
            return ExitCode::from(1);
        }
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

fn run_query(intent: QueryIntent) -> ExitCode {
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

/// Drena diagnósticos depois de materializar todas as fontes referidas.
fn drain_to_stderr(
    world: &SystemWorld,
    diagnostics: &[SourceDiagnostic],
    input: &Path,
    colored: bool,
) {
    drain_to_stderr_with_primary(world, diagnostics, input, colored, None);
}

fn drain_to_stderr_with_primary(
    world: &SystemWorld,
    diagnostics: &[SourceDiagnostic],
    input: &Path,
    colored: bool,
    primary: Option<&Source>,
) {
    let main_id = world.main();
    let cwd = std::env::current_dir().ok();
    for diag in diagnostics {
        let mut ids = Vec::new();
        if let Some(id) = diag.span.id() {
            ids.push(id);
        }
        for point in &diag.trace {
            if let Some(id) = point.span.id() {
                if !ids.contains(&id) {
                    ids.push(id);
                }
            }
        }
        if ids.is_empty() {
            ids.push(main_id);
        }

        let sources = ids
            .into_iter()
            .filter_map(|id| {
                let is_primary = primary.is_some_and(|source| source.id() == id);
                let source = primary
                    .filter(|source| source.id() == id)
                    .cloned()
                    .or_else(|| world.source(id).ok())?;
                let path = if is_primary {
                    input.to_path_buf()
                } else {
                    world.path_of(id).unwrap_or_else(|| input.to_path_buf())
                };
                let display = cwd
                    .as_deref()
                    .and_then(|base| path.strip_prefix(base).ok())
                    .unwrap_or(&path)
                    .display()
                    .to_string();
                Some(DiagnosticSource::new(source, display))
            })
            .collect::<Vec<_>>();
        eprint!("{}", format_diagnostic(diag, &sources, colored));
    }
}
