# L0 — wiring/init
Hash do Código: 9d9d73b6

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/init.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/init.rs:10` — `run_init`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/init.rs`.

## Owned contract

- Own run_init composition and pass template/destination/certificate inputs to the existing L3 initialize operation.
- On success print the existing destination and entrypoint lines and exit 0; initialization failure exits 1. Validation, download, cleanup and transactional project writes remain in L3.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
