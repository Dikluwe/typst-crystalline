# L0 — stdlib/context
Hash do Código: 14f940b4

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/context/block-semantics.toml sha256:0ede5ade7908670f0440672becaba2aa356acc51346fd590dc426e4788dc6dd0

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/context.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/context.rs:23` (pub fn native_context) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only native_context construction. Reject named arguments, require the existing zero-argument Func body, do not evaluate it immediately, and return the current ContextBlock carrier with its eval-assigned stable id and closure.

The pinned context nucleus defines shared semantics. Contextual expansion, occurrence Location, runtime state/counter reads and pipeline scheduling belong to their actual eval/introspection/pipeline owners, not to this constructor. The former full expansion algorithm was duplicated normative ownership and is removed here, without changing delayed evaluation or authorizing layout-time Func execution.

Preserve the constructor ABI, function identity, arity/type diagnostics and Content morphology. A block must retain its captured closure, not be converted into an eagerly evaluated string. Unresolved layout-metric/nested-context claims are not closed by source inspection.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
