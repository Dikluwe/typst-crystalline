# L0 — infra/plugin_host/tests
Hash do Código: 207daaf4

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


## Private encoder constant maintenance

### Evidence

`lab/results/construction/diagnostics/clippy-locais-2026-10-16/before.json.gz`
retains sources/L0/hashes/HEAD/uncommitted tree/UTC/diffstat for tests.rs:14.
Consumer/use inspection preceded deciding; selected Clippy gate must fail before
code changes. This is Rust representation/observation, no new upstream intent/parity.

### Contract and acceptance

Remove only unused private I64 from the WASM fixture encoder. Preserve fixture
bytes/opcodes and all tests, including signature rejections; this says nothing
about product I64 support. Continuous ADR-0127 adds no public/default/phase
contract or lint suppression. L0 first, scoped resealing, exact inventory reduction/
no new warnings, Clippy RED→GREEN/unfulfilled_lint_expectations as error, workspace
suite/build/fmt/Rustdoc/lineage under 12 GiB, zero swap, one job. No derive/Entry/
module-placement mirror tests. The historical bbox requirement still needs real
observation in its own scope; other controls retain fixtures/expectations.
