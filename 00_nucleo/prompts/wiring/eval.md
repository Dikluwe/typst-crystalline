# L0 — wiring/eval
Hash do Código: b263b887

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/eval.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/compiler-feature-gates.toml sha256:59d8938dc06d347ccc9db23ae1b740876b369227daacd266a219811a661b3cb9
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/eval.rs:16` — `run_eval`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/eval.rs`.

## Owned contract

- Own run_eval composition. No input uses minimal World; `-` uses stdin/transient Source; physical input uses current root/main/font/CA setup. Materialize the expression Source for accurate diagnostics.
- With a document call contextual L3 evaluation, otherwise isolated features/sink evaluation. Error diagnostics drain before warnings on Err; on Ok warnings precede L2 serialization/stdout. Preserve each diagnostic once and its origin.
- Semantic/serialization failures exit 1, input/output/argument failures exit 2 and success exits 0. Do not unify this path with query or move evaluation/serialization into wiring.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
