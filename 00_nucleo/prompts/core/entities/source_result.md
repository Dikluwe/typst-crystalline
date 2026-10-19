# Prompt L0 — entities/source_result
Hash do Código: 6f1ac9a2

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/source_result.rs`

## Contract

SourceResult<T> transports compiler results; SourceDiagnostic represents an
error or warning; Tracepoint represents an error-stack point. These are pure
Span/message/hints data with no I/O. Preserve the interface:

```rust
pub enum Severity { Error, Warning }

pub enum Tracepoint {
    Call(Option<String>),
    Show(String),
    Import(String),
    Include(String),
}

pub struct SourceDiagnostic {
    pub severity: Severity,
    pub span:     Span,
    pub message:  String,
    pub hints:    Vec<String>,
    pub trace:    Vec<Spanned<Tracepoint>>,
}

impl SourceDiagnostic {
    pub fn error(span: Span, message: impl Into<String>) -> Self
    pub fn warning(span: Span, message: impl Into<String>) -> Self
    pub fn with_hint(self, hint: impl Into<String>) -> Self
}

pub type SourceResult<T> = Result<T, Vec<SourceDiagnostic>>;
```

Use String instead of the historical EcoString, Vec<SourceDiagnostic> instead
of EcoVec, Vec<String> hints instead of spanned EcoString hints, and
Vec<Spanned<Tracepoint>> traces instead of EcoVec. Adding spans to hints was
previously deferred to evaluator migration; it remains pending rather than
silently authorized by this cleanup.

SourceDiagnostic has structural field equality/hash for transport through the
internal tracked warning channel. This changes neither messages, severity,
spans, hints, traces, nor deduplication by (span,message).

## Acceptance

error(Span::detached(), "msg") has Severity::Error;
error(...).with_hint("h") has hints vec!["h"].
SourceResult::Err(vec![diag]).is_err() is true, and
SourceResult::Ok(42u32).unwrap() is 42. Preserve warning construction and the
existing public field/return types.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/source_result.md`. Historical observations are not a new parity certification.
