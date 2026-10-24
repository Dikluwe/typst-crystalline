# L0 — compiler/eval/repr/math
Hash do Código: e88e9ef8


Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/content/symbol-identity.toml sha256:5060f0989415e92d54eab2a82afd84ba11472aef635d15830f9fb6ee3469aea9
- 00_nucleo/prompts/_nuclei/math-attach-slot-presence.toml sha256:cf64af6ef56faaa1ba19fa32ec6c134dc3f4f16a4a74fde21eca40259cf3b6ba


## Ownership and evidence

Layer L1. Exclusive productive consumer: `01_core/src/compiler/eval/repr/math.rs`. At HEAD `6b147bd213e07e1dc896a8e798b84bf4d8dbbdcb` with a clean tree, `01_core/src/compiler/eval/repr.rs:571-672 e 1374-1466` contains the separated behavior. SHA-256, time and state are recorded in `lab/results/construction/diagnostics/atomizacao-repr-2026-09-30-before.json.gz`. The purpose is to make algorithms discoverable by behavior, with no presumed performance gains.

## Contract

Format mathematical helper fields, Attach, canonical Binom and matrices with identity, order and causal presence. Do not infer fields from derived geometry or substitute one mathematical constructor for another.

The detailed contract and historical parity evidence remain in `00_nucleo/prompts/core/compiler/eval/repr.md`; extraction preserves output, public signatures and phase. No I/O or mutable global state is permitted.

## Acceptance

Preserve language morphology tests, build, and pass `crystalline-lint .` with a valid owner and hash.


## Explicit domain guards

### Evidence before decision

T034 is at compiler/eval/repr/math.rs:167. Current sources/L0 were read before
prescription; `lab/results/construction/diagnostics/tekt-guards-2026-10-16/before.json.gz`
retains SHA-256, HEAD `06848cdcb5450f2c555626bf116bc7acd82bd82b`, uncommitted working
tree, UTC, status and git diff HEAD --stat. The previous hash-reconfirmed
inventory has 28 guards in 21 owners; the pinned CI rule requires splitting
or moving the condition into the body. This is Rust structure, not functional
failure or measured AI-reading improvement. Infer no upstream intention.

### Exclusive contract and acceptance

Unify Some(augment): the body chooses an integer when hline is empty, vline has
exactly one item and stroke is absent, retaining current order; other cases
preserve dictionary/repr of offsets/stroke. None remains none and explicit bits
stay unchanged.

This internal ADR-0127 correction adds no public contract/default/compatibility/
phase change. Only this exclusive consumer is authorized. No helper merely to
hide the conjunction, suppression or linter-configuration change. Run existing
functional controls before code with unchanged expectations. Real RED is V17's
28-warning gate; GREEN requires zero V17. Verify success/fallback/error,
short-circuiting and effects, then restricted owner resealing. Validate build,
full workspace suite, all-targets Clippy with warnings/unfulfilled expectations
as errors, fmt, Rustdoc warnings as errors, strict/reverse V1/V5/V15/V26 lineage
and SARIF without new warnings, under 12 GiB, zero swap and one job. No memory/
performance/global-parity claim without independent measurement.

## Approved batch E integration

Measurement: pre-E inputs and bilateral reflection in
`lab/results/symbol-identity-2026-10-23/{baseline.json,before/receipt.json,additional/receipt.json}`.
The owner approved the native Symbol declaration on 2026-10-23.
Handle Symbol explicitly as a native childless textual leaf in exhaustive complements and existing projections. Preserve the complete Unicode payload and reconstruct the same variant when mapping. Do not silently classify Symbol as a bare equation outside math.
Preserve the existing unit responsibility and all other variants/effects; no public contract beyond the approved Content addition or phase move.
