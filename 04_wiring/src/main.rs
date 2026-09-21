//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring.md
//! @prompt-hash 21563a44
//! @layer L4
//! @updated 2026-09-20
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
use std::time::{Duration, Instant};

use typst_core::contracts::world::World;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_infra::pipeline::{
    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override,
    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override,
    compile_to_png_bytes_with_timings_full_error_and_features_and_ppi_override,
    compile_to_svg_string_with_timings_full_error_and_features_and_pretty_override,
};
use typst_infra::world::SystemWorld;
use typst_shell::cli::{
    self, CompileIntent, CompletionsIntent, EvalIntent, FontsIntent, InfoFormat,
    InfoIntent, InitIntent, OutputFormat, QueryIntent, RunIntent, WatchIntent,
};
use typst_shell::diagnostic::{format_diagnostic, DiagnosticSource};
use typst_shell::duration::{format_watch_line, WatchStatus};

const HTML_EXPERIMENTAL_WARNING: &str = concat!(
    "warning: html export is under active development and incomplete\n",
    " = hint: its behaviour may change at any time\n",
    " = hint: do not rely on this feature for production use cases\n",
    " = hint: see https://github.com/typst/typst/issues/5512 for more information\n\n",
);

#[cfg(test)]
mod tests {
    // Snippet A/B independente; integrar dentro do módulo #[cfg(test)] de main.rs.
    // Derivado de wiring.md §P1323 e do vanilla ratificado, sem leitura de main.rs.
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
    fn p1362_watch_status_for_outcome_selects_typed_status() {
        use std::process::ExitCode;
        use std::time::Duration;
        use typst_shell::duration::WatchStatus;

        let elapsed = Duration::new(2, 849_200_000);

        assert!(matches!(
            super::watch_status_for_outcome(ExitCode::SUCCESS, false, elapsed),
            WatchStatus::Success(actual) if actual == elapsed
        ));
        assert!(matches!(
            super::watch_status_for_outcome(ExitCode::SUCCESS, true, elapsed),
            WatchStatus::PartialSuccess(actual) if actual == elapsed
        ));
        assert!(matches!(
            super::watch_status_for_outcome(ExitCode::from(1), false, elapsed),
            WatchStatus::Error
        ));
        assert!(matches!(
            super::watch_status_for_outcome(ExitCode::from(2), true, elapsed),
            WatchStatus::Error
        ));
    }

    #[test]
    fn p1363_watch_line_at_uses_the_supplied_literal_time() {
        use std::time::Duration;
        use time::Time;
        use typst_shell::duration::WatchStatus;

        let time = Time::from_hms(3, 4, 5).expect("03:04:05 is a valid literal time");
        let elapsed = Duration::new(2, 849_200_000);

        assert_eq!(
            super::watch_line_at(&WatchStatus::Compiling, time),
            "[03:04:05] compiling ..."
        );
        assert_eq!(
            super::watch_line_at(&WatchStatus::Success(elapsed), time),
            "[03:04:05] compiled successfully in 2.85 s"
        );
        assert_eq!(
            super::watch_line_at(&WatchStatus::PartialSuccess(elapsed), time),
            "[03:04:05] compiled with warnings in 2.85 s"
        );
        assert_eq!(
            super::watch_line_at(&WatchStatus::Error, time),
            "[03:04:05] compiled with errors"
        );
    }

    #[test]
    fn p1364_watch_block_at_uses_one_lf_for_compiling_and_two_for_terminal_statuses() {
        use std::time::Duration;
        use time::Time;
        use typst_shell::duration::WatchStatus;

        let time = Time::from_hms(3, 4, 5).expect("03:04:05 is a valid literal time");
        let elapsed = Duration::new(2, 849_200_000);
        let cases = [
            (WatchStatus::Compiling, "[03:04:05] compiling ...\n"),
            (
                WatchStatus::Success(elapsed),
                "[03:04:05] compiled successfully in 2.85 s\n\n",
            ),
            (
                WatchStatus::PartialSuccess(elapsed),
                "[03:04:05] compiled with warnings in 2.85 s\n\n",
            ),
            (WatchStatus::Error, "[03:04:05] compiled with errors\n\n"),
        ];

        for (status, expected) in cases {
            let actual = super::watch_block_at(&status, time);
            assert_eq!(actual, expected);
            assert!(!actual.contains('\r'));
        }
    }

