# L0 — External armed watch capability tests
Hash do Código: d064eb94

**Layer:** L3 (integration tests)
**Exclusive consumer:** `03_infra/tests/armed_watch_contract.rs`

## Current evidence

- `03_infra/tests/armed_watch_contract.rs` — armed transitions and public-surface inspection.
- `03_infra/src/watch.rs` — consuming publication/abandonment methods.
- `04_wiring/src/watch.rs` — the current composition cycle.

These are current local source anchors, not a new upstream parity measurement.

## Contract

Own this external test only. The productive API belongs to `00_nucleo/prompts/infra/watch.md`; the cycle belongs to `00_nucleo/prompts/wiring/watch.md`. Watching the staging path must demonstrate capture in arm before publication/abandonment; observing its absence only afterwards is insufficient.

`arm: fn(&[PathBuf]) -> ArmedWatch`. File publish consumes the token and renames staging to destination, preserving bytes and returning the pretransition WatchSnapshot. File abandon consumes the token, removes staging and returns the earlier snapshot without changing the last valid destination. Preserve the corresponding consuming directory operations for bundle output. Rename failure preserves ErrorKind/raw_os_error and attempts best-effort cleanup; cleanup failure cannot mask the original error.

Type-check snapshot/wait functions and the consuming capability signatures. Complete transitions before starting the waiter; timeout bounds failure, not readiness. Do not use sleep, retry, artificial load or corrective stimuli.

Pinned rustdoc JSON must show public ArmedWatch with private state and no public fields. Its inherent public method allowlist is exactly `abandon`, `abandon_directory`, `publish`, `publish_directory`. `arm` is the only public free function referencing ArmedWatch; `commit_output` and `discard_output` are absent. Private representation/fingerprint remains unconstrained.

## Verification boundary

Verify successful file/directory transitions, original-error retention, signatures and exact public inventory. This test does not replace the separate snapshot or CLI suite.
