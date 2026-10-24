//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/watch.md
//! @prompt-hash c328f691
//! @layer L4
//! @updated 2026-10-08

use super::{
    compile::run_compile_observed_with_sessions, diagnostics, eviction, Published,
};
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};
use typst_core::entities::world_types::{FileError, FileResult};
use typst_shell::cli::{OutputFormat, WatchIntent};
use typst_shell::duration::{format_watch_line, WatchStatus};

pub(super) fn run_watch(intent: WatchIntent) -> Result<ExitCode, Vec<std::io::Error>> {
    if intent.compile.output == Path::new("-") {
        diagnostics::write_stderr(
            "error: watch requires a file output; stdout is not supported\n",
        )
        .map_err(|error| vec![error])?;
        return Ok(ExitCode::from(2));
    }

    let server = if !intent.no_serve
        && matches!(
            intent.compile.output_format,
            OutputFormat::Html | OutputFormat::Bundle
        ) {
        match typst_infra::watch::BundleHttpServer::new(
            &intent.compile.input.display().to_string(),
            intent.port,
            !intent.no_reload,
        ) {
            Ok(server) => {
                diagnostics::write_stderr(&format!(
                    "watching at http://{}/\n",
                    server.addr()
                ))
                .map_err(|error| vec![error])?;
                Some(server)
            }
            Err(error) => {
                diagnostics::write_stderr(&format!(
                    "error: failed to start local server: {error}\n"
                ))
                .map_err(|error| vec![error])?;
                return Ok(ExitCode::from(2));
            }
        }
    } else {
        None
    };
    let destination = intent.compile.output.clone();
    let file_name = destination
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("output");
    let staging = destination.with_file_name(format!(
        ".{file_name}.crystalline-watch-{}.tmp",
        std::process::id()
    ));

    let font_session = std::sync::Arc::new(typst_infra::world::FontCatalogSession::new());
    let source_session = std::sync::Arc::new(typst_infra::world::SourceSession::new());
    loop {
        let mut compile = intent.compile.clone();
        compile.output = staging.clone();
        let (started, compiling) = prepare_watch_cycle();
        emit_watch_block(&compiling).map_err(|error| vec![error])?;
        let mut report_status = |exit_code, has_warnings| {
            let status =
                watch_status_for_outcome(exit_code, has_warnings, started.elapsed());
            emit_watch_block(&watch_block_now(&status))
        };
        let mut written_outputs = Vec::new();
        let (exit_code, dependencies, published) = after_compilation(
            run_compile_observed_with_sessions(
                compile,
                &mut report_status,
                Some(font_session.clone()),
                Some(source_session.clone()),
                &mut written_outputs,
            ),
            || discard_written_outputs(&written_outputs, intent.compile.output_format),
        )?;
        let armed = match with_dependencies(
            dependencies,
            &intent.compile.input,
            || discard_staging(&staging, intent.compile.output_format),
            typst_infra::watch::arm,
        ) {
            Ok(armed) => armed,
            Err(errors) => {
                for error in errors {
                    diagnostics::write_stderr(&format!(
                        "error: failed to finalize watch dependencies: {error}\n"
                    ))
                    .map_err(|error| vec![error])?;
                }
                return Ok(ExitCode::from(2));
            }
        };

        let snapshot = if exit_code == ExitCode::SUCCESS {
            let publication = if intent.compile.output_format == OutputFormat::Bundle {
                armed.publish_directory(&staging, &destination)
            } else {
                armed.publish(&staging, &destination)
            };
            match publication {
                Ok(snapshot) => snapshot,
                Err(error) => {
                    return Err(failed_publication(
                        error,
                        &destination,
                        || {
                            discard_written_outputs(
                                &written_outputs,
                                intent.compile.output_format,
                            )
                        },
                        diagnostics::write_stderr,
                    ));
                }
            }
        } else if intent.compile.output_format == OutputFormat::Bundle {
            armed.abandon_directory(&staging)
        } else {
            armed.abandon(&staging)
        };

        if exit_code == ExitCode::SUCCESS {
            if let (Some(server), Some(published)) = (&server, published) {
                match published {
                    Published::Html(html) => server.publish_html(&html),
                    Published::Bundle(files) => server.publish(&files),
                }
            }
        }

        eviction::crystalline_evict(10);
        typst_infra::watch::wait_for_change_since(
            snapshot,
            std::time::Duration::from_millis(100),
        );
    }
}

