//! Crystalline Lineage
//! @prompt 00_nucleo/prompts/shell/diagnostic.md
//! @prompt-hash 92ea4e96
//! @layer L2
//! @updated 2026-08-23
//!
//! Formatter humano vanilla-espelhado (P1139; adendo à ADR-0045).
//! L4 materializa as fontes; L2 limita-se à apresentação em memória.

use typst_core::entities::file_id::FileId;
use typst_core::entities::source::Source;
use typst_core::entities::source_result::{Severity, SourceDiagnostic};

struct DiagnosticFiles<'a> {
    sources: &'a [DiagnosticSource],
}

impl<'a> codespan_reporting::files::Files<'a> for DiagnosticFiles<'a> {
    type FileId = FileId;
    type Name = String;
    type Source = String;

    fn name(&self, id: FileId) -> Result<Self::Name, codespan_reporting::files::Error> {
        self.sources
            .iter()
            .find(|item| item.source.id() == id)
            .map(|item| item.name.clone())
            .ok_or(codespan_reporting::files::Error::FileMissing)
    }

    fn source(
        &self,
        id: FileId,
    ) -> Result<Self::Source, codespan_reporting::files::Error> {
        self.sources
            .iter()
            .find(|item| item.source.id() == id)
            .map(|item| item.source.text().to_owned())
            .ok_or(codespan_reporting::files::Error::FileMissing)
    }

    fn line_index(
        &self,
        id: FileId,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        let source = self.source(id)?;
        if byte_index > source.len() {
            return Err(codespan_reporting::files::Error::IndexTooLarge {
                given: byte_index,
                max: source.len(),
            });
        }
        let starts = codespan_reporting::files::line_starts(&source).collect::<Vec<_>>();
        Ok(match starts.binary_search(&byte_index) {
            Ok(line) => line,
            Err(next) => next.saturating_sub(1),
        })
    }

    fn line_range(
        &self,
        id: FileId,
        line_index: usize,
    ) -> Result<std::ops::Range<usize>, codespan_reporting::files::Error> {
        let source = self.source(id)?;
        let starts = codespan_reporting::files::line_starts(&source).collect::<Vec<_>>();
        let Some(&start) = starts.get(line_index) else {
            return Err(codespan_reporting::files::Error::LineTooLarge {
                given: line_index,
                max: starts.len().saturating_sub(1),
            });
        };
        let end = starts.get(line_index + 1).copied().unwrap_or(source.len());
        Ok(start..end)
    }

    fn column_number(
        &self,
        id: FileId,
        line_index: usize,
        byte_index: usize,
    ) -> Result<usize, codespan_reporting::files::Error> {
        let source = self.source(id)?;
        let range = self.line_range(id, line_index)?;
        Ok(codespan_reporting::files::column_index(&source, range, byte_index))
    }
}

/// Fonte já materializada por L4 para apresentação em L2.
#[derive(Debug, Clone)]
pub struct DiagnosticSource {
    source: Source,
    name: String,
}

