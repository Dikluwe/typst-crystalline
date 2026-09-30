# L0 — infra/plugin_host/loading
Hash do Código: 9b8af1bd

**Layer:** L3
**Exclusive consumer:** `03_infra/src/plugin_host/loading.rs`

## Current evidence

- `03_infra/src/plugin_host/loading.rs:13` — `PluginBase`.
- `03_infra/src/plugin_host/loading.rs:18` — `load`.
- `03_infra/src/plugin_host/loading.rs:43` — `exports`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/plugin_host/loading.rs`.

## Owned contract

- Own PluginBase, load and exports only. Compile binary WASM before checking mandatory exported `memory`; do not instantiate at load or validate function signatures/imports eagerly.
- Preserve load errors `failed to load WebAssembly module ({err})` and `plugin does not export its memory`. Build the linker through typst_env, allocate a Relaxed fresh id only after success and insert a shared base with no snapshot.
- Enumerate only function exports in wasmi order as EcoString, including an empty list when none. Use the facade's defensive lookup; snapshots do not change exports.
- Keep the existing L3 wasmi dependency configured without default WAT features and with SIMD; binary-invalid error details come from that runtime. This does not authorize a version/feature upgrade or expose wasmi in L1.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
