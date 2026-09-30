# L0 — wiring/tests/crystalline_lint
Hash do Código: 687b9f29

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

Run `cargo test -p typst --test crystalline_lint -- --test-threads=1` with the harness prerequisites above.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
