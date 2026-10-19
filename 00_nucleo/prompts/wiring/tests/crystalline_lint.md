# L0 — wiring/tests/crystalline_lint
Hash do Código: 8eeeec37

**Layer:** L4
**Exclusive consumer:** `04_wiring/tests/crystalline_lint.rs`

## Current evidence

- `04_wiring/tests/crystalline_lint.rs:20` — `temp_project`.
- `04_wiring/tests/crystalline_lint.rs:29` — `write_file`.
- `04_wiring/tests/crystalline_lint.rs:38` — `setup_project_dirs`.
- `04_wiring/tests/crystalline_lint.rs:85` — `type_level_violation_ecow_ecomap`.
- `04_wiring/tests/crystalline_lint.rs:116` — `type_level_allowed_ecow_ecostring`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/tests/crystalline_lint.rs`.

## Owned contract

- Own integration of the type-level V14 whitelist, not the typst CLI observables. Invoke crystalline-lint from PATH against minimal temporary crystalline projects.
- Declare L0/L1, Rust grammar, entities port, V14 severity and ecow allowlist for EcoString/EcoVec. EcoMap import must report V14 and package ecow; EcoString must not report V14. Do not require the entire forbidden item path when the linter identifies the package.
- Combine stdout/stderr for the report and isolate/clean fixtures. This test does not consume the CLI-observables nucleus or authorize a linter implementation change.

## Verification and boundary

Run `cargo test -p typst-wiring --test crystalline_lint -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.


## Private helper parameter representation

### Evidence

`lab/results/construction/diagnostics/clippy-expressions-2026-10-16/before.json.gz`
retains sources/L0, HEAD/uncommitted tree/status/diffstat/UTC;
initial-diagnostics.json retains suggestions/spans at crystalline_lint.rs:29 and
:38 (clippy::ptr_arg). Uses/types were inspected first. These are internal Rust
representations, no new upstream parity/performance claim.

### Contract and acceptance

Private write_file/setup_project_dirs take &Path instead of &PathBuf; keep PathBuf
as temp_project's return and add the Path import. Existing callers use borrowing
coercion without copying or cleanup/process/V14 changes. Continuous ADR-0127:
no public API/default/phase/dependency/suppression change. L0 first, scoped
resealing, unchanged input/result controls/tolerances. Require selected-lint
RED→GREEN, workspace suite/build/all-target Clippy/fmt/Rustdoc/lineage; no tests
merely mirroring Rust syntax.
