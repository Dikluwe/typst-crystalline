# L0 — infra/plugin_host
Hash do Código: e5695a34

**Layer:** L3
**Exclusive consumer:** `03_infra/src/plugin_host.rs`

## Current evidence

- `03_infra/src/plugin_host.rs:19` — `WasmiPluginHost`.
- `03_infra/src/plugin_host.rs:40` — `PluginEntry`.
- `03_infra/src/plugin_host.rs:48` — `new`.
- `03_infra/src/plugin_host.rs:61` — `default`.
- `03_infra/src/plugin_host.rs:67` — `load`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/plugin_host.rs`.

## Owned contract

- Own WasmiPluginHost facade state, PluginEntry, new/Default, defensive lookup and the existing PluginHost delegation. Preserve public Send+Sync and the L1 contract at `00_nucleo/prompts/core/contracts/plugin_host.md`.
- new disables relaxed SIMD, initializes a private engine, AtomicU64 at 1 and Mutex map of shared entries. PluginEntry retains a shared compiled base and optional owned memory snapshot.
- Delegate load/exports to loading and call/transition to execution; linker protocol and memory operations belong to their own children. Do not rebuild their algorithms in the facade, add a pool or change runtime/features.
- Lookup clones the entry Arc or returns `plugin module not found`. This is defensive direct-Rust API behavior, not a new upstream language parity claim.

## Child owners

- `00_nucleo/prompts/infra/plugin_host/execution.md` owns only `03_infra/src/plugin_host/execution.rs`.
- `00_nucleo/prompts/infra/plugin_host/loading.md` owns only `03_infra/src/plugin_host/loading.rs`.
- `00_nucleo/prompts/infra/plugin_host/snapshot.md` owns only `03_infra/src/plugin_host/snapshot.rs`.
- `00_nucleo/prompts/infra/plugin_host/tests.md` owns only `03_infra/src/plugin_host/tests.rs`.
- `00_nucleo/prompts/infra/plugin_host/typst_env.md` owns only `03_infra/src/plugin_host/typst_env.rs`.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