    #[test]
    fn p1365_prepare_watch_cycle_captures_before_formatting_once() {
        use std::cell::{Cell, RefCell};

        let order = RefCell::new(Vec::new());
        let capture_calls = Cell::new(0);
        let format_calls = Cell::new(0);
        let expected_token = String::from("captured-token-73");
        let expected_block = String::from("[03:04:05] compiling ...\n");

        let (token, block) = super::prepare_watch_cycle_with(
            || {
                order.borrow_mut().push("capture");
                capture_calls.set(capture_calls.get() + 1);
                expected_token.clone()
            },
            || {
                order.borrow_mut().push("format");
                format_calls.set(format_calls.get() + 1);
                expected_block.clone()
            },
        );

        assert_eq!(&*order.borrow(), &["capture", "format"]);
        assert_eq!(capture_calls.get(), 1);
        assert_eq!(format_calls.get(), 1);
        assert_eq!(token, expected_token);
        assert_eq!(block, expected_block);
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

    #[test]
    fn p1367_write_watch_block_writes_then_flushes_and_propagates_failures() {
        use std::io::{self, ErrorKind, Write};

        #[derive(Clone, Copy)]
        enum Mode {
            Success,
            WriteError,
            FlushError,
        }

        #[derive(Debug, PartialEq, Eq)]
        enum Event {
            Write(Vec<u8>),
            Flush,
        }

        struct FakeWriter {
            mode: Mode,
            bytes: Vec<u8>,
            events: Vec<Event>,
            write_calls: usize,
            flush_calls: usize,
        }

        impl FakeWriter {
            fn new(mode: Mode) -> Self {
                Self {
                    mode,
                    bytes: Vec::new(),
                    events: Vec::new(),
                    write_calls: 0,
                    flush_calls: 0,
                }
            }
        }

        impl Write for FakeWriter {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.write_calls += 1;
                self.events.push(Event::Write(buf.to_vec()));
                if matches!(self.mode, Mode::WriteError) {
                    return Err(io::Error::new(ErrorKind::BrokenPipe, "write sentinel"));
                }
                self.bytes.extend_from_slice(buf);
                Ok(buf.len())
            }

            fn flush(&mut self) -> io::Result<()> {
                self.flush_calls += 1;
                self.events.push(Event::Flush);
                if matches!(self.mode, Mode::FlushError) {
                    return Err(io::Error::new(ErrorKind::TimedOut, "flush sentinel"));
                }
                Ok(())
            }
        }

        let block = "[03:04:05] status:\0 café\n\n";
        let expected_bytes = block.as_bytes();

        let mut success = FakeWriter::new(Mode::Success);
        assert!(super::write_watch_block(&mut success, block).is_ok());
        assert_eq!(success.bytes, expected_bytes);
        assert_eq!(success.events, [Event::Write(expected_bytes.to_vec()), Event::Flush]);
        assert_eq!(success.write_calls, 1);
        assert_eq!(success.flush_calls, 1);

        let mut write_error = FakeWriter::new(Mode::WriteError);
        let error = super::write_watch_block(&mut write_error, block)
            .expect_err("the BrokenPipe sentinel must be propagated");
        assert_eq!(error.kind(), ErrorKind::BrokenPipe);
        assert_eq!(write_error.bytes, b"");
        assert_eq!(write_error.events, [Event::Write(expected_bytes.to_vec())]);
        assert_eq!(write_error.write_calls, 1);
        assert_eq!(write_error.flush_calls, 0);

        let mut flush_error = FakeWriter::new(Mode::FlushError);
        let error = super::write_watch_block(&mut flush_error, block)
            .expect_err("the TimedOut sentinel must be propagated");
        assert_eq!(error.kind(), ErrorKind::TimedOut);
        assert_eq!(flush_error.bytes, expected_bytes);
        assert_eq!(
            flush_error.events,
            [Event::Write(expected_bytes.to_vec()), Event::Flush]
        );
        assert_eq!(flush_error.write_calls, 1);
        assert_eq!(flush_error.flush_calls, 1);
    }
}

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
        let (started, compiling) = prepare_watch_cycle();
        emit_watch_block(&compiling);
        let mut report_status = |exit_code, has_warnings| {
            let status =
                watch_status_for_outcome(exit_code, has_warnings, started.elapsed());
            emit_watch_block(&watch_block_now(&status));
        };
        let (exit_code, dependencies, _) =
            run_compile_observed(compile, &mut report_status);
        let dependencies = if dependencies.is_empty() {
            vec![intent.compile.input.clone()]
        } else {
            dependencies
        };
        let armed = typst_infra::watch::arm(&dependencies);

        let snapshot = if exit_code == ExitCode::SUCCESS {
            match armed.publish(&staging, &destination) {
                Ok(snapshot) => snapshot,
                Err(error) => {
                    eprintln!(
                        "error: failed to replace {} atomically: {}",
                        destination.display(),
                        error
                    );
                    return ExitCode::from(2);
                }
            }
        } else {
            armed.abandon(&staging)
        };

        eviction::crystalline_evict(10);
        typst_infra::watch::wait_for_change_since(
            snapshot,
            std::time::Duration::from_millis(100),
        );
    }
}

fn write_watch_block(out: &mut impl std::io::Write, block: &str) -> std::io::Result<()> {
    out.write_all(block.as_bytes())?;
    out.flush()
}

