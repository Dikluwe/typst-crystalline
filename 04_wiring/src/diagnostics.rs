//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/wiring/diagnostics.md
//! @prompt-hash 8db8d8ae
//! @layer L4
//! @updated 2026-10-06

use std::path::Path;
use typst_core::contracts::world::World;
use typst_core::entities::file_id::FileId;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::SourceDiagnostic;
use typst_core::entities::world_types::{FileError, FileResult};
use typst_infra::world::SystemWorld;
use typst_shell::diagnostic::{
    format_diagnostic, format_diagnostic_with_unresolved_traces, DiagnosticSource,
};

/// Prepare diagnostics before publication, retaining unavailable-source errors.
pub(super) fn prepare(
    world: &SystemWorld,
    diagnostics: &[SourceDiagnostic],
    input: &Path,
    colored: bool,
) -> (String, Vec<FileError>) {
    prepare_with_primary(world, diagnostics, input, colored, None)
}

pub(super) fn prepare_with_primary(
    world: &SystemWorld,
    diagnostics: &[SourceDiagnostic],
    input: &Path,
    colored: bool,
    primary: Option<&Source>,
) -> (String, Vec<FileError>) {
    let cwd = std::env::current_dir().ok();
    format_with_sources(world.main(), diagnostics, colored, |id| {
        let (source, path) = if let Some(source) = primary.filter(|s| s.id() == id) {
            (source.clone(), input.to_path_buf())
        } else {
            let registered_path = world.path_of(id)?;
            let source = match world.source(id) {
                Ok(source) => source,
                Err(FileError::NotFound) if registered_path.is_none() => return Ok(None),
                Err(error) => return Err(error),
            };
            let path = registered_path.unwrap_or_else(|| input.to_path_buf());
            (source, path)
        };
        let display = cwd
            .as_deref()
            .and_then(|base| pathdiff::diff_paths(&path, base))
            .unwrap_or(path)
            .display()
            .to_string();
        Ok(Some(DiagnosticSource::new(source, display)))
    })
}

fn format_with_sources(
    main_id: FileId,
    diagnostics: &[SourceDiagnostic],
    colored: bool,
    resolve: impl Fn(FileId) -> FileResult<Option<DiagnosticSource>>,
) -> (String, Vec<FileError>) {
    let mut text = String::new();
    let mut errors = Vec::new();
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
        let errors_before = errors.len();
        let mut sources = Vec::new();
        for id in ids {
            match resolve(id) {
                Ok(Some(source)) => sources.push(source),
                Ok(None) => {}
                Err(error) => errors.push(error),
            }
        }
        let formatted = if errors.len() > errors_before {
            format_diagnostic_with_unresolved_traces(diag, &sources, colored)
        } else {
            format_diagnostic(diag, &sources, colored)
        };
        text.push_str(&formatted);
    }
    (text, errors)
}

/// Emit already formatted diagnostic bytes through a checked process sink.
pub(super) fn write_stderr(text: &str) -> std::io::Result<()> {
    if text.is_empty() {
        return Ok(());
    }
    let stderr = std::io::stderr();
    let mut out = stderr.lock();
    write_stderr_to(&mut out, text)
}

