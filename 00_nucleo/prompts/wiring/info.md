# L0 — wiring/info
Hash do Código: 50377ddb

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/info.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:4b9f69ff41790a55a5b2d6a7b9982da858092e4bb2f0c503d6332a14a510e3ec
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:e9d6623a18ffc35f36051b280d96a75a0e4c1bd1768361a3a50c98587c568aef
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