fn failed_publication(
    publication_error: std::io::Error,
    destination: &Path,
    discard: impl FnOnce() -> Result<(), Vec<std::io::Error>>,
    mut report: impl FnMut(&str) -> std::io::Result<()>,
) -> Vec<std::io::Error> {
    let primary_message = format!(
        "error: failed to replace {} atomically: {}\n",
        destination.display(),
        publication_error
    );
    let mut errors = vec![publication_error];
    if let Err(cleanup_errors) = discard() {
        errors.extend(cleanup_errors);
    }
    if let Err(error) = report(&primary_message) {
        errors.push(error);
        return errors;
    }
    let reporting_error = errors.iter().skip(1).find_map(|error| {
        report(&format!("error: failed to discard watch staging: {error}\n")).err()
    });
    if let Some(error) = reporting_error {
        errors.push(error);
    }
    errors
}

fn after_compilation<T>(
    outcome: std::io::Result<T>,
    discard: impl FnOnce() -> Result<(), Vec<std::io::Error>>,
) -> Result<T, Vec<std::io::Error>> {
    match outcome {
        Ok(value) => Ok(value),
        Err(error) => {
            let mut errors = vec![error];
            if let Err(cleanup_errors) = discard() {
                errors.extend(cleanup_errors);
            }
            Err(errors)
        }
    }
}

