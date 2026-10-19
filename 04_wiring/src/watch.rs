//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/watch.md
//! @prompt-hash cde7c073
//! @layer L4
//! @updated 2026-10-08

use super::{compile::run_compile_observed_with_sessions, eviction, Published};
use std::path::Path;
use std::process::ExitCode;
use std::time::{Duration, Instant};
use typst_shell::cli::{OutputFormat, WatchIntent};
use typst_shell::duration::{format_watch_line, WatchStatus};

pub(super) fn run_watch(intent: WatchIntent) -> ExitCode {
    if intent.compile.output == Path::new("-") {
        eprintln!("error: watch requires a file output; stdout is not supported");
        return ExitCode::from(2);
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
                eprintln!("watching at http://{}/", server.addr());
                Some(server)
            }
            Err(error) => {
                eprintln!("error: failed to start local server: {error}");
                return ExitCode::from(2);
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
        emit_watch_block(&compiling);
        let mut report_status = |exit_code, has_warnings| {
            let status =
                watch_status_for_outcome(exit_code, has_warnings, started.elapsed());
            emit_watch_block(&watch_block_now(&status));
        };
        let (exit_code, dependencies, _, published) = run_compile_observed_with_sessions(
            compile,
            &mut report_status,
            Some(font_session.clone()),
            Some(source_session.clone()),
        );
        let dependencies = if dependencies.is_empty() {
            vec![intent.compile.input.clone()]
        } else {
            dependencies
        };
        let armed = typst_infra::watch::arm(&dependencies);

        let snapshot = if exit_code == ExitCode::SUCCESS {
            let publication = if intent.compile.output_format == OutputFormat::Bundle {
                armed.publish_directory(&staging, &destination)
            } else {
                armed.publish(&staging, &destination)
            };
            match publication {
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