fn write_stderr_to(out: &mut impl std::io::Write, text: &str) -> std::io::Result<()> {
    if text.is_empty() {
        return Ok(());
    }
    out.write_all(text.as_bytes())?;
    out.flush()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    use typst_core::entities::source_result::Tracepoint;
    use typst_core::entities::span::{Span, Spanned};

    #[test]
    fn metadata_diagnostic_failure_preserves_original_messages_hints_and_trace() {
        let source = Source::detached("original").expect("source fixture");
        let span = Span::from_range(source.id(), 0..8);
        let mut error =
            SourceDiagnostic::error(span, "original error").with_hint("original hint");
        error
            .trace
            .push(Spanned::new(Tracepoint::Call(Some("original_call".into())), span));
        let warning = SourceDiagnostic::warning(span, "original warning");
        let diagnostics = vec![error, warning];
        let before = diagnostics.clone();
        let calls = Cell::new(0);
        let (text, errors) =
            format_with_sources(source.id(), &diagnostics, false, |_| {
                calls.set(calls.get() + 1);
                Err(FileError::Other("source slots lock is poisoned".into()))
            });
        assert_eq!(diagnostics, before);
        assert_eq!(calls.get(), 2);
        assert_eq!(
            errors,
            vec![FileError::Other("source slots lock is poisoned".into()); 2]
        );
        for original in
            ["original error", "original hint", "original_call", "original warning"]
        {
            assert!(text.contains(original), "missing {original}: {text}");
        }
        assert!(text.find("original error") < text.find("original warning"));
        assert!(!text.contains("invented.typ"));
    }

    #[test]
    fn metadata_missing_identity_is_distinct_from_acquisition_failure() {
        let source = Source::detached("x").expect("source fixture");
        let diagnostics = [SourceDiagnostic::error(
            Span::from_range(source.id(), 0..1),
            "unknown source",
        )];
        let (text, errors) =
            format_with_sources(source.id(), &diagnostics, false, |_| Ok(None));
        assert!(errors.is_empty());
        assert!(text.contains("unknown source"));
    }

    #[test]
    fn metadata_primary_and_imported_source_names_preserve_real_origins() {
        let world = SystemWorld::for_stdin("/actual", "original".into());
        let source = world.source(world.main()).expect("stdin fixture");
        let diagnostics =
            [SourceDiagnostic::error(Span::from_range(source.id(), 0..1), "origin")];
        let (normal, errors) =
            prepare(&world, &diagnostics, Path::new("unused.typ"), false);
        assert!(errors.is_empty());
        assert!(normal.contains("actual/<stdin>"));
        assert!(!normal.contains("unused.typ"));
        let (primary, errors) = prepare_with_primary(
            &world,
            &diagnostics,
            Path::new("<input-expression>"),
            false,
            Some(&source),
        );
        assert!(errors.is_empty());
        assert!(primary.contains("<input-expression>"));
        assert!(!primary.contains("actual/<stdin>"));
    }
    #[test]
    fn metadata_registered_missing_source_is_acquisition_error() {
        let root = std::env::temp_dir()
            .join(format!("tekt-metadata-missing-{}", std::process::id()));
        std::fs::create_dir_all(&root).unwrap();
        let world = SystemWorld::for_eval(&root);
        let id = world.register_file(root.join("missing.typ")).unwrap();
        let diagnostics = [SourceDiagnostic::error(
            Span::from_range(id, 0..1),
            "original missing-source cause",
        )];
        let (text, errors) =
            prepare(&world, &diagnostics, Path::new("unrelated.typ"), false);
        assert_eq!(errors, vec![FileError::NotFound]);
        assert!(text.contains("original missing-source cause"));
        assert!(!text.contains("unrelated.typ"));
        std::fs::remove_dir(root).unwrap();
    }
    #[test]
    fn metadata_unresolved_trace_mode_is_selected_per_diagnostic() {
        let src = Source::detached("x").expect("source fixture");
        let span = Span::from_range(src.id(), 0..1);
        let mut diag = SourceDiagnostic::error(span, "original");
        diag.trace.push(Spanned::new(Tracepoint::Call(None), span));
        let calls = Cell::new(0);
        let (text, errors) =
            format_with_sources(src.id(), &[diag.clone(), diag.clone()], false, |_| {
                let call = calls.get();
                calls.set(call + 1);
                if call == 0 {
                    Err(FileError::NotFound)
                } else {
                    Ok(None)
                }
            });
        assert_eq!(errors, vec![FileError::NotFound]);
        assert_eq!(
            text,
            format!(
                "{}{}",
                format_diagnostic_with_unresolved_traces(&diag, &[], false),
                format_diagnostic(&diag, &[], false)
            )
        );
        assert_eq!(text.matches("source unavailable").count(), 1);
    }
}

#[cfg(test)]
mod stderr_tests {
    use super::*;
    use std::io::{self, Write};

    struct Sink {
        bytes: Vec<u8>,
        calls: usize,
        flushes: usize,
        write_error: bool,
        flush_error: bool,
        interrupted: bool,
    }
    impl Write for Sink {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            self.calls += 1;
            if self.interrupted {
                self.interrupted = false;
                return Err(io::ErrorKind::Interrupted.into());
            }
            if self.write_error {
                return Err(io::Error::from_raw_os_error(28));
            }
            let n = bytes.len().min(2);
            self.bytes.extend_from_slice(&bytes[..n]);
            Ok(n)
        }
        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.flush_error {
                Err(io::Error::from_raw_os_error(32))
            } else {
                Ok(())
            }
        }
    }
    fn sink() -> Sink {
        Sink {
            bytes: vec![],
            calls: 0,
            flushes: 0,
            write_error: false,
            flush_error: false,
            interrupted: false,
        }
    }
    #[test]
    fn stderr_write_preserves_bytes_across_short_and_interrupted_writes() {
        let mut out = sink();
        out.interrupted = true;
        let text = "original error\n = hint: café\n  while calling function\n";
        write_stderr_to(&mut out, text).expect("healthy short-write fixture");
        assert_eq!(out.bytes, text.as_bytes());
        assert!(out.calls > 2);
        assert_eq!(out.flushes, 1);
    }
    #[test]
    fn stderr_write_returns_original_write_and_flush_errors() {
        let mut out = sink();
        out.write_error = true;
        let error = write_stderr_to(&mut out, "original").expect_err("write must fail");
        assert_eq!(error.raw_os_error(), Some(28));
        assert_eq!(out.flushes, 0);
        assert!(out.bytes.is_empty());
        let mut out = sink();
        out.flush_error = true;
        let error = write_stderr_to(&mut out, "original").expect_err("flush must fail");
        assert_eq!(error.raw_os_error(), Some(32));
        assert_eq!(out.bytes, b"original");
        assert_eq!(out.flushes, 1);
    }
    #[test]
    fn stderr_empty_message_does_not_touch_unavailable_sink() {
        let mut out = sink();
        out.write_error = true;
        out.flush_error = true;
        write_stderr_to(&mut out, "").expect("no output means no resource operation");
        assert_eq!((out.calls, out.flushes), (0, 0));
    }
}
