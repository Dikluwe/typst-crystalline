# L0 — Public duration presentation regression tests
Hash do Código: 3659e29b

**Layer:** L2 integration test
**Exclusive consumer:** `02_shell/tests/p1361_duration.rs`

## Evidence and ownership

The consumer's `CASES` at `02_shell/tests/p1361_duration.rs:19` and tests at `:146`, `:170` and `:190` exercise public `format_duration`, `WatchStatus::message` and `format_watch_line`. The productive contract and complete literal vector belong to `00_nucleo/prompts/shell/duration.md`; this owner exclusively legitimizes the integration test, not productive code.

Inputs are literal Durations and timestamp components. The test observes public presentation only: no binding, World, clock, subprocess, watch command, sleep, filesystem or execution-time-derived expectation.

## Contract

`p1361_duration_exact_vector_is_deterministic_and_boundary_safe` checks every duration/output pair specified by the productive owner, including zero and 300.006 seconds. Repeat canonical and reversed order three times, comparing observations by stable case identity.

Distinguish ns/µs/ms/s boundaries, the five-minute truncation transition and the day transition. Preserve the seemingly unusual `4 min 60 s` result immediately below five minutes; do not normalize it into another output.

`p1362_watch_status_messages_are_exact_and_error_has_no_duration` uses literal 2.8492 seconds. Check all four bodies, distinct success/warning prefixes and no duration or ` in ` suffix for Error.

`p1363_watch_line_pads_timestamp_and_preserves_all_status_bodies` uses literal 03:04:05 and the same duration. Check all variants, two-digit padding, one space after `]`, complete preservation of each body and absence of carriage return/newline.

## Acceptance and execution discipline

Run each named test with an exact-name filter, one build job and a bounded runner timeout. A run executing no matching test, timing out or failing from resource exhaustion earns no acceptance credit. Runtime limits are failure bounds, never input or evidence for timing behavior.

Require direct literal observations and deterministic repeat/reverse controls. Do not claim independent pre-implementation attestation from this test's existence: its lineage-order limitation is retained in `00_nucleo/diagnosticos/sanitization-shell-history.md`.
