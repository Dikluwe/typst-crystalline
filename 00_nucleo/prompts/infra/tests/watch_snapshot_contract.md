# L0 — External watch snapshot contract tests
Hash do Código: fd064303

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/watch_snapshot_contract.rs`

## Current evidence

- `03_infra/tests/watch_snapshot_contract.rs` — transition controls, type checks and pinned rustdoc JSON inspection.
- `03_infra/src/watch.rs` — public product owned by `00_nucleo/prompts/infra/watch.md`.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own this integration test only. A captured snapshot must precede the stimulus; `wait_for_change_since` receives that token. Preserve the legacy `wait_for_change` API. Observe public behavior without fixing private fields, layout, fingerprint algorithm or collection representation.

Each case uses its own temporary directory and RAII cleanup. Detect equal-length content replacement, creation of an initially absent path and removal of an initially present path after capture. Type-check the three snapshot/wait entrypoints. On Linux, retain the public pseudo-file control whose content changes between reads.

Complete each transition before starting the waiting thread. `recv_timeout` bounds failure; it is not readiness evidence. Do not add sleep, retry, artificial load or a corrective second mutation.

Pinned-toolchain rustdoc JSON must show public WatchSnapshot with nonempty private state, no public fields/inherent items/direct traits, and only snapshot/wait_for_change_since among public functions whose signatures reference the token. Accept named or tuple representation without choosing field names/types. Missing identity, unsupported version/schema or invalid JSON fails the test.

## Verification boundary

Verify transitions, signatures and public inventory. This test does not prove general watch equivalence or replace the CLI process suite.