fn emit_watch_block(block: &str) {
    let stderr = std::io::stderr();
    let mut out = stderr.lock();
    write_watch_block(&mut out, block).expect("failed to write watch status");
}

fn watch_line_at(status: &WatchStatus, time: time::Time) -> String {
    format_watch_line(time.hour(), time.minute(), time.second(), status)
}

fn watch_block_at(status: &WatchStatus, time: time::Time) -> String {
    let mut block = watch_line_at(status, time);
    if matches!(status, WatchStatus::Compiling) {
        block.push('\n');
    } else {
        block.push_str("\n\n");
    }
    block
}

fn watch_block_now(status: &WatchStatus) -> String {
    let now = time::OffsetDateTime::now_local()
        .unwrap_or_else(|_| time::OffsetDateTime::now_utc());
    watch_block_at(status, now.time())
}

fn prepare_watch_cycle_with<T>(
    capture: impl FnOnce() -> T,
    format_compiling: impl FnOnce() -> String,
) -> (T, String) {
    let started = capture();
    let compiling = format_compiling();
    (started, compiling)
}

fn prepare_watch_cycle() -> (Instant, String) {
    prepare_watch_cycle_with(Instant::now, || watch_block_now(&WatchStatus::Compiling))
}

fn watch_status_for_outcome(
    exit_code: ExitCode,
    has_warnings: bool,
    elapsed: Duration,
) -> WatchStatus {
    if exit_code != ExitCode::SUCCESS {
        WatchStatus::Error
    } else if has_warnings {
        WatchStatus::PartialSuccess(elapsed)
    } else {
        WatchStatus::Success(elapsed)
    }
}

fn report_in_order(status: impl FnOnce(), diagnostics: impl FnOnce()) {
    status();
    diagnostics();
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
    let mut report_status = |_, _| {};
    run_compile_observed(intent, &mut report_status).0
}

fn run_compile_observed(
    intent: CompileIntent,
    report_status: &mut impl FnMut(ExitCode, bool),
) -> (ExitCode, Vec<PathBuf>, bool) {
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
    } = intent;

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
        return (ExitCode::from(1), vec![input], false);
    }

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
            return (ExitCode::from(2), vec![input], false);
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
            report_in_order(
                || report_status(ExitCode::from(2), false),
                || eprintln!("error: {}", e),
            );
            return (ExitCode::from(2), vec![input], false);
        }
    };

    let source = match world.source(world.main()) {
        Ok(s) => s,
        Err(e) => {
            report_in_order(
                || report_status(ExitCode::from(2), false),
                || eprintln!("error: failed to load source: {:?}", e),
            );
            return (ExitCode::from(2), world.dependencies(), false);
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
    let (result, warnings, timings): (
        Result<Vec<u8>, Vec<SourceDiagnostic>>,
        Vec<SourceDiagnostic>,
        typst_infra::pipeline::Timings,
    ) = match output_format {
        OutputFormat::Pdf => {
            if oracle_pdf {
                let (r, w) =
                    typst_infra::pipeline::compile_to_pdf_bytes_oracle_with_features_and_tagged_and_pretty_and_pages_override(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tagged_override,
                        features,
                        pretty_override,
                        pages.clone(),
                    );
                (r, w, typst_infra::pipeline::Timings::default())
            } else if timings_json.is_some() {
                let (r, w, t) =
                    compile_to_pdf_bytes_with_timings_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tagged_override,
                        features,
                        pretty_override,
                        pages.clone(),
                    );
                (r, w, t)
            } else {
                let (r, w) =
                    compile_to_pdf_bytes_full_error_and_document_id_with_features_and_tagged_and_pretty_and_pages_override(
                        &world,
                        &source,
                        full_error,
                        document_id,
                        stream_mode,
                        pdf_tagged_override,
                        features,
                        pretty_override,
                        pages.clone(),
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
            if let Err(e) = std::fs::write(&output, &output_bytes) {
                operational_error =
                    Some(format!("error: failed to write {}: {}", output.display(), e));
            } else if let Some(path) = timings_json.as_ref() {
                if let Err(e) = std::fs::write(&path, timings.to_json()) {
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
                ExitCode::SUCCESS
            }
        }
        Err(compilation_errors) => {
            errors = Some(compilation_errors);
            if let Some(path) = timings_json.as_ref() {
                if let Err(e) = std::fs::write(&path, timings.to_json()) {
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
            if emit_html_experimental_warning {
                eprint!("{HTML_EXPERIMENTAL_WARNING}");
            }
            drain_to_stderr(&world, &warnings, &input, colored);
            if let Some(errors) = errors.as_deref() {
                drain_to_stderr(&world, errors, &input, colored);
            }
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

    (exit_code, dependencies, has_warnings)
}

fn run_eval(intent: EvalIntent) -> ExitCode {
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
