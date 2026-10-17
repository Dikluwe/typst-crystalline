# L0 — entities/content/sequence
Hash do Código: 7bbb85ee

**Layer:** L1
**Exclusive consumer:** `01_core/src/entities/content/sequence.rs`
**Governance:** ADR-0107, ADR-0129, ADR-0130.

## Contract

Own construction and normalization of `Content::Sequence`. Empty input becomes
`Content::Empty`, a singleton is unwrapped, and multiple items use the closed
sequence carrier without changing order.

## Acceptance

Normalization remains pure, deterministic, allocation-aware, and covered by a
local test. All callers retain the existing `Content::sequence` API.
