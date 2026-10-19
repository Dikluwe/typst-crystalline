# L0 — core/compiler/stdlib/primitives_constructors
Hash do Código: 4ff17d71

Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/stdlib/primitive-calls.toml sha256:b7a180bc63908fdeadc405b120c0db8b1c2587f2217d815980b9db35f0e282c4

Layer: L1
Exclusive consumer: `01_core/src/compiler/stdlib/primitives_constructors.rs`

## Evidence and classification

`01_core/src/compiler/stdlib/primitives_constructors.rs:1` (@prompt) identifies the current owner and its local entrypoints. This is implementation inspection, not a new bilateral parity measurement or an inference of upstream intent. The historical decision evidence and its original snapshot qualifications are retained in `lab/results/construction/diagnostics/sanitization-parser-stdlib-history.md`.

## Contract and ownership

Own only private declarations of array, decimal, duration and version, direct native reexports, internal version discovery/dispatch and the array-from-bytes bridge. Preserve native_decimal/native_duration/native_version public reexports and pub(crate) visibility for native_array_bytes, version_type_field and dispatch_version_method.

Test support remains cfg(test), with its existing synthetic Args/EvalContext/FileId/NullWorld fixtures. Individual constructor shape, conversion, index rules and diagnostics belong to child owners under `00_nucleo/prompts/core/compiler/stdlib/primitives_constructors/`. The primitive-call nucleus remains pinned and unchanged; version.at is a method, not a constructor obligation. This façade adds no cast, wrapper, lookup, registry, external API or I/O.

## Acceptance

Preserve the specified language semantics, syntax, morphology, diagnostics, spans, defaults and public identities. Do not change signatures, compatibility, feature defaults or phase ownership through this documentary revision. Shared pinned claims remain authoritative; this prompt owns only its exclusive consumer. Architectural decisions follow ADR-0107, ADR-0108, ADR-0109, ADR-0127, ADR-0129 and ADR-0130.
