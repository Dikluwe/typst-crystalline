# L0 — infra/plugin_host/snapshot
Hash do Código: 771c2423

**Layer:** L3
**Exclusive consumer:** `03_infra/src/plugin_host/snapshot.rs`

## Current evidence

- `03_infra/src/plugin_host/snapshot.rs:11` — `Snapshot`.
- `03_infra/src/plugin_host/snapshot.rs:18` — `instance_memory`.
- `03_infra/src/plugin_host/snapshot.rs:29` — `snapshot_memory`.
- `03_infra/src/plugin_host/snapshot.rs:42` — `restore_memory`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/plugin_host/snapshot.rs`.

## Owned contract

- Own Snapshot (mem_pages and mem_data), instance_memory, snapshot_memory and restore_memory only. Execution owns when to invoke them.
- Capture exported memory's page count and owned byte copy. Restore by growing only when necessary and copying the saved prefix, without shrinking memory. Preserve internal memory-export/grow expects.
- Capture no WASM globals. No disk serialization, new limits, changed lifetime or new restore policy is authorized.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
