# L0 — wiring/info
Hash do Código: 50377ddb

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/info.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/info.rs:11` — `run_info`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/info.rs`.

## Owned contract

- Own run_info: compose L3 snapshot, L1 PARITY_VERSION, L2 build commit/platform and ordered environment data into L2 InfoData.
- Preserve current font/package paths and declared false feature defaults, not inferred effective feature activation. Certificate reporting reveals presence from flag/environment only, never paths or secret bytes.
- Delegate human/JSON-pretty serialization to L2. Serialization/stdout failures exit 2; success exits 0. No TLS configuration or unapproved info fields/defaults are introduced.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