fn discard_written_outputs(
    outputs: &[std::path::PathBuf],
    format: OutputFormat,
) -> Result<(), Vec<std::io::Error>> {
    let mut errors = Vec::new();
    for output in outputs {
        if let Err(error) = discard_staging(output, format) {
            errors.push(error);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}

fn with_dependencies<T>(
    dependencies: FileResult<Vec<std::path::PathBuf>>,
    input: &Path,
    discard: impl FnOnce() -> std::io::Result<()>,
    arm: impl FnOnce(&[std::path::PathBuf]) -> T,
) -> Result<T, Vec<FileError>> {
    match dependencies {
        Ok(mut paths) => {
            if paths.is_empty() {
                paths.push(input.to_path_buf());
            }
            Ok(arm(&paths))
        }
        Err(error) => {
            let mut errors = vec![error];
            if let Err(error) = discard() {
                errors.push(FileError::Other(format!(
                    "failed to discard watch staging: {error}"
                )));
            }
            Err(errors)
        }
    }
}

fn discard_staging(staging: &Path, format: OutputFormat) -> std::io::Result<()> {
    let result = if format == OutputFormat::Bundle {
        std::fs::remove_dir_all(staging)
    } else {
        std::fs::remove_file(staging)
    };
    match result {
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        result => result,
    }
}

fn write_watch_block(out: &mut impl std::io::Write, block: &str) -> std::io::Result<()> {
    out.write_all(block.as_bytes())?;
    out.flush()
}

fn emit_watch_block(block: &str) -> std::io::Result<()> {
    let stderr = std::io::stderr();
    let mut out = stderr.lock();
    write_watch_block(&mut out, block)
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

#[cfg(test)]
mod tests {
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

#[cfg(test)]
mod metadata_tests {
    use super::*;
    use std::cell::Cell;
    use std::io::{Read, Write};

    fn served(server: &typst_infra::watch::BundleHttpServer) -> String {
        let mut stream =
            std::net::TcpStream::connect(server.addr()).expect("HTTP fixture");
        stream.set_read_timeout(Some(Duration::from_secs(5))).unwrap();
        stream.write_all(b"GET /index.html HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n").unwrap();
        let mut text = String::new();
        stream.read_to_string(&mut text).unwrap();
        text
    }

    #[test]
    fn metadata_failed_graph_preserves_disk_and_http_until_separate_healthy_run() {
        for format in [OutputFormat::Html, OutputFormat::Bundle] {
            let root = std::env::temp_dir()
                .join(format!("tekt-metadata-watch-{}-{format:?}", std::process::id()));
            std::fs::create_dir_all(&root).unwrap();
            let input = root.join("input.typ");
            std::fs::write(&input, "input").unwrap();
            let destination = root.join("destination");
            let staging = root.join("staging");
            let (old_file, new_file) = if format == OutputFormat::Bundle {
                std::fs::create_dir_all(&destination).unwrap();
                std::fs::create_dir_all(&staging).unwrap();
                (destination.join("index.html"), staging.join("index.html"))
            } else {
                (destination.clone(), staging.clone())
            };
            std::fs::write(&old_file, b"previous valid artifact").unwrap();
            std::fs::write(&new_file, b"must not publish").unwrap();
            let server =
                typst_infra::watch::BundleHttpServer::new("metadata", Some(0), false)
                    .unwrap();
            server.publish_html("previous valid HTTP");
            let called = Cell::new(false);
            let error =
                FileError::Other("file identity registry lock is poisoned".into());
            let outcome = with_dependencies(
                Err(error.clone()),
                &input,
                || discard_staging(&staging, format),
                |paths| {
                    called.set(true);
                    let armed = typst_infra::watch::arm(paths);
                    if format == OutputFormat::Bundle {
                        armed.publish_directory(&staging, &destination).unwrap();
                    } else {
                        armed.publish(&staging, &destination).unwrap();
                    }
                    server.publish_html("must not publish HTTP");
                },
            );
            assert_eq!(outcome, Err(vec![error]));
            assert!(!called.get());
            assert!(!staging.exists());
            assert_eq!(std::fs::read(&old_file).unwrap(), b"previous valid artifact");
            assert!(served(&server).ends_with("previous valid HTTP"));
            if format == OutputFormat::Bundle {
                std::fs::create_dir_all(&staging).unwrap();
            }
            std::fs::write(&new_file, b"new healthy artifact").unwrap();
            with_dependencies(
                Ok(vec![input.clone()]),
                &input,
                || panic!("healthy graph must not discard"),
                |paths| {
                    let armed = typst_infra::watch::arm(paths);
                    if format == OutputFormat::Bundle {
                        armed.publish_directory(&staging, &destination).unwrap();
                    } else {
                        armed.publish(&staging, &destination).unwrap();
                    }
                    server.publish_html("new healthy HTTP");
                },
            )
            .unwrap();
            assert_eq!(std::fs::read(&old_file).unwrap(), b"new healthy artifact");
            assert!(served(&server).ends_with("new healthy HTTP"));
            drop(server);
            std::fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn metadata_watch_retains_graph_error_and_real_discard_error() {
        let root = std::env::temp_dir()
            .join(format!("tekt-metadata-discard-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let error = FileError::Other("graph acquisition failed".into());
        let armed = Cell::new(false);
        let outcome = with_dependencies(
            Err(error.clone()),
            Path::new("input.typ"),
            || discard_staging(&root, OutputFormat::Html),
            |_| {
                armed.set(true);
            },
        );
        let errors = outcome.unwrap_err();
        assert_eq!(errors.first(), Some(&error));
        assert_eq!(errors.len(), 2);
        assert!(
            matches!(&errors[1], FileError::Other(message) if message.starts_with("failed to discard watch staging:"))
        );
        assert!(!armed.get());
        assert!(root.is_dir());
        std::fs::remove_dir(root).unwrap();
    }

    #[test]
    fn metadata_watch_healthy_empty_graph_keeps_only_existing_input_fallback() {
        let input = Path::new("input.typ");
        let called = Cell::new(0);
        with_dependencies(
            Ok(vec![]),
            input,
            || panic!("healthy graph must not discard"),
            |paths| {
                called.set(called.get() + 1);
                assert_eq!(paths, &[input.to_path_buf()]);
            },
        )
        .unwrap();
        assert_eq!(called.get(), 1);
    }

    #[test]
    fn stderr_terminal_failure_discards_staging_before_arm_and_preserves_live_output() {
        struct TerminalWriter {
            fail_write: bool,
        }
        impl std::io::Write for TerminalWriter {
            fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
                if self.fail_write {
                    Err(std::io::Error::from_raw_os_error(28))
                } else {
                    Ok(bytes.len())
                }
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Err(std::io::Error::from_raw_os_error(32))
            }
        }

        for format in [OutputFormat::Html, OutputFormat::Bundle] {
            for fail_write in [true, false] {
                let root = std::env::temp_dir().join(format!(
                    "tekt-stderr-watch-{}-{format:?}-{fail_write}",
                    std::process::id()
                ));
                std::fs::create_dir_all(&root).unwrap();
                let staging = root.join("staging");
                let destination = root.join("destination");
                let (new_file, old_file) = if format == OutputFormat::Bundle {
                    std::fs::create_dir(&staging).unwrap();
                    std::fs::create_dir(&destination).unwrap();
                    (staging.join("index.html"), destination.join("index.html"))
                } else {
                    (staging.clone(), destination.clone())
                };
                std::fs::write(&new_file, b"must not publish").unwrap();
                std::fs::write(&old_file, b"previous disk").unwrap();
                let server =
                    typst_infra::watch::BundleHttpServer::new("stderr", Some(0), false)
                        .unwrap();
                server.publish_html("previous HTTP");
                let continuation = Cell::new(false);
                let terminal = write_watch_block(
                    &mut TerminalWriter { fail_write },
                    "[03:04:05] compiled successfully in 2.85 s\n\n",
                );
                let outcome = after_compilation(terminal, || {
                    discard_written_outputs(std::slice::from_ref(&staging), format)
                })
                .map(|()| {
                    continuation.set(true);
                    let armed = typst_infra::watch::arm(&[root.join("input.typ")]);
                    if format == OutputFormat::Bundle {
                        armed.publish_directory(&staging, &destination).unwrap();
                    } else {
                        armed.publish(&staging, &destination).unwrap();
                    }
                    server.publish_html("must not publish HTTP");
                    eviction::crystalline_evict(10);
                });
                let errors = outcome.unwrap_err();
                assert_eq!(errors.len(), 1);
                assert_eq!(
                    errors[0].raw_os_error(),
                    Some(if fail_write { 28 } else { 32 })
                );
                assert!(!continuation.get());
                assert!(!staging.exists());
                assert_eq!(std::fs::read(&old_file).unwrap(), b"previous disk");
                assert!(served(&server).ends_with("previous HTTP"));

                if format == OutputFormat::Bundle {
                    std::fs::create_dir(&staging).unwrap();
                }
                std::fs::write(&new_file, b"healthy disk").unwrap();
                after_compilation(Ok(()), || panic!("healthy must not discard"))
                    .map(|()| {
                        let armed = typst_infra::watch::arm(&[root.join("input.typ")]);
                        if format == OutputFormat::Bundle {
                            armed.publish_directory(&staging, &destination).unwrap();
                        } else {
                            armed.publish(&staging, &destination).unwrap();
                        }
                        server.publish_html("healthy HTTP");
                    })
                    .unwrap();
                assert_eq!(std::fs::read(&old_file).unwrap(), b"healthy disk");
                assert!(served(&server).ends_with("healthy HTTP"));
                drop(server);
                std::fs::remove_dir_all(root).unwrap();
            }
        }
    }

    #[test]
    fn stderr_terminal_failure_retains_primary_payload_and_real_cleanup_error() {
        let root = std::env::temp_dir()
            .join(format!("tekt-stderr-watch-discard-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let expected_cleanup = std::fs::remove_file(&root).unwrap_err();
        let primary =
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, std::fmt::Error);
        let result: std::io::Result<()> = Err(primary);
        let errors = after_compilation(result, || {
            discard_written_outputs(std::slice::from_ref(&root), OutputFormat::Html)
        })
        .unwrap_err();
        assert_eq!(errors.len(), 2);
        assert_eq!(errors[0].kind(), std::io::ErrorKind::BrokenPipe);
        assert!(errors[0]
            .get_ref()
            .unwrap()
            .downcast_ref::<std::fmt::Error>()
            .is_some());
        assert_eq!(errors[1].kind(), expected_cleanup.kind());
        assert_eq!(errors[1].raw_os_error(), expected_cleanup.raw_os_error());
        assert_eq!(errors[1].to_string(), expected_cleanup.to_string());
        assert!(root.is_dir());
        std::fs::remove_dir(root).unwrap();
    }

    #[test]
    fn stderr_missing_staging_keeps_original_error_without_fake_cleanup_failure() {
        let root = std::env::temp_dir()
            .join(format!("tekt-stderr-watch-absent-{}", std::process::id()));
        let result: std::io::Result<()> = Err(std::io::Error::from_raw_os_error(28));
        let errors = after_compilation(result, || {
            discard_written_outputs(std::slice::from_ref(&root), OutputFormat::Html)
        })
        .unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].raw_os_error(), Some(28));
    }

    #[test]
    fn stderr_svg_manifest_cleanup_removes_only_exact_recorded_outputs() {
        let root = std::env::temp_dir()
            .join(format!("tekt-stderr-svg-manifest-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let final_literal = root.join("out-{p}.svg");
        let final_one = root.join("out-1.svg");
        let final_two = root.join("out-2.svg");
        let unrelated = root.join(".out-3.svg.crystalline-watch-other.tmp");
        for path in [&final_literal, &final_one, &final_two, &unrelated] {
            std::fs::write(path, b"untouched sentinel").unwrap();
        }
        let literal_stage = root.join(".out-{p}.svg.crystalline-watch-fixture.tmp");
        let stage_one = root.join(".out-1.svg.crystalline-watch-fixture.tmp");
        let stage_two = root.join(".out-2.svg.crystalline-watch-fixture.tmp");
        for path in [&stage_one, &stage_two] {
            std::fs::write(path, b"unpublished SVG").unwrap();
        }
        let manifest = vec![literal_stage, stage_one.clone(), stage_two.clone()];
        let result: std::io::Result<()> = Err(std::io::Error::from_raw_os_error(32));
        let continued = Cell::new(false);
        let errors = after_compilation(result, || {
            discard_written_outputs(&manifest, OutputFormat::Svg)
        })
        .map(|()| continued.set(true))
        .unwrap_err();
        assert_eq!(errors.len(), 1);
        assert_eq!(errors[0].raw_os_error(), Some(32));
        assert!(!continued.get());
        assert!(!stage_one.exists());
        assert!(!stage_two.exists());
        for path in [&final_literal, &final_one, &final_two, &unrelated] {
            assert_eq!(std::fs::read(path).unwrap(), b"untouched sentinel");
        }
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn stderr_manifest_cleanup_attempts_all_outputs_and_retains_each_actual_error() {
        let root = std::env::temp_dir()
            .join(format!("tekt-stderr-manifest-errors-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let wrong_kind = root.join("directory-in-file-manifest");
        std::fs::create_dir(&wrong_kind).unwrap();
        let valid_stage = root.join("exact.svg.tmp");
        std::fs::write(&valid_stage, b"must remove after first error").unwrap();
        let wrong_parent = root.join("file-parent");
        std::fs::write(&wrong_parent, b"must preserve").unwrap();
        let invalid_child = wrong_parent.join("child.svg.tmp");
        let first_cleanup = std::fs::remove_file(&wrong_kind).unwrap_err();
        let second_cleanup = std::fs::remove_file(&invalid_child).unwrap_err();
        let manifest = vec![wrong_kind.clone(), valid_stage.clone(), invalid_child];
        let primary =
            std::io::Error::new(std::io::ErrorKind::BrokenPipe, std::fmt::Error);
        let result: std::io::Result<()> = Err(primary);
        let errors = after_compilation(result, || {
            discard_written_outputs(&manifest, OutputFormat::Svg)
        })
        .unwrap_err();
        assert_eq!(errors.len(), 3);
        assert_eq!(errors[0].kind(), std::io::ErrorKind::BrokenPipe);
        assert!(errors[0]
            .get_ref()
            .unwrap()
            .downcast_ref::<std::fmt::Error>()
            .is_some());
        for (actual, expected) in errors[1..].iter().zip([first_cleanup, second_cleanup])
        {
            assert_eq!(actual.kind(), expected.kind());
            assert_eq!(actual.raw_os_error(), expected.raw_os_error());
            assert_eq!(actual.to_string(), expected.to_string());
        }
        assert!(!valid_stage.exists());
        assert!(wrong_kind.is_dir());
        assert_eq!(std::fs::read(&wrong_parent).unwrap(), b"must preserve");
        std::fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn stderr_failed_publication_discards_manifest_before_reporting_and_keeps_destinations(
    ) {
        for fail_report in [false, true] {
            let root = std::env::temp_dir().join(format!(
                "tekt-stderr-publication-{}-{fail_report}",
                std::process::id()
            ));
            std::fs::create_dir_all(&root).unwrap();
            let literal_staging = root.join(".out-{p}.svg.tmp");
            let destination = root.join("out-{p}.svg");
            let final_page = root.join("out-1.svg");
            let exact_staging = root.join(".out-1.svg.tmp");
            let unrecorded = root.join(".out-2.svg-unrecorded.tmp");
            for path in [&destination, &final_page, &unrecorded] {
                std::fs::write(path, b"previous valid sentinel").unwrap();
            }
            std::fs::write(&exact_staging, b"unpublished SVG").unwrap();
            let armed = typst_infra::watch::arm(&[root.join("input.typ")]);
            let error = match armed.publish(&literal_staging, &destination) {
                Err(error) => error,
                Ok(_) => panic!("literal staging is absent"),
            };
            let publication_kind = error.kind();
            let publication_errno = error.raw_os_error();
            let expected = format!(
                "error: failed to replace {} atomically: {}\n",
                destination.display(),
                error
            );
            let calls = Cell::new(0);
            let errors = failed_publication(
                error,
                &destination,
                || {
                    discard_written_outputs(
                        std::slice::from_ref(&exact_staging),
                        OutputFormat::Svg,
                    )
                },
                |message| {
                    calls.set(calls.get() + 1);
                    assert!(!exact_staging.exists(), "cleanup must precede reporting");
                    assert_eq!(message, expected);
                    if fail_report {
                        Err(std::io::Error::from_raw_os_error(32))
                    } else {
                        Ok(())
                    }
                },
            );
            assert_eq!(calls.get(), 1);
            assert_eq!(errors.len(), if fail_report { 2 } else { 1 });
            assert_eq!(errors[0].kind(), publication_kind);
            assert_eq!(errors[0].raw_os_error(), publication_errno);
            if fail_report {
                assert_eq!(errors[1].raw_os_error(), Some(32));
            }
            assert!(!exact_staging.exists());
            for path in [&destination, &final_page, &unrecorded] {
                assert_eq!(std::fs::read(path).unwrap(), b"previous valid sentinel");
            }
            std::fs::remove_dir_all(root).unwrap();
        }
    }

    #[test]
    fn stderr_publication_cleanup_errors_survive_and_reporting_stops_at_first_failure() {
        for fail_call in [None, Some(1), Some(2)] {
            let root = std::env::temp_dir().join(format!(
                "tekt-stderr-publication-cleanup-{}-{fail_call:?}",
                std::process::id()
            ));
            std::fs::create_dir_all(&root).unwrap();
            let blocked = root.join("directory-in-file-manifest");
            std::fs::create_dir(&blocked).unwrap();
            let exact_staging = root.join("exact.svg.tmp");
            std::fs::write(&exact_staging, b"must remove despite cleanup error").unwrap();
            let original_cleanup = std::fs::remove_file(&blocked).unwrap_err();
            let manifest = vec![blocked.clone(), exact_staging.clone()];
            let destination = root.join("out.svg");
            std::fs::write(&destination, b"previous valid destination").unwrap();
            let primary = std::io::Error::new(
                std::io::ErrorKind::PermissionDenied,
                std::fmt::Error,
            );
            let expected_primary = format!(
                "error: failed to replace {} atomically: {}\n",
                destination.display(),
                primary
            );
            let expected_cleanup =
                format!("error: failed to discard watch staging: {}\n", original_cleanup);
            let calls = Cell::new(0);
            let errors = failed_publication(
                primary,
                &destination,
                || discard_written_outputs(&manifest, OutputFormat::Svg),
                |message| {
                    calls.set(calls.get() + 1);
                    assert!(!exact_staging.exists());
                    match calls.get() {
                        1 => assert_eq!(message, expected_primary),
                        2 => assert_eq!(message, expected_cleanup),
                        _ => panic!("reporting must not retry"),
                    }
                    if fail_call == Some(calls.get()) {
                        Err(std::io::Error::from_raw_os_error(32))
                    } else {
                        Ok(())
                    }
                },
            );
            assert_eq!(calls.get(), if fail_call == Some(1) { 1 } else { 2 });
            assert_eq!(errors.len(), if fail_call.is_some() { 3 } else { 2 });
            assert_eq!(errors[0].kind(), std::io::ErrorKind::PermissionDenied);
            assert!(errors[0]
                .get_ref()
                .unwrap()
                .downcast_ref::<std::fmt::Error>()
                .is_some());
            assert_eq!(errors[1].kind(), original_cleanup.kind());
            assert_eq!(errors[1].raw_os_error(), original_cleanup.raw_os_error());
            if fail_call.is_some() {
                assert_eq!(errors[2].raw_os_error(), Some(32));
            }
            assert!(blocked.is_dir());
            assert!(!exact_staging.exists());
            assert_eq!(
                std::fs::read(&destination).unwrap(),
                b"previous valid destination"
            );
            std::fs::remove_dir_all(root).unwrap();
        }
    }
}
