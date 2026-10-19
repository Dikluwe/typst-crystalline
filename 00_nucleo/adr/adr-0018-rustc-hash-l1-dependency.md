# ADR-0018 — `rustc_hash` → `[l1_allowed_external]` — supersedes ADR-0007

**Status:** ACCEPTED
**Supersedes:** ADR-0007
**Date:** 2026-03-23

---

## Context

ADR-0007 replaced `FxHashMap`/`FxHashSet` from `rustc_hash` with `std::collections::HashMap`/`HashSet` in the parser, on the grounds that `rustc_hash` was an unjustified performance dependency in L1.

That decision was wrong. Analysis of `typst-library/foundations/` revealed that `rustc_hash` is required for `Scope` (which is required for a real `Module`), forcing a reevaluation.

## Why ADR-0007 was wrong

`rustc_hash` provides `FxHasher` — a hasher without HashDoS attack resistance, optimized for small keys such as compiler identifiers. The Rust compiler itself uses it internally.

Relevant properties for L1:

- zero I/O;
- no mutable global state;
- no side effects;
- fully deterministic — the same input yields the same hash in every environment.

ADR-0007 applied the criterion “remove external” without checking whether the external dependency violated an L1 principle. It does not. `std`’s `SipHash-1-3` resists HashDoS — a relevant property for web servers receiving adversarial input, not for compilers processing user code. Replacing it with `std` cost performance without a real architectural gain.

**Recorded lesson:** the criterion for `[l1_allowed_external]` is not “is it external?” but “does it violate L1 functional purity?” Crates without I/O, mutable global state, or side effects are eligible whether or not they are “from std.”

## Decision

Add `rustc_hash` to `[l1_allowed_external]`:

```toml
[l1_allowed_external]
rust = [
    "thiserror",
    "comemo",
    "unicode_ident",
    "unicode_math_class",
    "unicode_script",
    "unicode_segmentation",
    "rustc_hash",
]
```

Reintroduce `FxHashMap` and `FxHashSet` in `01_core/rules/parse.rs`, replacing the `std::collections::HashMap` and `HashSet` added by ADR-0007.

Use `rustc_hash` as the hashing default throughout L1 where lookup performance matters. `std::collections::HashMap` remains valid when N is trivially small or readability justifies it.

## Impact on the parser (reversal of ADR-0007)

ADR-0007 made 2 substitutions in `parse.rs` (type declarations) and 16 substitutions in uses. Revert them:

```rust
// Remove (introduced by ADR-0007):
use std::collections::{HashMap, HashSet};

// Restore (original):
use rustc_hash::{FxHashMap, FxHashSet};
```

## Impact on Scope (Step 8+)

`Scope` in `typst-library/foundations/scope.rs` uses `indexmap` with `rustc_hash` as its hasher (`IndexMap<_, _, FxBuildHasher>`). With `rustc_hash` authorized in L1, `Scope` can migrate to L1 when `indexmap` is also authorized (ADR-0019).

## Affected prompts

| Prompt | Change |
|--------|--------|
| `00_nucleo/prompts/engine/parse.md` | Restore `rustc_hash`; refer to ADR-0018 and ADR-0007’s supersession |

## Consequences

**Positive:** parser hashing performance is restored; `rustc_hash` is available for `Scope` and other L1 types that need efficient hashing.

**Negative:** the ADR-0007 work must be reverted. The cost is low (search/replace), but the process created an unnecessary cycle.

## Alternatives considered

| Alternative | Advantages | Disadvantages |
|-------------|------------|---------------|
| Keep ADR-0007 (`std` in parser), use ADR-0018 only for Scope | Apparent consistency | Real inconsistency — the same crate would follow opposite decisions in L1 |
| Authorize `rustc_hash` only for Scope | Scope works | Parser remains slower for no reason |

## References

- ADR-0007 — superseded decision.
- `foundations/` analysis — `Scope` requires `rustc_hash`.
- `rustc-hash`: https://github.com/rust-lang/rustc-hash
