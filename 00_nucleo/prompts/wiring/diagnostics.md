# L0 — wiring/diagnostics
Hash do Código: c7d8e923

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/diagnostics.rs`

Pinned Tekt dependencies (canonical linter metadata):

```text
Núcleos Tekt:
- 00_nucleo/prompts/_nuclei/wiring/cli-observables.toml sha256:0e59744a924f0f6acbe7c4d20db9efc7caed4783c3b8d6b2c2c5a9cace340ece
```

## Current evidence

- `04_wiring/src/diagnostics.rs:15` — `drain_to_stderr`.
- `04_wiring/src/diagnostics.rs:24` — `drain_to_stderr_with_primary`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/diagnostics.rs`.

## Owned contract

- Own drain_to_stderr and drain_to_stderr_with_primary. Resolve diagnostic and trace FileId sources once, deduplicate, and use the existing main fallback for absent identity.
- A supplied transient primary overrides World for its own id. Display relative-to-cwd paths where appropriate with existing input fallback, then invoke the L2 DiagnosticSource formatter and emit to stderr.
- Preserve spans, traces, order and formatting selection without new coloring, filtering, reordering or a World dependency in L2. Do not centralize stdin in this helper or merge distinct eval/query provenance.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
