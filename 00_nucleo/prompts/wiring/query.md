# L0 — wiring/query
Hash do Código: 3d51292a

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/query.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/network/custom-ca-cert.toml sha256:0b28776068ad6b8e85a028a26cfc359679770b03f76d250bfd3ad68ff72643e3
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/query.rs:16` — `run_query`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/query.rs`.

## Owned contract

- Own run_query composition. Emit the existing deprecation warning first; preserve stdin markup/transient source versus physical root/main/font/CA paths.
- Call L3 query_elements, drain warnings then semantic errors, and delegate field/one/pretty/format output to L2 serialize_query_with_format. Selectors/cardinality are not reimplemented in L4.
- Success exits 0; semantic/serialization errors exit 1; I/O/argument errors exit 2. Preserve diagnostic origins through the dedicated drain owner.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
