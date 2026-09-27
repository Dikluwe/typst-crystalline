# L0 — infra/shaper/cache
Hash do Código: d1228c74

**Layer:** L3
**Exclusive consumer:** `03_infra/src/shaper/cache.rs`

## Current evidence

- `03_infra/src/shaper/cache.rs:19` — `ShapeCache`.
- `03_infra/src/shaper/cache.rs:24` — `CachedRun`.
- `03_infra/src/shaper/cache.rs:30` — `new`.
- `03_infra/src/shaper/cache.rs:34` — `key`.
- `03_infra/src/shaper/cache.rs:64` — `FaceCache`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/shaper/cache.rs`.

## Owned contract

- Own ShapeCache/CachedRun and FaceCache/CachedFace. Run cache is local to a document; face cache is per instance/compilation and may be shared by the metrics facade.
- Preserve exact textual run keys: text, slot, direction, variations formatted to four decimal places, absolute/em tracking to four places and effective ssty level. Values retain glyphs and i32 width in font units.
- Cache faces by slot including absence. Owned Font bytes keep parsed face borrows valid while CachedFace exists; preserve face index zero and existing encapsulated unsafe. No global cache, new eviction or key precision is authorized.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
