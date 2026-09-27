# L0 — infra/system-world/cache
Hash do Código: fedb3809

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/cache.rs`

## Current evidence

- `03_infra/src/world/cache.rs:20` — `SourceSlot`.
- `03_infra/src/world/cache.rs:27` — `new`.
- `03_infra/src/world/cache.rs:31` — `synthetic`.
- `03_infra/src/world/cache.rs:40` — `get`.
- `03_infra/src/world/cache.rs:58` — `register_file`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/cache.rs`.

## Owned contract

- Own SourceSlot/OnceLock, file registration, dependency enumeration and byte/source cache helpers; per-world storage remains in the facade.
- Synthetic sources are already loaded; physical sources load lazily except main. OnceLock memoizes the first Source result, including errors. Canonicalize registration when possible, otherwise retain the original path; preserve id allocation/wrapping. Registration must atomically associate one canonical path with its FileId and initialized slot under concurrent calls; keep the existing fast lookup and serialize the publication under next_id → path_to_id → slots, rechecking identity after acquiring the allocation lock.
- Successful byte reads share Arc by canonical FileId across path/id access. Do not cache failed byte reads, so they may retry; failed Source reads remain memoized. Preserve FileError/Bytes conversion behavior.
- Dependencies are sorted and unique and include main, sources, assets and failed attempts, not system fonts. Equivalent physical paths share identity without an independent cache.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
