# L0 — entities/content/emptiness
Hash do Código: c7d9184f

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/emptiness.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own the semantic `Content::is_empty` projection. The exhaustive static match
must preserve which variants are observable and delegate element-specific
semantics to their owners.

## Acceptance

Existing emptiness behavior and tests remain unchanged; no rendering, I/O, or
dynamic registry is introduced.
