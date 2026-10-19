# L0 — Human source diagnostics
Hash do Código: 0096c215

**Layer:** L2
**Exclusive consumer:** `02_shell/src/diagnostic.rs`

## Evidence and boundary

`02_shell/src/diagnostic.rs:92` defines memory-only sources and `:112` formats diagnostics. This is presentation in L2 under ADR-0045's human-format amendment and ADR-0050. Source collection/channel publication belong to `00_nucleo/prompts/wiring/diagnostics.md`; color resolution belongs to `00_nucleo/prompts/shell/cli.md`.

Diagnostic text and displayed locations are observables. No inference from Rust equality or renderer implementation establishes language parity; source-backed exact text tests are appropriate for these observables.

## Public API

```rust
pub struct DiagnosticSource { /* private Source and String fields */ }
impl DiagnosticSource {
    pub fn new(source: Source, name: impl Into<String>) -> Self;
    pub fn source(&self) -> &Source;
    pub fn name(&self) -> &str;
}
pub fn format_diagnostic(
    diag: &SourceDiagnostic,
    sources: &[DiagnosticSource],
    colored: bool,
) -> String;
```

A source ID is its identity; the caller-supplied name is presentation only. There is no operational path, World or loading callback.

## Resolution and principal block

Match each principal/trace span ID against the supplied sources. The first matching FileId wins; callers should deduplicate. Missing sources and unresolvable spans omit their corresponding label/trace rather than inventing a location. The formatter never loads sources.

Use `codespan-reporting 0.11.1` with `tab_width: 2` and otherwise default configuration. Map error/warning severities directly. Preserve `diag.message` verbatim, a resolvable principal span as the primary label, and every hint as a `hint: {text}` note. Do not fabricate hint spans. Detached diagnostics remain readable without a false location.

The uncolored principal example is:

```text
error: unclosed delimiter
  ┌─ input.typ:1:9
  │
1 │ #let x = (
  │          ^
```

Preserve codespan's newline/blank-line structure. In particular a detached error is `error: boom\n\n`; hint notes retain the ` = hint: ` prefix.

## Trace and snippets

Append resolvable traces in stored order using:

```text
  {kind} at {display_name}:{line}:{column}
    {snippet}
```

Preserve these kind literals; backticks around names are part of output:

```text
while calling `{name}`
while calling function
while showing {name} element
while importing `{name}`
while including `{name}`
```

Resolve positions through Source's span-to-line/column API, not fabricated offsets or regex rewriting.

Snippets come from the trace span itself. The existing multiline requirement describes the first line followed, when applicable, by `…` and the last non-whitespace character. The consumer currently checks only the actual final character of the last line and suppresses the suffix if that character is whitespace. That discrepancy requires a decision; this owner must not silently authorize trimming or drop the existing obligation. See `lab/results/construction/diagnostics/sanitization-shell-history.md`.

Trace lines end with newline; when a snippet was appended, retain the additional trailing blank line.

## Color, purity and scope

`colored = false` uses a non-ANSI writer; true uses codespan's ANSI writer. Stripping ANSI preserves the uncolored words and structure. No environment or TTY detection occurs here. No clap, SystemWorld, filesystem, stderr publication or L3/L4 import is allowed. The termcolor API may be obtained through codespan's reexport.

## Acceptance

Cover caret/source, warning/error, unspanned hints, tab width, Unicode, multiline/detached spans, all trace kinds, another source's trace, missing-source omission and ANSI equivalence. Preserve the unresolved multiline obligation explicitly. Short/JSON/SARIF formats, a public diagnostic-format flag, spanned hints in the domain and changes to L1 message literals are outside this contract.
