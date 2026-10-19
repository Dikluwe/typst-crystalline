# Prompt L0 — Sink
Hash do Código: 1a0641db

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/sink.rs`
**References:** ADR-0042, ADR-0043, ADR-0017, ADR-0036.

## Contract

Sink collects nonfatal SourceDiagnostic warnings during eval for unsupported,
deprecated or ambiguous constructions. Its private fields are
`diagnostics: Vec<SourceDiagnostic>` and `seen: FxHashSet<(Span, String)>`.
Preserve insertion order and first occurrence. Deduplication uses only span
and message; severity, hints and trace do not participate.

Preserve new()->Self and default() as empty; warn(&mut self, SourceDiagnostic)
adds only a new key; is_empty(&self)->bool; and
into_diagnostics(self)->Vec<SourceDiagnostic>, consuming the sink in order.
No reuse after extraction and no I/O.

The local #[comemo::track] impl is empty; the real API lives in an untracked
impl. Direct integration remains deferred until SourceDiagnostic is hashable
or a tracked wrapper is decided. The tracked interface in
`01_core/src/entities/world_types.rs` maintains eval's TrackedMut<Sink>:
warn_note(span,message,hint) and warn_note2(span,message,hint,hint2) delegate to
record with shared deduplication. Empty hint strings mean absent hints because
the comemo macro does not accept Option<&str>. The two-hint variant supports
the conditional state/counter hint of warn_for_discarded_content.

## Equivalent Default: evidence and acceptance

At `01_core/src/entities/sink.rs:73`, Clippy identified the manual Default;
its fields used their defaults. The nongeneric type adds no public bounds
through derivation. HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b` had an
uncommitted tree; source/owners/spans/UTC/status/diffstat are preserved in
`lab/results/construction/diagnostics/expressoes-defaults-ci-2026-10-13/before.json.gz`.
This is implementation evidence, not new parity or performance measurement.

Use derive(Default) instead of only that manual implementation, preserving the
existing trait/value: empty diagnostics and seen in existing field order.
Preserve Clone, deduplication, ordering and comemo integration. No signature,
field, variant, product default or phase change. ADR-0127 continuous internal
maintenance; preserve tests/expectations and validate workspace all-targets,
core suite, test inventory, build, Rustdoc, Clippy and lineage.

## Pending integration record

The original contract recorded a pending pilot consumer (DEBT-49), eval's
_sink: TrackedMut<Sink>, and incomplete internal propagation. The later
tracked methods establish their own scope but do not, by themselves, certify
closure of every pilot/propagation debt. Keep that integration status explicit
until verified by its owning contract.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/sink.md`. Historical observations are not a new parity certification.
