# L0 — wiring/fonts
Hash do Código: a0c07ba3

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/fonts.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/fonts.rs:11` — `run_fonts`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/fonts.rs`.

## Owned contract

- Own run_fonts composition: call L3 inventory_fonts with requested paths/include-system and map family/path/index/style/weight/stretch/embedded into L2 FontDisplayEntry.
- Serialize with the existing format_fonts variants and write stdout. Preserve discovery/order; do not rescan, resort or implement font policy. Success exits 0 and output failure exits 2.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
