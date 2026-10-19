# L0 — wiring/completions
Hash do Código: a0f0267c

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/completions.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
```

## Current evidence

- `04_wiring/src/completions.rs:11` — `run_completions`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/completions.rs`.

## Owned contract

- Own run_completions only: pass the requested shell to the L2 generator and write unchanged script bytes to stdout.
- Success exits 0; output I/O failure exits 2. No new CLI parser, local template, filesystem artifact or completion policy belongs in wiring.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
