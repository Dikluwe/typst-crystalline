# L0 — Projected-font buffer semantic regressions
Hash do Código: 93aeee6d

**Layer:** L3 (tests only)
**Exclusive consumer:** `03_infra/src/fontdb/selective/single_reserve_tests.rs`

## Evidence before the test contract

The owning acquisition contract is `00_nucleo/prompts/infra/fontdb/selective.md`.
Its C23 lab has independent stable-input projection and adversarial controls,
including a post declaration beyond EOF whose abbreviated prefix still fits.
The original projector is an additional mechanical oracle, not language intent.

## Contract

Own cfg(test) child regressions over the real private projector. Preserve the lab's
stable projected bytes/requested-count oracle and explicit synthetic expectations.
Cover directory order, duplicates, omitted invalid payloads, empty selected tables,
sfnt/TTC signatures/indices/absolute offsets, post versions/prefixes/counts/full
declarations, record-count and complete-buffer boundaries, Reader budget/overflow,
short/Interrupted/EOF/read/seek failures, real metadata/exact coverage, successful
empty coverage versus decline and File fallback through the parent.

Use the six existing licensed fixtures under CARGO_MANIFEST_DIR/fixtures/fonts;
temporary writes use std::env::temp_dir with unique names and cleanup. No external
lab paths, host-family prerequisite or public instrumentation. No global allocator
or injected allocator failure here: those belong only to the separate executable.
For oversized output, use a logging reader to prove rejection before payload I/O;
keep the stronger allocation-before-limit check in that executable. Access the
parent loader through crate::fontdb, not an external self-crate alias. No clock or
performance thresholds. Preserve the counterexample where a post prefix/payload
fits but its full declaration does not, for both v2 and v3.

The facade/extractor/pairing implementations remain owned by their existing
prompts. These tests own no productive algorithm or cross-file contract.
