# L0 — Human source diagnostics
Hash do Código: 7ad13784

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

## Checked rendering safety

The measured public-span boundary is recorded in
`lab/results/diagnostic-formatter-2026-10-22/MEASUREMENT.md`. A principal range
is renderable only when its source exists, `start <= end`, both endpoints are no
greater than the source byte length, and both endpoints are UTF-8 character
boundaries. Otherwise omit only the location block while retaining severity,
message, hints, and traces. Never clamp, reorder, or move an invalid range to a
plausible location.

`codespan-reporting::term::emit` is fallible even with an in-memory writer. If it
rejects the prepared files/range, preserve the same detached principal form:
`{severity}: {message}\n`, all ` = hint: {hint}\n` lines in order, then the
usual blank line. Continue appending trace output under the existing rules. Do
not expose a fabricated location, return an empty string, suppress the original
diagnostic, retry an external sink, or panic. If the dependency unexpectedly
produces invalid UTF-8 bytes, use replacement characters while preserving all
valid bytes; the public formatter remains total and keeps its existing `String`
signature.

This is an internal safety correction under ADR-0127. It adds no mode, flag,
public signature, entity, phase, I/O channel, or compatibility break.

## Acceptance

Cover caret/source, warning/error, unspanned hints, tab width, Unicode, multiline/detached spans, all trace kinds, another source's trace, missing-source omission and ANSI equivalence. Cover unordered, out-of-bounds, and mid-codepoint principal ranges with exact retained text and no panic or fabricated location. Preserve the unresolved multiline obligation explicitly. Short/JSON/SARIF formats, a public diagnostic-format flag, spanned hints in the domain and changes to L1 message literals are outside this contract.


## Fallible Source and file-registration integration

Human approval on 2026-10-22 is retained in
`lab/results/panic-elimination-2026-10-22/source-contract-approval.json`;
its review SHA-256 is `2c879e8afa66721a8952dc8b9e7cf3c92dce58bf91aae5e28da5d603f5aa4d42`.
Source::{new,new_with_parser,detached,detached_with_parser} now return the existing
SourceResult<Source>; SystemWorld::register_file returns FileResult<FileId>.
Preserve this unit's successful content/id/span/order/diagnostic behavior while
propagating construction/registration failures through its existing error channel.
A tuple-returning entry returns its error in the existing result component and
retains already accumulated warnings. A synthetic source slot stores a failed
construction as FileResult rather than inventing a Source. Test-only fixture
helpers must explicitly require their valid construction/registration result;
all previous substantive assertions remain. No productive unwrap/expect, panic
catch, default Source, identity reuse or silent error is permitted for this migration.
This clause supersedes only the old infallible return/panic expectations for the
five approved APIs; it does not authorize unrelated signatures, fields, phases
or broadening/loss of existing diagnostics or negative controls.


## Approved additive unresolved-trace formatter — approved 2026-10-22

Add exactly this public function, preserving all existing signatures:

```rust
pub fn format_diagnostic_with_unresolved_traces(
    diag: &SourceDiagnostic,
    sources: &[DiagnosticSource],
    colored: bool,
) -> String;
```

The existing format_diagnostic keeps every current behavior, including omission
of an unresolved trace. Both functions share a private implementation; ordinary
source-backed labels, hints, trace kinds, snippets, order, color and whitespace
remain identical. The new function emits an unresolved trace as exactly
`  {kind} (source unavailable)\n`, using the same five existing kind literals.
Never invent a FileId, path, location or snippet; preserve every trace in stored
order. A retained unresolved trace participates in the final trace blank line.
No World, path I/O, global mode, flag, public field or error entity is added.
Keep the existing missing-source omission and all golden/ANSI tests for the old
API. Add exact unresolved/mixed/anonymous/detached/all-kind/color controls for the
new API and equivalence controls when every trace is resolvable. Existing panic
assumptions of the underlying codespan/UTF8 formatter are outside this change;
no additional unwrap/expect/panic path or suppression is permitted.

Approval receipt: `lab/results/world-metadata-errors-2026-10-22/trace-format-approval.json`; review SHA-256 `c7507b43112d46931bdafb95c26bd83f4fdc605e543911a163ca112c0c95cfb8`.
