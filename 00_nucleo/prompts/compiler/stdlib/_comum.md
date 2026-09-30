# L0 — stdlib/_comum
Hash do Código: 425bacc9

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/mod.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/mod.rs:212` (pubsuper) fn errmsg: impl Into<String>) -> SourceResult<Value> {); `01_core/src/compiler/stdlib/mod.rs:221` (pubsuper) fn expect_no_named) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `00_nucleo/diagnosticos/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own the stdlib façade: module declarations, explicit reexports, local err/expect_no_named helpers and cfg(test) test-module composition. Preserve established Rust public/internal paths and their specified visibility, with discrepancies explicitly retained below; do not create wrappers, duplicate namespace factories or move algorithms into the hub.

`err` produces one Error diagnostic with a detached span. `expect_no_named` rejects the first unknown named key in insertion order with the existing literal `argumento nomeado inesperado: 'KEY'` and a detached span. These helpers do not replace owner-specific origin-sensitive validation.

The actual native ABIs are the current Func native/Engine interfaces; the historical shortened signature is not an API specification. Per-native casts, diagnostics, return values and World use belong to their owners. Numeric helpers and promotion rules are not implemented here.

Registry composition belongs to `00_nucleo/prompts/compiler/stdlib/registry/mod.md`; eval keeps its own façade/call dispatch. Preserve the direct linkage of make_stdlib_with_features, calc_abs, format factory, collection call-span metadata, float/int/version dispatch, array-from-bytes, codec encoders, outline entry operations, spot construction and all existing native reexports. Do not globally register aliases from this façade.

Constructor hubs `00_nucleo/prompts/compiler/stdlib/foundations.md`, `00_nucleo/prompts/compiler/stdlib/text.md`, `00_nucleo/prompts/compiler/stdlib/structural.md` and `00_nucleo/prompts/compiler/stdlib/primitives-constructors.md` own only their composition. Production behavior belongs to the respective descendants; the atomized test suite is owned by `00_nucleo/prompts/compiler/stdlib/tests/mod.md` and its test owners.

The contract requires the native_flush façade reexport to be pub(crate), not an externally public API. The current public reexport conflicts with that requirement. Keep this obligation explicit; the observed code does not supersede it. No Rust visibility change is made or authorized during sanitation, and the discrepancy remains for a separate contract/code resolution.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
