# L0 — wiring/eviction
Hash do Código: 76195d6f

**Layer:** L4
**Exclusive consumer:** `04_wiring/src/eviction.rs`

## Current evidence

- `04_wiring/src/eviction.rs:43` — `crystalline_evict`.
- `04_wiring/src/eviction.rs:53` — `p204e_crystalline_evict_existe`.
- `04_wiring/src/eviction.rs:64` — `p204e_crystalline_evict_aceita_max_age_parametro`.

These are local source anchors, not a new upstream parity measurement.
Reproduce discovery with `rg -n "fn |struct |enum |mod |pub use" 04_wiring/src/eviction.rs`.

## Owned contract

- Own only crystalline_evict(max_age: usize), recording the request through `00_nucleo/prompts/infra/measurements.md` before delegating unchanged to comemo::evict(max_age), and its local smoke tests.
- max_age is passed through without additional scheduling, cache policy or global state. The watch caller owns its existing evict(10) placement.
- No other comemo API or introspector/layout implementation is wrapped here. Verify the dependency and delegation through the existing wrapper controls.

## Verification and boundary

Verify this unit through its existing family/CLI tests when validating behavior.
Ownership is strictly prompt-to-consumer 1:1 (ADR-0129); referenced owners do not share this prompt.
Preserve existing public contracts, defaults and pipeline phases. Referenced child owners govern their own implementations; dependencies do not create shared ownership.
