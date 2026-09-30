# L0 — infra/plugin_host/tests
Hash do Código: 047e42c9

**Layer:** L3
**Exclusive consumer:** `03_infra/src/plugin_host/tests.rs`

## Current evidence

- `03_infra/src/plugin_host/tests.rs:17` — `uleb`.
- `03_infra/src/plugin_host/tests.rs:32` — `sleb`.
- `03_infra/src/plugin_host/tests.rs:46` — `vbytes`.
- `03_infra/src/plugin_host/tests.rs:52` — `name`.
- `03_infra/src/plugin_host/tests.rs:56` — `section`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/plugin_host/tests.rs`.

## Owned contract

- Own only the unit suite and deterministic binary WASM encoder, with no external fixture generator or WASM toolchain requirement.
- Keep Send+Sync, hello/output, function exports, missing-memory/binary failure, signature/arity, traps, bounds read/write, UTF-8 and invalid protocol cases. Invalid-binary runtime detail is checked only to the existing stable prefix.
- Test transition memory persistence, original-entry isolation, chaining and failure without derived state. Exact host messages are observables; fixtures/encoder are not production host logic or evidence of current upstream parity.

## Verification and boundary

Run `cargo test -p typst-infra --lib plugin_host::tests -- --test-threads=1` when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
