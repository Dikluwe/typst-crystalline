# L0 — wiring/tests/float_is_infinite
Hash do Código: 1dce4f5e

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/float_is_infinite.rs`

## Current evidence

- `04_wiring/tests/float_is_infinite.rs:8` — `eval`.
- `04_wiring/tests/float_is_infinite.rs:16` — `p1289_float_is_infinite_reports_finite_infinite_and_nan`.
- `04_wiring/tests/float_is_infinite.rs:30` — `p1289_float_is_infinite_preserves_public_signature_and_errors`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/float_is_infinite.rs`.

## Owned contract

- Own only public float.is-infinite black-box tests: finite and NaN return false; positive/negative infinity return true.
- Preserve public identity, arity, unknown named arguments and invalid-cast language diagnostics. No runner score, serialized baseline, frozen hash or administrative artifact is an oracle.

## Verification and boundary

Run `cargo test -p typst-wiring --test float_is_infinite -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
