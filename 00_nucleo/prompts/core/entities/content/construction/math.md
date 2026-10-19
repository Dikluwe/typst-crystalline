# L0 — entities/content/construction/math
Hash do Código: 0c6baaf0

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/construction/math.rs`

## Evidence before specification

At clean commit `f7195cdf6` (2026-09-30), the extracted behavior was in
`01_core/src/entities/content/construction.rs:167-614`. This establishes the
ownership boundary; it measures neither performance nor new bilateral parity.

## Contract

Mathematical Content constructors preserve payloads, defaults, argument presence, and existing signatures. Preserve public contracts, default behavior, pipeline phase,
compatibility, purity, and static dispatch.

## Acceptance

The L1 suite, build, and `crystalline-lint` must pass. Separation alone is not
proof of a performance improvement.

## Historical record

The original specification and its construction chronology are preserved in
`lab/history/prompt-consolidation-2026-10-18/records/00_nucleo/prompts/core/entities/content/construction/math.md`. Historical observations are not a new parity certification.
