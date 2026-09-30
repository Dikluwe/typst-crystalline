# L0 — wiring/tests/p1291_callback_runtime
Hash do Código: 6eb40e0c

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/p1291_callback_runtime.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/math/callback-realization.toml sha256:3cf8b79e22fcda6466dc38b7691a73ada4540956fbed14129c9e1ed5a6a56c43
```

## Current evidence

- `04_wiring/tests/p1291_callback_runtime.rs:18` — `Scratch`.
- `04_wiring/tests/p1291_callback_runtime.rs:21` — `new`.
- `04_wiring/tests/p1291_callback_runtime.rs:34` — `drop`.
- `04_wiring/tests/p1291_callback_runtime.rs:39` — `diagnostic`.
- `04_wiring/tests/p1291_callback_runtime.rs:48` — `compile`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/p1291_callback_runtime.rs`.

## Owned contract

- Own only math callback black-box integration. Compile isolated fixtures and observe final geometry/diagnostics, not implementation internals or provisional documents.
- Angle callback agrees with explicit-angle control; lexical text and derived script math sizes remain observable. cross overrides inverted in two-line morphology. Invalid returns preserve call path/line, fail without panic and produce no output.
- Use local cleaned fixtures. Unknown is not success; vec/underline generalization is outside this suite. Keep the pinned callback-realization obligation without authorizing productive changes.

## Verification and boundary

Run `cargo test -p typst --test p1291_callback_runtime -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
