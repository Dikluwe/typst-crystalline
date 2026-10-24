//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/init.md
//! @prompt-hash a50cd53b
//! @layer L4
//! @updated 2026-09-26

use super::diagnostics::write_stderr;
use std::io::Write;
use std::path::Path;
use std::process::ExitCode;
use typst_shell::cli::InitIntent;

pub(super) fn run_init(intent: InitIntent) -> std::io::Result<ExitCode> {
    match typst_infra::project_init::initialize(
        &intent.template,
        intent.directory,
        intent.cert_path,
    ) {
        Ok(result) => {
            let stdout = std::io::stdout();
            let mut out = stdout.lock();
            write_success_to(&mut out, &result.destination, &result.entrypoint)?;
            Ok(ExitCode::SUCCESS)
        }
        Err(error) => {
            write_stderr(&format!("error: {error}\n"))?;
            Ok(ExitCode::from(1))
        }
    }
}

fn write_success_to(
    out: &mut impl Write,
    destination: &Path,
    entrypoint: &Path,
) -> std::io::Result<()> {
    writeln!(out, "Successfully created project in {}", destination.display())?;
    writeln!(out, "Entrypoint: {}", entrypoint.display())?;
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::{self, Write};
    use std::path::Path;

    struct Writer {
        bytes: Vec<u8>,
        max_write: usize,
        fail_after: Option<usize>,
        write_failed: bool,
        flush_failed: bool,
        flushes: usize,
    }

    impl Writer {
        fn new() -> Self {
            Self {
                bytes: Vec::new(),
                max_write: 2,
                fail_after: None,
                write_failed: false,
                flush_failed: false,
                flushes: 0,
            }
        }
    }

    impl Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            assert!(!self.write_failed, "a failed write must not be retried");
            if self.fail_after == Some(self.bytes.len()) {
                self.write_failed = true;
                return Err(io::Error::new(
                    io::ErrorKind::BrokenPipe,
                    "injected write failure",
                ));
            }
            let remaining =
                self.fail_after.map_or(usize::MAX, |limit| limit - self.bytes.len());
            let count = bytes.len().min(self.max_write).min(remaining);
            self.bytes.extend_from_slice(&bytes[..count]);
            Ok(count)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.flush_failed {
                Err(io::Error::new(
                    io::ErrorKind::PermissionDenied,
                    "injected flush failure",
                ))
            } else {
                Ok(())
            }
        }
    }

    const EXPECTED: &[u8] = b"Successfully created project in new project\nEntrypoint: new project/main.typ\n";

    #[test]
    fn init_notification_short_writes_preserve_bytes_and_flush() {
        let mut out = Writer::new();
        write_success_to(
            &mut out,
            Path::new("new project"),
            Path::new("new project/main.typ"),
        )
        .unwrap();
        assert_eq!(out.bytes, EXPECTED);
        assert_eq!(out.flushes, 1);
    }

    #[test]
    fn init_notification_write_failures_preserve_error_and_stop() {
        let first_line = b"Successfully created project in new project\n".len();
        for limit in [0, first_line, first_line + 3] {
            let mut out = Writer::new();
            out.fail_after = Some(limit);
            let error = write_success_to(
                &mut out,
                Path::new("new project"),
                Path::new("new project/main.typ"),
            )
            .unwrap_err();
            assert_eq!(error.kind(), io::ErrorKind::BrokenPipe);
            assert_eq!(error.to_string(), "injected write failure");
            assert_eq!(out.bytes, EXPECTED[..limit]);
            assert_eq!(out.flushes, 0);
        }
    }

    #[test]
    fn init_notification_flush_failure_propagates_after_both_lines() {
        let mut out = Writer::new();
        out.flush_failed = true;
        let error = write_success_to(
            &mut out,
            Path::new("new project"),
            Path::new("new project/main.typ"),
        )
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::PermissionDenied);
        assert_eq!(error.to_string(), "injected flush failure");
        assert_eq!(out.bytes, EXPECTED);
        assert_eq!(out.flushes, 1);
    }

    #[test]
    fn init_notification_zero_progress_is_a_write_error() {
        let mut out = Writer::new();
        out.max_write = 0;
        let error = write_success_to(
            &mut out,
            Path::new("new project"),
            Path::new("new project/main.typ"),
        )
        .unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::WriteZero);
        assert!(out.bytes.is_empty());
        assert_eq!(out.flushes, 0);
    }
}