impl DiagnosticSource {
    pub fn new(source: Source, name: impl Into<String>) -> Self {
        Self { source, name: name.into() }
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

/// Formata um `SourceDiagnostic` no formato humano do vanilla ratificado.
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    sources: &[DiagnosticSource],
    colored: bool,
) -> String {
    use codespan_reporting::diagnostic::{Diagnostic, Label};
    use codespan_reporting::term;
    use codespan_reporting::term::termcolor::Buffer;

    let files = DiagnosticFiles { sources };
    let find = |id: FileId| sources.iter().find(|item| item.source.id() == id);

    let label = diag.span.id().and_then(|id| {
        let item = find(id)?;
        let range = item.source.span_byte_range(diag.span)?;
        (range.end <= item.source.text().len()).then(|| Label::primary(id, range))
    });
    let diagnostic = match diag.severity {
        Severity::Error => Diagnostic::error(),
        Severity::Warning => Diagnostic::warning(),
    }
    .with_message(diag.message.clone())
    .with_notes(diag.hints.iter().map(|hint| format!("hint: {hint}")).collect())
    .with_labels(label.into_iter().collect());

    let mut buffer = if colored { Buffer::ansi() } else { Buffer::no_color() };
    let config = term::Config { tab_width: 2, ..Default::default() };
    term::emit(&mut buffer, &config, &files, &diagnostic)
        .expect("fontes e ranges de diagnóstico foram validados");
    let mut out = String::from_utf8(buffer.into_inner())
        .expect("codespan-reporting produz UTF-8 válido");

    let mut traced = false;
    for point in &diag.trace {
        let Some(id) = point.span.id() else { continue };
        let Some(item) = find(id) else { continue };
        let source = &item.source;
        let Some((line, col)) = source.span_to_line_col(point.span) else {
            continue;
        };
        let kind = match &point.v {
            typst_core::entities::source_result::Tracepoint::Call(Some(name)) => {
                format!("while calling `{name}`")
            }
            typst_core::entities::source_result::Tracepoint::Call(None) => {
                "while calling function".to_string()
            }
            typst_core::entities::source_result::Tracepoint::Show(name) => {
                format!("while showing {name} element")
            }
            typst_core::entities::source_result::Tracepoint::Import(name) => {
                format!("while importing `{name}`")
            }
            typst_core::entities::source_result::Tracepoint::Include(name) => {
                format!("while including `{name}`")
            }
        };
        out.push_str(&format!("  {kind} at {}:{line}:{col}\n", item.name));

        let Some(range) = source.span_byte_range(point.span) else { continue };
        let Some(text) = source.text().get(range) else { continue };
        let mut lines = text.lines();
        let first = lines.next().unwrap_or("");
        let mut snippet = first.to_string();
        if let Some(last) = lines.next_back() {
            if let Some(last_char) = last.chars().next_back() {
                if !last_char.is_whitespace() {
                    snippet = format!("{first}…{last_char}");
                }
            }
        }
        out.push_str(&format!("    {snippet}\n"));
        traced = true;
    }
    if traced {
        out.push('\n');
    }

    out
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU16;

    use super::*;
    use typst_core::entities::file_id::FileId;
    use typst_core::entities::source_result::Tracepoint;
    use typst_core::entities::span::{Span, Spanned};

    fn source(id: u16, text: &str) -> Source {
        Source::new(FileId::from_raw(NonZeroU16::new(id).unwrap()), text.to_string())
    }

    fn strip_ansi(input: &str) -> String {
        let mut output = String::new();
        let mut chars = input.chars().peekable();
        while let Some(ch) = chars.next() {
            if ch == '\x1b' && chars.peek() == Some(&'[') {
                chars.next();
                for control in chars.by_ref() {
                    if control.is_ascii_alphabetic() {
                        break;
                    }
                }
            } else {
                output.push(ch);
            }
        }
        output
    }

    #[test]
    fn p1139_human_source_and_caret() {
        let src = source(2, "#let x = (");
        let diag = SourceDiagnostic::error(
            Span::from_range(src.id(), 9..10),
            "unclosed delimiter",
        );
        let out =
            format_diagnostic(&diag, &[DiagnosticSource::new(src, "input.typ")], false);
        assert_eq!(
            out,
            "error: unclosed delimiter\n  ┌─ input.typ:1:9\n  │\n1 │ #let x = (\n  │          ^\n\n"
        );
    }

    #[test]
    fn p1139_hints_are_notes() {
        let diag = SourceDiagnostic::warning(Span::detached(), "careful")
            .with_hint("first")
            .with_hint("second");
        let out = format_diagnostic(&diag, &[], false);
        assert_eq!(out, "warning: careful\n = hint: first\n = hint: second\n\n");
    }

    #[test]
    fn p1139_trace_resolves_second_source() {
        let broken = source(2, "#unknown");
        let caller = source(3, "#include \"broken.typ\"");
        let mut diag = SourceDiagnostic::error(
            Span::from_range(broken.id(), 1..8),
            "unknown variable `unknown`",
        );
        diag.trace.push(Spanned::new(
            Tracepoint::Include("broken.typ".into()),
            Span::from_range(caller.id(), 1..21),
        ));
        let out = format_diagnostic(
            &diag,
            &[
                DiagnosticSource::new(broken, "broken.typ"),
                DiagnosticSource::new(caller, "main.typ"),
            ],
            false,
        );
        assert!(out.contains("┌─ broken.typ:1:1"), "{out}");
        assert!(out.contains("while including `broken.typ` at main.typ:1:1"), "{out}");
    }

    #[test]
    fn p1139_detached_does_not_invent_location() {
        let diag = SourceDiagnostic::error(Span::detached(), "boom");
        assert_eq!(format_diagnostic(&diag, &[], false), "error: boom\n\n");
    }

    #[test]
    fn p1139_missing_source_is_omitted_without_panic() {
        let src = source(2, "x");
        let diag = SourceDiagnostic::error(Span::from_range(src.id(), 0..1), "boom");
        assert_eq!(format_diagnostic(&diag, &[], false), "error: boom\n\n");
    }

    #[test]
    fn p1139_colored_preserves_semantics() {
        let src = source(2, "x");
        let diag = SourceDiagnostic::warning(Span::from_range(src.id(), 0..1), "careful");
        let source = DiagnosticSource::new(src, "input.typ");
        let colored = format_diagnostic(&diag, std::slice::from_ref(&source), true);
        let plain = format_diagnostic(&diag, &[source], false);
        assert!(colored.contains("\x1b["));
        assert_eq!(strip_ansi(&colored), plain);
    }

    #[test]
    fn p1139_tab_and_unicode_keep_public_column_and_caret() {
        let src = source(2, "\táx");
        let diag = SourceDiagnostic::error(Span::from_range(src.id(), 3..4), "boom");
        let out =
            format_diagnostic(&diag, &[DiagnosticSource::new(src, "input.typ")], false);
        assert!(out.contains("input.typ:1:2"), "{out}");
        assert!(out.contains("1 │   áx\n  │    ^"), "{out}");
    }

    #[test]
    fn p1139_trace_multiline_uses_ellipsis() {
        let src = source(2, "#f(\n  1\n)\n");
        let mut diag = SourceDiagnostic::error(Span::detached(), "boom");
        diag.trace.push(Spanned::new(
            Tracepoint::Call(Some("f".into())),
            Span::from_range(src.id(), 1..9),
        ));
        let out =
            format_diagnostic(&diag, &[DiagnosticSource::new(src, "input.typ")], false);
        assert!(out.contains("  while calling `f` at input.typ:1:1\n    f(…)\n"));
    }
}
