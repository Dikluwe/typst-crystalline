## Operational conventions for `#[comemo::track]` in L1

Three discoveries from the migration must be considered by future work touching tracked types.

### 1. Tracked structs require `#[derive(Clone)]`

`#[comemo::track] impl T { ... }` requires `T: Clone`. Comemo clones the value internally to roll back tracked mutations; without `Clone`, the impl fails to compile. The cost is O(n) across fields and is acceptable for small structs or structs with deduplication. Discovered in Step 106 (materializing `Sink`).

### 2. `Option<&str>` fails in tracked methods

Tracked methods taking `Option<&str>` fail because the proc macro does not preserve lifetime elision. Use `&str` with the convention `"" = absent`. This accepted limitation cannot distinguish absent from an intentionally empty string. Discovered in Step 107 (adding a hint to `Sink::warn_note`).

### 3. Use `TrackedMut::reborrow_mut` when descending lifetimes

`TrackedMut<'outer, T>` does not automatically shrink to a shorter lifetime. Constructing a nested structure with `'inner < 'outer` requires `TrackedMut::reborrow_mut(&mut *outer)` at every descent point. Otherwise compilation fails with `E0597`. Discovered in Step 109 (Engine<'a> scope-local).
