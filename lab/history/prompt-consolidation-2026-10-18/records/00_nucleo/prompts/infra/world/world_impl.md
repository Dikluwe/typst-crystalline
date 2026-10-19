# L0 — infra/system-world/world_impl
Hash do Código: 6915739b

**Layer:** L3
**Exclusive consumer:** `03_infra/src/world/world_impl.rs`

## Current evidence

- `03_infra/src/world/world_impl.rs:19` — `library`.
- `03_infra/src/world/world_impl.rs:23` — `book`.
- `03_infra/src/world/world_impl.rs:27` — `main`.
- `03_infra/src/world/world_impl.rs:31` — `source`.
- `03_infra/src/world/world_impl.rs:35` — `file`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 03_infra/src/world/world_impl.rs`.

## Owned contract

- Own the sole `impl World for SystemWorld` adapter. Delegate cache/font/path/package behavior to the corresponding siblings; obtain book through the font child once-initialized catalog, return facade library/main and clone SysInputs/optional plugin Arc.
- Preserve `00_nucleo/prompts/core/contracts/world.md` signatures and public SystemWorld paths. Do not add a trait/vtable or export private descendants.
- today(None) uses local date; today(Some(offset)) uses UTC plus the exact converted duration. Overflow, clock or local-offset failure returns None without panic. Do not change the default or leak clock access into L1.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
